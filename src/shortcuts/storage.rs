use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to read shortcuts file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse shortcuts file: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Config directory not found")]
    ConfigDirNotFound,
}

/// A single directory shortcut
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Shortcut {
    /// The directory path
    pub path: PathBuf,
    /// When the shortcut was last accessed
    pub last_accessed: DateTime<Utc>,
    /// When the shortcut was created
    pub created_at: DateTime<Utc>,
}

impl Shortcut {
    /// Create a new shortcut with the current time
    pub fn new(path: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            path,
            last_accessed: now,
            created_at: now,
        }
    }

    /// Update the last accessed time to now
    pub fn touch(&mut self) {
        self.last_accessed = Utc::now();
    }

    /// Get a display name for the shortcut (abbreviated path)
    pub fn display_name(&self) -> String {
        let path_str = self.path.display().to_string();

        if let Some(home) = dirs::home_dir() {
            let home_str = home.display().to_string();
            if path_str.starts_with(&home_str) {
                return path_str.replacen(&home_str, "~", 1);
            }
        }

        path_str
    }

    /// Get a human-readable relative time since last access
    ///
    /// Returns strings like "now", "5m", "2h", "3d", "2w", "1mo"
    pub fn time_ago(&self) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_accessed);

        let seconds = duration.num_seconds();
        if seconds < 60 {
            return "now".to_string();
        }

        let minutes = duration.num_minutes();
        if minutes < 60 {
            return format!("{}m", minutes);
        }

        let hours = duration.num_hours();
        if hours < 24 {
            return format!("{}h", hours);
        }

        let days = duration.num_days();
        if days < 7 {
            return format!("{}d", days);
        }

        let weeks = days / 7;
        if weeks < 4 {
            return format!("{}w", weeks);
        }

        let months = days / 30;
        format!("{}mo", months.max(1))
    }
}

/// Container for all shortcuts (for JSON serialization)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShortcutsData {
    pub shortcuts: Vec<Shortcut>,
}

/// Get the path to the shortcuts config file
pub fn get_config_path() -> Result<PathBuf, StorageError> {
    let config_dir = dirs::config_dir().ok_or(StorageError::ConfigDirNotFound)?;
    Ok(config_dir.join("jerm").join("shortcuts.json"))
}

/// Ensure the config directory exists
pub fn ensure_config_dir() -> Result<PathBuf, StorageError> {
    let config_path = get_config_path()?;
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(config_path)
}

/// Load shortcuts from disk
pub fn load_shortcuts() -> Result<ShortcutsData, StorageError> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(ShortcutsData::default());
    }

    let contents = fs::read_to_string(&config_path)?;
    let data: ShortcutsData = serde_json::from_str(&contents)?;
    Ok(data)
}

/// Save shortcuts to disk
pub fn save_shortcuts(data: &ShortcutsData) -> Result<(), StorageError> {
    let config_path = ensure_config_dir()?;
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(config_path, contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_new() {
        let shortcut = Shortcut::new(PathBuf::from("/tmp"));
        assert_eq!(shortcut.path, PathBuf::from("/tmp"));
        assert!(shortcut.created_at <= Utc::now());
        assert_eq!(shortcut.created_at, shortcut.last_accessed);
    }

    #[test]
    fn test_shortcut_touch() {
        let mut shortcut = Shortcut::new(PathBuf::from("/tmp"));
        let original_accessed = shortcut.last_accessed;
        std::thread::sleep(std::time::Duration::from_millis(10));
        shortcut.touch();
        assert!(shortcut.last_accessed >= original_accessed);
    }

    #[test]
    fn test_display_name() {
        let shortcut = Shortcut::new(PathBuf::from("/tmp"));
        assert_eq!(shortcut.display_name(), "/tmp");
    }

    #[test]
    fn test_serialization() {
        let data = ShortcutsData {
            shortcuts: vec![Shortcut::new(PathBuf::from("/tmp"))],
        };

        let json = serde_json::to_string(&data).unwrap();
        let parsed: ShortcutsData = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.shortcuts.len(), 1);
        assert_eq!(parsed.shortcuts[0].path, PathBuf::from("/tmp"));
    }

    #[test]
    fn test_time_ago_now() {
        let shortcut = Shortcut::new(PathBuf::from("/tmp"));
        assert_eq!(shortcut.time_ago(), "now");
    }

    #[test]
    fn test_time_ago_minutes() {
        use chrono::Duration;
        let mut shortcut = Shortcut::new(PathBuf::from("/tmp"));
        shortcut.last_accessed = Utc::now() - Duration::minutes(5);
        assert_eq!(shortcut.time_ago(), "5m");
    }

    #[test]
    fn test_time_ago_hours() {
        use chrono::Duration;
        let mut shortcut = Shortcut::new(PathBuf::from("/tmp"));
        shortcut.last_accessed = Utc::now() - Duration::hours(2);
        assert_eq!(shortcut.time_ago(), "2h");
    }

    #[test]
    fn test_time_ago_days() {
        use chrono::Duration;
        let mut shortcut = Shortcut::new(PathBuf::from("/tmp"));
        shortcut.last_accessed = Utc::now() - Duration::days(3);
        assert_eq!(shortcut.time_ago(), "3d");
    }

    #[test]
    fn test_time_ago_weeks() {
        use chrono::Duration;
        let mut shortcut = Shortcut::new(PathBuf::from("/tmp"));
        shortcut.last_accessed = Utc::now() - Duration::weeks(2);
        assert_eq!(shortcut.time_ago(), "2w");
    }

    #[test]
    fn test_time_ago_months() {
        use chrono::Duration;
        let mut shortcut = Shortcut::new(PathBuf::from("/tmp"));
        shortcut.last_accessed = Utc::now() - Duration::days(45);
        assert_eq!(shortcut.time_ago(), "1mo");
    }
}
