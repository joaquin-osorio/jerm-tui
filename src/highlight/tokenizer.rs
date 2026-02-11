//! Command line tokenizer for syntax highlighting

use ratatui::style::Style;
use ratatui::text::Span;

use crate::theme::Palette;

/// Type of token for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Command name (first word or after pipe/&&/||)
    Command,
    /// Flag like --help or -v
    Flag,
    /// File path (contains / or starts with ./ or ~)
    Path,
    /// Quoted string
    String,
    /// Numeric value
    Number,
    /// Operators: |, >, >>, <, &&, ||, ;
    Operator,
    /// Whitespace
    Whitespace,
    /// Plain text (arguments)
    Text,
}

/// A token with its text and type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

impl Token {
    /// Create a new token
    pub fn new(text: impl Into<String>, token_type: TokenType) -> Self {
        Self {
            text: text.into(),
            token_type,
        }
    }
}

/// Tokenizer for command line input
pub struct Tokenizer;

impl Tokenizer {
    /// Tokenize a command line string
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        let mut expect_command = true;

        while chars.peek().is_some() {
            // Handle whitespace
            if chars.peek().is_some_and(|c| c.is_whitespace()) {
                let mut ws = String::new();
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    ws.push(chars.next().unwrap());
                }
                tokens.push(Token::new(ws, TokenType::Whitespace));
                continue;
            }

            // Handle operators (|, >, <, &&, ||, ;)
            if let Some(&c) = chars.peek() {
                if let Some(op) = Self::try_parse_operator(&mut chars, c) {
                    // After |, &&, ||, ; we expect a command
                    // After >, >>, < we expect a file argument (not a command)
                    let is_redirect = op == ">" || op == ">>" || op == "<";
                    tokens.push(Token::new(op, TokenType::Operator));
                    expect_command = !is_redirect;
                    continue;
                }
            }

            // Handle quoted strings
            if let Some(&c) = chars.peek() {
                if c == '"' || c == '\'' {
                    let quote = chars.next().unwrap();
                    let mut s = String::from(quote);
                    while let Some(&ch) = chars.peek() {
                        s.push(chars.next().unwrap());
                        if ch == quote {
                            break;
                        }
                    }
                    tokens.push(Token::new(s, TokenType::String));
                    expect_command = false;
                    continue;
                }
            }

            // Parse word
            let word = Self::parse_word(&mut chars);
            if word.is_empty() {
                continue;
            }

            let token_type = Self::classify_word(&word, expect_command);
            tokens.push(Token::new(word, token_type));
            expect_command = false;
        }

        tokens
    }

    /// Try to parse an operator
    fn try_parse_operator(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        c: char,
    ) -> Option<String> {
        match c {
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    Some("||".to_string())
                } else {
                    Some("|".to_string())
                }
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    Some("&&".to_string())
                } else {
                    Some("&".to_string())
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    Some(">>".to_string())
                } else {
                    Some(">".to_string())
                }
            }
            '<' => {
                chars.next();
                Some("<".to_string())
            }
            ';' => {
                chars.next();
                Some(";".to_string())
            }
            _ => None,
        }
    }

    /// Parse a word (non-whitespace, non-operator sequence)
    fn parse_word(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace()
                || c == '|'
                || c == '&'
                || c == '>'
                || c == '<'
                || c == ';'
                || c == '"'
                || c == '\''
            {
                break;
            }
            word.push(chars.next().unwrap());
        }
        word
    }

    /// Classify a word token
    fn classify_word(word: &str, expect_command: bool) -> TokenType {
        // Flags: start with - or --
        if word.starts_with("--") {
            TokenType::Flag
        } else if word.starts_with('-') && word.len() > 1 {
            // Check if it's a negative number
            let rest = &word[1..];
            if rest.chars().all(|c| c.is_ascii_digit() || c == '.') {
                TokenType::Number
            } else {
                TokenType::Flag
            }
        }
        // Paths: contain /, start with ./ or ~/, or end with /
        else if word.contains('/') || word.starts_with("./") || word.starts_with("~/") {
            TokenType::Path
        }
        // Numbers
        else if word.chars().all(|c| c.is_ascii_digit() || c == '.') && !word.is_empty() {
            TokenType::Number
        }
        // Command (first word or after operator)
        else if expect_command {
            TokenType::Command
        }
        // Plain text/arguments
        else {
            TokenType::Text
        }
    }

    /// Convert tokens to styled spans for rendering
    pub fn to_spans(tokens: &[Token]) -> Vec<Span<'static>> {
        tokens
            .iter()
            .map(|token| {
                let style = match token.token_type {
                    TokenType::Command => Style::default().fg(Palette::SYNTAX_COMMAND),
                    TokenType::Flag => Style::default().fg(Palette::SYNTAX_FLAG),
                    TokenType::Path => Style::default().fg(Palette::SYNTAX_PATH),
                    TokenType::String => Style::default().fg(Palette::SYNTAX_STRING),
                    TokenType::Number => Style::default().fg(Palette::SYNTAX_NUMBER),
                    TokenType::Operator => Style::default().fg(Palette::SYNTAX_OPERATOR),
                    TokenType::Whitespace | TokenType::Text => {
                        Style::default().fg(Palette::SYNTAX_TEXT)
                    }
                };
                Span::styled(token.text.clone(), style)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_command() {
        let tokens = Tokenizer::tokenize("git status");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[0].text, "git");
        assert_eq!(tokens[1].token_type, TokenType::Whitespace);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[2].text, "status");
    }

    #[test]
    fn test_tokenize_with_flags() {
        let tokens = Tokenizer::tokenize("ls -la --color");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[2].token_type, TokenType::Flag);
        assert_eq!(tokens[2].text, "-la");
        assert_eq!(tokens[4].token_type, TokenType::Flag);
        assert_eq!(tokens[4].text, "--color");
    }

    #[test]
    fn test_tokenize_with_path() {
        let tokens = Tokenizer::tokenize("cd ~/projects");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[2].token_type, TokenType::Path);
        assert_eq!(tokens[2].text, "~/projects");
    }

    #[test]
    fn test_tokenize_with_string() {
        let tokens = Tokenizer::tokenize("echo \"hello world\"");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[2].token_type, TokenType::String);
        assert_eq!(tokens[2].text, "\"hello world\"");
    }

    #[test]
    fn test_tokenize_with_pipe() {
        let tokens = Tokenizer::tokenize("ls | grep foo");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[2].token_type, TokenType::Operator);
        assert_eq!(tokens[2].text, "|");
        assert_eq!(tokens[4].token_type, TokenType::Command);
        assert_eq!(tokens[4].text, "grep");
    }

    #[test]
    fn test_tokenize_with_and() {
        let tokens = Tokenizer::tokenize("make && make install");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[2].token_type, TokenType::Operator);
        assert_eq!(tokens[2].text, "&&");
        assert_eq!(tokens[4].token_type, TokenType::Command);
    }

    #[test]
    fn test_tokenize_with_number() {
        let tokens = Tokenizer::tokenize("sleep 5");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[2].text, "5");
    }

    #[test]
    fn test_tokenize_complex() {
        let tokens = Tokenizer::tokenize("git commit -m \"test message\" --amend");
        assert_eq!(tokens[0].token_type, TokenType::Command);
        assert_eq!(tokens[0].text, "git");
        // Find the -m flag
        let m_flag = tokens.iter().find(|t| t.text == "-m").unwrap();
        assert_eq!(m_flag.token_type, TokenType::Flag);
        // Find the string
        let string = tokens.iter().find(|t| t.text.contains("test")).unwrap();
        assert_eq!(string.token_type, TokenType::String);
        // Find --amend
        let amend = tokens.iter().find(|t| t.text == "--amend").unwrap();
        assert_eq!(amend.token_type, TokenType::Flag);
    }

    #[test]
    fn test_tokenize_empty() {
        let tokens = Tokenizer::tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_redirect() {
        let tokens = Tokenizer::tokenize("echo test > file.txt");
        let redirect = tokens.iter().find(|t| t.text == ">").unwrap();
        assert_eq!(redirect.token_type, TokenType::Operator);
        let path = tokens.iter().find(|t| t.text == "file.txt").unwrap();
        assert_eq!(path.token_type, TokenType::Text);
    }
}
