[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function check\_msaa 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#749)

```rust
pub fn check_msaa(
    deferred_views: Query<'_, '_, &mut Msaa, (With<Camera>, With<DeferredPrepass>)>,
)
```