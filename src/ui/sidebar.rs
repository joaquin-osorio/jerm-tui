use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::shortcuts::ShortcutManager;

/// Render the shortcuts sidebar
pub fn render_sidebar(f: &mut Frame, area: Rect, shortcuts: &ShortcutManager) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" Shortcuts ");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if shortcuts.is_empty() {
        // Show help text when no shortcuts
        let help_items = vec![
            ListItem::new(Line::from(Span::styled(
                "No shortcuts",
                Style::default().fg(Color::DarkGray),
            ))),
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(Span::styled(
                "Cmd+I to add",
                Style::default().fg(Color::DarkGray),
            ))),
        ];

        let help_list = List::new(help_items);
        f.render_widget(help_list, inner_area);
        return;
    }

    // Get shortcuts sorted by last accessed
    let shortcut_list = shortcuts.get_shortcuts();

    // Create list items with numbers
    let items: Vec<ListItem> = shortcut_list
        .iter()
        .take(9) // Only show first 9 (for Ctrl+1 through Ctrl+9)
        .enumerate()
        .map(|(i, shortcut)| {
            let number_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);

            let path_style = Style::default().fg(Color::White);

            let display_name = shortcut.display_name();

            // Truncate path if too long for sidebar
            let max_width = (area.width as usize).saturating_sub(6); // Account for borders and number
            let truncated = if display_name.len() > max_width {
                format!("..{}", &display_name[display_name.len() - max_width + 2..])
            } else {
                display_name
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", i + 1), number_style),
                Span::styled(truncated, path_style),
            ]))
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);
}
