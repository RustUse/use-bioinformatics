# use-motif

Primitive biological motif vocabulary.

`use-motif` stores motif names, plain motif patterns, motif kind labels, and optional hit locations. It does not implement motif search algorithms, regex engines, position-weight matrices, or biological significance inference.

```rust
use use_motif::{MotifHit, MotifKind, MotifName, MotifPattern};

let hit = MotifHit::new(
    MotifName::new("TATA box").unwrap(),
    MotifPattern::new("TATA").unwrap(),
    MotifKind::Dna,
)
.with_sequence_range(3, 7)
.unwrap();

assert_eq!(hit.pattern().as_str(), "TATA");
assert_eq!(hit.sequence_range(), Some((3, 7)));
```
