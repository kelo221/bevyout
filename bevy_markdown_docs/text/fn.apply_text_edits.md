[bevy](../index.html)::[text](index.html)

# Function apply\_text\_edits 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/editing.rs.html#294-305)

```rust
pub fn apply_text_edits(
    query: Query<'_, '_, (Entity, &mut EditableText, Option<&EditableTextFilter>, &EditableTextGeneration)>,
    font_context: ResMut<'_, FontCx>,
    layout_context: ResMut<'_, LayoutCx>,
    clipboard: ResMut<'_, Clipboard>,
    commands: Commands<'_, '_>,
)
```

Applies pending text edit actions to all [`EditableText`](struct.EditableText.html "struct bevy::text::EditableText") widgets.