[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Type Alias PathStream 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#312)

```rust
pub type PathStream = dyn Stream<Item = PathBuf> + Send + Unpin;
```