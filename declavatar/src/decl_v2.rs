pub mod data;
pub mod error;
mod sexpr;

use crate::decl_v2::{data::avatar::DeclAvatar, error::DeclError, sexpr::load_avatar_sexpr};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeclarationFormat {
    Sexpr,
    Lua,
}

pub fn load_declaration(text: &str, format: DeclarationFormat) -> Result<DeclAvatar, DeclError> {
    match format {
        DeclarationFormat::Sexpr => load_avatar_sexpr(text),
        _ => Err(DeclError::UnsupportedFormat),
    }
}
