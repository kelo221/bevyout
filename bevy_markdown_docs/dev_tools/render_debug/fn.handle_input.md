[bevy](../../index.html)::[dev\_tools](../index.html)::[render\_debug](index.html)

# Function handle\_input 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#103-106)

```rust
pub fn handle_input(
    keyboard: Res<'_, ButtonInput<KeyCode>>,
    events: MessageWriter<'_, RenderDebugOverlayEvent>,
)
```

Automatically attach keybinds to make render debug overlays available to users without code changes when the feature is enabled.