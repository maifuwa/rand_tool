use clap::Parser;
use rand_tool::{args::Args, base64_decode, generate_password, generate_port, generate_uuid};

fn main() {
    let args = Args::parse();

    if args.port {
        generate_port(args);
    } else if args.uuid {
        generate_uuid(args);
    } else if args.base64 {
        base64_decode(args);
    } else {
        generate_password(args);
    }
}
