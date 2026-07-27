//! Serialisation of the accessibility tree.
//!
//! Three encodings, because "which encoding is cheapest" is an empirical
//! question the spike is supposed to answer rather than assume. TOON's win is
//! concentrated in uniform arrays (tables); a heterogeneous UI tree may well be
//! cheaper as terse indentation. We measure instead of guessing.

use crate::ax::Node;

fn esc(s: &str) -> String {
    s.replace('\n', "\\n")
}

/// Indentation-based. Geometry is emitted as `@x,y,w,h` so the agent can target
/// a click/keystroke, and states as `[selected]`.
pub fn compact(node: &Node, depth: usize, out: &mut String) {
    compact_opt(node, depth, true, out)
}

/// Same, with geometry optional. Coordinates are by far the most expensive part
/// of the encoding, and they are only worth paying for when the agent needs to
/// aim a mouse -- keyboard-driven navigation never reads them. Measuring both
/// separates "the tree is compact" from "we dropped the rects".
pub fn compact_opt(node: &Node, depth: usize, geometry: bool, out: &mut String) {
    let pad = "  ".repeat(depth);
    out.push_str(&pad);
    out.push_str(&node.role);
    if let Some(n) = &node.name {
        out.push_str(&format!(" \"{}\"", esc(n)));
    }
    if !node.states.is_empty() {
        out.push_str(&format!(" [{}]", node.states.join(",")));
    }
    if geometry {
        out.push_str(&format!(
            " @{},{},{},{}",
            node.rect[0], node.rect[1], node.rect[2], node.rect[3]
        ));
    }
    if let Some(v) = &node.value {
        out.push_str(&format!(" {}", esc(v)));
    }
    out.push('\n');
    for c in &node.children {
        compact_opt(c, depth + 1, geometry, out);
    }
}

/// TOON-flavoured. Tables collapse to a header line plus one comma-separated
/// line per row, which is where this encoding actually pays for itself.
pub fn toon(node: &Node, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);

    if node.role == "table" && !node.children.is_empty() {
        let ncols = node
            .children
            .iter()
            .map(|r| r.children.len())
            .max()
            .unwrap_or(0);
        let header: Vec<String> = node
            .children
            .iter()
            .find(|r| r.role == "rowheader")
            .map(|r| {
                r.children
                    .iter()
                    .map(|c| c.value.clone().unwrap_or_default())
                    .collect()
            })
            .unwrap_or_else(|| (0..ncols).map(|i| format!("c{i}")).collect());
        let body: Vec<&Node> = node
            .children
            .iter()
            .filter(|r| r.role != "rowheader")
            .collect();
        out.push_str(&format!(
            "{pad}{} {}[{}]{{{}}}:\n",
            node.role,
            node.name.clone().unwrap_or_default(),
            body.len(),
            header.join(",")
        ));
        for r in body {
            let cells: Vec<String> = r
                .children
                .iter()
                .map(|c| c.value.clone().unwrap_or_default())
                .collect();
            let mark = if r.states.iter().any(|s| s == "selected") {
                "*"
            } else {
                " "
            };
            out.push_str(&format!("{pad}  {mark}{}\n", cells.join(",")));
        }
        return;
    }

    if (node.role == "list" || node.role == "tree") && !node.children.is_empty() {
        out.push_str(&format!(
            "{pad}{} {}[{}]:\n",
            node.role,
            node.name.clone().unwrap_or_default(),
            node.children.len()
        ));
        for c in &node.children {
            let mark = if c.states.iter().any(|s| s == "selected") {
                "*"
            } else {
                " "
            };
            let depth_tag = c
                .states
                .iter()
                .find(|s| s.starts_with("depth="))
                .map(|s| format!("{}:", &s[6..]))
                .unwrap_or_default();
            out.push_str(&format!(
                "{pad}  {mark}{depth_tag}{}\n",
                esc(c.value.as_deref().unwrap_or(""))
            ));
        }
        return;
    }

    out.push_str(&format!("{pad}{}", node.role));
    if let Some(n) = &node.name {
        out.push_str(&format!(" {}", esc(n)));
    }
    if let Some(v) = &node.value {
        out.push_str(&format!(": {}", esc(v)));
    }
    out.push('\n');
    for c in &node.children {
        toon(c, depth + 1, out);
    }
}

/// Byte and character size of an encoding.
///
/// Deliberately *not* a token count. Token counts are model-specific and must
/// come from the provider (`POST /v1/messages/count_tokens`) — a local BPE
/// approximation built for a different model family undercounts Claude by
/// 15-20% on prose and considerably more on the box-drawing and code-like
/// content a terminal dump is made of. `xtask report` does the real counting;
/// this is only a cheap offline proxy for eyeballing relative size.
pub fn size_of(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}
