use clap::Parser;
use rand_tool::args::Args;

#[test]
fn test_default_args() {
    let args = Args::parse_from(&["test"]);
    assert!(!args.port);
    assert!(!args.uuid);
    assert_eq!(args.count, 5);
    assert_eq!(args.range, "1024-49151");
    assert_eq!(args.length, 18);
    assert!(args.number);
    assert!(args.uppercase);
    assert!(args.lowercase);
    assert!(!args.symbols);
}

#[test]
fn test_port_arg() {
    let args = Args::parse_from(&["test", "-P"]);
    assert!(args.port);
}

#[test]
fn test_uuid_arg() {
    let args = Args::parse_from(&["test", "-U"]);
    assert!(args.uuid);
}

#[test]
fn test_count_arg() {
    let args = Args::parse_from(&["test", "-c", "10"]);
    assert_eq!(args.count, 10);
}

#[test]
fn test_range_arg() {
    let args = Args::parse_from(&["test", "-r", "2000-3000"]);
    assert_eq!(args.range, "2000-3000");
}

#[test]
fn test_length_arg() {
    let args = Args::parse_from(&["test", "-l", "12"]);
    assert_eq!(args.length, 12);
}

#[test]
fn test_number_arg() {
    let args = Args::parse_from(&["test", "-n"]);
    assert!(!args.number);
}

#[test]
fn test_uppercase_arg() {
    let args = Args::parse_from(&["test", "-u"]);
    assert!(!args.uppercase);
}

#[test]
fn test_lowercase_arg() {
    let args = Args::parse_from(&["test", "-o"]);
    assert!(!args.lowercase);
}

#[test]
fn test_symbols_arg() {
    let args = Args::parse_from(&["test", "-s"]);
    assert!(args.symbols);
}
 