use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Render the main terminal area
pub fn render_terminal(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Terminal ");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Calculate how many lines we can show
    let available_height = inner_area.height as usize;

    // Build output lines
    let mut lines: Vec<Line> = app
        .output
        .iter()
        .map(|line| Line::from(line.as_str()))
        .collect();

    // Add current prompt and input
    let prompt = app.prompt();
    let input_line = format!("{}{}", prompt, app.input);
    lines.push(Line::from(input_line.clone()));

    // Calculate scroll
    let total_lines = lines.len();
    let scroll = total_lines.saturating_sub(available_height);

    // Take visible lines
    let visible_lines: Vec<Line> = lines.into_iter().skip(scroll).collect();

    let paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });

    f.render_widget(paragraph, inner_area);

    // Position cursor at the input position
    let cursor_x = inner_area.x + (prompt.len() + app.cursor_pos) as u16;
    let cursor_y = inner_area.y + (available_height.saturating_sub(1)) as u16;

    // Make sure cursor is within bounds
    let cursor_x = cursor_x.min(inner_area.x + inner_area.width - 1);
    let cursor_y = cursor_y.min(inner_area.y + inner_area.height - 1);

    f.set_cursor(cursor_x, cursor_y);
}

/// Render a status bar at the bottom of the terminal
#[allow(dead_code)]
pub fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let mode_text = match app.mode {
        crate::app::AppMode::Normal => "NORMAL",
        crate::app::AppMode::NavigationList => "NAV",
    };

    let status = Line::from(vec![
        Span::styled(
            format!(" {} ", mode_text),
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" "),
        Span::styled(
            app.current_dir.display().to_string(),
            Style::default().fg(Color::Gray),
        ),
    ]);

    let paragraph = Paragraph::new(status);
    f.render_widget(paragraph, area);
}
