[bevy](../index.html)::[pbr](index.html)

# Function main\_transmissive\_pass\_3d 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/node.rs.html#19-32)

```rust
pub fn main_transmissive_pass_3d(
    world: &World,
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ExtractedView, &ScreenSpaceTransmission, &ViewTarget, Option<&ViewTransmissionTexture>, &ViewDepthTexture, Option<&MainPassResolutionOverride>)>,
    transmissive_phases: Res<'_, ViewSortedRenderPhases<Transmissive3d>>,
    ctx: RenderContext<'_, '_>,
)
```