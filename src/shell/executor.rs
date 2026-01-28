use std::path::Path;
use std::process::Command;

use thiserror::Error;

/// Errors that can occur during command execution
#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Failed to execute command: {0}")]
    ExecutionFailed(#[from] std::io::Error),

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

/// Result of command execution
#[derive(Debug)]
pub struct CommandResult {
    /// Standard output lines
    pub stdout: Vec<String>,
    /// Standard error lines
    pub stderr: Vec<String>,
    /// Exit code (0 = success)
    #[allow(dead_code)]
    pub exit_code: i32,
}

impl CommandResult {
    /// Returns true if the command succeeded
    #[allow(dead_code)]
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }

    /// Get all output lines (stdout followed by stderr)
    pub fn all_lines(&self) -> Vec<String> {
        let mut lines = self.stdout.clone();
        lines.extend(self.stderr.clone());
        lines
    }
}

/// Execute a shell command in the given directory
pub fn execute_command(command: &str, current_dir: &Path) -> Result<CommandResult, ExecutorError> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(current_dir)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect();

    let stderr = String::from_utf8_lossy(&output.stderr)
        .lines()
        .map(String::from)
        .collect();

    let exit_code = output.status.code().unwrap_or(-1);

    Ok(CommandResult {
        stdout,
        stderr,
        exit_code,
    })
}

/// Resolve a path for cd command
/// Handles ~, relative paths, and absolute paths
pub fn resolve_cd_path(
    path: &str,
    current_dir: &Path,
) -> Result<std::path::PathBuf, ExecutorError> {
    let expanded = if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            if path == "~" {
                home
            } else {
                home.join(&path[2..]) // Skip "~/"
            }
        } else {
            return Err(ExecutorError::InvalidPath("Cannot expand ~".to_string()));
        }
    } else if path.starts_with('/') {
        std::path::PathBuf::from(path)
    } else if path == "-" {
        // TODO: Handle cd - (previous directory) - would need to track previous dir
        return Err(ExecutorError::InvalidPath(
            "cd - not yet implemented".to_string(),
        ));
    } else {
        current_dir.join(path)
    };

    // Canonicalize to resolve .. and . and symlinks
    let canonical = expanded
        .canonicalize()
        .map_err(|_| ExecutorError::DirectoryNotFound(path.to_string()))?;

    if !canonical.is_dir() {
        return Err(ExecutorError::NotADirectory(path.to_string()));
    }

    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_execute_echo() {
        let result = execute_command("echo hello", Path::new("/tmp")).unwrap();
        assert!(result.success());
        assert_eq!(result.stdout, vec!["hello"]);
    }

    #[test]
    fn test_execute_pwd() {
        let result = execute_command("pwd", Path::new("/tmp")).unwrap();
        assert!(result.success());
        // On macOS, /tmp is a symlink to /private/tmp
        assert!(result.stdout[0].contains("tmp"));
    }

    #[test]
    fn test_execute_failing_command() {
        let result = execute_command("exit 1", Path::new("/tmp")).unwrap();
        assert!(!result.success());
        assert_eq!(result.exit_code, 1);
    }

    #[test]
    fn test_resolve_absolute_path() {
        let result = resolve_cd_path("/tmp", Path::new("/"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_resolve_home_path() {
        let result = resolve_cd_path("~", Path::new("/tmp"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), dirs::home_dir().unwrap());
    }

    #[test]
    fn test_resolve_nonexistent_path() {
        let result = resolve_cd_path("/nonexistent/path/12345", Path::new("/"));
        assert!(result.is_err());
    }
}
