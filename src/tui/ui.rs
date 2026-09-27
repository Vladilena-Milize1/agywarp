use crate::tui::app::{App, FocusArea};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

// High-contrast, theme-adaptive color definitions
const ACCENT_COLOR: Color = Color::Blue;
const SUCCESS_COLOR: Color = Color::Green;
const ERROR_COLOR: Color = Color::Red;
const WARN_COLOR: Color = Color::Indexed(166); // Warm amber/orange, visible on both light and dark themes

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Vertical layout: Header (3) -> Main (fill) -> Footer (3)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(size);

    draw_header(f, chunks[0]);
    draw_main(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header_text = Line::from(vec![
        Span::styled(" agywarp ", Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)),
        Span::styled(
            "— Process Routing via Cloudflare WARP & Mihomo Core",
            Style::default().add_modifier(Modifier::DIM),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().add_modifier(Modifier::DIM));

    let paragraph = Paragraph::new(header_text)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn draw_main(f: &mut Frame, area: Rect, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(8),
        ])
        .split(main_chunks[0]);

    draw_network_card(f, left_chunks[0], app);
    draw_process_list(f, left_chunks[1], app);
    draw_console(f, main_chunks[1], app);
}

fn draw_network_card(f: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.focus == FocusArea::NetworkCard;
    let border_style = if is_focused {
        Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)
    } else {
        Style::default().add_modifier(Modifier::DIM)
    };

    let (service_text, service_style) = if app.service_active {
        ("ACTIVE (ON)", Style::default().fg(SUCCESS_COLOR).add_modifier(Modifier::BOLD))
    } else {
        ("INACTIVE (OFF)", Style::default().fg(ERROR_COLOR).add_modifier(Modifier::BOLD))
    };

    let (warp_text, warp_style) = if app.warp_status == "CONNECTED" {
        ("CONNECTED", Style::default().fg(SUCCESS_COLOR).add_modifier(Modifier::BOLD))
    } else if !app.warp_installed {
        ("NOT INSTALLED", Style::default().fg(ERROR_COLOR).add_modifier(Modifier::BOLD))
    } else {
        (&app.warp_status[..], Style::default().fg(WARN_COLOR).add_modifier(Modifier::BOLD))
    };

    let exit_line = if let Some(ref exit) = app.exit_info {
        format!("{} ({}) | {:?} ping", exit.ip, exit.colo, exit.latency)
    } else if app.service_active {
        "Verifying WARP connection...".to_string()
    } else {
        "---".to_string()
    };

    let text = vec![
        Line::from(vec![
            Span::raw("Routing Service: "),
            Span::styled(service_text, service_style),
            Span::styled("  (Press [Space] to toggle)", Style::default().add_modifier(Modifier::DIM)),
        ]),
        Line::from(vec![
            Span::raw("Current Node:    "),
            Span::styled(&app.current_node, Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("Airport/Source:  "),
            Span::styled(&app.airport_name, Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("WARP Daemon:     "),
            Span::styled(warp_text, warp_style),
            Span::styled(format!("  (Port: {})", app.warp_port), Style::default().add_modifier(Modifier::DIM)),
        ]),
        Line::from(vec![
            Span::raw("Protocol Mode:   "),
            Span::styled(app.proxy_mode.as_str().to_uppercase(), Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)),
            Span::styled("  (Press [p] to change)", Style::default().add_modifier(Modifier::DIM)),
        ]),
        Line::from(vec![
            Span::raw("WARP Exit IP:    "),
            Span::styled(exit_line, Style::default().fg(ACCENT_COLOR)),
        ]),
    ];

    let block = Block::default()
        .title(" Network Card ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, area);
}

fn draw_process_list(f: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.focus == FocusArea::ProcessList;
    let border_style = if is_focused {
        Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)
    } else {
        Style::default().add_modifier(Modifier::DIM)
    };

    let items: Vec<ListItem> = app
        .profiles
        .iter()
        .enumerate()
        .map(|(idx, profile)| {
            let is_selected = idx == app.selected_profile_idx;
            let (status_text, status_color) = if profile.enabled {
                ("[ON] ", SUCCESS_COLOR)
            } else {
                ("[OFF]", ERROR_COLOR)
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            let matchers_summary: Vec<String> = profile
                .matchers
                .iter()
                .map(|m| m.pattern.clone())
                .collect();

            let line = Line::from(vec![
                Span::styled(prefix, Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)),
                Span::styled(status_text, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled(&profile.label, Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!(" ({})", matchers_summary.join(", ")), Style::default().add_modifier(Modifier::DIM)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let block = Block::default()
        .title(" Process Groups ([Space] Toggle, [↑/↓] Select) ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn draw_console(f: &mut Frame, area: Rect, app: &App) {
    let is_focused = app.focus == FocusArea::Console;
    let border_style = if is_focused {
        Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)
    } else {
        Style::default().add_modifier(Modifier::DIM)
    };

    let logs: Vec<Line> = app
        .logs
        .iter()
        .map(|l| {
            let style = if l.contains("[OK]") {
                Style::default().fg(SUCCESS_COLOR)
            } else if l.contains("[WARN]") {
                Style::default().fg(WARN_COLOR)
            } else if l.contains("[ERROR]") {
                Style::default().fg(ERROR_COLOR).add_modifier(Modifier::BOLD)
            } else {
                Style::default() // Respects terminal foreground (black on light theme, white on dark theme)
            };
            Line::from(Span::styled(l, style))
        })
        .collect();

    let block = Block::default()
        .title(" Output Console ([↑/↓] Scroll) ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let paragraph = Paragraph::new(logs)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.console_scroll as u16, 0));

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let focus_hint = match app.focus {
        FocusArea::NetworkCard => "Focused: Network Card",
        FocusArea::ProcessList => "Focused: Process Groups",
        FocusArea::Console => "Focused: Output Console",
    };

    let text = Line::from(vec![
        Span::styled(format!(" [{}] ", focus_hint), Style::default().fg(ACCENT_COLOR).add_modifier(Modifier::BOLD)),
        Span::styled(" | [Tab] Switch Focus | [Space] Toggle | [p] Protocol | [r] Refresh | [q] Quit", Style::default().add_modifier(Modifier::DIM)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().add_modifier(Modifier::DIM));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
