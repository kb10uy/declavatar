mod application;

use crate::application::{Arguments, FileOption, Subcommand};

use std::{fs::read_to_string, path::PathBuf};

use anyhow::{bail, Result};
use clap::Parser;
use declavatar::{
    avatar_v2::transform_avatar,
    decl_v2::{data::avatar::DeclAvatar, load_declaration, DeclarationFormat},
};

fn main() -> Result<()> {
    let args = Arguments::parse();

    match args.subcommand {
        Subcommand::Load(FileOption {
            file,
            indented,
            library_paths,
        }) => match load_declaration_auto(file, library_paths) {
            Ok(a) => {
                if indented {
                    println!("{a:#?}");
                } else {
                    println!("{a:?}");
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        },
        Subcommand::Compile(FileOption {
            file,
            indented,
            library_paths,
        }) => {
            let decl_avatar = load_declaration_auto(file, library_paths)?;
            let avatar_result = transform_avatar(decl_avatar);
            if let Some(avatar) = avatar_result.avatar {
                let json = if indented {
                    serde_json::to_string_pretty(&avatar)
                } else {
                    serde_json::to_string(&avatar)
                }?;
                println!("{json}");
            } else {
                for log in avatar_result.logs {
                    println!("{:?}: {}", log.severity, log.kind);
                    println!();
                }
            }
        }
    }
    Ok(())
}

fn load_declaration_auto(file: PathBuf, paths: Vec<PathBuf>) -> Result<DeclAvatar> {
    let file_ext = file.extension();
    let Some(file_ext) = file_ext else {
        bail!("file format cannot be determined");
    };
    let format = match file_ext.to_str().expect("cannot convert") {
        "declisp" | "lisp" | "scm" => DeclarationFormat::Sexpr(paths),
        "declua" | "lua" => DeclarationFormat::Lua(paths),
        ext => bail!("unknown file type: {ext}"),
    };

    let text = read_to_string(file)?;
    let decl_avatar = load_declaration(&text, format)?;
    Ok(decl_avatar)
}
