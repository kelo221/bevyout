[bevy](../index.html)::[sprite](index.html)

# Function update\_text2d\_layout 

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#167-193)

```rust
pub fn update_text2d_layout(
    last_logical_viewport_size: Local<'_, Vec2>,
    target_scale_factors: Local<'_, Vec<(f32, RenderLayers)>>,
    reprocess_queue: Local<'_, EntityHashSet>,
    textures: ResMut<'_, Assets<Image>>,
    fonts: Res<'_, Assets<Font>>,
    camera_query: Query<'_, '_, (&Camera, &VisibleEntities, Option<&RenderLayers>)>,
    font_atlas_set: ResMut<'_, FontAtlasSet>,
    text_pipeline: ResMut<'_, TextPipeline>,
    text_query: Query<'_, '_, (Entity, Ref<'_, Text2d>, Option<&RenderLayers>, Ref<'_, TextLayout>, Ref<'_, TextBounds>, &mut TextLayoutInfo, &mut ComputedTextBlock, Ref<'_, FontHinting>)>,
    text_reader: TextReader<'_, '_, Text2d>,
    font_system: ResMut<'_, FontCx>,
    layout_cx: ResMut<'_, LayoutCx>,
    scale_cx: ResMut<'_, ScaleCx>,
    rem_size: Res<'_, RemSize>,
    primary_window: Option<Single<'_, '_, &Window, With<PrimaryWindow>>>,
)
```

Updates the layout and size information whenever the text or style is changed. This information is computed by the [`TextPipeline`](../text/struct.TextPipeline.html "struct bevy::text::TextPipeline") on insertion, then stored.

### World Resources

[`ResMut<Assets<Image>>`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") – This system only adds new [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") assets. It does not modify or observe existing ones.