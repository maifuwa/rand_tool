use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use passwords::analyzer::analyze;
use passwords::scorer::score;
use passwords::PasswordGenerator;
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

pub fn generate_pwd(
    length: u8,
    numbers: bool,
    uppercase: bool,
    lowercase: bool,
    symbols: bool,
    count: u8,
) -> HashMap<String, String> {
    let pg = PasswordGenerator::new()
        .length(length as usize)
        .numbers(numbers)
        .lowercase_letters(uppercase)
        .uppercase_letters(lowercase)
        .symbols(symbols)
        .spaces(false)
        .exclude_similar_characters(true)
        .strict(true);

    let pwds = pg
        .generate(count as usize)
        .expect("Unable to generate passwords");

    let scores: Vec<_> = pwds
        .iter()
        .map(|pw| score(&analyze(pw)).to_string())
        .collect();

    pwds.into_iter().zip(scores).collect()
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
