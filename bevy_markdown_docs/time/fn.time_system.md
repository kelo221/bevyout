[bevy](../index.html)::[time](index.html)

# Function time\_system 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#146-154)

```rust
pub fn time_system(
    real_time: ResMut<'_, Time<Real>>,
    virtual_time: ResMut<'_, Time<Virtual>>,
    fixed_time: Res<'_, Time<Fixed>>,
    time: ResMut<'_, Time>,
    update_strategy: Res<'_, TimeUpdateStrategy>,
    time_recv: Option<Res<'_, TimeReceiver>>,
    has_received_time: Local<'_, bool>,
)
```

The system used to update the [`Time`](../prelude/struct.Time.html "struct bevy::prelude::Time") used by app logic. If there is a render world the time is sent from there to this system through channels. Otherwise the time is updated in this system.