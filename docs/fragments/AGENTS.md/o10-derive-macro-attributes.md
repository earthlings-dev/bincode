### Derive Macro Attributes

Container-level `#[bincode(...)]` attributes:
- `crate = "..."` - custom path to bincode
- `bounds = "..."` / `encode_bounds` / `decode_bounds` / `borrow_decode_bounds` - custom trait bounds
- `decode_context = "..."` - specify the Context type for Decode

Field-level:
- `#[bincode(with_serde)]` - use serde traits for that field instead of bincode's
