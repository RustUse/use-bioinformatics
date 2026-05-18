# use-bioinformatics

Primitive biological sequence and genomic data vocabulary for RustUse.

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

`use-bioinformatics` provides small, composable Rust primitives for biological sequences, residues, alphabets, nucleotides, amino acids, genomic ranges, features, annotations, motifs, alignments, and sequence identifiers.

It is not a genome analysis engine, sequence aligner, annotation pipeline, database client, variant caller, medical interpretation library, lab information system, or bioinformatics framework.

## Relationship to use-biology

`use-bioinformatics` complements `use-biology`.

- `use-biology` owns organism, taxonomy, species, cell, gene, trait, tissue, organ, biological system, life stage, and reproduction vocabulary.
- `use-bioinformatics` owns sequence, residue, alphabet, genomic range, feature, annotation, motif, alignment, and sequence identifier vocabulary.

## Usage

```rust
use use_bioinformatics::prelude::{
    Accession, AlignedSequence, Annotation, AnnotationKey, AnnotationSet, AnnotationValue,
    BioAlphabet, BioSequence, FeatureKind, FeatureName, GenomicPosition, GenomicRange, MotifKind,
    MotifName, MotifPattern, Nucleotide, SequenceFeature, SequenceKind,
};

let sequence = BioSequence::new(SequenceKind::Dna, "ACGTN").unwrap();
let residue = Nucleotide::parse_symbol('A').unwrap();
let dna = BioAlphabet::dna();
let range = GenomicRange::new(
    GenomicPosition::new(10),
    GenomicPosition::new(15),
).unwrap();
let feature = SequenceFeature::new(FeatureKind::Gene, FeatureName::new("BRCA1 region").unwrap())
    .with_range(range.clone());
let annotation = Annotation::new(
    AnnotationKey::new("source").unwrap(),
    AnnotationValue::new("curated example"),
);
let mut annotations = AnnotationSet::new();
annotations.insert(annotation);
let motif = MotifPattern::new("TATA").unwrap();
let motif_name = MotifName::new("TATA box").unwrap();
let aligned = AlignedSequence::new("ACG-TN").unwrap();
let accession = Accession::new("NM_007294").unwrap();

assert_eq!(sequence.as_str(), "ACGTN");
assert_eq!(residue.to_string(), "A");
assert!(dna.contains('A'));
assert_eq!(feature.range(), Some(&range));
assert_eq!(annotations.get("source").unwrap().as_str(), "curated example");
assert_eq!(motif.as_str(), "TATA");
assert_eq!(motif_name.to_string(), "TATA box");
assert_eq!(MotifKind::Dna.to_string(), "dna");
assert_eq!(aligned.aligned_len(), 6);
assert_eq!(accession.to_string(), "NM_007294");
```

This example describes bioinformatics concepts. It does not analyze, align, infer, diagnose, fetch, or parse external file formats.

## Crates

- `use-bioinformatics`: facade crate
- `use-sequence`: primitive biological sequence vocabulary
- `use-residue`: primitive residue vocabulary
- `use-alphabet`: primitive biological alphabet vocabulary
- `use-nucleotide`: primitive nucleotide vocabulary
- `use-amino-acid`: primitive amino acid vocabulary
- `use-genomic-range`: primitive genomic range and interval vocabulary
- `use-feature`: primitive sequence and genomic feature vocabulary
- `use-annotation`: primitive annotation vocabulary
- `use-motif`: primitive biological motif vocabulary
- `use-alignment`: primitive alignment metadata vocabulary
- `use-sequence-id`: primitive sequence identifier vocabulary

## Related Sets

- `use-biology`
- `use-data`
- `use-text`
- `use-encoding`
- `use-validate`
- `use-stats`
- `use-math`

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
