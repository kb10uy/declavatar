use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    pub avatar: PathBuf,
}
