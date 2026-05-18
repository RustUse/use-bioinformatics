# use-amino-acid

Primitive amino acid vocabulary.

`use-amino-acid` parses and displays common one-letter amino-acid symbols and exposes a small three-letter-code helper. It is symbol vocabulary only; it does not infer protein structure, calculate molecular mass, implement substitution matrices, or translate nucleotide sequences.

```rust
use use_amino_acid::AminoAcid;

let alanine = AminoAcid::parse_symbol('A').unwrap();
let ambiguous = AminoAcid::parse_symbol('X').unwrap();

assert_eq!(alanine.to_string(), "A");
assert_eq!(alanine.three_letter_code(), Some("Ala"));
assert_eq!(ambiguous.to_string(), "X");
```
