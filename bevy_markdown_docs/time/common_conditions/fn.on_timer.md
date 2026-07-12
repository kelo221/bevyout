[bevy](../../index.html)::[time](../index.html)::[common\_conditions](index.html)

# Function on\_timer 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/common_conditions.rs.html#36)

```rust
pub fn on_timer(duration: Duration) -> impl FnMut(Res<'_, Time>) + Clone
```

Run condition that is active on a regular time interval, using [`Time`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks at the rate of [`Time::relative_speed`](../../prelude/struct.Time.html#method.relative_speed "method bevy::prelude::Time::relative_speed").

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            tick.run_if(on_timer(Duration::from_secs(1))),
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

examples/diagnostics/enabling\_disabling\_diagnostic.rs ([line 20](../../../src/enabling_disabling_diagnostic/enabling_disabling_diagnostic.rs.html#20))

```rust
11fn main() {
12    App::new()
13        .add_plugins((
14            DefaultPlugins,
15            FrameTimeDiagnosticsPlugin::default(),
16            LogDiagnosticsPlugin::default(),
17        ))
18        .add_systems(
19            Update,
20            toggle.run_if(on_timer(Duration::from_secs_f32(10.0))),
21        )
22        .run();
23}
```

Hide additional examples

examples/ecs/observer\_propagation.rs ([line 14](../../../src/observer_propagation/observer_propagation.rs.html#14))

```rust
8fn main() {
9    App::new()
10        .add_plugins((MinimalPlugins, LogPlugin::default()))
11        .add_systems(Startup, setup)
12        .add_systems(
13            Update,
14            attack_armor.run_if(on_timer(Duration::from_millis(200))),
15        )
16        // Add a global observer that will emit a line whenever an attack hits an entity.
17        .add_observer(attack_hits)
18        .run();
19}
```

examples/remote/app\_under\_test.rs ([line 33](../../../src/app_under_test/app_under_test.rs.html#33))

```rust
21fn main() {
22    App::new()
23        .add_plugins(DefaultPlugins)
24        // To make the app available for integration testing, we add these
25        // remote plugins to expose API’s for a testing framework to call.
26        .add_plugins(RemotePlugin::default())
27        .add_plugins(RemoteHttpPlugin::default())
28        .insert_resource(SeededRng(ChaCha8Rng::seed_from_u64(19878367467712)))
29        .add_systems(Startup, setup)
30        .add_systems(
31            Update,
32            (
33                move_button.run_if(on_timer(std::time::Duration::from_secs(5))),
34                log_button_position,
35            ),
36        )
37        .run();
38}
```