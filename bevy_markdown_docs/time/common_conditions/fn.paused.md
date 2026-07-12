[bevy](../../index.html)::[time](../index.html)::[common\_conditions](index.html)

# Function paused 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/common_conditions.rs.html#234)

```rust
pub fn paused(time: Res<'_, Time<Virtual>>) -> bool
```

Run condition that is active when the [`Time<Virtual>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") clock is paused. Use [`bevy_ecs::schedule::common_conditions::not`](../../prelude/fn.not.html "fn bevy::prelude::not") to make it active when it’s not paused.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            (
                is_paused.run_if(paused),
                not_paused.run_if(not(paused)),
            )
        )
    .run();
}
fn is_paused() {
    // ran when time is paused
}

fn not_paused() {
    // ran when time is not paused
}
```