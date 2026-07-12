[bevy](../index.html)::[sprite\_render](index.html)

# Function check\_views\_need\_specialization 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#126-137)

```rust
pub fn check_views_need_specialization(
    view_key_cache: ResMut<'_, ViewKeyCache>,
    dirty_specializations: ResMut<'_, DirtySpecializations>,
    cameras: Query<'_, '_, (&MainEntity, &ExtractedView, &ExtractedCamera, &Msaa, Option<&Tonemapping>, Option<&DebandDither>)>,
)
```