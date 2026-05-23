#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;
use use_annotation::AnnotationSet;
use use_genomic_range::GenomicRange;

fn non_empty_text(value: impl AsRef<str>) -> Result<String, FeatureValueError> {
    let trimmed = value.as_ref().trim();

    if trimmed.is_empty() {
        Err(FeatureValueError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

/// Error returned by feature vocabulary constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureValueError {
    /// The supplied value was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for FeatureValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("feature value cannot be empty"),
        }
    }
}

impl Error for FeatureValueError {}

/// A non-empty feature identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FeatureId(String);

impl FeatureId {
    /// Creates a feature identifier from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FeatureValueError::Empty`] when the trimmed identifier is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FeatureValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeatureId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FeatureId {
    type Err = FeatureValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty feature name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FeatureName(String);

impl FeatureName {
    /// Creates a feature name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`FeatureValueError::Empty`] when the trimmed name is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FeatureValueError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the feature name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeatureName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FeatureName {
    type Err = FeatureValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive feature kind.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FeatureKind {
    /// Gene feature.
    Gene,
    /// Exon feature.
    Exon,
    /// Intron feature.
    Intron,
    /// Coding sequence feature.
    Cds,
    /// Untranslated region feature.
    Utr,
    /// Promoter feature.
    Promoter,
    /// Enhancer feature.
    Enhancer,
    /// Repeat feature.
    Repeat,
    /// Motif feature.
    Motif,
    /// Variant feature label.
    Variant,
    /// Generic region feature.
    Region,
    /// Unknown feature kind.
    Unknown,
    /// Domain-specific feature kind.
    Custom(String),
}

impl fmt::Display for FeatureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gene => formatter.write_str("gene"),
            Self::Exon => formatter.write_str("exon"),
            Self::Intron => formatter.write_str("intron"),
            Self::Cds => formatter.write_str("cds"),
            Self::Utr => formatter.write_str("utr"),
            Self::Promoter => formatter.write_str("promoter"),
            Self::Enhancer => formatter.write_str("enhancer"),
            Self::Repeat => formatter.write_str("repeat"),
            Self::Motif => formatter.write_str("motif"),
            Self::Variant => formatter.write_str("variant"),
            Self::Region => formatter.write_str("region"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(kind) => formatter.write_str(kind),
        }
    }
}

impl FromStr for FeatureKind {
    type Err = core::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let kind = match value.trim().to_ascii_lowercase().as_str() {
            "gene" => Self::Gene,
            "exon" => Self::Exon,
            "intron" => Self::Intron,
            "cds" => Self::Cds,
            "utr" => Self::Utr,
            "promoter" => Self::Promoter,
            "enhancer" => Self::Enhancer,
            "repeat" => Self::Repeat,
            "motif" => Self::Motif,
            "variant" => Self::Variant,
            "region" => Self::Region,
            "unknown" | "" => Self::Unknown,
            _ => Self::Custom(value.to_string()),
        };

        Ok(kind)
    }
}

/// A sequence or genomic feature with optional range and deterministic attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequenceFeature {
    kind: FeatureKind,
    name: FeatureName,
    id: Option<FeatureId>,
    range: Option<GenomicRange>,
    attributes: AnnotationSet,
}

impl SequenceFeature {
    /// Creates a sequence feature.
    #[must_use]
    pub const fn new(kind: FeatureKind, name: FeatureName) -> Self {
        Self {
            kind,
            name,
            id: None,
            range: None,
            attributes: AnnotationSet::new(),
        }
    }

    /// Sets the optional feature identifier.
    #[must_use]
    pub fn with_id(mut self, id: FeatureId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the optional genomic range.
    #[must_use]
    pub fn with_range(mut self, range: GenomicRange) -> Self {
        self.range = Some(range);
        self
    }

    /// Sets deterministic feature attributes.
    #[must_use]
    pub fn with_attributes(mut self, attributes: AnnotationSet) -> Self {
        self.attributes = attributes;
        self
    }

    /// Returns the feature kind.
    #[must_use]
    pub const fn kind(&self) -> &FeatureKind {
        &self.kind
    }

    /// Returns the feature name.
    #[must_use]
    pub const fn name(&self) -> &FeatureName {
        &self.name
    }

    /// Returns the optional feature identifier.
    #[must_use]
    pub const fn id(&self) -> Option<&FeatureId> {
        self.id.as_ref()
    }

    /// Returns the optional genomic range.
    #[must_use]
    pub const fn range(&self) -> Option<&GenomicRange> {
        self.range.as_ref()
    }

    /// Returns deterministic feature attributes.
    #[must_use]
    pub const fn attributes(&self) -> &AnnotationSet {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use super::{FeatureKind, FeatureName, FeatureValueError, SequenceFeature};
    use core::str::FromStr;
    use use_genomic_range::{GenomicPosition, GenomicRange};

    #[test]
    fn creates_valid_feature_name() {
        let name = FeatureName::new("BRCA1 region").expect("valid feature name");

        assert_eq!(name.as_str(), "BRCA1 region");
    }

    #[test]
    fn rejects_empty_feature_name() {
        assert_eq!(FeatureName::new(" "), Err(FeatureValueError::Empty));
    }

    #[test]
    fn feature_kind_displays_and_parses() {
        assert_eq!(FeatureKind::Cds.to_string(), "cds");
        assert_eq!(FeatureKind::from_str("enhancer"), Ok(FeatureKind::Enhancer));
    }

    #[test]
    fn supports_custom_feature_kind() {
        assert_eq!(
            FeatureKind::from_str("operator"),
            Ok(FeatureKind::Custom("operator".into()))
        );
    }

    #[test]
    fn creates_feature_with_genomic_range() {
        let range = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(20))
            .expect("valid range");
        let feature = SequenceFeature::new(
            FeatureKind::Gene,
            FeatureName::new("BRCA1 region").expect("valid feature name"),
        )
        .with_range(range.clone());

        assert_eq!(feature.kind(), &FeatureKind::Gene);
        assert_eq!(feature.range(), Some(&range));
    }
}
