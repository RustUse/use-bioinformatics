# use-sequence

Primitive biological sequence vocabulary.

`use-sequence` stores owned biological sequence text with a descriptive sequence kind. It preserves caller-provided casing and does not parse FASTA/FASTQ, align sequences, fetch sequence data, calculate biological meaning, or implement sequence databases.

```rust
use use_sequence::{BioSequence, SequenceKind};

let sequence = BioSequence::new(SequenceKind::Dna, "AcgT").unwrap();

assert_eq!(sequence.kind(), &SequenceKind::Dna);
assert_eq!(sequence.as_str(), "AcgT");
assert_eq!(sequence.len(), 4);
```
