use std::path::Path;
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub branch: String,
    pub is_detached: bool,
    pub is_dirty: bool,
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    CommandFailed(String),
    #[error("Not a git repository")]
    NotARepository,
    #[error("Operation timed out")]
    Timeout,
}

#[derive(Debug, Clone)]
pub enum GitMessage {
    UpdateStatus { dir: String, with_fetch: bool },
    StatusUpdate(Option<GitStatus>),
    Shutdown,
}

pub fn is_git_repo(dir: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(dir)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn get_branch_name(dir: &Path) -> Result<String, GitError> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(GitError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_short_hash(dir: &Path) -> Result<String, GitError> {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(dir)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(GitError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn check_dirty_state(dir: &Path) -> Result<bool, GitError> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(dir)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(GitError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(!output.stdout.is_empty())
}

pub fn get_ahead_behind(dir: &Path) -> Result<(u32, u32), GitError> {
    let output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
        .current_dir(dir)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        // No upstream branch configured - not an error, just return 0,0
        return Ok((0, 0));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.trim().split_whitespace().collect();

    if parts.len() != 2 {
        return Ok((0, 0));
    }

    let ahead = parts[0].parse::<u32>().unwrap_or(0);
    let behind = parts[1].parse::<u32>().unwrap_or(0);

    Ok((ahead, behind))
}

pub fn get_git_status(dir: &Path) -> Result<GitStatus, GitError> {
    if !is_git_repo(dir) {
        return Err(GitError::NotARepository);
    }

    let branch = get_branch_name(dir)?;
    let (is_detached, branch_display) = if branch == "HEAD" {
        // Detached HEAD state
        let hash = get_short_hash(dir)?;
        (true, hash)
    } else {
        (false, branch)
    };

    let is_dirty = check_dirty_state(dir)?;
    let (ahead, behind) = get_ahead_behind(dir)?;

    Ok(GitStatus {
        branch: branch_display,
        is_detached,
        is_dirty,
        ahead,
        behind,
    })
}

pub fn spawn_git_worker() -> (Sender<GitMessage>, Receiver<GitMessage>) {
    let (main_tx, worker_rx) = mpsc::channel::<GitMessage>();
    let (worker_tx, main_rx) = mpsc::channel::<GitMessage>();

    thread::spawn(move || {
        loop {
            match worker_rx.recv() {
                Ok(GitMessage::UpdateStatus { dir, with_fetch }) => {
                    // Optionally run git fetch
                    if with_fetch {
                        let _ = Command::new("git")
                            .args(["fetch"])
                            .current_dir(&dir)
                            .output();
                    }

                    // Query git status
                    let status = get_git_status(Path::new(&dir)).ok();
                    let _ = worker_tx.send(GitMessage::StatusUpdate(status));
                }
                Ok(GitMessage::Shutdown) => {
                    break;
                }
                Ok(GitMessage::StatusUpdate(_)) => {
                    // Worker shouldn't receive this message, ignore
                }
                Err(_) => {
                    // Channel closed, exit
                    break;
                }
            }
        }
    });

    (main_tx, main_rx)
}
