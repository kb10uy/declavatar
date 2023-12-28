mod application;

use crate::application::{Arguments, FileOption, Subcommand};

use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use anyhow::{bail, Result};
use clap::Parser;
use declavatar::{
    avatar_v2::transform_avatar,
    decl_v2::{data::avatar::DeclAvatar, load_declaration, DeclarationFormat},
    i18n::get_log_messages,
};
use strfmt::Format;
use sys_locale::get_locale;

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
                let i18n_log = I18nLog::load_current_locale();
                for log in avatar_result.logs {
                    let message = i18n_log.localize(log.kind, log.args);
                    println!("{:?}: {message}", log.severity);
                    for ctx in log.context {
                        println!("@ {ctx}");
                    }
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

struct I18nLog {
    localization: HashMap<String, String>,
}

impl I18nLog {
    fn load_current_locale() -> I18nLog {
        let locale = get_locale().unwrap_or("en_US".to_string());
        let i18n_json = get_log_messages(&locale)
            .or(get_log_messages("en_US"))
            .expect("en_US should exist");
        let localization = serde_json::from_str(i18n_json).expect("should deserialize");

        I18nLog { localization }
    }

    fn localize(&self, kind: String, args: Vec<String>) -> String {
        let Some(base) = self.localization.get(&kind) else {
            return kind;
        };
        base.format(
            &args
                .into_iter()
                .enumerate()
                .map(|(i, a)| (i.to_string(), a))
                .collect(),
        )
        .expect("failed to localize")
    }
}
