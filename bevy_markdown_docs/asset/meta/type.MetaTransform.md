[bevy](../../index.html)::[asset](../index.html)::[meta](index.html)

# Type Alias MetaTransform 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#20)

```rust
pub type MetaTransform = Box<dyn Fn(&mut (dyn AssetMetaDyn + 'static)) + Send + Sync>;
```

## Aliased Type

```rust
pub struct MetaTransform(/* private fields */);
```