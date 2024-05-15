use clap::Parser;

/// Create a table with the long-multiplication
/// method given two values: the multiplicand
/// and the multiplier.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The first coefficient of the multiplication.
    #[arg(long)]
    pub multiplicand: String,

    /// The second coefficient of the multiplication.
    #[arg(long)]
    pub multiplier: String,
}

pub fn get_args() -> Args {
    return Args::parse()
}
