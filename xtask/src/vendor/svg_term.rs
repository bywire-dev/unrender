//! Parses Rich's terminal SVG export format (what Textual's own snapshot
//! tests use for golden files) back into a character grid, then re-emits it
//! as ANSI — the SVG equivalent of `ansi.rs`'s `Buffer -> ANSI` writer.
//!
//! Confirmed by hand against five varied real snapshots (table, tree,
//! progress bar, list, log) before trusting this broadly, per the plan: the
//! format looks fixed at a glance but Rich mangles the `terminal-*` id
//! prefix into `terminal-<random-u32>-*` on some renders (apparently to
//! avoid id collisions when multiple SVGs land in one HTML page), which a
//! parser hardcoded to the literal `terminal-` prefix silently reads as
//! empty. Every id is matched relative to a prefix captured from the file
//! itself, never assumed.

use std::collections::HashMap;

use anyhow::{Context, Result};
use regex::Regex;

pub type Rgb = (u8, u8, u8);

#[derive(Clone, Debug, Default)]
struct StyleClass {
    fg: Option<Rgb>,
    bold: bool,
    italic: bool,
    underline: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Cell {
    pub ch: char,
    pub fg: Option<Rgb>,
    pub bg: Option<Rgb>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn blank(rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            cells: vec![
                Cell {
                    ch: ' ',
                    ..Cell::default()
                };
                rows * cols
            ],
        }
    }

    fn at_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        if row < self.rows && col < self.cols {
            self.cells.get_mut(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn at(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row * self.cols + col]
    }

    /// True if every cell is the untouched default -- almost certainly a
    /// parsing failure rather than a genuinely empty screen.
    pub fn is_blank(&self) -> bool {
        self.cells
            .iter()
            .all(|c| c.ch == ' ' && c.fg.is_none() && c.bg.is_none())
    }
}

fn parse_hex(s: &str) -> Option<Rgb> {
    if s.len() != 6 {
        return None;
    }
    Some((
        u8::from_str_radix(&s[0..2], 16).ok()?,
        u8::from_str_radix(&s[2..4], 16).ok()?,
        u8::from_str_radix(&s[4..6], 16).ok()?,
    ))
}

/// Rich's SVG export uses numeric character references in both decimal
/// (`&#160;`) and hex (`&#x27;`) form, plus the five standard XML entities.
/// There is no DOCTYPE with custom entities in this format, so this short
/// list is exhaustive for real input.
fn unescape(s: &str) -> String {
    let hex_re = Regex::new(r"&#x([0-9a-fA-F]+);").unwrap();
    let s = hex_re.replace_all(s, |caps: &regex::Captures| {
        u32::from_str_radix(&caps[1], 16)
            .ok()
            .and_then(char::from_u32)
            .map(String::from)
            .unwrap_or_default()
    });
    let dec_re = Regex::new(r"&#(\d+);").unwrap();
    let s = dec_re.replace_all(&s, |caps: &regex::Captures| {
        caps[1]
            .parse::<u32>()
            .ok()
            .and_then(char::from_u32)
            .map(String::from)
            .unwrap_or_default()
    });
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

pub fn parse(svg: &str) -> Result<Grid> {
    let prefix_re = Regex::new(r#"<clipPath id="(terminal(?:-\d+)?)-line-0">"#).unwrap();
    let prefix = prefix_re
        .captures(svg)
        .context("no terminal-line-0 clipPath -- not a Rich terminal SVG export")?
        .get(1)
        .unwrap()
        .as_str();
    let p = regex::escape(prefix);

    // Style classes: `.terminal-rN { fill: #hex[;font-weight: bold][;font-style: italic][;text-decoration: underline] }`
    let class_re = Regex::new(&format!(r"\.{p}-r(\d+)\s*\{{([^}}]*)\}}")).unwrap();
    let fill_re = Regex::new(r"fill:\s*#([0-9a-fA-F]{6})").unwrap();
    let mut classes: HashMap<u32, StyleClass> = HashMap::new();
    for cap in class_re.captures_iter(svg) {
        let id: u32 = cap[1].parse().unwrap();
        let body = &cap[2];
        let fg = fill_re.captures(body).and_then(|c| parse_hex(&c[1]));
        classes.insert(
            id,
            StyleClass {
                fg,
                bold: body.contains("font-weight: bold") || body.contains("font-weight:bold"),
                italic: body.contains("font-style: italic") || body.contains("font-style:italic"),
                underline: body.contains("text-decoration: underline")
                    || body.contains("text-decoration:underline"),
            },
        );
    }

    // Row y-origins from the per-line clip rects, giving us line_height and
    // row count without trusting any hardcoded font metric.
    let line_re = Regex::new(&format!(
        r#"<clipPath id="{p}-line-(\d+)">\s*<rect[^>]*\by="([\d.]+)""#
    ))
    .unwrap();
    let mut line_y: HashMap<usize, f64> = HashMap::new();
    for cap in line_re.captures_iter(svg) {
        line_y.insert(cap[1].parse().unwrap(), cap[2].parse().unwrap());
    }
    let rows = line_y.keys().max().map(|m| m + 1).unwrap_or(0);
    anyhow::ensure!(rows > 0, "no terminal-line clipPaths found");
    let y0 = line_y[&0];
    let line_height = if rows > 1 { line_y[&1] - y0 } else { 1.0 };

    // Column count from the terminal viewport width, divided by a cell
    // width measured from the file's own smallest non-zero background rect
    // -- never assumed from a font-size constant, which drifted between the
    // sample files (12.2px in most, 24.4px in a double-width-font capture).
    let clip_re = Regex::new(&format!(
        r#"<clipPath id="{p}-clip-terminal">\s*<rect x="0" y="0" width="([\d.]+)""#
    ))
    .unwrap();
    let term_width: f64 = clip_re
        .captures(svg)
        .context("no terminal-clip-terminal clipPath")?[1]
        .parse()?;

    let bg_re = Regex::new(
        r##"<rect fill="#([0-9a-fA-F]{6})"[^>]*\bx="([\d.]+)"[^>]*\by="([\d.]+)"[^>]*\bwidth="([\d.]+)""##,
    )
    .unwrap();
    let cell_width = bg_re
        .captures_iter(svg)
        .filter_map(|c| c[4].parse::<f64>().ok())
        .filter(|w| *w > 0.1)
        .fold(f64::MAX, f64::min);
    anyhow::ensure!(
        cell_width.is_finite(),
        "no background rects to measure cell width from"
    );
    let cols = (term_width / cell_width).round() as usize;
    anyhow::ensure!(cols > 0, "computed zero columns");

    let mut grid = Grid::blank(rows, cols);

    // Background fills, placed by measured row/col -- these carry no
    // per-cell row id the way text elements do, so they're the one place
    // this parser still relies on line_height arithmetic.
    for cap in bg_re.captures_iter(svg) {
        let hex = &cap[1];
        let x: f64 = cap[2].parse()?;
        let y: f64 = cap[3].parse()?;
        let w: f64 = cap[4].parse()?;
        if w <= 0.1 {
            continue;
        }
        let Some(rgb) = parse_hex(hex) else { continue };
        let row = ((y - y0) / line_height).round();
        if row < 0.0 {
            continue;
        }
        let row = row as usize;
        let col0 = (x / cell_width).round() as usize;
        let span = (w / cell_width).round() as usize;
        for col in col0..col0 + span {
            if let Some(cell) = grid.at_mut(row, col) {
                cell.bg = Some(rgb);
            }
        }
    }

    // Foreground text runs: row comes directly from the clip-path id, never
    // from y-coordinate math, so this is exact even where line_height above
    // is only an approximation.
    let text_re = Regex::new(&format!(
        r#"(?s)<text class="{p}-r(\d+)" x="([\d.]+)" y="[\d.]+" textLength="[\d.]+" clip-path="url\(#{p}-line-(\d+)\)">(.*?)</text>"#
    ))
    .unwrap();
    for cap in text_re.captures_iter(svg) {
        let class_id: u32 = cap[1].parse().unwrap();
        let x: f64 = cap[2].parse()?;
        let row: usize = cap[3].parse().unwrap();
        let content = unescape(&cap[4]);
        let style = classes.get(&class_id).cloned().unwrap_or_default();
        let col0 = (x / cell_width).round() as usize;
        for (i, ch) in content.chars().enumerate() {
            if ch == '\n' {
                continue;
            }
            if let Some(cell) = grid.at_mut(row, col0 + i) {
                cell.ch = ch;
                cell.fg = style.fg;
                cell.bold = style.bold;
                cell.italic = style.italic;
                cell.underline = style.underline;
            }
        }
    }

    Ok(grid)
}

/// (fg, bg, bold, italic, underline) -- enough to detect a style-run change.
type StyleKey = (Option<Rgb>, Option<Rgb>, bool, bool, bool);

/// Same run-length SGR encoding as `ansi.rs`, but colors here are already
/// concrete RGB (Rich resolves its own theme to hex before export), so
/// there's no named-color table to consult -- everything is 24-bit.
pub fn grid_to_ansi(grid: &Grid) -> String {
    let mut out = String::new();
    for row in 0..grid.rows {
        let mut current: Option<StyleKey> = None;
        for col in 0..grid.cols {
            let cell = grid.at(row, col);
            let key = (cell.fg, cell.bg, cell.bold, cell.italic, cell.underline);
            if current != Some(key) {
                out.push_str(&sgr(key.0, key.1, key.2, key.3, key.4));
                current = Some(key);
            }
            out.push(cell.ch);
        }
        out.push_str("\x1b[0m\r\n");
    }
    out
}

fn sgr(fg: Option<Rgb>, bg: Option<Rgb>, bold: bool, italic: bool, underline: bool) -> String {
    let mut codes = vec!["0".to_string()];
    if bold {
        codes.push("1".into());
    }
    if italic {
        codes.push("3".into());
    }
    if underline {
        codes.push("4".into());
    }
    if let Some((r, g, b)) = fg {
        codes.push(format!("38;2;{r};{g};{b}"));
    }
    if let Some((r, g, b)) = bg {
        codes.push(format!("48;2;{r};{g};{b}"));
    }
    format!("\x1b[{}m", codes.join(";"))
}
