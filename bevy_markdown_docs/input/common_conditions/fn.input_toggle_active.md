[bevy](../../index.html)::[input](../index.html)::[common\_conditions](index.html)

# Function input\_toggle\_active 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/common_conditions.rs.html#51-56)

```rust
pub fn input_toggle_active<T>(
    default: bool,
    input: T,
) -> impl FnMut(Res<'_, ButtonInput<T>>) + Clonewhere
    T: Clone + Eq + Hash + Send + Sync + 'static,
```

Stateful run condition that can be toggled via an input press using [`ButtonInput::just_pressed`](../../prelude/struct.ButtonInput.html#method.just_pressed "method bevy::prelude::ButtonInput::just_pressed").

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, pause_menu.run_if(input_toggle_active(false, KeyCode::Escape)))
        .run();
}

fn pause_menu() {
    println!("in pause menu");
}
```

If you want other systems to be able to access whether the toggled state is active, you should use a custom resource or a state for that:

```rust
#[derive(Resource, Default)]
struct Paused(bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Paused>()
        .add_systems(Update, toggle_pause_state.run_if(input_just_pressed(KeyCode::Escape)))
        .add_systems(Update, pause_menu.run_if(|paused: Res<Paused>| paused.0))
        .run();
}

fn toggle_pause_state(mut paused: ResMut<Paused>) {
    paused.0 = !paused.0;
}

fn pause_menu() {
    println!("in pause menu");
}
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/2d/2d\_shapes.rs ([line 38](../../../src/2d_shapes/2d_shapes.rs.html#38))

```rust
23fn main() {
24    let mut app = App::new();
25    app.add_plugins((
26        DefaultPlugins,
27        #[cfg(not(target_arch = "wasm32"))]
28        Wireframe2dPlugin::default(),
29    ))
30    .add_systems(Startup, setup);
31    #[cfg(not(target_arch = "wasm32"))]
32    app.add_systems(
33        Update,
34        toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)),
35    );
36    app.add_systems(
37        Update,
38        rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
39    );
40    app.run();
41}
```

Hide additional examples

examples/3d/3d\_shapes.rs ([line 39](../../../src/3d_shapes/3d_shapes.rs.html#39))

```rust
28fn main() {
29    App::new()
30        .add_plugins((
31            DefaultPlugins.set(ImagePlugin::default_nearest()),
32            #[cfg(not(target_arch = "wasm32"))]
33            WireframePlugin::default(),
34        ))
35        .add_systems(Startup, setup)
36        .add_systems(
37            Update,
38            (
39                rotate.run_if(input_toggle_active(true, KeyCode::KeyR)),
40                advance_rows.run_if(input_just_pressed(KeyCode::Tab)),
41                #[cfg(not(target_arch = "wasm32"))]
42                toggle_wireframe,
43            ),
44        )
45        .run();
46}
```

examples/math/custom\_primitives.rs ([line 145](../../../src/custom_primitives/custom_primitives.rs.html#145))

```rust
130fn main() {
131    let mut app = App::new();
132
133    app.add_plugins(DefaultPlugins);
134
135    #[cfg(not(target_family = "wasm"))]
136    app.add_plugins(WireframePlugin::default());
137
138    app.init_state::<BoundingShape>()
139        .init_state::<ShapeActive>()
140        .add_systems(Startup, setup)
141        .add_systems(
142            Update,
143            (
144                (
145                    rotate_2d_shapes.run_if(input_toggle_active(true, KeyCode::KeyR)),
146                    bounding_shapes_2d,
147                )
148                    .run_if(state_in_one_of([ShapeActive::Heart, ShapeActive::Ring])),
149                (
150                    rotate_3d_shapes.run_if(input_toggle_active(true, KeyCode::KeyR)),
151                    bounding_shapes_3d,
152                )
153                    .run_if(state_in_one_of([
154                        ShapeActive::Extrusion,
155                        ShapeActive::RingExtrusion,
156                    ])),
157                update_bounding_shape.run_if(input_just_pressed(KeyCode::KeyB)),
158                switch_shapes.run_if(input_just_pressed(KeyCode::Tab)),
159            ),
160        );
161
162    #[cfg(not(target_family = "wasm"))]
163    app.add_systems(
164        Update,
165        toggle_wireframes.run_if(input_just_pressed(KeyCode::Space)),
166    );
167
168    app.run();
169}
```