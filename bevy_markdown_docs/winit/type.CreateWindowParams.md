[bevy](../index.html)::[winit](index.html)

# Type Alias CreateWindowParams 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#228)

```rust
pub type CreateWindowParams<'w, 's> = (Commands<'w, 's>, Query<'w, 's, (Entity, &'static mut Window, &'static CursorOptions, Option<&'static RawHandleWrapperHolder>), Added<Window>>, MessageWriter<'w, WindowCreated>, ResMut<'w, WinitActionRequestHandlers>, Res<'w, AccessibilityRequested>, Res<'w, WinitMonitors>);
```

The parameters of the [`create_windows`](fn.create_windows.html "fn bevy::winit::create_windows") system.