# nova

[![Documentation](https://docs.rs/nova/badge.svg)](https://docs.rs/nova)

Create newtypes with great convenience.

All types generated by the following macros implement `Debug`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd` 
and `Hash`. For `Copy` types, the newtype also implements `Copy`.

## Usage

```toml
[dependencies]
nova = "0.4"
```

### Example

```rust
use nova::newtype;

#[newtype(serde, borrow = "str")]
pub type Meow = String;

#[newtype(new, copy)]
pub(crate) type SpecialUuid = uuid::Uuid;

fn example() {
    let meow = Meow("this is a string".to_string());
    let special_uuid = SpecialUuid::from(uuid::Uuid::new_v4());

    // Get inner:
    let inner = special_uuid.into_inner();
}

```

## Supported attributes

### Crate compatibility attributes

- **serde**: enables support for the `serde` attribute to derive `Serialize` and `Deserialize` for newtypes.
- **sqlx**: enables support for the `sqlx` attribute to derive `sqlx::Type` for newtypes.
- **async_graphql**: enables support for the `async_graphql` attribute to implement `Scalar` for newtypes.

### Generation attributes

- **copy**: derives `Copy` on the newtype.
- **opaque**: disables generating a `Deref` and `into_inner` functions to create an opaque type.
- **borrow = "&lt;type&gt;"**: sets the type to be used for the `Deref` implementation, if needed.
- **new**: create default construction `new` function and `From` implementation.
- **derive(...)**: replace the default derives for the newtype with the provided list. Same syntax as the normal `#[derive(...)]` attribute.

## License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
