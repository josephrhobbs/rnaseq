//! Coding DNA sequence phase abstraction.

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

#[derive(Clone, Copy, Debug)]
/// Abstraction over a coding DNA sequence phase.
pub enum Phase {
    /// Codon begins on first nucleotide of this CDS feature.
    Zero,

    /// Codon begins on second nucleotide of this CDS feature.
    One,

    /// Codon begins on third nucleotide of this CDS feature.
    Two,
}

impl TryFrom<&str> for Phase {
    type Error = RnaseqError;

    fn try_from(s: &str) -> RnaseqResult<Self> {
        use Phase::*;
        match s {
            "0" => Ok (Zero),
            "1" => Ok (One),
            "2" => Ok (Two),
            _   => Err (RnaseqError::InvalidPhase (s.to_string())),
        }
    }
}