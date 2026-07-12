[bevy](../../index.html)::[pbr](../index.html)::[deferred](index.html)

# Function insert\_deferred\_lighting\_pass\_id\_component 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#381-384)

```rust
pub fn insert_deferred_lighting_pass_id_component(
    commands: Commands<'_, '_>,
    views: Query<'_, '_, Entity, (With<DeferredPrepass>, Without<PbrDeferredLightingDepthId>)>,
)
```