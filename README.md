# 🪪 `idtype`

[![Crates.io](https://img.shields.io/crates/v/idtype)](https://crates.io/crates/idtype)
[![docs.rs](https://img.shields.io/docsrs/idtype)][documentation]

`idtype` provides convenient macros to use the [`newtype`] pattern to generate
types for identifiers.

## Usage

The crate provides three macros to generate identifier types:

- `id!` can be used for numerical ids
- `name!` can be used for unique usernames or string-based ids
- `secret!` can be used for sensitive strings like passwords

The generated types implement a few common traits so that they can be easily
converted from and to their inner types.

```rust
use idtype::id;

id!(
    /// A numeric id
    Id
);

fn main() {
    let id: Id = 42.into();
}
```

Check out the crate's [documentation] for more information.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`newtype`]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
[documentation]: https://docs.rs/idtype
