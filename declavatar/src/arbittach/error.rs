use thiserror::Error as ThisError;

#[derive(Debug, Clone, ThisError)]
pub enum TypeError {
    #[error("unsupported type")]
    UnsupportedType,

    #[error("length mismatch; {expected} expected, {found} found")]
    LengthMismatch { expected: usize, found: usize },

    #[error("type mismatch; {expected} expected, {found} found")]
    TypeMismatch { expected: String, found: String },
}
