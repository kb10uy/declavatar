pub mod data;
pub mod error;
mod sexpr;

use std::path::PathBuf;

use crate::decl_v2::{data::avatar::DeclAvatar, error::DeclError, sexpr::load_avatar_sexpr};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeclarationFormat {
    Sexpr(Vec<PathBuf>),
    Lua(Vec<PathBuf>),
}

pub fn load_declaration(text: &str, format: DeclarationFormat) -> Result<DeclAvatar, DeclError> {
    match format {
        DeclarationFormat::Sexpr(paths) => load_avatar_sexpr(text, paths),
        _ => Err(DeclError::UnsupportedFormat),
    }
}
