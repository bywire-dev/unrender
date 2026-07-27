//! Fixture generation and results reporting.
//!
//! Free to use zellij, Go, Node, Python, and the network -- none of it is
//! part of the published crate or the test suite, which stay pure. This is
//! the one place in the workspace that's allowed to shell out or open a
//! socket.

mod report;
mod tokens;

fn main() -> anyhow::Result<()> {
    let root = unrender_eval::workspace_root();
    let cmd = std::env::args().nth(1);
    match cmd.as_deref() {
        Some("report") => report::run(&root),
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            eprintln!("usage: xtask report");
            std::process::exit(2);
        }
        None => {
            eprintln!("usage: xtask report");
            std::process::exit(2);
        }
    }
}
