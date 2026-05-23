#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned by sequence vocabulary constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceError {
    /// Sequence text was empty.
    EmptyText,
}

impl fmt::Display for SequenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText => formatter.write_str("sequence text cannot be empty"),
        }
    }
}

impl Error for SequenceError {}

/// A descriptive kind for biological sequence text.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SequenceKind {
    /// DNA sequence text.
    Dna,
    /// RNA sequence text.
    Rna,
    /// Protein sequence text.
    Protein,
    /// Generic nucleotide sequence text.
    Nucleotide,
    /// Generic amino-acid sequence text.
    AminoAcid,
    /// Unknown sequence kind.
    Unknown,
    /// Domain-specific sequence kind.
    Custom(String),
}

impl fmt::Display for SequenceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna => formatter.write_str("dna"),
            Self::Rna => formatter.write_str("rna"),
            Self::Protein => formatter.write_str("protein"),
            Self::Nucleotide => formatter.write_str("nucleotide"),
            Self::AminoAcid => formatter.write_str("amino-acid"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for SequenceKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "dna" => Self::Dna,
            "rna" => Self::Rna,
            "protein" => Self::Protein,
            "nucleotide" => Self::Nucleotide,
            "amino-acid" | "amino_acid" | "amino acid" => Self::AminoAcid,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// Owned biological sequence text.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SequenceText(String);

impl SequenceText {
    /// Creates sequence text from a non-empty string.
    ///
    /// The text is stored exactly as supplied. Casing is not normalized.
    ///
    /// # Errors
    ///
    /// Returns [`SequenceError::EmptyText`] when the supplied text is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SequenceError> {
        let value = value.as_ref();

        if value.is_empty() {
            Err(SequenceError::EmptyText)
        } else {
            Ok(Self(value.to_string()))
        }
    }

    /// Creates explicitly empty sequence text.
    ///
    /// Empty sequences are only produced through this constructor so callers can make that choice
    /// visible in code.
    #[must_use]
    pub const fn empty() -> Self {
        Self(String::new())
    }

    /// Returns the number of Unicode scalar values in the sequence text.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.chars().count()
    }

    /// Returns true when the sequence text is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the sequence text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the sequence text and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SequenceText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SequenceText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SequenceText {
    type Err = SequenceError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A sequence length value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SequenceLength(usize);

impl SequenceLength {
    /// Creates a sequence length from a count.
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the stored length.
    #[must_use]
    pub const fn value(self) -> usize {
        self.0
    }

    /// Returns true when the length is zero.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for SequenceLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Owned biological sequence data with a descriptive kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BioSequence {
    kind: SequenceKind,
    text: SequenceText,
}

impl BioSequence {
    /// Creates a biological sequence from non-empty owned text.
    ///
    /// # Errors
    ///
    /// Returns [`SequenceError::EmptyText`] when the supplied text is empty.
    pub fn new(kind: SequenceKind, text: impl AsRef<str>) -> Result<Self, SequenceError> {
        Ok(Self {
            kind,
            text: SequenceText::new(text)?,
        })
    }

    /// Creates an explicitly empty sequence with the given kind.
    #[must_use]
    pub const fn empty(kind: SequenceKind) -> Self {
        Self {
            kind,
            text: SequenceText::empty(),
        }
    }

    /// Returns the descriptive sequence kind.
    #[must_use]
    pub const fn kind(&self) -> &SequenceKind {
        &self.kind
    }

    /// Returns the owned sequence text wrapper.
    #[must_use]
    pub const fn text(&self) -> &SequenceText {
        &self.text
    }

    /// Returns the number of Unicode scalar values in the sequence text.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns the sequence length wrapper.
    #[must_use]
    pub fn sequence_length(&self) -> SequenceLength {
        SequenceLength::new(self.len())
    }

    /// Returns true when the sequence text is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the sequence text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::{BioSequence, SequenceError, SequenceKind, SequenceLength, SequenceText};
    use core::str::FromStr;

    #[test]
    fn creates_valid_sequence() {
        let sequence = BioSequence::new(SequenceKind::Dna, "ACGT").expect("valid sequence");

        assert_eq!(sequence.kind(), &SequenceKind::Dna);
        assert_eq!(sequence.as_str(), "ACGT");
    }

    #[test]
    fn rejects_empty_sequence_text_by_default() {
        assert_eq!(SequenceText::new(""), Err(SequenceError::EmptyText));
        assert_eq!(
            BioSequence::new(SequenceKind::Unknown, ""),
            Err(SequenceError::EmptyText)
        );
    }

    #[test]
    fn supports_explicit_empty_sequence() {
        let sequence = BioSequence::empty(SequenceKind::Unknown);

        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn sequence_kind_displays_and_parses() {
        assert_eq!(SequenceKind::from_str("DNA"), Ok(SequenceKind::Dna));
        assert_eq!(SequenceKind::AminoAcid.to_string(), "amino-acid");
        assert_eq!(
            SequenceKind::from_str("plasmid"),
            Ok(SequenceKind::Custom("plasmid".into()))
        );
    }

    #[test]
    fn length_helper_reports_text_length() {
        let sequence = BioSequence::new(SequenceKind::Rna, "ACGU").expect("valid sequence");

        assert_eq!(sequence.len(), 4);
        assert_eq!(sequence.sequence_length(), SequenceLength::new(4));
    }

    #[test]
    fn sequence_text_casing_is_preserved() {
        let sequence = BioSequence::new(SequenceKind::Dna, "AcgTn").expect("valid sequence");

        assert_eq!(sequence.as_str(), "AcgTn");
    }
}
