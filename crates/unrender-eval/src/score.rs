//! Structural scoring against a framework's real widget tree.
//!
//! Matching is by **geometry** (IoU), never by name: a node counts as found
//! only if the unrenderer put it in the right place on screen, which is the
//! actual claim under test. This is the axis that catches a tree which
//! preserves every character but invents the wrong structure — something
//! round-trip fidelity scores perfectly and cannot detect.

use anyhow::{Context, Result};
use serde::Deserialize;
use unrender::ax::Node;
use unrender::build_tree;
use unrender::rects::Rect;

pub const IOU_THRESHOLD: f64 = 0.5;

#[derive(Deserialize)]
struct TruthRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct TruthNode {
    role: String,
    #[serde(default)]
    #[allow(dead_code)]
    name: String,
    rect: TruthRect,
}

#[derive(Deserialize)]
struct Truth {
    #[allow(dead_code)]
    app: String,
    #[serde(default)]
    selected: Option<usize>,
    nodes: Vec<TruthNode>,
}

/// Did the tree report the same selected row the app says is selected?
#[derive(Debug, Clone)]
pub struct SelectionCheck {
    pub expected: usize,
    pub found: Vec<usize>,
    pub ok: bool,
}

#[derive(Debug, Clone)]
pub struct ScoreResult {
    pub truth_nodes: usize,
    pub inferred_nodes: usize,
    pub matched: usize,
    pub role_ok: usize,
    pub mean_iou: f64,
    pub selection: Option<SelectionCheck>,
}

impl ScoreResult {
    pub fn recall(&self) -> f64 {
        ratio(self.matched, self.truth_nodes)
    }
    pub fn precision(&self) -> f64 {
        ratio(self.matched, self.inferred_nodes)
    }
    pub fn role_agreement(&self) -> f64 {
        ratio(self.role_ok, self.matched)
    }
}

fn ratio(num: usize, den: usize) -> f64 {
    if den == 0 {
        0.0
    } else {
        num as f64 / den as f64
    }
}

/// Roles that mean the same thing across the two vocabularies.
///
/// `panel` is deliberately absent: it is the unrenderer's generic container,
/// and letting it satisfy every container-ish truth role would score a tree
/// that found the right rectangle but failed to classify it as a full match.
fn role_matches(truth: &str, inferred: &str) -> bool {
    if truth == inferred {
        return true;
    }
    let equiv: &[(&str, &[&str])] = &[
        ("text", &["log", "text", "paragraph"]),
        ("log", &["text", "log"]),
        ("heading", &["heading", "text", "statusbar"]),
        ("statusbar", &["statusbar", "text", "heading"]),
        ("table", &["table"]),
        ("list", &["list"]),
        ("tree", &["tree"]),
        ("progressbar", &["progressbar"]),
        ("application", &["application"]),
    ];
    equiv
        .iter()
        .find(|(t, _)| *t == truth)
        .map(|(_, ok)| ok.contains(&inferred))
        .unwrap_or(false)
}

/// Score one capture against its truth file.
pub fn compute_score(ansi: &[u8], truth_json: &str) -> Result<ScoreResult> {
    let truth: Truth = serde_json::from_str(truth_json).context("parsing truth json")?;
    let tree = build_tree(ansi);
    let mut flat: Vec<&Node> = Vec::new();
    tree.flatten(&mut flat);

    // Score container-level nodes only; rows and cells have no counterpart in
    // a framework's widget tree, so counting them would deflate precision.
    let candidates: Vec<&&Node> = flat
        .iter()
        .filter(|n| {
            !matches!(
                n.role.as_str(),
                "row" | "rowheader" | "cell" | "listitem" | "treeitem" | "property"
            )
        })
        .collect();

    let mut matched = 0usize;
    let mut role_ok = 0usize;
    let mut used = vec![false; candidates.len()];
    let mut ious = Vec::new();

    for t in &truth.nodes {
        let tr = Rect {
            x: t.rect.x,
            y: t.rect.y,
            w: t.rect.w,
            h: t.rect.h,
        };
        let mut best = (0.0f64, usize::MAX);
        for (i, c) in candidates.iter().enumerate() {
            if used[i] {
                continue;
            }
            let iou = tr.iou(&c.rect_of());
            if iou > best.0 {
                best = (iou, i);
            }
        }
        if best.0 >= IOU_THRESHOLD && best.1 != usize::MAX {
            used[best.1] = true;
            matched += 1;
            ious.push(best.0);
            if role_matches(&t.role, &candidates[best.1].role) {
                role_ok += 1;
            }
        }
    }

    let selection = truth.selected.map(|expected| {
        let mut found = Vec::new();
        for n in &flat {
            if matches!(n.role.as_str(), "table" | "list" | "tree") {
                let body: Vec<&Node> = n
                    .children
                    .iter()
                    .filter(|c| c.role != "rowheader")
                    .collect();
                for (i, c) in body.iter().enumerate() {
                    if c.states.iter().any(|s| s == "selected") {
                        found.push(i);
                    }
                }
            }
        }
        SelectionCheck {
            ok: found.contains(&expected),
            expected,
            found,
        }
    });

    let mean_iou = if ious.is_empty() {
        0.0
    } else {
        ious.iter().sum::<f64>() / ious.len() as f64
    };

    Ok(ScoreResult {
        truth_nodes: truth.nodes.len(),
        inferred_nodes: candidates.len(),
        matched,
        role_ok,
        mean_iou,
        selection,
    })
}
