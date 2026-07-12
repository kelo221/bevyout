[bevy](../../index.html)::[scene](../index.html)::[macro\_utils](index.html)

# Type Alias PathResolveHelper 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/macro_utils.rs.html#149)

```rust
pub type PathResolveHelper<T> = T;
```

This is used by the [`bsn!`](crate::bsn) derive to work around [this Rust limitation](https://github.com/rust-lang/rust/issues/86935). A fix is implemented and on track for stabilization. If it is ever implemented, we can remove this.