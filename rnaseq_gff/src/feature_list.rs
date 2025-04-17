//! Feature list abstraction.

use rnaseq_err::{
    RnaseqResult,
};

use crate::Feature;

#[derive(Clone, Debug)]
/// Abstraction over a genomic feature list.
pub struct FeatureList {
    /// All features contained in this list.
    pub features: Vec<Feature>,
}

impl FeatureList {
    /// Construct a feature list from an input ASCII file.
    pub fn from(ascii: &str) -> RnaseqResult<Self> {
        // Break file into lines
        let lines = ascii.split('\n').collect::<Vec<&str>>();

        // All features
        let mut features = Vec::new();

        for line in lines {
            // Check for comments and empty lines
            if line.starts_with("#") || line.len() == 0 {
                continue;
            }

            // Parse this line
            let feature = Feature::from(&line)?;

            features.push(feature);
        }

        Ok (Self {
            features,
        })
    }
}

#[cfg(test)]
const EXAMPLE_GFF3: &str = include_str!("../example.gff3");

#[test]
fn create_feature_list() {
    let feature_list = FeatureList::from(EXAMPLE_GFF3);

    assert!(feature_list.is_ok());

    dbg!(feature_list.unwrap());
}