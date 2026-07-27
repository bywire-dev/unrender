//! Vendors `fixtures/vendor/ink.ansi` -- the Node/Ink half of the
//! deploy-console corpus, same mechanism as `bubbletea.rs`. Ink has no
//! offline test backend either, so this is a real zellij pty capture, not a
//! cheaper reconstruction.

use std::path::Path;

use anyhow::{Context, Result};

use super::zellij;

const SESSION: &str = "unrender-vendor-ink";

pub fn run(root: &Path) -> Result<()> {
    let src = root.join("fixtures/vendor-src/ink");
    install(&src)?;
    let dir = root.join("fixtures/vendor");
    std::fs::create_dir_all(&dir)?;

    let cmd = vec![
        "node".to_string(),
        src.join("app.mjs")
            .to_str()
            .context("non-utf8 path")?
            .to_string(),
    ];
    let (client, pane_id) = zellij::start_app(SESSION, &cmd, 100, 30)?;

    let ansi = zellij::dump_screen(SESSION, &pane_id)?;
    std::fs::write(dir.join("ink.ansi"), &ansi)?;
    println!("wrote fixtures/vendor/ink.ansi");

    drop(client);
    zellij::kill_session(SESSION);
    Ok(())
}

fn install(src: &Path) -> Result<()> {
    if src.join("node_modules").is_dir() {
        return Ok(());
    }
    let status = std::process::Command::new("npm")
        .args(["install", "--no-audit", "--no-fund"])
        .current_dir(src)
        .status()
        .context("running `npm install` -- is npm on PATH?")?;
    anyhow::ensure!(
        status.success(),
        "npm install failed for fixtures/vendor-src/ink"
    );
    Ok(())
}
