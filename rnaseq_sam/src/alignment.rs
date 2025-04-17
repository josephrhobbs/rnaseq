//! Alignment line abstraction.

use rnaseq_err::{
    RnaseqError,
    RnaseqResult,
};

use crate::{
    AlignmentFlag,
    Cigar,
    MappingQuality,
    Position,
    Quality,
    Sequence,
    TemplateLength,
};

#[derive(Clone, Debug)]
/// Abstraction over a single alignment line.
pub struct Alignment {
    /// Query name.
    pub qname: Option<String>,

    /// Bitwise flag.
    pub flag: AlignmentFlag,

    /// Reference sequence name.
    pub rname: Option<String>,

    /// Position.
    pub pos: Option<Position>,

    /// Mapping quality.
    pub mapq: Option<MappingQuality>,

    /// CIGAR string.
    pub cigar: Option<Cigar>,

    /// Reference name of mate/next read.
    pub rnext: Option<String>,

    /// Position of mate/next read.
    pub pnext: Option<Position>,

    /// Observed template length.
    pub tlen: Option<TemplateLength>,

    /// Segment sequence.
    pub seq: Option<Sequence>,

    /// Segment quality.
    pub qual: Option<Quality>,
}

impl Alignment {
    /// Parse an alignment line from an ASCII string.
    pub fn from(string: &str) -> RnaseqResult<Self> {
        let fields = string.split('\t').collect::<Vec<&str>>();

        // Each line has 11 mandatory fields
        if fields.len() < 11 {
            return Err (RnaseqError::MissingAlignmentFields (fields.len()));
        }

        // Parse query name
        let qname = if fields[0] == "*" {
            None
        } else {
            Some (fields[0].to_string())
        };

        // Parse flag
        let flag = AlignmentFlag::from(str::parse::<u16>(fields[1])?);

        // Parse reference name
        let rname = if fields[2] == "*" {
            None
        } else {
            Some (fields[2].to_string())
        };

        // Parse position
        let pos_val = str::parse::<Position>(fields[3])?;
        let pos = if pos_val == 0 {
            None
        } else {
            Some (pos_val)
        };

        // Parse mapping quality
        let mapq_val = str::parse::<MappingQuality>(fields[4])?;
        let mapq = if mapq_val == 255 {
            None
        } else {
            Some (mapq_val)
        };

        // Parse CIGAR string
        let cigar = if fields[5] == "*" {
            None
        } else {
            Some (Cigar::from(fields[5])?)
        };

        // Parse mate reference name
        let rnext = if fields[6] == "*" {
            None
        } else if fields[6] == "=" {
            rname.clone()
        } else {
            Some (fields[6].to_string())  
        };

        // Parse mate position
        let pnext_val = str::parse::<Position>(fields[7])?;
        let pnext = if pnext_val == 0 {
            None
        } else {
            Some (pnext_val)
        };

        // Parse template length
        let tlen_val = str::parse::<TemplateLength>(fields[8])?;
        let tlen = if tlen_val == 0 {
            None
        } else {
            Some (tlen_val)
        };

        // Parse segment sequence
        let seq = if fields[9] == "*" {
            None
        } else {
            Some (Sequence::from(fields[9])?)
        };

        // Parse quality sequence
        let qual = if fields[10] == "*" {
            None
        } else {
            Some (Quality::from(fields[10]))
        };

        // Assert that segment and quality sequences are same length
        let seq_len = seq.clone().map(|s| s.bases.len());
        let qual_len = qual.clone().map(|q| q.sequence.len());
        if let Some (s) = seq_len {
            if let Some (q) = qual_len {
                if s != q {
                    return Err (RnaseqError::MismatchedQualitySequence (s, q));
                }
            }
        }

        Ok (Self {
            qname,
            flag,
            rname,
            pos,
            mapq,
            cigar,
            rnext,
            pnext,
            tlen,
            seq,
            qual,
        })
    }
}

#[test]
fn parse_alignment_line() {
    let raw = "1:497:R:-272+13M17D24M	113	1	497	37	37M	15	100338662	0	CGGGTCTGACCTGAGGAGAACTGTGCTCCGCCTTCAG	0;==-==9;>>>>>=>>>>>>>>>>>=>>>>>>>>>>	XT:A:U	NM:i:0	SM:i:37	AM:i:0	X0:i:1	X1:i:0	XM:i:0	XO:i:0	XG:i:0	MD:Z:37";

    let alignment = Alignment::from(raw);

    // Make sure this is OK
    assert!(alignment.is_ok());

    dbg!(alignment.unwrap());
}