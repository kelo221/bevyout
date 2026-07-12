[bevy](../index.html)::[pbr](index.html)

# Function check\_prepass\_views\_need\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#796-806)

```rust
pub fn check_prepass_views_need_specialization(
    view_key_cache: ResMut<'_, ViewKeyPrepassCache>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
    views: Query<'_, '_, (&ExtractedView, &Msaa, Option<&DepthPrepass>, Option<&NormalPrepass>, Option<&MotionVectorPrepass>)>,
)
```