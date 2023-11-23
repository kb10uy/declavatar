pub mod data;
pub mod error;

mod compiler;

use crate::{
    compiler::Compile,
    decl::{
        compiler::DeclCompiler,
        data::Document,
        error::{DeclError, DeclErrorKind, Result},
    },
};

use kdl::KdlDocument;
use miette::Result as MietteResult;

pub fn parse_document(source: &str) -> MietteResult<Document> {
    let kdl: KdlDocument = source.parse()?;

    let mut compiler = DeclCompiler::new();
    let document = compiler.compile(kdl).map_err(|e| e.with_source(source))?;

    Ok(document)
}
