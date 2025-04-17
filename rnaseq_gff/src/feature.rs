//! Genomic feature abstraction.

use std::collections::HashMap;

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

use rnaseq_sam::Position;

use crate::{
    FeatureScore,
    FeatureType,
    Phase,
    Strand,
};

#[derive(Clone, Debug)]
/// Abstraction over a genomic feature.
pub struct Feature {
    /// Sequence ID (likely chromosome number).
    pub seqid: Option<String>,

    /// Operating procedure responsible for generating this feature.
    pub source: Option<String>,

    /// Feature type.
    pub feature_type: Option<FeatureType>,

    /// Start coordinate.
    pub start: Option<Position>,

    /// End coordinate.
    pub end: Option<Position>,

    /// Feature score.
    pub score: Option<FeatureScore>,

    /// Strand corresponding to this feature.
    pub strand: Strand,

    /// Feature phase.
    pub phase: Option<Phase>,

    /// Feature name.
    pub name: Option<String>,
}

impl Feature {
    /// Parse this feature from an ASCII string.
    pub fn from(ascii: &str) -> RnaseqResult<Self> {
        let fields = ascii.split('\t').collect::<Vec<&str>>();

        // Each line has exactly 9 mandatory fields
        if fields.len() != 9 {
            return Err (RnaseqError::MissingFeatureFields (fields.len()));
        }

        // Parse sequence ID
        let seqid = if fields[0] == "." {
            None
        } else {
            Some (fields[0].to_string())
        };

        // Parse source
        let source = if fields[1] == "." {
            None
        } else {
            Some (fields[1].to_string())
        };

        // Parse type
        let feature_type = if fields[2] == "." {
            None
        } else {
            Some (FeatureType::try_from(fields[2])?)
        };

        // Parse start
        let start = if fields[3] == "." {
            None
        } else {
            Some (str::parse::<Position>(fields[3])?)
        };

        // Parse end
        let end = if fields[4] == "." {
            None
        } else {
            Some (str::parse::<Position>(fields[4])?)
        };

        // Parse feature score
        let score = if fields[5] == "." {
            None
        } else {
            Some (str::parse::<FeatureScore>(fields[5])?)
        };

        // Parse strand
        let strand = Strand::try_from(fields[6])?;

        // Parse phase
        let phase = if fields[7] == "." {
            None
        } else {
            Some (Phase::try_from(fields[7])?)
        };

        // Parse attributes
        let raw_attributes = fields[8].split(';').collect::<Vec<&str>>();
        let mut attributes = HashMap::<String, String>::new();
        for key_value in raw_attributes {
            // Split across equals sign
            let split = key_value.split('=').collect::<Vec<&str>>();

            // Skip invalid entries
            if split.len() < 2 {
                continue;
            }

            // Store valid entries in a hash map
            let (key, value) = (split[0], split[1]);
            attributes.insert(key.to_string(), value.to_string());
        }

        // Get name
        let name: Option<String> = attributes.get("Name").cloned();

        Ok (Self {
            seqid,
            source,
            feature_type,
            start,
            end,
            score,
            strand,
            phase,
            name,
        })
    }
}

#[test]
fn parse_gff_feature() {
    let raw = "ctg123\t.\tgene\t1000\t9000\t.\t+\t.\tID=gene00001;Name=EDEN";

    let feature = Feature::from(raw);

    dbg!(feature);
}