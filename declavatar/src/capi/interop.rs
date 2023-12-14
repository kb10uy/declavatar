use crate::{
    avatar_v2::{data::avatar::Avatar, logger::Severity, transform_avatar},
    decl_v2::{load_declaration, DeclarationFormat},
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

    pub fn compile(&mut self, source: &str, kind: u32) -> Result<(), StatusCode> {
        if self.in_use {
            return Err(StatusCode::AlreadyInUse);
        } else {
            self.in_use = true;
        }

        let format = match kind {
            1 => DeclarationFormat::Sexpr(vec![]),
            2 => DeclarationFormat::Lua(vec![]),
            _ => return Err(StatusCode::CompileError),
        };

        let decl_avatar = match load_declaration(source, format) {
            Ok(decl_avatar) => decl_avatar,
            Err(report) => {
                self.errors
                    .push((ErrorKind::SyntaxError, report.to_string()));
                return Err(StatusCode::CompileError);
            }
        };

        let transformed = transform_avatar(decl_avatar);
        let avatar = match transformed.avatar {
            Some(avatar) => avatar,
            None => {
                for (level, message) in transformed.logs {
                    let error_kind = match level {
                        Severity::Information => ErrorKind::SemanticInfo,
                        Severity::Warning => ErrorKind::SemanticWarning,
                        Severity::Error => ErrorKind::SemanticError,
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
