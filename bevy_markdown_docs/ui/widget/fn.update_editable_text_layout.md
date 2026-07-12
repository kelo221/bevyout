[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function update\_editable\_text\_layout 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#259-279)

```rust
pub fn update_editable_text_layout(
    font_cx: ResMut<'_, FontCx>,
    layout_cx: ResMut<'_, LayoutCx>,
    scale_cx: ResMut<'_, ScaleCx>,
    font_atlas_set: ResMut<'_, FontAtlasSet>,
    textures: ResMut<'_, Assets<Image>>,
    input_field_query: Query<'_, '_, (Entity, &TextFont, Ref<'_, FontHinting>, Ref<'_, ComputedUiRenderTargetInfo>, &mut EditableText, &mut TextLayoutInfo, Ref<'_, ComputedNode>, &mut EditableTextGeneration)>,
    rem_size: Res<'_, RemSize>,
    input_focus: Option<Res<'_, InputFocus>>,
    cursor_timer: Local<'_, Duration>,
    time: Res<'_, Time<Real>>,
)
```

Refreshes the [`EditableText`](../../text/struct.EditableText.html "struct bevy::text::EditableText")’s layout if stale and then writes it it to [`TextLayoutInfo`](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo") for rendering and picking. Adds required glyphs to the texture atlas