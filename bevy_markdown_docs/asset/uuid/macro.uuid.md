[bevy](../../index.html)::[asset](../index.html)::[uuid](index.html)

# Macro uuid 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/macros.rs.html#35)

```rust
macro_rules! uuid {
    ($uuid:expr) => { ... };
}
```

Parse [`Uuid`](https://docs.rs/uuid/*/uuid/struct.Uuid.html)s from string literals at compile time.

### Usage

This macro transforms the string literal representation of a [`Uuid`](https://docs.rs/uuid/*/uuid/struct.Uuid.html) into the bytes representation, raising a compilation error if it cannot properly be parsed.

### Examples

Setting a global constant:

```rust
pub const SCHEMA_ATTR_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000000");
pub const SCHEMA_ATTR_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000001");
pub const SCHEMA_ATTR_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000002");
```

Defining a local variable:

```rust
let uuid = uuid!("urn:uuid:F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4");
```

Using a const variable:

```rust
const UUID_STR: &str = "12345678-1234-5678-1234-567812345678";
let UUID = uuid!(UUID_STR);
```