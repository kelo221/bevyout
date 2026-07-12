[bevy](../../index.html)::[input](../index.html)::[common\_conditions](index.html)

# Function input\_just\_released 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/common_conditions.rs.html#96-98)

```rust
pub fn input_just_released<T>(
    input: T,
) -> impl FnMut(Res<'_, ButtonInput<T>>) + Clonewhere
    T: Clone + Eq + Hash + Send + Sync + 'static,
```

Run condition that is active if [`ButtonInput::just_released`](../../prelude/struct.ButtonInput.html#method.just_released "method bevy::prelude::ButtonInput::just_released") is true for the given input.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/showcase/desk\_toy.rs ([line 42](../../../src/desk_toy/desk_toy.rs.html#42))

```rust
19fn main() {
20    App::new()
21        .add_plugins(DefaultPlugins.set(WindowPlugin {
22            primary_window: Some(Window {
23                title: "Bevy Desk Toy".into(),
24                transparent: true,
25                #[cfg(target_os = "macos")]
26                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
27                ..default()
28            }),
29            ..default()
30        }))
31        .insert_resource(ClearColor(WINDOW_CLEAR_COLOR))
32        .insert_resource(WindowTransparency(false))
33        .insert_resource(CursorWorldPos(None))
34        .add_systems(Startup, setup)
35        .add_systems(
36            Update,
37            (
38                get_cursor_world_pos,
39                update_cursor_hit_test,
40                (
41                    start_drag.run_if(input_just_pressed(MouseButton::Left)),
42                    end_drag.run_if(input_just_released(MouseButton::Left)),
43                    drag.run_if(resource_exists::<DragOperation>),
44                    quit.run_if(input_just_pressed(MouseButton::Right)),
45                    toggle_transparency.run_if(input_just_pressed(KeyCode::Space)),
46                    move_pupils.after(drag),
47                ),
48            )
49                .chain(),
50        )
51        .run();
52}
```