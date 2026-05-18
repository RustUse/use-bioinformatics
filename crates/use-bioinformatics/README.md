# use-bioinformatics

Facade crate for primitive RustUse bioinformatics vocabulary.

`use-bioinformatics` re-exports focused crates for biological sequences, residues, alphabets, nucleotides, amino acids, genomic ranges, features, annotations, motifs, alignments, and sequence identifiers.

It is not a bioinformatics framework, alignment engine, genome-analysis toolkit, variant caller, medical library, annotation pipeline, lab information system, or database client.

```rust
use use_bioinformatics::prelude::{
    Accession, AlignedSequence, Annotation, AnnotationKey, AnnotationSet, AnnotationValue,
    BioAlphabet, BioSequence, FeatureKind, FeatureName, GenomicPosition, GenomicRange, MotifHit,
    MotifKind, MotifName, MotifPattern, Nucleotide, SequenceFeature, SequenceKind,
};

let sequence = BioSequence::new(SequenceKind::Dna, "ACGTN").unwrap();
let residue = Nucleotide::parse_symbol('A').unwrap();
let dna = BioAlphabet::dna();
let range = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(15)).unwrap();
let feature = SequenceFeature::new(FeatureKind::Gene, FeatureName::new("BRCA1 region").unwrap())
    .with_range(range.clone());
let annotation = Annotation::new(
    AnnotationKey::new("source").unwrap(),
    AnnotationValue::new("curated example"),
);
let mut annotations = AnnotationSet::new();
annotations.insert(annotation);
let motif = MotifHit::new(
    MotifName::new("TATA box").unwrap(),
    MotifPattern::new("TATA").unwrap(),
    MotifKind::Dna,
);
let aligned = AlignedSequence::new("ACG-TN").unwrap();
let accession = Accession::new("NM_007294").unwrap();

assert_eq!(sequence.as_str(), "ACGTN");
assert_eq!(residue.to_string(), "A");
assert!(dna.contains('A'));
assert_eq!(feature.range(), Some(&range));
assert_eq!(annotations.get("source").unwrap().as_str(), "curated example");
assert_eq!(motif.pattern().as_str(), "TATA");
assert_eq!(aligned.aligned_len(), 6);
assert_eq!(accession.to_string(), "NM_007294");
```

This crate describes bioinformatics concepts. It does not analyze, align, infer, diagnose, fetch, or parse external file formats.
