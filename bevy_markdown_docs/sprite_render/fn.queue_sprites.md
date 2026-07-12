[bevy](../index.html)::[sprite\_render](index.html)

# Function queue\_sprites 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#499-515)

```rust
pub fn queue_sprites(
    view_entities: Local<'_, FixedBitSet>,
    draw_functions: Res<'_, DrawFunctions<Transparent2d>>,
    sprite_pipeline: Res<'_, SpritePipeline>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<SpritePipeline>>,
    pipeline_cache: Res<'_, PipelineCache>,
    extracted_sprites: Res<'_, ExtractedSprites>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<Transparent2d>>,
    cameras: Query<'_, '_, (&RenderVisibleEntities, &ExtractedCamera, &ExtractedView, &Msaa, Option<&Tonemapping>, Option<&DebandDither>)>,
)
```