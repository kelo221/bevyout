[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Function not 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1239-1242)

```rust
pub fn not<Marker, TOut, T>(
    condition: T,
) -> AdapterSystem<NotMarker, <T as IntoSystem<(), TOut, Marker>>::System>where
    TOut: Not,
    T: IntoSystem<(), TOut, Marker>,
```

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that inverses the result of passed one.

## Example

```rust
app.add_systems(
    // `not` will inverse any condition you pass in.
    // Since the condition we choose always returns true
    // this system will never run
    my_system.run_if(not(always)),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

fn always() -> bool {
    true
}

app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 0);
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 23](../../../src/animated_mesh_control/animated_mesh_control.rs.html#23))

```rust
12fn main() {
13    App::new()
14        .insert_resource(GlobalAmbientLight {
15            color: Color::WHITE,
16            brightness: 2000.,
17            ..default()
18        })
19        .add_plugins(DefaultPlugins)
20        .add_systems(Startup, setup)
21        .add_systems(
22            Update,
23            spawn_fox_asset_when_ready.run_if(not(resource_exists::<Animations>)),
24        )
25        .add_systems(
26            Update,
27            keyboard_control.run_if(resource_exists::<Animations>),
28        )
29        .run();
30}
```

Hide additional examples

examples/shader/gpu\_readback.rs ([line 55](../../../src/gpu_readback/gpu_readback.rs.html#55))

```rust
44    fn build(&self, app: &mut App) {
45        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
46            return;
47        };
48        render_app
49            .add_systems(RenderStartup, init_compute_pipeline)
50            .add_systems(
51                Render,
52                prepare_bind_group
53                    .in_set(RenderSystems::PrepareBindGroups)
54                    // We don't need to recreate the bind group every frame
55                    .run_if(not(resource_exists::<GpuBufferBindGroup>)),
56            )
57            .add_systems(RenderGraph, compute);
58    }
```

examples/showcase/stepping.rs ([line 63](../../../src/breakout/stepping.rs.html#63))

```rust
34    fn build(&self, app: &mut App) {
35        app.add_systems(Startup, build_stepping_hint);
36        if cfg!(not(feature = "bevy_debug_stepping")) {
37            return;
38        }
39
40        // create and insert our debug schedule into the main schedule order.
41        // We need an independent schedule so we have access to all other
42        // schedules through the `Stepping` resource
43        app.init_schedule(DebugSchedule);
44        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
45        order.insert_after(Update, DebugSchedule);
46
47        // create our stepping resource
48        let mut stepping = Stepping::new();
49        for label in &self.schedule_labels {
50            stepping.add_schedule(*label);
51        }
52        app.insert_resource(stepping);
53
54        // add our startup & stepping systems
55        app.insert_resource(State {
56            ui_top: self.top,
57            ui_left: self.left,
58            systems: Vec::new(),
59        })
60        .add_systems(
61            DebugSchedule,
62            (
63                build_ui.run_if(not(initialized)),
64                handle_input,
65                update_ui.run_if(initialized),
66            )
67                .chain(),
68        );
69    }
```

examples/ecs/run\_conditions.rs ([line 49](../../../src/run_conditions/run_conditions.rs.html#49))

```rust
5fn main() {
6    println!();
7    println!("For the first 2 seconds you will not be able to increment the counter");
8    println!("Once that time has passed you can press space, enter, left mouse, right mouse or touch the screen to increment the counter");
9    println!();
10
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .init_resource::<InputCounter>()
14        .add_systems(
15            Update,
16            (
17                increment_input_counter
18                    // The common_conditions module has a few useful run conditions
19                    // for checking resources and states. These are included in the prelude.
20                    .run_if(resource_exists::<InputCounter>)
21                    // `.or_else()` is a run condition combinator that only evaluates the second condition
22                    // if the first condition returns `false`. This behavior is known as "short-circuiting",
23                    // and is how the `||` operator works in Rust (as well as most C-family languages).
24                    // In this case, the `has_user_input` run condition will be evaluated since the `Unused` resource has not been initialized.
25                    .run_if(resource_exists::<Unused>.or_else(
26                        // This is a custom run condition, defined using a system that returns
27                        // a `bool` and which has read-only `SystemParam`s.
28                        // Only a single run condition must return `true` in order for the system to run.
29                        has_user_input,
30                    )),
31                print_input_counter
32                    // `.and_then()` is a run condition combinator that only evaluates the second condition
33                    // if the first condition returns `true`, analogous to the `&&` operator.
34                    // In this case, the short-circuiting behavior prevents the second run condition from
35                    // panicking if the `InputCounter` resource has not been initialized.
36                    .run_if(resource_exists::<InputCounter>.and_then(
37                        // This is a custom run condition in the form of a closure.
38                        // This is useful for small, simple run conditions you don't need to reuse.
39                        // All the normal rules still apply: all parameters must be read only except for local parameters.
40                        |counter: Res<InputCounter>| counter.is_changed() && !counter.is_added(),
41                    )),
42                print_time_message
43                    // This function returns a custom run condition, much like the common conditions module.
44                    // It will only return true once 2 seconds have passed.
45                    .run_if(time_passed(2.0))
46                    // You can use the `not` condition from the common_conditions module
47                    // to inverse a run condition. In this case it will return true if
48                    // less than 2.5 seconds have elapsed since the app started.
49                    .run_if(not(time_passed(2.5))),
50            ),
51        )
52        .run();
53}
```