[bevy](../../index.html)::[dev\_tools](../index.html)::[fps\_overlay](index.html)

# Constant FPS\_OVERLAY\_ZINDEX 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#37)

```rust
pub const FPS_OVERLAY_ZINDEX: i32 = _; // 2_147_483_615i32
```

[`GlobalZIndex`](../../prelude/struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex") used to render the fps overlay.

We use a number slightly under `i32::MAX` so you can render on top of it if you really need to.