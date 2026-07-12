[bevy](../index.html)::[reflect](index.html)

# Macro hash\_error 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#197)

```rust
macro_rules! hash_error {
    ( $key:expr ) => { ... };
}
```

Used to produce an error message when an attempt is made to hash a [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value that does not support hashing.