# use-annotation

Primitive annotation vocabulary.

`use-annotation` stores generic string annotations in deterministic key order. It does not perform ontology lookup, parse GFF/GTF, connect to databases, or infer biological meaning.

```rust
use use_annotation::{Annotation, AnnotationKey, AnnotationSet, AnnotationValue};

let mut annotations = AnnotationSet::new();
annotations.insert(Annotation::new(
    AnnotationKey::new("source").unwrap(),
    AnnotationValue::new("manual"),
));

assert_eq!(annotations.get("source").unwrap().as_str(), "manual");
```
