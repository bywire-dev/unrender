//! Vendors `fixtures/vendor/bubbletea.ansi` from the same deploy-console
//! corpus app the old spike used, built fresh from pinned source in
//! `fixtures/vendor-src/bubbletea/` and captured through a real zellij pty
//! session (`zellij::start_app`) -- bubbletea has no offline test backend
//! the way ratatui does, so there is no cheaper mechanism here.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::zellij;

const SESSION: &str = "unrender-vendor-bubbletea";

pub fn run(root: &Path) -> Result<()> {
    let bin = build(root)?;
    let dir = root.join("fixtures/vendor");
    std::fs::create_dir_all(&dir)?;

    let cmd = vec![bin.to_string_lossy().to_string()];
    let (client, pane_id) = zellij::start_app(SESSION, &cmd, 100, 30)?;

    let ansi = zellij::dump_screen(SESSION, &pane_id)?;
    std::fs::write(dir.join("bubbletea.ansi"), &ansi)?;
    println!("wrote fixtures/vendor/bubbletea.ansi");

    drop(client);
    zellij::kill_session(SESSION);
    Ok(())
}

fn build(root: &Path) -> Result<PathBuf> {
    let src = root.join("fixtures/vendor-src/bubbletea");
    let out = std::env::temp_dir().join("unrender-vendor-bubbletea-bin");
    let status = std::process::Command::new("go")
        .args(["build", "-o"])
        .arg(&out)
        .arg(".")
        .current_dir(&src)
        .status()
        .context("running `go build` -- is Go on PATH?")?;
    anyhow::ensure!(
        status.success(),
        "go build failed for fixtures/vendor-src/bubbletea"
    );
    Ok(out)
}
