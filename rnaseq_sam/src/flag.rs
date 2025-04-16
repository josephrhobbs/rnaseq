//! Alignment flag abstraction.

#[derive(Clone, Copy, Debug)]
/// Abstraction over a 16-bit alignment flag.
pub struct AlignmentFlag {
    /// Read is paired.
    pub read_paired: bool,

    /// Read is mapped in proper pair.
    pub read_mapped_proper_pair: bool,

    /// Read is unmapped.
    pub read_unmapped: bool,

    /// Mate is unmapped.
    pub mate_unmapped: bool,

    /// Read reverse strand.
    pub read_reverse_strand: bool,

    /// Mate reverse strand.
    pub mate_reverse_strand: bool,

    /// First in pair.
    pub first_in_pair: bool,

    /// Second in pair.
    pub second_in_pair: bool,

    /// Not primary alignment.
    pub not_primary_alignment: bool,

    /// Read fails quality checks.
    pub read_fails_quality_checks: bool,

    /// Read is PCR or optical duplicate.
    pub read_duplicate: bool,

    /// Supplementary alignment.
    pub supplementary_alignment: bool,
}

impl AlignmentFlag {
    /// Construct a new alignment flag.
    pub fn from(flag: u16) -> Self {
        Self {
            read_paired:                flag & 0x1   != 0,
            read_mapped_proper_pair:    flag & 0x2   != 0,
            read_unmapped:              flag & 0x4   != 0,
            mate_unmapped:              flag & 0x8   != 0,
            read_reverse_strand:        flag & 0x10  != 0,
            mate_reverse_strand:        flag & 0x20  != 0,
            first_in_pair:              flag & 0x40  != 0,
            second_in_pair:             flag & 0x80  != 0,
            not_primary_alignment:      flag & 0x100 != 0,
            read_fails_quality_checks:  flag & 0x200 != 0,
            read_duplicate:             flag & 0x400 != 0,
            supplementary_alignment:    flag & 0x800 != 0,
        }
    }
}

#[test]
fn check_alignment_flag() {
    // Flag to be checked
    let flag = AlignmentFlag::from(2459);

    assert!(flag.read_paired);
    assert!(flag.read_mapped_proper_pair);
    assert!(!flag.read_unmapped);
    assert!(flag.mate_unmapped);
    assert!(flag.read_reverse_strand);
    assert!(!flag.mate_reverse_strand);
    assert!(!flag.first_in_pair);
    assert!(flag.second_in_pair);
    assert!(flag.not_primary_alignment);
    assert!(!flag.read_fails_quality_checks);
    assert!(!flag.read_duplicate);
    assert!(flag.supplementary_alignment);
}