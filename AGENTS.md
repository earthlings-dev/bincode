# AGENTS.md

This file provides guidance to Agents when working with code in this repository.

## Project Overview

Bincode is a compact binary encoder/decoder library for Rust (v4.0.0). It provides its own `Encode`/`Decode` traits (preferred) with optional serde compatibility via the `serde` feature flag. The crate is `#![no_std]` by default with optional `alloc` and `std` features.

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

## Workspace Structure

- **`bincode`** (root) - Main crate: encode/decode traits, config, varint, error types, feature-gated modules
- **`derive/`** (`bincode_derive`) - Proc-macro crate providing `#[derive(Encode, Decode, BorrowDecode)]`, built on the `virtue` proc-macro framework
- **`compatibility/`** (`bincode_compatibility`) - Internal crate that tests backward compatibility with randomized roundtrips
- **`fuzz/`** (`bincode-fuzz`) - Fuzz targets (not a workspace member; separate `cargo fuzz` workflow)

## Architecture

### Core Traits (no_std, no alloc)

- **`Encode`** (`src/enc/mod.rs`) - Serialization trait. Types implement `encode<E: Encoder>()`.
- **`Decode<Context>`** (`src/de/mod.rs`) - Deserialization for owned types. Generic over a `Context` type parameter for arena allocators etc.
- **`BorrowDecode<'de, Context>`** (`src/de/mod.rs`) - Deserialization for borrowed types (`&str`, `&[u8]`).

### Encoder/Decoder Pipeline

Encoding: `value.encode(&mut EncoderImpl<Writer, Config>)` → Writer (SliceWriter, VecWriter, or std::io::Write adapter)
Decoding: `T::decode(&mut DecoderImpl<Reader, Config, Context>)` ← Reader (SliceReader or std::io::Read adapter)

The `Config` trait (`src/config.rs`) is a compile-time configuration using const generics / associated constants. `Configuration<E, I, L, T>` is parameterized by endianness, int encoding, limit, and trailing bytes policy. Config options are enforced at the type level, not runtime.

### Varint Encoding (`src/varint/`)

Variable-length integer encoding using a prefix byte scheme (values < 251 in 1 byte, then 2/4/8/16 byte payloads). Signed integers use zigzag encoding. Separate files for encode/decode of signed/unsigned.

### Feature-Gated Modules (`src/features/`)

- `impl_alloc.rs` - `Vec`, `String`, `Box`, etc. support + `encode_to_vec`
- `impl_std.rs` - `HashMap`, `HashSet`, io-based encode/decode
- `derive.rs` - Re-exports from `bincode_derive`
- `serde/` - Full serde integration: `Compat<T>`/`BorrowCompat<T>` wrappers, serde-specific encode/decode functions, v1-compatible `Options` API in `serde::compat`

### Derive Macro Attributes

Container-level `#[bincode(...)]` attributes:
- `crate = "..."` - custom path to bincode
- `bounds = "..."` / `encode_bounds` / `decode_bounds` / `borrow_decode_bounds` - custom trait bounds
- `decode_context = "..."` - specify the Context type for Decode

Field-level:
- `#[bincode(with_serde)]` - use serde traits for that field instead of bincode's

### Generated Code Inspection

The derive macro exports generated code to `target/generated/bincode/<Name>_Encode.rs` and `<Name>_Decode.rs` for debugging.

## Key Design Decisions

- Configuration is entirely compile-time via type-level parameters (no runtime branching on config)
- Enum variants are always encoded as `u32` (varint-compressed to ~1 byte with standard config)
- Serde attributes like `#[serde(flatten)]`, `#[serde(skip)]`, `#[serde(tag)]`, `#[serde(untagged)]` are **incompatible** with bincode and will cause data loss
- The `Decode` trait's `Context` parameter enables arena-based allocation patterns without baking a specific allocator into the trait
