use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    // City name
    #[arg(short = 'C', long, help = "A city definition")]
    pub city: Option<String>,

    // Country
    #[arg(short = 'O', long, help = "Country of the city")]
    pub country: Option<String>,

    // The path to the file to read
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Path to the configuration file"
    )]
    pub config: Option<PathBuf>,

    // verbose flag
    #[arg(short, long, help = "Verbose mode")]
    pub verbose: bool,
}
