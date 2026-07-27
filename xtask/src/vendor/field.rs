//! `xtask field <app>` -- launch-only capture of real independently-built
//! apps, never driven by keystrokes. Most TUIs show something substantial on
//! startup, so one default screen per app is enough to exercise round-trip
//! fidelity, the LLM benchmark, and human legibility without anyone needing
//! to know how to navigate 8 different programs.
//!
//! `tig`, `ncdu`, and `mc` are deferred, like `k9s` in the plan: all three
//! need either system packages (ncurses dev headers) or a toolchain (Zig for
//! ncdu 2.x) not installable without root in this environment. Not a design
//! decision -- just an environment limit, recorded here rather than silently
//! dropped.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{bail, Context, Result};

use super::zellij;

pub fn run(root: &Path, app: Option<&str>) -> Result<()> {
    match app {
        Some("lazygit") => {
            disable_lazygit_startup_popups()?;
            capture_in_repo(root, "lazygit", &["lazygit".to_string()])
        }
        Some("gitui") => capture_in_repo(root, "gitui", &["gitui".to_string()]),
        Some("btop") => capture(root, "btop", &["btop".to_string()], None),
        Some("htop") => capture(root, "htop", &["htop".to_string()], None),
        Some("bat") => capture_bat(root),
        Some(other) => bail!(
            "unknown field target: {other}\nsupported: lazygit, gitui, btop, htop, bat\n\
             deferred (need root or a toolchain unavailable here): tig, ncdu, mc, k9s"
        ),
        None => bail!("usage: xtask field <app>\nsupported: lazygit, gitui, btop, htop, bat"),
    }
}

fn capture(root: &Path, name: &str, cmd: &[String], cwd: Option<&Path>) -> Result<()> {
    let dir = root.join("fixtures/field");
    std::fs::create_dir_all(&dir)?;
    let session = format!("unrender-field-{name}");

    let (mut client, pane_id) = zellij::start_app_in(&session, cmd, 100, 30, cwd)?;
    // btop/htop paint their first real frame (with actual CPU/mem numbers,
    // not zeros) a beat after launch; everything else is already settled by
    // start_app_in's own pacing.
    std::thread::sleep(Duration::from_millis(1500));
    client.drain();

    let ansi = zellij::dump_screen(&session, &pane_id)?;
    std::fs::write(dir.join(format!("{name}.ansi")), ansi)?;
    println!("wrote fixtures/field/{name}.ansi");

    drop(client);
    zellij::kill_session(&session);
    Ok(())
}

/// Config, not a keystroke: lazygit's first-run tips dialog would otherwise
/// cover the entire screen on every capture, since a launch-only capture by
/// definition never presses <enter> to dismiss it. Turning it off in config
/// stays inside the "no keystrokes, no navigation" rule.
fn disable_lazygit_startup_popups() -> Result<()> {
    let config_dir = std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config")
        })
        .join("lazygit");
    std::fs::create_dir_all(&config_dir)?;
    let path = config_dir.join("config.yml");
    let existing = std::fs::read_to_string(&path).unwrap_or_default();
    let patched = if existing.contains("disableStartupPopups:") {
        existing.replace("disableStartupPopups: false", "disableStartupPopups: true")
    } else {
        format!("{existing}\ndisableStartupPopups: true\n")
    };
    std::fs::write(&path, patched)?;
    Ok(())
}

fn capture_in_repo(root: &Path, name: &str, cmd: &[String]) -> Result<()> {
    let repo = make_synthetic_repo()?;
    capture(root, name, cmd, Some(&repo))
}

fn capture_bat(root: &Path) -> Result<()> {
    // A real, syntax-highlighted source file from this repo -- not a
    // throwaway sample, so the fixture is representative of what bat
    // actually looks like in use.
    // With paging off, a file longer than the terminal just scrolls past --
    // dump-screen would then show whatever's left at the bottom, not bat's
    // own header. Capping the range keeps the file's header banner on
    // screen, which is the representative "launched bat" view.
    let target = root.join("xtask/src/vendor/svg_term.rs");
    let cmd = vec![
        "bat".to_string(),
        "--paging=never".to_string(),
        "--color=always".to_string(),
        "--line-range".to_string(),
        ":25".to_string(),
        target.to_str().context("non-utf8 path")?.to_string(),
    ];
    capture(root, "bat", &cmd, None)
}

/// A small, deterministic git repo -- a few commits, a clean history, one
/// staged addition and one unstaged edit -- so lazygit/gitui have real state
/// to render without depending on this project's own live working tree
/// (which would make the fixture's content drift on every capture).
fn make_synthetic_repo() -> Result<PathBuf> {
    let dir = std::env::temp_dir().join("unrender-field-synthetic-repo");
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(dir.join("src"))?;

    let git = |args: &[&str], cwd: &Path| -> Result<()> {
        let status = std::process::Command::new("git")
            .args(args)
            .current_dir(cwd)
            .status()
            .context("running git")?;
        anyhow::ensure!(status.success(), "git {:?} failed", args);
        Ok(())
    };

    git(&["init", "-q"], &dir)?;
    git(&["config", "user.email", "demo@example.com"], &dir)?;
    git(&["config", "user.name", "Demo"], &dir)?;

    std::fs::write(
        dir.join("src/main.rs"),
        "fn main() {\n    println!(\"hello\");\n}\n",
    )?;
    git(&["add", "src/main.rs"], &dir)?;
    git(&["commit", "-q", "-m", "initial commit"], &dir)?;

    std::fs::write(dir.join("README.md"), "# demo\n")?;
    git(&["add", "README.md"], &dir)?;
    git(&["commit", "-q", "-m", "add readme"], &dir)?;

    std::fs::write(
        dir.join("src/main.rs"),
        "fn main() {\n    println!(\"hello\");\n}\n\nfn helper() {}\n",
    )?;
    git(&["add", "-A"], &dir)?;
    git(&["commit", "-q", "-m", "add helper"], &dir)?;

    // Leave real uncommitted state: one staged new file, one unstaged edit.
    std::fs::write(
        dir.join("src/main.rs"),
        "fn main() {\n    println!(\"hello\");\n}\n\nfn helper() {}\n\n// TODO: fix this\n",
    )?;
    std::fs::write(
        dir.join("src/lib.rs"),
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
    )?;
    git(&["add", "src/lib.rs"], &dir)?;

    Ok(dir)
}
