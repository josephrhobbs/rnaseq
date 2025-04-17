//! Library for Sequence Alignment/Map file processing.

mod alignment;
mod base_pair;
mod cigar;
mod flag;
mod quality;
mod sequence;
mod transcriptome;

pub use alignment::Alignment;

pub use base_pair::BasePair;

pub use cigar::Cigar;

pub use flag::AlignmentFlag;

pub use quality::Quality;

pub use sequence::Sequence;

pub use transcriptome::Transcriptome;

/// One-based leftmost position mapping.
pub type Position = u32;

/// Mapping position Phred quality score.
type MappingQuality = u8;

/// Template length.
type TemplateLength = i32;

/// Base pair Phred quality score.
type BasePairQuality = u8;