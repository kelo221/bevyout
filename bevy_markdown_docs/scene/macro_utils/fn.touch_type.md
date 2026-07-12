[bevy](../../index.html)::[scene](../index.html)::[macro\_utils](index.html)

# Function touch\_type 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/macro_utils.rs.html#6)

```rust
pub const fn touch_type<T>()
```

This is used by the [`bsn!`](crate::bsn) macro to generate compile-time only references to symbols. Currently this is used to add IDE support for nested type names, as it allows us to pass the input Ident from the input to the output code.