use crate::{
    avatar_v2::{data::avatar::Avatar, Transformer},
    decl_v2::{load_declaration, DeclarationFormat, PreprocessData},
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
    compiled_avatar: Option<Avatar>,
    compiled_avatar_json: Option<String>,
    json_errors: Vec<String>,
    library_paths: Vec<PathBuf>,
    preprocess: PreprocessData,
}

impl Declavatar {
    pub fn new() -> Declavatar {
        Declavatar {
            in_use: false,
            compiled_avatar: None,
            compiled_avatar_json: None,
            json_errors: vec![],
            library_paths: vec![],
            preprocess: PreprocessData::default(),
        }
    }

    pub fn reset(&mut self) {
        self.in_use = false;
        self.compiled_avatar = None;
        self.compiled_avatar_json = None;
        self.json_errors.clear();
        self.library_paths.clear();
        self.preprocess.symbols.clear();
        self.preprocess.localizations.clear();
    }

    pub fn add_library_path(&mut self, path: impl AsRef<Path>) {
        self.library_paths.push(path.as_ref().to_owned());
    }

    pub fn define_symbol(&mut self, symbol: impl Into<String>) {
        self.preprocess.symbols.insert(symbol.into());
    }

    pub fn define_localization(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.preprocess
            .localizations
            .insert(key.into(), value.into());
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

        let decl_avatar = match load_declaration(source, format, self.preprocess.clone()) {
            Ok(decl_avatar) => decl_avatar,
            Err(report) => {
                self.json_errors.push(
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
                self.json_errors.extend(
                    transformed
                        .logs
                        .iter()
                        .map(|f| serde_json::to_string(f).expect("should serialize into JSON")),
                );
                return Err(StatusCode::CompileError);
            }
        };
        let avatar_json = serde_json::to_string(&avatar).map_err(|_| StatusCode::CompileError)?;

        self.compiled_avatar = Some(avatar);
        self.compiled_avatar_json = Some(avatar_json);

        Ok(())
    }

    pub fn avatar_json(&self) -> Result<&str, StatusCode> {
        let Some(json) = self.compiled_avatar_json.as_deref() else {
            return Err(StatusCode::NotCompiled);
        };

        Ok(json)
    }

    pub fn log_jsons(&self) -> &[String] {
        &self.json_errors
    }
}
