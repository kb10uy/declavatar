mod cli;

use crate::cli::Arguments;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use declavatar::{avatar::transform_avatar, decl::parse_document};
use miette::{IntoDiagnostic, Result as MietteResult};

fn main() -> MietteResult<()> {
    let args = Arguments::parse();

    let mut file = BufReader::new(File::open(args.avatar).into_diagnostic()?);
    let mut source = String::new();
    file.read_to_string(&mut source).into_diagnostic()?;

    let document = parse_document(&source)?;
    let transformed = transform_avatar(document.avatar);
    let avatar = match transformed.avatar {
        Some(avatar) => avatar,
        None => {
            for (level, message) in transformed.logs {
                println!("{level:?}: {message}");
            }
            return Ok(());
        }
    };

    let json = serde_json::to_string(&avatar).into_diagnostic()?;
    println!("{json}");

    Ok(())
}
