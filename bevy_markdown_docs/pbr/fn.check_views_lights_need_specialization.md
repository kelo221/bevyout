[bevy](../index.html)::[pbr](index.html)

# Function check\_views\_lights\_need\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2212-2217)

```rust
pub fn check_views_lights_need_specialization(
    view_light_entities: Query<'_, '_, (&LightEntity, &ExtractedView)>,
    shadow_render_phases: Res<'_, ViewBinnedRenderPhases<Shadow>>,
    light_key_cache: ResMut<'_, LightKeyCache>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
)
```