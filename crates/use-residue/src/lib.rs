#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned by residue vocabulary constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidueError {
    /// The supplied residue symbol text was empty.
    EmptySymbol,
    /// The supplied residue symbol text contained more than one character.
    MultipleSymbols,
}

impl fmt::Display for ResidueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySymbol => formatter.write_str("residue symbol cannot be empty"),
            Self::MultipleSymbols => formatter.write_str("residue symbol must be one character"),
        }
    }
}

impl Error for ResidueError {}

/// A descriptive kind for a biological residue.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResidueKind {
    /// Nucleotide residue.
    Nucleotide,
    /// Amino-acid residue.
    AminoAcid,
    /// Gap residue, commonly represented as `-`.
    Gap,
    /// Ambiguous residue symbol.
    Ambiguous,
    /// Unknown residue kind.
    Unknown,
    /// Domain-specific residue kind.
    Custom(String),
}

impl fmt::Display for ResidueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nucleotide => formatter.write_str("nucleotide"),
            Self::AminoAcid => formatter.write_str("amino-acid"),
            Self::Gap => formatter.write_str("gap"),
            Self::Ambiguous => formatter.write_str("ambiguous"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for ResidueKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "nucleotide" => Self::Nucleotide,
            "amino-acid" | "amino_acid" | "amino acid" => Self::AminoAcid,
            "gap" => Self::Gap,
            "ambiguous" => Self::Ambiguous,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A single residue symbol.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResidueSymbol(char);

impl ResidueSymbol {
    /// Creates a residue symbol from a single character.
    #[must_use]
    pub const fn from_char(value: char) -> Self {
        Self(value)
    }

    /// Creates a residue symbol from text containing exactly one character.
    ///
    /// # Errors
    ///
    /// Returns [`ResidueError::EmptySymbol`] for empty text and
    /// [`ResidueError::MultipleSymbols`] for text with more than one character.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ResidueError> {
        let mut chars = value.as_ref().chars();
        let Some(symbol) = chars.next() else {
            return Err(ResidueError::EmptySymbol);
        };

        if chars.next().is_some() {
            Err(ResidueError::MultipleSymbols)
        } else {
            Ok(Self(symbol))
        }
    }

    /// Returns the residue symbol character.
    #[must_use]
    pub const fn as_char(self) -> char {
        self.0
    }
}

impl fmt::Display for ResidueSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for ResidueSymbol {
    type Err = ResidueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A single residue symbol with a descriptive kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Residue {
    symbol: ResidueSymbol,
    kind: ResidueKind,
}

impl Residue {
    /// Creates a residue from a symbol and kind.
    #[must_use]
    pub fn new(symbol: char, kind: ResidueKind) -> Self {
        Self {
            symbol: ResidueSymbol::from_char(symbol),
            kind,
        }
    }

    /// Creates a gap residue represented by `-`.
    #[must_use]
    pub fn gap() -> Self {
        Self::new('-', ResidueKind::Gap)
    }

    /// Creates an ambiguous residue with the supplied symbol.
    #[must_use]
    pub fn ambiguous(symbol: char) -> Self {
        Self::new(symbol, ResidueKind::Ambiguous)
    }

    /// Returns the residue symbol.
    #[must_use]
    pub fn symbol(&self) -> ResidueSymbol {
        self.symbol
    }

    /// Returns the descriptive residue kind.
    #[must_use]
    pub fn kind(&self) -> &ResidueKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{Residue, ResidueError, ResidueKind, ResidueSymbol};
    use core::str::FromStr;

    #[test]
    fn creates_valid_residue_symbol() {
        let symbol = ResidueSymbol::new("A").expect("valid symbol");

        assert_eq!(symbol.as_char(), 'A');
        assert_eq!(symbol.to_string(), "A");
    }

    #[test]
    fn rejects_empty_or_multiple_symbol_text() {
        assert_eq!(ResidueSymbol::new(""), Err(ResidueError::EmptySymbol));
        assert_eq!(ResidueSymbol::new("AC"), Err(ResidueError::MultipleSymbols));
    }

    #[test]
    fn creates_gap_residue() {
        let residue = Residue::gap();

        assert_eq!(residue.symbol().as_char(), '-');
        assert_eq!(residue.kind(), &ResidueKind::Gap);
    }

    #[test]
    fn creates_ambiguous_residue() {
        let residue = Residue::ambiguous('N');

        assert_eq!(residue.symbol().as_char(), 'N');
        assert_eq!(residue.kind(), &ResidueKind::Ambiguous);
    }

    #[test]
    fn residue_kind_displays_and_parses() {
        assert_eq!(ResidueKind::AminoAcid.to_string(), "amino-acid");
        assert_eq!(ResidueKind::from_str("gap"), Ok(ResidueKind::Gap));
    }

    #[test]
    fn supports_custom_residue_kind() {
        assert_eq!(
            ResidueKind::from_str("modified"),
            Ok(ResidueKind::Custom("modified".into()))
        );
    }
}
