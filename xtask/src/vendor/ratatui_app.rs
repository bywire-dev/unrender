//! Vendors `fixtures/vendor/ratatui/{default,moved}.{ansi,truth.json}` by
//! rendering the same deploy-console layout as the old spike's `corpus/rat`
//! app, but through `TestBackend` instead of a live terminal — no zellij, no
//! keystrokes, no pty. The layout code is unchanged from the spike: ratatui
//! computes the same real `Rect`s either way, so the ground truth is exactly
//! as honest as driving a live terminal was.

use ratatui::backend::TestBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Cell, Gauge, List, ListItem, Paragraph, Row, Table, TableState};
use ratatui::Terminal;

use super::ansi::buffer_to_ansi;

const SERVICES: &[(&str, &str, &str)] = &[
    ("api-gateway", "running", "12ms"),
    ("auth-service", "running", "31ms"),
    ("billing", "degraded", "412ms"),
    ("search-index", "running", "88ms"),
    ("mailer", "stopped", "-"),
    ("scheduler", "running", "5ms"),
];

const EVENTS: &[&str] = &[
    "deploy started",
    "image pulled",
    "health check ok",
    "traffic shifted 10%",
    "traffic shifted 50%",
];

fn truth_node(role: &str, name: &str, r: Rect) -> serde_json::Value {
    serde_json::json!({
        "role": role,
        "name": name,
        "rect": {"x": r.x, "y": r.y, "w": r.width, "h": r.height},
    })
}

/// Renders one frame with the table selection at `selected`, returning the
/// ANSI text and the ground truth ratatui itself computed for that frame.
pub fn render(selected: usize) -> anyhow::Result<(String, serde_json::Value)> {
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend)?;
    let mut state = TableState::default().with_selected(Some(selected));
    let mut truth = serde_json::Value::Null;

    terminal.draw(|frame| {
        let area = frame.area();
        let outer = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);
        let mid = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(outer[1]);
        let right = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(mid[1]);

        let header = Paragraph::new(Line::from("Deploy Console  ::  cluster prod-eu-1"))
            .block(Block::bordered().title("zellij-spike"));
        frame.render_widget(header, outer[0]);

        let rows: Vec<Row> = SERVICES
            .iter()
            .map(|(n, s, l)| {
                let color = match *s {
                    "running" => Color::Green,
                    "degraded" => Color::Yellow,
                    _ => Color::Red,
                };
                Row::new(vec![
                    Cell::from(*n),
                    Cell::from(*s).style(Style::default().fg(color)),
                    Cell::from(*l),
                ])
            })
            .collect();
        let table = Table::new(
            rows,
            [
                Constraint::Length(16),
                Constraint::Length(10),
                Constraint::Length(8),
            ],
        )
        .header(
            Row::new(vec!["SERVICE", "STATE", "P99"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(Block::bordered().title("services"))
        .row_highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("> ");
        frame.render_stateful_widget(table, mid[0], &mut state);

        let items: Vec<ListItem> = EVENTS.iter().map(|e| ListItem::new(*e)).collect();
        let list = List::new(items).block(Block::bordered().title("events"));
        frame.render_widget(list, right[0]);

        let gauge = Gauge::default()
            .block(Block::bordered().title("rollout"))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(50);
        frame.render_widget(gauge, right[1]);

        let status = Paragraph::new(Line::from(" q quit  j/k move  ENTER select "))
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));
        frame.render_widget(status, outer[2]);

        truth = serde_json::json!({
            "app": "ratatui",
            "selected": state.selected(),
            "nodes": [
                truth_node("application", "root", area),
                truth_node("heading", "zellij-spike", outer[0]),
                truth_node("table", "services", mid[0]),
                truth_node("list", "events", right[0]),
                truth_node("progressbar", "rollout", right[1]),
                truth_node("statusbar", "", outer[2]),
            ],
        });
    })?;

    let ansi = buffer_to_ansi(terminal.backend().buffer());
    Ok((ansi, truth))
}
