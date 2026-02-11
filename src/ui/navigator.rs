use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::navigation::NavigationState;
use crate::theme::Palette;

/// Render the cd -list navigation overlay
pub fn render_navigator(f: &mut Frame, area: Rect, nav: &mut NavigationState) {
    // Clear the area first
    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Palette::BORDER_ACTIVE))
        .title(" Select Directory ");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if inner_area.height < 3 {
        return;
    }

    // Reserve space for header and footer
    let header_height = 1;
    let footer_height = 2;
    let list_height = (inner_area.height as usize)
        .saturating_sub(header_height)
        .saturating_sub(footer_height);

    // Render current path header
    let header_area = Rect {
        x: inner_area.x,
        y: inner_area.y,
        width: inner_area.width,
        height: 1,
    };

    let current_path_display = nav.current_path.display().to_string();
    let path_text = if current_path_display.len() > inner_area.width as usize - 2 {
        format!(
            "..{}",
            &current_path_display[current_path_display.len() - (inner_area.width as usize - 4)..]
        )
    } else {
        current_path_display
    };

    let header = Paragraph::new(Line::from(Span::styled(
        path_text,
        Style::default()
            .fg(Palette::NAV_HEADER)
            .add_modifier(Modifier::BOLD),
    )));
    f.render_widget(header, header_area);

    // Adjust scroll for visible height
    nav.adjust_scroll(list_height);

    // Render directory list
    let list_area = Rect {
        x: inner_area.x,
        y: inner_area.y + header_height as u16,
        width: inner_area.width,
        height: list_height as u16,
    };

    let visible_entries = nav.get_visible_entries(list_height);

    let items: Vec<ListItem> = visible_entries
        .iter()
        .map(|(idx, entry)| {
            let is_selected = nav.is_selected(*idx);

            let style = if is_selected {
                Style::default()
                    .fg(Palette::NAV_SELECTED_FG)
                    .bg(Palette::NAV_SELECTED_BG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Palette::TEXT_NORMAL)
            };

            let prefix = if is_selected { "> " } else { "  " };
            let _icon = if entry.name == ".." {
                "\u{2191} " // Up arrow
            } else {
                "\u{1F4C1} " // Folder icon (may not render in all terminals)
            };

            // Fallback to simple text if icons don't work
            let display = format!("{}{}{}", prefix, "", entry.name);

            ListItem::new(Line::from(Span::styled(display, style)))
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, list_area);

    // Render footer with key hints
    let footer_area = Rect {
        x: inner_area.x,
        y: inner_area.y + header_height as u16 + list_height as u16,
        width: inner_area.width,
        height: footer_height as u16,
    };

    let hint_style = Style::default().fg(Palette::TEXT_MUTED);
    let key_style = Style::default().fg(Palette::NAV_KEY_HINT);

    let footer_lines = vec![
        Line::from(vec![
            Span::styled("\u{2191}\u{2193}", key_style),
            Span::styled(" move  ", hint_style),
            Span::styled("\u{2192}", key_style),
            Span::styled(" enter  ", hint_style),
            Span::styled("\u{2190}", key_style),
            Span::styled(" up", hint_style),
        ]),
        Line::from(vec![
            Span::styled("Enter", key_style),
            Span::styled(" confirm  ", hint_style),
            Span::styled("Esc", key_style),
            Span::styled(" cancel", hint_style),
        ]),
    ];

    let footer = Paragraph::new(footer_lines);
    f.render_widget(footer, footer_area);
}
