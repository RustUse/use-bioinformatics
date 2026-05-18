# use-feature

Primitive sequence and genomic feature vocabulary.

`use-feature` stores descriptive feature identifiers, names, kinds, optional genomic ranges, and deterministic attributes. It does not build annotation pipelines, infer features, parse GFF/GTF, or connect to databases.

```rust
use use_feature::{FeatureKind, FeatureName, SequenceFeature};
use use_genomic_range::{GenomicPosition, GenomicRange};

let range = GenomicRange::new(GenomicPosition::new(10), GenomicPosition::new(20)).unwrap();
let feature = SequenceFeature::new(FeatureKind::Gene, FeatureName::new("BRCA1 region").unwrap())
    .with_range(range.clone());

assert_eq!(feature.kind(), &FeatureKind::Gene);
assert_eq!(feature.range(), Some(&range));
```
