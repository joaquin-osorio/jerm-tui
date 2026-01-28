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

    // Calculate how many visual lines we'll actually render (accounting for wrapping)
    let width = inner_area.width.max(1) as usize;
    let mut visual_line_count = 0;
    let mut input_line_visual_start = 0;

    for (i, line) in visible_lines.iter().enumerate() {
        let line_width = line.width();
        let wrapped_lines = if line_width == 0 {
            1
        } else {
            (line_width + width - 1) / width
        };

        // If this is the last line (the input line), remember where it starts
        if i == visible_lines.len().saturating_sub(1) {
            input_line_visual_start = visual_line_count;
        }

        visual_line_count += wrapped_lines;
    }

    let paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });

    f.render_widget(paragraph, inner_area);

    // Position cursor at the input position
    let cursor_x = inner_area.x + (prompt.len() + app.cursor_pos) as u16;
    // The input line starts at input_line_visual_start, and we need to add the offset
    // within that line based on where the cursor is
    let prompt_and_cursor_len = prompt.len() + app.cursor_pos;
    let lines_into_input = prompt_and_cursor_len / width;
    let cursor_y = inner_area.y + (input_line_visual_start + lines_into_input).min(available_height.saturating_sub(1)) as u16;

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
        crate::app::AppMode::ShortcutSelection => "GOTO",
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
