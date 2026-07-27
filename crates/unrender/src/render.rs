//! Tree → grid: the inverse of [`crate::build_tree`].
//!
//! This exists to measure **information loss**, not to reproduce a screenshot.
//! `build_tree` deliberately throws things away — box-drawing chrome is
//! collapsed once a panel is recognised, colour is discarded once it has been
//! read as selection state — so a perfect round trip is neither achievable nor
//! desirable. What the diff answers is narrower and more useful: *of the
//! content that was on screen, how much survived the translation?*
//!
//! Read the resulting score together with the structural score, never alone. A
//! tree that flattened an entire screen into one `log` block would score ~100%
//! here while having destroyed every bit of structure — fidelity measures
//! preservation, and only IoU measures correctness.

use crate::ax::Node;
use crate::grid::{Grid, Style};

/// Reconstruct a grid from a tree. Dimensions come from the root's rect, which
/// `build_tree` always sets to the full screen.
pub fn render(root: &Node) -> Grid {
    let w = root.rect[2];
    let h = root.rect[3];
    let mut g = Grid::blank(w, h);
    place(root, &mut g);
    g
}

fn rect_of(n: &Node) -> (u16, u16, u16, u16) {
    (n.rect[0], n.rect[1], n.rect[2], n.rect[3])
}

fn max_x(n: &Node) -> u16 {
    let (x, _, w, _) = rect_of(n);
    x.saturating_add(w).saturating_sub(1)
}

fn place(node: &Node, g: &mut Grid) {
    match node.role.as_str() {
        "table" => render_table(node, g),
        "list" | "tree" => render_list_like(node, g),
        "progressbar" => render_progressbar(node, g),
        "log" | "text" | "statusbar" | "heading" => render_text_block(node, g),
        // `panel` and `application` are pure containers: their chrome was
        // collapsed during parsing, so there is nothing of their own to draw.
        _ => {
            for c in &node.children {
                place(c, g);
            }
        }
    }
}

/// Cells carry their own column extents, recovered from the original column
/// gaps, so a table reconstructs at its true horizontal positions rather than
/// being re-laid-out by a guess.
fn render_table(node: &Node, g: &mut Grid) {
    for row in &node.children {
        let (_, ry, _, _) = rect_of(row);
        if row.children.is_empty() {
            if let Some(v) = &row.value {
                let (rx, _, _, _) = rect_of(row);
                g.write_str(rx, ry, max_x(row), v, Style::default());
            }
            continue;
        }
        for cell in &row.children {
            let (cx, cy, _, _) = rect_of(cell);
            if let Some(v) = &cell.value {
                g.write_str(cx, cy, max_x(cell), v, Style::default());
            }
        }
    }
}

fn render_list_like(node: &Node, g: &mut Grid) {
    for item in &node.children {
        let (ix, iy, _, _) = rect_of(item);
        let Some(v) = &item.value else { continue };
        // Tree depth was recorded as a state rather than as leading whitespace,
        // because the original used box-drawing connectors we do not rebuild.
        // Re-indent so the shape survives even though the glyphs do not.
        let depth = item
            .states
            .iter()
            .find_map(|s| s.strip_prefix("depth="))
            .and_then(|d| d.parse::<u16>().ok())
            .unwrap_or(0);
        let x = ix.saturating_add(depth.saturating_mul(4));
        g.write_str(x, iy, max_x(item), v, Style::default());
    }
}

/// A gauge's bar is drawn with block glyphs or paint, neither of which the tree
/// keeps — only the percentage survives parsing, so only the percentage is
/// redrawn. Gauge-heavy screens therefore score low on fidelity by design.
fn render_progressbar(node: &Node, g: &mut Grid) {
    let (x, y, _, _) = rect_of(node);
    if let Some(v) = &node.value {
        g.write_str(x, y, max_x(node), v, Style::default());
    }
}

fn render_text_block(node: &Node, g: &mut Grid) {
    let (x, y, _, h) = rect_of(node);
    let Some(v) = &node.value else { return };
    for (i, line) in v.lines().enumerate() {
        let i = i as u16;
        if i >= h.max(1) {
            break;
        }
        g.write_str(x, y.saturating_add(i), max_x(node), line, Style::default());
    }
}

/// Cells that are pure decoration: box-drawing frames and the block glyphs a
/// gauge is painted with.
///
/// These are dropped on purpose — recognising a border is precisely how a
/// panel gets identified, after which redrawing it would be busywork. Counting
/// them as lost content makes the metric measure the wrong thing entirely: on
/// `dialog`, 81% of the non-blank cells are box-drawing, so a reconstruction
/// that preserved every word still scored 13%.
///
/// Only unambiguous glyphs count. ASCII `-` and `|` are excluded because they
/// are far more often real text than chrome.
fn is_chrome(ch: char) -> bool {
    // Box Drawing (2500-257F) runs straight into Block Elements (2580-259F).
    matches!(ch as u32, 0x2500..=0x259F)
}

/// How much of the original screen survived the round trip.
#[derive(Debug, Clone, Copy)]
pub struct Fidelity {
    /// Fraction of all cells whose character matches.
    ///
    /// Inflated by whitespace — most screens are mostly blank, so an empty
    /// reconstruction still scores high. Kept for completeness only.
    pub overall_match: f64,
    /// Fraction of non-blank cells that match, chrome included.
    ///
    /// Dominated by how much box-drawing a screen contains, which is a fact
    /// about the app's style rather than about unrender. Kept so the
    /// deliberate chrome loss stays visible instead of being quietly excluded.
    pub nonblank_match: f64,
    /// **The honest measure**: fraction of non-blank, non-chrome cells that
    /// match. This is content preservation with the intentional losses
    /// factored out.
    pub content_match: f64,
    pub total_cells: usize,
    pub nonblank_cells: usize,
    /// Non-blank cells that were pure decoration, and so are excluded from
    /// `content_match`. A large number here means a heavily-framed UI, not a
    /// failure.
    pub chrome_cells: usize,
}

/// Compare two grids by **character content only**.
///
/// Style is deliberately ignored: colour is consumed during parsing to infer
/// selection and headers, so a style diff would only re-measure a known and
/// intentional loss.
pub fn compare(original: &Grid, rendered: &Grid) -> Fidelity {
    let w = original.w.min(rendered.w);
    let h = original.h.min(rendered.h);
    let total = (original.w as usize) * (original.h as usize);

    let mut matched = 0usize;
    let mut nonblank = 0usize;
    let mut nonblank_matched = 0usize;
    let mut chrome = 0usize;
    let mut content = 0usize;
    let mut content_matched = 0usize;

    for y in 0..original.h {
        for x in 0..original.w {
            let o = original.at(x, y);
            let o_blank = o.is_blank();
            let o_chrome = !o_blank && is_chrome(o.ch());
            if !o_blank {
                nonblank += 1;
                if o_chrome {
                    chrome += 1;
                } else {
                    content += 1;
                }
            }
            // Anything outside the reconstruction counts as blank, so a
            // too-small render is penalised rather than silently skipped.
            let r_text = if x < w && y < h {
                rendered.at(x, y).text.as_str()
            } else {
                " "
            };
            if o.text.trim() == r_text.trim() {
                matched += 1;
                if !o_blank {
                    nonblank_matched += 1;
                    if !o_chrome {
                        content_matched += 1;
                    }
                }
            }
        }
    }

    Fidelity {
        overall_match: frac(matched, total),
        nonblank_match: frac(nonblank_matched, nonblank),
        content_match: frac(content_matched, content),
        total_cells: total,
        nonblank_cells: nonblank,
        chrome_cells: chrome,
    }
}

fn frac(num: usize, den: usize) -> f64 {
    if den == 0 {
        1.0
    } else {
        num as f64 / den as f64
    }
}
