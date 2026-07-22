### Feature-Gated Modules (`src/features/`)

- `impl_alloc.rs` - `Vec`, `String`, `Box`, etc. support + `encode_to_vec`
- `impl_std.rs` - `HashMap`, `HashSet`, io-based encode/decode
- `derive.rs` - Re-exports from `bincode_derive`
- `serde/` - Full serde integration: `Compat<T>`/`BorrowCompat<T>` wrappers, serde-specific encode/decode functions, v1-compatible `Options` API in `serde::compat`
