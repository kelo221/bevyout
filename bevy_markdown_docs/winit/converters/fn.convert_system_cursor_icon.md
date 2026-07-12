[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_system\_cursor\_icon 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#651)

```rust
pub fn convert_system_cursor_icon(cursor_icon: SystemCursorIcon) -> CursorIcon
```

Converts a Bevy [`SystemCursorIcon`](../../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon") to a [`winit::window::CursorIcon`](https://docs.rs/cursor-icon/1.2.0/x86_64-unknown-linux-gnu/cursor_icon/enum.CursorIcon.html "enum cursor_icon::CursorIcon").