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
pub struct Arguments {
    library_paths: HashSet<PathBuf>,
    symbols: HashSet<String>,
    localizations: HashMap<String, String>,
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.library_paths.clear();
        self.symbols.clear();
        self.localizations.clear();
    }

    pub fn add_library_path(&mut self, path: impl Into<PathBuf>) -> bool {
        self.library_paths.insert(path.into())
    }

    pub fn define_symbol(&mut self, symbol: &str) -> bool {
        let canonical_symbol = symbol.trim();
        if canonical_symbol.is_empty() {
            false
        } else {
            self.symbols.insert(canonical_symbol.to_string())
        }
    }

    pub fn define_localization(&mut self, key: &str, value: &str) -> Option<String> {
        let canonical_key = key.trim();
        if canonical_key.is_empty() {
            None
        } else {
            self.localizations
                .insert(canonical_key.to_string(), value.to_string())
        }
    }

    pub fn library_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.library_paths.iter()
    }

    pub fn symbols(&self) -> &HashSet<String> {
        &self.symbols
    }

    pub fn localizations(&self) -> &HashMap<String, String> {
        &self.localizations
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeclarationFormat {
    Sexpr,
    Lua,
}

pub fn compile_declaration(
    text: &str,
    format: DeclarationFormat,
    args: Arguments,
) -> Result<DeclAvatar, DeclError> {
    match format {
        DeclarationFormat::Sexpr => load_avatar_sexpr(text, args),
        _ => Err(DeclError::UnsupportedFormat),
    }
}
