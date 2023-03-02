mod cli;

use crate::cli::Arguments;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use declavatar::{avatar::compile_avatar, decl::parse_document};
use miette::{IntoDiagnostic, Result as MietteResult};

fn main() -> MietteResult<()> {
    let args = Arguments::parse();

    let mut file = BufReader::new(File::open(args.avatar).into_diagnostic()?);
    let mut source = String::new();
    file.read_to_string(&mut source).into_diagnostic()?;

    let document = parse_document(&source)?;
    let avatar = match compile_avatar(document.avatar)? {
        Ok(avatar) => avatar,
        Err(errors) => {
            for error in errors {
                println!("{error}");
            }
            return Ok(());
        }
    };

    let json = serde_json::to_string(&avatar).into_diagnostic()?;
    println!("{json}");

    Ok(())
}
