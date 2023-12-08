mod application;

use std::fs::read_to_string;

use crate::application::{Arguments, FileOption, Subcommand};

use anyhow::{bail, Result};
use clap::Parser;
use declavatar::decl_v2::{load_declaration, DeclarationFormat};

fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.subcommand {
        Subcommand::Load(FileOption { file }) => {
            let file_ext = file.extension();
            let Some(file_ext) = file_ext else {
                bail!("file format cannot be determined");
            };
            let format = match file_ext.to_str().expect("cannot convert") {
                "lisp" | "scm" => DeclarationFormat::Sexpr,
                "lua" => DeclarationFormat::Lua,
                ext => bail!("unknown file type: {ext}"),
            };

            let text = read_to_string(file)?;
            match load_declaration(&text, format) {
                Ok(a) => {
                    println!("{a:?}");
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                }
            }
        }
        Subcommand::Compile(_) => unimplemented!("compile mode is uninmplemented"),
    }
    Ok(())
}
