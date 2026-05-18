# Contributing

Thanks for helping improve `use-bioinformatics`.

This repository contains small RustUse crates for biological sequence and genomic data vocabulary. Keep changes focused, documented, well tested, and free of framework or runtime assumptions.

Before opening a pull request, run:

```sh
cargo fmt
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```
