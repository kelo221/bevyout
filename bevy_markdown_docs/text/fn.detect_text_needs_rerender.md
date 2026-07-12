[bevy](../index.html)::[text](index.html)

# Function detect\_text\_needs\_rerender 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1223-1259)

```rust
pub fn detect_text_needs_rerender(
    changed_roots: Query<'_, '_, Entity, (Or<(Changed<TextFont>, Changed<TextLayout>, Changed<LineHeight>, Changed<LetterSpacing>, Changed<Children>)>, With<TextFont>, With<TextLayout>)>,
    changed_spans: Query<'_, '_, (Entity, Option<&ChildOf>, Has<TextLayout>), (Or<(Changed<TextSpan>, Changed<TextFont>, Changed<LineHeight>, Changed<LetterSpacing>, Changed<Children>, Changed<ChildOf>, Added<TextLayout>)>, With<TextSpan>, With<TextFont>)>,
    computed: Query<'_, '_, (Option<&ChildOf>, Option<&mut ComputedTextBlock>, Has<TextSpan>)>,
)
```

System that detects changes to text blocks and sets `ComputedTextBlock::should_rerender`.

Does not check root text components (e.g. `Text`/`Text2d`) for changes. Their systems must handle change detection.