use std::fs;

use args::Args;
use base64::{prelude::BASE64_STANDARD, Engine};
use passwords::{analyzer, scorer, PasswordGenerator};
use rand::Rng;

pub mod args;

/// Generate random port numbers
pub fn generate_port(args: Args) {
    let range = args.additional.unwrap_or("1024-49151".to_string());
    let (range_start, range_end) = parse_range(&range);
    println!("generated port range: {}-{}", range_start, range_end);

    let mut rng = rand::thread_rng();
    for _ in 0..args.count {
        let port: usize = rng.gen_range(range_start..=range_end);
        println!("{}", port);
    }
}

/// Parse a range string into a tuple of start and end values
fn parse_range(range: &str) -> (usize, usize) {
    let parts: Vec<&str> = range.split('-').collect();
    let range_start = parts[0].parse::<usize>().unwrap();
    let range_end = parts[1].parse::<usize>().unwrap();
    (range_start, range_end)
}

/// Generate random UUIDs
pub fn generate_uuid(args: Args) {
    println!("generated v4 uuids:");
    for _ in 0..args.count {
        let uuid = uuid::Uuid::new_v4();
        println!("{}", uuid);
    }
}

/// Generate random passwords
pub fn generate_password(args: Args) {
    let pg = PasswordGenerator::new()
        .length(args.length as usize)
        .numbers(args.number)
        .lowercase_letters(args.lowercase)
        .uppercase_letters(args.uppercase)
        .symbols(args.symbols)
        .spaces(false)
        .exclude_similar_characters(true)
        .strict(true);

    let pwds = pg.generate(args.count as usize).unwrap();

    println!("generated passwords:");

    for pwd in pwds {
        let score = scorer::score(&analyzer::analyze(&pwd));
        println!("password: {}  score: {:.3}", pwd, score);
    }
}

/// Decode a base64 string
pub fn base64_decode(args: Args) {
    let base64: String = if let Some(additional) = args.additional {
        match fs::metadata(&additional) {
            Ok(_) => fs::read_to_string(&additional).unwrap(),
            Err(_) => additional,
        }
    } else {
        eprintln!("No base64 string provided");
        return;
    };

    let decoded = match BASE64_STANDARD.decode(base64.as_bytes()) {
        Ok(decoded) => decoded,
        Err(e) => {
            eprintln!("Error decoding base64: {}", e);
            return;
        }
    };

    let original_str = String::from_utf8_lossy(&decoded).to_string();
    println!("original_str: {}", original_str);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_range() {
        let range = "1000-2000";
        let (range_start, range_end) = parse_range(range);
        assert_eq!(range_start, 1000);
        assert_eq!(range_end, 2000);
    }
}
