# Sprint 2 Requirements

## Goal

Add Git awareness to the terminal prompt, giving the user at-a-glance visibility into repository state without leaving the normal workflow.

**Target platform:** macOS only

**Success criteria:** The prompt line displays accurate, up-to-date Git information (branch, dirty state, ahead/behind remote) with minimal visual noise and no perceptible impact on input responsiveness.

---

## Core Features

### 1. Git-Aware Prompt

#### Prompt Format

When inside a Git repository:

```
~/dev/project main* ↑2↓3 $
```

| Element        | Description                                | Condition                  |
| -------------- | ------------------------------------------ | -------------------------- |
| `~/dev/project`| Current working directory (existing)       | Always shown               |
| `main`         | Current branch name                        | Inside a Git repo          |
| `*`            | Dirty indicator (uncommitted changes)      | Staged, unstaged, or untracked changes exist |
| `↑N`           | Commits ahead of remote                    | Only when N > 0            |
| `↓N`           | Commits behind remote                      | Only when N > 0            |
| `$`            | Input indicator (existing)                 | Always shown               |

When **not** inside a Git repository, the prompt remains unchanged:

```
~/dev/project $
```

#### Detached HEAD

When in detached HEAD state, display the short commit hash instead of a branch name:

```
~/dev/project abc1234* $
```

---

### 2. Visual Style

| Element          | Color            |
| ---------------- | ---------------- |
| Directory path   | Existing style   |
| Branch / hash    | Gray (subdued)   |
| `*` indicator    | Gray (same as branch) |
| `↑N` / `↓N`     | Cyan             |
| `$`              | Existing style   |

The Git information should feel **subtle** — present but not visually dominant.

---

### 3. Data Retrieval

#### Git State

The following data must be collected:

| Data point       | Source command                          |
| ---------------- | -------------------------------------- |
| Current branch   | `git rev-parse --abbrev-ref HEAD`      |
| Detached HEAD    | `git rev-parse --short HEAD` (when branch is `HEAD`) |
| Dirty state      | `git status --porcelain`               |
| Ahead/behind     | `git rev-list --left-right --count HEAD...@{upstream}` |

#### Update Strategy

Git state is refreshed on **two triggers**:

1. **After each command execution** — reads local state only (branch, dirty, ahead/behind from local refs). No network calls.
2. **Periodic polling every 30 seconds** — runs `git fetch` first to update remote refs, then reads state. This is the only trigger that performs network operations.

#### Error Handling

- If any Git command fails or times out, the prompt **renders without Git information** (falls back to the existing format).
- Git operations must run **asynchronously** and never block prompt rendering or user input.

---

## Architecture Notes

### Affected Modules

- **`src/app.rs`** — `App::prompt()` method (line ~83) currently formats the prompt as `{dir} $ `. This must be extended to include Git information.
- **`src/ui/terminal.rs`** — `render_terminal()` function (line ~67) renders the prompt. Must support colored segments for Git info.

### Suggested New Module

```
src/
└── git/
    ├── mod.rs        // Module exports
    └── status.rs     // Git state retrieval and caching
```

### Data Model

```rust
pub struct GitStatus {
    pub branch: String,         // Branch name or short hash
    pub is_detached: bool,      // True if HEAD is detached
    pub is_dirty: bool,         // Any uncommitted changes
    pub ahead: u32,             // Commits ahead of upstream
    pub behind: u32,            // Commits behind upstream
}
```

---

## Out of Scope

The following are explicitly **NOT included** in Sprint 2:

- New `jerm` commands (no `jerm status`, `jerm log`, etc.)
- Merge/rebase in-progress indicators
- Distinguishing between staged, unstaged, and untracked changes (only a single `*`)
- Git operations beyond read-only queries (no commits, pushes, etc.)
- Visual customization or user-configurable Git prompt format
- Stash indicator
- Submodule status
- Cross-platform compatibility (Linux, Windows)

---

## Acceptance Criteria

### Git Prompt Display

- [ ] Branch name is shown in the prompt when inside a Git repo
- [ ] Branch name is gray and visually subdued
- [ ] `*` appears immediately after branch name when there are uncommitted changes
- [ ] `*` covers staged, unstaged, and untracked files
- [ ] `↑N` is shown in cyan when local is ahead of remote (N > 0)
- [ ] `↓N` is shown in cyan when local is behind remote (N > 0)
- [ ] `↑N` and `↓N` are hidden when their values are 0
- [ ] Short commit hash is shown instead of branch name in detached HEAD state

### Non-Git Directories

- [ ] Prompt shows only path and `$` when not in a Git repo
- [ ] No errors or visual artifacts outside of Git repos

### Update Behavior

- [ ] Git state refreshes after each command execution (local only)
- [ ] Git state refreshes every 30 seconds via polling with `git fetch`
- [ ] `git fetch` only runs during periodic polling, not after each command
- [ ] Git operations never block user input or prompt rendering

### Error Resilience

- [ ] Prompt renders without Git info if Git commands fail
- [ ] Prompt renders without Git info if Git commands time out
- [ ] No crashes or hangs due to Git operations

---

## Suggested Implementation Order

1. Git module setup: `src/git/mod.rs`, `src/git/status.rs`, `GitStatus` data model
2. Git state retrieval: implement functions to query branch, dirty, ahead/behind
3. Prompt integration: extend `App::prompt()` to include Git info
4. Colored rendering: update `render_terminal()` to support colored prompt segments
5. Post-command refresh: trigger Git state update after each command execution
6. Async polling: implement 30-second background polling with `git fetch`
7. Error handling: graceful fallback when Git operations fail
8. Edge cases: detached HEAD, no upstream, non-Git directories
