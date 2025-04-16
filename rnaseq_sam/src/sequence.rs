//! cDNA sequence abstraction.

use rnaseq_err::RnaseqResult;

use crate::BasePair;

#[derive(Clone, Debug)]
/// Abstraction over a complementary DNA sequence.
pub struct Sequence {
    pub bases: Vec<BasePair>,
}

impl Sequence {
    /// Construct a sequence from an ASCII string.
    pub fn from(ascii: &str) -> RnaseqResult<Self> {
        let mut bases = Vec::new();

        // Convert each base pair
        for c in ascii.chars() {
            let bp = BasePair::try_from(c)?;
            bases.push(bp);
        }

        Ok (Self {
            bases,
        })
    }
}