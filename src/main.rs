use clap::{ArgGroup, Parser, Subcommand};
use rand_tool::{
    base64_decode, base64_encode, generate_passwords, generate_ports, generate_uuids, parse_range,
    DEFAULT_PORT_RANGE,
};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "rand_tool", version, about, subcommand_required = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Default quantity of items to generate (default: 5)
    #[arg(short, long, default_value_t = 5, hide_default_value = true)]
    count: usize,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate random passwords with configurable character sets
    /// (default: 18 chars with numbers, uppercase, lowercase, no symbols)
    Pwd {
        /// Password length
        #[arg(long, short = 'c', default_value_t = 18, hide_default_value = true)]
        length: usize,

        /// Include digits 0-9 (default: true)
        #[arg(long, short, action = clap::ArgAction::SetFalse)]
        numbers: bool,

        /// Include uppercase letters A-Z (default: true)
        #[arg(long, short, action = clap::ArgAction::SetFalse)]
        uppercase: bool,

        /// Include uppercase letters a-z (default: true)
        #[arg(long, short, action = clap::ArgAction::SetFalse)]
        lowercase: bool,

        /// Include special characters (default: false)
        #[arg(long, short)]
        symbols: bool,
    },

    /// Generate random port numbers within specified range (default: 1024-49151)
    Port {
        /// Port range in format "min-max" (default: 1024-49151)
        #[arg(long, short, default_value = DEFAULT_PORT_RANGE, hide_default_value = true)]
        range: String,

        /// Generate unique ports without duplicates
        #[arg(long)]
        unique: bool,
    },

    /// Generate UUIDs
    Uuid,

    /// Base64 encoding and decoding operations
    #[command(group(
        ArgGroup::new("operation")
            .required(true)
            .multiple(false)
            .args(["decode", "encode"])
    ))]
    Base64 {
        /// Decode Base64 string to UTF-8
        #[arg(long, short, group = "operation")]
        decode: Option<String>,

        /// Encode UTF-8 string to Base64
        #[arg(long, short, group = "operation")]
        encode: Option<String>,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), rand_tool::RandToolError> {
    let cli = Cli::parse();

    match cli.command {
        Command::Pwd {
            length,
            numbers,
            uppercase,
            lowercase,
            symbols,
        } => {
            generate_passwords(length, numbers, uppercase, lowercase, symbols, cli.count)?
                .into_iter()
                .for_each(|p| println!("{p}"));
        }
        Command::Port { range, unique } => {
            let (start, end) = parse_range(&range);
            println!("generated port range: {start}-{end}");
            for port in generate_ports(start, end, cli.count, unique)? {
                println!("{port}");
            }
        }
        Command::Uuid => {
            for uuid in generate_uuids(cli.count) {
                println!("{uuid}");
            }
        }
        Command::Base64 { decode, encode } => {
            if let Some(decode) = decode {
                println!("{}", base64_decode(&decode)?);
            } else if let Some(encode) = encode {
                println!("{}", base64_encode(&encode));
            }
        }
    }
    Ok(())
}
