use std::path::PathBuf;

use clap::Parser;

/// Declavatar CLI
#[derive(Debug, Clone, Parser)]
#[clap(version, author)]
pub struct Arguments {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Shows result struct in indented form.
    #[clap(short, long)]
    pub indented: bool,
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
}
