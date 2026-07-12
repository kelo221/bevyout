[bevy](../index.html)::[color](index.html)

# Trait Hue 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#95)

```rust
pub trait Hue: Sized {
    // Required methods
    fn with_hue(&self, hue: f32) -> Self;
    fn hue(&self) -> f32;
    fn set_hue(&mut self, hue: f32);

    // Provided method
    fn rotate_hue(&self, degrees: f32) -> Self { ... }
}
```

Trait for manipulating the hue of a color.

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#97)

#### fn [with\_hue](#tymethod.with_hue)(&self, hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a new version of this color with the hue channel set to the given value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#100)

#### fn [hue](#tymethod.hue)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the hue of this color \[0.0, 360.0\].

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#103)

#### fn [set\_hue](#tymethod.set_hue)(&mut self, hue: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the hue of this color.

## Provided Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#106)

#### fn [rotate\_hue](#method.rotate_hue)(&self, degrees: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a new version of this color with the hue channel rotated by the given degrees.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/asset/asset\_saving\_with\_subassets.rs ([line 300](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#300))

```rust
297fn rotate_hue(time: Res<Time>, mut sprites: Query<&mut Sprite, With<RotateHue>>) {
298    for mut sprite in sprites.iter_mut() {
299        // Make a full rotation every 2 seconds.
300        sprite.color = sprite.color.rotate_hue(time.delta_secs() * 180.0);
301    }
302}
```

Hide additional examples

examples/3d/animated\_material.rs ([line 42](../../src/animated_material/animated_material.rs.html#42))

```rust
13fn setup(
14    mut commands: Commands,
15    asset_server: Res<AssetServer>,
16    mut meshes: ResMut<Assets<Mesh>>,
17    mut materials: ResMut<Assets<StandardMaterial>>,
18) {
19    commands.spawn((
20        Camera3d::default(),
21        Transform::from_xyz(3.0, 1.0, 3.0).looking_at(Vec3::new(0.0, -0.5, 0.0), Vec3::Y),
22        EnvironmentMapLight {
23            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
24            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
25            intensity: 2_000.0,
26            ..default()
27        },
28    ));
29
30    let cube = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
31
32    const GOLDEN_ANGLE: f32 = 137.507_77;
33
34    let mut hsla = Hsla::hsl(0.0, 1.0, 0.5);
35    for x in -1..2 {
36        for z in -1..2 {
37            commands.spawn((
38                Mesh3d(cube.clone()),
39                MeshMaterial3d(materials.add(Color::from(hsla))),
40                Transform::from_translation(Vec3::new(x as f32, 0.0, z as f32)),
41            ));
42            hsla = hsla.rotate_hue(GOLDEN_ANGLE);
43        }
44    }
45}
46
47fn animate_materials(
48    material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
49    time: Res<Time>,
50    mut materials: ResMut<Assets<StandardMaterial>>,
51) {
52    for material_handle in material_handles.iter() {
53        if let Some(mut material) = materials.get_mut(material_handle)
54            && let Color::Hsla(ref mut hsla) = material.base_color
55        {
56            *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
57        }
58    }
59}
```

examples/gltf/query\_gltf\_primitives.rs ([line 31](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#31))

```rust
16fn find_top_material_and_mesh(
17    mut materials: ResMut<Assets<StandardMaterial>>,
18    mut meshes: ResMut<Assets<Mesh>>,
19    time: Res<Time>,
20    mat_query: Query<(
21        &MeshMaterial3d<StandardMaterial>,
22        &Mesh3d,
23        &GltfMaterialName,
24    )>,
25) {
26    for (mat_handle, mesh_handle, name) in mat_query.iter() {
27        // locate a material by material name
28        if name.0 == "Top" {
29            if let Some(mut material) = materials.get_mut(mat_handle) {
30                if let Color::Hsla(ref mut hsla) = material.base_color {
31                    *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
32                } else {
33                    material.base_color = Color::from(Hsla::hsl(0.0, 0.9, 0.7));
34                }
35            }
36
37            if let Some(mut mesh) = meshes.get_mut(mesh_handle)
38                && let Some(VertexAttributeValues::Float32x3(positions)) =
39                    mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
40            {
41                for position in positions {
42                    *position = (
43                        position[0],
44                        1.5 + 0.5 * ops::sin(time.elapsed_secs() / 2.0),
45                        position[2],
46                    )
47                        .into();
48                }
49            }
50        }
51    }
52}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#817)

### impl [Hue](../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Color](../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#149)

### impl [Hue](../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Hsla](../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#120)

### impl [Hue](../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Hsva](../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#123)

### impl [Hue](../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Hwba](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#153)

### impl [Hue](../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Lcha](../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#148)

### impl [Hue](../prelude/trait.Hue.html "trait bevy::prelude::Hue") for [Oklcha](../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")