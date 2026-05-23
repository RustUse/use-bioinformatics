#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when parsing nucleotide symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NucleotideParseError {
    /// The supplied symbol was not recognized as a primitive nucleotide symbol.
    InvalidSymbol(char),
    /// The supplied text was not exactly one character.
    InvalidSymbolText,
}

impl fmt::Display for NucleotideParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSymbol(symbol) => {
                write!(formatter, "invalid nucleotide symbol `{symbol}`")
            },
            Self::InvalidSymbolText => {
                formatter.write_str("nucleotide symbol text must be one character")
            },
        }
    }
}

impl Error for NucleotideParseError {}

/// A descriptive nucleotide kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NucleotideKind {
    /// A DNA-only nucleotide symbol.
    Dna,
    /// An RNA-only nucleotide symbol.
    Rna,
    /// A nucleotide symbol shared by DNA and RNA alphabets.
    Shared,
    /// Gap symbol.
    Gap,
    /// Ambiguous nucleotide symbol.
    Ambiguous,
    /// Unknown nucleotide symbol.
    Unknown,
    /// Domain-specific nucleotide kind.
    Custom(String),
}

impl fmt::Display for NucleotideKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna => formatter.write_str("dna"),
            Self::Rna => formatter.write_str("rna"),
            Self::Shared => formatter.write_str("shared"),
            Self::Gap => formatter.write_str("gap"),
            Self::Ambiguous => formatter.write_str("ambiguous"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for NucleotideKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "dna" => Self::Dna,
            "rna" => Self::Rna,
            "shared" => Self::Shared,
            "gap" => Self::Gap,
            "ambiguous" => Self::Ambiguous,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A descriptive nucleotide sequence kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NucleotideSequenceKind {
    /// DNA sequence.
    Dna,
    /// RNA sequence.
    Rna,
    /// Mixed or generic nucleotide sequence.
    Nucleotide,
    /// Unknown nucleotide sequence kind.
    Unknown,
    /// Domain-specific sequence kind.
    Custom(String),
}

impl fmt::Display for NucleotideSequenceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna => formatter.write_str("dna"),
            Self::Rna => formatter.write_str("rna"),
            Self::Nucleotide => formatter.write_str("nucleotide"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for NucleotideSequenceKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "dna" => Self::Dna,
            "rna" => Self::Rna,
            "nucleotide" => Self::Nucleotide,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A primitive nucleotide symbol.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Nucleotide {
    /// Adenine, displayed as `A`.
    Adenine,
    /// Cytosine, displayed as `C`.
    Cytosine,
    /// Guanine, displayed as `G`.
    Guanine,
    /// Thymine, displayed as `T`.
    Thymine,
    /// Uracil, displayed as `U`.
    Uracil,
    /// Gap, displayed as `-`.
    Gap,
    /// Ambiguous nucleotide symbol, such as `N`.
    Ambiguous(char),
    /// Explicit unknown nucleotide, displayed as `?`.
    Unknown,
}

impl Nucleotide {
    /// Parses a common nucleotide symbol.
    ///
    /// Recognized symbols are `A`, `C`, `G`, `T`, `U`, `-`, and `N` in either case for letters.
    /// Other symbols return [`NucleotideParseError::InvalidSymbol`].
    ///
    /// # Errors
    ///
    /// Returns [`NucleotideParseError::InvalidSymbol`] when the symbol is not recognized.
    pub const fn parse_symbol(symbol: char) -> Result<Self, NucleotideParseError> {
        match symbol.to_ascii_uppercase() {
            'A' => Ok(Self::Adenine),
            'C' => Ok(Self::Cytosine),
            'G' => Ok(Self::Guanine),
            'T' => Ok(Self::Thymine),
            'U' => Ok(Self::Uracil),
            'N' => Ok(Self::Ambiguous('N')),
            '-' => Ok(Self::Gap),
            _ => Err(NucleotideParseError::InvalidSymbol(symbol)),
        }
    }

    /// Returns the canonical display symbol.
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Self::Adenine => 'A',
            Self::Cytosine => 'C',
            Self::Guanine => 'G',
            Self::Thymine => 'T',
            Self::Uracil => 'U',
            Self::Gap => '-',
            Self::Ambiguous(symbol) => symbol,
            Self::Unknown => '?',
        }
    }

    /// Returns the descriptive nucleotide kind.
    #[must_use]
    pub const fn kind(self) -> NucleotideKind {
        match self {
            Self::Thymine => NucleotideKind::Dna,
            Self::Uracil => NucleotideKind::Rna,
            Self::Adenine | Self::Cytosine | Self::Guanine => NucleotideKind::Shared,
            Self::Gap => NucleotideKind::Gap,
            Self::Ambiguous(_) => NucleotideKind::Ambiguous,
            Self::Unknown => NucleotideKind::Unknown,
        }
    }
}

impl fmt::Display for Nucleotide {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.symbol())
    }
}

impl FromStr for Nucleotide {
    type Err = NucleotideParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.chars();
        let Some(symbol) = chars.next() else {
            return Err(NucleotideParseError::InvalidSymbolText);
        };

        if chars.next().is_some() {
            Err(NucleotideParseError::InvalidSymbolText)
        } else {
            Self::parse_symbol(symbol)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Nucleotide, NucleotideKind, NucleotideParseError, NucleotideSequenceKind};
    use core::str::FromStr;

    #[test]
    fn parses_common_nucleotide_symbols() {
        assert_eq!(Nucleotide::parse_symbol('A'), Ok(Nucleotide::Adenine));
        assert_eq!(Nucleotide::parse_symbol('C'), Ok(Nucleotide::Cytosine));
        assert_eq!(Nucleotide::parse_symbol('G'), Ok(Nucleotide::Guanine));
        assert_eq!(Nucleotide::parse_symbol('T'), Ok(Nucleotide::Thymine));
        assert_eq!(Nucleotide::parse_symbol('U'), Ok(Nucleotide::Uracil));
    }

    #[test]
    fn parses_gap_symbol() {
        assert_eq!(Nucleotide::parse_symbol('-'), Ok(Nucleotide::Gap));
    }

    #[test]
    fn parses_ambiguous_n_symbol() {
        assert_eq!(
            Nucleotide::parse_symbol('N'),
            Ok(Nucleotide::Ambiguous('N'))
        );
        assert_eq!(
            Nucleotide::parse_symbol('n'),
            Ok(Nucleotide::Ambiguous('N'))
        );
    }

    #[test]
    fn displays_canonical_symbols() {
        assert_eq!(Nucleotide::Adenine.to_string(), "A");
        assert_eq!(Nucleotide::Gap.to_string(), "-");
        assert_eq!(Nucleotide::Ambiguous('N').to_string(), "N");
    }

    #[test]
    fn invalid_symbols_are_rejected() {
        assert_eq!(
            Nucleotide::parse_symbol('X'),
            Err(NucleotideParseError::InvalidSymbol('X'))
        );
        assert_eq!(
            Nucleotide::from_str("AC"),
            Err(NucleotideParseError::InvalidSymbolText)
        );
    }

    #[test]
    fn nucleotide_kinds_are_descriptive() {
        assert_eq!(Nucleotide::Thymine.kind(), NucleotideKind::Dna);
        assert_eq!(
            NucleotideSequenceKind::from_str("rna"),
            Ok(NucleotideSequenceKind::Rna)
        );
    }
}
