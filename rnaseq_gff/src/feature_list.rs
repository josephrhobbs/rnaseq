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
    /// Construct a feature list from an input ASCII string.
    fn features(ascii: &str) -> RnaseqResult<Vec<Feature>> {
        // Break file into lines
        let lines = ascii.split('\n').collect::<Vec<&str>>();

        // All features
        let mut features = Vec::<Feature>::with_capacity(lines.len());

        for line in lines {
            // Check for comments and empty lines
            if line.starts_with("#") || line.len() == 0 {
                continue;
            }

            // Parse this line
            let feature = Feature::from(&line)?;

            features.push(feature);
        }

        // TODO implement bisection search for linear time complexity, not log-linear
        features.sort_by(|f1, f2| f1.start.cmp(&f2.start));

        Ok (features)
    }

    /// Construct a `FeatureList` structure from an input ASCII string, subject to a given
    ///     filter constraint `check`.
    pub fn from(ascii: &str, check: fn(&Feature) -> bool) -> RnaseqResult<Self> {
        // Get features
        let features = Self::features(ascii)?;

        Ok (Self {
            features: features.into_iter().filter(|f: &Feature| check(f)).collect::<Vec<Feature>>(),
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