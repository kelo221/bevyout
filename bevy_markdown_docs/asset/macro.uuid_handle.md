[bevy](../index.html)::[asset](index.html)

# Macro uuid\_handle 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#706)

```rust
macro_rules! uuid_handle {
    ($uuid:expr) => { ... };
}
```

Creates a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") from a string literal containing a UUID.

## Examples

```rust
const IMAGE: Handle<Image> = uuid_handle!("1347c9b7-c46a-48e7-b7b8-023a354b7cac");
```