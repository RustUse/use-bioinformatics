# use-nucleotide

Primitive nucleotide vocabulary.

`use-nucleotide` parses and displays common nucleotide symbols. It keeps DNA/RNA distinctions descriptive and does not perform genome analysis, variant calling, mutation-effect prediction, transcription, or translation.

```rust
use use_nucleotide::Nucleotide;

let base = Nucleotide::parse_symbol('A').unwrap();
let ambiguous = Nucleotide::parse_symbol('N').unwrap();

assert_eq!(base.to_string(), "A");
assert_eq!(ambiguous.to_string(), "N");
```
