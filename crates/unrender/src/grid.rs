//! ANSI screen dump -> attributed cell grid.
//!
//! We do not hand-roll a VT parser; `vt100` is the same lineage of parser the
//! multiplexers themselves use. The input is already screen-shaped (a dump),
//! but running it through a real emulator means odd SGR usage, wide chars and
//! cursor motion inside the dump are all handled correctly.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum Color {
    Default,
    Idx(u8),
    Rgb(u8, u8, u8),
}

impl From<vt100::Color> for Color {
    fn from(c: vt100::Color) -> Self {
        match c {
            vt100::Color::Default => Color::Default,
            vt100::Color::Idx(i) => Color::Idx(i),
            vt100::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub inverse: bool,
    pub underline: bool,
    pub italic: bool,
}

impl Style {
    /// Effective background, accounting for reverse video. Reverse video is how
    /// a large fraction of TUIs express "selected", so it must fold into bg
    /// before any region segmentation happens.
    pub fn eff_bg(&self) -> Color {
        let fg = self.fg.unwrap_or(Color::Default);
        let bg = self.bg.unwrap_or(Color::Default);
        if self.inverse {
            fg
        } else {
            bg
        }
    }

    /// True when the cell paints a background distinct from the terminal default.
    pub fn has_paint(&self) -> bool {
        self.inverse || !matches!(self.bg.unwrap_or(Color::Default), Color::Default)
    }

    /// Identity of a cell's paint for segmentation purposes.
    ///
    /// Reverse video must stay distinguishable from normal even when both
    /// resolve to default colours -- `inverse` on an otherwise unstyled cell is
    /// how Ink, dialog and prompt_toolkit all mark the selected row, and
    /// folding it into the background would erase exactly the signal we need.
    pub fn paint_key(&self) -> (Color, bool) {
        (self.eff_bg(), self.inverse)
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub text: String,
    pub style: Style,
}

impl Cell {
    pub fn ch(&self) -> char {
        self.text.chars().next().unwrap_or(' ')
    }
    pub fn is_blank(&self) -> bool {
        self.text.trim().is_empty()
    }
}

pub struct Grid {
    pub w: u16,
    pub h: u16,
    cells: Vec<Cell>,
}

impl Grid {
    /// An all-blank grid, for reconstructing a screen from a tree.
    pub fn blank(w: u16, h: u16) -> Grid {
        let cell = Cell {
            text: " ".to_string(),
            style: Style::default(),
        };
        Grid {
            w,
            h,
            cells: vec![cell; (w as usize) * (h as usize)],
        }
    }

    /// Write one cell. Out-of-bounds writes are dropped rather than panicking:
    /// a reconstructed tree can legitimately describe content wider than the
    /// grid it is being drawn into (a `value` longer than its own rect), and
    /// clipping is the same thing the original terminal did.
    pub fn set(&mut self, x: u16, y: u16, text: &str, style: Style) {
        if x >= self.w || y >= self.h {
            return;
        }
        let idx = (y as usize) * (self.w as usize) + (x as usize);
        self.cells[idx] = Cell {
            text: text.to_string(),
            style,
        };
    }

    /// Write a string starting at `x`, clipped to `max_x` inclusive.
    ///
    /// Advances by display width, so a CJK or emoji cell consumes the two
    /// columns it actually occupies — writing per-`char` would shear every
    /// subsequent column on a screen containing wide glyphs.
    pub fn write_str(&mut self, x: u16, y: u16, max_x: u16, s: &str, style: Style) {
        use unicode_width::UnicodeWidthChar;
        let mut cx = x;
        for ch in s.chars() {
            if cx > max_x || cx >= self.w {
                break;
            }
            let w = ch.width().unwrap_or(0) as u16;
            self.set(cx, y, &ch.to_string(), style);
            cx += w.max(1);
        }
    }

    pub fn from_ansi(bytes: &[u8], w: u16, h: u16) -> Grid {
        let mut parser = vt100::Parser::new(h, w, 0);
        // Dumps are line-oriented text, so a bare LF must also return the
        // carriage or every row starts where the previous one ended.
        // A trailing newline on the last row would scroll the whole screen up
        // by one, silently shifting every coordinate we are about to report.
        let mut bytes = bytes;
        while bytes.last() == Some(&b'\n') || bytes.last() == Some(&b'\r') {
            bytes = &bytes[..bytes.len() - 1];
        }
        let mut norm = Vec::with_capacity(bytes.len() + bytes.len() / 40);
        let mut prev = 0u8;
        for &b in bytes {
            if b == b'\n' && prev != b'\r' {
                norm.push(b'\r');
            }
            norm.push(b);
            prev = b;
        }
        parser.process(&norm);
        let screen = parser.screen();
        let mut cells = Vec::with_capacity((w as usize) * (h as usize));
        for y in 0..h {
            for x in 0..w {
                let cell = screen.cell(y, x);
                let (text, style) = match cell {
                    Some(c) => {
                        let t = c.contents();
                        (
                            if t.is_empty() {
                                " ".to_string()
                            } else {
                                t.to_string()
                            },
                            Style {
                                fg: Some(c.fgcolor().into()),
                                bg: Some(c.bgcolor().into()),
                                bold: c.bold(),
                                inverse: c.inverse(),
                                underline: c.underline(),
                                italic: c.italic(),
                            },
                        )
                    }
                    None => (" ".to_string(), Style::default()),
                };
                cells.push(Cell { text, style });
            }
        }
        Grid { w, h, cells }
    }

    /// Infer dimensions from a raw dump then parse. Zellij emits one line per
    /// screen row, so the row count is the line count and the width is the
    /// widest line once escape sequences are discounted.
    pub fn from_ansi_autosize(bytes: &[u8]) -> Grid {
        let text = String::from_utf8_lossy(bytes);
        let lines: Vec<&str> = text.lines().collect();
        let h = lines.len().max(1) as u16;
        let w = lines
            .iter()
            .map(|l| visible_width(l))
            .max()
            .unwrap_or(80)
            .max(1) as u16;
        Grid::from_ansi(bytes, w, h)
    }

    #[inline]
    pub fn at(&self, x: u16, y: u16) -> &Cell {
        &self.cells[(y as usize) * (self.w as usize) + (x as usize)]
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as u16) < self.w && (y as u16) < self.h
    }

    /// Plain text of a row slice, trailing whitespace trimmed.
    pub fn row_text(&self, y: u16, x0: u16, x1: u16) -> String {
        let mut s = String::new();
        for x in x0..=x1.min(self.w.saturating_sub(1)) {
            s.push_str(&self.at(x, y).text);
        }
        s.trim_end().to_string()
    }
}

/// Width of a line ignoring CSI/OSC escape sequences.
pub fn visible_width(line: &str) -> usize {
    use unicode_width::UnicodeWidthChar;
    let mut w = 0usize;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    for c2 in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c2) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    while let Some(c2) = chars.next() {
                        if c2 == '\x07' {
                            break;
                        }
                        if c2 == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {
                    chars.next();
                }
            }
            continue;
        }
        w += c.width().unwrap_or(0);
    }
    w
}
