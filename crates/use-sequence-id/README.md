# use-sequence-id

Primitive sequence identifier vocabulary.

`use-sequence-id` stores sequence identifiers, accessions, optional accession versions, and source labels. It preserves identifier casing and punctuation, and does not validate against remote databases, fetch metadata, resolve accessions, or implement database clients.

```rust
use use_sequence_id::{Accession, SequenceSource, VersionedAccession};

let accession = Accession::new("NM_007294").unwrap();
let versioned = VersionedAccession::with_version(accession, "3").unwrap();

assert_eq!(versioned.to_string(), "NM_007294.3");
assert_eq!(SequenceSource::GenBank.to_string(), "genbank");
```
