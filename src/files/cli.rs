use clap::{crate_version, Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Format {
    Json,
    Compact,
}

#[derive(Parser, Debug)]
#[command(version = crate_version!(), about = "Weather CLI tool", long_about = None)]
pub struct Cli {
    #[arg(short, long_help = "City name e.g. London, Paris")]
    pub q: String,
    #[arg(value_enum, short, long, default_value_t = Format::Compact, long_help = "Output format")]
    pub format: Format,
    #[arg(long, long_help = "Include air quality index")]
    pub aqi: bool,
}
