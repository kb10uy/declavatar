use crate::{
    avatar::{compile_avatar, data::Avatar},
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
}

pub struct Declavatar {
    in_use: bool,
    compiled_avatar: Option<Avatar>,
    errors: Vec<(ErrorKind, String)>,
}

impl Declavatar {
    pub fn new() -> Declavatar {
        Declavatar {
            in_use: false,
            compiled_avatar: None,
            errors: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.in_use = false;
        self.compiled_avatar = None;
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

    pub fn compile(&mut self, source: &str) -> StatusCode {
        if self.in_use {
            return StatusCode::AlreadyInUse;
        } else {
            self.in_use = true;
        }

        let avatar_decl = match parse_document(source) {
            Ok(avatar_decl) => avatar_decl,
            Err(report) => {
                self.errors
                    .push((ErrorKind::SyntaxError, report.to_string()));
                return StatusCode::CompileError;
            }
        };

        self.compiled_avatar = match compile_avatar(avatar_decl.avatar) {
            Ok(Ok(avatar)) => Some(avatar),
            Ok(Err(errors)) => {
                for error in errors {
                    self.errors
                        .push((ErrorKind::SemanticError, error.to_string()));
                }
                return StatusCode::CompileError;
            }
            Err(e) => {
                self.errors.push((ErrorKind::CompilerError, e.to_string()));
                return StatusCode::CompileError;
            }
        };

        StatusCode::Success
    }
}
