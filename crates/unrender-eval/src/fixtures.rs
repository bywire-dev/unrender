//! Fixture discovery.
//!
//! Fixtures live at the workspace root rather than inside the published crate,
//! so they are excluded from `cargo publish` by construction. That means tests
//! have to locate them relative to the workspace, not the crate.

use std::path::{Path, PathBuf};

/// A captured screen, plus whatever ground truth exists for it.
#[derive(Debug, Clone)]
pub struct Fixture {
    /// Stable identifier, e.g. `legacy/ratatui-moved`.
    pub name: String,
    /// Top-level category: `legacy`, `vendor`, or `field`.
    pub category: String,
    pub ansi: PathBuf,
    /// Present only where a framework or a human supplied a real widget tree.
    pub truth: Option<PathBuf>,
}

impl Fixture {
    pub fn read_ansi(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.ansi)
    }

    pub fn read_truth(&self) -> Option<String> {
        self.truth
            .as_ref()
            .and_then(|p| std::fs::read_to_string(p).ok())
    }

    /// `framework`, `human`, or `none` — what the truth file (if any) is worth.
    /// Fixtures with a truth source are held to a hard quality floor; the rest
    /// are only checked against their own recorded baseline.
    pub fn truth_source(&self) -> &'static str {
        match &self.truth {
            None => "none",
            Some(p) if p.to_string_lossy().contains("eval-set") => "human",
            Some(_) => "framework",
        }
    }
}

/// Workspace root, derived from this crate's manifest directory.
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("crates/unrender-eval is two levels below the workspace root")
        .to_path_buf()
}

/// Every `.ansi` fixture under `fixtures/`, sorted by name for stable output.
pub fn discover(root: &Path) -> Vec<Fixture> {
    let mut out = Vec::new();
    let fixtures_dir = root.join("fixtures");
    if !fixtures_dir.is_dir() {
        return out;
    }
    for entry in walkdir::WalkDir::new(&fixtures_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("ansi") {
            continue;
        }
        let rel = path.strip_prefix(&fixtures_dir).unwrap_or(path);
        let name = rel.with_extension("").to_string_lossy().replace('\\', "/");
        let category = name.split('/').next().unwrap_or("unknown").to_string();
        let truth = path.with_extension("truth.json");
        out.push(Fixture {
            name,
            category,
            ansi: path.to_path_buf(),
            truth: truth.is_file().then_some(truth),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}
