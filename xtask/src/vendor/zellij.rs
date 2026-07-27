//! Thin driver over the zellij CLI plus a real pty-attached client -- a Rust
//! port of the old spike's `harness/zj.py`. Two details cost real debugging
//! time there and are preserved verbatim rather than rediscovered:
//!
//! - **Size the pty client before the app launches.** Anything drawn before
//!   a client attaches renders at zellij's small background-session default
//!   geometry, and its per-row wrap flags survive a later resize -- which
//!   corrupts `dump-screen`'s line structure for the pane's entire life.
//! - **`toggle-pane-frames` immediately after attach.** Pane frames steal a
//!   row and a column on each side, so without this, pane geometry and
//!   content geometry silently disagree.
//!
//! This is the primitive `field/` capture (Phase 5) will reuse -- built once
//! here for bubbletea/Ink rather than written twice.

use std::collections::HashSet;
use std::ffi::OsStr;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::process::Output;
use std::time::Duration;

use anyhow::{Context, Result};
use nix::fcntl::{fcntl, FcntlArg, OFlag};

fn run<I, S>(args: I, session: Option<&str>) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = std::process::Command::new("zellij");
    cmd.env("TERM", "xterm-256color");
    if let Some(s) = session {
        cmd.args(["--session", s]);
    }
    cmd.args(args);
    cmd.output().context("running zellij")
}

fn action<I, S>(args: I, session: Option<&str>) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut full: Vec<std::ffi::OsString> = vec!["action".into()];
    full.extend(args.into_iter().map(|s| s.as_ref().to_os_string()));
    run(full, session)
}

pub fn kill_session(session: &str) {
    let _ = run(["delete-session", session, "--force"], None);
}

/// Bar-free single-pane layout running `cmd` directly.
fn write_layout(path: &Path, cmd: &[String]) -> Result<()> {
    let (prog, rest) = cmd.split_first().context("empty command")?;
    let args_line = if rest.is_empty() {
        String::new()
    } else {
        let joined = rest
            .iter()
            .map(|a| serde_json::to_string(a).unwrap())
            .collect::<Vec<_>>()
            .join(" ");
        format!("        args {joined}\n")
    };
    let contents = format!(
        "layout {{\n    default_tab_template {{\n        children\n    }}\n    pane command={} {{\n{args_line}    }}\n}}\n",
        serde_json::to_string(prog)?
    );
    std::fs::write(path, contents)?;
    Ok(())
}

fn new_session(session: &str, layout_path: &Path) -> Result<()> {
    kill_session(session);
    let out = run(
        [
            "attach",
            "--create-background",
            session,
            "options",
            "--default-layout",
            layout_path.to_str().context("non-utf8 layout path")?,
        ],
        None,
    )?;
    anyhow::ensure!(
        out.status.success(),
        "zellij attach --create-background failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    std::thread::sleep(Duration::from_secs(1));
    Ok(())
}

fn list_panes(session: &str) -> Result<serde_json::Value> {
    let out = action(["list-panes", "--json", "--all"], Some(session))?;
    Ok(serde_json::from_slice(&out.stdout).unwrap_or(serde_json::Value::Null))
}

/// zellij's `--json --all` output has been seen both as a flat array and as
/// an object keyed by tab; handle either rather than assume one.
fn terminal_panes(session: &str) -> Result<Vec<serde_json::Value>> {
    let data = list_panes(session)?;
    let mut panes = Vec::new();
    match data {
        serde_json::Value::Array(a) => panes.extend(a),
        serde_json::Value::Object(o) => {
            for v in o.into_values() {
                if let serde_json::Value::Array(a) = v {
                    panes.extend(a);
                }
            }
        }
        _ => {}
    }
    Ok(panes
        .into_iter()
        .filter(|p| {
            !p.get("is_plugin")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        })
        .collect())
}

fn pane_id_of(p: &serde_json::Value) -> Option<String> {
    p.get("id").map(|v| v.to_string())
}

fn new_pane(session: &str, cmd: &[String], cwd: Option<&Path>) -> Result<()> {
    let mut args = vec!["new-pane".to_string()];
    if let Some(dir) = cwd {
        args.push("--cwd".to_string());
        args.push(dir.to_str().context("non-utf8 cwd")?.to_string());
    }
    args.push("--".to_string());
    args.extend(cmd.iter().cloned());
    action(args, Some(session))?;
    Ok(())
}

pub fn dump_screen(session: &str, pane_id: &str) -> Result<String> {
    let out = action(
        ["dump-screen", "--pane-id", pane_id, "--ansi"],
        Some(session),
    )?;
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Not called anywhere right now -- no fixture currently needs navigation --
/// but kept as a real capability rather than deleted. The bar for using it:
/// revealing a genuinely different part of an app's UI (a different tab, a
/// panel that only appears after selecting something) is worth it; moving a
/// cursor down a list to prove the harness *can* send keys is not.
#[allow(dead_code)]
pub fn send_keys(session: &str, pane_id: &str, keys: &[&str]) -> Result<()> {
    let mut args = vec![
        "send-keys".to_string(),
        "--pane-id".to_string(),
        pane_id.to_string(),
    ];
    args.extend(keys.iter().map(|s| s.to_string()));
    action(args, Some(session))?;
    Ok(())
}

fn set_nonblocking(fd: std::os::fd::RawFd) -> Result<()> {
    let borrowed = unsafe { std::os::fd::BorrowedFd::borrow_raw(fd) };
    let flags = fcntl(borrowed, FcntlArg::F_GETFL)?;
    let mut oflags = OFlag::from_bits_truncate(flags);
    oflags.insert(OFlag::O_NONBLOCK);
    fcntl(borrowed, FcntlArg::F_SETFL(oflags))?;
    Ok(())
}

/// A real zellij client attached inside a pty of a chosen size. Background
/// sessions have no client and therefore default to a small geometry --
/// attaching a sized pty client is the only way to give the session real
/// dimensions before anything draws into it.
pub struct PtyClient {
    pty: pty_process::blocking::Pty,
    child: std::process::Child,
}

impl PtyClient {
    pub fn new(session: &str, cols: u16, rows: u16) -> Result<Self> {
        let (pty, pts) = pty_process::blocking::open().context("opening pty")?;
        pty.resize(pty_process::Size::new(rows, cols))
            .context("sizing pty")?;
        set_nonblocking(pty.as_raw_fd())?;

        let child = pty_process::blocking::Command::new("zellij")
            .args(["attach", session])
            .spawn(pts)
            .context("spawning zellij attach")?;
        std::thread::sleep(Duration::from_secs(2));

        action(["toggle-pane-frames"], Some(session))?;
        std::thread::sleep(Duration::from_secs(1));

        let mut client = PtyClient { pty, child };
        client.drain();
        Ok(client)
    }

    /// Discard pending client output so the pty buffer never blocks a later
    /// write. The fd is non-blocking, so a `WouldBlock` just means "empty".
    pub fn drain(&mut self) {
        let mut buf = [0u8; 65536];
        loop {
            match self.pty.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => continue,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }
    }
}

impl Drop for PtyClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Brings up a session, sizes it with a real client, and only then launches
/// `cmd`. Order matters -- see the module docs. Returns the client (keep it
/// alive for the session's lifetime) and the new pane's id.
pub fn start_app(
    session: &str,
    cmd: &[String],
    cols: u16,
    rows: u16,
) -> Result<(PtyClient, String)> {
    start_app_in(session, cmd, cols, rows, None)
}

/// Same as [`start_app`], but launches `cmd` in `cwd` instead of the xtask
/// process's own directory -- needed for apps like lazygit/gitui that must
/// run inside a specific (in our case, synthetic) git repo.
pub fn start_app_in(
    session: &str,
    cmd: &[String],
    cols: u16,
    rows: u16,
    cwd: Option<&Path>,
) -> Result<(PtyClient, String)> {
    kill_session(session);
    let boot_layout = std::env::temp_dir().join(format!("zj-{session}-boot.kdl"));
    write_layout(&boot_layout, &["sleep".to_string(), "100000".to_string()])?;
    new_session(session, &boot_layout)?;

    let mut client = PtyClient::new(session, cols, rows)?;
    let before: HashSet<String> = terminal_panes(session)?
        .iter()
        .filter_map(pane_id_of)
        .collect();

    new_pane(session, cmd, cwd)?;
    std::thread::sleep(Duration::from_millis(2500));
    client.drain();

    let after = terminal_panes(session)?;
    let pane_id = after
        .iter()
        .find_map(|p| pane_id_of(p).filter(|id| !before.contains(id)))
        .or_else(|| after.last().and_then(pane_id_of))
        .context("no pane found after launching app")?;

    // Drop the bootstrap sleep pane so the app owns the whole terminal.
    for p in &after {
        if let Some(id) = pane_id_of(p) {
            if id != pane_id {
                let _ = action(["close-pane", "--pane-id", &id], Some(session));
            }
        }
    }
    std::thread::sleep(Duration::from_millis(1200));
    client.drain();

    Ok((client, pane_id))
}
