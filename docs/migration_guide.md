# Migrating from bincode 1 to 4

Bincode 4 supports both bincode's native `Encode` and `Decode` traits and an optional serde integration. It also restores the bincode 1 convenience functions and `Options` workflow as a compatibility facade over the current implementation.

## Choose the compatibility facade or native configuration API

For a minimal migration, enable the `serde` feature and keep using the familiar root functions:

```rust,ignore
let bytes = bincode::serialize(&value)?;
let decoded = bincode::deserialize::<MyType>(&bytes)?;
```

The restored [`Options`](https://docs.rs/bincode/4.0.0/bincode/trait.Options.html) facade starts with `bincode::options()` and uses the current [`Configuration`](https://docs.rs/bincode/4.0.0/bincode/config/struct.Configuration.html) builders:

```rust,ignore
# old
bincode_1::DefaultOptions::new().with_varint_encoding()

# new
bincode::options().with_variable_int_encoding()
```

The main compatibility mappings are:

| Bincode 1                                                              | Bincode 4                                                        |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------- |
| `bincode_1::DefaultOptions::new()`                                     | `bincode::options()`                                             |
| Version 1.0–1.2 `DefaultOptions` encoding                             | `config::legacy()`                                               |
| Version 1.3+ `DefaultOptions` encoding                                | `config::legacy().with_variable_int_encoding()`                  |
| Top-level `bincode::serialize` and `bincode::deserialize`             | The restored top-level functions, which use `config::legacy()`   |

For new code, prefer `config::standard()` with the native encode/decode functions or the functions under `bincode::serde`.

Configuration builder names differ from bincode 1 as follows:

- `.with_limit(n)` has been changed to `.with_limit::<n>()`.
- `.with_varint_encoding()` has been renamed to `.with_variable_int_encoding()`.
- `.with_fixint_encoding()` has been renamed to `.with_fixed_int_encoding()`.
- `.with_native_endian()`, `.reject_trailing_bytes()`, and `.allow_trailing_bytes()` remain available.

Native bincode 4 calls take a `Configuration` explicitly. The compatibility functions and `Options` methods retain bincode 1-style call shapes.

## Migrating with `serde`

You may wish to stick with `serde` when migrating to bincode 4, for example if you are using serde-exclusive derive features such as `#[serde(deserialize_with)]`.

If so, include bincode 4 with the `serde` feature enabled. You can use either the compatibility facade described above or the native `bincode::serde::*` functions:

```toml
[dependencies]
bincode = { version = "4.0", features = ["serde"] }

# Optionally you can disable the `derive` feature:
# bincode = { version = "4.0", default-features = false, features = ["std", "serde"] }
```

Then replace the following functions: (`Configuration` is `bincode::config::legacy()` by default)

| Bincode 1                                       | Bincode 4 native serde API                                                                                                      |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `bincode::deserialize(&[u8])`                   | `bincode::serde::decode_from_slice(&[u8], Configuration)`<br />`bincode::serde::borrow_decode_from_slice(&[u8], Configuration)` |
| `bincode::deserialize_from(std::io::Read)`      | `bincode::serde::decode_from_std_read(std::io::Read, Configuration)`                                                            |
| `bincode::deserialize_from_custom(BincodeRead)` | `bincode::serde::decode_from_reader(Reader, Configuration)`                                                                     |
|                                                 |                                                                                                                                 |
| `bincode::serialize(T)`                         | `bincode::serde::encode_to_vec(T, Configuration)`<br />`bincode::serde::encode_into_slice(T, &mut [u8], Configuration)`         |
| `bincode::serialize_into(std::io::Write, T)`    | `bincode::serde::encode_into_std_write(T, std::io::Write, Configuration)`                                                       |
| `bincode::serialized_size(T)`                   | `bincode::serialized_size(T)` through the compatibility facade                                                                  |

Seeded serde decoding is available through a complete direct and v1-compatible matrix:

| Source | Direct seeded API | `Options` façade |
| --- | --- | --- |
| Borrowed slice | `bincode::serde::seed_decode_from_slice` | `Options::deserialize_seed` |
| Bincode `Reader` | `bincode::serde::seed_decode_from_reader` | `Options::deserialize_from_reader_seed` |
| `std::io::Read` | `bincode::serde::seed_decode_from_std_read` | `Options::deserialize_from_seed` |

## Migrating from `serde` to `bincode-derive`

`bincode-derive` is enabled by default. If you're using `default-features = false`, make sure to add `features = ["derive"]` to your `Cargo.toml`.

```toml,ignore
[dependencies]
bincode = "4.0"

# If you need `no_std` with `alloc`:
# bincode = { version = "4.0", default-features = false, features = ["derive", "alloc"] }

# If you need `no_std` and no `alloc`:
# bincode = { version = "4.0", default-features = false, features = ["derive"] }
```

Replace or add the following attributes. You are able to use both `serde-derive` and `bincode-derive` side-by-side.

| serde-derive                    | bincode-derive               |
| ------------------------------- | ---------------------------- |
| `#[derive(serde::Serialize)]`   | `#[derive(bincode::Encode)]` |
| `#[derive(serde::Deserialize)]` | `#[derive(bincode::Decode)]` |

**note:** To implement these traits manually, see the documentation of [Encode](https://docs.rs/bincode/4.0.0/bincode/enc/trait.Encode.html) and [Decode](https://docs.rs/bincode/4.0.0/bincode/de/trait.Decode.html).

**note:** For more information on using `bincode-derive` with external libraries, see [below](#bincode-derive-and-libraries).

Then replace the following functions: (`Configuration` is `bincode::config::legacy()` by default)

| Bincode 1                                       | Bincode 4 native API                                                                                                |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `bincode::deserialize(&[u8])`                   | `bincode::decode_from_slice(&bytes, Configuration)`<br />`bincode::borrow_decode_from_slice(&[u8], Configuration)` |
| `bincode::deserialize_from(std::io::Read)`      | `bincode::decode_from_std_read(std::io::Read, Configuration)`                                                      |
| `bincode::deserialize_from_custom(BincodeRead)` | `bincode::decode_from_reader(Reader, Configuration)`                                                               |
|                                                 |                                                                                                                    |
| `bincode::serialize(T)`                         | `bincode::encode_to_vec(T, Configuration)`<br />`bincode::encode_into_slice(t: T, &mut [u8], Configuration)`       |
| `bincode::serialize_into(std::io::Write, T)`    | `bincode::encode_into_std_write(T, std::io::Write, Configuration)`                                                 |
| `bincode::serialized_size(T)`                   | `bincode::serialized_size(T)` through the compatibility facade                                                     |

### Bincode derive and libraries

Currently not many libraries support the traits `Encode` and `Decode`. There are a couple of options if you want to use `#[derive(bincode::Encode, bincode::Decode)]`:

- Enable the `serde` feature and add a `#[bincode(with_serde)]` above each field that implements `serde::Serialize/Deserialize` but not `Encode/Decode`
- Enable the `serde` feature and wrap your field in [bincode::serde::Compat](https://docs.rs/bincode/4.0.0/bincode/serde/struct.Compat.html) or [bincode::serde::BorrowCompat](https://docs.rs/bincode/4.0.0/bincode/serde/struct.BorrowCompat.html)
- Make a pull request to the library:
  - Make sure to be respectful, most of the developers are doing this in their free time.
  - Add a dependency `bincode = { version = "4.0", default-features = false, optional = true }` to the `Cargo.toml`
  - Implement [Encode](https://docs.rs/bincode/4.0.0/bincode/enc/trait.Encode.html)
  - Implement [Decode](https://docs.rs/bincode/4.0.0/bincode/de/trait.Decode.html)
  - Make sure both of these implementations have a `#[cfg(feature = "bincode")]` attribute.
