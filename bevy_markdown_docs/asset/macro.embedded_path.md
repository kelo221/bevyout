[bevy](../index.html)::[asset](index.html)

# Macro embedded\_path 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#220)

```rust
macro_rules! embedded_path {
    ($path_str: expr) => { ... };
    ($source_path: expr, $path_str: expr) => { ... };
}
```

Returns the [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") for a given `embedded` asset. This is used internally by [`embedded_asset`](macro.embedded_asset.html "macro bevy::asset::embedded_asset") and can be used to get a [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") that matches the [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath") used by that asset.