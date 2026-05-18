# Releasing

`use-bioinformatics` uses lockstep 0.x versioning for the focused crates and facade crate.

Run the local validation path before publishing:

```sh
cargo fmt
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

For a first publish, publish focused crates before the facade crate so crates.io can resolve sibling package versions.