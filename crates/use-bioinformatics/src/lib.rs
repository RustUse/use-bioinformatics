#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_alignment as alignment;
pub use use_alphabet as alphabet;
pub use use_amino_acid as amino_acid;
pub use use_annotation as annotation;
pub use use_feature as feature;
pub use use_genomic_range as genomic_range;
pub use use_motif as motif;
pub use use_nucleotide as nucleotide;
pub use use_residue as residue;
pub use use_sequence as sequence;
pub use use_sequence_id as sequence_id;

/// Common primitive bioinformatics vocabulary reexports.
pub mod prelude {
    pub use use_alignment::{
        AlignedSequence, AlignmentId, AlignmentKind, AlignmentScore, AlignmentSummary,
        AlignmentValueError,
    };
    pub use use_alphabet::{AlphabetError, AlphabetKind, AlphabetSymbolSet, BioAlphabet};
    pub use use_amino_acid::{AminoAcid, AminoAcidCode, AminoAcidKind, AminoAcidParseError};
    pub use use_annotation::{
        Annotation, AnnotationKey, AnnotationKeyError, AnnotationSet, AnnotationValue,
    };
    pub use use_feature::{
        FeatureId, FeatureKind, FeatureName, FeatureValueError, SequenceFeature,
    };
    pub use use_genomic_range::{
        CoordinateSystem, GenomicPosition, GenomicRange, GenomicRangeError, Strand,
    };
    pub use use_motif::{MotifHit, MotifKind, MotifName, MotifPattern, MotifValueError};
    pub use use_nucleotide::{
        Nucleotide, NucleotideKind, NucleotideParseError, NucleotideSequenceKind,
    };
    pub use use_residue::{Residue, ResidueError, ResidueKind, ResidueSymbol};
    pub use use_sequence::{
        BioSequence, SequenceError, SequenceKind, SequenceLength, SequenceText,
    };
    pub use use_sequence_id::{
        Accession, SequenceId, SequenceIdError, SequenceSource, VersionedAccession,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::{
        Accession, AlignedSequence, Annotation, AnnotationKey, AnnotationSet, AnnotationValue,
        BioAlphabet, BioSequence, FeatureKind, FeatureName, GenomicPosition, GenomicRange,
        MotifHit, MotifKind, MotifName, MotifPattern, Nucleotide, SequenceFeature, SequenceKind,
    };

    #[test]
    fn facade_composes_bioinformatics_primitives_without_analysis() {
        let sequence = BioSequence::new(SequenceKind::Dna, "ACGTN").expect("valid sequence");
        let residue = Nucleotide::parse_symbol('A').expect("valid nucleotide");
        let dna = BioAlphabet::dna();
        let range = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(15))
            .expect("valid range");
        let feature = SequenceFeature::new(
            FeatureKind::Gene,
            FeatureName::new("BRCA1 region").expect("valid feature name"),
        )
        .with_range(range.clone());
        let annotation = Annotation::new(
            AnnotationKey::new("source").expect("valid annotation key"),
            AnnotationValue::new("curated example"),
        );
        let mut annotations = AnnotationSet::new();
        annotations.insert(annotation);
        let motif = MotifHit::new(
            MotifName::new("TATA box").expect("valid motif name"),
            MotifPattern::new("TATA").expect("valid motif pattern"),
            MotifKind::Dna,
        );
        let aligned = AlignedSequence::new("ACG-TN").expect("valid aligned sequence");
        let accession = Accession::new("NM_007294").expect("valid accession");

        assert_eq!(sequence.as_str(), "ACGTN");
        assert_eq!(residue.to_string(), "A");
        assert!(dna.contains('A'));
        assert_eq!(feature.range(), Some(&range));
        assert_eq!(
            annotations
                .get("source")
                .expect("stored annotation")
                .as_str(),
            "curated example"
        );
        assert_eq!(motif.pattern().as_str(), "TATA");
        assert_eq!(aligned.aligned_len(), 6);
        assert_eq!(accession.to_string(), "NM_007294");
    }
}
