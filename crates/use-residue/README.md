# use-residue

Primitive residue vocabulary.

`use-residue` describes single biological residue symbols generically across nucleotide and amino-acid sequences. It supports gap and ambiguous residues, but does not infer chemistry, translate residues, score substitutions, or implement alignment algorithms.

```rust
use use_residue::{Residue, ResidueKind};

let gap = Residue::gap();
let ambiguous = Residue::new('N', ResidueKind::Ambiguous);

assert_eq!(gap.symbol().as_char(), '-');
assert_eq!(ambiguous.kind(), &ResidueKind::Ambiguous);
```
