use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

use ratatui::style::Style;
use ratatui::text::Span;

use crate::git::{spawn_git_worker, GitMessage, GitStatus};
use crate::theme::Palette;
use crate::navigation::directory::NavigationState;
use crate::shortcuts::manager::ShortcutManager;

/// Application modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// Normal terminal mode - typing commands
    Normal,
    /// Navigation list mode - browsing directories with arrow keys
    NavigationList,
    /// Shortcut selection mode - navigating shortcuts with arrow keys
    ShortcutSelection,
}

/// Main application state
pub struct App {
    /// Current working directory
    pub current_dir: PathBuf,
    /// Command history
    pub history: Vec<String>,
    /// Current position in history (for up/down navigation)
    pub history_index: Option<usize>,
    /// Current input buffer
    pub input: String,
    /// Cursor position in input
    pub cursor_pos: usize,
    /// Output buffer (terminal output lines)
    pub output: Vec<String>,
    /// Current application mode
    pub mode: AppMode,
    /// Navigation state for cd -list mode
    pub navigation_state: NavigationState,
    /// Shortcut manager
    pub shortcuts: ShortcutManager,
    /// Selected shortcut index for goto mode
    pub selected_shortcut_index: usize,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Scroll offset for output (reserved for future use)
    #[allow(dead_code)]
    pub output_scroll: usize,
    /// Git status for current directory
    pub git_status: Option<GitStatus>,
    /// Channel to send messages to git worker
    git_tx: Sender<GitMessage>,
    /// Channel to receive messages from git worker
    git_rx: Receiver<GitMessage>,
    /// Last time git was polled
    last_git_poll: Instant,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let shortcuts = ShortcutManager::new();
        let (git_tx, git_rx) = spawn_git_worker();

        let mut app = Self {
            current_dir: current_dir.clone(),
            history: Vec::new(),
            history_index: None,
            input: String::new(),
            cursor_pos: 0,
            output: Vec::new(),
            mode: AppMode::Normal,
            navigation_state: NavigationState::new(),
            shortcuts,
            selected_shortcut_index: 0,
            should_quit: false,
            output_scroll: 0,
            git_status: None,
            git_tx,
            git_rx,
            last_git_poll: Instant::now(),
        };

        // Trigger initial git status
        app.refresh_git_status(false);
        app
    }

    /// Refresh git status for current directory
    pub fn refresh_git_status(&mut self, with_fetch: bool) {
        let _ = self.git_tx.send(GitMessage::UpdateStatus {
            dir: self.current_dir.display().to_string(),
            with_fetch,
        });
    }

    /// Poll for git updates from worker thread
    pub fn poll_git_updates(&mut self) {
        // Drain all messages from git_rx
        while let Ok(msg) = self.git_rx.try_recv() {
            if let GitMessage::StatusUpdate(status) = msg {
                self.git_status = status;
            }
        }

        // Check 30s interval for background fetch
        if self.last_git_poll.elapsed() >= Duration::from_secs(30) {
            self.refresh_git_status(true); // with fetch
            self.last_git_poll = Instant::now();
        }
    }

    /// Get prompt as styled spans for colored rendering
    pub fn prompt_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        // Directory (with ~ replacement)
        let dir = self.current_dir.display().to_string();
        let home = dirs::home_dir().map(|h| h.display().to_string());

        let display_dir = if let Some(home_path) = home {
            if dir.starts_with(&home_path) {
                dir.replacen(&home_path, "~", 1)
            } else {
                dir
            }
        } else {
            dir
        };

        spans.push(Span::raw(display_dir));
        spans.push(Span::raw(" "));

        // Git info
        if let Some(ref git) = self.git_status {
            if !git.branch.is_empty() {
                // Branch in muted gray
                spans.push(Span::styled(
                    git.branch.clone(),
                    Style::default().fg(Palette::GIT_BRANCH),
                ));

                // Dirty indicator
                if git.is_dirty {
                    spans.push(Span::styled(
                        "*".to_string(),
                        Style::default().fg(Palette::GIT_BRANCH),
                    ));
                }

                // Ahead indicator in cyan
                if git.ahead > 0 {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        format!("↑{}", git.ahead),
                        Style::default().fg(Palette::GIT_AHEAD_BEHIND),
                    ));
                }

                // Behind indicator in cyan
                if git.behind > 0 {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(
                        format!("↓{}", git.behind),
                        Style::default().fg(Palette::GIT_AHEAD_BEHIND),
                    ));
                }

                spans.push(Span::raw(" "));
            }
        }

        spans.push(Span::raw("$ "));
        spans
    }

    /// Get prompt as plain string (for output history and cursor calculations)
    pub fn prompt_string(&self) -> String {
        self.prompt_spans()
            .iter()
            .map(|s| s.content.as_ref())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Get the current prompt string (backward compatibility)
    pub fn prompt(&self) -> String {
        self.prompt_string()
    }

    /// Add a line to the output buffer
    pub fn add_output(&mut self, line: &str) {
        self.output.push(line.to_string());
    }

    /// Add the current command to output (with prompt)
    pub fn add_command_to_output(&mut self, command: &str) {
        let prompt = self.prompt();
        self.add_output(&format!("{prompt}{command}"));
    }

    /// Clear the input buffer
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_pos = 0;
        self.history_index = None;
    }

    /// Add command to history
    pub fn add_to_history(&mut self, command: &str) {
        if !command.trim().is_empty() {
            // Don't add duplicates of the last command
            if self.history.last().map(String::as_str) != Some(command) {
                self.history.push(command.to_string());
            }
        }
    }

    /// Navigate to previous command in history
    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => self.history.len().saturating_sub(1),
            Some(0) => 0,
            Some(i) => i.saturating_sub(1),
        };

        self.history_index = Some(new_index);
        self.input = self.history[new_index].clone();
        self.cursor_pos = self.input.len();
    }

    /// Navigate to next command in history
    pub fn history_next(&mut self) {
        match self.history_index {
            None => {}
            Some(i) if i >= self.history.len().saturating_sub(1) => {
                self.history_index = None;
                self.input.clear();
                self.cursor_pos = 0;
            }
            Some(i) => {
                let new_index = i + 1;
                self.history_index = Some(new_index);
                self.input = self.history[new_index].clone();
                self.cursor_pos = self.input.len();
            }
        }
    }

    /// Insert a character at the cursor position
    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    /// Delete the character before the cursor
    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.input.remove(self.cursor_pos);
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        if self.cursor_pos < self.input.len() {
            self.cursor_pos += 1;
        }
    }

    /// Move cursor to start of input
    pub fn cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    /// Move cursor to end of input
    pub fn cursor_end(&mut self) {
        self.cursor_pos = self.input.len();
    }

    /// Enter navigation list mode
    pub fn enter_navigation_mode(&mut self) {
        self.mode = AppMode::NavigationList;
        self.navigation_state
            .start_navigation(self.current_dir.clone());
    }

    /// Exit navigation list mode
    pub fn exit_navigation_mode(&mut self) {
        self.mode = AppMode::Normal;
    }

    /// Confirm navigation and change to selected directory
    pub fn confirm_navigation(&mut self) {
        if let Some(selected_path) = self.navigation_state.get_selected_path() {
            self.current_dir = selected_path;
        }
        self.exit_navigation_mode();
    }

    /// Enter shortcut selection mode
    pub fn enter_goto_mode(&mut self) {
        if !self.shortcuts.is_empty() {
            self.mode = AppMode::ShortcutSelection;
            self.selected_shortcut_index = 0;
        }
    }

    /// Exit shortcut selection mode
    pub fn exit_goto_mode(&mut self) {
        self.mode = AppMode::Normal;
    }

    /// Move selection up in shortcut list
    pub fn goto_move_up(&mut self) {
        if self.selected_shortcut_index > 0 {
            self.selected_shortcut_index -= 1;
        }
    }

    /// Move selection down in shortcut list
    pub fn goto_move_down(&mut self) {
        let max_index = self.shortcuts.get_shortcuts().len().saturating_sub(1).min(8);
        if self.selected_shortcut_index < max_index {
            self.selected_shortcut_index += 1;
        }
    }

    /// Confirm shortcut selection and navigate
    pub fn confirm_goto(&mut self) {
        if let Some(shortcut) = self.shortcuts.get_shortcut(self.selected_shortcut_index + 1) {
            let path = shortcut.path.clone();
            if path.is_dir() {
                self.add_output(&format!("cd {}", path.display()));
                self.current_dir = path.clone();
                self.shortcuts.touch_shortcut(&path);
            } else {
                self.add_output(&format!("Error: {} no longer exists", path.display()));
            }
        }
        self.exit_goto_mode();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // Send shutdown message to git worker thread
        let _ = self.git_tx.send(GitMessage::Shutdown);
    }
}
