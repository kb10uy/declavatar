use std::path::PathBuf;

use clap::Parser;

/// Declavatar CLI
#[derive(Debug, Clone, Parser)]
#[clap(version, author)]
pub struct Arguments {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Clone, Parser)]
pub enum Subcommand {
    /// Loads declaration file and expands into internal format.
    Load(FileOption),

    /// Loads declaration file and compiles with validation.
    Compile(FileOption),
}

#[derive(Debug, Clone, Parser)]
pub struct FileOption {
    /// Filename.
    pub file: PathBuf,

    /// Shows the result struct in indented form.
    #[clap(short, long)]
    pub indented: bool,

    /// Adds a library directory.
    #[clap(short = 'L', long = "library")]
    pub library_paths: Vec<PathBuf>,

    /// Defines a symbol.
    #[clap(short = 's', long = "symbol")]
    pub symbols: Vec<String>,

    /// Defines a localization pair.
    #[clap(short = 'l', long = "localize", value_parser = parse_localization_pair)]
    pub localizations: Vec<(String, String)>,
}

fn parse_localization_pair(s: &str) -> Result<(String, String), String> {
    if let Some((key, value)) = s.split_once(':') {
        Ok((key.to_string(), value.to_string()))
    } else {
        Err(format!("invalid localization pair definition: {s}"))
    }
}
