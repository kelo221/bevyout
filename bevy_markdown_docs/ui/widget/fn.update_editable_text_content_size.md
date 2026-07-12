[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function update\_editable\_text\_content\_size 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#68-79)

```rust
pub fn update_editable_text_content_size(
    text_input_query: Query<'_, '_, (Ref<'_, EditableText>, Ref<'_, TextFont>, Ref<'_, LineHeight>, Ref<'_, ComputedUiRenderTargetInfo>, &mut ContentSize)>,
    fonts: Res<'_, Assets<Font>>,
    font_cx: ResMut<'_, FontCx>,
    rem_size: Res<'_, RemSize>,
)
```

If `visible_lines` or `visible_width` are `Some`, sets a `ContentSize` that determines:

*   node height as `line_height * visible_lines`, using the resolved font line height.
*   node width as `advance('0') * visible_width`, where `advance('0')` is looked up from font metrics.