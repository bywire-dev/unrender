//! Borderless structure recovery via painted-background segmentation.
//!
//! A large fraction of modern TUIs draw no box at all -- fzf, bubbletea lists,
//! prompt_toolkit completion menus, most status bars, and every "selected row"
//! highlight. Their structure is expressed purely as runs of background colour
//! (or reverse video). Box-drawing detection alone would be blind to all of it,
//! so this is a first-class path, not a fallback.

use crate::grid::{Color, Grid};
use crate::rects::Rect;

#[derive(Debug, Clone)]
pub struct PaintRegion {
    pub rect: Rect,
    pub color: (Color, bool),
    /// Rows covered, so a one-row region can be recognised as a selection.
    pub rows: Vec<u16>,
}

/// Connected components of equally-painted cells (4-connectivity).
pub fn find_painted(g: &Grid) -> Vec<PaintRegion> {
    let w = g.w as usize;
    let h = g.h as usize;
    let mut seen = vec![false; w * h];
    let mut out = Vec::new();

    for y in 0..g.h {
        for x in 0..g.w {
            let idx = (y as usize) * w + x as usize;
            if seen[idx] {
                continue;
            }
            let style = g.at(x, y).style;
            if !style.has_paint() {
                seen[idx] = true;
                continue;
            }
            let key = style.paint_key();
            // Flood fill.
            let mut stack = vec![(x, y)];
            let mut minx = x;
            let mut maxx = x;
            let mut miny = y;
            let mut maxy = y;
            let mut rows = Vec::new();
            seen[idx] = true;
            let mut n = 0usize;
            while let Some((cx, cy)) = stack.pop() {
                n += 1;
                minx = minx.min(cx);
                maxx = maxx.max(cx);
                miny = miny.min(cy);
                maxy = maxy.max(cy);
                if !rows.contains(&cy) {
                    rows.push(cy);
                }
                for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if !g.in_bounds(nx, ny) {
                        continue;
                    }
                    let (nx, ny) = (nx as u16, ny as u16);
                    let nidx = (ny as usize) * w + nx as usize;
                    if seen[nidx] {
                        continue;
                    }
                    let s2 = g.at(nx, ny).style;
                    if s2.has_paint() && s2.paint_key() == key {
                        seen[nidx] = true;
                        stack.push((nx, ny));
                    }
                }
            }
            // Ignore specks; a real region is at least a few cells wide.
            if n >= 4 && maxx > minx {
                rows.sort_unstable();
                out.push(PaintRegion {
                    rect: Rect {
                        x: minx,
                        y: miny,
                        w: maxx - minx + 1,
                        h: maxy - miny + 1,
                    },
                    color: key,
                    rows,
                });
            }
        }
    }
    out.sort_by_key(|r| std::cmp::Reverse(r.rect.area()));
    out
}

/// A painted band spanning (almost) the full width at the very top or bottom is
/// the near-universal idiom for a header / status bar.
pub fn is_bar(g: &Grid, r: &Rect) -> bool {
    let full = r.w as f64 / g.w as f64 > 0.8;
    full && r.h <= 2 && (r.y == 0 || r.y1() >= g.h.saturating_sub(2))
}
