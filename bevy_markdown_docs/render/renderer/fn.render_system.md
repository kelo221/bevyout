[bevy](../../index.html)::[render](../index.html)::[renderer](index.html)

# Function render\_system 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#69-72)

```rust
pub fn render_system(
    world: &mut World,
    state: &mut SystemState<Query<'_, '_, (&ViewTarget, &ExtractedCamera)>>,
)
```

The main render system that drives the rendering process. This system runs the [`RenderGraph`](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph") schedule, runs any finalization commands like screenshot captures and GPU readbacks, and calls present on swap chains that need to be presented.