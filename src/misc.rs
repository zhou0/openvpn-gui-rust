use crate::options::{Version, Connection};
use crate::config_parser::config_parse;
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};
use widestring::U16String;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn version_compare(a: &Version, b: &Version) -> Ordering {
    match a.major.cmp(&b.major) {
        Ordering::Equal => match a.minor.cmp(&b.minor) {
            Ordering::Equal => match a.release.cmp(&b.release) {
                Ordering::Equal => a.stage.cmp(&b.stage),
                other => other,
            },
            other => other,
        },
        other => other,
    }
}

pub fn wcs_concat2(src1: &str, src2: &str, sep: &str) -> String {
    if !src1.is_empty() && !src2.is_empty() {
        format!("{}{}{}", src1, sep, src2)
    } else if !src1.is_empty() {
        src1.to_string()
    } else if !src2.is_empty() {
        src2.to_string()
    } else {
        String::new()
    }
}

pub fn wchar_to_utf8(wstr: &[u16]) -> String {
    U16String::from_vec(wstr).to_string_lossy()
}

pub fn widen(s: &str) -> Vec<u16> {
    U16String::from_str(s).into_vec()
}

pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::create_dir_all(path).is_ok()
}

pub fn escape_string(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        if c == '"' || c == '\\' || c == ' ' {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

pub fn url_decode(src: &str) -> String {
    let mut out = String::new();
    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut hex = String::new();
            if let Some(h1) = chars.next() { hex.push(h1); }
            if let Some(h2) = chars.next() { hex.push(h2); }
            if let Ok(val) = u8::from_str_radix(&hex, 16) {
                out.push(val as char);
            } else {
                out.push('%');
                out.push_str(&hex);
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn parse_management_address(conn: &mut Connection) -> bool {
    let config_path = PathBuf::from(&conn.config_dir).join(&conn.config_file);
    let entries = match config_parse(config_path) {
        Ok(e) => e,
        Err(_) => return false,
    };

    let mut workdir = conn.config_dir.clone();
    let mut pw_file = None;
    let mut found = false;

    for entry in entries {
        if entry.tokens.len() >= 3 && entry.tokens[0] == "management" {
            if let Ok(addr) = Ipv4Addr::from_str(&entry.tokens[1]) {
                conn.manage.skaddr.sin_addr = addr.octets();
                if let Ok(port) = entry.tokens[2].parse::<u16>() {
                    conn.manage.skaddr.sin_port = port.to_be();
                    found = true;
                }
            }
            if entry.tokens.len() >= 4 {
                pw_file = Some(entry.tokens[3].clone());
            }
        } else if entry.tokens.len() >= 2 && entry.tokens[0] == "cd" {
            workdir = entry.tokens[1].clone();
        }
    }

    if found {
        if let Some(pwf) = pw_file {
            let pw_path = if Path::new(&pwf).is_relative() {
                PathBuf::from(workdir).join(pwf)
            } else {
                PathBuf::from(pwf)
            };

            if let Ok(pw) = fs::read_to_string(pw_path) {
                let pw = pw.trim_matches(|c| c == '\r' || c == '\n');
                let bytes = pw.as_bytes();
                let len = bytes.len().min(conn.manage.password.len());
                conn.manage.password[..len].copy_from_slice(&bytes[..len]);
            } else {
                return false;
            }
        }
    }

    found
use std::fs;
use std::path::Path;
use base64::{prelude::BASE64_STANDARD, Engine};
use crate::options::Version;

pub fn wchar_to_utf8(wstr: &[u16]) -> String {
    String::from_utf16_lossy(wstr)
}

pub fn widen(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn base64_encode(input: &[u8]) -> String {
    BASE64_STANDARD.encode(input)
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64_STANDARD.decode(input)
}

pub fn ensure_dir_exists<P: AsRef<Path>>(dir: P) -> std::io::Result<()> {
    fs::create_dir_all(dir)
}

pub fn version_compare(a: &Version, b: &Version) -> std::cmp::Ordering {
    a.major.cmp(&b.major)
        .then_with(|| a.minor.cmp(&b.minor))
        .then_with(|| a.release.cmp(&b.release))
        .then_with(|| a.stage.cmp(&b.stage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compare() {
        let v1 = Version { major: 2, minor: 5, release: 0, stage: 0 };
        let v2 = Version { major: 2, minor: 5, release: 0, stage: 0 };
        let v3 = Version { major: 2, minor: 6, release: 0, stage: 0 };
        let v4 = Version { major: 2, minor: 5, release: 1, stage: 0 };

        assert_eq!(version_compare(&v1, &v2), Ordering::Equal);
        assert_eq!(version_compare(&v1, &v3), Ordering::Less);
        assert_eq!(version_compare(&v3, &v1), Ordering::Greater);
        assert_eq!(version_compare(&v1, &v4), Ordering::Less);
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("foo bar"), "foo\\ bar");
        assert_eq!(escape_string("C:\\path"), "C:\\\\path");
        assert_eq!(escape_string("quote\""), "quote\\\"");
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("foo%20bar"), "foo bar");
        assert_eq!(url_decode("foo%2fbar"), "foo/bar");
    fn test_base64() {
        let input = b"hello world";
        let encoded = base64_encode(input);
        assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_version_compare() {
        let v1 = Version { major: 2, minor: 4, release: 0, stage: 0 };
        let v2 = Version { major: 2, minor: 5, release: 0, stage: 0 };
        assert_eq!(version_compare(&v1, &v2), std::cmp::Ordering::Less);
        assert_eq!(version_compare(&v2, &v1), std::cmp::Ordering::Greater);
        assert_eq!(version_compare(&v1, &v1), std::cmp::Ordering::Equal);
    }
}
