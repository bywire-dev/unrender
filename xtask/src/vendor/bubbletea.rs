//! Vendors `fixtures/vendor/bubbletea/{default,moved}.ansi` from the same
//! deploy-console corpus app the old spike used, built fresh from pinned
//! source in `fixtures/vendor-src/bubbletea/` and captured through a real
//! zellij pty session (`zellij::start_app`) -- bubbletea has no offline test
//! backend the way ratatui does, so there is no cheaper mechanism here.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};

use super::zellij;

const SESSION: &str = "unrender-vendor-bubbletea";

pub fn run(root: &Path) -> Result<()> {
    let bin = build(root)?;
    let dir = root.join("fixtures/vendor/bubbletea");
    std::fs::create_dir_all(&dir)?;

    let cmd = vec![bin.to_string_lossy().to_string()];
    let (mut client, pane_id) = zellij::start_app(SESSION, &cmd, 100, 30)?;

    let default_ansi = zellij::dump_screen(SESSION, &pane_id)?;
    std::fs::write(dir.join("default.ansi"), &default_ansi)?;
    println!("wrote fixtures/vendor/bubbletea/default.ansi");

    // One key per send-keys call, ~350ms apart: batching multiple keys into
    // one call was found to silently drop all but the first (FINDINGS.md).
    // Two presses of 'j' moves the cursor api-gateway -> auth-service ->
    // billing, matching the "billing" selection used across the rest of
    // this corpus's `-moved`/`moved` fixture pairs.
    for _ in 0..2 {
        zellij::send_keys(SESSION, &pane_id, &["j"])?;
        std::thread::sleep(Duration::from_millis(350));
    }
    std::thread::sleep(Duration::from_millis(300));
    client.drain();

    let moved_ansi = zellij::dump_screen(SESSION, &pane_id)?;
    std::fs::write(dir.join("moved.ansi"), &moved_ansi)?;
    println!("wrote fixtures/vendor/bubbletea/moved.ansi");

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
