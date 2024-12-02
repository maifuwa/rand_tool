use clap::Parser;
use rand_tool::{args::Args, generate_password, generate_port, generate_uuid};

#[test]
fn generate_port_test() {
    let args = Args::parse_from(&["test", "-P"]);
    generate_port(args);
}

#[test]
fn generate_uuid_test() {
    let args = Args::parse_from(&["test", "-U"]);
    generate_uuid(args);
}

#[test]
fn generate_password_test() {
    let args = Args::parse_from(&["test"]);
    generate_password(args);
}
