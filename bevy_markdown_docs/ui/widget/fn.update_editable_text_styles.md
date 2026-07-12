[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function update\_editable\_text\_styles 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#163-173)

```rust
pub fn update_editable_text_styles(
    fonts: Res<'_, Assets<Font>>,
    editable_text_query: Query<'_, '_, (&mut EditableText, Ref<'_, TextFont>, Ref<'_, LineHeight>, Ref<'_, ComputedUiRenderTargetInfo>, Ref<'_, TextLayout>)>,
    rem_size: Res<'_, RemSize>,
)
```

Syncs each [`EditableText`](../../text/struct.EditableText.html "struct bevy::text::EditableText") entity’s [`PlainEditor`](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/editing/editor/struct.PlainEditor.html "struct parley::editing::editor::PlainEditor") style properties to match its [`TextFont`](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), [`LineHeight`](../../text/enum.LineHeight.html "enum bevy::text::LineHeight"), and [`TextLayout`](../../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout") components.