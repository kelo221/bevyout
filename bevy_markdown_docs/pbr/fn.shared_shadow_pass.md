[bevy](../index.html)::[pbr](index.html)

# Function shared\_shadow\_pass 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2762-2767)

```rust
pub fn shared_shadow_pass<const IS_LATE: bool>(
    world: &World,
    view_light_query: ViewQuery<'_, '_, (Entity, &ShadowView, &ExtractedView, Has<OcclusionCulling>)>,
    shadow_render_phases: Res<'_, ViewBinnedRenderPhases<Shadow>>,
    ctx: RenderContext<'_, '_>,
)
```

Renders the shadow maps that aren’t associated with a specific view.

At present, these consist of the point and spot light shadow maps.