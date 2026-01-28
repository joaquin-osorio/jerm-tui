use std::path::PathBuf;

use super::storage::{load_shortcuts, save_shortcuts, Shortcut, ShortcutsData};

/// Manages directory shortcuts
pub struct ShortcutManager {
    data: ShortcutsData,
}

impl ShortcutManager {
    /// Create a new shortcut manager, loading existing shortcuts from disk
    pub fn new() -> Self {
        let data = load_shortcuts().unwrap_or_default();
        Self { data }
    }

    /// Get all shortcuts, sorted by last accessed (most recent first)
    pub fn get_shortcuts(&self) -> Vec<&Shortcut> {
        let mut shortcuts: Vec<_> = self.data.shortcuts.iter().collect();
        shortcuts.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        shortcuts
    }

    /// Get a shortcut by index (1-based, for Ctrl+1 through Ctrl+9)
    pub fn get_shortcut(&self, index: usize) -> Option<&Shortcut> {
        if index == 0 || index > 9 {
            return None;
        }
        self.get_shortcuts().get(index - 1).copied()
    }

    /// Add a new shortcut or update existing one's access time
    pub fn add_shortcut(&mut self, path: PathBuf) {
        // Check if shortcut already exists
        if let Some(existing) = self.data.shortcuts.iter_mut().find(|s| s.path == path) {
            existing.touch();
        } else {
            self.data.shortcuts.push(Shortcut::new(path));
        }

        // Save to disk
        let _ = save_shortcuts(&self.data);
    }

    /// Update the access time for a shortcut
    pub fn touch_shortcut(&mut self, path: &PathBuf) {
        if let Some(shortcut) = self.data.shortcuts.iter_mut().find(|s| &s.path == path) {
            shortcut.touch();
            let _ = save_shortcuts(&self.data);
        }
    }

    /// Remove a shortcut by path
    #[allow(dead_code)]
    pub fn remove_shortcut(&mut self, path: &PathBuf) {
        self.data.shortcuts.retain(|s| &s.path != path);
        let _ = save_shortcuts(&self.data);
    }

    /// Get the number of shortcuts
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.data.shortcuts.len()
    }

    /// Check if there are no shortcuts
    pub fn is_empty(&self) -> bool {
        self.data.shortcuts.is_empty()
    }

    /// Reload shortcuts from disk
    #[allow(dead_code)]
    pub fn reload(&mut self) {
        if let Ok(data) = load_shortcuts() {
            self.data = data;
        }
    }
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_new() {
        let manager = ShortcutManager::new();
        // Just verify it doesn't crash
        let _ = manager.get_shortcuts();
    }

    #[test]
    fn test_get_shortcut_bounds() {
        let manager = ShortcutManager::new();
        assert!(manager.get_shortcut(0).is_none());
        assert!(manager.get_shortcut(10).is_none());
    }
}
