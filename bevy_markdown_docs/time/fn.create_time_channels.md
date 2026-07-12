[bevy](../index.html)::[time](index.html)

# Function create\_time\_channels 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#137)

```rust
pub fn create_time_channels() -> (TimeSender, TimeReceiver)
```

Available on **crate feature `std`** only.

Creates channels used for sending time between the render world and the main world.