# Bincode compatibility tests

This internal package verifies that bincode's native, serde, and v1-compatible API surfaces produce the same wire output when they use the same configuration. It encodes values across a range of settings, compares the resulting bytes, and then verifies that all three surfaces decode the original value.

## Adding a test case for your project

To add a test case for your project, follow these steps:

- [ ] Fork the [bincode repository](https://github.com/bincode-org/bincode).
- [ ] Create a new file at `compatibility/src/<name>.rs`.
- [ ] Add a link to your project
- [ ] Add `Licence: MIT OR Apache-2.0` if you agree to distribute your code under this license
- [ ] Add a `mod <name>;` in the `lib.rs`. Make sure it's alphabetically ordered (check the ordering in your file system).
- [ ] Add your structs.
  - Adding references to libraries is not recommended. Libraries do not necessarily implement bincode's native encoding and decoding traits.
  - If you need references to libraries, consider adding a test case for that library, and then referencing that test.
- [ ] Make sure structs derive the following traits:
  - [ ] `serde::Serialize` and `serde::Deserialize`, like normal
  - [ ] `bincode::Encode` and `bincode::Decode`, for the native API
  - [ ] `Debug` and `PartialEq`

```rust
#[derive(Serialize, Deserialize, bincode::Encode, bincode::Decode, Debug, PartialEq)]
pub struct YourStruct {
}
```

- [ ] Use `rand` to be able to generate a random test case for your project.
  - [ ] For strings there is a helper function in `crate`: `gen_string(rng: &mut impl Rng) -> String`
- [ ] Add the following code:

```rust
#[test]
pub fn test() {
    let mut rng = rand::rng();
    for _ in 0..1000 {
        crate::test_same(<your_rand_function>(&mut rng));
    }
}
```

For examples, see the existing cases in `compatibility/src/`.

- [ ] Open a [pull request](https://github.com/bincode-org/bincode/pulls) with the title `Bincode 1 compatibility: <name of your project>`
