//! Round-trip fidelity across every fixture.
//!
//! Runs on *all* fixtures, not just the ones with ground truth — no truth file
//! is needed to ask "did the content survive". Gated on drift from
//! `results/baseline.json`, same as structural scoring.
//!
//!   cargo test -p unrender-eval --test roundtrip -- --nocapture
//!   UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval

use unrender::build_tree;
use unrender::grid::Grid;
use unrender::render::{compare, render};
use unrender_eval::baseline::{Baseline, TOLERANCE};
use unrender_eval::{discover, workspace_root};

#[test]
fn roundtrip_fidelity_matches_baseline() {
    let root = workspace_root();
    let updating = std::env::var_os("UNRENDER_UPDATE_BASELINE").is_some();
    let fixtures = discover(&root);
    assert!(!fixtures.is_empty(), "no fixtures discovered");

    let recorded = Baseline::load(&root).expect("load baseline");

    println!(
        "\n{:<26} {:>9} {:>9} {:>8} {:>8}  vs baseline",
        "fixture", "content", "w/chrome", "content", "chrome"
    );

    let mut fresh = std::collections::BTreeMap::new();
    let mut regressions = Vec::new();
    let mut improvements = Vec::new();

    for f in &fixtures {
        let bytes = f.read_ansi().expect("read fixture");
        let original = Grid::from_ansi_autosize(&bytes);
        let tree = build_tree(&bytes);
        let rebuilt = render(&tree);
        let fid = compare(&original, &rebuilt);

        let was = recorded
            .as_ref()
            .and_then(|b| b.fidelity.get(&f.name))
            .copied();

        let status = match was {
            None => "NEW".to_string(),
            Some(w) => {
                let d = fid.content_match - w;
                if d.abs() <= TOLERANCE {
                    "=".to_string()
                } else if d < 0.0 {
                    regressions.push(format!(
                        "{}: content {:.3} -> {:.3}",
                        f.name, w, fid.content_match
                    ));
                    format!("REGRESSED {:.3}->{:.3}", w, fid.content_match)
                } else {
                    improvements.push(format!(
                        "{}: content {:.3} -> {:.3}",
                        f.name, w, fid.content_match
                    ));
                    format!("IMPROVED {:.3}->{:.3}", w, fid.content_match)
                }
            }
        };

        println!(
            "{:<26} {:>8.1}% {:>8.1}% {:>8} {:>8}  {}",
            f.name,
            fid.content_match * 100.0,
            fid.nonblank_match * 100.0,
            fid.nonblank_cells - fid.chrome_cells,
            fid.chrome_cells,
            status
        );
        fresh.insert(f.name.clone(), fid.content_match);
    }

    if updating {
        let mut b = recorded.unwrap_or_default();
        b.fidelity = fresh;
        b.save(&root).expect("write baseline");
        println!("\nfidelity baseline re-recorded");
        return;
    }

    let mut msg = String::new();
    if !regressions.is_empty() {
        msg.push_str("round-trip fidelity REGRESSED:\n  ");
        msg.push_str(&regressions.join("\n  "));
    }
    if !improvements.is_empty() {
        if !msg.is_empty() {
            msg.push('\n');
        }
        msg.push_str("round-trip fidelity IMPROVED (re-record):\n  ");
        msg.push_str(&improvements.join("\n  "));
    }
    assert!(
        msg.is_empty(),
        "{msg}\n\nre-record with `UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval`"
    );
}
