//! Transcriptome abstraction.

use rnaseq_err::{
    RnaseqResult,
};

use crate::Alignment;

#[derive(Clone, Debug)]
/// Abstraction over an organism's transcriptome.
pub struct Transcriptome {
    /// Sequence Alignment/Map file header.
    pub header: String,

    /// Sequence of alignment lines.
    pub alignments: Vec<Alignment>,
}

impl Transcriptome {
    /// Construct a transcriptome from an ASCII Sequence Alignment/Map file.
    pub fn from(ascii: &str) -> RnaseqResult<Self> {
        let lines = ascii.split('\n').collect::<Vec<&str>>();

        // File header
        // TODO actually parse the header properly
        let mut header = String::new();

        // Alignments
        let mut alignments = Vec::<Alignment>::with_capacity(lines.len());

        for line in lines {
            if line.chars().nth(0) == Some ('@') {
                // Header line
                header.push_str(line);
                header.push('\n');
            } else if line.len() == 0 {
                // Empty line
                continue;
            } else {
                // Alignment line
                let alignment = Alignment::from(line)?;
                alignments.push(alignment);
            }
        }

        Ok (Self {
            header,
            alignments,
        })
    }
}

#[cfg(test)]
const EXAMPLE_SAM: &str = include_str!("../example.sam");

#[test]
fn create_transcriptome() {
    let transcriptome = Transcriptome::from(EXAMPLE_SAM);

    assert!(transcriptome.is_ok());

    dbg!(transcriptome.unwrap());
}