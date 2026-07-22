## Build Commands

```sh
cargo build                          # build with default features (std, derive, serde)
cargo build --no-default-features    # build no_std core only
cargo clippy --workspace --all-targets  # lint the entire workspace
cargo +nightly fmt --all --check     # check formatting (edition 2024 requires nightly fmt)
cargo test --workspace               # run all tests
cargo test --test basic_types        # run a single test file
cargo test --test issues             # run the issues regression tests
cargo bench --bench varint           # run a specific benchmark (varint, inline, string)
```

Tests in `tests/` use a shared `utils.rs` helper with `the_same()` / `the_same_with_comparer()` that roundtrip-tests a value across all config combinations (endianness x int encoding), including serde paths when the feature is enabled.
