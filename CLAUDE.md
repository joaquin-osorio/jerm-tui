# Jerm

A modern terminal emulator for macOS with assisted navigation and directory shortcuts.

## Tech Stack

- **Language**: Rust (2021 edition, minimum 1.75)
- **TUI Framework**: `ratatui`
- **Input Handling**: `crossterm`
- **Serialization**: `serde` + `serde_json`
- **Filesystem**: `std::fs` + `dirs`
- **Shell Integration**: `std::process::Command`

## Project Structure

```
jerm/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── terminal.rs
│   │   ├── navigator.rs
│   │   └── sidebar.rs
│   ├── shell/
│   │   ├── mod.rs
│   │   ├── executor.rs
│   │   └── parser.rs
│   ├── navigation/
│   │   ├── mod.rs
│   │   └── directory.rs
│   └── shortcuts/
│       ├── mod.rs
│       ├── manager.rs
│       └── storage.rs
└── docs/
    └── SPRINT_1_REQUIREMENTS.md
```

## Code Style

- Format with `rustfmt`
- Lint with `cargo clippy -- -W clippy::pedantic`
- Document public functions with `///`
- Use `Result<T, E>` for fallible operations
- Use `thiserror` for custom error types

## Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Scopes:** `ui`, `nav`, `shortcuts`, `shell`, `core`

**Examples:**

```
feat(nav): implement arrow key navigation in cd -list
fix(shortcuts): correct ordering by last access time
docs: add installation instructions to README
chore: update ratatui to 0.26
```

**Rules:**

- One logical change per commit
- Use imperative mood, lowercase, no period
- Max 72 characters for the first line

## Development Commands

```bash
cargo build          # Build
cargo run            # Run
cargo test           # Test
cargo clippy         # Lint
cargo fmt            # Format
cargo build --release # Release build
```

## Application Modes

```rust
pub enum AppMode {
    Normal,
    NavigationList,
}
```

## Shortcuts Storage

Location: `~/.config/jerm/shortcuts.json`

```json
{
  "shortcuts": [
    {
      "path": "/Users/user/projects",
      "last_accessed": "2024-01-15T10:30:00Z",
      "created_at": "2024-01-10T08:00:00Z"
    }
  ]
}
```

## Key Bindings (Sprint 1)

| Context  | Key         | Action                 |
| -------- | ----------- | ---------------------- |
| Global   | `Ctrl+1..9` | Navigate to shortcut N |
| cd -list | `↑/↓`       | Move selection         |
| cd -list | `→`         | Enter directory        |
| cd -list | `←`         | Go up one level        |
| cd -list | `Enter`     | Confirm navigation     |
| cd -list | `Escape`    | Cancel                 |

## Commands (Sprint 1)

| Command     | Action                             |
| ----------- | ---------------------------------- |
| `jerm save` | Save current directory as shortcut |
| `jerm goto` | Navigate to a saved shortcut       |

## Dependencies

```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
```
