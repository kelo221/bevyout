[bevy](../index.html)::[winit](index.html)

# Constant WINIT\_WINDOWS 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#53-57)

```rust
pub const WINIT_WINDOWS: LocalKey<RefCell<WinitWindows>>;
```

Temporary storage of WinitWindows data to replace usage of `!Send` resources. This will be replaced with proper storage of `!Send` data after issue #17667 is complete.