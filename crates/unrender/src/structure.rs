//! Intra-panel content structure: table / tree / list / key-value / text.
//!
//! Once a panel's rectangle is known, the question becomes what KIND of content
//! it holds. Every detector here works off geometry and styling only -- column
//! alignment, indentation, marker glyphs, paint runs -- never off application
//! names, so nothing is tuned to a specific CLI.

use crate::ax::Node;
use crate::grid::Grid;
use crate::rects::Rect;

const TREE_CONNECTORS: [char; 8] = ['├', '└', '│', '─', '┬', '┼', '╰', '╷'];
const EXPANDERS: [char; 8] = ['▼', '▶', '▸', '▾', '►', '▽', '+', '-'];
const BULLETS: [char; 6] = ['•', '*', '‣', '◦', '·', '>'];

struct RowInfo {
    y: u16,
    text: String,
    painted: bool,
    /// Longest contiguous off-base paint run, kept so selection can also be
    /// found by comparing rows against each other.
    run: usize,
}

/// The background a panel is *normally* drawn on. Themed TUIs (Textual, and
/// anything on a dark theme) paint every cell, so "has a background colour"
/// says nothing. What marks a selection is differing from the local norm.
type PaintKey = (crate::grid::Color, bool);

fn modal_bg(g: &Grid, area: Rect) -> PaintKey {
    use std::collections::HashMap;
    let mut counts: HashMap<PaintKey, usize> = HashMap::new();
    for y in area.y..=area.y1().min(g.h - 1) {
        for x in area.x..=area.x1().min(g.w - 1) {
            *counts.entry(g.at(x, y).style.paint_key()).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, n)| *n)
        .map(|(c, _)| c)
        .unwrap_or((crate::grid::Color::Default, false))
}

/// Selection expressed as a text marker rather than styling: radio buttons,
/// checkboxes and cursor arrows. prompt_toolkit's RadioList, for one, applies
/// no styling whatsoever to the chosen row -- the only signal is `(*)` vs `( )`.
fn marker_selected(text: &str) -> bool {
    let t = text.trim_start();
    const ON: [&str; 6] = ["(*)", "[x]", "[X]", "[*]", "(•)", "(●)"];
    if ON.iter().any(|m| t.starts_with(m)) {
        return true;
    }
    matches!(t.chars().next(), Some('>' | '▶' | '▸' | '→' | '●' | '◉' | '✓'))
}

fn rows_of(g: &Grid, area: Rect, base: PaintKey) -> Vec<RowInfo> {
    let mut out = Vec::new();
    for y in area.y..=area.y1().min(g.h - 1) {
        let text = g.row_text(y, area.x, area.x1());
        // A contiguous run, not a majority of the row: ncurses `dialog` and
        // many menus highlight only the item label, leaving the rest of the
        // row on the panel background.
        let mut run = 0usize;
        let mut best = 0usize;
        for x in area.x..=area.x1().min(g.w - 1) {
            if g.at(x, y).style.paint_key() != base {
                run += 1;
                best = best.max(run);
            } else {
                run = 0;
            }
        }
        let threshold = ((area.w as usize) / 4).max(4);
        // An empty row is never a selected item, however it is painted --
        // blank filler below a menu is often a solid block of panel colour,
        // and letting it count would mask the real selection.
        let has_text = !text.trim().is_empty();
        out.push(RowInfo {
            painted: has_text && (best >= threshold || marker_selected(&text)),
            run: if has_text { best } else { 0 },
            y,
            text,
        });
    }
    out
}

/// Fallback selection rule: the selected row is the one that stands out from
/// its siblings. ncurses `dialog` highlights only the item label (7 cells)
/// against rows that already colour their hotkey (4 cells), so no absolute
/// threshold separates them -- but a single clear outlier does.
fn mark_outlier_selection(rows: &mut [RowInfo]) {
    if rows.iter().any(|r| r.painted) {
        return;
    }
    let runs: Vec<usize> = rows
        .iter()
        .filter(|r| !r.text.trim().is_empty())
        .map(|r| r.run)
        .collect();
    if runs.len() < 3 {
        return;
    }
    let mut sorted = runs.clone();
    sorted.sort_unstable();
    let median = sorted[sorted.len() / 2];
    let max = *sorted.last().unwrap();
    if max < median + 3 || runs.iter().filter(|&&r| r == max).count() != 1 {
        return;
    }
    for r in rows.iter_mut() {
        if r.run == max && !r.text.trim().is_empty() {
            r.painted = true;
        }
    }
}

fn content_rows(rows: &[RowInfo]) -> Vec<&RowInfo> {
    rows.iter().filter(|r| !r.text.trim().is_empty()).collect()
}

/// Columns that are blank across (nearly) every content row are column gaps.
/// Real prose almost never produces a strictly-blank column two cells wide
/// across several lines, which is what makes this a usable table discriminator.
fn separator_columns(g: &Grid, area: Rect, rows: &[&RowInfo]) -> Vec<bool> {
    let mut sep = Vec::with_capacity(area.w as usize);
    for x in area.x..=area.x1().min(g.w - 1) {
        let mut blank = 0usize;
        let mut bars = 0usize;
        for r in rows {
            let c = g.at(x, r.y);
            if c.is_blank() {
                blank += 1;
            } else if c.ch() == '│' || c.ch() == '|' {
                bars += 1;
            }
        }
        let n = rows.len().max(1);
        sep.push(blank as f64 / n as f64 >= 0.9 || bars as f64 / n as f64 >= 0.9);
    }
    sep
}

fn spans_from_sep(sep: &[bool]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start: Option<usize> = None;
    for (i, &s) in sep.iter().enumerate() {
        if s {
            if let Some(st) = start.take() {
                spans.push((st, i - 1));
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }
    if let Some(st) = start {
        spans.push((st, sep.len() - 1));
    }
    spans
}

fn max_gap(sep: &[bool]) -> usize {
    let mut best = 0;
    let mut cur = 0;
    for &s in sep {
        if s {
            cur += 1;
            best = best.max(cur);
        } else {
            cur = 0;
        }
    }
    best
}

fn style_differs(g: &Grid, area: Rect, a: u16, b: u16) -> bool {
    let f = |y: u16| {
        (area.x..=area.x1().min(g.w - 1))
            .filter(|&x| !g.at(x, y).is_blank())
            .map(|x| {
                let s = g.at(x, y).style;
                (s.bold, s.underline, s.inverse, s.fg)
            })
            .next()
    };
    match (f(a), f(b)) {
        (Some(x), Some(y)) => x != y,
        _ => false,
    }
}

fn strip_expander(s: &str) -> (String, bool) {
    let t = s.trim_start();
    let mut chars = t.chars();
    if let Some(c) = chars.next() {
        if EXPANDERS.contains(&c) {
            let expanded = matches!(c, '▼' | '▾' | '▽' | '-');
            return (chars.as_str().trim().to_string(), expanded);
        }
    }
    (t.to_string(), false)
}

fn try_tree(g: &Grid, area: Rect, rows: &[&RowInfo]) -> Option<Node> {
    let connected = rows
        .iter()
        .filter(|r| r.text.chars().take(12).any(|c| c == '├' || c == '└'))
        .count();
    if connected < 2 {
        return None;
    }
    let mut node = Node::new("tree", area);
    for r in rows {
        // Depth is how far the label sits past the connector run. This must be
        // measured in CHARACTERS -- box-drawing glyphs are 3 bytes each, so a
        // byte offset silently triples the depth.
        let chars: Vec<char> = r.text.chars().collect();
        let p = chars
            .iter()
            .position(|c| !c.is_whitespace() && !TREE_CONNECTORS.contains(c))
            .unwrap_or(0);
        let depth = p / 4;
        let label: String = chars[p..].iter().collect();
        let label = label.trim();
        if label.is_empty() {
            continue;
        }
        let (label, expanded) = strip_expander(label);
        let mut item = Node::new("treeitem", Rect { x: area.x, y: r.y, w: area.w, h: 1 })
            .valued(label);
        item.states.push(format!("depth={depth}"));
        if expanded {
            item.states.push("expanded".into());
        }
        if r.painted {
            item.states.push("selected".into());
        }
        node.children.push(item);
    }
    if node.children.len() < 2 {
        return None;
    }
    let _ = g;
    Some(node)
}

fn try_table(g: &Grid, area: Rect, rows: &[&RowInfo]) -> Option<Node> {
    if rows.len() < 2 {
        return None;
    }
    let sep = separator_columns(g, area, rows);
    let spans = spans_from_sep(&sep);
    if spans.len() < 2 {
        return None;
    }
    // A one-cell gap is a word space; a real column gap is wider (or an
    // explicit rule). Without this, ordinary prose parses as a table.
    let has_bar = (area.x..=area.x1().min(g.w - 1))
        .any(|x| rows.iter().filter(|r| g.at(x, r.y).ch() == '│').count() >= rows.len() - 1);
    if max_gap(&sep) < 2 && !has_bar {
        return None;
    }
    // Most rows must actually populate 2+ columns.
    let populated = rows
        .iter()
        .filter(|r| {
            spans
                .iter()
                .filter(|(a, b)| {
                    (*a..=*b).any(|i| {
                        let x = area.x + i as u16;
                        x < g.w && !g.at(x, r.y).is_blank()
                    })
                })
                .count()
                >= 2
        })
        .count();
    if (populated as f64) < 0.6 * rows.len() as f64 {
        return None;
    }

    let mut node = Node::new("table", area);
    // The first row is a header when it is styled unlike the rest -- unless it
    // is styled unlike the rest because it is *selected*. A menu whose first
    // entry is highlighted would otherwise lose that entry into a phantom
    // header row.
    let header_idx = if rows.len() > 2
        && style_differs(g, area, rows[0].y, rows[1].y)
        && !rows[0].painted
    {
        Some(0)
    } else {
        None
    };

    for (i, r) in rows.iter().enumerate() {
        let role = if Some(i) == header_idx { "rowheader" } else { "row" };
        let mut row_node = Node::new(role, Rect { x: area.x, y: r.y, w: area.w, h: 1 });
        for (a, b) in &spans {
            let x0 = area.x + *a as u16;
            let x1 = area.x + *b as u16;
            let text = g.row_text(r.y, x0, x1).trim().to_string();
            row_node.children.push(
                Node::new("cell", Rect { x: x0, y: r.y, w: x1 - x0 + 1, h: 1 }).valued(text),
            );
        }
        // The header is off-base by definition -- that is how it was found --
        // so painting must not also read as a selection there.
        if r.painted && Some(i) != header_idx {
            row_node.states.push("selected".into());
        }
        node.children.push(row_node);
    }
    Some(node)
}

fn try_keyvalue(_g: &Grid, area: Rect, rows: &[&RowInfo]) -> Option<Node> {
    let mut pairs = Vec::new();
    for r in rows {
        let t = r.text.trim();
        if let Some(i) = t.find(':') {
            let (k, v) = t.split_at(i);
            let v = v[1..].trim();
            if !k.is_empty() && !v.is_empty() && k.len() <= 24 && !k.contains("  ") {
                pairs.push((r.y, k.trim().to_string(), v.to_string()));
                continue;
            }
        }
        return None;
    }
    if pairs.len() < 2 {
        return None;
    }
    let mut node = Node::new("list", area).named(Some("properties".into()));
    for (y, k, v) in pairs {
        node.children.push(
            Node::new("property", Rect { x: area.x, y, w: area.w, h: 1 })
                .named(Some(k))
                .valued(v),
        );
    }
    Some(node)
}

fn try_list(_g: &Grid, area: Rect, rows: &[&RowInfo]) -> Option<Node> {
    if rows.len() < 2 {
        return None;
    }
    let marked = rows
        .iter()
        .filter(|r| {
            let t = r.text.trim_start();
            let c = t.chars().next().unwrap_or(' ');
            BULLETS.contains(&c)
                || EXPANDERS.contains(&c)
                || t.chars().next().map_or(false, |c| c.is_ascii_digit())
                    && (t.contains('.') || t.contains(')'))
        })
        .count();
    // Either explicit markers, or short non-wrapping entries -- which is what a
    // plain framework List widget produces (ratatui List, bubbletea list).
    let widths: Vec<usize> = rows.iter().map(|r| r.text.trim_end().len()).collect();
    let avg_fill =
        widths.iter().sum::<usize>() as f64 / (widths.len() as f64 * area.w.max(1) as f64);
    let marker_ratio = marked as f64 / rows.len() as f64;
    if marker_ratio < 0.6 && avg_fill > 0.75 {
        return None;
    }

    let mut node = Node::new("list", area);
    for r in rows {
        let (label, _) = strip_expander(r.text.trim());
        let mut item = Node::new("listitem", Rect { x: area.x, y: r.y, w: area.w, h: 1 })
            .valued(label);
        if r.painted {
            item.states.push("selected".into());
        }
        node.children.push(item);
    }
    Some(node)
}

fn percent_in(text: &str) -> Option<String> {
    let bytes: Vec<char> = text.chars().collect();
    for (i, c) in bytes.iter().enumerate() {
        if *c == '%' {
            let mut j = i;
            while j > 0 && bytes[j - 1].is_ascii_digit() {
                j -= 1;
            }
            if j < i {
                return Some(bytes[j..=i].iter().collect());
            }
        }
    }
    None
}

/// Classify the contents of one panel interior.
pub fn analyze(g: &Grid, area: Rect) -> Vec<Node> {
    if area.w == 0 || area.h == 0 {
        return Vec::new();
    }
    let base = modal_bg(g, area);
    let mut rows = rows_of(g, area, base);
    mark_outlier_selection(&mut rows);
    let content = content_rows(&rows);
    if content.is_empty() {
        return Vec::new();
    }

    // A bar of paint plus a percentage is the universal gauge idiom.
    let joined = content
        .iter()
        .map(|r| r.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let painted_cells = (area.x..=area.x1().min(g.w - 1))
        .flat_map(|x| (area.y..=area.y1().min(g.h - 1)).map(move |y| (x, y)))
        .filter(|&(x, y)| g.at(x, y).style.paint_key() != base)
        .count();
    // Gauges are drawn one of two ways: a painted background run, or a run of
    // block-element glyphs in the foreground. ratatui's Gauge uses the latter,
    // so testing background alone silently misses it.
    let block_cells = (area.x..=area.x1().min(g.w - 1))
        .flat_map(|x| (area.y..=area.y1().min(g.h - 1)).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            let c = g.at(x, y).ch() as u32;
            (0x2580..=0x259F).contains(&c) || matches!(g.at(x, y).ch(), '#' | '=')
        })
        .count();
    let bar_like =
        painted_cells >= (area.w as usize) / 4 || block_cells >= (area.w as usize) / 5;
    if bar_like && content.len() <= 3 {
        let value = percent_in(&joined).unwrap_or_else(|| {
            let filled = block_cells as f64 / (area.w.max(1) as f64 * content.len().max(1) as f64);
            format!("{:.0}%", filled * 100.0)
        });
        return vec![Node::new("progressbar", area).valued(value)];
    }

    if let Some(n) = try_tree(g, area, &content) {
        return vec![n];
    }
    if let Some(n) = try_table(g, area, &content) {
        return vec![n];
    }
    if let Some(n) = try_keyvalue(g, area, &content) {
        return vec![n];
    }
    if let Some(n) = try_list(g, area, &content) {
        return vec![n];
    }

    // Fallback: a block of prose / log output.
    let text = content
        .iter()
        .map(|r| r.text.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let role = if content.len() > 1 { "log" } else { "text" };
    vec![Node::new(role, area).valued(text)]
}
