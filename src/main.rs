use clap::Parser;
use rand_tool::{args::Args, generate_password, generate_port};

fn main() {
    let args = Args::parse();

    if args.port {
        generate_port(args);
    } else {
        generate_password(args);
    }
}
