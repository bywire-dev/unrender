//! Recorded metrics, and drift against them.
//!
//! The suite deliberately does **not** assert that `unrender` is perfect. It
//! asserts that it has not got *worse*. Known failures are recorded here as
//! facts rather than hidden, so a later fix shows up as a visible positive
//! delta instead of merely turning a red check green.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// How far a float metric may fall before it counts as a regression.
/// Scoring is deterministic, so this only absorbs float formatting noise.
pub const TOLERANCE: f64 = 0.005;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FixtureMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_agreement: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_iou: Option<f64>,
    /// `None` when the fixture's truth declares no selection to check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_ok: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Baseline {
    /// Free-text note explaining anything a reader would otherwise misread —
    /// notably which entries record a *known failure* on purpose.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    #[serde(default)]
    pub fixtures: BTreeMap<String, FixtureMetrics>,
}

pub fn path(root: &Path) -> PathBuf {
    root.join("results").join("baseline.json")
}

impl Baseline {
    pub fn load(root: &Path) -> Result<Option<Baseline>> {
        let p = path(root);
        if !p.is_file() {
            return Ok(None);
        }
        let raw =
            std::fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
        Ok(Some(
            serde_json::from_str(&raw).with_context(|| format!("parsing {}", p.display()))?,
        ))
    }

    pub fn save(&self, root: &Path) -> Result<()> {
        let p = path(root);
        if let Some(dir) = p.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut json = serde_json::to_string_pretty(self)?;
        json.push('\n');
        std::fs::write(&p, json).with_context(|| format!("writing {}", p.display()))?;
        Ok(())
    }
}

/// One metric that moved.
#[derive(Debug, Clone)]
pub struct Drift {
    pub fixture: String,
    pub metric: String,
    pub was: String,
    pub now: String,
    pub improved: bool,
}

fn cmp_f64(out: &mut Vec<Drift>, fixture: &str, metric: &str, was: Option<f64>, now: Option<f64>) {
    if let (Some(w), Some(n)) = (was, now) {
        if (n - w).abs() > TOLERANCE {
            out.push(Drift {
                fixture: fixture.to_string(),
                metric: metric.to_string(),
                was: format!("{w:.3}"),
                now: format!("{n:.3}"),
                improved: n > w,
            });
        }
    }
}

/// Every metric that differs between the recorded baseline and a fresh run.
/// Reports improvements as well as regressions — a silent improvement is a
/// baseline that needs updating, which is worth surfacing too.
pub fn diff(was: &FixtureMetrics, now: &FixtureMetrics, fixture: &str) -> Vec<Drift> {
    let mut out = Vec::new();
    cmp_f64(&mut out, fixture, "recall", was.recall, now.recall);
    cmp_f64(&mut out, fixture, "precision", was.precision, now.precision);
    cmp_f64(
        &mut out,
        fixture,
        "role_agreement",
        was.role_agreement,
        now.role_agreement,
    );
    cmp_f64(&mut out, fixture, "mean_iou", was.mean_iou, now.mean_iou);
    if let (Some(w), Some(n)) = (was.selection_ok, now.selection_ok) {
        if w != n {
            out.push(Drift {
                fixture: fixture.to_string(),
                metric: "selection_ok".to_string(),
                was: w.to_string(),
                now: n.to_string(),
                improved: n,
            });
        }
    }
    out
}
