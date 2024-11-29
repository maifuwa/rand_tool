use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Set the number of passwords or ports to generate.
    #[arg(short, long, default_value = "5")]
    pub count: u8,

    /// Generates random ports instead of passwords.
    #[arg(short = 'P', long, default_value_t = false)]
    pub port: bool,

    /// Set the range of ports to generate.
    #[arg(short, long, default_value = "1024-49151")]
    pub range: String,

    #[arg(short = 'U', long, default_value_t = false)]
    pub uuid: bool,

    /// Do not include numbers in the password.
    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    /// Do not include uppercase characters in the password.
    #[arg(short, long, default_value_t = true)]
    pub uppercase: bool,

    /// Do not include lowercase characters in the password.
    #[arg(short = 'o', long, default_value_t = true)]
    pub lowercase: bool,

    /// Include special characters in the password.
    #[arg(short, long, default_value_t = false)]
    pub symbols: bool,

    /// Include spaces in the password.
    #[arg(short = 'p', long, default_value_t = false)]
    pub spaces: bool,

    /// Set the password length.
    #[arg(short, long, default_value = "18")]
    pub length: u8,
}
