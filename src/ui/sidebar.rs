use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::shortcuts::ShortcutManager;
use crate::theme::{Icons, Palette};

/// Render the shortcuts sidebar
pub fn render_sidebar(
    f: &mut Frame,
    area: Rect,
    shortcuts: &ShortcutManager,
    selected_index: Option<usize>,
) {
    let icons = Icons::new();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Palette::BORDER_DEFAULT))
        .title(" Shortcuts ");

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if shortcuts.is_empty() {
        // Show help text when no shortcuts
        let help_items = vec![
            ListItem::new(Line::from(Span::styled(
                "No shortcuts",
                Style::default().fg(Palette::TEXT_MUTED),
            ))),
            ListItem::new(Line::from("")),
            ListItem::new(Line::from(Span::styled(
                "jerm save to add",
                Style::default().fg(Palette::TEXT_MUTED),
            ))),
        ];

        let help_list = List::new(help_items);
        f.render_widget(help_list, inner_area);
        return;
    }

    // Get shortcuts sorted by last accessed
    let shortcut_list = shortcuts.get_shortcuts();
    let inner_width = inner_area.width as usize;

    // Create list items with numbers, icons, paths, and times
    let items: Vec<ListItem> = shortcut_list
        .iter()
        .take(9) // Only show first 9 (for Ctrl+1 through Ctrl+9)
        .enumerate()
        .map(|(i, shortcut)| {
            let is_selected = selected_index == Some(i);

            let number_style = if is_selected {
                Style::default()
                    .fg(Palette::SIDEBAR_NUMBER)
                    .bg(Palette::BG_SELECTED)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Palette::SIDEBAR_NUMBER)
                    .add_modifier(Modifier::BOLD)
            };

            let path_style = if is_selected {
                Style::default()
                    .fg(Palette::SIDEBAR_PATH)
                    .bg(Palette::BG_SELECTED)
            } else {
                Style::default().fg(Palette::SIDEBAR_PATH)
            };

            let time_style = if is_selected {
                Style::default()
                    .fg(Palette::SIDEBAR_TIME)
                    .bg(Palette::BG_SELECTED)
            } else {
                Style::default().fg(Palette::SIDEBAR_TIME)
            };

            let display_name = shortcut.display_name();
            let time_ago = shortcut.time_ago();

            // Layout: [num] [icon] [path...] [time]
            // num: 2 chars ("1 ")
            // icon: 2 chars if nerd fonts (" " or "~ "), else 0
            // time: variable (right-aligned)

            let icon = if display_name.starts_with('~') {
                icons.home()
            } else {
                icons.folder()
            };

            let icon_width = if icons.has_nerd_fonts() { 2 } else { 0 };
            let num_width = 2; // "1 "
            let time_width = time_ago.len() + 1; // " 2h"

            // Only show time if we have enough width (at least 20 chars)
            let show_time = inner_width >= 20;

            let available_for_path = if show_time {
                inner_width
                    .saturating_sub(num_width)
                    .saturating_sub(icon_width)
                    .saturating_sub(time_width)
            } else {
                inner_width
                    .saturating_sub(num_width)
                    .saturating_sub(icon_width)
            };

            // Truncate path if needed
            let truncated_path = if display_name.len() > available_for_path {
                if available_for_path > 3 {
                    format!(
                        "..{}",
                        &display_name[display_name.len() - (available_for_path - 2)..]
                    )
                } else {
                    display_name.chars().take(available_for_path).collect()
                }
            } else {
                display_name.clone()
            };

            // Calculate padding for right-aligned time
            let path_len = truncated_path.len();
            let padding_len = if show_time {
                available_for_path.saturating_sub(path_len)
            } else {
                0
            };
            let padding = " ".repeat(padding_len);

            let mut spans = vec![Span::styled(format!("{} ", i + 1), number_style)];

            if icons.has_nerd_fonts() {
                spans.push(Span::styled(format!("{} ", icon), path_style));
            }

            spans.push(Span::styled(truncated_path, path_style));

            if show_time {
                spans.push(Span::styled(padding, path_style));
                spans.push(Span::styled(format!(" {}", time_ago), time_style));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);
}
