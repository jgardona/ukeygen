use clap::Parser;

use self::key::{get_key, KeySize};

mod key;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The key size
    #[arg(value_enum)]
    size: KeySize,
    /// Include symbols on key
    #[arg(short, long)]
    symbol: bool,
    /// Include uppercase alphabet characters
    #[arg(short, long)]
    uppercase: bool,
    /// Include lowercase alphabet characters
    #[arg(short, long)]
    lowercase: bool,
    /// Include numeric characters
    #[arg(short, long)]
    numeric: bool,
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    let key = get_key(
        cli.symbol,
        cli.uppercase,
        cli.lowercase,
        cli.numeric,
        cli.size,
    );

    print!("{key}");

    Ok(())
}
