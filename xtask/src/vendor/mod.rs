//! `xtask vendor <framework>` — generates `fixtures/vendor/<framework>/*`
//! directly from the framework's own layout engine, so structural ground
//! truth (`truth.json`) comes from real computed `Rect`s rather than a human
//! guessing at one.

mod ansi;
mod ratatui_app;
mod svg_term;
mod textual;

use std::path::Path;

use anyhow::{bail, Result};

pub fn run(root: &Path, framework: Option<&str>) -> Result<()> {
    match framework {
        Some("ratatui") => ratatui(root),
        Some("textual") => textual::run(root),
        Some(other) => bail!("unknown vendor target: {other}\nsupported: ratatui, textual"),
        None => bail!("usage: xtask vendor <framework>\nsupported: ratatui, textual"),
    }
}

fn ratatui(root: &Path) -> Result<()> {
    let dir = root.join("fixtures/vendor/ratatui");
    std::fs::create_dir_all(&dir)?;

    for (name, selected) in [("default", 0usize), ("moved", 2)] {
        let (ansi, truth) = ratatui_app::render(selected)?;
        std::fs::write(dir.join(format!("{name}.ansi")), ansi)?;
        std::fs::write(
            dir.join(format!("{name}.truth.json")),
            serde_json::to_string(&truth)?,
        )?;
        println!("wrote fixtures/vendor/ratatui/{name}.{{ansi,truth.json}}");
    }
    Ok(())
}
