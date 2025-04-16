//! Transcriptome abstraction.

use crate::Alignment;

#[derive(Clone, Debug)]
/// Abstraction over an organism's transcriptome.
pub struct Transcriptome {
    /// Sequence of alignment lines.
    pub alignments: Vec<Alignment>,
}