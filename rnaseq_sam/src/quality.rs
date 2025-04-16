//! Sequence quality abstraction.

use crate::BasePairQuality;

#[derive(Clone, Debug)]
/// Abstraction over a complementary DNA quality sequence.
pub struct Quality {
    /// Base pair Phred quality scores (ASCII).
    pub sequence: Vec<BasePairQuality>,
}

impl Quality {
    /// Construct a new quality sequence from an ASCII string.
    pub fn from(ascii: &str) -> Self {
        // Convert characters to 8-bit Phred quality scores
        let sequence = ascii.chars().map(|c: char| c as u8 - 33).collect::<Vec<u8>>();

        Self {
            sequence,
        }
    }
}