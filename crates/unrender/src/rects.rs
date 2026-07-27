//! Box-drawing line art -> nested rectangles.
//!
//! The premise of the whole spike is that TUI frameworks do not invent their
//! own chrome: they draw frames with the Unicode box-drawing block. If we can
//! recover those frames we recover the framework's own layout tree for free.
//!
//! Each glyph is decoded into the directions it connects (up/down/left/right),
//! which turns "find the panels" into a graph problem rather than a pile of
//! per-character special cases.

use crate::grid::Grid;

pub const UP: u8 = 1;
pub const DOWN: u8 = 2;
pub const LEFT: u8 = 4;
pub const RIGHT: u8 = 8;

/// Which directions this glyph connects to. 0 = not line art.
pub fn edges(c: char) -> u8 {
    let h = LEFT | RIGHT;
    let v = UP | DOWN;
    match c {
        // ASCII fallbacks, used by TERM=dumb / ASCII-only modes.
        '-' | '=' | '~' => h,
        '|' => v,
        '+' => h | v,
        _ => match c as u32 {
            // light/heavy horizontals and their dashed variants
            0x2500 | 0x2501 | 0x2504..=0x2505 | 0x2508..=0x2509 | 0x254C..=0x254D => h,
            0x2502 | 0x2503 | 0x2506..=0x2507 | 0x250A..=0x250B | 0x254E..=0x254F => v,
            // corners: down+right
            0x250C..=0x250F | 0x2552..=0x2554 | 0x256D => DOWN | RIGHT,
            // corners: down+left
            0x2510..=0x2513 | 0x2555..=0x2557 | 0x256E => DOWN | LEFT,
            // corners: up+right
            0x2514..=0x2517 | 0x2558..=0x255A | 0x2570 => UP | RIGHT,
            // corners: up+left
            0x2518..=0x251B | 0x255B..=0x255D | 0x256F => UP | LEFT,
            // tees
            0x251C..=0x2523 | 0x255E..=0x2560 => UP | DOWN | RIGHT,
            0x2524..=0x252B | 0x2561..=0x2563 => UP | DOWN | LEFT,
            0x252C..=0x2533 | 0x2564..=0x2566 => DOWN | LEFT | RIGHT,
            0x2534..=0x253B | 0x2567..=0x2569 => UP | LEFT | RIGHT,
            // crosses
            0x253C..=0x254B | 0x256A..=0x256C => h | v,
            // doubles
            0x2550 => h,
            0x2551 => v,
            // stubs
            0x2574 | 0x2578 => LEFT,
            0x2575 | 0x2579 => UP,
            0x2576 | 0x257A => RIGHT,
            0x2577 | 0x257B => DOWN,
            0x257C | 0x257E => h,
            0x257D | 0x257F => v,
            _ => 0,
        },
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn x1(&self) -> u16 {
        self.x + self.w - 1
    }
    pub fn y1(&self) -> u16 {
        self.y + self.h - 1
    }
    pub fn area(&self) -> u32 {
        self.w as u32 * self.h as u32
    }
    pub fn contains(&self, o: &Rect) -> bool {
        o.x >= self.x && o.y >= self.y && o.x1() <= self.x1() && o.y1() <= self.y1()
    }
    pub fn intersection_area(&self, o: &Rect) -> u32 {
        let x0 = self.x.max(o.x);
        let y0 = self.y.max(o.y);
        let x1 = self.x1().min(o.x1());
        let y1 = self.y1().min(o.y1());
        if x1 < x0 || y1 < y0 {
            0
        } else {
            (x1 - x0 + 1) as u32 * (y1 - y0 + 1) as u32
        }
    }
    pub fn iou(&self, o: &Rect) -> f64 {
        let i = self.intersection_area(o) as f64;
        let u = self.area() as f64 + o.area() as f64 - i;
        if u == 0.0 {
            0.0
        } else {
            i / u
        }
    }
    /// The area inside the border.
    pub fn interior(&self) -> Option<Rect> {
        if self.w < 3 || self.h < 3 {
            None
        } else {
            Some(Rect {
                x: self.x + 1,
                y: self.y + 1,
                w: self.w - 2,
                h: self.h - 2,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoxFrame {
    pub rect: Rect,
    pub title: Option<String>,
}

/// Fraction of cells on a horizontal edge that are line art, plus the leftover
/// text. Frameworks embed panel titles *into* the top border
/// (`┌─ services ───┐`), so an edge is allowed to be interrupted -- and the
/// interruption is exactly the accessible name we want.
fn scan_h(g: &Grid, y: u16, x0: u16, x1: u16) -> (f64, String) {
    let row: Vec<char> = (x0..=x1).map(|x| g.at(x, y).ch()).collect();
    let mut boxy = 0usize;
    let mut total = 0usize;
    let mut text = String::new();
    for (i, &c) in row.iter().enumerate() {
        total += 1;
        let is_edge = edges(c) & (LEFT | RIGHT) != 0;
        if is_edge {
            boxy += 1;
        }
        // ASCII '-' and '|' double as ordinary punctuation. Only treat them as
        // border when they run at least three long, so a title like
        // "zellij-spike" keeps its hyphen instead of losing it to the frame.
        let ascii_fallback = c.is_ascii();
        let long_run = {
            let mut n = 1usize;
            let mut j = i;
            while j > 0 && row[j - 1] == c {
                n += 1;
                j -= 1;
            }
            let mut k = i;
            while k + 1 < row.len() && row[k + 1] == c {
                n += 1;
                k += 1;
            }
            n >= 3
        };
        if is_edge && (!ascii_fallback || long_run) {
            text.push(' ');
        } else {
            text.push(c);
        }
    }
    let ratio = if total == 0 {
        0.0
    } else {
        boxy as f64 / total as f64
    };
    // Frameworks bracket titles with their own punctuation (prompt_toolkit
    // renders `─| services |─`); keep the words, drop the decoration.
    let title = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|c| matches!(c, '|' | '-' | '=' | '<' | '>' | ' '))
        .to_string();
    (ratio, title)
}

fn scan_v(g: &Grid, x: u16, y0: u16, y1: u16) -> f64 {
    let mut boxy = 0usize;
    let mut total = 0usize;
    for y in y0..=y1 {
        let c = g.at(x, y).ch();
        total += 1;
        if edges(c) & (UP | DOWN) != 0 {
            boxy += 1;
        }
    }
    if total == 0 {
        0.0
    } else {
        boxy as f64 / total as f64
    }
}

const MIN_H_EDGE: f64 = 0.5; // titles live here, so tolerate interruption
const MIN_V_EDGE: f64 = 0.8; // side borders are rarely interrupted

/// Find every closed rectangle drawn in line art.
pub fn find_frames(g: &Grid) -> Vec<BoxFrame> {
    let mut out: Vec<BoxFrame> = Vec::new();

    // Candidate top-left corners.
    for y0 in 0..g.h {
        for x0 in 0..g.w {
            let e = edges(g.at(x0, y0).ch());
            if e & DOWN == 0 || e & RIGHT == 0 {
                continue;
            }
            // Candidate top-right corners on the same row.
            for x1 in (x0 + 2)..g.w {
                let e_tr = edges(g.at(x1, y0).ch());
                if e_tr & DOWN == 0 || e_tr & LEFT == 0 {
                    continue;
                }
                let (top_ratio, title) = scan_h(g, y0, x0, x1);
                if top_ratio < MIN_H_EDGE {
                    continue;
                }
                // Candidate bottom edge.
                for y1 in (y0 + 2)..g.h {
                    let e_bl = edges(g.at(x0, y1).ch());
                    let e_br = edges(g.at(x1, y1).ch());
                    if e_bl & UP == 0 || e_bl & RIGHT == 0 {
                        continue;
                    }
                    if e_br & UP == 0 || e_br & LEFT == 0 {
                        continue;
                    }
                    let (bot_ratio, _) = scan_h(g, y1, x0, x1);
                    if bot_ratio < MIN_H_EDGE {
                        continue;
                    }
                    if scan_v(g, x0, y0, y1) < MIN_V_EDGE {
                        continue;
                    }
                    if scan_v(g, x1, y0, y1) < MIN_V_EDGE {
                        continue;
                    }
                    let rect = Rect {
                        x: x0,
                        y: y0,
                        w: x1 - x0 + 1,
                        h: y1 - y0 + 1,
                    };
                    let title = if title.is_empty() { None } else { Some(title) };
                    if !out.iter().any(|f| f.rect == rect) {
                        out.push(BoxFrame { rect, title });
                    }
                    // Stop at the first (innermost) bottom edge for this pair;
                    // outer frames are found from their own corners.
                    break;
                }
            }
        }
    }

    out.sort_by_key(|f| std::cmp::Reverse(f.rect.area()));
    out
}
