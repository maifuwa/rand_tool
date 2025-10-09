pub mod pwd_info;

use crate::pwd_info::PwdInfo;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use passwords::analyzer::analyze;
use passwords::scorer::score;
use passwords::PasswordGenerator;
use rand::Rng;
use std::error::Error;
use uuid::Uuid;

pub fn generate_pwd(
    length: u8,
    numbers: bool,
    uppercase: bool,
    lowercase: bool,
    symbols: bool,
    count: u8,
) -> Vec<PwdInfo> {
    let pg = PasswordGenerator::new()
        .length(length as usize)
        .numbers(numbers)
        .lowercase_letters(lowercase)
        .uppercase_letters(uppercase)
        .symbols(symbols)
        .spaces(false)
        .exclude_similar_characters(true)
        .strict(true);

    let pwds = pg
        .generate(count as usize)
        .expect("Unable to generate passwords");

    pwds.into_iter()
        .map(|p| {
            let analysis = analyze(&p);
            PwdInfo::new(p, score(&analysis))
        })
        .collect()
}

pub fn generate_port(start: u16, end: u16, count: u8) -> Vec<u16> {
    let mut rng = rand::rng();

    (0..count).map(|_| rng.random_range(start..end)).collect()
}

pub fn generate_uuid(count: u8) -> Vec<String> {
    (0..count).map(|_| Uuid::new_v4().to_string()).collect()
}

pub fn base64_decode(content: String) -> Result<String, Box<dyn Error>> {
    let decode = BASE64_STANDARD.decode(content.as_bytes())?;
    Ok(String::from_utf8(decode)?)
}

pub fn base64_encode(content: String) -> String {
    BASE64_STANDARD.encode(content.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pwd_basic() {
        let result = generate_pwd(12, true, true, true, false, 3);
        assert_eq!(result.len(), 3);
        for pwd_info in result.iter() {
            assert_eq!(pwd_info.password.len(), 12);
        }
    }

    #[test]
    fn test_generate_pwd_with_symbols() {
        let result = generate_pwd(16, true, true, true, true, 1);
        assert_eq!(result.len(), 1);
        for pwd_info in result.iter() {
            assert_eq!(pwd_info.password.len(), 16);
        }
    }

    #[test]
    fn test_generate_pwd_only_numbers() {
        let result = generate_pwd(10, true, false, false, false, 2);
        assert_eq!(result.len(), 2);
        for pwd_info in result.iter() {
            assert_eq!(pwd_info.password.len(), 10);
            assert!(pwd_info.password.chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[test]
    fn test_generate_port_range() {
        let ports = generate_port(8000, 9000, 10);
        assert_eq!(ports.len(), 10);
        for port in ports {
            assert!((8000..9000).contains(&port));
        }
    }

    #[test]
    fn test_generate_port_single() {
        let ports = generate_port(3000, 4000, 1);
        assert_eq!(ports.len(), 1);
        assert!(ports[0] >= 3000 && ports[0] < 4000);
    }

    #[test]
    fn test_generate_uuid_basic() {
        let uuids = generate_uuid(3);
        assert_eq!(uuids.len(), 3);
        for uuid in uuids {
            assert_eq!(uuid.len(), 36);
            assert!(uuid.contains('-'));
        }
    }

    #[test]
    fn test_generate_uuid_uniqueness() {
        let uuids = generate_uuid(100);
        let unique_count = uuids.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, 100);
    }

    #[test]
    fn test_base64_encode_basic() {
        let result = base64_encode("Hello, World!".to_string());
        assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_base64_encode_empty() {
        let result = base64_encode("".to_string());
        assert_eq!(result, "");
    }

    #[test]
    fn test_base64_encode_special_chars() {
        let result = base64_encode("Test@123!#$".to_string());
        assert!(!result.is_empty());
    }

    #[test]
    fn test_base64_decode_basic() {
        let result = base64_decode("SGVsbG8sIFdvcmxkIQ==".to_string()).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_base64_decode_empty() {
        let result = base64_decode("".to_string()).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_base64_decode_invalid() {
        let result = base64_decode("Invalid@Base64!".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_roundtrip() {
        let original = "The quick brown fox jumps over the lazy dog";
        let encoded = base64_encode(original.to_string());
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base64_roundtrip_unicode() {
        let original = "Hello 世界 🌍";
        let encoded = base64_encode(original.to_string());
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
