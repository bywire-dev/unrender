//! unrender: ANSI screen dump -> accessibility tree.
//!
//!   unrender <dump.ansi> [--format compact|toon|json] [--stats]

use unrender::{build_tree, emit, strip_ansi};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: unrender <dump.ansi> [--format compact|toon|json] [--stats]");
        std::process::exit(2);
    }
    let path = &args[1];
    let mut format = "compact".to_string();
    let mut stats = false;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--format" => {
                format = args.get(i + 1).cloned().unwrap_or_default();
                i += 1;
            }
            "--stats" => stats = true,
            _ => {}
        }
        i += 1;
    }

    let bytes = std::fs::read(path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    let root = build_tree(&bytes);

    let mut compact = String::new();
    emit::compact(&root, 0, &mut compact);
    let mut nogeo = String::new();
    emit::compact_opt(&root, 0, false, &mut nogeo);
    let mut toon = String::new();
    emit::toon(&root, 0, &mut toon);
    let json = serde_json::to_string_pretty(&root).unwrap();

    print!(
        "{}",
        match format.as_str() {
            "json" => &json,
            "toon" => &toon,
            "nogeo" => &nogeo,
            _ => &compact,
        }
    );

    if stats {
        let raw = String::from_utf8_lossy(&bytes).to_string();
        let plain = strip_ansi(&raw);
        eprintln!(
            "STATS\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            path,
            root.count(),
            emit::count_tokens(&raw),
            emit::count_tokens(&plain),
            emit::count_tokens(&compact),
            emit::count_tokens(&nogeo),
            emit::count_tokens(&toon),
            emit::count_tokens(&serde_json::to_string(&root).unwrap()),
        );
    }
}
