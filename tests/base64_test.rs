use clap::Parser;
use rand_tool::{args::Args, base64_decode};

#[test]
fn test_base64_decode() {
    let args = Args::parse_from(&["test", "-B", "5L2g5aW9LCDkuJbnlYwh"]);
    base64_decode(args);
}