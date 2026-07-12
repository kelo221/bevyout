[bevy](../index.html)::[prelude](index.html)

# Struct Time 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#192)

```rust
pub struct Time<T = ()>where
    T: Default,{ /* private fields */ }
```

A generic clock resource that tracks how much it has advanced since its previous update and since its creation.

Multiple instances of this resource are inserted automatically by [`TimePlugin`](../time/struct.TimePlugin.html "struct bevy::time::TimePlugin"):

*   [`Time<Real>`](struct.Real.html "struct bevy::prelude::Real") tracks real wall-clock time elapsed.
*   [`Time<Virtual>`](struct.Virtual.html "struct bevy::prelude::Virtual") tracks virtual game time that may be paused or scaled.
*   [`Time<Fixed>`](struct.Fixed.html "struct bevy::prelude::Fixed") tracks fixed timesteps based on virtual time.
*   [`Time`](struct.Time.html "struct bevy::prelude::Time") is a generic clock that corresponds to “current” or “default” time for systems. It contains [`Time<Virtual>`](struct.Virtual.html "struct bevy::prelude::Virtual") except inside the [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain") schedule when it contains [`Time<Fixed>`](struct.Fixed.html "struct bevy::prelude::Fixed").

The time elapsed since the previous time this clock was advanced is saved as [`delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") and the total amount of time the clock has advanced is saved as [`elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed"). Both are represented as exact [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration") values with fixed nanosecond precision. The clock does not support time moving backwards, but it can be updated with [`Duration::ZERO`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html#associatedconstant.ZERO "associated constant core::time::Duration::ZERO") which will set [`delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") to zero.

These values are also available in seconds as `f32` via [`delta_secs()`](struct.Time.html#method.delta_secs "method bevy::prelude::Time::delta_secs") and [`elapsed_secs()`](struct.Time.html#method.elapsed_secs "method bevy::prelude::Time::elapsed_secs"), and also in seconds as `f64` via [`delta_secs_f64()`](struct.Time.html#method.delta_secs_f64 "method bevy::prelude::Time::delta_secs_f64") and [`elapsed_secs_f64()`](struct.Time.html#method.elapsed_secs_f64 "method bevy::prelude::Time::elapsed_secs_f64").

Since [`elapsed_secs()`](struct.Time.html#method.elapsed_secs "method bevy::prelude::Time::elapsed_secs") will grow constantly and is `f32`, it will exhibit gradual precision loss. For applications that require an `f32` value but suffer from gradual precision loss there is [`elapsed_secs_wrapped()`](struct.Time.html#method.elapsed_secs_wrapped "method bevy::prelude::Time::elapsed_secs_wrapped") available. The same wrapped value is also available as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration") and `f64` for consistency. The wrap period is by default 1 hour, and can be set by [`set_wrap_period()`](struct.Time.html#method.set_wrap_period "method bevy::prelude::Time::set_wrap_period").

## Accessing clocks

By default, any systems requiring current [`delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") or [`elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") should use `Res<Time>` to access the default time configured for the program. By default, this refers to [`Time<Virtual>`](struct.Virtual.html "struct bevy::prelude::Virtual") except during the [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain") schedule when it refers to [`Time<Fixed>`](struct.Fixed.html "struct bevy::prelude::Fixed"). This ensures your system can be used either in [`Update`](struct.Update.html "struct bevy::prelude::Update") or [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") schedule depending on what is needed.

```rust
fn ambivalent_system(time: Res<Time>) {
    println!("this how I see time: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
}
```

If your system needs to react based on real time (wall clock time), like for user interfaces, it should use `Res<Time<Real>>`. The [`delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") and [`elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") values will always correspond to real time and will not be affected by pause, time scaling or other tweaks.

```rust
fn real_time_system(time: Res<Time<Real>>) {
    println!("this will always be real time: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
}
```

If your system specifically needs to access fixed timestep clock, even when placed in `Update` schedule, you should use `Res<Time<Fixed>>`. The [`delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") and [`elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") values will correspond to the latest fixed timestep that has been run.

```rust
fn fixed_time_system(time: Res<Time<Fixed>>) {
    println!("this will always be the last executed fixed timestep: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
}
```

Finally, if your system specifically needs to know the current virtual game time, even if placed inside [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate"), for example to know if the game is [`was_paused()`](struct.Time.html#method.was_paused "method bevy::prelude::Time::was_paused") or to use [`effective_speed()`](struct.Time.html#method.effective_speed "method bevy::prelude::Time::effective_speed"), you can use `Res<Time<Virtual>>`. However, if the system is placed in [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate"), extra care must be used because your system might be run multiple times with the same [`delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") and [`elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") values as the virtual game time has not changed between the iterations.

```rust
fn fixed_time_system(time: Res<Time<Virtual>>) {
    println!("this will be virtual time for this update: delta {:?}, elapsed {:?}", time.delta(), time.elapsed());
    println!("also the relative speed of the game is now {}", time.effective_speed());
}
```

If you need to change the settings for any of the clocks, for example to [`pause()`](struct.Time.html#method.pause "method bevy::prelude::Time::pause") the game, you should use `ResMut<Time<Virtual>>`.

```rust
#[derive(Message)]
struct Pause(bool);

fn pause_system(mut time: ResMut<Time<Virtual>>, mut pause_reader: MessageReader<Pause>) {
    for pause in pause_reader.read() {
        if pause.0 {
            time.pause();
        } else {
            time.unpause();
        }
    }
}
```

## Adding custom clocks

New custom clocks can be created by creating your own struct as a context and passing it to [`new_with()`](struct.Time.html#method.new_with "associated function bevy::prelude::Time::new_with"). These clocks can be inserted as resources as normal and then accessed by systems. You can use the [`advance_by()`](struct.Time.html#method.advance_by "method bevy::prelude::Time::advance_by") or [`advance_to()`](struct.Time.html#method.advance_to "method bevy::prelude::Time::advance_to") methods to move the clock forwards based on your own logic.

If you want to add methods for your time instance and they require access to both your context and the generic time part, it’s probably simplest to add a custom trait for them and implement it for `Time<Custom>`.

Your context struct will need to implement the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait because [`Time`](struct.Time.html "struct bevy::prelude::Time") structures support reflection. It also makes initialization trivial by being able to call `app.init_resource::<Time<Custom>>()`.

You can also replace the “generic” `Time` clock resource if the “default” time for your game should not be the default virtual time provided. You can get a “generic” snapshot of your clock by calling `as_generic()` and then overwrite the [`Time`](struct.Time.html "struct bevy::prelude::Time") resource with it. The default systems added by [`TimePlugin`](../time/struct.TimePlugin.html "struct bevy::time::TimePlugin") will overwrite the [`Time`](struct.Time.html "struct bevy::prelude::Time") clock during [`First`](struct.First.html "struct bevy::prelude::First") and [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") schedules.

```rust
#[derive(Debug)]
struct Custom {
    last_external_time: Instant,
}

impl Default for Custom {
    fn default() -> Self {
        Self {
            last_external_time: Instant::now(),
        }
    }
}

trait CustomTime {
    fn update_from_external(&mut self, instant: Instant);
}

impl CustomTime for Time<Custom> {
    fn update_from_external(&mut self, instant: Instant) {
         let delta = instant - self.context().last_external_time;
         self.advance_by(delta);
         self.context_mut().last_external_time = instant;
    }
}
```

## Implementations

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#74)

### impl [Time](struct.Time.html "struct bevy::prelude::Time")<[Fixed](struct.Fixed.html "struct bevy::prelude::Fixed")\>

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#83)

#### pub fn [from\_duration](#method.from_duration)(timestep: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Time](struct.Time.html "struct bevy::prelude::Time")<[Fixed](struct.Fixed.html "struct bevy::prelude::Fixed")\>

Return new fixed time clock with given timestep as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

##### Panics

Panics if `timestep` is zero.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/time/time.rs ([line 115](../../src/time/time.rs.html#115))

```rust
111fn main() {
112    App::new()
113        .add_plugins(MinimalPlugins)
114        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
115        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
116        .add_systems(PreUpdate, print_real_time)
117        .add_systems(FixedUpdate, print_fixed_time)
118        .add_systems(Update, print_time)
119        .set_runner(runner)
120        .run();
121}
```

Hide additional examples

examples/stress\_tests/bevymark\_3d.rs ([lines 134-136](../../src/bevymark_3d/bevymark_3d.rs.html#134-136))

```rust
96fn main() {
97    // `from_env` panics on the web
98    #[cfg(not(target_arch = "wasm32"))]
99    let args: Args = argh::from_env();
100    #[cfg(target_arch = "wasm32")]
101    let args = Args::from_args(&[], &[]).unwrap();
102
103    App::new()
104        .add_plugins((
105            DefaultPlugins.set(WindowPlugin {
106                primary_window: Some(Window {
107                    title: "BevyMark 3D".into(),
108                    resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
109                    present_mode: PresentMode::AutoNoVsync,
110                    ..default()
111                }),
112                ..default()
113            }),
114            FrameTimeDiagnosticsPlugin::default(),
115            LogDiagnosticsPlugin::default(),
116        ))
117        .insert_resource(WinitSettings::continuous())
118        .insert_resource(args)
119        .insert_resource(BevyCounter {
120            count: 0,
121            color: Color::WHITE,
122        })
123        .add_systems(Startup, setup)
124        .add_systems(FixedUpdate, scheduled_spawner)
125        .add_systems(
126            Update,
127            (
128                mouse_handler,
129                movement_system,
130                collision_system,
131                counter_system,
132            ),
133        )
134        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
135            FIXED_TIMESTEP,
136        )))
137        .run();
138}
```

examples/stress\_tests/bevymark.rs ([lines 166-168](../../src/bevymark/bevymark.rs.html#166-168))

```rust
127fn main() {
128    // `from_env` panics on the web
129    #[cfg(not(target_arch = "wasm32"))]
130    let args: Args = argh::from_env();
131    #[cfg(target_arch = "wasm32")]
132    let args = Args::from_args(&[], &[]).unwrap();
133
134    App::new()
135        .add_plugins((
136            DefaultPlugins.set(WindowPlugin {
137                primary_window: Some(Window {
138                    title: "BevyMark".into(),
139                    resolution: WindowResolution::new(1920, 1080).with_scale_factor_override(1.0),
140                    present_mode: PresentMode::AutoNoVsync,
141                    ..default()
142                }),
143                ..default()
144            }),
145            FrameTimeDiagnosticsPlugin::default(),
146            LogDiagnosticsPlugin::default(),
147        ))
148        .insert_resource(StaticTransformOptimizations::Disabled)
149        .insert_resource(WinitSettings::continuous())
150        .insert_resource(args)
151        .insert_resource(BevyCounter {
152            count: 0,
153            color: Color::WHITE,
154        })
155        .add_systems(Startup, setup)
156        .add_systems(FixedUpdate, scheduled_spawner)
157        .add_systems(
158            Update,
159            (
160                mouse_handler,
161                movement_system,
162                collision_system,
163                counter_system,
164            ),
165        )
166        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
167            FIXED_TIMESTEP,
168        )))
169        .run();
170}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#94)

#### pub fn [from\_seconds](#method.from_seconds)(seconds: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Time](struct.Time.html "struct bevy::prelude::Time")<[Fixed](struct.Fixed.html "struct bevy::prelude::Fixed")\>

Return new fixed time clock with given timestep seconds as `f64`

##### Panics

Panics if `seconds` is zero, negative or not finite.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/async\_tasks/external\_source\_external\_thread.rs ([line 16](../../src/external_source_external_thread/external_source_external_thread.rs.html#16))

```rust
9fn main() {
10    App::new()
11        .add_message::<StreamMessage>()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .add_systems(Update, (spawn_text, move_text))
15        .add_systems(FixedUpdate, read_stream)
16        .insert_resource(Time::<Fixed>::from_seconds(0.5))
17        .run();
18}
```

Hide additional examples

examples/ecs/fixed\_timestep.rs ([line 13](../../src/fixed_timestep/fixed_timestep.rs.html#13))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        // this system will run once every update (it should match your screen's refresh rate)
9        .add_systems(Update, frame_update)
10        // add our system to the fixed timestep schedule
11        .add_systems(FixedUpdate, fixed_update)
12        // configure our fixed timestep schedule to run twice a second
13        .insert_resource(Time::<Fixed>::from_seconds(0.5))
14        .run();
15}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#105)

#### pub fn [from\_hz](#method.from_hz)(hz: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Time](struct.Time.html "struct bevy::prelude::Time")<[Fixed](struct.Fixed.html "struct bevy::prelude::Fixed")\>

Return new fixed time clock with given timestep frequency in Hertz (1/seconds)

##### Panics

Panics if `hz` is zero, negative or not finite.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/2d/rotation.rs ([line 10](../../src/rotation/rotation.rs.html#10))

```rust
7fn main() {
8    App::new()
9        .add_plugins(DefaultPlugins)
10        .insert_resource(Time::<Fixed>::from_hz(60.0))
11        .add_systems(Startup, setup)
12        .add_systems(
13            FixedUpdate,
14            (
15                player_movement_system,
16                snap_to_player_system,
17                rotate_to_player_system,
18            ),
19        )
20        .run();
21}
```

Hide additional examples

examples/2d/cpu\_draw.rs ([line 25](../../src/cpu_draw/cpu_draw.rs.html#25))

```rust
18fn main() {
19    App::new()
20        .add_plugins(DefaultPlugins)
21        // In this example, we will use a fixed timestep to draw a pattern on the screen
22        // one pixel at a time, so the pattern will gradually emerge over time, and
23        // the speed at which it appears is not tied to the framerate.
24        // Let's make the fixed update very fast, so it doesn't take too long. :)
25        .insert_resource(Time::<Fixed>::from_hz(1024.0))
26        .add_systems(Startup, setup)
27        .add_systems(FixedUpdate, draw)
28        .run();
29}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#114)

#### pub fn [timestep](#method.timestep)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns the amount of virtual time that must pass before the fixed timestep schedule is run again.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/ecs/iter\_combinations.rs ([line 81](../../src/iter_combinations/iter_combinations.rs.html#81))

```rust
38fn generate_bodies(
39    time: Res<Time<Fixed>>,
40    mut commands: Commands,
41    mut meshes: ResMut<Assets<Mesh>>,
42    mut materials: ResMut<Assets<StandardMaterial>>,
43) {
44    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());
45
46    let color_range = 0.5..1.0;
47    let vel_range = -0.5..0.5;
48
49    // We're seeding the PRNG here to make this example deterministic for testing purposes.
50    // This isn't strictly required in practical use unless you need your app to be deterministic.
51    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
52    for _ in 0..NUM_BODIES {
53        let radius: f32 = rng.random_range(0.1..0.7);
54        let mass_value = FloatPow::cubed(radius) * 10.;
55
56        let position = Vec3::new(
57            rng.random_range(-1.0..1.0),
58            rng.random_range(-1.0..1.0),
59            rng.random_range(-1.0..1.0),
60        )
61        .normalize()
62            * ops::cbrt(rng.random_range(0.2f32..1.0))
63            * 15.;
64
65        commands.spawn((
66            BodyBundle {
67                mesh: Mesh3d(mesh.clone()),
68                material: MeshMaterial3d(materials.add(Color::srgb(
69                    rng.random_range(color_range.clone()),
70                    rng.random_range(color_range.clone()),
71                    rng.random_range(color_range.clone()),
72                ))),
73                mass: Mass(mass_value),
74                acceleration: Acceleration(Vec3::ZERO),
75                last_pos: LastPos(
76                    position
77                        - Vec3::new(
78                            rng.random_range(vel_range.clone()),
79                            rng.random_range(vel_range.clone()),
80                            rng.random_range(vel_range.clone()),
81                        ) * time.timestep().as_secs_f32(),
82                ),
83            },
84            Transform {
85                translation: position,
86                scale: Vec3::splat(radius),
87                ..default()
88            },
89        ));
90    }
91
92    // add bigger "star" body in the center
93    let star_radius = 1.;
94    commands
95        .spawn((
96            BodyBundle {
97                mesh: Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap())),
98                material: MeshMaterial3d(materials.add(StandardMaterial {
99                    base_color: ORANGE_RED.into(),
100                    emissive: LinearRgba::from(ORANGE_RED) * 2.,
101                    ..default()
102                })),
103
104                mass: Mass(500.0),
105                ..default()
106            },
107            Transform::from_scale(Vec3::splat(star_radius)),
108            Star,
109        ))
110        .with_child(PointLight {
111            color: Color::WHITE,
112            range: 100.0,
113            radius: star_radius,
114            ..default()
115        });
116    commands.spawn((
117        Camera3d::default(),
118        Transform::from_xyz(0.0, 10.5, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
119    ));
120}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#128)

#### pub fn [set\_timestep](#method.set_timestep)(&mut self, timestep: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Sets the amount of virtual time that must pass before the fixed timestep schedule is run again, as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

Takes effect immediately on the next run of the schedule, respecting what is currently in [`Self::overstep`](struct.Time.html#method.overstep "method bevy::prelude::Time::overstep").

##### Panics

Panics if `timestep` is zero.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#150)

#### pub fn [set\_timestep\_seconds](#method.set_timestep_seconds)(&mut self, seconds: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Sets the amount of virtual time that must pass before the fixed timestep schedule is run again, as seconds.

Timestep is stored as a [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), which has fixed nanosecond resolution and will be converted from the floating point number.

Takes effect immediately on the next run of the schedule, respecting what is currently in [`Self::overstep`](struct.Time.html#method.overstep "method bevy::prelude::Time::overstep").

##### Panics

Panics if `seconds` is zero, negative or not finite.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#172)

#### pub fn [set\_timestep\_hz](#method.set_timestep_hz)(&mut self, hz: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Sets the amount of virtual time that must pass before the fixed timestep schedule is run again, as frequency.

The timestep value is set to `1 / hz`, converted to a [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration") which has fixed nanosecond resolution.

Takes effect immediately on the next run of the schedule, respecting what is currently in [`Self::overstep`](struct.Time.html#method.overstep "method bevy::prelude::Time::overstep").

##### Panics

Panics if `hz` is zero, negative or not finite.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#181)

#### pub fn [overstep](#method.overstep)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns the amount of overstep time accumulated toward new steps, as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/ecs/fixed\_timestep.rs ([line 37](../../src/fixed_timestep/fixed_timestep.rs.html#37))

```rust
26fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
27    // Default `Time`is `Time<Fixed>` here
28    info!(
29        "time since last fixed_update: {}\n",
30        time.elapsed_secs() - *last_time
31    );
32
33    info!("fixed timestep: {}\n", time.delta_secs());
34    // If we want to see the overstep, we need to access `Time<Fixed>` specifically
35    info!(
36        "time accrued toward next fixed_update: {}\n",
37        fixed_time.overstep().as_secs_f32()
38    );
39    *last_time = time.elapsed_secs();
40}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#189)

#### pub fn [accumulate\_overstep](#method.accumulate_overstep)(&mut self, delta: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Increase the overstep time accumulated towards new steps.

This method is provided for use in tests. Ordinarily, the [`run_fixed_main_schedule`](../time/fn.run_fixed_main_schedule.html "fn bevy::time::run_fixed_main_schedule") system is responsible for calculating the overstep.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#197)

#### pub fn [discard\_overstep](#method.discard_overstep)(&mut self, discard: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Discard a part of the overstep amount.

If `discard` is higher than overstep, the overstep becomes zero.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#205)

#### pub fn [overstep\_fraction](#method.overstep_fraction)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the amount of overstep time accumulated toward new steps, as an [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") fraction of the timestep.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/movement/physics\_in\_fixed\_timestep.rs ([line 404](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#404))

```rust
390fn interpolate_rendered_transform(
391    fixed_time: Res<Time<Fixed>>,
392    mut query: Query<(
393        &mut Transform,
394        &PhysicalTranslation,
395        &PreviousPhysicalTranslation,
396    )>,
397) {
398    for (mut transform, current_physical_translation, previous_physical_translation) in
399        query.iter_mut()
400    {
401        let previous = previous_physical_translation.0;
402        let current = current_physical_translation.0;
403        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
404        let alpha = fixed_time.overstep_fraction();
405
406        let rendered_translation = previous.lerp(current, alpha);
407        transform.translation = rendered_translation;
408    }
409}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#212)

#### pub fn [overstep\_fraction\_f64](#method.overstep_fraction_f64)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the amount of overstep time accumulated toward new steps, as an [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") fraction of the timestep.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#61)

### impl [Time](struct.Time.html "struct bevy::prelude::Time")<[Real](struct.Real.html "struct bevy::prelude::Real")\>

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#64)

#### pub fn [new](#method.new)(startup: [Instant](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")) -> [Time](struct.Time.html "struct bevy::prelude::Time")<[Real](struct.Real.html "struct bevy::prelude::Real")\>

Constructs a new `Time<Real>` instance with a specific startup [`Instant`](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant").

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#76)

#### pub fn [update](#method.update)(&mut self)

Updates the internal time measurements.

Calling this method as part of your app will most likely result in inaccurate timekeeping, as the [`Time`](struct.Time.html "struct bevy::prelude::Time") resource is ordinarily managed by the [`TimePlugin`](../time/struct.TimePlugin.html "struct bevy::time::TimePlugin").

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#88)

#### pub fn [update\_with\_duration](#method.update_with_duration)(&mut self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Updates time with a specified [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

This method is provided for use in tests.

Calling this method as part of your app will most likely result in inaccurate timekeeping, as the [`Time`](struct.Time.html "struct bevy::prelude::Time") resource is ordinarily managed by the [`TimePlugin`](../time/struct.TimePlugin.html "struct bevy::time::TimePlugin").

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#99)

#### pub fn [update\_with\_instant](#method.update_with_instant)(&mut self, instant: [Instant](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant"))

Updates time with a specified [`Instant`](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant").

This method is provided for use in tests.

Calling this method as part of your app will most likely result in inaccurate timekeeping, as the [`Time`](struct.Time.html "struct bevy::prelude::Time") resource is ordinarily managed by the [`TimePlugin`](../time/struct.TimePlugin.html "struct bevy::time::TimePlugin").

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#115)

#### pub fn [startup](#method.startup)(&self) -> [Instant](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

Returns the [`Instant`](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant") the clock was created.

This usually represents when the app was started.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#124)

#### pub fn [first\_update](#method.first_update)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Instant](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")\>

Returns the [`Instant`](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant") when [`Self::update`](struct.Time.html#method.update "method bevy::prelude::Time::update") was first called, if it exists.

This usually represents when the first app update started.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#133)

#### pub fn [last\_update](#method.last_update)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Instant](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")\>

Returns the [`Instant`](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant") when [`Self::update`](struct.Time.html#method.update "method bevy::prelude::Time::update") was last called, if it exists.

This usually represents when the current app update started.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#206)

### impl<T> [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#211)

#### pub fn [new\_with](#method.new_with)(context: T) -> [Time](struct.Time.html "struct bevy::prelude::Time")<T>

Create a new clock from context with [`Self::delta`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") and [`Self::elapsed`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") starting from zero.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#223)

#### pub fn [advance\_by](#method.advance_by)(&mut self, delta: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Advance this clock by adding a `delta` duration to it.

The added duration will be returned by [`Self::delta`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") and [`Self::elapsed`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") will be increased by the duration. Adding [`Duration::ZERO`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html#associatedconstant.ZERO "associated constant core::time::Duration::ZERO") is allowed and will set [`Self::delta`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") to zero.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#244)

#### pub fn [advance\_to](#method.advance_to)(&mut self, elapsed: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Advance this clock to a specific `elapsed` time.

[`Self::delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") will return the amount of time the clock was advanced and [`Self::elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") will be the `elapsed` value passed in. Cannot be used to move time backwards.

##### Panics

Panics if `elapsed` is less than `Self::elapsed()`.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#256)

#### pub fn [wrap\_period](#method.wrap_period)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns the modulus used to calculate [`elapsed_wrapped`](#method.elapsed_wrapped).

**Note:** The default modulus is one hour.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#268)

#### pub fn [set\_wrap\_period](#method.set_wrap_period)(&mut self, wrap\_period: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Sets the modulus used to calculate [`elapsed_wrapped`](#method.elapsed_wrapped).

**Note:** This will not take effect until the next update.

##### Panics

Panics if `wrap_period` is a zero-length duration.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#276)

#### pub fn [delta](#method.delta)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns how much time has advanced since the last [`update`](#method.update), as a [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/app/plugin.rs ([line 49](../../src/plugin/plugin.rs.html#49))

```rust
48fn print_message_system(mut state: ResMut<PrintMessageState>, time: Res<Time>) {
49    if state.timer.tick(time.delta()).is_finished() {
50        info!("{}", state.message);
51    }
52}
```

Hide additional examples

examples/time/time.rs ([line 90](../../src/time/time.rs.html#90))

```rust
87fn print_real_time(time: Res<Time<Real>>) {
88    println!(
89        "PreUpdate: this is real time clock, delta is {:?} and elapsed is {:?}",
90        time.delta(),
91        time.elapsed()
92    );
93}
94
95fn print_fixed_time(time: Res<Time>) {
96    println!(
97        "FixedUpdate: this is generic time clock inside fixed, delta is {:?} and elapsed is {:?}",
98        time.delta(),
99        time.elapsed()
100    );
101}
102
103fn print_time(time: Res<Time>) {
104    println!(
105        "Update: this is generic time clock, delta is {:?} and elapsed is {:?}",
106        time.delta(),
107        time.elapsed()
108    );
109}
```

examples/stress\_tests/many\_sprites.rs ([line 120](../../src/many_sprites/many_sprites.rs.html#120))

```rust
119fn print_sprite_count(time: Res<Time>, mut timer: Local<PrintingTimer>, sprites: Query<&Sprite>) {
120    timer.tick(time.delta());
121
122    if timer.just_finished() {
123        info!("Sprites: {}", sprites.iter().count());
124    }
125}
```

examples/stress\_tests/many\_lights.rs ([line 143](../../src/many_lights/many_lights.rs.html#143))

```rust
142fn print_light_count(time: Res<Time>, mut timer: Local<PrintingTimer>, lights: Query<&PointLight>) {
143    timer.0.tick(time.delta());
144
145    if timer.0.just_finished() {
146        info!("Lights: {}", lights.iter().len());
147    }
148}
149
150struct LogVisibleLights;
151
152impl Plugin for LogVisibleLights {
153    fn build(&self, app: &mut App) {
154        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
155            return;
156        };
157
158        render_app.add_systems(
159            Render,
160            print_visible_light_count.in_set(RenderSystems::Prepare),
161        );
162    }
163}
164
165// System for printing the number of meshes on every tick of the timer
166fn print_visible_light_count(
167    time: Res<Time>,
168    mut timer: Local<PrintingTimer>,
169    visible: Query<&ExtractedPointLight>,
170    global_clusterable_object_meta: Res<GlobalClusterableObjectMeta>,
171) {
172    timer.0.tick(time.delta());
173
174    if timer.0.just_finished() {
175        // Note that it's not generally a safe assumption that the number of
176        // lights equals the number of clusterable objects, since some objects
177        // other than lights are clusterable. However, in this specific example,
178        // the only clusterable objects are lights.
179        info!(
180            "Visible Lights: {}, Rendered Lights: {}",
181            visible.iter().len(),
182            global_clusterable_object_meta.entity_to_index.len()
183        );
184    }
185}
```

examples/ecs/generic\_system.rs ([line 68](../../src/generic_system/generic_system.rs.html#68))

```rust
66fn print_text_system(time: Res<Time>, mut query: Query<(&mut PrinterTick, &TextToPrint)>) {
67    for (mut timer, text) in &mut query {
68        if timer.tick(time.delta()).just_finished() {
69            info!("{}", text.0);
70        }
71    }
72}
```

examples/stress\_tests/many\_sprite\_meshes.rs ([line 126](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#126))

```rust
121fn print_sprite_count(
122    time: Res<Time>,
123    mut timer: Local<PrintingTimer>,
124    sprites: Query<&SpriteMesh>,
125) {
126    timer.tick(time.delta());
127
128    if timer.just_finished() {
129        info!("Sprites: {}", sprites.iter().count());
130    }
131}
```

Additional examples can be found in:  

*   [examples/time/timers.rs](../../src/timers/timers.rs.html#50)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#128)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#107)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#103)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#45)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#141)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#568)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#140)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#71)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#179)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#310)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#28)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#122)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#201)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#110)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#114)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#116)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#81)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#200)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#289)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#168)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#98)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#118)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#103)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#176)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#60)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#178)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#276)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#207)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#293)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#283)

#### pub fn [delta\_secs](#method.delta_secs)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns how much time has advanced since the last [`update`](#method.update), as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") seconds.

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/shader/extended\_material.rs ([line 70](../../src/extended_material/extended_material.rs.html#70))

```rust
68fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
69    for mut t in &mut q {
70        t.rotate_y(time.delta_secs());
71    }
72}
```

Hide additional examples

examples/3d/3d\_shapes.rs ([line 227](../../src/3d_shapes/3d_shapes.rs.html#227))

```rust
225fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
226    for mut transform in &mut query {
227        transform.rotate_y(time.delta_secs() / 2.);
228    }
229}
```

examples/picking/mesh\_picking.rs ([line 188](../../src/mesh_picking/mesh_picking.rs.html#188))

```rust
186fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
187    for mut transform in &mut query {
188        transform.rotate_y(time.delta_secs() / 2.);
189    }
190}
```

examples/2d/2d\_shapes.rs ([line 168](../../src/2d_shapes/2d_shapes.rs.html#168))

```rust
166fn rotate(mut query: Query<&mut Transform, With<Mesh2d>>, time: Res<Time>) {
167    for mut transform in &mut query {
168        transform.rotate_z(time.delta_secs() / 2.0);
169    }
170}
```

examples/gizmos/light\_gizmos.rs ([line 151](../../src/light_gizmos/light_gizmos.rs.html#151))

```rust
150fn rotate_camera(mut transform: Single<&mut Transform, With<Camera>>, time: Res<Time>) {
151    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs() / 2.));
152}
153
154fn update_config(
155    mut config_store: ResMut<GizmoConfigStore>,
156    keyboard: Res<ButtonInput<KeyCode>>,
157    time: Res<Time>,
158    color_text_query: Single<Entity, With<GizmoColorText>>,
159    mut writer: TextUiWriter,
160) {
161    if keyboard.just_pressed(KeyCode::KeyD) {
162        for (_, config, _) in config_store.iter_mut() {
163            config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
164        }
165    }
166
167    let (config, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
168    if keyboard.pressed(KeyCode::ArrowRight) {
169        config.line.width += 5. * time.delta_secs();
170        config.line.width = config.line.width.clamp(0., 50.);
171    }
172    if keyboard.pressed(KeyCode::ArrowLeft) {
173        config.line.width -= 5. * time.delta_secs();
174        config.line.width = config.line.width.clamp(0., 50.);
175    }
176    if keyboard.just_pressed(KeyCode::KeyA) {
177        config.enabled ^= true;
178    }
179    if keyboard.just_pressed(KeyCode::KeyC) {
180        light_config.color = match light_config.color {
181            LightGizmoColor::Manual(_) => LightGizmoColor::Varied,
182            LightGizmoColor::Varied => LightGizmoColor::MatchLightColor,
183            LightGizmoColor::MatchLightColor => LightGizmoColor::ByLightType,
184            LightGizmoColor::ByLightType => LightGizmoColor::Manual(GRAY.into()),
185        };
186        *writer.text(*color_text_query, 1) = gizmo_color_text(light_config);
187    }
188}
```

examples/3d/parenting.rs ([line 21](../../src/parenting/parenting.rs.html#21))

```rust
19fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
20    for mut transform in &mut query {
21        transform.rotate_x(3.0 * time.delta_secs());
22    }
23}
```

Additional examples can be found in:  

*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#139)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#41)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#101)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#99)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#124)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#243)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#318)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#183)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#136)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#260)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#93)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#101)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#314)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#283)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#112)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#300)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#85)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#164)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#106)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#104)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#208)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#55)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#294)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#177)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#102)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#100)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#139)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#69)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#94)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#157)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#556)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#295)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#163)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#563)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#422)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#69)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#45)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#126)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#282)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#56)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#53)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#386)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#151)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#69)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#353)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#359)
*   [examples/ecs/fixed\_timestep.rs](../../src/fixed_timestep/fixed_timestep.rs.html#33)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#138)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#34)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#80)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#114)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#120)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#205)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#294)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#120)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#191)
*   [examples/state/states.rs](../../src/states/states.rs.html#147)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#112)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#204)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#88)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#309)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#443)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#180)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#514)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#103)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#85)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#288)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#70)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#31)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#277)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#135)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#349)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#170)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#123)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#299)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#116)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#269)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#50)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#134)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#229)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#125)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#143)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#144)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#118)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#396)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#290)

#### pub fn [delta\_secs\_f64](#method.delta_secs_f64)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns how much time has advanced since the last [`update`](#method.update), as [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") seconds.

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/stress\_tests/bevymark\_3d.rs ([line 317](../../src/bevymark_3d/bevymark_3d.rs.html#317))

```rust
297fn mouse_handler(
298    mut commands: Commands,
299    args: Res<Args>,
300    time: Res<Time>,
301    mouse_button_input: Res<ButtonInput<MouseButton>>,
302    cube_resources: ResMut<CubeResources>,
303    mut counter: ResMut<BevyCounter>,
304    mut rng: Local<Option<ChaCha8Rng>>,
305    mut wave: Local<usize>,
306) {
307    if rng.is_none() {
308        *rng = Some(ChaCha8Rng::seed_from_u64(42));
309    }
310    let rng = rng.as_mut().unwrap();
311
312    if mouse_button_input.just_released(MouseButton::Left) {
313        counter.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
314    }
315
316    if mouse_button_input.pressed(MouseButton::Left) {
317        let spawn_count = (CUBES_PER_SECOND as f64 * time.delta_secs_f64()) as usize;
318        spawn_cubes(
319            &mut commands,
320            args.into_inner(),
321            &mut counter,
322            spawn_count,
323            cube_resources.into_inner(),
324            None,
325            *wave,
326        );
327        *wave += 1;
328    }
329}
```

Hide additional examples

examples/stress\_tests/bevymark.rs ([line 350](../../src/bevymark/bevymark.rs.html#350))

```rust
323fn mouse_handler(
324    mut commands: Commands,
325    args: Res<Args>,
326    time: Res<Time>,
327    mouse_button_input: Res<ButtonInput<MouseButton>>,
328    window: Query<&Window>,
329    bird_resources: ResMut<BirdResources>,
330    mut counter: ResMut<BevyCounter>,
331    mut rng: Local<Option<ChaCha8Rng>>,
332    mut wave: Local<usize>,
333) {
334    let Ok(window) = window.single() else {
335        return;
336    };
337
338    if rng.is_none() {
339        // We're seeding the PRNG here to make this example deterministic for testing purposes.
340        // This isn't strictly required in practical use unless you need your app to be deterministic.
341        *rng = Some(ChaCha8Rng::seed_from_u64(42));
342    }
343    let rng = rng.as_mut().unwrap();
344
345    if mouse_button_input.just_released(MouseButton::Left) {
346        counter.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
347    }
348
349    if mouse_button_input.pressed(MouseButton::Left) {
350        let spawn_count = (BIRDS_PER_SECOND as f64 * time.delta_secs_f64()) as usize;
351        spawn_birds(
352            &mut commands,
353            args.into_inner(),
354            &window.resolution,
355            &mut counter,
356            spawn_count,
357            bird_resources.into_inner(),
358            None,
359            *wave,
360        );
361        *wave += 1;
362    }
363}
```

examples/ui/text/text\_debug.rs ([line 275](../../src/text_debug/text_debug.rs.html#275))

```rust
248fn change_text_system(
249    mut fps_history: Local<VecDeque<f64>>,
250    mut time_history: Local<VecDeque<Duration>>,
251    time: Res<Time>,
252    diagnostics: Res<DiagnosticsStore>,
253    query: Query<Entity, With<TextChanges>>,
254    mut writer: TextUiWriter,
255) {
256    time_history.push_front(time.elapsed());
257    time_history.truncate(120);
258    let avg_fps = (time_history.len() as f64)
259        / (time_history.front().copied().unwrap_or_default()
260            - time_history.back().copied().unwrap_or_default())
261        .as_secs_f64()
262        .max(0.0001);
263    fps_history.push_front(avg_fps);
264    fps_history.truncate(120);
265    let fps_variance = std_deviation(fps_history.make_contiguous()).unwrap_or_default();
266
267    for entity in &query {
268        let mut fps = 0.0;
269        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
270            && let Some(fps_smoothed) = fps_diagnostic.smoothed()
271        {
272            fps = fps_smoothed;
273        }
274
275        let mut frame_time = time.delta_secs_f64();
276        if let Some(frame_time_diagnostic) =
277            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
278            && let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed()
279        {
280            frame_time = frame_time_smoothed;
281        }
282
283        *writer.text(entity, 0) =
284            format!("{avg_fps:.1} avg fps, {fps_variance:.1} frametime variance",);
285
286        *writer.text(entity, 1) = format!(
287            "\nThis text changes in the bottom right - {fps:.1} fps, {frame_time:.3} ms/frame",
288        );
289
290        *writer.text(entity, 4) = format!("{fps:.1}");
291
292        *writer.text(entity, 6) = format!("{frame_time:.3}");
293    }
294}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#296)

#### pub fn [elapsed](#method.elapsed)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns how much time has advanced since [`startup`](#method.startup), as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/window/window\_settings.rs ([line 129](../../src/window_settings/window_settings.rs.html#129))

```rust
126fn change_title(mut window: Single<&mut Window>, time: Res<Time>) {
127    window.title = format!(
128        "Seconds since startup: {}",
129        time.elapsed().as_secs_f32().round()
130    );
131}
```

Hide additional examples

examples/time/time.rs ([line 91](../../src/time/time.rs.html#91))

```rust
87fn print_real_time(time: Res<Time<Real>>) {
88    println!(
89        "PreUpdate: this is real time clock, delta is {:?} and elapsed is {:?}",
90        time.delta(),
91        time.elapsed()
92    );
93}
94
95fn print_fixed_time(time: Res<Time>) {
96    println!(
97        "FixedUpdate: this is generic time clock inside fixed, delta is {:?} and elapsed is {:?}",
98        time.delta(),
99        time.elapsed()
100    );
101}
102
103fn print_time(time: Res<Time>) {
104    println!(
105        "Update: this is generic time clock, delta is {:?} and elapsed is {:?}",
106        time.delta(),
107        time.elapsed()
108    );
109}
```

examples/scene/world\_serialization.rs ([line 91](../../src/world_serialization/world_serialization.rs.html#91))

```rust
88    fn from_world(world: &mut World) -> Self {
89        let time = world.resource::<Time>();
90        ComponentB {
91            _time_since_startup: time.elapsed(),
92            value: "Default Value".to_string(),
93        }
94    }
```

examples/ui/text/text\_debug.rs ([line 256](../../src/text_debug/text_debug.rs.html#256))

```rust
248fn change_text_system(
249    mut fps_history: Local<VecDeque<f64>>,
250    mut time_history: Local<VecDeque<Duration>>,
251    time: Res<Time>,
252    diagnostics: Res<DiagnosticsStore>,
253    query: Query<Entity, With<TextChanges>>,
254    mut writer: TextUiWriter,
255) {
256    time_history.push_front(time.elapsed());
257    time_history.truncate(120);
258    let avg_fps = (time_history.len() as f64)
259        / (time_history.front().copied().unwrap_or_default()
260            - time_history.back().copied().unwrap_or_default())
261        .as_secs_f64()
262        .max(0.0001);
263    fps_history.push_front(avg_fps);
264    fps_history.truncate(120);
265    let fps_variance = std_deviation(fps_history.make_contiguous()).unwrap_or_default();
266
267    for entity in &query {
268        let mut fps = 0.0;
269        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
270            && let Some(fps_smoothed) = fps_diagnostic.smoothed()
271        {
272            fps = fps_smoothed;
273        }
274
275        let mut frame_time = time.delta_secs_f64();
276        if let Some(frame_time_diagnostic) =
277            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
278            && let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed()
279        {
280            frame_time = frame_time_smoothed;
281        }
282
283        *writer.text(entity, 0) =
284            format!("{avg_fps:.1} avg fps, {fps_variance:.1} frametime variance",);
285
286        *writer.text(entity, 1) = format!(
287            "\nThis text changes in the bottom right - {fps:.1} fps, {frame_time:.3} ms/frame",
288        );
289
290        *writer.text(entity, 4) = format!("{fps:.1}");
291
292        *writer.text(entity, 6) = format!("{frame_time:.3}");
293    }
294}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#306)

#### pub fn [elapsed\_secs](#method.elapsed_secs)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns how much time has advanced since [`startup`](#method.startup), as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") seconds.

**Note:** This is a monotonically increasing value. Its precision will degrade over time. If you need an `f32` but that precision loss is unacceptable, use [`elapsed_secs_wrapped`](#method.elapsed_secs_wrapped).

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/ecs/extraction.rs ([line 75](../../src/extraction/extraction.rs.html#75))

```rust
74fn setup(mut commands: Commands, time: Res<Time>) {
75    commands.spawn(A(time.elapsed_secs()));
76    commands.spawn(B(time.elapsed_secs()));
77    commands.spawn(C(time.elapsed_secs()));
78}
79
80// Sets the elapsed time on each of the components on the Main World. Runs each frame.
81fn set_time(mut a: Single<&mut A>, mut b: Single<&mut B>, mut c: Single<&mut C>, time: Res<Time>) {
82    a.0 = time.elapsed_secs();
83    b.0 = time.elapsed_secs();
84    c.0 = time.elapsed_secs();
85}
```

Hide additional examples

examples/remote/server.rs ([line 81](../../src/server/server.rs.html#81))

```rust
79fn move_cube(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
80    for mut transform in &mut query {
81        transform.translation.y = -cos(time.elapsed_secs()) + 1.5;
82    }
83}
```

examples/3d/meshlet.rs ([line 124](../../src/meshlet/meshlet.rs.html#124))

```rust
122fn bunny_wiggler(mut bunny: Query<&mut Transform, With<BunnyWiggler>>, time: Res<Time>) {
123    bunny.single_mut().as_deref_mut().unwrap().translation.z +=
124        ops::cos(time.elapsed_secs() * 10.0) * 0.003;
125}
```

examples/shader/extended\_material\_bindless.rs ([line 152](../../src/extended_material_bindless/extended_material_bindless.rs.html#152))

```rust
149fn rotate_sphere(mut meshes: Query<&mut Transform, With<Mesh3d>>, time: Res<Time>) {
150    for mut transform in &mut meshes {
151        transform.rotation =
152            Quat::from_euler(EulerRot::YXZ, -time.elapsed_secs(), FRAC_PI_2 * 3.0, 0.0);
153    }
154}
```

examples/ecs/fixed\_timestep.rs ([line 21](../../src/fixed_timestep/fixed_timestep.rs.html#21))

```rust
17fn frame_update(mut last_time: Local<f32>, time: Res<Time>) {
18    // Default `Time` is `Time<Virtual>` here
19    info!(
20        "time since last frame_update: {}",
21        time.elapsed_secs() - *last_time
22    );
23    *last_time = time.elapsed_secs();
24}
25
26fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
27    // Default `Time`is `Time<Fixed>` here
28    info!(
29        "time since last fixed_update: {}\n",
30        time.elapsed_secs() - *last_time
31    );
32
33    info!("fixed timestep: {}\n", time.delta_secs());
34    // If we want to see the overstep, we need to access `Time<Fixed>` specifically
35    info!(
36        "time accrued toward next fixed_update: {}\n",
37        fixed_time.overstep().as_secs_f32()
38    );
39    *last_time = time.elapsed_secs();
40}
```

examples/shader/shader\_prepass.rs ([line 175](../../src/shader_prepass/shader_prepass.rs.html#175))

```rust
173fn rotate(mut q: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
174    for mut t in q.iter_mut() {
175        let rot = (ops::sin(time.elapsed_secs()) * 0.5 + 0.5) * std::f32::consts::PI * 2.0;
176        t.rotation = Quat::from_rotation_z(rot);
177    }
178}
```

Additional examples can be found in:  

*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#111)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#82)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#275)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#226)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#91)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#69)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#629)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#199)
*   [examples/state/states.rs](../../src/states/states.rs.html#155)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#120)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#80)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#181)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#59)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#44)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#515)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#186)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#41)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#225)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#248)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#19)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#372)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#162)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#397)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#92)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#247)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#286)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#178)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#102)
*   [examples/animation/color\_animation.rs](../../src/color_animation/color_animation.rs.html#93)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#51)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#589)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#131)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#97)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#141)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#209)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#438)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#81)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#26)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#291)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#68)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#71)
*   [examples/ecs/change\_detection.rs](../../src/change_detection/change_detection.rs.html#38)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#80)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#99)
*   [examples/gizmos/anchored\_text\_gizmos.rs](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#22)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#31)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#599)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#268)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#69)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#306)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#113)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#82)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#44)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#146)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#201)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#135)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#46)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#432)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#126)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#105)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#94)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#143)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#312)

#### pub fn [elapsed\_secs\_f64](#method.elapsed_secs_f64)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns how much time has advanced since [`startup`](#method.startup), as [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") seconds.

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/ui/styling/box\_shadow.rs ([line 529](../../src/box_shadow/box_shadow.rs.html#529))

```rust
519fn button_system(
520    mut interaction_query: Query<
521        (&Interaction, &SettingsButton),
522        (Changed<Interaction>, With<Button>),
523    >,
524    mut shadow: ResMut<ShadowSettings>,
525    mut shape: ResMut<ShapeSettings>,
526    mut held: ResMut<HeldButton>,
527    time: Res<Time>,
528) {
529    let now = time.elapsed_secs_f64();
530    for (interaction, btn) in &mut interaction_query {
531        match *interaction {
532            Interaction::Pressed => {
533                trigger_button_action(btn, &mut shadow, &mut shape);
534                held.button = Some(*btn);
535                held.pressed_at = Some(now);
536                held.last_repeat = Some(now);
537            }
538            Interaction::None | Interaction::Hovered => {
539                if held.button == Some(*btn) {
540                    held.button = None;
541                    held.pressed_at = None;
542                    held.last_repeat = None;
543                }
544            }
545        }
546    }
547}
548
549fn trigger_button_action(
550    btn: &SettingsButton,
551    shadow: &mut ShadowSettings,
552    shape: &mut ShapeSettings,
553) {
554    match btn {
555        SettingsButton::XOffsetInc => shadow.x_offset += 1.0,
556        SettingsButton::XOffsetDec => shadow.x_offset -= 1.0,
557        SettingsButton::YOffsetInc => shadow.y_offset += 1.0,
558        SettingsButton::YOffsetDec => shadow.y_offset -= 1.0,
559        SettingsButton::BlurInc => shadow.blur = (shadow.blur + 1.0).max(0.0),
560        SettingsButton::BlurDec => shadow.blur = (shadow.blur - 1.0).max(0.0),
561        SettingsButton::SpreadInc => shadow.spread += 1.0,
562        SettingsButton::SpreadDec => shadow.spread -= 1.0,
563        SettingsButton::CountInc => {
564            if shadow.count < 3 {
565                shadow.count += 1;
566            }
567        }
568        SettingsButton::CountDec => {
569            if shadow.count > 1 {
570                shadow.count -= 1;
571            }
572        }
573        SettingsButton::ShapePrev => {
574            if shape.index == 0 {
575                shape.index = SHAPES.len() - 1;
576            } else {
577                shape.index -= 1;
578            }
579        }
580        SettingsButton::ShapeNext => {
581            shape.index = (shape.index + 1) % SHAPES.len();
582        }
583        SettingsButton::Reset => {
584            *shape = SHAPE_DEFAULT_SETTINGS;
585            *shadow = SHADOW_DEFAULT_SETTINGS;
586        }
587        SettingsButton::SamplesInc => shadow.samples += 1,
588        SettingsButton::SamplesDec => {
589            if shadow.samples > 1 {
590                shadow.samples -= 1;
591            }
592        }
593    }
594}
595
596// System to repeat button action while held
597fn button_repeat_system(
598    time: Res<Time>,
599    mut held: ResMut<HeldButton>,
600    mut shadow: ResMut<ShadowSettings>,
601    mut shape: ResMut<ShapeSettings>,
602    mut request_redraw_writer: MessageWriter<RequestRedraw>,
603) {
604    if held.button.is_some() {
605        request_redraw_writer.write(RequestRedraw);
606    }
607    const INITIAL_DELAY: f64 = 0.15;
608    const REPEAT_RATE: f64 = 0.08;
609    if let (Some(btn), Some(pressed_at)) = (held.button, held.pressed_at) {
610        let now = time.elapsed_secs_f64();
611        let since_pressed = now - pressed_at;
612        let last_repeat = held.last_repeat.unwrap_or(pressed_at);
613        let since_last = now - last_repeat;
614        if since_pressed > INITIAL_DELAY && since_last > REPEAT_RATE {
615            trigger_button_action(&btn, &mut shadow, &mut shape);
616            held.last_repeat = Some(now);
617        }
618    }
619}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#319)

#### pub fn [elapsed\_wrapped](#method.elapsed_wrapped)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns how much time has advanced since [`startup`](#method.startup) modulo the [`wrap_period`](#method.wrap_period), as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#329)

#### pub fn [elapsed\_secs\_wrapped](#method.elapsed_secs_wrapped)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns how much time has advanced since [`startup`](#method.startup) modulo the [`wrap_period`](#method.wrap_period), as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") seconds.

This method is intended for applications (e.g. shaders) that require an [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") value but suffer from the gradual precision loss of [`elapsed_secs`](#method.elapsed_secs).

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#336)

#### pub fn [elapsed\_secs\_wrapped\_f64](#method.elapsed_secs_wrapped_f64)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns how much time has advanced since [`startup`](#method.startup) modulo the [`wrap_period`](#method.wrap_period), as [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") seconds.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#342)

#### pub fn [context](#method.context)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the context of this specific clock.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#348)

#### pub fn [context\_mut](#method.context_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a mutable reference to the context of this specific clock.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#354)

#### pub fn [as\_generic](#method.as_generic)(&self) -> [Time](struct.Time.html "struct bevy::prelude::Time")

Returns a copy of this clock as fully generic clock without context.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#82)

### impl [Time](struct.Time.html "struct bevy::prelude::Time")<[Virtual](struct.Virtual.html "struct bevy::prelude::Virtual")\>

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#93)

#### pub fn [from\_max\_delta](#method.from_max_delta)(max\_delta: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Time](struct.Time.html "struct bevy::prelude::Time")<[Virtual](struct.Virtual.html "struct bevy::prelude::Virtual")\>

Create new virtual clock with given maximum delta step [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

##### Panics

Panics if `max_delta` is zero.

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/time/time.rs ([line 114](../../src/time/time.rs.html#114))

```rust
111fn main() {
112    App::new()
113        .add_plugins(MinimalPlugins)
114        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
115        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
116        .add_systems(PreUpdate, print_real_time)
117        .add_systems(FixedUpdate, print_fixed_time)
118        .add_systems(Update, print_time)
119        .set_runner(runner)
120        .run();
121}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#112)

#### pub fn [max\_delta](#method.max_delta)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns the maximum amount of time that can be added to this clock by a single update, as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

This is the maximum value [`Self::delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") will return and also to maximum time [`Self::elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") will be increased by in a single update.

This ensures that even if no updates happen for an extended amount of time, the clock will not have a sudden, huge advance all at once. This also indirectly limits the maximum number of fixed update steps that can run in a single update.

The default value is 250 milliseconds.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#140)

#### pub fn [set\_max\_delta](#method.set_max_delta)(&mut self, max\_delta: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"))

Sets the maximum amount of time that can be added to this clock by a single update, as [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration").

This is the maximum value [`Self::delta()`](struct.Time.html#method.delta "method bevy::prelude::Time::delta") will return and also to maximum time [`Self::elapsed()`](struct.Time.html#method.elapsed "method bevy::prelude::Time::elapsed") will be increased by in a single update.

This is used to ensure that even if the game freezes for a few seconds, or is suspended for hours or even days, the virtual clock doesn’t suddenly jump forward for that full amount, which would likely cause gameplay bugs or having to suddenly simulate all the intervening time.

If no updates happen for an extended amount of time, this limit prevents having a sudden, huge advance all at once. This also indirectly limits the maximum number of fixed update steps that can run in a single update.

The default value is 250 milliseconds. If you want to disable this feature, set the value to [`Duration::MAX`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html#associatedconstant.MAX "associated constant core::time::Duration::MAX").

##### Panics

Panics if `max_delta` is zero.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#148)

#### pub fn [relative\_speed](#method.relative_speed)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the speed the clock advances relative to your system clock, as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32"). This is known as “time scaling” or “time dilation” in other engines.

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/time/virtual\_time.rs ([line 158](../../src/virtual_time/virtual_time.rs.html#158))

```rust
157fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
158    let time_speed = (time.relative_speed() + DELTA as f32)
159        .round()
160        .clamp(0.25, 5.);
161
162    // set the speed of the virtual time to speed it up or slow it down
163    time.set_relative_speed(time_speed);
164}
165
166/// Pause or resume `Relative` time
167fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
168    time.toggle();
169}
170
171/// Update the `Real` time info text
172fn update_real_time_info_text(time: Res<Time<Real>>, mut query: Query<&mut Text, With<RealTime>>) {
173    for mut text in &mut query {
174        **text = format!(
175            "REAL TIME\nElapsed: {:.1}\nDelta: {:.5}\n",
176            time.elapsed_secs(),
177            time.delta_secs(),
178        );
179    }
180}
181
182/// Update the `Virtual` time info text
183fn update_virtual_time_info_text(
184    time: Res<Time<Virtual>>,
185    mut query: Query<&mut Text, With<VirtualTime>>,
186) {
187    for mut text in &mut query {
188        **text = format!(
189            "VIRTUAL TIME\nElapsed: {:.1}\nDelta: {:.5}\nSpeed: {:.2}",
190            time.elapsed_secs(),
191            time.delta_secs(),
192            time.relative_speed()
193        );
194    }
195}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#155)

#### pub fn [relative\_speed\_f64](#method.relative_speed_f64)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the speed the clock advances relative to your system clock, as [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64"). This is known as “time scaling” or “time dilation” in other engines.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#165)

#### pub fn [effective\_speed](#method.effective_speed)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the speed the clock advanced relative to your system clock in this update, as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32").

Returns `0.0` if the game was paused or what the `relative_speed` value was at the start of this update.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#175)

#### pub fn [effective\_speed\_f64](#method.effective_speed_f64)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the speed the clock advanced relative to your system clock in this update, as [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64").

Returns `0.0` if the game was paused or what the `relative_speed` value was at the start of this update.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#188)

#### pub fn [set\_relative\_speed](#method.set_relative_speed)(&mut self, ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the speed the clock advances relative to your system clock, given as an [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32").

For example, setting this to `2.0` will make the clock advance twice as fast as your system clock.

##### Panics

Panics if `ratio` is negative or not finite.

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

examples/time/time.rs ([line 52](../../src/time/time.rs.html#52))

```rust
35fn runner(mut app: App) -> AppExit {
36    banner();
37    help();
38    let stdin = io::stdin();
39    for line in stdin.lock().lines() {
40        if let Err(err) = line {
41            println!("read err: {err:#}");
42            break;
43        }
44        match line.unwrap().as_str() {
45            "" => {
46                app.update();
47            }
48            "f" => {
49                println!("FAST: setting relative speed to 2x");
50                app.world_mut()
51                    .resource_mut::<Time<Virtual>>()
52                    .set_relative_speed(2.0);
53            }
54            "n" => {
55                println!("NORMAL: setting relative speed to 1x");
56                app.world_mut()
57                    .resource_mut::<Time<Virtual>>()
58                    .set_relative_speed(1.0);
59            }
60            "s" => {
61                println!("SLOW: setting relative speed to 0.5x");
62                app.world_mut()
63                    .resource_mut::<Time<Virtual>>()
64                    .set_relative_speed(0.5);
65            }
66            "p" => {
67                println!("PAUSE: pausing virtual clock");
68                app.world_mut().resource_mut::<Time<Virtual>>().pause();
69            }
70            "u" => {
71                println!("UNPAUSE: resuming virtual clock");
72                app.world_mut().resource_mut::<Time<Virtual>>().unpause();
73            }
74            "q" => {
75                println!("QUITTING!");
76                break;
77            }
78            _ => {
79                help();
80            }
81        }
82    }
83
84    AppExit::Success
85}
```

Hide additional examples

examples/time/virtual\_time.rs ([line 46](../../src/virtual_time/virtual_time.rs.html#46))

```rust
43fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut time: ResMut<Time<Virtual>>) {
44    // start with double `Virtual` time resulting in one of the sprites moving at twice the speed
45    // of the other sprite which moves based on `Real` (unscaled) time
46    time.set_relative_speed(2.);
47
48    commands.spawn(Camera2d);
49
50    let virtual_color = GOLD.into();
51    let sprite_scale = Vec2::splat(0.5).extend(1.);
52    let texture_handle = asset_server.load("branding/icon.png");
53
54    // the sprite moving based on real time
55    commands.spawn((
56        Sprite::from_image(texture_handle.clone()),
57        Transform::from_scale(sprite_scale),
58        RealTime,
59    ));
60
61    // the sprite moving based on virtual time
62    commands.spawn((
63        Sprite {
64            image: texture_handle,
65            color: virtual_color,
66            ..Default::default()
67        },
68        Transform {
69            scale: sprite_scale,
70            translation: Vec3::new(0., -160., 0.),
71            ..default()
72        },
73        VirtualTime,
74    ));
75
76    // info UI
77    let font_size = FontSize::Px(33.);
78
79    commands.spawn((
80        Node {
81            display: Display::Flex,
82            justify_content: JustifyContent::SpaceBetween,
83            width: percent(100),
84            position_type: PositionType::Absolute,
85            top: px(0),
86            padding: UiRect::all(px(20)),
87            ..default()
88        },
89        children![
90            (
91                Text::default(),
92                TextFont {
93                    font_size,
94                    ..default()
95                },
96                RealTime,
97            ),
98            (
99                Text::new("CONTROLS\n(Un)pause: Space\nSpeed+: Up\nSpeed-: Down"),
100                TextFont {
101                    font_size,
102                    ..default()
103                },
104                TextColor(Color::srgb(0.85, 0.85, 0.85)),
105                TextLayout::justify(Justify::Center),
106            ),
107            (
108                Text::default(),
109                TextFont {
110                    font_size,
111                    ..default()
112                },
113                TextColor(virtual_color),
114                TextLayout::justify(Justify::Right),
115                VirtualTime,
116            ),
117        ],
118    ));
119}
120
121/// Move sprites using `Real` (unscaled) time
122fn move_real_time_sprites(
123    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<RealTime>)>,
124    // `Real` time which is not scaled or paused
125    time: Res<Time<Real>>,
126) {
127    for mut transform in sprite_query.iter_mut() {
128        // move roughly half the screen in a `Real` second
129        // when the time is scaled the speed is going to change
130        // and the sprite will stay still the time is paused
131        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
132    }
133}
134
135/// Move sprites using `Virtual` (scaled) time
136fn move_virtual_time_sprites(
137    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<VirtualTime>)>,
138    // the default `Time` is either `Time<Virtual>` in regular systems
139    // or `Time<Fixed>` in fixed timestep systems so `Time::delta()`,
140    // `Time::elapsed()` will return the appropriate values either way
141    time: Res<Time>,
142) {
143    for mut transform in sprite_query.iter_mut() {
144        // move roughly half the screen in a `Virtual` second
145        // when time is scaled using `Time<Virtual>::set_relative_speed` it's going
146        // to move at a different pace and the sprite will stay still when time is
147        // `Time<Virtual>::is_paused()`
148        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
149    }
150}
151
152fn get_sprite_translation_x(elapsed: f32) -> f32 {
153    ops::sin(elapsed) * 500.
154}
155
156/// Update the speed of `Time<Virtual>.` by `DELTA`
157fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
158    let time_speed = (time.relative_speed() + DELTA as f32)
159        .round()
160        .clamp(0.25, 5.);
161
162    // set the speed of the virtual time to speed it up or slow it down
163    time.set_relative_speed(time_speed);
164}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#201)

#### pub fn [set\_relative\_speed\_f64](#method.set_relative_speed_f64)(&mut self, ratio: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Sets the speed the clock advances relative to your system clock, given as an [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64").

For example, setting this to `2.0` will make the clock advance twice as fast as your system clock.

##### Panics

Panics if `ratio` is negative or not finite.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#209)

#### pub fn [toggle](#method.toggle)(&mut self)

Stops the clock if it is running, otherwise resumes the clock.

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

examples/time/virtual\_time.rs ([line 168](../../src/virtual_time/virtual_time.rs.html#168))

```rust
167fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
168    time.toggle();
169}
```

Hide additional examples

examples/3d/solari.rs ([line 448](../../src/solari/solari.rs.html#448))

```rust
446fn pause_scene(mut time: ResMut<Time<Virtual>>, key_input: Res<ButtonInput<KeyCode>>) {
447    if key_input.just_pressed(KeyCode::Space) {
448        time.toggle();
449    }
450}
```

examples/gizmos/2d\_gizmos.rs ([line 213](../../src/2d_gizmos/2d_gizmos.rs.html#213))

```rust
126fn update_config(
127    mut config_store: ResMut<GizmoConfigStore>,
128    keyboard: Res<ButtonInput<KeyCode>>,
129    real_time: Res<Time<Real>>,
130    mut virtual_time: ResMut<Time<Virtual>>,
131) {
132    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
133    if keyboard.pressed(KeyCode::ArrowRight) {
134        config.line.width += 5. * real_time.delta_secs();
135        config.line.width = config.line.width.clamp(0., 50.);
136    }
137    if keyboard.pressed(KeyCode::ArrowLeft) {
138        config.line.width -= 5. * real_time.delta_secs();
139        config.line.width = config.line.width.clamp(0., 50.);
140    }
141    if keyboard.just_pressed(KeyCode::Digit1) {
142        config.enabled ^= true;
143    }
144    if keyboard.just_pressed(KeyCode::KeyU) {
145        config.line.style = match config.line.style {
146            GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
147            GizmoLineStyle::Dotted => GizmoLineStyle::Dashed {
148                gap_scale: 3.0,
149                line_scale: 5.0,
150            },
151            _ => GizmoLineStyle::Solid,
152        };
153    }
154    if keyboard.just_pressed(KeyCode::KeyI) {
155        config.line.style = match config.line.style {
156            GizmoLineStyle::Solid => GizmoLineStyle::Dashed {
157                gap_scale: 3.0,
158                line_scale: 5.0,
159            },
160            GizmoLineStyle::Dotted => GizmoLineStyle::Solid,
161            _ => GizmoLineStyle::Dotted,
162        };
163    }
164    if keyboard.just_pressed(KeyCode::KeyJ) {
165        config.line.joints = match config.line.joints {
166            GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
167            GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
168            GizmoLineJoint::Round(_) => GizmoLineJoint::None,
169            GizmoLineJoint::None => GizmoLineJoint::Bevel,
170        };
171    }
172
173    if keyboard.just_pressed(KeyCode::KeyK) {
174        config.line.joints = match config.line.joints {
175            GizmoLineJoint::Bevel => GizmoLineJoint::None,
176            GizmoLineJoint::Miter => GizmoLineJoint::Bevel,
177            GizmoLineJoint::Round(_) => GizmoLineJoint::Miter,
178            GizmoLineJoint::None => GizmoLineJoint::Round(4),
179        };
180    }
181
182    let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
183    if keyboard.pressed(KeyCode::ArrowUp) {
184        my_config.line.width += 5. * real_time.delta_secs();
185        my_config.line.width = my_config.line.width.clamp(0., 50.);
186    }
187    if keyboard.pressed(KeyCode::ArrowDown) {
188        my_config.line.width -= 5. * real_time.delta_secs();
189        my_config.line.width = my_config.line.width.clamp(0., 50.);
190    }
191    if keyboard.just_pressed(KeyCode::Digit2) {
192        my_config.enabled ^= true;
193    }
194    if keyboard.just_pressed(KeyCode::KeyI) {
195        my_config.line.style = match my_config.line.style {
196            GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
197            GizmoLineStyle::Dotted => GizmoLineStyle::Dashed {
198                gap_scale: 3.0,
199                line_scale: 5.0,
200            },
201            _ => GizmoLineStyle::Solid,
202        };
203    }
204    if keyboard.just_pressed(KeyCode::KeyK) {
205        my_config.line.joints = match my_config.line.joints {
206            GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
207            GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
208            GizmoLineJoint::Round(_) => GizmoLineJoint::None,
209            GizmoLineJoint::None => GizmoLineJoint::Bevel,
210        };
211    }
212    if keyboard.just_pressed(KeyCode::Space) {
213        virtual_time.toggle();
214    }
215}
```

examples/gizmos/3d\_gizmos.rs ([line 295](../../src/3d_gizmos/3d_gizmos.rs.html#295))

```rust
207fn update_config(
208    mut config_store: ResMut<GizmoConfigStore>,
209    keyboard: Res<ButtonInput<KeyCode>>,
210    real_time: Res<Time<Real>>,
211    mut virtual_time: ResMut<Time<Virtual>>,
212) {
213    if keyboard.just_pressed(KeyCode::KeyT) {
214        for (_, config, _) in config_store.iter_mut() {
215            config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
216        }
217    }
218    if keyboard.just_pressed(KeyCode::KeyP) {
219        for (_, config, _) in config_store.iter_mut() {
220            // Toggle line perspective
221            config.line.perspective ^= true;
222            // Increase the line width when line perspective is on
223            config.line.width *= if config.line.perspective { 5. } else { 1. / 5. };
224        }
225    }
226
227    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
228    if keyboard.pressed(KeyCode::ArrowRight) {
229        config.line.width += 5. * real_time.delta_secs();
230        config.line.width = config.line.width.clamp(0., 50.);
231    }
232    if keyboard.pressed(KeyCode::ArrowLeft) {
233        config.line.width -= 5. * real_time.delta_secs();
234        config.line.width = config.line.width.clamp(0., 50.);
235    }
236    if keyboard.just_pressed(KeyCode::Digit1) {
237        config.enabled ^= true;
238    }
239    if keyboard.just_pressed(KeyCode::KeyU) {
240        config.line.style = match config.line.style {
241            GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
242            GizmoLineStyle::Dotted => GizmoLineStyle::Dashed {
243                gap_scale: 3.0,
244                line_scale: 5.0,
245            },
246            _ => GizmoLineStyle::Solid,
247        };
248    }
249    if keyboard.just_pressed(KeyCode::KeyJ) {
250        config.line.joints = match config.line.joints {
251            GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
252            GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
253            GizmoLineJoint::Round(_) => GizmoLineJoint::None,
254            GizmoLineJoint::None => GizmoLineJoint::Bevel,
255        };
256    }
257
258    let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
259    if keyboard.pressed(KeyCode::ArrowUp) {
260        my_config.line.width += 5. * real_time.delta_secs();
261        my_config.line.width = my_config.line.width.clamp(0., 50.);
262    }
263    if keyboard.pressed(KeyCode::ArrowDown) {
264        my_config.line.width -= 5. * real_time.delta_secs();
265        my_config.line.width = my_config.line.width.clamp(0., 50.);
266    }
267    if keyboard.just_pressed(KeyCode::Digit2) {
268        my_config.enabled ^= true;
269    }
270    if keyboard.just_pressed(KeyCode::KeyI) {
271        my_config.line.style = match my_config.line.style {
272            GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
273            GizmoLineStyle::Dotted => GizmoLineStyle::Dashed {
274                gap_scale: 3.0,
275                line_scale: 5.0,
276            },
277            _ => GizmoLineStyle::Solid,
278        };
279    }
280    if keyboard.just_pressed(KeyCode::KeyK) {
281        my_config.line.joints = match my_config.line.joints {
282            GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
283            GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
284            GizmoLineJoint::Round(_) => GizmoLineJoint::None,
285            GizmoLineJoint::None => GizmoLineJoint::Bevel,
286        };
287    }
288
289    if keyboard.just_pressed(KeyCode::KeyB) {
290        // AABB gizmos are normally only drawn on entities with a ShowAabbGizmo component
291        // We can change this behavior in the configuration of AabbGizmoGroup
292        config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
293    }
294    if keyboard.just_pressed(KeyCode::Space) {
295        virtual_time.toggle();
296    }
297}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#215)

#### pub fn [pause](#method.pause)(&mut self)

Stops the clock, preventing it from advancing until resumed.

##### [Examples found in repository](#scraped-examples-16)[?](../../scrape-examples-help.html)

examples/time/time.rs ([line 68](../../src/time/time.rs.html#68))

```rust
35fn runner(mut app: App) -> AppExit {
36    banner();
37    help();
38    let stdin = io::stdin();
39    for line in stdin.lock().lines() {
40        if let Err(err) = line {
41            println!("read err: {err:#}");
42            break;
43        }
44        match line.unwrap().as_str() {
45            "" => {
46                app.update();
47            }
48            "f" => {
49                println!("FAST: setting relative speed to 2x");
50                app.world_mut()
51                    .resource_mut::<Time<Virtual>>()
52                    .set_relative_speed(2.0);
53            }
54            "n" => {
55                println!("NORMAL: setting relative speed to 1x");
56                app.world_mut()
57                    .resource_mut::<Time<Virtual>>()
58                    .set_relative_speed(1.0);
59            }
60            "s" => {
61                println!("SLOW: setting relative speed to 0.5x");
62                app.world_mut()
63                    .resource_mut::<Time<Virtual>>()
64                    .set_relative_speed(0.5);
65            }
66            "p" => {
67                println!("PAUSE: pausing virtual clock");
68                app.world_mut().resource_mut::<Time<Virtual>>().pause();
69            }
70            "u" => {
71                println!("UNPAUSE: resuming virtual clock");
72                app.world_mut().resource_mut::<Time<Virtual>>().unpause();
73            }
74            "q" => {
75                println!("QUITTING!");
76                break;
77            }
78            _ => {
79                help();
80            }
81        }
82    }
83
84    AppExit::Success
85}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#221)

#### pub fn [unpause](#method.unpause)(&mut self)

Resumes the clock.

##### [Examples found in repository](#scraped-examples-17)[?](../../scrape-examples-help.html)

examples/time/time.rs ([line 72](../../src/time/time.rs.html#72))

```rust
35fn runner(mut app: App) -> AppExit {
36    banner();
37    help();
38    let stdin = io::stdin();
39    for line in stdin.lock().lines() {
40        if let Err(err) = line {
41            println!("read err: {err:#}");
42            break;
43        }
44        match line.unwrap().as_str() {
45            "" => {
46                app.update();
47            }
48            "f" => {
49                println!("FAST: setting relative speed to 2x");
50                app.world_mut()
51                    .resource_mut::<Time<Virtual>>()
52                    .set_relative_speed(2.0);
53            }
54            "n" => {
55                println!("NORMAL: setting relative speed to 1x");
56                app.world_mut()
57                    .resource_mut::<Time<Virtual>>()
58                    .set_relative_speed(1.0);
59            }
60            "s" => {
61                println!("SLOW: setting relative speed to 0.5x");
62                app.world_mut()
63                    .resource_mut::<Time<Virtual>>()
64                    .set_relative_speed(0.5);
65            }
66            "p" => {
67                println!("PAUSE: pausing virtual clock");
68                app.world_mut().resource_mut::<Time<Virtual>>().pause();
69            }
70            "u" => {
71                println!("UNPAUSE: resuming virtual clock");
72                app.world_mut().resource_mut::<Time<Virtual>>().unpause();
73            }
74            "q" => {
75                println!("QUITTING!");
76                break;
77            }
78            _ => {
79                help();
80            }
81        }
82    }
83
84    AppExit::Success
85}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#227)

#### pub fn [is\_paused](#method.is_paused)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the clock is currently paused.

##### [Examples found in repository](#scraped-examples-18)[?](../../scrape-examples-help.html)

examples/3d/solari.rs ([line 540](../../src/solari/solari.rs.html#540))

```rust
528fn update_control_text(
529    mut text: Single<&mut Text, With<ControlText>>,
530    robot_light_material: Option<Res<RobotLightMaterial>>,
531    materials: Res<Assets<StandardMaterial>>,
532    directional_light: Query<Entity, With<DirectionalLight>>,
533    time: Res<Time<Virtual>>,
534    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))] dlss_rr_supported: Option<
535        Res<DlssRayReconstructionSupported>,
536    >,
537) {
538    text.0.clear();
539
540    if time.is_paused() {
541        text.0.push_str("(Space): Resume");
542    } else {
543        text.0.push_str("(Space): Pause");
544    }
545
546    if directional_light.single().is_ok() {
547        text.0.push_str("\n(1): Disable directional light");
548    } else {
549        text.0.push_str("\n(1): Enable directional light");
550    }
551
552    match robot_light_material.and_then(|m| materials.get(&m.0)) {
553        Some(robot_light_material) if robot_light_material.emissive != LinearRgba::BLACK => {
554            text.0.push_str("\n(2): Disable robot emissive light");
555        }
556        _ => {
557            text.0.push_str("\n(2): Enable robot emissive light");
558        }
559    }
560
561    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))]
562    if dlss_rr_supported.is_some() {
563        text.0
564            .push_str("\nDenoising: DLSS Ray Reconstruction enabled");
565    } else {
566        text.0
567            .push_str("\nDenoising: DLSS Ray Reconstruction not supported");
568    }
569
570    #[cfg(any(not(feature = "dlss"), feature = "force_disable_dlss"))]
571    text.0
572        .push_str("\nDenoising: App not compiled with DLSS support");
573}
```

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#233)

#### pub fn [was\_paused](#method.was_paused)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the clock was paused at the start of this update.

## Trait Implementations

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

### impl<T> [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Time](struct.Time.html "struct bevy::prelude::Time")<T>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

### impl<T> [Component](trait.Component.html "trait bevy::prelude::Component") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"), [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### const [STORAGE\_TYPE](trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::SparseSet

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### type [Mutability](trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](trait.Component.html "trait bevy::prelude::Component"). [Read more](trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### fn [register\_required\_components](trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### fn [clone\_behavior](trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### fn [relationship\_accessor](trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>>

Returns [`ComponentRelationshipAccessor`](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

### impl<T> [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

### impl<T> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#371)

### impl<T> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#372)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Time](struct.Time.html "struct bevy::prelude::Time")<T>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Time](struct.Time.html "struct bevy::prelude::Time")<T>

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Time](struct.Time.html "struct bevy::prelude::Time")<T> as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Time](struct.Time.html "struct bevy::prelude::Time")<T>: 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [reflect\_clone](trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#363)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Time](struct.Time.html "struct bevy::prelude::Time")<T>>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

### impl<T> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"), [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [field](trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [field\_mut](trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [field\_at](trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [field\_at\_mut](trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [name\_at](trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [index\_of\_name](trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [field\_len](trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [iter\_fields](trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [to\_dynamic\_struct](trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl<T> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

### impl<T> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<T> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

### impl<T> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

### impl<T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<T> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

### impl<T> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#722)

### impl<T> [ErasedBundleTemplate](../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#723)

#### unsafe fn [apply](../scene/trait.ErasedBundleTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../scene/trait.ErasedBundleTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#729)

#### fn [clone\_template](../scene/trait.ErasedBundleTemplate.html#tymethod.clone_template)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedBundleTemplate](../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}