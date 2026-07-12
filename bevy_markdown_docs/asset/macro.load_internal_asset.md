[bevy](../index.html)::[asset](index.html)

# Macro load\_internal\_asset 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#378)

```rust
macro_rules! load_internal_asset {
    ($app: ident, $handle: expr, $path_str: expr, $loader: expr) => { ... };
    ($app: ident, $handle: ident, $path_str: expr, $loader: expr $(, $param:expr)+) => { ... };
}
```

Loads an “internal” asset by embedding the string stored in the given `path_str` and associates it with the given handle.