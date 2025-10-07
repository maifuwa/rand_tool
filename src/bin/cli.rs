use clap::{Parser, Subcommand};
use rand_tool::{base64_decode, base64_encode, generate_port, generate_pwd, generate_uuid};

const DEFAULT_PORT_STAR: u16 = 1024;
const DEFAULT_PORT_END: u16 = 49151;
const DEFAULT_PORT_RANGE: &str = "1024-49151";

#[derive(Parser, Debug)]
#[command(name = "rand_tool", version, about, subcommand_required = true)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Default quantity of items to generate (default: 5)
    #[arg(short, long, default_value_t = 5, hide_default_value = true)]
    count: u8,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate random passwords with configurable character sets
    /// (default: 18 chars with numbers, uppercase, lowercase, no symbols)
    Pwd {
        /// Password length
        #[arg(long, short = 'c', default_value_t = 18, hide_default_value = true)]
        length: u8,

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
    },

    /// Generate UUIDs
    UUID,

    /// Base64 encoding and decoding operations
    BASE64 {
        /// Decode Base64 string to UTF-8
        #[arg(long, short)]
        decode: Option<String>,

        /// Encode UTF-8 string to Base64
        #[arg(long, short)]
        encode: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Pwd {
            length,
            numbers,
            uppercase,
            lowercase,
            symbols,
        } => {
            let pwd_and_score =
                generate_pwd(length, numbers, uppercase, lowercase, symbols, cli.count);
            for (pwd, score) in pwd_and_score {
                println!("password: {}  score: {:.3}", pwd, score);
            }
        }
        Command::Port { range } => {
            let (start, end) = parse_range(&range);
            println!("generated port range: {}-{}", start, end);
            for port in generate_port(start, end, cli.count) {
                println!("{}", port);
            }
        }
        Command::UUID => {
            for uuid in generate_uuid(cli.count) {
                println!("{}", uuid);
            }
        }
        Command::BASE64 { decode, encode } => {
            if let Some(decode) = decode {
                match base64_decode(decode) {
                    Ok(decoded) => println!("{}", decoded),
                    Err(err) => println!("{}", err),
                }
            } else {
                match encode {
                    None => {
                        println!("Please provide some input.");
                    }
                    Some(encode) => {
                        println!("{}", base64_encode(encode));
                    }
                }
            }
        }
    }
}

fn parse_range(range: &str) -> (u16, u16) {
    if range == DEFAULT_PORT_RANGE {
        return (DEFAULT_PORT_STAR, DEFAULT_PORT_END);
    }

    range
        .split_once('-')
        .map(|(start, end)| {
            let start = start.parse::<u16>().unwrap_or(DEFAULT_PORT_STAR);
            let end = end.parse::<u16>().unwrap_or(DEFAULT_PORT_END);

            let mid = start;

            (
                start.min(end).max(DEFAULT_PORT_STAR),
                end.max(mid).min(DEFAULT_PORT_END),
            )
        })
        .unwrap_or((DEFAULT_PORT_STAR, DEFAULT_PORT_END))
}
