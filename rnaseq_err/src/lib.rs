//! Main library for RNASEQ Toolkit error handling.

mod error;

pub use error::RnaseqError;

/// RNASEQ Toolkit result type.
pub type RnaseqResult<T> = Result<T, RnaseqError>;