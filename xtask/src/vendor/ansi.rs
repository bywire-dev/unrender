//! Serializes a ratatui `Buffer` back to ANSI text — the inverse of what
//! `vt100` does to a live capture. This is real new code (`TestBackend` has
//! no such writer): it's what lets `xtask vendor` produce fixtures whose
//! `.ansi` file is indistinguishable, to `unrender`, from a real terminal
//! capture, without ever opening a pty.

use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier};

pub fn buffer_to_ansi(buf: &Buffer) -> String {
    let area = buf.area;
    let mut out = String::new();
    for y in area.top()..area.bottom() {
        let mut current: Option<(Color, Color, Modifier)> = None;
        for x in area.left()..area.right() {
            let cell = &buf[(x, y)];
            let key = (cell.fg, cell.bg, cell.modifier);
            if current != Some(key) {
                out.push_str(&sgr(key.0, key.1, key.2));
                current = Some(key);
            }
            let sym = cell.symbol();
            out.push_str(if sym.is_empty() { " " } else { sym });
        }
        out.push_str("\x1b[0m\r\n");
    }
    out
}

fn sgr(fg: Color, bg: Color, modifier: Modifier) -> String {
    let mut codes = vec!["0".to_string()];
    if modifier.contains(Modifier::BOLD) {
        codes.push("1".into());
    }
    if modifier.contains(Modifier::DIM) {
        codes.push("2".into());
    }
    if modifier.contains(Modifier::ITALIC) {
        codes.push("3".into());
    }
    if modifier.contains(Modifier::UNDERLINED) {
        codes.push("4".into());
    }
    if modifier.contains(Modifier::SLOW_BLINK) {
        codes.push("5".into());
    }
    if modifier.contains(Modifier::RAPID_BLINK) {
        codes.push("6".into());
    }
    if modifier.contains(Modifier::REVERSED) {
        codes.push("7".into());
    }
    if modifier.contains(Modifier::HIDDEN) {
        codes.push("8".into());
    }
    if modifier.contains(Modifier::CROSSED_OUT) {
        codes.push("9".into());
    }
    if let Some(c) = color_code(fg, false) {
        codes.push(c);
    }
    if let Some(c) = color_code(bg, true) {
        codes.push(c);
    }
    format!("\x1b[{}m", codes.join(";"))
}

/// `None` for `Color::Reset` — the cell inherits the terminal default, which
/// is exactly what a bare SGR reset (already emitted) leaves in place.
fn color_code(c: Color, bg: bool) -> Option<String> {
    let base = if bg { 10 } else { 0 };
    Some(match c {
        Color::Reset => return None,
        Color::Black => (30 + base).to_string(),
        Color::Red => (31 + base).to_string(),
        Color::Green => (32 + base).to_string(),
        Color::Yellow => (33 + base).to_string(),
        Color::Blue => (34 + base).to_string(),
        Color::Magenta => (35 + base).to_string(),
        Color::Cyan => (36 + base).to_string(),
        Color::Gray => (37 + base).to_string(),
        Color::DarkGray => (90 + base).to_string(),
        Color::LightRed => (91 + base).to_string(),
        Color::LightGreen => (92 + base).to_string(),
        Color::LightYellow => (93 + base).to_string(),
        Color::LightBlue => (94 + base).to_string(),
        Color::LightMagenta => (95 + base).to_string(),
        Color::LightCyan => (96 + base).to_string(),
        Color::White => (97 + base).to_string(),
        Color::Rgb(r, g, b) => format!("{};2;{r};{g};{b}", if bg { 48 } else { 38 }),
        Color::Indexed(i) => format!("{};5;{i}", if bg { 48 } else { 38 }),
    })
}
