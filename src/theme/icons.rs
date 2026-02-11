//! Icon definitions with Nerd Font support
//!
//! Provides icons with fallback variants for terminals without Nerd Fonts.

use std::env;

/// A pair of icons: nerd font version and fallback
#[derive(Debug, Clone, Copy)]
pub struct IconPair {
    /// Nerd Font icon
    pub nerd: &'static str,
    /// Fallback for terminals without Nerd Fonts
    pub fallback: &'static str,
}

impl IconPair {
    /// Create a new icon pair
    pub const fn new(nerd: &'static str, fallback: &'static str) -> Self {
        Self { nerd, fallback }
    }
}

/// Icon collection with runtime Nerd Font detection
#[derive(Debug, Clone)]
pub struct Icons {
    use_nerd_fonts: bool,
}

impl Icons {
    /// Folder icon
    pub const FOLDER: IconPair = IconPair::new("\u{f07b}", ""); //

    /// Home directory icon
    pub const HOME: IconPair = IconPair::new("\u{f015}", "~"); //

    /// Git branch icon
    pub const GIT_BRANCH: IconPair = IconPair::new("\u{e725}", ""); //

    /// Up arrow (for parent directory)
    pub const UP_ARROW: IconPair = IconPair::new("\u{f062}", ".."); //

    /// Create Icons with Nerd Font detection
    pub fn new() -> Self {
        Self {
            use_nerd_fonts: detect_nerd_font_support(),
        }
    }

    /// Get the appropriate folder icon
    pub fn folder(&self) -> &'static str {
        if self.use_nerd_fonts {
            Self::FOLDER.nerd
        } else {
            Self::FOLDER.fallback
        }
    }

    /// Get the appropriate home icon
    pub fn home(&self) -> &'static str {
        if self.use_nerd_fonts {
            Self::HOME.nerd
        } else {
            Self::HOME.fallback
        }
    }

    /// Get the appropriate git branch icon
    #[allow(dead_code)]
    pub fn git_branch(&self) -> &'static str {
        if self.use_nerd_fonts {
            Self::GIT_BRANCH.nerd
        } else {
            Self::GIT_BRANCH.fallback
        }
    }

    /// Get the appropriate up arrow icon
    #[allow(dead_code)]
    pub fn up_arrow(&self) -> &'static str {
        if self.use_nerd_fonts {
            Self::UP_ARROW.nerd
        } else {
            Self::UP_ARROW.fallback
        }
    }

    /// Check if Nerd Fonts are enabled
    pub fn has_nerd_fonts(&self) -> bool {
        self.use_nerd_fonts
    }
}

impl Default for Icons {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect if Nerd Font support is enabled via environment variable
///
/// Set `JERM_NERD_FONTS=1` to enable Nerd Font icons
pub fn detect_nerd_font_support() -> bool {
    env::var("JERM_NERD_FONTS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_pair_creation() {
        let pair = IconPair::new("", "DIR");
        assert_eq!(pair.nerd, "");
        assert_eq!(pair.fallback, "DIR");
    }

    #[test]
    fn test_icons_fallback_default() {
        // Without env var set, should use fallback
        let icons = Icons { use_nerd_fonts: false };
        assert_eq!(icons.folder(), "");
        assert_eq!(icons.home(), "~");
    }

    #[test]
    fn test_icons_nerd_fonts() {
        let icons = Icons { use_nerd_fonts: true };
        assert_eq!(icons.folder(), "\u{f07b}");
        assert_eq!(icons.home(), "\u{f015}");
    }
}
