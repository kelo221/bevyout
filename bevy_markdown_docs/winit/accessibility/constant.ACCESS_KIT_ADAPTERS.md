[bevy](../../index.html)::[winit](../index.html)::[accessibility](index.html)

# Constant ACCESS\_KIT\_ADAPTERS 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#23-27)

```rust
pub const ACCESS_KIT_ADAPTERS: LocalKey<RefCell<AccessKitAdapters>>;
```

Temporary storage of access kit adapter data to replace usage of `!Send` resources. This will be replaced with proper storage of `!Send` data after issue #17667 is complete.