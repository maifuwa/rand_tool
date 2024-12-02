use args::Args;
use passwords::{analyzer, scorer, PasswordGenerator};
use rand::Rng;

pub mod args;

/// Generate random port numbers
pub fn generate_port(args: Args) {
    let mut rng = rand::thread_rng();
    let (range_start, range_end) = parse_range(&args.range);
    println!("generated port range: {}-{}", range_start, range_end);
    for _ in 0..args.count {
        let port: usize = rng.gen_range(range_start..=range_end);
        println!("{}", port);
    }
}

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
