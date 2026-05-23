#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;
use use_genomic_range::GenomicRange;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, MotifValueError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(MotifValueError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by motif vocabulary constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotifValueError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
    /// The supplied sequence range ended before it started.
    SequenceRangeEndBeforeStart,
}

impl fmt::Display for MotifValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("motif value cannot be empty"),
            Self::SequenceRangeEndBeforeStart => {
                formatter.write_str("motif hit sequence range end cannot be before start")
            },
        }
    }
}

impl Error for MotifValueError {}

/// A non-empty motif name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MotifName(String);

impl MotifName {
    /// Creates a motif name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`MotifValueError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, MotifValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the motif name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MotifName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MotifName {
    type Err = MotifValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty plain motif pattern.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MotifPattern(String);

impl MotifPattern {
    /// Creates a motif pattern from non-empty plain text.
    ///
    /// The pattern is stored descriptively. It is not treated as a regex or search expression.
    ///
    /// # Errors
    ///
    /// Returns [`MotifValueError::Empty`] when the trimmed pattern is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, MotifValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the motif pattern text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MotifPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MotifPattern {
    type Err = MotifValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive motif kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MotifKind {
    /// DNA motif.
    Dna,
    /// RNA motif.
    Rna,
    /// Protein motif.
    Protein,
    /// Regulatory motif.
    Regulatory,
    /// Binding-site motif.
    BindingSite,
    /// Repeat motif.
    Repeat,
    /// Unknown motif kind.
    Unknown,
    /// Domain-specific motif kind.
    Custom(String),
}

impl fmt::Display for MotifKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna => formatter.write_str("dna"),
            Self::Rna => formatter.write_str("rna"),
            Self::Protein => formatter.write_str("protein"),
            Self::Regulatory => formatter.write_str("regulatory"),
            Self::BindingSite => formatter.write_str("binding-site"),
            Self::Repeat => formatter.write_str("repeat"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for MotifKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "dna" => Self::Dna,
            "rna" => Self::Rna,
            "protein" => Self::Protein,
            "regulatory" => Self::Regulatory,
            "binding-site" | "binding_site" | "binding site" => Self::BindingSite,
            "repeat" => Self::Repeat,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A motif hit with optional sequence or genomic location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotifHit {
    name: MotifName,
    pattern: MotifPattern,
    kind: MotifKind,
    genomic_range: Option<GenomicRange>,
    sequence_range: Option<(usize, usize)>,
}

impl MotifHit {
    /// Creates motif hit metadata with no location.
    #[must_use]
    pub const fn new(name: MotifName, pattern: MotifPattern, kind: MotifKind) -> Self {
        Self {
            name,
            pattern,
            kind,
            genomic_range: None,
            sequence_range: None,
        }
    }

    /// Sets the optional genomic range.
    #[must_use]
    pub fn with_genomic_range(mut self, range: GenomicRange) -> Self {
        self.genomic_range = Some(range);
        self
    }

    /// Sets the optional sequence range.
    ///
    /// # Errors
    ///
    /// Returns [`MotifValueError::SequenceRangeEndBeforeStart`] when `end < start`.
    pub fn with_sequence_range(
        mut self,
        start: usize,
        end: usize,
    ) -> Result<Self, MotifValueError> {
        if end < start {
            Err(MotifValueError::SequenceRangeEndBeforeStart)
        } else {
            self.sequence_range = Some((start, end));
            Ok(self)
        }
    }

    /// Returns the motif name.
    #[must_use]
    pub const fn name(&self) -> &MotifName {
        &self.name
    }

    /// Returns the motif pattern.
    #[must_use]
    pub const fn pattern(&self) -> &MotifPattern {
        &self.pattern
    }

    /// Returns the motif kind.
    #[must_use]
    pub const fn kind(&self) -> &MotifKind {
        &self.kind
    }

    /// Returns the optional genomic range.
    #[must_use]
    pub const fn genomic_range(&self) -> Option<&GenomicRange> {
        self.genomic_range.as_ref()
    }

    /// Returns the optional sequence range as `(start, end)`.
    #[must_use]
    pub const fn sequence_range(&self) -> Option<(usize, usize)> {
        self.sequence_range
    }
}

#[cfg(test)]
mod tests {
    use super::{MotifHit, MotifKind, MotifName, MotifPattern, MotifValueError};
    use core::str::FromStr;

    #[test]
    fn creates_valid_motif_name() {
        let name = MotifName::new("TATA box").expect("valid motif name");

        assert_eq!(name.as_str(), "TATA box");
    }

    #[test]
    fn rejects_empty_motif_name() {
        assert_eq!(MotifName::new(" "), Err(MotifValueError::Empty));
    }

    #[test]
    fn creates_valid_motif_pattern() {
        let pattern = MotifPattern::new("TATA").expect("valid motif pattern");

        assert_eq!(pattern.as_str(), "TATA");
    }

    #[test]
    fn rejects_empty_motif_pattern() {
        assert_eq!(MotifPattern::new(""), Err(MotifValueError::Empty));
    }

    #[test]
    fn motif_kind_displays_and_parses() {
        assert_eq!(MotifKind::BindingSite.to_string(), "binding-site");
        assert_eq!(MotifKind::from_str("dna"), Ok(MotifKind::Dna));
    }

    #[test]
    fn constructs_motif_hit() {
        let hit = MotifHit::new(
            MotifName::new("TATA box").expect("valid motif name"),
            MotifPattern::new("TATA").expect("valid motif pattern"),
            MotifKind::Dna,
        )
        .with_sequence_range(3, 7)
        .expect("valid sequence range");

        assert_eq!(hit.pattern().as_str(), "TATA");
        assert_eq!(hit.sequence_range(), Some((3, 7)));
    }

    #[test]
    fn supports_custom_motif_kind() {
        assert_eq!(
            MotifKind::from_str("hairpin"),
            Ok(MotifKind::Custom("hairpin".into()))
        );
    }
}
