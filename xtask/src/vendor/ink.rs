//! Vendors `fixtures/vendor/ink/{default,moved}.ansi` -- the Node/Ink half
//! of the deploy-console corpus, same mechanism as `bubbletea.rs`. Ink has
//! no offline test backend either, so this is a real zellij pty capture,
//! not a cheaper reconstruction.

use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};

use super::zellij;

const SESSION: &str = "unrender-vendor-ink";

pub fn run(root: &Path) -> Result<()> {
    let src = root.join("fixtures/vendor-src/ink");
    install(&src)?;
    let dir = root.join("fixtures/vendor/ink");
    std::fs::create_dir_all(&dir)?;

    let cmd = vec![
        "node".to_string(),
        src.join("app.mjs")
            .to_str()
            .context("non-utf8 path")?
            .to_string(),
    ];
    let (mut client, pane_id) = zellij::start_app(SESSION, &cmd, 100, 30)?;

    let default_ansi = zellij::dump_screen(SESSION, &pane_id)?;
    std::fs::write(dir.join("default.ansi"), &default_ansi)?;
    println!("wrote fixtures/vendor/ink/default.ansi");

    // Same rationale as bubbletea.rs: one key per call, paced, two presses
    // of 'j' to reach "billing" (index 2).
    for _ in 0..2 {
        zellij::send_keys(SESSION, &pane_id, &["j"])?;
        std::thread::sleep(Duration::from_millis(350));
    }
    std::thread::sleep(Duration::from_millis(300));
    client.drain();

    let moved_ansi = zellij::dump_screen(SESSION, &pane_id)?;
    std::fs::write(dir.join("moved.ansi"), &moved_ansi)?;
    println!("wrote fixtures/vendor/ink/moved.ansi");

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
