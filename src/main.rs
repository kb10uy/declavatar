mod avatar;
mod decl;

use crate::{avatar::compiler::compile_avatar, decl::document::Document};

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

    let compiled_avatar = compile_avatar(document.avatar)?;
    let avatar_json = serde_json::to_string_pretty(&compiled_avatar).into_diagnostic()?;
    println!("{avatar_json}");
    Ok(())
}
