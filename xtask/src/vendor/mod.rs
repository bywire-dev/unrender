//! `xtask vendor <framework>` — generates `fixtures/vendor/<framework>/*`
//! directly from the framework's own layout engine, so structural ground
//! truth (`truth.json`) comes from real computed `Rect`s rather than a human
//! guessing at one.

mod ansi;
mod bubbletea;
mod ink;
mod ratatui_app;
mod svg_term;
mod textual;
mod zellij;

use std::path::Path;

use anyhow::{bail, Result};

pub fn run(root: &Path, framework: Option<&str>) -> Result<()> {
    match framework {
        Some("ratatui") => ratatui(root),
        Some("textual") => textual::run(root),
        Some("bubbletea") => bubbletea::run(root),
        Some("ink") => ink::run(root),
        Some(other) => {
            bail!("unknown vendor target: {other}\nsupported: ratatui, textual, bubbletea, ink")
        }
        None => {
            bail!("usage: xtask vendor <framework>\nsupported: ratatui, textual, bubbletea, ink")
        }
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
