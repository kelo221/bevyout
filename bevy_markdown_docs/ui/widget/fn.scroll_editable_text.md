[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function scroll\_editable\_text 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#503-514)

```rust
pub fn scroll_editable_text(
    input_focus: Option<Res<'_, InputFocus>>,
    previous_focus: Local<'_, Option<Entity>>,
    query: Query<'_, '_, (Entity, Ref<'_, EditableText>, Ref<'_, EditableTextGeneration>, &mut TextScroll, &ComputedNode, &TextLayoutInfo)>,
)
```

Scroll editable text to keep cursor in view after edits.