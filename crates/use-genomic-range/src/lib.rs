#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Error returned by genomic range constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenomicRangeError {
    /// The end position was before the start position.
    EndBeforeStart,
}

impl fmt::Display for GenomicRangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EndBeforeStart => formatter.write_str("genomic range end cannot be before start"),
        }
    }
}

impl Error for GenomicRangeError {}

/// A genomic position value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GenomicPosition(u64);

impl GenomicPosition {
    /// Creates a genomic position.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the stored position value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for GenomicPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Strand orientation vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Strand {
    /// Forward strand.
    Forward,
    /// Reverse strand.
    Reverse,
    /// Unstranded interval.
    Unstranded,
    /// Unknown strand.
    Unknown,
}

impl fmt::Display for Strand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forward => formatter.write_str("forward"),
            Self::Reverse => formatter.write_str("reverse"),
            Self::Unstranded => formatter.write_str("unstranded"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

impl FromStr for Strand {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let strand = match value.trim().to_ascii_lowercase().as_str() {
            "+" | "forward" | "plus" => Self::Forward,
            "-" | "reverse" | "minus" => Self::Reverse,
            "." | "unstranded" | "none" => Self::Unstranded,
            "unknown" | "?" | "" => Self::Unknown,
            _ => Self::Unknown,
        };

        Ok(strand)
    }
}

/// Coordinate-system vocabulary for genomic ranges.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CoordinateSystem {
    /// Zero-based half-open coordinates: `[start, end)`.
    ZeroBasedHalfOpen,
    /// One-based closed coordinates: `[start, end]`.
    OneBasedClosed,
    /// Unknown coordinate assumptions.
    Unknown,
    /// Domain-specific coordinate system.
    Custom(String),
}

impl fmt::Display for CoordinateSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroBasedHalfOpen => formatter.write_str("zero-based-half-open"),
            Self::OneBasedClosed => formatter.write_str("one-based-closed"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(system) => formatter.write_str(system),
        }
    }
}

impl FromStr for CoordinateSystem {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let system = match value.trim().to_ascii_lowercase().as_str() {
            "zero-based-half-open" | "zero_based_half_open" | "0-based-half-open" => {
                Self::ZeroBasedHalfOpen
            },
            "one-based-closed" | "one_based_closed" | "1-based-closed" => Self::OneBasedClosed,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(system)
    }
}

/// A genomic interval with explicit coordinate assumptions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenomicRange {
    start: GenomicPosition,
    end: GenomicPosition,
    strand: Strand,
    coordinate_system: CoordinateSystem,
}

impl GenomicRange {
    /// Creates a genomic range using zero-based half-open coordinates and unstranded orientation.
    ///
    /// # Errors
    ///
    /// Returns [`GenomicRangeError::EndBeforeStart`] when `end < start`.
    pub fn new(start: GenomicPosition, end: GenomicPosition) -> Result<Self, GenomicRangeError> {
        if end < start {
            Err(GenomicRangeError::EndBeforeStart)
        } else {
            Ok(Self {
                start,
                end,
                strand: Strand::Unstranded,
                coordinate_system: CoordinateSystem::ZeroBasedHalfOpen,
            })
        }
    }

    /// Sets the strand without changing positions.
    #[must_use]
    pub fn with_strand(mut self, strand: Strand) -> Self {
        self.strand = strand;
        self
    }

    /// Sets the coordinate system without converting positions.
    #[must_use]
    pub fn with_coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.coordinate_system = coordinate_system;
        self
    }

    /// Returns the start position.
    #[must_use]
    pub const fn start(&self) -> GenomicPosition {
        self.start
    }

    /// Returns the end position.
    #[must_use]
    pub const fn end(&self) -> GenomicPosition {
        self.end
    }

    /// Returns the strand.
    #[must_use]
    pub fn strand(&self) -> &Strand {
        &self.strand
    }

    /// Returns the coordinate system.
    #[must_use]
    pub fn coordinate_system(&self) -> &CoordinateSystem {
        &self.coordinate_system
    }

    /// Returns the interval length according to the stored coordinate system.
    ///
    /// No coordinate conversion is performed. For unknown and custom coordinate systems, the
    /// helper returns `end - start` after constructor validation.
    #[must_use]
    pub fn len(&self) -> u64 {
        match &self.coordinate_system {
            CoordinateSystem::OneBasedClosed => self.end.value() - self.start.value() + 1,
            CoordinateSystem::ZeroBasedHalfOpen
            | CoordinateSystem::Unknown
            | CoordinateSystem::Custom(_) => self.end.value() - self.start.value(),
        }
    }

    /// Returns true when the coordinate-system-specific length is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{CoordinateSystem, GenomicPosition, GenomicRange, GenomicRangeError, Strand};
    use core::str::FromStr;

    #[test]
    fn creates_valid_range() {
        let range = GenomicRange::new(GenomicPosition::new(2), GenomicPosition::new(8))
            .expect("valid range");

        assert_eq!(range.start().value(), 2);
        assert_eq!(range.end().value(), 8);
    }

    #[test]
    fn rejects_reversed_range() {
        assert_eq!(
            GenomicRange::new(GenomicPosition::new(8), GenomicPosition::new(2)),
            Err(GenomicRangeError::EndBeforeStart)
        );
    }

    #[test]
    fn strand_displays_and_parses() {
        assert_eq!(Strand::Forward.to_string(), "forward");
        assert_eq!(Strand::from_str("-"), Ok(Strand::Reverse));
    }

    #[test]
    fn coordinate_system_displays_and_parses() {
        assert_eq!(
            CoordinateSystem::ZeroBasedHalfOpen.to_string(),
            "zero-based-half-open"
        );
        assert_eq!(
            CoordinateSystem::from_str("one-based-closed"),
            Ok(CoordinateSystem::OneBasedClosed)
        );
    }

    #[test]
    fn length_helper_uses_coordinate_system() {
        let zero_based = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(15))
            .expect("valid range");
        let one_based = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(15))
            .expect("valid range")
            .with_coordinate_system(CoordinateSystem::OneBasedClosed);

        assert_eq!(zero_based.len(), 5);
        assert_eq!(one_based.len(), 6);
    }
}
