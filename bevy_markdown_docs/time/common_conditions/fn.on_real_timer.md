[bevy](../../index.html)::[time](../index.html)::[common\_conditions](index.html)

# Function on\_real\_timer 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/common_conditions.rs.html#76)

```rust
pub fn on_real_timer(
    duration: Duration,
) -> impl FnMut(Res<'_, Time<Real>>) + Clone
```

Run condition that is active on a regular time interval, using [`Time<Real>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks are not scaled.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            tick.run_if(on_real_timer(Duration::from_secs(1))),
        )
        .run();
}
fn tick() {
    // ran once a second
}
```

Note that this does **not** guarantee that systems will run at exactly the specified interval. If delta time is larger than the specified `duration` then the system will only run once even though the timer may have completed multiple times. This condition should only be used with large time durations (relative to delta time).

For more accurate timers, use the [`Timer`](../../prelude/struct.Timer.html "struct bevy::prelude::Timer") class directly (see [`Timer::times_finished_this_tick`](../../prelude/struct.Timer.html#method.times_finished_this_tick "method bevy::prelude::Timer::times_finished_this_tick") to address the problem mentioned above), or use fixed timesteps that allow systems to run multiple times per frame.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/time/virtual\_time.rs ([line 28](../../../src/virtual_time/virtual_time.rs.html#28))

```rust
11fn main() {
12    App::new()
13        .add_plugins(DefaultPlugins)
14        .add_systems(Startup, setup)
15        .add_systems(
16            Update,
17            (
18                move_virtual_time_sprites,
19                move_real_time_sprites,
20                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
21                change_time_speed::<1>.run_if(input_just_pressed(KeyCode::ArrowUp)),
22                change_time_speed::<-1>.run_if(input_just_pressed(KeyCode::ArrowDown)),
23                (update_virtual_time_info_text, update_real_time_info_text)
24                    // update the texts on a timer to make them more readable.
25                    // `on_timer` run condition uses `Virtual` time meaning it's scaled
26                    // and would result in the UI updating at different intervals based
27                    // on `Time<Virtual>::relative_speed` and `Time<Virtual>::is_paused()`
28                    .run_if(on_real_timer(Duration::from_millis(250))),
29            ),
30        )
31        .run();
32}
```