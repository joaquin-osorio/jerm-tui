use std::fs;
use std::path::PathBuf;

/// Entry in a directory listing
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Name of the entry
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// Whether this is a directory
    pub is_dir: bool,
}

/// State for the cd -list navigation mode
#[derive(Debug, Clone)]
pub struct NavigationState {
    /// Current virtual directory being browsed
    pub current_path: PathBuf,
    /// Entries in the current directory
    pub entries: Vec<DirEntry>,
    /// Currently selected index
    pub selected_index: usize,
    /// Scroll offset for long lists
    pub scroll_offset: usize,
}

impl NavigationState {
    /// Create a new navigation state
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::new(),
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
        }
    }

    /// Start navigation from a given path
    pub fn start_navigation(&mut self, path: PathBuf) {
        self.current_path = path;
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.refresh_entries();
    }

    /// Refresh the entries list from the current path
    pub fn refresh_entries(&mut self) {
        self.entries.clear();

        // Add parent directory entry if not at root
        if self.current_path.parent().is_some() {
            self.entries.push(DirEntry {
                name: "..".to_string(),
                path: self.current_path.parent().unwrap().to_path_buf(),
                is_dir: true,
            });
        }

        // Read directory entries
        if let Ok(read_dir) = fs::read_dir(&self.current_path) {
            let mut dirs: Vec<DirEntry> = read_dir
                .filter_map(std::result::Result::ok)
                .filter_map(|entry| {
                    let path = entry.path();
                    let is_dir = path.is_dir();

                    // Only show directories in cd -list mode
                    if !is_dir {
                        return None;
                    }

                    let name = entry.file_name().to_string_lossy().to_string();

                    // Skip hidden directories by default
                    if name.starts_with('.') {
                        return None;
                    }

                    Some(DirEntry { name, path, is_dir })
                })
                .collect();

            // Sort alphabetically
            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            self.entries.extend(dirs);
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.entries.len() {
            self.selected_index = self.entries.len().saturating_sub(1);
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;

            // Adjust scroll if needed
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected_index < self.entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Adjust scroll offset for visible height
    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        } else if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }

    /// Enter the selected directory (right arrow)
    pub fn enter_selected(&mut self) {
        if let Some(entry) = self.entries.get(self.selected_index) {
            if entry.is_dir && entry.name != ".." {
                self.current_path = entry.path.clone();
                self.selected_index = 0;
                self.scroll_offset = 0;
                self.refresh_entries();
            }
        }
    }

    /// Go up one level (left arrow)
    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.selected_index = 0;
            self.scroll_offset = 0;
            self.refresh_entries();
        }
    }

    /// Get the currently selected path (for confirmation)
    pub fn get_selected_path(&self) -> Option<PathBuf> {
        self.entries
            .get(self.selected_index)
            .map(|e| e.path.clone())
    }

    /// Get visible entries based on scroll offset
    pub fn get_visible_entries(&self, visible_height: usize) -> Vec<(usize, &DirEntry)> {
        self.entries
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(visible_height)
            .collect()
    }

    /// Check if a given index is selected
    pub fn is_selected(&self, index: usize) -> bool {
        index == self.selected_index
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_state_new() {
        let state = NavigationState::new();
        assert!(state.entries.is_empty());
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_start_navigation() {
        let mut state = NavigationState::new();
        state.start_navigation(PathBuf::from("/tmp"));
        assert_eq!(state.current_path, PathBuf::from("/tmp"));
    }

    #[test]
    fn test_move_up_at_zero() {
        let mut state = NavigationState::new();
        state.move_up();
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_move_down_empty() {
        let mut state = NavigationState::new();
        state.move_down();
        assert_eq!(state.selected_index, 0);
    }
}
