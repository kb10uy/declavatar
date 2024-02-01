pub mod data;
pub mod error;
mod sexpr;

use crate::decl_v2::{data::avatar::DeclAvatar, error::DeclError, sexpr::load_avatar_sexpr};

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CompilerArguments {
    pub library_paths: HashSet<PathBuf>,
    pub symbols: HashSet<String>,
    pub localizations: HashMap<String, String>,
}

impl CompilerArguments {
    pub fn new() -> CompilerArguments {
        CompilerArguments {
            library_paths: HashSet::new(),
            symbols: HashSet::new(),
            localizations: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PreprocessData {
    pub symbols: HashSet<String>,
    pub localizations: HashMap<String, String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeclarationFormat {
    Sexpr(Vec<PathBuf>),
    Lua(Vec<PathBuf>),
}

pub fn load_declaration(
    text: &str,
    format: DeclarationFormat,
    preprocess: PreprocessData,
) -> Result<DeclAvatar, DeclError> {
    match format {
        DeclarationFormat::Sexpr(paths) => load_avatar_sexpr(text, paths, preprocess),
        _ => Err(DeclError::UnsupportedFormat),
    }
}
