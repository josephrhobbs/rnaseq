//! cDNA base pair abstraction.

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

#[derive(Clone, Copy, Debug)]
/// Enumerated data type describing complementary DNA base pairs.
pub enum BasePair {
    /// Adenine (pairs with RNA Uracil).
    Adenine,

    /// Thymine (pairs with RNA Adenine).
    Thymine,

    /// Cytosine (pairs with RNA Guanine).
    Cytosine,

    /// Guanine (pairs with RNA Cytosine).
    Guanine,
}

impl TryFrom<char> for BasePair {
    type Error = RnaseqError;

    fn try_from(c: char) -> RnaseqResult<Self> {
        use BasePair::*;
        match c {
            'a' | 'A' => Ok (Adenine),
            't' | 'T' => Ok (Thymine),
            'c' | 'C' => Ok (Cytosine),
            'g' | 'G' => Ok (Guanine),
            _         => Err (RnaseqError::InvalidBasePair (c)),
        }
    }
}