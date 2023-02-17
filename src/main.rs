mod decl;

use crate::decl::document::Document;

use std::{
    env::args,
    fs::File,
    io::{BufReader, Read},
};

use kdl::KdlDocument;
use miette::{IntoDiagnostic, Result as MietteResult};
use thiserror::Error as ThisError;

#[derive(Debug, Clone, ThisError)]
enum MiscError {
    #[error("not enough arguments")]
    InsufficientArguments,
}

fn main() -> MietteResult<()> {
    let args: Vec<_> = args().collect();
    if args.len() <= 1 {
        return Err(MiscError::InsufficientArguments).into_diagnostic();
    }

    let mut file = BufReader::new(File::open(&args[1]).into_diagnostic()?);
    let mut source = String::new();
    file.read_to_string(&mut source).into_diagnostic()?;

    let kdl: KdlDocument = source.parse()?;
    let document = Document::parse(&kdl, &source)?;
    println!("{document:?}");
    Ok(())
}
