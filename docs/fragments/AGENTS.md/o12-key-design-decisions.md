## Key Design Decisions

- Configuration is entirely compile-time via type-level parameters (no runtime branching on config)
- Enum variants are always encoded as `u32` (varint-compressed to ~1 byte with standard config)
- Serde attributes like `#[serde(flatten)]`, `#[serde(skip)]`, `#[serde(tag)]`, `#[serde(untagged)]` are **incompatible** with bincode and will cause data loss
- The `Decode` trait's `Context` parameter enables arena-based allocation patterns without baking a specific allocator into the trait
