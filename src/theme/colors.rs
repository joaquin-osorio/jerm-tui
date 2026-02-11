//! Cold-tone color palette for Jerm UI
//!
//! Inspired by Warp Terminal with blues, cyans, and teals.

use ratatui::style::Color;

/// Color palette for the entire application
pub struct Palette;

impl Palette {
    // ─────────────────────────────────────────────────────────────────────────
    // UI Colors
    // ─────────────────────────────────────────────────────────────────────────

    /// Default border color (muted blue-gray)
    pub const BORDER_DEFAULT: Color = Color::Rgb(88, 110, 130);

    /// Active/focused border color (bright cyan)
    pub const BORDER_ACTIVE: Color = Color::Rgb(80, 200, 220);

    /// Muted text for hints and secondary info
    pub const TEXT_MUTED: Color = Color::Rgb(100, 120, 140);

    /// Normal text color
    pub const TEXT_NORMAL: Color = Color::Rgb(200, 210, 220);

    /// Highlighted/selected background
    pub const BG_SELECTED: Color = Color::Rgb(45, 65, 85);

    // ─────────────────────────────────────────────────────────────────────────
    // Syntax Highlighting Colors
    // ─────────────────────────────────────────────────────────────────────────

    /// Command names (blue)
    pub const SYNTAX_COMMAND: Color = Color::Rgb(100, 160, 240);

    /// Flags like --help, -v (orange/amber)
    pub const SYNTAX_FLAG: Color = Color::Rgb(230, 160, 80);

    /// File paths (teal/cyan)
    pub const SYNTAX_PATH: Color = Color::Rgb(80, 200, 180);

    /// Quoted strings (yellow/gold)
    pub const SYNTAX_STRING: Color = Color::Rgb(230, 200, 100);

    /// Numbers (purple/lavender)
    pub const SYNTAX_NUMBER: Color = Color::Rgb(180, 140, 220);

    /// Operators like |, >, &&, etc. (light gray)
    pub const SYNTAX_OPERATOR: Color = Color::Rgb(160, 170, 180);

    /// Plain text (default)
    pub const SYNTAX_TEXT: Color = Color::Rgb(200, 210, 220);

    // ─────────────────────────────────────────────────────────────────────────
    // Git Colors
    // ─────────────────────────────────────────────────────────────────────────

    /// Git branch name (gray)
    pub const GIT_BRANCH: Color = Color::Rgb(140, 150, 160);

    /// Git ahead/behind indicators (cyan)
    pub const GIT_AHEAD_BEHIND: Color = Color::Rgb(80, 200, 220);

    // ─────────────────────────────────────────────────────────────────────────
    // Sidebar Colors
    // ─────────────────────────────────────────────────────────────────────────

    /// Shortcut number (bright cyan)
    pub const SIDEBAR_NUMBER: Color = Color::Rgb(80, 200, 220);

    /// Shortcut path (normal text)
    pub const SIDEBAR_PATH: Color = Color::Rgb(200, 210, 220);

    /// Relative time indicator (muted)
    pub const SIDEBAR_TIME: Color = Color::Rgb(100, 120, 140);

    // ─────────────────────────────────────────────────────────────────────────
    // Navigator Colors
    // ─────────────────────────────────────────────────────────────────────────

    /// Header path in navigator (amber/gold)
    pub const NAV_HEADER: Color = Color::Rgb(230, 180, 100);

    /// Selected item background (dark blue)
    pub const NAV_SELECTED_BG: Color = Color::Rgb(40, 80, 120);

    /// Selected item foreground (bright white)
    pub const NAV_SELECTED_FG: Color = Color::Rgb(240, 245, 250);

    /// Key hints (cyan)
    pub const NAV_KEY_HINT: Color = Color::Rgb(80, 200, 220);
}
