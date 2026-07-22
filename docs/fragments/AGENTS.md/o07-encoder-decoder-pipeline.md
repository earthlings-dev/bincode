### Encoder/Decoder Pipeline

Encoding: `value.encode(&mut EncoderImpl<Writer, Config>)` → Writer (SliceWriter, VecWriter, or std::io::Write adapter)
Decoding: `T::decode(&mut DecoderImpl<Reader, Config, Context>)` ← Reader (SliceReader or std::io::Read adapter)

The `Config` trait (`src/config.rs`) is a compile-time configuration using const generics / associated constants. `Configuration<E, I, L, T>` is parameterized by endianness, int encoding, limit, and trailing bytes policy. Config options are enforced at the type level, not runtime.
