//! Structural scoring over every fixture that has ground truth.
//!
//! This gates on **drift from the recorded baseline**, not on absolute
//! quality. `unrender` has known failures; they are recorded in
//! `results/baseline.json` as facts. A regression fails the build; an
//! improvement also fails, with a message telling you to re-record — because
//! an unrecorded improvement means the baseline is lying about where we are.
//!
//!   cargo test -p unrender-eval -- --nocapture     # see the table
//!   UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval   # re-record

use unrender_eval::baseline::{diff, Baseline, FixtureMetrics};
use unrender_eval::{compute_score, discover, workspace_root};

#[test]
fn structural_score_matches_baseline() {
    let root = workspace_root();
    let updating = std::env::var_os("UNRENDER_UPDATE_BASELINE").is_some();

    let fixtures: Vec<_> = discover(&root)
        .into_iter()
        .filter(|f| f.truth.is_some())
        .collect();
    assert!(
        !fixtures.is_empty(),
        "no truth-bearing fixtures under {}/fixtures — discovery is broken",
        root.display()
    );

    let recorded = Baseline::load(&root).expect("load baseline");
    let mut fresh = Baseline {
        note: "Recorded metrics for truth-bearing fixtures. Entries with \
               selection_ok=false are KNOWN FAILURES, recorded deliberately so \
               that fixing them registers as a visible improvement rather than \
               a silently-greening check."
            .to_string(),
        fixtures: Default::default(),
    };

    println!(
        "\n{:<26} {:>6} {:>6} {:>7} {:>7} {:>8}  vs baseline",
        "fixture", "recall", "prec", "role%", "meanIoU", "select"
    );

    let mut drifts = Vec::new();
    let mut unbaselined = Vec::new();

    for f in &fixtures {
        let ansi = f.read_ansi().expect("read fixture");
        let truth = f.read_truth().expect("read truth");
        let s = compute_score(&ansi, &truth).expect("score");

        let now = FixtureMetrics {
            recall: Some(s.recall()),
            precision: Some(s.precision()),
            role_agreement: Some(s.role_agreement()),
            mean_iou: Some(s.mean_iou),
            selection_ok: s.selection.as_ref().map(|c| c.ok),
        };

        let status = match recorded.as_ref().and_then(|b| b.fixtures.get(&f.name)) {
            Some(was) => {
                let d = diff(was, &now, &f.name);
                let label = if d.is_empty() {
                    "=".to_string()
                } else {
                    d.iter()
                        .map(|x| {
                            format!(
                                "{} {} {}->{}",
                                if x.improved { "IMPROVED" } else { "REGRESSED" },
                                x.metric,
                                x.was,
                                x.now
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                drifts.extend(d);
                label
            }
            None => {
                unbaselined.push(f.name.clone());
                "NEW (not baselined)".to_string()
            }
        };

        println!(
            "{:<26} {:>5.0}% {:>5.0}% {:>6.0}% {:>7.2} {:>8}  {}",
            f.name,
            now.recall.unwrap_or(0.0) * 100.0,
            now.precision.unwrap_or(0.0) * 100.0,
            now.role_agreement.unwrap_or(0.0) * 100.0,
            now.mean_iou.unwrap_or(0.0),
            match now.selection_ok {
                Some(true) => "ok",
                Some(false) => "MISS",
                None => "-",
            },
            status
        );

        fresh.fixtures.insert(f.name.clone(), now);
    }

    if updating {
        fresh.save(&root).expect("write baseline");
        println!("\nbaseline re-recorded at results/baseline.json");
        return;
    }

    if recorded.is_none() {
        panic!(
            "no results/baseline.json — record one with \
             `UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval`"
        );
    }

    assert!(
        unbaselined.is_empty(),
        "fixtures with no baseline entry: {}\nre-record with \
         `UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval`",
        unbaselined.join(", ")
    );

    if !drifts.is_empty() {
        let (better, worse): (Vec<_>, Vec<_>) = drifts.iter().partition(|d| d.improved);
        let mut msg = String::from("metrics moved against results/baseline.json\n");
        for d in worse.iter().chain(better.iter()) {
            msg.push_str(&format!(
                "  {} {}: {} {} -> {}\n",
                if d.improved { "IMPROVED " } else { "REGRESSED" },
                d.fixture,
                d.metric,
                d.was,
                d.now
            ));
        }
        msg.push_str(
            "\nregressions are bugs; improvements just need re-recording with \
             `UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval`",
        );
        panic!("{msg}");
    }
}
