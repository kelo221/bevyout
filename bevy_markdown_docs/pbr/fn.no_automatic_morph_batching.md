[bevy](../index.html)::[pbr](index.html)

# Function no\_automatic\_morph\_batching 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#433-437)

```rust
pub fn no_automatic_morph_batching(
    commands: Commands<'_, '_>,
    query: Query<'_, '_, Entity, (With<MeshMorphWeights>, Without<NoAutomaticBatching>)>,
    render_device: Res<'_, RenderDevice>,
)
```