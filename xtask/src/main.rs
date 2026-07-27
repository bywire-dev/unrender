//! Fixture generation and results reporting.
//!
//! Free to use zellij, Go, Node, Python, and the network -- none of it is
//! part of the published crate or the test suite, which stay pure. This is
//! the one place in the workspace that's allowed to shell out or open a
//! socket.

mod report;
mod tokens;
mod vendor;

fn main() -> anyhow::Result<()> {
    let root = unrender_eval::workspace_root();
    let mut args = std::env::args().skip(1);
    let cmd = args.next();
    match cmd.as_deref() {
        Some("report") => report::run(&root),
        Some("vendor") => vendor::run(&root, args.next().as_deref()),
        Some("field") => vendor::field::run(&root, args.next().as_deref()),
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            eprintln!("usage: xtask report | xtask vendor <framework> | xtask field <app>");
            std::process::exit(2);
        }
        None => {
            eprintln!("usage: xtask report | xtask vendor <framework> | xtask field <app>");
            std::process::exit(2);
        }
    }
}
