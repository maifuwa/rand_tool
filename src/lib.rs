#![doc = include_str!("../README.md")]

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use passwords::analyzer::analyze;
use passwords::scorer::score;
use passwords::PasswordGenerator;
use rand::seq::SliceRandom;
use rand::RngExt;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

pub const DEFAULT_PORT_START: u16 = 1024;
pub const DEFAULT_PORT_END: u16 = 49151;
pub const DEFAULT_PORT_RANGE: &str = "1024-49151";

#[derive(Debug, Clone, PartialEq)]
pub struct PwdInfo {
    pub password: String,
    pub score: f64,
}

impl Display for PwdInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "password: {}  score: {:.3}", self.password, self.score)
    }
}

impl PwdInfo {
    fn new(password: String, score: f64) -> Self {
        Self { password, score }
    }
}

#[derive(Debug, Error)]
pub enum RandToolError {
    #[error("unable to generate passwords: {0}")]
    PasswordGeneration(&'static str),
    #[error("failed to decode Base64 input: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("decoded Base64 content is not valid UTF-8: {0}")]
    Base64Utf8(#[from] std::string::FromUtf8Error),
    #[error(
        "requested {requested} unique ports, but range {start}-{end} only contains {available}"
    )]
    NotEnoughUniquePorts {
        requested: usize,
        start: u16,
        end: u16,
        available: usize,
    },
}

pub fn generate_passwords(
    length: usize,
    numbers: bool,
    uppercase: bool,
    lowercase: bool,
    symbols: bool,
    count: usize,
) -> Result<Vec<PwdInfo>, RandToolError> {
    let pg = PasswordGenerator::new()
        .length(length)
        .numbers(numbers)
        .lowercase_letters(lowercase)
        .uppercase_letters(uppercase)
        .symbols(symbols)
        .spaces(false)
        .exclude_similar_characters(true)
        .strict(true);

    let passwords = pg
        .generate(count)
        .map_err(RandToolError::PasswordGeneration)?;

    Ok(passwords
        .into_iter()
        .map(|password| {
            let analysis = analyze(&password);
            PwdInfo::new(password, score(&analysis))
        })
        .collect())
}

pub fn generate_ports(
    start: u16,
    end: u16,
    count: usize,
    unique: bool,
) -> Result<Vec<u16>, RandToolError> {
    if unique {
        let available = usize::from(end - start) + 1;
        if count > available {
            return Err(RandToolError::NotEnoughUniquePorts {
                requested: count,
                start,
                end,
                available,
            });
        }

        let mut ports: Vec<u16> = (start..=end).collect();
        ports.shuffle(&mut rand::rng());
        ports.truncate(count);
        return Ok(ports);
    }

    let mut rng = rand::rng();
    Ok((0..count).map(|_| rng.random_range(start..=end)).collect())
}

pub fn generate_uuids(count: usize) -> Vec<String> {
    (0..count).map(|_| Uuid::new_v4().to_string()).collect()
}

pub fn base64_decode(content: &str) -> Result<String, RandToolError> {
    let decoded = BASE64_STANDARD.decode(content.as_bytes())?;
    Ok(String::from_utf8(decoded)?)
}

pub fn base64_encode(content: &str) -> String {
    BASE64_STANDARD.encode(content.as_bytes())
}

pub fn parse_range(range: &str) -> (u16, u16) {
    if range == DEFAULT_PORT_RANGE {
        return (DEFAULT_PORT_START, DEFAULT_PORT_END);
    }

    range
        .split_once('-')
        .map(|(start, end)| {
            let start = start
                .parse::<u16>()
                .unwrap_or(DEFAULT_PORT_START)
                .clamp(DEFAULT_PORT_START, DEFAULT_PORT_END);
            let end = end
                .parse::<u16>()
                .unwrap_or(DEFAULT_PORT_END)
                .clamp(DEFAULT_PORT_START, DEFAULT_PORT_END);

            (start.min(end), start.max(end))
        })
        .unwrap_or((DEFAULT_PORT_START, DEFAULT_PORT_END))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Version;

    mod generate_passwords {
        use super::*;

        #[test]
        fn returns_requested_number_of_passwords_when_options_are_valid() {
            let passwords = generate_passwords(18, true, true, true, false, 3)
                .expect("password generation should succeed");

            assert_eq!(passwords.len(), 3);
        }

        #[test]
        fn returns_passwords_with_requested_length_when_options_are_valid() {
            let passwords = generate_passwords(24, true, true, true, false, 1)
                .expect("password generation should succeed");

            assert_eq!(passwords[0].password.len(), 24);
        }

        #[test]
        fn excludes_digits_when_numbers_are_disabled() {
            let passwords = generate_passwords(20, false, true, true, false, 3)
                .expect("password generation should succeed");

            assert!(passwords
                .iter()
                .all(|info| info.password.chars().all(|ch| !ch.is_ascii_digit())));
        }

        #[test]
        fn returns_error_when_all_character_sets_are_disabled() {
            let error = generate_passwords(18, false, false, false, false, 1)
                .expect_err("password generation should fail");

            assert!(matches!(
                error,
                RandToolError::PasswordGeneration(
                    "You need to enable at least one kind of characters."
                )
            ));
        }
    }

    mod generate_ports {
        use super::*;

        #[test]
        fn returns_requested_number_of_ports_within_inclusive_range() {
            let ports =
                generate_ports(3000, 3010, 20, false).expect("port generation should succeed");

            assert_eq!(ports.len(), 20);
            assert!(ports.iter().all(|port| (3000..=3010).contains(port)));
        }

        #[test]
        fn supports_single_value_ranges_without_panicking() {
            let ports =
                generate_ports(8080, 8080, 3, false).expect("port generation should succeed");

            assert_eq!(ports, vec![8080, 8080, 8080]);
        }

        #[test]
        fn returns_unique_ports_when_requested() {
            let ports = generate_ports(9000, 9010, 11, true)
                .expect("unique port generation should succeed");

            let unique: std::collections::HashSet<_> = ports.iter().copied().collect();

            assert_eq!(ports.len(), 11);
            assert_eq!(unique.len(), 11);
        }

        #[test]
        fn returns_error_when_unique_port_request_exceeds_range_capacity() {
            let error = generate_ports(7000, 7001, 3, true)
                .expect_err("unique port generation should fail");

            assert!(matches!(
                error,
                RandToolError::NotEnoughUniquePorts {
                    requested: 3,
                    start: 7000,
                    end: 7001,
                    available: 2,
                }
            ));
        }
    }

    mod generate_uuids {
        use super::*;

        #[test]
        fn returns_requested_number_of_v4_uuids() {
            let uuids = generate_uuids(4);

            assert_eq!(uuids.len(), 4);
            assert!(uuids.iter().all(|value| {
                let uuid = Uuid::parse_str(value).expect("generated UUID should parse");
                uuid.get_version() == Some(Version::Random)
            }));
        }
    }

    mod base64_decode {
        use super::*;

        #[test]
        fn returns_original_utf8_when_input_is_valid() {
            let decoded =
                base64_decode("SGVsbG8sIFdvcmxkIQ==").expect("valid base64 should decode");

            assert_eq!(decoded, "Hello, World!");
        }

        #[test]
        fn returns_error_when_input_is_invalid() {
            let decoded = base64_decode("%%%");

            assert!(decoded.is_err());
        }
    }

    mod base64_encode {
        use super::*;

        #[test]
        fn returns_expected_base64_for_utf8_input() {
            let encoded = base64_encode("Hello, World!");

            assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
        }
    }

    mod parse_range {
        use super::*;

        #[test]
        fn returns_default_range_when_input_matches_default_range() {
            let range = parse_range(DEFAULT_PORT_RANGE);

            assert_eq!(range, (DEFAULT_PORT_START, DEFAULT_PORT_END));
        }

        #[test]
        fn returns_default_range_when_input_format_is_invalid() {
            let range = parse_range("invalid");

            assert_eq!(range, (DEFAULT_PORT_START, DEFAULT_PORT_END));
        }

        #[test]
        fn sorts_and_clamps_bounds_when_input_is_out_of_order_or_out_of_range() {
            let range = parse_range("1-0");

            assert_eq!(range, (DEFAULT_PORT_START, DEFAULT_PORT_START));
        }

        #[test]
        fn sorts_bounds_when_input_is_reversed() {
            let range = parse_range("9000-8000");

            assert_eq!(range, (8000, 9000));
        }
    }
}
