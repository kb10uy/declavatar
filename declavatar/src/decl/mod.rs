pub mod data;
pub mod document;
pub mod error;

mod compiler;

use crate::decl::{
    document::Document,
    error::{DeclError, DeclErrorKind, Result},
};

use kdl::KdlDocument;
use miette::{Result as MietteResult, SourceOffset, SourceSpan};

pub fn parse_document(source: &str) -> MietteResult<Document> {
    let first_span = SourceSpan::new(
        SourceOffset::from_location(source, 1, 1),
        SourceOffset::from_location(source, 1, 1),
    );

    let kdl: KdlDocument = source.parse()?;
    let document = Document::parse(&kdl, &first_span).map_err(|e| e.with_source(source))?;
    Ok(document)
}
