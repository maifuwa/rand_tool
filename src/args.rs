use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Generates random ports instead of passwords.
    #[arg(short = 'P', long, action = clap::ArgAction::SetTrue)]
    pub port: bool,

    /// Generates UUIDs instead of passwords.
    #[arg(short = 'U', long, action = clap::ArgAction::SetTrue)]
    pub uuid: bool,

    /// enable base64 decoding
    #[arg(short = 'B', long, action = clap::ArgAction::SetTrue)]
    pub base64: bool,

    /// Set the number of passwords or ports to generate.
    #[arg(short, long, default_value = "5")]
    pub count: u8,

    /// Set the password length.
    #[arg(short, long, default_value = "18")]
    pub length: u8,

    /// Do not include numbers in the password.
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    pub number: bool,

    /// Do not include uppercase characters in the password.
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    pub uppercase: bool,

    /// Do not include lowercase characters in the password.
    #[arg(short = 'o', long, action = clap::ArgAction::SetFalse)]
    pub lowercase: bool,

    /// Include special characters in the password.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub symbols: bool,

    /// Additional message:
    /// - Port generation: range of ports default 1024-49151
    /// - Base64 decode: base64 string
    #[arg()]
    pub additional: Option<String>,
}
