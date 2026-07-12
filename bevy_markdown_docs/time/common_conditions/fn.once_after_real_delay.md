[bevy](../../index.html)::[time](../index.html)::[common\_conditions](index.html)

# Function once\_after\_real\_delay 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/common_conditions.rs.html#136)

```rust
pub fn once_after_real_delay(
    duration: Duration,
) -> impl FnMut(Res<'_, Time<Real>>) + Clone
```

Run condition that is active _once_ after the specified delay, using [`Time<Real>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks are not scaled.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            tick.run_if(once_after_delay(Duration::from_secs(1))),
        )
    .run();
}
fn tick() {
    // ran once, after a second
}
```