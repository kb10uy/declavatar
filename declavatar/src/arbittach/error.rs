use thiserror::Error as ThisError;

#[derive(Debug, Clone, ThisError)]
pub enum TypeError {}
