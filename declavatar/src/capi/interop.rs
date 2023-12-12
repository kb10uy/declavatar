use miette::SourceSpan;

use crate::{
    avatar::{data::Avatar, transform_avatar, LogLevel},
    decl::parse_document,
};

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

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    CompilerError = 0,
    SyntaxError = 1,
    SemanticError = 2,
    SemanticInfo = 3,
    SemanticWarning = 4,
}

pub struct Declavatar {
    in_use: bool,
    compiled_avatar: Option<Avatar>,
    compiled_avatar_json: Option<String>,
    errors: Vec<(ErrorKind, String)>,
}

impl Declavatar {
    pub fn new() -> Declavatar {
        Declavatar {
            in_use: false,
            compiled_avatar: None,
            compiled_avatar_json: None,
            errors: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.in_use = false;
        self.compiled_avatar = None;
        self.compiled_avatar_json = None;
        self.errors.clear();
    }

    pub fn push_example_errors(&mut self) {
        self.in_use = true;
        self.errors
            .push((ErrorKind::CompilerError, "compiler error".to_string()));
        self.errors
            .push((ErrorKind::SyntaxError, "syntax error".to_string()));
        self.errors
            .push((ErrorKind::SemanticError, "semantic error".to_string()));
        self.errors
            .push((ErrorKind::SemanticInfo, "semantic info".to_string()));
    }

    pub fn errors(&self) -> &[(ErrorKind, String)] {
        &self.errors
    }

    pub fn compile(&mut self, source: &str) -> Result<(), StatusCode> {
        if self.in_use {
            return Err(StatusCode::AlreadyInUse);
        } else {
            self.in_use = true;
        }

        let avatar_decl = match parse_document(source) {
            Ok(avatar_decl) => avatar_decl,
            Err(report) => {
                self.errors
                    .push((ErrorKind::SyntaxError, report.to_string()));
                return Err(StatusCode::CompileError);
            }
        };

        let transformed = transform_avatar(avatar_decl.avatar);
        let avatar = match transformed.avatar {
            Some(avatar) => avatar,
            None => {
                for (level, message) in transformed.logs {
                    let error_kind = match level {
                        LogLevel::Information => ErrorKind::SemanticInfo,
                        LogLevel::Warning => ErrorKind::SemanticWarning,
                        LogLevel::Error => ErrorKind::SemanticError,
                    };
                    self.errors.push((error_kind, message));
                }
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
}
