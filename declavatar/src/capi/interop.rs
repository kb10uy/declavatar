use crate::{
    avatar_v2::{data::avatar::Avatar, Transformer},
    decl_v2::{compile_declaration, Arguments, DeclarationFormat},
    log::Log,
};

use std::path::{Path, PathBuf};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Success = 0,
    Utf8Error = 1,
    CompileError = 2,
    AlreadyInUse = 3,
    NotCompiled = 4,
    InvalidPointer = 128,
}

#[derive(Debug)]
pub struct Declavatar {
    in_use: bool,
    args: Arguments,
    avatar_json: Option<String>,
    logs_jsons: Vec<String>,
}

impl Declavatar {
    pub fn new() -> Declavatar {
        Declavatar {
            in_use: false,
            args: Arguments::default(),
            avatar_json: None,
            logs_jsons: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.in_use = false;
        self.args.clear();
        self.avatar_json = None;
        self.logs_jsons.clear();
    }

    pub fn args_mut(&mut self) -> &mut Arguments {
        &mut self.args
    }

    pub fn compile(&mut self, source: &str, kind: u32) -> Result<(), StatusCode> {
        if self.in_use {
            return Err(StatusCode::AlreadyInUse);
        } else {
            self.in_use = true;
        }

        let format = match kind {
            1 => DeclarationFormat::Sexpr(self.library_paths.clone()),
            2 => DeclarationFormat::Lua(self.library_paths.clone()),
            _ => return Err(StatusCode::CompileError),
        };

        let decl_avatar = match compile_declaration(source, format, self.preprocess.clone()) {
            Ok(decl_avatar) => decl_avatar,
            Err(report) => {
                self.logs_jsons.push(
                    serde_json::to_string(&report.serialize_log([]))
                        .expect("should serialize into JSON"),
                );
                return Err(StatusCode::CompileError);
            }
        };

        let transformed = transform_avatar(decl_avatar);
        let avatar = match transformed.avatar {
            Some(avatar) => avatar,
            None => {
                self.logs_jsons.extend(
                    transformed
                        .logs
                        .iter()
                        .map(|f| serde_json::to_string(f).expect("should serialize into JSON")),
                );
                return Err(StatusCode::CompileError);
            }
        };

        self.avatar_json =
            Some(serde_json::to_string(&avatar).map_err(|_| StatusCode::CompileError)?);
        Ok(())
    }

    pub fn avatar_json(&self) -> Result<&str, StatusCode> {
        let Some(json) = self.avatar_json.as_deref() else {
            return Err(StatusCode::NotCompiled);
        };

        Ok(json)
    }

    pub fn log_jsons(&self) -> &[String] {
        &self.logs_jsons
    }
}
