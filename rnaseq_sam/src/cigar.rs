//! CIGAR string abstraction.

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

#[derive(Clone, Debug)]
/// Abstraction over a Concise Idiosyncratic Gapped Alignment Report.
pub struct Cigar {
    pub pairs: Vec<CigarPair>,
}

impl Cigar {
    /// Construct a CIGAR string from an ASCII string.
    pub fn from(ascii: &str) -> RnaseqResult<Self> {
        let mut tokens = Vec::new();

        // Construct list of tokens from ASCII string
        let mut current = String::new();
        let mut is_alpha = false;
        for c in ascii.chars() {
            if ('0'..='9').contains(&c) {
                // Character is numeric

                // Are we switching to numeric?
                if is_alpha {
                    is_alpha = false;
                    tokens.push(current.to_owned());
                    current.clear();
                }
                
                current.push(c);
            } else if ('A'..='Z').contains(&c) || ('a'..='z').contains(&c) || c == '=' {
                // Character is alphabetic

                is_alpha = true;

                // Alpha characters are always alone
                if current.len() != 0 {
                    // Only push previous token if this isn't the first token and the previous token exists
                    tokens.push(current.to_owned());
                }
                current.clear();
                current.push(c);
            }
        }

        // Push last token
        tokens.push(current.to_owned());

        // CIGAR pairs
        let mut pairs = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            // Parse integer count
            let count = if let Ok (v) = str::parse::<usize>(&tokens[i]) {
                i += 1;
                v
            } else {
                1usize
            };

            // Parse operation
            use CigarPair::*;
            let pair = match tokens[i].as_str() {
                "m" | "M" => Match (count),
                "d" | "D" => Deletion (count),
                "i" | "I" => Insertion (count),
                "s" | "S" => SoftClipping (count),
                "h" | "H" => HardClipping (count),
                "="       => Match (count),
                "n" | "N" => Deletion (count),
                "x" | "X" => Mismatch (count),
                _ => return Err (RnaseqError::InvalidCigarOp (tokens[i].to_owned())),
            };

            pairs.push(pair);
            i += 1;
        }

        Ok (Self {
            pairs,
        })
    }
}

#[derive(Clone, Copy, Debug)]
/// Abstraction over a single alignment report pair.
pub enum CigarPair {
    /// Alignment column contains identical base pairs.
    Match (usize),

    /// Alignment column contains mismatched base pairs.
    Mismatch (usize),

    /// Deletion (gap in query sequence).
    Deletion (usize),

    /// Insertion (gap in reference sequence).
    Insertion (usize),

    /// Segment of query sequence does not appear in alignment, but full-length query is given.
    SoftClipping (usize),

    /// Segment of query sequence does not appear in alignment, and only aligned segment of query is given.
    HardClipping (usize),
}

#[test]
fn construct_cigar_string() {
    let raw = "8M2I4M1D3M";

    let cigar = Cigar::from(raw);

    assert!(cigar.is_ok());

    dbg!(cigar.unwrap());
}