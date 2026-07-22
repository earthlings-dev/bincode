### Core Traits (no_std, no alloc)

- **`Encode`** (`src/enc/mod.rs`) - Serialization trait. Types implement `encode<E: Encoder>()`.
- **`Decode<Context>`** (`src/de/mod.rs`) - Deserialization for owned types. Generic over a `Context` type parameter for arena allocators etc.
- **`BorrowDecode<'de, Context>`** (`src/de/mod.rs`) - Deserialization for borrowed types (`&str`, `&[u8]`).
