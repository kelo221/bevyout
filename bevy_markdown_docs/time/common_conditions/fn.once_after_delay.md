[bevy](../../index.html)::[time](../index.html)::[common\_conditions](index.html)

# Function once\_after\_delay 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/common_conditions.rs.html#106)

```rust
pub fn once_after_delay(duration: Duration) -> impl FnMut(Res<'_, Time>) + Clone
```

Run condition that is active _once_ after the specified delay, using [`Time`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks at the rate of [`Time::relative_speed`](../../prelude/struct.Time.html#method.relative_speed "method bevy::prelude::Time::relative_speed").

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