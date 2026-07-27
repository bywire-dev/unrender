//! Shared unrender pipeline, so the CLI and the scorer analyse screens
//! through exactly the same code path.

pub mod ax;
pub mod emit;
pub mod grid;
pub mod rects;
pub mod regions;
pub mod structure;

use ax::Node;
use grid::Grid;
use rects::{BoxFrame, Rect};

fn build_frame_tree(g: &Grid, frames: &[BoxFrame]) -> Vec<Node> {
    let mut parent: Vec<Option<usize>> = vec![None; frames.len()];
    for i in 0..frames.len() {
        let mut best: Option<usize> = None;
        for j in 0..frames.len() {
            if i == j
                || !frames[j].rect.contains(&frames[i].rect)
                || frames[j].rect == frames[i].rect
            {
                continue;
            }
            best = match best {
                None => Some(j),
                Some(b) if frames[j].rect.area() < frames[b].rect.area() => Some(j),
                other => other,
            };
        }
        parent[i] = best;
    }

    fn make(g: &Grid, frames: &[BoxFrame], parent: &[Option<usize>], i: usize) -> Node {
        let f = &frames[i];
        let kids: Vec<usize> = (0..frames.len())
            .filter(|&j| parent[j] == Some(i))
            .collect();
        let mut node = Node::new("panel", f.rect).named(f.title.clone());
        if kids.is_empty() {
            if let Some(inner) = f.rect.interior() {
                node.children = structure::analyze(g, inner);
            }
        } else {
            let mut kid_nodes: Vec<Node> =
                kids.iter().map(|&j| make(g, frames, parent, j)).collect();
            kid_nodes.sort_by_key(|n| (n.rect[1], n.rect[0]));
            node.children = kid_nodes;
        }
        // A bordered box holding exactly one structure IS that structure --
        // the frame is chrome. Collapsing keeps the tree shallow and stops the
        // wrapper from shadowing the real role.
        if node.children.len() == 1 {
            let only = &node.children[0];
            if matches!(
                only.role.as_str(),
                "table" | "list" | "tree" | "log" | "text" | "progressbar"
            ) {
                let mut c = node.children.remove(0);
                c.name = node.name.clone();
                c.rect = node.rect;
                return c;
            }
        }
        node
    }

    let mut roots: Vec<Node> = (0..frames.len())
        .filter(|&i| parent[i].is_none())
        .map(|i| make(g, frames, &parent, i))
        .collect();
    roots.sort_by_key(|n| (n.rect[1], n.rect[0]));
    roots
}

fn uncovered_bands(g: &Grid, roots: &[Node]) -> Vec<(u16, u16)> {
    let mut covered = vec![false; g.h as usize];
    for n in roots {
        let r = n.rect_of();
        for y in r.y..=r.y1().min(g.h - 1) {
            covered[y as usize] = true;
        }
    }
    let mut bands = Vec::new();
    let mut start: Option<u16> = None;
    for y in 0..g.h {
        let has_content = (0..g.w).any(|x| !g.at(x, y).is_blank());
        if !covered[y as usize] && has_content {
            if start.is_none() {
                start = Some(y);
            }
        } else if let Some(s) = start.take() {
            bands.push((s, y - 1));
        }
    }
    if let Some(s) = start {
        bands.push((s, g.h - 1));
    }
    bands
}

/// Full pipeline: ANSI dump bytes -> accessibility tree.
pub fn build_tree(bytes: &[u8]) -> Node {
    let g = Grid::from_ansi_autosize(bytes);
    let frames = rects::find_frames(&g);
    let mut roots = build_frame_tree(&g, &frames);
    let painted = regions::find_painted(&g);

    for (y0, y1) in uncovered_bands(&g, &roots) {
        let area = Rect {
            x: 0,
            y: y0,
            w: g.w,
            h: y1 - y0 + 1,
        };
        let bar = painted
            .iter()
            .any(|p| regions::is_bar(&g, &p.rect) && p.rect.y <= y1 && p.rect.y1() >= y0);
        if bar || area.h == 1 {
            let role = if y0 == 0 { "heading" } else { "statusbar" };
            let text = (y0..=y1)
                .map(|y| g.row_text(y, 0, g.w - 1).trim().to_string())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            roots.push(Node::new(role, area).valued(text));
        } else {
            roots.extend(structure::analyze(&g, area));
        }
    }
    roots.sort_by_key(|n| (n.rect[1], n.rect[0]));

    let mut root = Node::new(
        "application",
        Rect {
            x: 0,
            y: 0,
            w: g.w,
            h: g.h,
        },
    );
    root.children = roots;
    root
}

/// Strip escape sequences, for token comparisons against the plain dump.
pub fn strip_ansi(raw: &str) -> String {
    raw.lines()
        .map(|l| {
            let mut s = String::new();
            let mut chars = l.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '\x1b' {
                    if chars.peek() == Some(&'[') {
                        chars.next();
                        for c2 in chars.by_ref() {
                            if ('\x40'..='\x7e').contains(&c2) {
                                break;
                            }
                        }
                    }
                    continue;
                }
                s.push(c);
            }
            s
        })
        .collect::<Vec<_>>()
        .join("\n")
}
