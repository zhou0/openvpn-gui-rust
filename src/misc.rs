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
