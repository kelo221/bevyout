[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function clear\_view\_attachments 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#1173)

```rust
pub fn clear_view_attachments(
    view_target_attachments: ResMut<'_, ViewTargetAttachments>,
)
```

Clears the view target [`OutputColorAttachment`](../texture/struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment")s.