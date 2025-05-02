//! Errors for the RNASEQ Toolkit.

use std::error::Error;

#[derive(Clone, Debug)]
/// Errors available to the RNASEQ Toolkit.
pub enum RnaseqError {
    /// Invalid base pair (character found).
    InvalidBasePair (char),

    /// Invalid base pair binary (data found).
    InvalidBasePairBinary (u8),

    /// Mismatched quality sequence (sequence length, quality length).
    MismatchedQualitySequence (usize, usize),

    /// Invalid CIGAR operation.
    InvalidCigarOp (String),

    /// Missing alignment fields in SAM file (fields found).
    MissingAlignmentFields (usize),

    /// Missing feature fields in GFF3 file (fields found).
    MissingFeatureFields (usize),

    /// Invalid strand character (character found).
    InvalidStrand (String),

    /// Invalid coding DNA sequence phase character (character found).
    InvalidPhase (String),

    /// Invalid feature type (character found).
    InvalidFeatureType (String),

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