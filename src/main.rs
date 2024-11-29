use clap::Parser;
use rand_tool::{args::Args, generate_password, generate_port, generate_uuid};

fn main() {
    let args = Args::parse();

    if args.port {
        generate_port(args);
    } else if args.uuid {
        generate_uuid(args);
    } else {
        generate_password(args);
    }
}
