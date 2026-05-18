#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::{collections::BTreeMap, error::Error};

/// Error returned by annotation key constructors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnotationKeyError {
    /// The supplied key was empty after trimming surrounding whitespace.
    Empty,
}

impl fmt::Display for AnnotationKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("annotation key cannot be empty"),
        }
    }
}

impl Error for AnnotationKeyError {}

/// A non-empty annotation key.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnnotationKey(String);

impl AnnotationKey {
    /// Creates an annotation key from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`AnnotationKeyError::Empty`] when the trimmed key is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, AnnotationKeyError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            Err(AnnotationKeyError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for AnnotationKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AnnotationKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AnnotationKey {
    type Err = AnnotationKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A plain string annotation value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnnotationValue(String);

impl AnnotationValue {
    /// Creates an annotation value from plain string data.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the value text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for AnnotationValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AnnotationValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<&str> for AnnotationValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for AnnotationValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// A single annotation key-value pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    key: AnnotationKey,
    value: AnnotationValue,
}

impl Annotation {
    /// Creates an annotation from a key and value.
    #[must_use]
    pub fn new(key: AnnotationKey, value: AnnotationValue) -> Self {
        Self { key, value }
    }

    /// Returns the annotation key.
    #[must_use]
    pub fn key(&self) -> &AnnotationKey {
        &self.key
    }

    /// Returns the annotation value.
    #[must_use]
    pub fn value(&self) -> &AnnotationValue {
        &self.value
    }

    /// Splits the annotation into its key and value.
    #[must_use]
    pub fn into_parts(self) -> (AnnotationKey, AnnotationValue) {
        (self.key, self.value)
    }
}

/// A deterministic annotation collection.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AnnotationSet {
    values: BTreeMap<AnnotationKey, AnnotationValue>,
}

impl AnnotationSet {
    /// Creates an empty annotation set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts an annotation and returns the previous value for the key, if any.
    pub fn insert(&mut self, annotation: Annotation) -> Option<AnnotationValue> {
        let (key, value) = annotation.into_parts();
        self.values.insert(key, value)
    }

    /// Inserts a key-value pair and returns the previous value for the key, if any.
    ///
    /// # Errors
    ///
    /// Returns [`AnnotationKeyError::Empty`] when the key is empty.
    pub fn insert_pair(
        &mut self,
        key: impl AsRef<str>,
        value: impl Into<AnnotationValue>,
    ) -> Result<Option<AnnotationValue>, AnnotationKeyError> {
        Ok(self.values.insert(AnnotationKey::new(key)?, value.into()))
    }

    /// Gets a value by key text.
    #[must_use]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&AnnotationValue> {
        let key = AnnotationKey::new(key).ok()?;
        self.values.get(&key)
    }

    /// Removes a value by key text.
    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<AnnotationValue> {
        let key = AnnotationKey::new(key).ok()?;
        self.values.remove(&key)
    }

    /// Iterates annotations in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = (&AnnotationKey, &AnnotationValue)> {
        self.values.iter()
    }

    /// Returns the number of annotations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true when the set has no annotations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{Annotation, AnnotationKey, AnnotationKeyError, AnnotationSet, AnnotationValue};

    #[test]
    fn creates_valid_annotation_key() {
        let key = AnnotationKey::new("source").expect("valid key");

        assert_eq!(key.as_str(), "source");
    }

    #[test]
    fn rejects_empty_annotation_key() {
        assert_eq!(AnnotationKey::new("  "), Err(AnnotationKeyError::Empty));
    }

    #[test]
    fn constructs_annotation_value() {
        let value = AnnotationValue::new("manual");

        assert_eq!(value.as_str(), "manual");
    }

    #[test]
    fn annotation_ordering_is_deterministic() {
        let mut annotations = AnnotationSet::new();
        annotations.insert_pair("zeta", "last").expect("valid key");
        annotations
            .insert_pair("alpha", "first")
            .expect("valid key");

        let keys = annotations
            .iter()
            .map(|(key, _)| key.as_str())
            .collect::<Vec<_>>();
        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn insert_get_remove_behavior() {
        let mut annotations = AnnotationSet::new();
        let annotation = Annotation::new(
            AnnotationKey::new("source").expect("valid key"),
            AnnotationValue::new("manual"),
        );

        assert_eq!(annotations.insert(annotation), None);
        assert_eq!(
            annotations.get("source").expect("stored value").as_str(),
            "manual"
        );
        assert_eq!(
            annotations.remove("source"),
            Some(AnnotationValue::new("manual"))
        );
        assert!(annotations.is_empty());
    }
}
