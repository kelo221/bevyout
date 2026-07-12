[bevy](../../index.html)::[ui](../index.html)::[ui\_surface](index.html)

# Function get\_text\_buffer 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/layout/ui_surface.rs.html#313-317)

```rust
pub fn get_text_buffer<'a>(
    needs_buffer: bool,
    ctx: &mut NodeMeasure,
    query: &'a mut Query<'_, '_, &mut ComputedTextBlock>,
) -> Option<&'a mut ComputedTextBlock>
```