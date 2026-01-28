/// Represents a parsed command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedCommand {
    /// Empty command (just whitespace or nothing)
    Empty,
    /// Change directory command with optional path
    Cd(Option<String>),
    /// Special cd -list command to show directory navigator
    CdList,
    /// Clear the terminal
    Clear,
    /// Exit the terminal
    Exit,
    /// Regular shell command to execute
    Shell(String),
}

/// Parse a command string into a `ParsedCommand`
pub fn parse_command(input: &str) -> ParsedCommand {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return ParsedCommand::Empty;
    }

    let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
    let command = parts[0];
    let args = parts.get(1).map(|s| s.trim());

    match command {
        "cd" => match args {
            Some("-list" | "--list") => ParsedCommand::CdList,
            Some(path) => ParsedCommand::Cd(Some(path.to_string())),
            None => ParsedCommand::Cd(None),
        },
        "clear" => ParsedCommand::Clear,
        "exit" | "quit" => ParsedCommand::Exit,
        _ => ParsedCommand::Shell(trimmed.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse_command(""), ParsedCommand::Empty);
        assert_eq!(parse_command("   "), ParsedCommand::Empty);
    }

    #[test]
    fn test_parse_cd() {
        assert_eq!(parse_command("cd"), ParsedCommand::Cd(None));
        assert_eq!(
            parse_command("cd /tmp"),
            ParsedCommand::Cd(Some("/tmp".to_string()))
        );
        assert_eq!(
            parse_command("cd ~/projects"),
            ParsedCommand::Cd(Some("~/projects".to_string()))
        );
    }

    #[test]
    fn test_parse_cd_list() {
        assert_eq!(parse_command("cd -list"), ParsedCommand::CdList);
        assert_eq!(parse_command("cd --list"), ParsedCommand::CdList);
    }

    #[test]
    fn test_parse_clear() {
        assert_eq!(parse_command("clear"), ParsedCommand::Clear);
    }

    #[test]
    fn test_parse_exit() {
        assert_eq!(parse_command("exit"), ParsedCommand::Exit);
        assert_eq!(parse_command("quit"), ParsedCommand::Exit);
    }

    #[test]
    fn test_parse_shell() {
        assert_eq!(
            parse_command("ls -la"),
            ParsedCommand::Shell("ls -la".to_string())
        );
        assert_eq!(
            parse_command("echo hello world"),
            ParsedCommand::Shell("echo hello world".to_string())
        );
    }
}
