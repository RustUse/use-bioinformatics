# use-genomic-range

Primitive genomic range and interval vocabulary.

`use-genomic-range` stores start and end positions, strand labels, and coordinate-system labels. It validates that the end is not before the start. It does not parse BED/GFF/GTF/SAM/BAM, implement interval trees, or behave as a genome browser.

```rust
use use_genomic_range::{CoordinateSystem, GenomicPosition, GenomicRange, Strand};

let range = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(20))
    .unwrap()
    .with_coordinate_system(CoordinateSystem::ZeroBasedHalfOpen)
    .with_strand(Strand::Forward);

assert_eq!(range.len(), 10);
assert_eq!(range.strand(), &Strand::Forward);
```
