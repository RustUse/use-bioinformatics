# use-alignment

Primitive alignment metadata vocabulary.

`use-alignment` models alignment data and metadata only. It can store aligned text with gaps and numeric scores, but it does not implement Needleman-Wunsch, Smith-Waterman, BLAST, scoring matrices, or actual alignment.

```rust
use use_alignment::{AlignedSequence, AlignmentKind, AlignmentScore, AlignmentSummary};

let aligned = AlignedSequence::new("ACG-T").unwrap();
let summary = AlignmentSummary::new(AlignmentKind::Pairwise).with_score(AlignmentScore::new(12.5));

assert_eq!(aligned.aligned_len(), 5);
assert_eq!(summary.kind(), &AlignmentKind::Pairwise);
```
