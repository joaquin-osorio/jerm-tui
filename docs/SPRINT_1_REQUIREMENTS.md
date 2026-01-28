# Sprint 1 Requirements

## Goal

Build a functional first version of Jerm focused on validating the core user experience and key terminal interactions.

**Target platform:** macOS only

**Success criteria:** A usable, stable, and coherent application that validates the central concept and serves as a solid foundation for future iterations.

---

## Core Features

### 1. Basic Terminal

The terminal must:

- Run correctly on macOS
- Interpret shell commands
- Reflect filesystem state
- Display command output
- Maintain session command history

---

### 2. Assisted Directory Navigation (`cd -list`)

#### Activation

- Activated via the `cd -list` command
- Enters a visual navigation mode

#### Visual Behavior

- Displays a navigable list of subdirectories in the current directory
- Clearly indicates current selection (highlight)
- Updates view when navigating virtually

#### Keyboard Controls

| Key      | Action                                                      |
| -------- | ----------------------------------------------------------- |
| `↑`      | Move selection up                                           |
| `↓`      | Move selection down                                         |
| `→`      | Enter selected subdirectory (virtual)                       |
| `←`      | Go up one directory level                                   |
| `Enter`  | Confirm and execute cd to selected directory                |
| `Escape` | Cancel and return to normal mode without changing directory |

#### Non-Functional Requirements

- Clear and fluid behavior
- No unexpected side effects
- Virtual navigation does NOT change the actual directory until confirmed with Enter

---

### 3. Sidebar Shortcuts

#### Display

- Permanently visible sidebar
- Shows user-saved shortcuts
- Ordered by **last access time** (most recent first)

#### Saving Shortcuts

- Keyboard shortcut to save current directory: `Cmd+I`

#### Navigation

- Selecting a shortcut navigates directly to that directory
- `Ctrl+1` through `Ctrl+9` for quick access to first 9 shortcuts

#### Data Model

```rust
pub struct Shortcut {
    pub path: PathBuf,
    pub last_accessed: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

#### Persistence

- Store shortcuts in JSON file
- Location: `~/.config/jerm/shortcuts.json`
- Update `last_accessed` every time a shortcut is used

#### Future Preparation

- Record `last_accessed` for all shortcuts
- This will enable automatic archiving by inactivity in future sprints
- Automatic archiving must NOT be active in this sprint

---

## Out of Scope

The following are explicitly **NOT included** in Sprint 1:

- Advanced visual customization
- User configuration
- Cross-platform compatibility (Linux, Windows)
- Complex persistence beyond shortcuts
- Editing existing shortcuts
- Manual deletion of shortcuts
- Sidebar visual customization
- Automatic archiving of inactive shortcuts
- Experimental features not explicitly described

---

## Acceptance Criteria

### Basic Terminal

- [ ] Executes shell commands and displays output
- [ ] `pwd` shows current directory correctly
- [ ] `ls` lists directory contents
- [ ] `cd <path>` changes directory
- [ ] Command history works within session

### cd -list Navigation

- [ ] `cd -list` activates navigation mode
- [ ] `↑/↓` arrows move selection
- [ ] `→` arrow enters subdirectory (virtual)
- [ ] `←` arrow goes up one level
- [ ] `Enter` confirms and executes cd
- [ ] `Escape` cancels without changes
- [ ] No unexpected side effects

### Shortcuts

- [ ] Sidebar displays shortcuts
- [ ] `Cmd+I` saves current directory
- [ ] Shortcuts ordered by last access
- [ ] Selecting shortcut navigates to directory
- [ ] JSON persistence works correctly
- [ ] `last_accessed` updates on shortcut use

---

## Suggested Implementation Order

1. Project setup: Cargo.toml, directory structure, dependencies
2. Basic terminal: TUI rendering, input loop, command execution
3. Mode system: Implement AppMode enum
4. cd -list: Visual directory navigation
5. Shortcuts storage: Data model and JSON persistence
6. Sidebar UI: Shortcut rendering
7. Integration: Connect shortcuts with navigation
8. Polish: Error handling, edge cases
