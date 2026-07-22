### Varint Encoding (`src/varint/`)

Variable-length integer encoding using a prefix byte scheme (values < 251 in 1 byte, then 2/4/8/16 byte payloads). Signed integers use zigzag encoding. Separate files for encode/decode of signed/unsigned.
