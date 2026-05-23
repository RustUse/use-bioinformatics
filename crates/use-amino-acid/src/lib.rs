#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned when parsing amino-acid symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AminoAcidParseError {
    /// The supplied symbol was not recognized as a primitive amino-acid symbol.
    InvalidSymbol(char),
    /// The supplied text was not exactly one character.
    InvalidSymbolText,
}

impl fmt::Display for AminoAcidParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSymbol(symbol) => {
                write!(formatter, "invalid amino-acid symbol `{symbol}`")
            },
            Self::InvalidSymbolText => {
                formatter.write_str("amino-acid symbol text must be one character")
            },
        }
    }
}

impl Error for AminoAcidParseError {}

/// A descriptive amino-acid kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AminoAcidKind {
    /// Standard amino acid.
    Standard,
    /// Stop symbol.
    Stop,
    /// Ambiguous amino-acid symbol.
    Ambiguous,
    /// Unknown amino-acid symbol.
    Unknown,
    /// Domain-specific amino-acid kind.
    Custom(String),
}

impl fmt::Display for AminoAcidKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => formatter.write_str("standard"),
            Self::Stop => formatter.write_str("stop"),
            Self::Ambiguous => formatter.write_str("ambiguous"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for AminoAcidKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "standard" => Self::Standard,
            "stop" => Self::Stop,
            "ambiguous" => Self::Ambiguous,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A validated one-letter amino-acid code.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AminoAcidCode(char);

impl AminoAcidCode {
    /// Creates a one-letter amino-acid code from a recognized symbol.
    ///
    /// # Errors
    ///
    /// Returns [`AminoAcidParseError::InvalidSymbol`] for unrecognized symbols.
    pub fn new(symbol: char) -> Result<Self, AminoAcidParseError> {
        let amino_acid = AminoAcid::parse_symbol(symbol)?;
        Ok(Self(amino_acid.symbol()))
    }

    /// Returns the one-letter code.
    #[must_use]
    pub const fn as_char(self) -> char {
        self.0
    }
}

impl fmt::Display for AminoAcidCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A primitive amino-acid symbol.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AminoAcid {
    /// Alanine.
    Alanine,
    /// Arginine.
    Arginine,
    /// Asparagine.
    Asparagine,
    /// Aspartic acid.
    AsparticAcid,
    /// Cysteine.
    Cysteine,
    /// Glutamine.
    Glutamine,
    /// Glutamic acid.
    GlutamicAcid,
    /// Glycine.
    Glycine,
    /// Histidine.
    Histidine,
    /// Isoleucine.
    Isoleucine,
    /// Leucine.
    Leucine,
    /// Lysine.
    Lysine,
    /// Methionine.
    Methionine,
    /// Phenylalanine.
    Phenylalanine,
    /// Proline.
    Proline,
    /// Serine.
    Serine,
    /// Threonine.
    Threonine,
    /// Tryptophan.
    Tryptophan,
    /// Tyrosine.
    Tyrosine,
    /// Valine.
    Valine,
    /// Stop symbol.
    Stop,
    /// Ambiguous amino-acid symbol, such as `X`.
    Ambiguous(char),
    /// Explicit unknown amino acid, displayed as `?`.
    Unknown,
}

impl AminoAcid {
    /// Parses a common one-letter amino-acid symbol.
    ///
    /// Recognized symbols are the 20 common one-letter codes, `X` for ambiguous, and `*` for stop.
    ///
    /// # Errors
    ///
    /// Returns [`AminoAcidParseError::InvalidSymbol`] when the symbol is not recognized.
    pub const fn parse_symbol(symbol: char) -> Result<Self, AminoAcidParseError> {
        match symbol.to_ascii_uppercase() {
            'A' => Ok(Self::Alanine),
            'R' => Ok(Self::Arginine),
            'N' => Ok(Self::Asparagine),
            'D' => Ok(Self::AsparticAcid),
            'C' => Ok(Self::Cysteine),
            'Q' => Ok(Self::Glutamine),
            'E' => Ok(Self::GlutamicAcid),
            'G' => Ok(Self::Glycine),
            'H' => Ok(Self::Histidine),
            'I' => Ok(Self::Isoleucine),
            'L' => Ok(Self::Leucine),
            'K' => Ok(Self::Lysine),
            'M' => Ok(Self::Methionine),
            'F' => Ok(Self::Phenylalanine),
            'P' => Ok(Self::Proline),
            'S' => Ok(Self::Serine),
            'T' => Ok(Self::Threonine),
            'W' => Ok(Self::Tryptophan),
            'Y' => Ok(Self::Tyrosine),
            'V' => Ok(Self::Valine),
            'X' => Ok(Self::Ambiguous('X')),
            '*' => Ok(Self::Stop),
            _ => Err(AminoAcidParseError::InvalidSymbol(symbol)),
        }
    }

    /// Returns the canonical one-letter symbol.
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Self::Alanine => 'A',
            Self::Arginine => 'R',
            Self::Asparagine => 'N',
            Self::AsparticAcid => 'D',
            Self::Cysteine => 'C',
            Self::Glutamine => 'Q',
            Self::GlutamicAcid => 'E',
            Self::Glycine => 'G',
            Self::Histidine => 'H',
            Self::Isoleucine => 'I',
            Self::Leucine => 'L',
            Self::Lysine => 'K',
            Self::Methionine => 'M',
            Self::Phenylalanine => 'F',
            Self::Proline => 'P',
            Self::Serine => 'S',
            Self::Threonine => 'T',
            Self::Tryptophan => 'W',
            Self::Tyrosine => 'Y',
            Self::Valine => 'V',
            Self::Stop => '*',
            Self::Ambiguous(symbol) => symbol,
            Self::Unknown => '?',
        }
    }

    /// Returns the descriptive amino-acid kind.
    #[must_use]
    pub const fn kind(self) -> AminoAcidKind {
        match self {
            Self::Stop => AminoAcidKind::Stop,
            Self::Ambiguous(_) => AminoAcidKind::Ambiguous,
            Self::Unknown => AminoAcidKind::Unknown,
            _ => AminoAcidKind::Standard,
        }
    }

    /// Returns the common three-letter code where one is defined by this primitive vocabulary.
    #[must_use]
    pub const fn three_letter_code(self) -> Option<&'static str> {
        match self {
            Self::Alanine => Some("Ala"),
            Self::Arginine => Some("Arg"),
            Self::Asparagine => Some("Asn"),
            Self::AsparticAcid => Some("Asp"),
            Self::Cysteine => Some("Cys"),
            Self::Glutamine => Some("Gln"),
            Self::GlutamicAcid => Some("Glu"),
            Self::Glycine => Some("Gly"),
            Self::Histidine => Some("His"),
            Self::Isoleucine => Some("Ile"),
            Self::Leucine => Some("Leu"),
            Self::Lysine => Some("Lys"),
            Self::Methionine => Some("Met"),
            Self::Phenylalanine => Some("Phe"),
            Self::Proline => Some("Pro"),
            Self::Serine => Some("Ser"),
            Self::Threonine => Some("Thr"),
            Self::Tryptophan => Some("Trp"),
            Self::Tyrosine => Some("Tyr"),
            Self::Valine => Some("Val"),
            Self::Stop => Some("Ter"),
            Self::Ambiguous('X') => Some("Xaa"),
            Self::Ambiguous(_) | Self::Unknown => None,
        }
    }
}

impl fmt::Display for AminoAcid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.symbol())
    }
}

impl FromStr for AminoAcid {
    type Err = AminoAcidParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut chars = value.chars();
        let Some(symbol) = chars.next() else {
            return Err(AminoAcidParseError::InvalidSymbolText);
        };

        if chars.next().is_some() {
            Err(AminoAcidParseError::InvalidSymbolText)
        } else {
            Self::parse_symbol(symbol)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AminoAcid, AminoAcidCode, AminoAcidKind, AminoAcidParseError};
    use core::str::FromStr;

    #[test]
    fn parses_common_one_letter_codes() {
        assert_eq!(AminoAcid::parse_symbol('A'), Ok(AminoAcid::Alanine));
        assert_eq!(AminoAcid::parse_symbol('W'), Ok(AminoAcid::Tryptophan));
        assert_eq!(AminoAcid::parse_symbol('v'), Ok(AminoAcid::Valine));
    }

    #[test]
    fn displays_one_letter_codes() {
        assert_eq!(AminoAcid::Alanine.to_string(), "A");
        assert_eq!(AminoAcid::Tryptophan.three_letter_code(), Some("Trp"));
    }

    #[test]
    fn parses_ambiguous_amino_acid() {
        assert_eq!(AminoAcid::parse_symbol('X'), Ok(AminoAcid::Ambiguous('X')));
        assert_eq!(AminoAcid::Ambiguous('X').kind(), AminoAcidKind::Ambiguous);
    }

    #[test]
    fn parses_stop_symbol_when_explicitly_supplied() {
        assert_eq!(AminoAcid::parse_symbol('*'), Ok(AminoAcid::Stop));
        assert_eq!(AminoAcid::Stop.to_string(), "*");
    }

    #[test]
    fn invalid_symbol_behavior_is_documented() {
        assert_eq!(
            AminoAcid::parse_symbol('#'),
            Err(AminoAcidParseError::InvalidSymbol('#'))
        );
        assert_eq!(
            AminoAcid::from_str("AA"),
            Err(AminoAcidParseError::InvalidSymbolText)
        );
    }

    #[test]
    fn amino_acid_code_preserves_valid_symbol() {
        let code = AminoAcidCode::new('m').expect("valid amino-acid code");

        assert_eq!(code.as_char(), 'M');
    }
}
