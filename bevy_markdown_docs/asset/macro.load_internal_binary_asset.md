[bevy](../index.html)::[asset](index.html)

# Macro load\_internal\_binary\_asset 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#407)

```rust
macro_rules! load_internal_binary_asset {
    ($app: ident, $handle: expr, $path_str: expr, $loader: expr) => { ... };
}
```

Loads an “internal” binary asset by embedding the bytes stored in the given `path_str` and associates it with the given handle.