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

    /// Unknown base pair.
    Unknown,
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
            'n' | 'N' => Ok (Unknown),
            _         => Err (RnaseqError::InvalidBasePair (c)),
        }
    }
}

// /// Convert this base pair into a reduced binary format for easier analysis.
// impl From<BasePair> for u8 {
//     fn from(bp: BasePair) -> Self {
//         use BasePair::*;
//         match bp {
//             Adenine  => 0b0000_0001,
//             Thymine  => 0b0000_0010,
//             Cytosine => 0b0000_0100,
//             Guanine  => 0b0000_1000,
//             Unknown  => 0b0000_0000,
//         }
//     }
// }

// /// Construct this base pair from a reduced binary format for easier analysis.
// impl TryFrom<u8> for BasePair {
//     type Error = RnaseqError;

//     fn try_from(binary: u8) -> RnaseqResult<Self> {
//         use BasePair::*;
//         match binary {
//             0b0000_0001 => Ok (Adenine),
//             0b0000_0010 => Ok (Thymine),
//             0b0000_0100 => Ok (Cytosine),
//             0b0000_1000 => Ok (Guanine),
//             0b0000_0000 => Ok (Unknown),
//             _           => Err (RnaseqError::InvalidBasePairBinary (binary)),
//         }
//     }
// }