# use-alphabet

Primitive biological alphabet vocabulary.

`use-alphabet` provides deterministic symbol sets for simple DNA, RNA, and protein alphabets, plus custom alphabets. It checks membership only; it does not parse sequence files, search sequences, score residues, or infer biology.

```rust
use use_alphabet::BioAlphabet;

let dna = BioAlphabet::dna();

assert!(dna.contains('A'));
assert!(dna.contains_all("ACGT"));
assert!(!dna.contains('U'));
```
