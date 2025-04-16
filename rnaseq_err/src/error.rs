//! Errors for the RNASEQ Toolkit.

use std::error::Error;

#[derive(Clone, Debug)]
/// Errors available to the RNASEQ Toolkit.
pub enum RnaseqError {
    /// Invalid base pair.
    InvalidBasePair (char),

    /// Mismatched quality sequence (sequence length, quality length).
    MismatchedQualitySequence (usize, usize),

    /// Invalid CIGAR operation.
    InvalidCigarOp (String),

    /// Missing alignment fields (fields found).
    MissingAlignmentFields (usize),

    /// Error raised by an external dependency.
    External (String),
}

impl<T> From<T> for RnaseqError where T: Error {
    fn from(e: T) -> Self {
        let string = if let Some (err) = e.source() {
            err.to_string()
        } else {
            String::new()
        };

        Self::External (string)
    }
}