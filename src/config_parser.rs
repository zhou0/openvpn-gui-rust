use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigParserError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: illegal backslash")]
    IllegalBackslash,
}

#[derive(Debug, Default, Clone)]
pub struct ConfigEntry {
    pub line: String,
    pub tokens: Vec<String>,
    pub comment: Option<String>,
}

pub fn legal_escape(c: char) -> bool {
    matches!(c, '"' | ' ' | '\\')
}

pub fn is_comment_char(c: char) -> bool {
    c == ';' || c == '#'
}

fn copy_token(chars: &mut std::iter::Peekable<std::str::Chars>, delim: &str) -> Result<String, ConfigParserError> {
    let mut token = String::new();
    while let Some(&c) = chars.peek() {
        if delim.contains(c) {
            break;
        }
        chars.next();
        if c == '\\' {
            if let Some(&next_c) = chars.peek() {
                if legal_escape(next_c) {
                    token.push(chars.next().unwrap());
                } else {
                    return Err(ConfigParserError::IllegalBackslash);
                }
            } else {
                return Err(ConfigParserError::IllegalBackslash);
            }
        } else {
            token.push(c);
        }
    }
    Ok(token)
}

pub fn tokenize(line: &str) -> Result<ConfigEntry, ConfigParserError> {
    let mut entry = ConfigEntry {
        line: line.to_string(),
        ..Default::default()
    };

    let mut chars = line.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if is_comment_char(c) {
            let comment: String = chars.collect();
            entry.comment = Some(comment);
            break;
        }

        if c == '\'' {
            chars.next();
            let mut token = String::new();
            while let Some(next_c) = chars.next() {
                if next_c == '\'' {
                    break;
                }
                token.push(next_c);
            }
            entry.tokens.push(token);
        } else if c == '"' {
            chars.next();
            let token = copy_token(&mut chars, "\"")?;
            chars.next(); // consume the closing quote
            entry.tokens.push(token);
        } else {
            let token = copy_token(&mut chars, " \t")?;
            entry.tokens.push(token);
        }
    }

    if !entry.tokens.is_empty() {
        if entry.tokens[0].starts_with("--") {
            entry.tokens[0] = entry.tokens[0][2..].to_string();
        }
    }

    Ok(entry)
}

pub fn config_parse<P: AsRef<Path>>(fname: P) -> Result<Vec<ConfigEntry>, ConfigParserError> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    let mut first = true;

    for line in reader.lines() {
        let mut line = line?;
        if first {
            if line.starts_with("\u{feff}") {
                line.remove(0);
            }
            first = false;
        }
        let entry = tokenize(&line)?;
        entries.push(entry);
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let entry = tokenize("remote 1.2.3.4 1194 udp").unwrap();
        assert_eq!(entry.tokens, vec!["remote", "1.2.3.4", "1194", "udp"]);
    }

    #[test]
    fn test_tokenize_quotes() {
        let entry = tokenize("secret \"my secret key\"").unwrap();
        assert_eq!(entry.tokens, vec!["secret", "my secret key"]);
    }

    #[test]
    fn test_tokenize_escapes() {
        let entry = tokenize("path C:\\\\Program\\ Files\\\\OpenVPN").unwrap();
        assert_eq!(entry.tokens, vec!["path", "C:\\Program Files\\OpenVPN"]);
    }

    #[test]
    fn test_tokenize_comments() {
        let entry = tokenize("dev tun ; this is a comment").unwrap();
        assert_eq!(entry.tokens, vec!["dev", "tun"]);
        assert_eq!(entry.comment, Some("; this is a comment".to_string()));
    }

    #[test]
    fn test_tokenize_leading_dashes() {
        let entry = tokenize("--config client.ovpn").unwrap();
        assert_eq!(entry.tokens, vec!["config", "client.ovpn"]);
    }

    #[test]
    fn test_tokenize_non_ascii() {
        let entry = tokenize("name \"München\" # comment with emoji 🚀").unwrap();
        assert_eq!(entry.tokens, vec!["name", "München"]);
        assert_eq!(entry.comment, Some("# comment with emoji 🚀".to_string()));
    }
}
