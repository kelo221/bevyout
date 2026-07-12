[bevy](../index.html)::[pbr](index.html)

# Function per\_view\_shadow\_pass 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2784-2790)

```rust
pub fn per_view_shadow_pass<const IS_LATE: bool>(
    world: &World,
    view: ViewQuery<'_, '_, &ViewLightEntities>,
    view_light_query: Query<'_, '_, (&ShadowView, &ExtractedView, Has<OcclusionCulling>)>,
    shadow_render_phases: Res<'_, ViewBinnedRenderPhases<Shadow>>,
    ctx: RenderContext<'_, '_>,
)
```

Renders the shadow maps that are associated with a specific view.

At present, these consist of the directional light shadows.