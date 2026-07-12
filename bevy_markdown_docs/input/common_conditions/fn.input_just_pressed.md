[bevy](../../index.html)::[input](../index.html)::[common\_conditions](index.html)

# Function input\_just\_pressed 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/common_conditions.rs.html#88-90)

```rust
pub fn input_just_pressed<T>(
    input: T,
) -> impl FnMut(Res<'_, ButtonInput<T>>) + Clonewhere
    T: Clone + Eq + Hash + Send + Sync + 'static,
```

Run condition that is active if [`ButtonInput::just_pressed`](../../prelude/struct.ButtonInput.html#method.just_pressed "method bevy::prelude::ButtonInput::just_pressed") is true for the given input.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, jump.run_if(input_just_pressed(KeyCode::Space)))
        .run();
}
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/remote/server.rs ([line 21](../../../src/server/server.rs.html#21))

```rust
15fn main() {
16    App::new()
17        .add_plugins(DefaultPlugins)
18        .add_plugins(RemotePlugin::default())
19        .add_plugins(RemoteHttpPlugin::default())
20        .add_systems(Startup, setup)
21        .add_systems(Update, remove.run_if(input_just_pressed(KeyCode::Space)))
22        .add_systems(Update, move_cube)
23        .run();
24}
```

Hide additional examples

examples/asset/alter\_mesh.rs ([line 14](../../../src/alter_mesh/alter_mesh.rs.html#14))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins)
11        .add_systems(Startup, (setup, spawn_text))
12        .add_systems(
13            Update,
14            alter_handle.run_if(input_just_pressed(KeyCode::Space)),
15        )
16        .add_systems(
17            Update,
18            alter_mesh.run_if(input_just_pressed(KeyCode::Enter)),
19        )
20        .run();
21}
```

examples/asset/alter\_sprite.rs ([line 14](../../../src/alter_sprite/alter_sprite.rs.html#14))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins)
11        .add_systems(Startup, (setup, spawn_text))
12        .add_systems(
13            Update,
14            alter_handle.run_if(input_just_pressed(KeyCode::Space)),
15        )
16        .add_systems(
17            Update,
18            alter_asset.run_if(input_just_pressed(KeyCode::Enter)),
19        )
20        .run();
21}
```

examples/2d/2d\_shapes.rs ([line 34](../../../src/2d_shapes/2d_shapes.rs.html#34))

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

examples/asset/asset\_saving.rs ([line 31](../../../src/asset_saving/asset_saving.rs.html#31))

```rust
20fn main() {
21    App::new()
22        .add_plugins(DefaultPlugins.set(AssetPlugin {
23            // This is just overriding the default asset paths to scope this to the correct example
24            // folder. You can generally skip this in your own projects.
25            file_path: "examples/asset/saved_assets".to_string(),
26            ..Default::default()
27        }))
28        .add_plugins(image_drawing_plugin)
29        .add_systems(
30            PreUpdate,
31            perform_save.run_if(input_just_pressed(KeyCode::F5)),
32        )
33        .run();
34}
```

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 18](../../../src/overflow_debug/overflow_debug.rs.html#18))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .init_resource::<AnimationState>()
14        .add_systems(Startup, setup)
15        .add_systems(
16            Update,
17            (
18                toggle_overflow.run_if(input_just_pressed(KeyCode::KeyO)),
19                next_container_size.run_if(input_just_pressed(KeyCode::KeyS)),
20                update_transform::<Move>,
21                update_transform::<Scale>,
22                update_transform::<Rotate>,
23                update_animation,
24            ),
25        )
26        .run();
27}
```

Additional examples can be found in:  

*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#40)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#18)
*   [examples/camera/2d\_screen\_shake.rs](../../../src/2d_screen_shake/2d_screen_shake.rs.html#62)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#31)
*   [examples/picking/debug\_picking.rs](../../../src/debug_picking/debug_picking.rs.html#26-28)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#60)
*   [examples/time/virtual\_time.rs](../../../src/virtual_time/virtual_time.rs.html#20)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#41)
*   [examples/math/custom\_primitives.rs](../../../src/custom_primitives/custom_primitives.rs.html#157)
*   [examples/math/render\_primitives.rs](../../../src/render_primitives/render_primitives.rs.html#22)