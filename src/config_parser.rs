use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub const MAX_LINE_LENGTH: usize = 256;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConfigEntry {
    pub line: String,
    pub tokens: Vec<String>,
    pub comment: Option<String>,
}

impl ConfigEntry {
    pub fn new() -> Self {
        Self::default()
    }
}

fn legal_escape(c: char) -> bool {
    c == '"' || c == ' ' || c == '\\'
}

fn is_comment(c: char) -> bool {
    c == ';' || c == '#'
}

fn tokenize(ce: &mut ConfigEntry) -> Result<(), String> {
    let mut chars = ce.line.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c == ' ' || c == '\t' {
            chars.next();
            continue;
        }

        if is_comment(c) {
            ce.comment = Some(chars.collect());
            break;
        }

        let mut token = String::new();
        let c = chars.next().unwrap();
        if c == '\'' {
            while let Some(nc) = chars.next() {
                if nc == '\'' {
                    break;
                }
                token.push(nc);
            }
        } else if c == '\"' {
            while let Some(nc) = chars.next() {
                if nc == '\"' {
                    break;
                }
                if nc == '\\' {
                    if let Some(&next_c) = chars.peek() {
                        if legal_escape(next_c) {
                            token.push(chars.next().unwrap());
                            continue;
                        } else {
                            return Err("Parse error: illegal backslash".to_string());
                        }
                    }
                }
                token.push(nc);
            }
        } else {
            let mut cur_c = c;
            loop {
                if cur_c == '\\' {
                    if let Some(&next_c) = chars.peek() {
                        if legal_escape(next_c) {
                            token.push(chars.next().unwrap());
                        } else {
                            return Err("Parse error: illegal backslash".to_string());
                        }
                    } else {
                        token.push(cur_c);
                    }
                } else {
                    token.push(cur_c);
                }

                if let Some(&next_c) = chars.peek() {
                    if next_c == ' ' || next_c == '\t' {
                        break;
                    }
                    cur_c = chars.next().unwrap();
                } else {
                    break;
                }
            }
        }
        ce.tokens.push(token);
    }

    if !ce.tokens.is_empty() {
        let first = ce.tokens[0].trim_start_matches('-');
        ce.tokens[0] = first.to_string();
    }

    Ok(())
}

pub fn config_readline<R: BufRead>(reader: &mut R, first: bool) -> Result<Option<ConfigEntry>, String> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => Ok(None),
        Ok(_) => {
            let mut ce = ConfigEntry::new();
            let line_str = line.trim_end_matches(&['\r', '\n'][..]);

            let mut line_to_parse = line_str;
            if first && line_str.starts_with('\u{FEFF}') {
                line_to_parse = &line_str[1..];
            }

            ce.line = line_to_parse.to_string();

            tokenize(&mut ce)?;
            Ok(Some(ce))
        }
        Err(e) => Err(format!("IO error: {}", e)),
    }
}

pub fn config_parse<P: AsRef<Path>>(fname: P) -> Result<Vec<ConfigEntry>, String> {
    let file = File::open(fname).map_err(|e| format!("Error opening file: {}", e))?;
    let mut reader = io::BufReader::new(file);
    let mut entries = Vec::new();

    let mut first = true;
    while let Some(entry) = config_readline(&mut reader, first)? {
        entries.push(entry);
        first = false;
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let mut ce = ConfigEntry::new();
        ce.line = "remote 1.2.3.4 1194 udp".to_string();
        tokenize(&mut ce).unwrap();
        assert_eq!(ce.tokens, vec!["remote", "1.2.3.4", "1194", "udp"]);
    }

    #[test]
    fn test_tokenize_quotes() {
        let mut ce = ConfigEntry::new();
        ce.line = "ca \"C:\\\\Program Files\\\\OpenVPN\\\\config\\\\ca.crt\"".to_string();
        tokenize(&mut ce).unwrap();
        assert_eq!(ce.tokens, vec!["ca", "C:\\Program Files\\OpenVPN\\config\\ca.crt"]);
    }

    #[test]
    fn test_tokenize_comment() {
        let mut ce = ConfigEntry::new();
        ce.line = "verb 3 # this is a comment".to_string();
        tokenize(&mut ce).unwrap();
        assert_eq!(ce.tokens, vec!["verb", "3"]);
        assert_eq!(ce.comment, Some("# this is a comment".to_string()));
    }
}
