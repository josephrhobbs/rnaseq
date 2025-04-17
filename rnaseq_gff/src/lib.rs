//! Library for General Feature Format (GFF3) file processing.

mod feature;
mod feature_list;
mod feature_type;
mod phase;
mod strand;

pub use feature::Feature;

pub use feature_list::FeatureList;

pub use feature_type::FeatureType;

pub use phase::Phase;

pub use strand::Strand;

/// Feature score.
type FeatureScore = f64;