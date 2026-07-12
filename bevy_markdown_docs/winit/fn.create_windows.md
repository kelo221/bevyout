[bevy](../index.html)::[winit](index.html)

# Function create\_windows 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/system.rs.html#49-59)

```rust
pub fn create_windows(
    event_loop: &ActiveEventLoop,
    _: <(Commands<'_, '_>, Query<'_, '_, (Entity, &'static mut Window, &'static CursorOptions, Option<&'static RawHandleWrapperHolder>), Added<Window>>, MessageWriter<'_, WindowCreated>, ResMut<'_, WinitActionRequestHandlers>, Res<'_, AccessibilityRequested>, Res<'_, WinitMonitors>) as SystemParam>::Item<'_, '_>,
)
```

Creates new windows on the [`winit`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/index.html "mod winit") backend for each entity with a newly-added [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") component.

If any of these entities are missing required components, those will be added with their default values.