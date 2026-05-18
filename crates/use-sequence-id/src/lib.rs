#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

fn non_empty_identifier(value: impl AsRef<str>) -> Result<String, SequenceIdError> {
    let value = value.as_ref();

    if value.trim().is_empty() {
        Err(SequenceIdError::Empty)
    } else {
        Ok(value.to_string())
    }
}

/// Error returned by sequence identifier constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceIdError {
    /// The supplied identifier was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for SequenceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("sequence identifier cannot be empty"),
        }
    }
}

impl Error for SequenceIdError {}

/// A non-empty sequence identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SequenceId(String);

impl SequenceId {
    /// Creates a sequence identifier from non-empty text.
    ///
    /// Casing and punctuation are preserved exactly as supplied.
    ///
    /// # Errors
    ///
    /// Returns [`SequenceIdError::Empty`] when the trimmed identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SequenceIdError> {
        non_empty_identifier(value).map(Self)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for SequenceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SequenceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SequenceId {
    type Err = SequenceIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty sequence accession.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Accession(String);

impl Accession {
    /// Creates a sequence accession from non-empty text.
    ///
    /// Casing and punctuation are preserved exactly as supplied.
    ///
    /// # Errors
    ///
    /// Returns [`SequenceIdError::Empty`] when the trimmed accession is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, SequenceIdError> {
        non_empty_identifier(value).map(Self)
    }

    /// Returns the accession text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Accession {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Accession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for Accession {
    type Err = SequenceIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A sequence accession with an optional descriptive version component.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VersionedAccession {
    accession: Accession,
    version: Option<String>,
}

impl VersionedAccession {
    /// Creates a versioned accession without a version component.
    #[must_use]
    pub fn without_version(accession: Accession) -> Self {
        Self {
            accession,
            version: None,
        }
    }

    /// Creates a versioned accession with a non-empty version component.
    ///
    /// # Errors
    ///
    /// Returns [`SequenceIdError::Empty`] when the trimmed version is empty.
    pub fn with_version(
        accession: Accession,
        version: impl AsRef<str>,
    ) -> Result<Self, SequenceIdError> {
        Ok(Self {
            accession,
            version: Some(non_empty_identifier(version)?),
        })
    }

    /// Returns the base accession.
    #[must_use]
    pub fn accession(&self) -> &Accession {
        &self.accession
    }

    /// Returns the optional version component.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

impl fmt::Display for VersionedAccession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.version() {
            Some(version) => write!(formatter, "{}.{}", self.accession, version),
            None => formatter.write_str(self.accession.as_str()),
        }
    }
}

/// Descriptive source labels for sequence identifiers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SequenceSource {
    /// GenBank source label.
    GenBank,
    /// RefSeq source label.
    RefSeq,
    /// Ensembl source label.
    Ensembl,
    /// UniProt source label.
    UniProt,
    /// Protein Data Bank source label.
    Pdb,
    /// Local sequence source.
    Local,
    /// Unknown sequence source.
    Unknown,
    /// Domain-specific source label.
    Custom(String),
}

impl fmt::Display for SequenceSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenBank => formatter.write_str("genbank"),
            Self::RefSeq => formatter.write_str("refseq"),
            Self::Ensembl => formatter.write_str("ensembl"),
            Self::UniProt => formatter.write_str("uniprot"),
            Self::Pdb => formatter.write_str("pdb"),
            Self::Local => formatter.write_str("local"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(source) => formatter.write_str(source),
        }
    }
}

impl FromStr for SequenceSource {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let source = match value.trim().to_ascii_lowercase().as_str() {
            "genbank" | "gen_bank" => Self::GenBank,
            "refseq" | "ref_seq" => Self::RefSeq,
            "ensembl" => Self::Ensembl,
            "uniprot" | "uni_prot" => Self::UniProt,
            "pdb" => Self::Pdb,
            "local" => Self::Local,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(source)
    }
}

#[cfg(test)]
mod tests {
    use super::{Accession, SequenceId, SequenceIdError, SequenceSource, VersionedAccession};
    use core::str::FromStr;

    #[test]
    fn creates_valid_sequence_id() {
        let id = SequenceId::new("chr1:10-20").expect("valid identifier");

        assert_eq!(id.as_str(), "chr1:10-20");
    }

    #[test]
    fn rejects_empty_sequence_id() {
        assert_eq!(SequenceId::new("  "), Err(SequenceIdError::Empty));
    }

    #[test]
    fn constructs_accession_preserving_text() {
        let accession = Accession::new("NM_007294").expect("valid accession");

        assert_eq!(accession.to_string(), "NM_007294");
    }

    #[test]
    fn constructs_versioned_accession() {
        let accession = Accession::new("NM_007294").expect("valid accession");
        let versioned = VersionedAccession::with_version(accession, "3").expect("valid version");

        assert_eq!(versioned.to_string(), "NM_007294.3");
        assert_eq!(versioned.version(), Some("3"));
    }

    #[test]
    fn sequence_source_displays_and_parses() {
        assert_eq!(SequenceSource::GenBank.to_string(), "genbank");
        assert_eq!(
            SequenceSource::from_str("UniProt"),
            Ok(SequenceSource::UniProt)
        );
    }

    #[test]
    fn supports_custom_source() {
        assert_eq!(
            SequenceSource::from_str("lab"),
            Ok(SequenceSource::Custom("lab".into()))
        );
    }

    #[test]
    fn versionless_accession_is_descriptive() {
        let accession = Accession::new("P12345").expect("valid accession");
        let versioned = VersionedAccession::without_version(accession);

        assert_eq!(versioned.to_string(), "P12345");
        assert_eq!(versioned.version(), None);
    }
}
