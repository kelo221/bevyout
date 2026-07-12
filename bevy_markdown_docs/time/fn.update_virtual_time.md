[bevy](../index.html)::[time](index.html)

# Function update\_virtual\_time 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#280)

```rust
pub fn update_virtual_time(
    current: &mut Time,
    virt: &mut Time<Virtual>,
    real: &Time<Real>,
)
```

Advances [`Time<Virtual>`](../prelude/struct.Time.html "struct bevy::prelude::Time") and [`Time`](../prelude/struct.Time.html "struct bevy::prelude::Time") based on the elapsed [`Time<Real>`](../prelude/struct.Time.html "struct bevy::prelude::Time").

The virtual time will be advanced up to the provided [`Time::max_delta`](../prelude/struct.Time.html#method.max_delta "method bevy::prelude::Time::max_delta").