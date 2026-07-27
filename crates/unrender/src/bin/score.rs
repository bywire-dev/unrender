//! Score an inferred accessibility tree against a framework's real widget tree.
//!
//! The instrumented corpus apps dump the layout THEY computed. Matching is by
//! geometry (IoU) rather than by name, so a node only counts as found if the
//! unrenderer put it in the right place on screen -- which is the actual claim
//! under test.

use serde::Deserialize;
use unrender::ax::Node;
use unrender::build_tree;
use unrender::rects::Rect;

#[derive(Deserialize)]
struct TruthRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct TruthNode {
    role: String,
    #[serde(default)]
    name: String,
    rect: TruthRect,
}

#[derive(Deserialize)]
struct Truth {
    app: String,
    #[serde(default)]
    selected: Option<usize>,
    nodes: Vec<TruthNode>,
}

/// Roles that mean the same thing across the two vocabularies. `panel` is the
/// unrenderer's generic container, so it is allowed to stand in for any
/// container-ish truth role -- geometry is what is being scored.
fn role_matches(truth: &str, inferred: &str) -> bool {
    if truth == inferred {
        return true;
    }
    let equiv: &[(&str, &[&str])] = &[
        ("text", &["log", "text", "paragraph"]),
        ("log", &["text", "log"]),
        ("heading", &["heading", "text", "statusbar"]),
        ("statusbar", &["statusbar", "text", "heading"]),
        ("table", &["table"]),
        ("list", &["list"]),
        ("tree", &["tree"]),
        ("progressbar", &["progressbar"]),
        ("application", &["application"]),
    ];
    equiv
        .iter()
        .find(|(t, _)| *t == truth)
        .map(|(_, ok)| ok.contains(&inferred))
        .unwrap_or(false)
}

const IOU_THRESHOLD: f64 = 0.5;

fn main() {
    let mut files: Vec<String> = std::env::args().skip(1).collect();
    if files.is_empty() {
        eprintln!("usage: score <capture.ansi> ...  (expects a sibling <name>.truth.json)");
        std::process::exit(2);
    }
    files.sort();

    let mut tot_truth = 0usize;
    let mut tot_matched = 0usize;
    let mut tot_inferred = 0usize;
    let mut tot_role_ok = 0usize;
    let mut ious: Vec<f64> = Vec::new();

    println!(
        "{:<22} {:>6} {:>6} {:>7} {:>7} {:>7} {:>8}",
        "capture", "truth", "infer", "matched", "recall", "role%", "meanIoU"
    );

    for f in &files {
        let truth_path = f.replace(".ansi", ".truth.json");
        let Ok(truth_raw) = std::fs::read_to_string(&truth_path) else {
            continue;
        };
        let truth: Truth = match serde_json::from_str(&truth_raw) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{truth_path}: {e}");
                continue;
            }
        };
        let bytes = std::fs::read(f).expect("read capture");
        let tree = build_tree(&bytes);
        let mut flat: Vec<&Node> = Vec::new();
        tree.flatten(&mut flat);
        // Score container-level nodes only; rows/cells have no counterpart in
        // the framework's widget tree.
        let candidates: Vec<&&Node> = flat
            .iter()
            .filter(|n| {
                !matches!(
                    n.role.as_str(),
                    "row" | "rowheader" | "cell" | "listitem" | "treeitem" | "property"
                )
            })
            .collect();

        let mut matched = 0usize;
        let mut role_ok = 0usize;
        let mut used = vec![false; candidates.len()];
        let mut local_ious = Vec::new();

        for t in &truth.nodes {
            let tr = Rect { x: t.rect.x, y: t.rect.y, w: t.rect.w, h: t.rect.h };
            let mut best = (0.0f64, usize::MAX);
            for (i, c) in candidates.iter().enumerate() {
                if used[i] {
                    continue;
                }
                let iou = tr.iou(&c.rect_of());
                if iou > best.0 {
                    best = (iou, i);
                }
            }
            if best.0 >= IOU_THRESHOLD && best.1 != usize::MAX {
                used[best.1] = true;
                matched += 1;
                local_ious.push(best.0);
                if role_matches(&t.role, &candidates[best.1].role) {
                    role_ok += 1;
                }
            }
        }

        let n_truth = truth.nodes.len();
        let recall = if n_truth > 0 { matched as f64 / n_truth as f64 } else { 0.0 };
        let role_pct = if matched > 0 { role_ok as f64 / matched as f64 } else { 0.0 };
        let mean_iou = if local_ious.is_empty() {
            0.0
        } else {
            local_ious.iter().sum::<f64>() / local_ious.len() as f64
        };

        let name = std::path::Path::new(f)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        println!(
            "{:<22} {:>6} {:>6} {:>7} {:>6.0}% {:>6.0}% {:>8.2}",
            name,
            n_truth,
            candidates.len(),
            matched,
            recall * 100.0,
            role_pct * 100.0,
            mean_iou
        );

        tot_truth += n_truth;
        tot_matched += matched;
        tot_inferred += candidates.len();
        tot_role_ok += role_ok;
        ious.extend(local_ious);

        // Selection agreement: does the tree report the row the app says is
        // selected? This is what makes the tree actionable rather than pretty.
        if let Some(sel) = truth.selected {
            let mut sel_rows: Vec<usize> = Vec::new();
            for n in &flat {
                if matches!(n.role.as_str(), "table" | "list" | "tree") {
                    let body: Vec<&Node> =
                        n.children.iter().filter(|c| c.role != "rowheader").collect();
                    for (i, c) in body.iter().enumerate() {
                        if c.states.iter().any(|s| s == "selected") {
                            sel_rows.push(i);
                        }
                    }
                }
            }
            let ok = sel_rows.contains(&sel);
            println!(
                "  selection: app says index {sel}, tree says {:?} -> {}",
                sel_rows,
                if ok { "MATCH" } else { "MISS" }
            );
        }
        let _ = &truth.app;
    }

    let recall = if tot_truth > 0 { tot_matched as f64 / tot_truth as f64 } else { 0.0 };
    let precision = if tot_inferred > 0 { tot_matched as f64 / tot_inferred as f64 } else { 0.0 };
    let role_pct = if tot_matched > 0 { tot_role_ok as f64 / tot_matched as f64 } else { 0.0 };
    let mean_iou = if ious.is_empty() { 0.0 } else { ious.iter().sum::<f64>() / ious.len() as f64 };
    println!(
        "\nTOTAL truth={tot_truth} inferred={tot_inferred} matched={tot_matched} \
         recall={:.0}% precision={:.0}% role_agreement={:.0}% mean_iou={:.2}",
        recall * 100.0,
        precision * 100.0,
        role_pct * 100.0,
        mean_iou
    );
}
