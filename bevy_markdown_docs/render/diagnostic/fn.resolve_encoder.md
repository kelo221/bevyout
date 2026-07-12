[bevy](../../index.html)::[render](../index.html)::[diagnostic](index.html)

# Function resolve\_encoder 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#109-113)

```rust
pub fn resolve_encoder(
    recorder: ResMut<'_, DiagnosticsRecorder>,
    render_device: Res<'_, RenderDevice>,
    pending_buffers: ResMut<'_, PendingCommandBuffers>,
)
```

Resolves the encoder used for diagnostic recording