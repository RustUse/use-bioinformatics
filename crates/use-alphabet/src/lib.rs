#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::{collections::BTreeSet, error::Error};

/// Error returned by alphabet constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlphabetError {
    /// The supplied symbol set was empty.
    EmptySymbolSet,
}

impl fmt::Display for AlphabetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySymbolSet => formatter.write_str("alphabet symbol set cannot be empty"),
        }
    }
}

impl Error for AlphabetError {}

/// A descriptive kind for a biological alphabet.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlphabetKind {
    /// DNA alphabet.
    Dna,
    /// RNA alphabet.
    Rna,
    /// Protein alphabet.
    Protein,
    /// DNA alphabet including ambiguity symbols.
    DnaWithAmbiguity,
    /// RNA alphabet including ambiguity symbols.
    RnaWithAmbiguity,
    /// Protein alphabet including ambiguity symbols.
    ProteinWithAmbiguity,
    /// Domain-specific alphabet.
    Custom(String),
}

impl fmt::Display for AlphabetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna => formatter.write_str("dna"),
            Self::Rna => formatter.write_str("rna"),
            Self::Protein => formatter.write_str("protein"),
            Self::DnaWithAmbiguity => formatter.write_str("dna-with-ambiguity"),
            Self::RnaWithAmbiguity => formatter.write_str("rna-with-ambiguity"),
            Self::ProteinWithAmbiguity => formatter.write_str("protein-with-ambiguity"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for AlphabetKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "dna" => Self::Dna,
            "rna" => Self::Rna,
            "protein" => Self::Protein,
            "dna-with-ambiguity" | "dna_with_ambiguity" => Self::DnaWithAmbiguity,
            "rna-with-ambiguity" | "rna_with_ambiguity" => Self::RnaWithAmbiguity,
            "protein-with-ambiguity" | "protein_with_ambiguity" => Self::ProteinWithAmbiguity,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// Deterministic set of biological alphabet symbols.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlphabetSymbolSet {
    symbols: BTreeSet<char>,
}

impl AlphabetSymbolSet {
    /// Creates a symbol set from characters.
    ///
    /// # Errors
    ///
    /// Returns [`AlphabetError::EmptySymbolSet`] when no symbols are supplied.
    pub fn new(symbols: impl IntoIterator<Item = char>) -> Result<Self, AlphabetError> {
        let symbols = symbols.into_iter().collect::<BTreeSet<_>>();

        if symbols.is_empty() {
            Err(AlphabetError::EmptySymbolSet)
        } else {
            Ok(Self { symbols })
        }
    }

    /// Creates a symbol set from string characters.
    ///
    /// # Errors
    ///
    /// Returns [`AlphabetError::EmptySymbolSet`] when the string is empty.
    pub fn from_symbols(symbols: impl AsRef<str>) -> Result<Self, AlphabetError> {
        Self::new(symbols.as_ref().chars())
    }

    /// Returns true when the symbol is present.
    #[must_use]
    pub fn contains(&self, symbol: char) -> bool {
        self.symbols.contains(&symbol)
    }

    /// Returns the number of distinct symbols.
    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns true when the set contains no symbols.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Iterates symbols in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.symbols.iter()
    }
}

/// A biological alphabet with a descriptive kind and symbol set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BioAlphabet {
    kind: AlphabetKind,
    symbols: AlphabetSymbolSet,
}

impl BioAlphabet {
    /// Creates an alphabet from a kind and symbol set.
    #[must_use]
    pub const fn new(kind: AlphabetKind, symbols: AlphabetSymbolSet) -> Self {
        Self { kind, symbols }
    }

    /// Returns the simple DNA alphabet `A`, `C`, `G`, `T`.
    #[must_use]
    pub fn dna() -> Self {
        Self::from_static(AlphabetKind::Dna, "ACGT")
    }

    /// Returns the simple RNA alphabet `A`, `C`, `G`, `U`.
    #[must_use]
    pub fn rna() -> Self {
        Self::from_static(AlphabetKind::Rna, "ACGU")
    }

    /// Returns the common protein alphabet.
    #[must_use]
    pub fn protein() -> Self {
        Self::from_static(AlphabetKind::Protein, "ACDEFGHIKLMNPQRSTVWY")
    }

    /// Returns a DNA alphabet including common ambiguity symbols.
    #[must_use]
    pub fn dna_with_ambiguity() -> Self {
        Self::from_static(AlphabetKind::DnaWithAmbiguity, "ACGTRYSWKMBDHVN")
    }

    /// Returns an RNA alphabet including common ambiguity symbols.
    #[must_use]
    pub fn rna_with_ambiguity() -> Self {
        Self::from_static(AlphabetKind::RnaWithAmbiguity, "ACGURYSWKMBDHVN")
    }

    /// Returns a protein alphabet including common ambiguity symbols.
    #[must_use]
    pub fn protein_with_ambiguity() -> Self {
        Self::from_static(
            AlphabetKind::ProteinWithAmbiguity,
            "ABCDEFGHIKLMNPQRSTVWXYZ*",
        )
    }

    /// Creates a custom alphabet.
    ///
    /// # Errors
    ///
    /// Returns [`AlphabetError::EmptySymbolSet`] when no symbols are supplied.
    pub fn custom(
        kind: impl Into<String>,
        symbols: impl AsRef<str>,
    ) -> Result<Self, AlphabetError> {
        Ok(Self::new(
            AlphabetKind::Custom(kind.into()),
            AlphabetSymbolSet::from_symbols(symbols)?,
        ))
    }

    /// Returns the descriptive alphabet kind.
    #[must_use]
    pub const fn kind(&self) -> &AlphabetKind {
        &self.kind
    }

    /// Returns the alphabet symbols.
    #[must_use]
    pub const fn symbols(&self) -> &AlphabetSymbolSet {
        &self.symbols
    }

    /// Returns true when the symbol is present in the alphabet.
    #[must_use]
    pub fn contains(&self, symbol: char) -> bool {
        self.symbols.contains(symbol)
    }

    /// Returns true when every character in the supplied text is present in the alphabet.
    #[must_use]
    pub fn contains_all(&self, text: impl AsRef<str>) -> bool {
        text.as_ref().chars().all(|symbol| self.contains(symbol))
    }

    fn from_static(kind: AlphabetKind, symbols: &str) -> Self {
        let symbols = AlphabetSymbolSet {
            symbols: symbols.chars().collect(),
        };
        Self::new(kind, symbols)
    }
}

#[cfg(test)]
mod tests {
    use super::{AlphabetError, AlphabetKind, AlphabetSymbolSet, BioAlphabet};
    use core::str::FromStr;

    #[test]
    fn dna_alphabet_contains_standard_symbols() {
        let dna = BioAlphabet::dna();

        assert!(dna.contains_all("ACGT"));
        assert_eq!(dna.kind(), &AlphabetKind::Dna);
    }

    #[test]
    fn rna_alphabet_contains_standard_symbols() {
        let rna = BioAlphabet::rna();

        assert!(rna.contains_all("ACGU"));
        assert_eq!(rna.kind(), &AlphabetKind::Rna);
    }

    #[test]
    fn protein_alphabet_contains_common_symbols() {
        let protein = BioAlphabet::protein();

        assert!(protein.contains_all("ACDEFGHIKLMNPQRSTVWY"));
    }

    #[test]
    fn invalid_symbol_is_rejected_by_membership_check() {
        let dna = BioAlphabet::dna();

        assert!(!dna.contains('U'));
        assert!(!dna.contains_all("ACGU"));
    }

    #[test]
    fn constructs_custom_alphabet() {
        let alphabet = BioAlphabet::custom("toy", "ABC").expect("valid alphabet");

        assert_eq!(alphabet.kind(), &AlphabetKind::Custom("toy".into()));
        assert!(alphabet.contains_all("CBA"));
    }

    #[test]
    fn rejects_empty_symbol_set() {
        assert_eq!(
            AlphabetSymbolSet::from_symbols(""),
            Err(AlphabetError::EmptySymbolSet)
        );
    }

    #[test]
    fn alphabet_kind_displays_and_parses() {
        assert_eq!(
            AlphabetKind::DnaWithAmbiguity.to_string(),
            "dna-with-ambiguity"
        );
        assert_eq!(AlphabetKind::from_str("protein"), Ok(AlphabetKind::Protein));
    }
}
