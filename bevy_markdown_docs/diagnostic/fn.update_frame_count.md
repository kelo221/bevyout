[bevy](../index.html)::[diagnostic](index.html)

# Function update\_frame\_count 

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/frame_count.rs.html#37)

```rust
pub fn update_frame_count(frame_count: ResMut<'_, FrameCount>)
```

A system used to increment [`FrameCount`](struct.FrameCount.html "struct bevy::diagnostic::FrameCount") with wrapping addition.

See [`FrameCount`](struct.FrameCount.html "struct bevy::diagnostic::FrameCount") for more details.