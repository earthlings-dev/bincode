## Workspace Structure

- **`bincode`** (root) - Main crate: encode/decode traits, config, varint, error types, feature-gated modules
- **`derive/`** (`bincode_derive`) - Proc-macro crate providing `#[derive(Encode, Decode, BorrowDecode)]`, built on the `virtue` proc-macro framework
- **`compatibility/`** (`bincode_compatibility`) - Internal crate that tests backward compatibility with randomized roundtrips
- **`fuzz/`** (`bincode-fuzz`) - Fuzz targets (not a workspace member; separate `cargo fuzz` workflow)
