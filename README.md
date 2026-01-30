# Jerm TUI

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![macOS](https://img.shields.io/badge/platform-macOS-lightgrey.svg)](https://www.apple.com/macos/)

A modern terminal emulator for macOS with smart directory navigation, instant shortcuts, and Git integration. Navigate your filesystem visually with `cd -list`, save your favorite directories, jump between them with a single keystroke, and see your Git status at a glance with a beautiful, Git-aware prompt.

## ✨ Features

### 🧭 Visual Directory Navigation
- **Interactive cd mode**: Enter `cd -list` to browse directories with arrow keys
- **Real-time preview**: Navigate through subdirectories before committing
- **Intuitive controls**: Use arrow keys to move, Enter to confirm, Escape to cancel

### ⚡ Smart Shortcuts
- **Quick save**: Save any directory with `jerm save`
- **Instant access**: Use `Ctrl+1` through `Ctrl+9` to jump to your top 9 shortcuts
- **Auto-sorted**: Shortcuts automatically organize by most recently used
- **Persistent**: Your shortcuts are saved between sessions

### 🔀 Git Integration
- **Git-aware prompt**: Automatically displays current branch name and status
- **Visual indicators**: See at a glance if you have uncommitted changes (`*`)
- **Sync status**: Track commits ahead (↑) and behind (↓) your remote branch
- **Non-blocking**: Git status updates in background without affecting performance
- **Auto-refresh**: Updates after directory changes and every 30 seconds

**Example prompt**: `~/dev/project main* ↑2↓3 $`

### 🖥️ Full Terminal Experience
- Execute any shell command
- Command history with up/down arrows
- Tab completion support
- Standard keyboard shortcuts (Ctrl+C, Ctrl+D, Ctrl+L)

## 🚀 Getting Started

### Prerequisites

- macOS (currently the only supported platform)
- Rust 1.75 or higher

### Installation

#### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/jerm.git
cd jerm

# Build and install
cargo install --path .

# Run
jerm
```

#### Using Cargo

```bash
cargo install jerm
```

## 📖 Usage

### Basic Commands

```bash
# Start Jerm
jerm

# Change directory normally
cd ~/projects

# Visual directory navigation
cd -list

# Save current directory as a shortcut
jerm save

# Enter shortcut selection mode
jerm goto

# Quick jump to shortcuts
Ctrl+1  # Jump to shortcut 1
Ctrl+2  # Jump to shortcut 2
# ... up to Ctrl+9
```

### Navigation Mode (`cd -list`)

When you enter `cd -list`, you'll see an interactive directory browser:

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move selection up/down |
| `→` | Enter selected directory |
| `←` | Go up one level |
| `Enter` | Confirm and navigate to selected directory |
| `Esc` | Cancel and return to previous directory |

### Shortcut Management

Shortcuts are automatically sorted by last access time, keeping your most-used directories at your fingertips.

```bash
# In any directory, save it as a shortcut
jerm save

# Navigate using keyboard shortcuts (fastest)
Ctrl+3  # Jump to your third most recent shortcut

# Or use the interactive selector
jerm goto
# Then use ↑/↓ to select and Enter to confirm
```

### Git Integration

Jerm automatically detects Git repositories and displays status information in your prompt:

```bash
# In a Git repository, your prompt shows:
~/dev/jerm-tui main $          # Clean repository on 'main' branch
~/dev/jerm-tui main* $         # Uncommitted changes (*)
~/dev/jerm-tui main* ↑2 $      # 2 commits ahead of remote
~/dev/jerm-tui main ↓1 $       # 1 commit behind remote
~/dev/jerm-tui main* ↑2↓3 $    # Both ahead and behind

# Detached HEAD shows short commit hash
~/dev/jerm-tui abc1234 $       # Detached HEAD at commit abc1234

# Non-Git directories show no Git info
/tmp $                         # No Git repository detected
```

Git status updates:
- Automatically when you `cd` to a new directory
- After executing shell commands
- Every 30 seconds in the background (with `git fetch`)

## 🏗️ Architecture

Jerm is built with a modular architecture:

```
jerm/
├── src/
│   ├── main.rs           # Application entry point and event loop
│   ├── app.rs            # Core application state and logic
│   ├── ui/               # User interface components
│   │   ├── terminal.rs   # Main terminal rendering with colored prompts
│   │   ├── navigator.rs  # cd -list visual navigator
│   │   └── sidebar.rs    # Shortcuts sidebar
│   ├── git/              # Git integration
│   │   └── status.rs     # Git status queries and background worker
│   ├── shell/            # Shell integration
│   │   ├── executor.rs   # Command execution
│   │   └── parser.rs     # Command parsing
│   ├── navigation/       # Directory navigation
│   │   └── directory.rs  # Navigation state management
│   └── shortcuts/        # Shortcut system
│       ├── manager.rs    # Shortcut lifecycle management
│       └── storage.rs    # JSON persistence
```

### Tech Stack

- **TUI Framework**: [ratatui](https://github.com/ratatui-org/ratatui) - Modern terminal UI library
- **Input Handling**: [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- **Serialization**: [serde](https://serde.rs/) + [serde_json](https://github.com/serde-rs/json) - Shortcuts persistence
- **Error Handling**: [thiserror](https://github.com/dtolnay/thiserror) - Ergonomic error types

## 🗂️ Configuration

Shortcuts are stored in:
```
~/.config/jerm/shortcuts.json
```

The file is automatically created on first use and follows this structure:

```json
{
  "shortcuts": [
    {
      "path": "/Users/username/projects",
      "last_accessed": "2024-01-28T10:30:00Z",
      "created_at": "2024-01-10T08:00:00Z"
    }
  ]
}
```

## 🛠️ Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/jerm.git
cd jerm

# Build
cargo build

# Run
cargo run

# Run tests
cargo test

# Run with release optimizations
cargo build --release
```

### Code Style

This project follows Rust standard conventions:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -W clippy::pedantic

# Run all checks
cargo fmt && cargo clippy -- -W clippy::pedantic && cargo test
```

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Scopes**: `ui`, `nav`, `shortcuts`, `shell`, `git`, `core`

**Examples**:
```bash
feat(nav): add fuzzy search in cd -list mode
feat(git): add git status integration to prompt
fix(shortcuts): prevent duplicate entries
docs: update installation instructions
```

## 📝 License

This project is licensed under the MIT License  

## 📬 Contact

- **Issues**: [GitHub Issues](https://github.com/joaquin-osorio/jerm-tui/issues)
- **Discussions**: [GitHub Discussions](https://github.com/joaquin-osorio/jerm-tui/discussions)

---

Made with ❤️ and Rust
