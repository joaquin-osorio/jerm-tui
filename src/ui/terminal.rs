use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::app::App;
use crate::highlight::Tokenizer;
use crate::theme::Palette;

/// Wrap a line of text into multiple lines based on width
fn wrap_line(line: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![String::new()];
    }

    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for ch in line.chars() {
        let char_width = ch.width().unwrap_or(0);

        if current_width + char_width > width {
            // Current line is full, start a new one
            result.push(current_line.clone());
            current_line.clear();
            current_width = 0;
        }

        current_line.push(ch);
        current_width += char_width;
    }

    // Push the last line
    result.push(current_line);
    result
}

/// Render the main terminal area
pub fn render_terminal(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Palette::BORDER_DEFAULT))
        .title(" Terminal ");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let width = inner_area.width.max(1) as usize;
    let available_height = inner_area.height as usize;

    // Build visual lines as Line objects
    let mut visual_lines: Vec<Line> = Vec::new();

    // Add output lines (with wrapping)
    for line in &app.output {
        let wrapped = wrap_line(line, width);
        for wrapped_line in wrapped {
            visual_lines.push(Line::from(wrapped_line));
        }
    }

    // Save where the input line starts
    let input_line_start = visual_lines.len();

    // Add current prompt and input (with wrapping and syntax highlighting)
    let prompt_spans = app.prompt_spans();

    // Tokenize and highlight the input
    let tokens = Tokenizer::tokenize(&app.input);
    let input_spans = Tokenizer::to_spans(&tokens);

    let mut full_line_spans = prompt_spans;
    full_line_spans.extend(input_spans);

    // For wrapping calculation, use plain string
    let prompt_str = app.prompt_string();
    let full_input_line = format!("{}{}", prompt_str, app.input);
    let wrapped_input = wrap_line(&full_input_line, width);

    // If no wrapping, use colored Line
    if wrapped_input.len() == 1 {
        visual_lines.push(Line::from(full_line_spans));
    } else {
        // Wrapping: first line colored, rest plain (acceptable limitation)
        visual_lines.push(Line::from(full_line_spans));
        for wrapped_part in wrapped_input.iter().skip(1) {
            visual_lines.push(Line::from(wrapped_part.clone()));
        }
    }

    // Calculate scroll to show the bottom
    let total_visual_lines = visual_lines.len();
    let scroll = total_visual_lines.saturating_sub(available_height);

    // Take visible lines
    let visible_lines: Vec<Line> = visual_lines.into_iter().skip(scroll).collect();

    // Render the visible lines
    let paragraph = Paragraph::new(visible_lines);
    f.render_widget(paragraph, inner_area);

    // Calculate cursor position
    let prompt_width = prompt_str.width();
    let input_before_cursor = &app.input[..app
        .input
        .char_indices()
        .nth(app.cursor_pos)
        .map(|(pos, _)| pos)
        .unwrap_or(app.input.len())];
    let cursor_visual_pos = prompt_width + input_before_cursor.width();

    // Which wrapped line within the input is the cursor on?
    let cursor_line_offset = cursor_visual_pos / width;
    let cursor_x_offset = cursor_visual_pos % width;

    // Absolute line number where cursor is
    let cursor_line_absolute = input_line_start + cursor_line_offset;

    // Relative to visible area
    let cursor_line_visible = cursor_line_absolute.saturating_sub(scroll);

    // Set cursor position
    let cursor_x = inner_area.x + cursor_x_offset as u16;
    let cursor_y = inner_area.y + cursor_line_visible as u16;

    // Ensure cursor is visible
    if cursor_line_visible < available_height {
        f.set_cursor(cursor_x, cursor_y);
    }
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
            Style::default().fg(Color::Black).bg(Palette::BORDER_ACTIVE),
        ),
        Span::raw(" "),
        Span::styled(
            app.current_dir.display().to_string(),
            Style::default().fg(Palette::TEXT_MUTED),
        ),
    ]);

    let paragraph = Paragraph::new(status);
    f.render_widget(paragraph, area);
}
