#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AlignmentValueError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(AlignmentValueError::Empty)
    } else {
        Ok(value.as_ref().to_string())
    }
}

/// Error returned by alignment vocabulary constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlignmentValueError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for AlignmentValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("alignment value cannot be empty"),
        }
    }
}

impl Error for AlignmentValueError {}

/// A non-empty alignment identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlignmentId(String);

impl AlignmentId {
    /// Creates an alignment identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`AlignmentValueError::Empty`] when the trimmed identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AlignmentValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AlignmentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AlignmentId {
    type Err = AlignmentValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive alignment kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlignmentKind {
    /// Pairwise alignment.
    Pairwise,
    /// Multiple sequence alignment.
    Multiple,
    /// Local alignment.
    Local,
    /// Global alignment.
    Global,
    /// Semi-global alignment.
    SemiGlobal,
    /// Unknown alignment kind.
    Unknown,
    /// Domain-specific alignment kind.
    Custom(String),
}

impl fmt::Display for AlignmentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pairwise => formatter.write_str("pairwise"),
            Self::Multiple => formatter.write_str("multiple"),
            Self::Local => formatter.write_str("local"),
            Self::Global => formatter.write_str("global"),
            Self::SemiGlobal => formatter.write_str("semi-global"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for AlignmentKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "pairwise" => Self::Pairwise,
            "multiple" => Self::Multiple,
            "local" => Self::Local,
            "global" => Self::Global,
            "semi-global" | "semiglobal" | "semi_global" => Self::SemiGlobal,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A numeric alignment score.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlignmentScore(f64);

impl AlignmentScore {
    /// Creates an alignment score.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the score value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// A non-empty aligned sequence string that may contain gaps.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlignedSequence(String);

impl AlignedSequence {
    /// Creates an aligned sequence from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`AlignmentValueError::Empty`] when the trimmed text is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AlignmentValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the aligned sequence text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the number of Unicode scalar values in the aligned text.
    #[must_use]
    pub fn aligned_len(&self) -> usize {
        self.0.chars().count()
    }

    /// Returns true when the aligned text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for AlignedSequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AlignedSequence {
    type Err = AlignmentValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Summary metadata for an alignment.
#[derive(Clone, Debug, PartialEq)]
pub struct AlignmentSummary {
    kind: AlignmentKind,
    score: Option<AlignmentScore>,
    sequences: Vec<AlignedSequence>,
}

impl AlignmentSummary {
    /// Creates an alignment summary with no score or sequences.
    #[must_use]
    pub fn new(kind: AlignmentKind) -> Self {
        Self {
            kind,
            score: None,
            sequences: Vec::new(),
        }
    }

    /// Sets the alignment score.
    #[must_use]
    pub fn with_score(mut self, score: AlignmentScore) -> Self {
        self.score = Some(score);
        self
    }

    /// Adds an aligned sequence to the summary.
    #[must_use]
    pub fn with_sequence(mut self, sequence: AlignedSequence) -> Self {
        self.sequences.push(sequence);
        self
    }

    /// Returns the alignment kind.
    #[must_use]
    pub fn kind(&self) -> &AlignmentKind {
        &self.kind
    }

    /// Returns the optional alignment score.
    #[must_use]
    pub fn score(&self) -> Option<AlignmentScore> {
        self.score
    }

    /// Returns the aligned sequences.
    #[must_use]
    pub fn sequences(&self) -> &[AlignedSequence] {
        &self.sequences
    }

    /// Returns the number of aligned sequences.
    #[must_use]
    pub fn sequence_count(&self) -> usize {
        self.sequences.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AlignedSequence, AlignmentKind, AlignmentScore, AlignmentSummary, AlignmentValueError,
    };
    use core::str::FromStr;

    #[test]
    fn alignment_kind_displays_and_parses() {
        assert_eq!(AlignmentKind::SemiGlobal.to_string(), "semi-global");
        assert_eq!(
            AlignmentKind::from_str("pairwise"),
            Ok(AlignmentKind::Pairwise)
        );
    }

    #[test]
    fn creates_valid_aligned_sequence() {
        let sequence = AlignedSequence::new("ACG-T").expect("valid aligned sequence");

        assert_eq!(sequence.as_str(), "ACG-T");
    }

    #[test]
    fn aligned_length_helper_counts_symbols() {
        let sequence = AlignedSequence::new("ACG-T").expect("valid aligned sequence");

        assert_eq!(sequence.aligned_len(), 5);
    }

    #[test]
    fn rejects_empty_aligned_sequence() {
        assert_eq!(AlignedSequence::new(" "), Err(AlignmentValueError::Empty));
    }

    #[test]
    fn constructs_alignment_score() {
        let score = AlignmentScore::new(42.5);

        assert_eq!(score.value(), 42.5);
    }

    #[test]
    fn supports_custom_alignment_kind() {
        assert_eq!(
            AlignmentKind::from_str("chain"),
            Ok(AlignmentKind::Custom("chain".into()))
        );
    }

    #[test]
    fn alignment_summary_stores_metadata_only() {
        let summary = AlignmentSummary::new(AlignmentKind::Pairwise)
            .with_score(AlignmentScore::new(1.0))
            .with_sequence(AlignedSequence::new("A-C").expect("valid aligned sequence"));

        assert_eq!(summary.kind(), &AlignmentKind::Pairwise);
        assert_eq!(summary.score(), Some(AlignmentScore::new(1.0)));
        assert_eq!(summary.sequence_count(), 1);
    }
}
