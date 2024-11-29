use args::Args;
use passwords::{analyzer, scorer, PasswordGenerator};
use rand::Rng;

pub mod args;

/// Generate random port numbers
pub fn generate_port(args: Args) {
    let mut rng = rand::thread_rng();
    let (range_start, range_end) = parse_range(&args.range);
    for _ in 0..args.count {
        let port: usize = rng.gen_range(range_start..=range_end);
        println!("generated port: {}", port);
    }
}

fn parse_range(range: &str) -> (usize, usize) {
    let parts: Vec<&str> = range.split('-').collect();
    let range_start = parts[0].parse::<usize>().unwrap();
    let range_end = parts[1].parse::<usize>().unwrap();
    (range_start, range_end)
}

/// Generate random passwords
pub fn generate_password(args: Args) {
    let pg = PasswordGenerator::new()
        .length(args.length as usize)
        .numbers(args.number)
        .lowercase_letters(args.lowercase)
        .uppercase_letters(args.uppercase)
        .symbols(args.symbols)
        .spaces(args.spaces)
        .exclude_similar_characters(true)
        .strict(true);

    let pwds = pg.generate(args.count as usize).unwrap();

    for pwd in pwds {
        let score = scorer::score(&analyzer::analyze(&pwd));
        println!("generated password: {}; score: {}", pwd, score);
    }
}