//! Genomic feature type abstraction.

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

#[derive(Clone, Copy, PartialEq, Debug)]
/// Abstraction over feature types.
/// 
/// Currently, the RNASEQ Toolkit only supports processing genes.  This may change in the future depending on development requirements.
pub enum FeatureType {
    /// Gene.
    Gene,

    /// Other feature.
    Other,
}

impl TryFrom<&str> for FeatureType {
    type Error = RnaseqError;

    fn try_from(s: &str) -> RnaseqResult<Self> {
        use FeatureType::*;
        match s {
            "gene" => Ok (Gene),
            _ => Ok (Other),
        }
    }
}