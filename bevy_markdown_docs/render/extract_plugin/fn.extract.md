[bevy](../../index.html)::[render](../index.html)::[extract\_plugin](index.html)

# Function extract 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#115)

```rust
pub fn extract(main_world: &mut World, render_world: &mut World)
```

Executes the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step of the renderer. This updates the render world with the extracted ECS data of the current frame.