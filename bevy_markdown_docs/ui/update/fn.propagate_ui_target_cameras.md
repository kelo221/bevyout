[bevy](../../index.html)::[ui](../index.html)::[update](index.html)

# Function propagate\_ui\_target\_cameras 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/update.rs.html#115-122)

```rust
pub fn propagate_ui_target_cameras(
    commands: Commands<'_, '_>,
    default_ui_camera: DefaultUiCamera<'_, '_>,
    ui_scale: Res<'_, UiScale>,
    camera_query: Query<'_, '_, &Camera>,
    target_camera_query: Query<'_, '_, &UiTargetCamera>,
    ui_root_nodes: UiRootNodes<'_, '_>,
)
```