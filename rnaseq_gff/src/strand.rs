//! Genomic strand abstraction.

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

#[derive(Clone, Copy, Debug)]
/// Abstraction over a genomic strand.
pub enum Strand {
    /// Positive strand, relative to landmark.
    Positive,

    /// Negative strand, relative to landmark.
    Negative,

    /// Feature is unstranded.
    Unstranded,

    /// Feature strandedness is relevant but unknown.
    Unknown,
}

impl TryFrom<&str> for Strand {
    type Error = RnaseqError;

    fn try_from(s: &str) -> RnaseqResult<Self> {
        use Strand::*;
        match s {
            "+" => Ok (Positive),
            "-" => Ok (Negative),
            "." => Ok (Unstranded),
            "?" => Ok (Unknown),
            _   => Err (RnaseqError::InvalidStrand (s.to_string())),
        }
    }
}