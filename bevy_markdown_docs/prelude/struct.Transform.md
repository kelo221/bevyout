[bevy](../index.html)::[prelude](index.html)

# Struct Transform 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#86)

```rust
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
```

Describe the position of an entity. If the entity has a parent, the position is relative to its parent position.

*   To place or move an entity, you should set its [`Transform`](struct.Transform.html "struct bevy::prelude::Transform").
*   To get the global transform of an entity, you should get its [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").
*   To be displayed, an entity must have both a [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") and a [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform"). [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is automatically inserted whenever [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") is inserted.

Transforms compose from right to left: if `t1` and `t2` are transforms, then `t1 * t2` corresponds to applying `t2` _first_, _then_ applying `t1`.

### [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") and [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[`Transform`](struct.Transform.html "struct bevy::prelude::Transform") is the position of an entity relative to its parent position, or the reference frame if it doesn’t have a [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") component.

[`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is the position of an entity relative to the reference frame.

[`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is updated from [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") in the [`TransformSystems::Propagate`](enum.TransformSystems.html#variant.Propagate "variant bevy::prelude::TransformSystems::Propagate") system set.

This system runs during [`PostUpdate`](struct.PostUpdate.html "struct bevy::prelude::PostUpdate"). If you update the [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") of an entity during this set or after, you will notice a 1 frame lag before the [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is updated.

## Examples

*   [`transform`](https://github.com/bevyengine/bevy/blob/latest/examples/transforms/transform.rs)

## Fields

`translation: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")`

Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.

See the [`translations`](https://github.com/bevyengine/bevy/blob/latest/examples/transforms/translation.rs) example for usage.

`rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat")`

Rotation of the entity.

See the [`3d_rotation`](https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs) example for usage.

`scale: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")`

Scale of the entity.

See the [`scale`](https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs) example for usage.

## Implementations

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#107)

### impl [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#109)

#### pub const [IDENTITY](#associatedconstant.IDENTITY): [Transform](struct.Transform.html "struct bevy::prelude::Transform")

An identity [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") with no translation, rotation, and a scale of 1 on all axes.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#119)

#### pub const fn [from\_xyz](#method.from_xyz)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Creates a new [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") at the position `(x, y, z)`. In 2d, the `z` component is used for z-ordering elements: higher `z`\-value will be in front of lower `z`\-value.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/usage/debug\_frustum\_culling.rs ([line 99](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#99))

```rust
99const FREE_CAMERA_START_TRANSFORM: Transform = Transform::from_xyz(-20., 10., 22.);
100const FREE_CAMERA_START_TARGET: Vec3 = Vec3::new(7., 1.5, 0.);
101
102fn setup(
103    mut commands: Commands,
104    windows: Query<&Window>,
105    mut config_store: ResMut<GizmoConfigStore>,
106    mut meshes: ResMut<Assets<Mesh>>,
107    mut materials: ResMut<Assets<StandardMaterial>>,
108) -> Result {
109    let window = windows.single()?;
110    // The camera that the user controls to observe the scene.
111    let free_camera = commands
112        .spawn((
113            Camera3d::default(),
114            FREE_CAMERA_START_TRANSFORM.looking_at(FREE_CAMERA_START_TARGET, Vec3::Y),
115            FreeCamera::default(),
116        ))
117        .id();
118
119    // The camera that we want to debug frustum culling for. This will be rendered
120    // as a picture-in-picture in the lower right ninth of the screen.
121    let my_camera = commands
122        .spawn((
123            Camera3d::default(),
124            Transform::from_xyz(0., 1.5, 0.).looking_at(Vec3::new(1.0, 1.5, 0.), Vec3::Y),
125            Camera {
126                order: 1,
127                // The camera-to-debug's view will be in the lower right ninth of the screen.
128                viewport: Some(Viewport {
129                    physical_position: window.physical_size() * 2 / 3,
130                    physical_size: window.physical_size() / 3,
131                    ..default()
132                }),
133                // Do not write the free camera's view rendering back into the P-I-P
134                msaa_writeback: MsaaWriteback::Off,
135                ..default()
136            },
137            MyCamera,
138        ))
139        .id();
140
141    // Instructions placed on top of the free_camera view
142    commands.spawn((
143        UiTargetCamera(free_camera),
144        Node {
145            width: percent(100),
146            height: percent(100),
147            ..default()
148        },
149        children![(
150            Text::new(
151                "This example utilizes free camera controls i.e. move with WASD and mouse grab to change orientation.\n\
152                Press '1' to move the free camera to where MyCamera is, matching its view frustum.\n\
153                Press '2' to move the free camera to its initial position in the example.",
154            ),
155            Node {
156                position_type: PositionType::Absolute,
157                top: px(12),
158                left: px(12),
159                ..default()
160            },
161        )]
162    ));
163    // Label for the picture-in-picture view of MyCamera
164    commands.spawn((
165        UiTargetCamera(my_camera),
166        Node {
167            width: percent(100),
168            height: percent(100),
169            ..default()
170        },
171        children![(
172            Text::new("View of MyCamera"),
173            Node {
174                position_type: PositionType::Absolute,
175                bottom: px(12),
176                right: px(100),
177                ..default()
178            },
179        )],
180    ));
181
182    // Green Floor Plane
183    commands.spawn((
184        Mesh3d(
185            meshes.add(
186                Plane3d::default()
187                    .mesh()
188                    .size(SHAPE_RING_RADIUS * 4., SHAPE_RING_RADIUS * 4.),
189            ),
190        ),
191        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
192    ));
193    // Blue Wall Plane
194    commands.spawn((
195        Mesh3d(meshes.add(Plane3d::default().mesh().size(5., 5.))),
196        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.5))),
197        Transform::from_xyz(20., 2.5, 10.).with_rotation(Quat::from_rotation_z(PI / 2.)),
198    ));
199    // Light
200    commands.spawn((
201        PointLight {
202            shadow_maps_enabled: true,
203            ..default()
204        },
205        Transform::from_xyz(0.0, 10.0, 0.0),
206    ));
207
208    // Configure all AABB's to have a default color of red
209    let (_, aabb_gizmo_config) = config_store.config_mut::<AabbGizmoConfigGroup>();
210    aabb_gizmo_config.default_color = Some(Color::LinearRgba(LinearRgba::RED));
211
212    // Configure the shapes on the ring that will have their AABB's drawn and updated
213    let white_matl = materials.add(Color::WHITE);
214    let shapes = [
215        meshes.add(Cuboid {
216            half_size: Vec3::new(2., 0.5, 1.),
217        }),
218        meshes.add(Tetrahedron {
219            vertices: [
220                Vec3::new(3., 4., 3.),
221                Vec3::new(-0.5, 4., -0.5),
222                Vec3::new(-0.5, -0.5, 3.),
223                Vec3::new(3., -0.5, -0.5),
224            ],
225        }),
226        meshes.add(Cylinder {
227            radius: 0.1,
228            half_height: 1.5,
229        }),
230        meshes.add(Cuboid {
231            half_size: Vec3::new(1., 0.1, 2.),
232        }),
233        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
234    ];
235    let shapes_len = shapes.len() as f32;
236    let mut shape_ring = commands.spawn((Transform::default(), Visibility::default(), ShapeRing));
237    for (i, shape) in shapes.into_iter().enumerate() {
238        // Space the shapes out evenly along the ring
239        let shape_angle = i as f32 * 2. * PI / shapes_len;
240        let (s, c) = ops::sin_cos(shape_angle);
241        let (x, z) = (SHAPE_RING_RADIUS * c, SHAPE_RING_RADIUS * s);
242        shape_ring.with_child((
243            Mesh3d(shape),
244            MeshMaterial3d(white_matl.clone()),
245            Transform::from_xyz(x, 1.5, z).with_rotation(Quat::from_rotation_x(-PI / 4.)),
246            MyShape,
247        ));
248    }
249
250    // Configure the shape that peeks out of the wall plane
251    let wall_shape = meshes.add(Torus::default());
252    commands.spawn((
253        Mesh3d(wall_shape),
254        MeshMaterial3d(white_matl.clone()),
255        Transform::from_xyz(25., 1.5, 12.5).with_rotation(Quat::from_rotation_x(-PI / 4.)),
256        WallShape,
257    ));
258
259    Ok(())
260}
```

Hide additional examples

examples/3d/clustered\_decals.rs ([line 204](../../src/clustered_decals/clustered_decals.rs.html#204))

```rust
201fn spawn_light(commands: &mut Commands) {
202    commands.spawn((
203        DirectionalLight::default(),
204        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
205    ));
206}
207
208/// Spawns the camera.
209fn spawn_camera(commands: &mut Commands) {
210    commands
211        .spawn(Camera3d::default())
212        .insert(Transform::from_xyz(0.0, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
213        // Tag the camera with `Selection::Camera`.
214        .insert(Selection::Camera);
215}
```

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 53](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#53))

```rust
50fn setup(mut commands: Commands) {
51    commands.spawn((
52        Camera3d::default(),
53        Transform::from_xyz(0.0, 7.5, 18.0).looking_at(Vec3::new(0.0, 5.5, 0.0), Vec3::Y),
54    ));
55}
56
57#[derive(Component, Debug, Default)]
58struct PendingScene(Handle<Gltf>);
59
60#[derive(Component, Debug, Default)]
61struct PendingAnimation((Handle<AnimationGraph>, AnimationNodeIndex));
62
63fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
64    commands.spawn((
65        PendingScene(asset_server.load("models/animated/Fox.glb")),
66        Transform::from_xyz(1.3, 4.3, 0.0)
67            .with_scale(Vec3::splat(0.08))
68            .looking_to(-Vec3::X, Vec3::Y),
69    ));
70}
71
72fn spawn_scene(
73    mut commands: Commands,
74    query: Query<(Entity, &PendingScene)>,
75    assets: Res<Assets<Gltf>>,
76    mut graphs: ResMut<Assets<AnimationGraph>>,
77) {
78    for (entity, PendingScene(asset)) in query.iter() {
79        if let Some(gltf) = assets.get(asset)
80            && let Some(scene_handle) = gltf.scenes.first()
81            && let Some(animation_handle) = gltf.named_animations.get("Run")
82        {
83            let (graph, graph_node_index) = AnimationGraph::from_clip(animation_handle.clone());
84
85            commands
86                .entity(entity)
87                .remove::<PendingScene>()
88                .insert((
89                    WorldAssetRoot(scene_handle.clone()),
90                    PendingAnimation((graphs.add(graph), graph_node_index)),
91                ))
92                .observe(play_animation);
93        }
94    }
95}
96
97fn play_animation(
98    trigger: On<WorldInstanceReady>,
99    mut commands: Commands,
100    children: Query<&Children>,
101    animations: Query<&PendingAnimation>,
102    mut players: Query<&mut AnimationPlayer>,
103) {
104    if let Ok(PendingAnimation((graph_handle, graph_node_index))) = animations.get(trigger.entity) {
105        for child in children.iter_descendants(trigger.entity) {
106            if let Ok(mut player) = players.get_mut(child) {
107                player.play(*graph_node_index).set_speed(0.6).repeat();
108
109                commands
110                    .entity(child)
111                    .insert(AnimationGraphHandle(graph_handle.clone()));
112            }
113        }
114    }
115
116    commands.entity(trigger.entity).remove::<PendingAnimation>();
117}
118
119type CustomAnimationId = i8;
120
121#[derive(Component)]
122struct CustomAnimation(CustomAnimationId);
123
124fn spawn_custom_meshes(
125    mut commands: Commands,
126    mut mesh_assets: ResMut<Assets<Mesh>>,
127    mut material_assets: ResMut<Assets<StandardMaterial>>,
128    mut inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
129) {
130    let mesh_handle = mesh_assets.add(
131        Mesh::new(
132            PrimitiveTopology::TriangleStrip,
133            // Test that skinned mesh bounds work even if the mesh is render
134            // world only.
135            RenderAssetUsages::RENDER_WORLD,
136        )
137        .with_inserted_attribute(
138            Mesh::ATTRIBUTE_POSITION,
139            vec![
140                [-0.5, 0.0, 0.0],
141                [0.5, 0.0, 0.0],
142                [-0.5, 0.5, 0.0],
143                [0.5, 0.5, 0.0],
144                [-0.5, 1.0, 0.0],
145                [0.5, 1.0, 0.0],
146                [-0.5, 1.5, 0.0],
147                [0.5, 1.5, 0.0],
148                [-0.5, 2.0, 0.0],
149                [0.5, 2.0, 0.0],
150            ],
151        )
152        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10])
153        .with_inserted_attribute(
154            Mesh::ATTRIBUTE_JOINT_INDEX,
155            VertexAttributeValues::Uint16x4(vec![
156                [1, 0, 0, 0],
157                [1, 0, 0, 0],
158                [1, 2, 0, 0],
159                [1, 2, 0, 0],
160                [1, 2, 0, 0],
161                [1, 2, 0, 0],
162                [2, 1, 0, 0],
163                [2, 1, 0, 0],
164                [2, 0, 0, 0],
165                [2, 0, 0, 0],
166            ]),
167        )
168        .with_inserted_attribute(
169            Mesh::ATTRIBUTE_JOINT_WEIGHT,
170            vec![
171                [1.00, 0.00, 0.0, 0.0],
172                [1.00, 0.00, 0.0, 0.0],
173                [0.75, 0.25, 0.0, 0.0],
174                [0.75, 0.25, 0.0, 0.0],
175                [0.50, 0.50, 0.0, 0.0],
176                [0.50, 0.50, 0.0, 0.0],
177                [0.75, 0.25, 0.0, 0.0],
178                [0.75, 0.25, 0.0, 0.0],
179                [1.00, 0.00, 0.0, 0.0],
180                [1.00, 0.00, 0.0, 0.0],
181            ],
182        )
183        .with_generated_skinned_mesh_bounds()
184        .unwrap(),
185    );
186
187    let inverse_bindposes_handle = inverse_bindposes_assets.add(vec![
188        Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
189        Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
190        Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0)),
191    ]);
192
193    struct MeshInstance {
194        animations: [CustomAnimationId; 2],
195    }
196
197    let mesh_instances = [
198        // Simple cases. First joint is still, second joint is all rotation/translation/scale variations.
199        MeshInstance { animations: [0, 1] },
200        MeshInstance { animations: [0, 2] },
201        MeshInstance { animations: [0, 3] },
202        MeshInstance { animations: [0, 4] },
203        MeshInstance { animations: [0, 5] },
204        MeshInstance { animations: [0, 6] },
205        MeshInstance { animations: [0, 7] },
206        MeshInstance { animations: [0, 8] },
207        // Skewed cases. First joint is non-uniform scaling, second joint is rotation/translation variations.
208        MeshInstance { animations: [9, 1] },
209        MeshInstance { animations: [9, 2] },
210        MeshInstance { animations: [9, 3] },
211        MeshInstance { animations: [9, 4] },
212        MeshInstance { animations: [9, 5] },
213    ];
214
215    for (i, mesh_instance) in mesh_instances.iter().enumerate() {
216        let x = ((i as f32) * 2.0) - ((mesh_instances.len() - 1) as f32);
217
218        let base_entity = commands
219            .spawn((Transform::from_xyz(x, 0.0, 0.0), Visibility::default()))
220            .id();
221
222        let joints = vec![
223            commands.spawn((Transform::IDENTITY,)).id(),
224            commands
225                .spawn((
226                    CustomAnimation(mesh_instance.animations[0]),
227                    Transform::IDENTITY,
228                ))
229                .id(),
230            commands
231                .spawn((
232                    CustomAnimation(mesh_instance.animations[1]),
233                    Transform::IDENTITY,
234                ))
235                .id(),
236        ];
237
238        commands.entity(joints[0]).insert(ChildOf(base_entity));
239
240        commands.entity(joints[1]).insert(ChildOf(joints[0]));
241        commands.entity(joints[2]).insert(ChildOf(joints[1]));
242
243        let mesh_entity = commands
244            .spawn((
245                Transform::IDENTITY,
246                Mesh3d(mesh_handle.clone()),
247                MeshMaterial3d(material_assets.add(StandardMaterial {
248                    base_color: Color::WHITE,
249                    cull_mode: None,
250                    ..default()
251                })),
252                SkinnedMesh {
253                    inverse_bindposes: inverse_bindposes_handle.clone(),
254                    joints: joints.clone(),
255                },
256                DynamicSkinnedMeshBounds,
257            ))
258            .id();
259
260        commands.entity(mesh_entity).insert(ChildOf(base_entity));
261    }
262}
```

examples/3d/mixed\_lighting.rs ([line 160](../../src/mixed_lighting/mixed_lighting.rs.html#160))

```rust
157fn spawn_camera(commands: &mut Commands) {
158    commands
159        .spawn(Camera3d::default())
160        .insert(Transform::from_xyz(-0.7, 0.7, 1.0).looking_at(vec3(0.0, 0.3, 0.0), Vec3::Y));
161}
```

examples/3d/clustered\_decal\_maps.rs ([line 235](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#235))

```rust
228fn spawn_light(commands: &mut Commands) {
229    commands.spawn((
230        PointLight {
231            intensity: 10_000_000.,
232            range: 100.0,
233            ..default()
234        },
235        Transform::from_xyz(8.0, 16.0, -8.0),
236    ));
237}
238
239/// Spawns a camera.
240fn spawn_camera(commands: &mut Commands) {
241    commands.spawn((
242        Camera3d::default(),
243        Transform::from_xyz(2.0, 0.0, -7.0).looking_at(Vec3::ZERO, Vec3::Y),
244        Hdr,
245    ));
246}
```

examples/3d/pccm.rs ([line 98](../../src/pccm/pccm.rs.html#98))

```rust
94fn spawn_camera(commands: &mut Commands) {
95    commands.spawn((
96        Camera3d::default(),
97        FreeCamera::default(),
98        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::new(0.0, -2.5, 0.0), Dir3::Y),
99        Hdr,
100    ));
101}
102
103/// Spawns the inner reflective cube in the scene.
104fn spawn_inner_cube(
105    commands: &mut Commands,
106    meshes: &mut Assets<Mesh>,
107    materials: &mut Assets<StandardMaterial>,
108) {
109    let cube_mesh = meshes.add(
110        Cuboid {
111            half_size: Vec3::new(5.0, 1.0, 2.0),
112        }
113        .mesh()
114        .build()
115        .with_duplicated_vertices()
116        .with_computed_flat_normals(),
117    );
118    let cube_material = materials.add(StandardMaterial {
119        base_color: Color::WHITE,
120        metallic: 1.0,
121        reflectance: 1.0,
122        perceptual_roughness: 0.0,
123        ..default()
124    });
125
126    commands.spawn((
127        Mesh3d(cube_mesh),
128        MeshMaterial3d(cube_material),
129        Transform::from_xyz(0.0, -4.0, -2.5),
130        InnerCube,
131    ));
132}
```

Additional examples can be found in:  

*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#370)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#215)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#24)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#128)
*   [examples/animation/color\_animation.rs](../../src/color_animation/color_animation.rs.html#76)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#237)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#27)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#17)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#58)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#306)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#85)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#129)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#24)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#98-102)
*   [examples/shader/animate\_shader.rs](../../src/animate_shader/animate_shader.rs.html#27)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#57)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#49)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#63)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#272)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#203)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#47)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#55)
*   [examples/shader/fallback\_image.rs](../../src/fallback_image/fallback_image.rs.html#43)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#202)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#230)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#63)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#79)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#310)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#32)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#33)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#61)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#58)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#143)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#30)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#196)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#225)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#326)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#20)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#37)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#53)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#49)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#54)
*   [examples/3d/3d\_scene.rs](../../src/3d_scene/3d_scene.rs.html#35)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#44)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#120)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#29)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#51)
*   [examples/shader/shader\_defs.rs](../../src/shader_defs/shader_defs.rs.html#37)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#22)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#109)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#140)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#41)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#229)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#83)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#88)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#32)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#76)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#194)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#54)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#64)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#24)
*   [examples/remote/server.rs](../../src/server/server.rs.html#42)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#128)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#112)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#64)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#21)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#162)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#267)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#130-135)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#121)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#63)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#84)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#21)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#28)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#62)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#97)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#73)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#133)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#34)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#92)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#39)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#20)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#68)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#35)
*   [examples/3d/lines.rs](../../src/lines/lines.rs.html#38)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#54)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#181)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#30)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#51)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#33)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#49)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#64)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#104)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#54)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#52)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#51)
*   [examples/shader/extended\_material.rs](../../src/extended_material/extended_material.rs.html#48)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#41)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#54)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#19)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#82)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#46)
*   [examples/3d/orthographic.rs](../../src/orthographic/orthographic.rs.html#28)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#99-103)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#144-149)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#76)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#89)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#21)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#24)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#206)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#212)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#79)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#110)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#55)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#97)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#183)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#32)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#146)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#103)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#49)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#62)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#274)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#31)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#65)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#80)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#71)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#64)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#85)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#72)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#50)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#84)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#64)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#165)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#662)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#71)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#51)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#51)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#59)
*   [examples/async\_tasks/async\_compute.rs](../../src/async_compute/async_compute.rs.html#82)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#46)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#65)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#34)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#435)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#49)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#58)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#49)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#64)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#150)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#61)
*   [examples/2d/mesh2d\_alpha\_mode.rs](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#39)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#50)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#118)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#127)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#98)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#98)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#63)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#40)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#64)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#32)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#60)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#67)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#111)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#37)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#188)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#43)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#89-93)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#79)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#111)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#60)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#230)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#49)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#47)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#88-93)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#84-88)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#147)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#118)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#221)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#128)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#183)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#37)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#112)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#138-142)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#48)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#45)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#32)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#60)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#31)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#76)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#273)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#126)

#### pub fn [from\_matrix](#method.from_matrix)(world\_from\_local: [Mat4](struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine transformation matrix.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([line 248](../../src/irradiance_volumes/irradiance_volumes.rs.html#248))

```rust
246fn spawn_irradiance_volume(commands: &mut Commands, assets: &ExampleAssets) {
247    commands.spawn((
248        Transform::from_matrix(VOXEL_FROM_WORLD),
249        IrradianceVolume {
250            voxels: assets.irradiance_volume.clone(),
251            intensity: IRRADIANCE_VOLUME_INTENSITY,
252            ..default()
253        },
254    ));
255}
```

Hide additional examples

examples/3d/mirror.rs ([lines 354-356](../../src/mirror/mirror.rs.html#354-356))

```rust
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#139)

#### pub const fn [from\_translation](#method.from_translation)(translation: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Creates a new [`Transform`](struct.Transform.html "struct bevy::prelude::Transform"), with `translation`. Rotation will be 0 and scale 1 on all axes.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_sprite\_meshes.rs ([line 108](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#108))

```rust
105fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
106    camera_transform.rotate_z(time.delta_secs() * 0.5);
107    **camera_transform = **camera_transform
108        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
109}
```

Hide additional examples

examples/stress\_tests/many\_sprites.rs ([line 106](../../src/many_sprites/many_sprites.rs.html#106))

```rust
103fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
104    camera_transform.rotate_z(time.delta_secs() * 0.5);
105    **camera_transform = **camera_transform
106        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
107}
```

examples/stress\_tests/many\_animated\_sprite\_meshes.rs ([line 104](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#104))

```rust
101fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
102    camera_transform.rotate(Quat::from_rotation_z(time.delta_secs() * 0.5));
103    **camera_transform = **camera_transform
104        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
105}
```

examples/stress\_tests/many\_animated\_sprites.rs ([line 102](../../src/many_animated_sprites/many_animated_sprites.rs.html#102))

```rust
99fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
100    camera_transform.rotate(Quat::from_rotation_z(time.delta_secs() * 0.5));
101    **camera_transform = **camera_transform
102        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
103}
```

examples/math/render\_primitives.rs ([line 320](../../src/render_primitives/render_primitives.rs.html#320))

```rust
314fn setup_lights(mut commands: Commands) {
315    commands.spawn((
316        PointLight {
317            intensity: 5000.0,
318            ..default()
319        },
320        Transform::from_translation(Vec3::new(-LEFT_RIGHT_OFFSET_3D, 2.0, 0.0))
321            .looking_at(Vec3::new(-LEFT_RIGHT_OFFSET_3D, 0.0, 0.0), Vec3::Y),
322    ));
323}
324
325/// Marker component for header text
326#[derive(Debug, Clone, Component, Default, Reflect)]
327pub struct HeaderText;
328
329/// Marker component for header node
330#[derive(Debug, Clone, Component, Default, Reflect)]
331pub struct HeaderNode;
332
333fn update_active_cameras(
334    state: Res<State<CameraActive>>,
335    camera_2d: Single<(Entity, &mut Camera), With<Camera2d>>,
336    camera_3d: Single<(Entity, &mut Camera), (With<Camera3d>, Without<Camera2d>)>,
337    mut text: Query<&mut UiTargetCamera, With<HeaderNode>>,
338) {
339    let (entity_2d, mut cam_2d) = camera_2d.into_inner();
340    let (entity_3d, mut cam_3d) = camera_3d.into_inner();
341    let is_camera_2d_active = matches!(*state.get(), CameraActive::Dim2);
342
343    cam_2d.is_active = is_camera_2d_active;
344    cam_3d.is_active = !is_camera_2d_active;
345
346    let active_camera = if is_camera_2d_active {
347        entity_2d
348    } else {
349        entity_3d
350    };
351
352    text.iter_mut().for_each(|mut target_camera| {
353        *target_camera = UiTargetCamera(active_camera);
354    });
355}
356
357fn switch_cameras(current: Res<State<CameraActive>>, mut next: ResMut<NextState<CameraActive>>) {
358    let next_state = match current.get() {
359        CameraActive::Dim2 => CameraActive::Dim3,
360        CameraActive::Dim3 => CameraActive::Dim2,
361    };
362    next.set(next_state);
363}
364
365fn setup_text(mut commands: Commands, cameras: Query<(Entity, &Camera)>) {
366    let active_camera = cameras
367        .iter()
368        .find_map(|(entity, camera)| camera.is_active.then_some(entity))
369        .expect("run condition ensures existence");
370    commands.spawn((
371        HeaderNode,
372        Node {
373            justify_self: JustifySelf::Center,
374            top: px(5),
375            ..Default::default()
376        },
377        UiTargetCamera(active_camera),
378        children![(
379            Text::default(),
380            HeaderText,
381            TextLayout::justify(Justify::Center),
382            children![
383                TextSpan::new("Primitive: "),
384                TextSpan(format!("{text}", text = PrimitiveSelected::default())),
385                TextSpan::new("\n\n"),
386                TextSpan::new(
387                    "Press 'C' to switch between 2D and 3D mode\n\
388                    Press 'Up' or 'Down' to switch to the next/previous primitive",
389                ),
390                TextSpan::new("\n\n"),
391                TextSpan::new("(If nothing is displayed, there's no rendering support yet)",),
392            ]
393        )],
394    ));
395}
396
397fn update_text(
398    primitive_state: Res<State<PrimitiveSelected>>,
399    header: Query<Entity, With<HeaderText>>,
400    mut writer: TextUiWriter,
401) {
402    let new_text = format!("{text}", text = primitive_state.get());
403    header.iter().for_each(|header_text| {
404        if let Some(mut text) = writer.get_text(header_text, 2) {
405            (*text).clone_from(&new_text);
406        };
407    });
408}
409
410fn switch_to_next_primitive(
411    current: Res<State<PrimitiveSelected>>,
412    mut next: ResMut<NextState<PrimitiveSelected>>,
413) {
414    let next_state = current.get().next();
415    next.set(next_state);
416}
417
418fn switch_to_previous_primitive(
419    current: Res<State<PrimitiveSelected>>,
420    mut next: ResMut<NextState<PrimitiveSelected>>,
421) {
422    let next_state = current.get().previous();
423    next.set(next_state);
424}
425
426fn in_mode(active: CameraActive) -> impl Fn(Res<State<CameraActive>>) -> bool {
427    move |state| *state.get() == active
428}
429
430fn draw_gizmos_2d(mut gizmos: Gizmos, state: Res<State<PrimitiveSelected>>, time: Res<Time>) {
431    const POSITION: Vec2 = Vec2::new(-LEFT_RIGHT_OFFSET_2D, 0.0);
432    let angle = time.elapsed_secs();
433    let isometry = Isometry2d::new(POSITION, Rot2::radians(angle));
434    let color = Color::WHITE;
435
436    #[expect(
437        clippy::match_same_arms,
438        reason = "Certain primitives don't have any 2D rendering support yet."
439    )]
440    match state.get() {
441        PrimitiveSelected::RectangleAndCuboid => {
442            gizmos.primitive_2d(&RECTANGLE, isometry, color);
443        }
444        PrimitiveSelected::CircleAndSphere => {
445            gizmos.primitive_2d(&CIRCLE, isometry, color);
446        }
447        PrimitiveSelected::Ellipse => drop(gizmos.primitive_2d(&ELLIPSE, isometry, color)),
448        PrimitiveSelected::Triangle => gizmos.primitive_2d(&TRIANGLE_2D, isometry, color),
449        PrimitiveSelected::Plane => gizmos.primitive_2d(&PLANE_2D, isometry, color),
450        PrimitiveSelected::Line => drop(gizmos.primitive_2d(&LINE_2D, isometry, color)),
451        PrimitiveSelected::Segment => {
452            drop(gizmos.primitive_2d(&SEGMENT_2D, isometry, color));
453        }
454        PrimitiveSelected::Polyline => gizmos.primitive_2d(
455            &Polyline2d {
456                vertices: POLYLINE_2D_VERTICES.to_vec(),
457            },
458            isometry,
459            color,
460        ),
461        PrimitiveSelected::ConvexPolygon => gizmos.primitive_2d(
462            &Polygon::from(ConvexPolygon::new(CONVEX_POLYGON_VERTICES).unwrap()),
463            isometry,
464            color,
465        ),
466        PrimitiveSelected::Polygon => gizmos.primitive_2d(
467            &Polygon {
468                vertices: vec![
469                    Vec2::new(-BIG_2D, -SMALL_2D),
470                    Vec2::new(BIG_2D, -SMALL_2D),
471                    Vec2::new(BIG_2D, SMALL_2D),
472                    Vec2::new(0.0, 0.0),
473                    Vec2::new(-BIG_2D, SMALL_2D),
474                ],
475            },
476            isometry,
477            color,
478        ),
479        PrimitiveSelected::RegularPolygon => {
480            gizmos.primitive_2d(&REGULAR_POLYGON, isometry, color);
481        }
482        PrimitiveSelected::Capsule => gizmos.primitive_2d(&CAPSULE_2D, isometry, color),
483        PrimitiveSelected::Cylinder => {}
484        PrimitiveSelected::Cone => {}
485        PrimitiveSelected::ConicalFrustum => {}
486        PrimitiveSelected::Torus => drop(gizmos.primitive_2d(&ANNULUS, isometry, color)),
487        PrimitiveSelected::Tetrahedron => {}
488        PrimitiveSelected::Arc => gizmos.primitive_2d(&ARC, isometry, color),
489        PrimitiveSelected::CircularSector => {
490            gizmos.primitive_2d(&CIRCULAR_SECTOR, isometry, color);
491        }
492        PrimitiveSelected::CircularSegment => {
493            gizmos.primitive_2d(&CIRCULAR_SEGMENT, isometry, color);
494        }
495    }
496}
497
498/// Marker for primitive meshes to record in which state they should be visible in
499#[derive(Debug, Clone, Component, Default, Reflect)]
500pub struct PrimitiveData {
501    camera_mode: CameraActive,
502    primitive_state: PrimitiveSelected,
503}
504
505/// Marker for meshes of 2D primitives
506#[derive(Debug, Clone, Component, Default)]
507pub struct MeshDim2;
508
509/// Marker for meshes of 3D primitives
510#[derive(Debug, Clone, Component, Default)]
511pub struct MeshDim3;
512
513fn spawn_primitive_2d(
514    mut commands: Commands,
515    mut materials: ResMut<Assets<ColorMaterial>>,
516    mut meshes: ResMut<Assets<Mesh>>,
517) {
518    const POSITION: Vec3 = Vec3::new(LEFT_RIGHT_OFFSET_2D, 0.0, 0.0);
519    let material: Handle<ColorMaterial> = materials.add(Color::WHITE);
520    let camera_mode = CameraActive::Dim2;
521    let polyline_2d = Polyline2d {
522        vertices: POLYLINE_2D_VERTICES.to_vec(),
523    };
524    let convex_polygon = ConvexPolygon::new(CONVEX_POLYGON_VERTICES).unwrap();
525    [
526        Some(RECTANGLE.mesh().build()),
527        Some(CIRCLE.mesh().build()),
528        Some(ELLIPSE.mesh().build()),
529        Some(TRIANGLE_2D.mesh().build()),
530        None, // plane
531        None, // line
532        Some(SEGMENT_2D.mesh().build()),
533        Some(polyline_2d.mesh().build()),
534        None, // polygon
535        Some(convex_polygon.mesh().build()),
536        Some(REGULAR_POLYGON.mesh().build()),
537        Some(CAPSULE_2D.mesh().build()),
538        None, // cylinder
539        None, // cone
540        None, // conical frustum
541        Some(ANNULUS.mesh().build()),
542        None, // tetrahedron
543        None, // arc
544        Some(CIRCULAR_SECTOR.mesh().build()),
545        Some(CIRCULAR_SEGMENT.mesh().build()),
546    ]
547    .into_iter()
548    .zip(PrimitiveSelected::ALL)
549    .for_each(|(maybe_mesh, state)| {
550        if let Some(mesh) = maybe_mesh {
551            commands.spawn((
552                MeshDim2,
553                PrimitiveData {
554                    camera_mode,
555                    primitive_state: state,
556                },
557                Mesh2d(meshes.add(mesh)),
558                MeshMaterial2d(material.clone()),
559                Transform::from_translation(POSITION),
560            ));
561        }
562    });
563}
564
565fn spawn_primitive_3d(
566    mut commands: Commands,
567    mut materials: ResMut<Assets<StandardMaterial>>,
568    mut meshes: ResMut<Assets<Mesh>>,
569) {
570    const POSITION: Vec3 = Vec3::new(-LEFT_RIGHT_OFFSET_3D, 0.0, 0.0);
571    let material: Handle<StandardMaterial> = materials.add(Color::WHITE);
572    let camera_mode = CameraActive::Dim3;
573    let polyline_3d = Polyline3d {
574        vertices: POLYLINE_3D_VERTICES.to_vec(),
575    };
576    [
577        Some(CUBOID.mesh().build()),
578        Some(SPHERE.mesh().build()),
579        None, // ellipse
580        Some(TRIANGLE_3D.mesh().build()),
581        Some(PLANE_3D.mesh().build()),
582        None, // line
583        Some(SEGMENT_3D.mesh().build()),
584        Some(polyline_3d.mesh().build()),
585        None, // polygon
586        None, // convex polygon
587        None, // regular polygon
588        Some(CAPSULE_3D.mesh().build()),
589        Some(CYLINDER.mesh().build()),
590        Some(CONE.mesh().build()),
591        Some(CONICAL_FRUSTUM.mesh().build()),
592        Some(TORUS.mesh().build()),
593        Some(TETRAHEDRON.mesh().build()),
594        None, // arc
595        None, // circular sector
596        None, // circular segment
597    ]
598    .into_iter()
599    .zip(PrimitiveSelected::ALL)
600    .for_each(|(maybe_mesh, state)| {
601        if let Some(mesh) = maybe_mesh {
602            commands.spawn((
603                MeshDim3,
604                PrimitiveData {
605                    camera_mode,
606                    primitive_state: state,
607                },
608                Mesh3d(meshes.add(mesh)),
609                MeshMaterial3d(material.clone()),
610                Transform::from_translation(POSITION),
611            ));
612        }
613    });
614}
```

examples/3d/fog\_volumes.rs ([line 77](../../src/fog_volumes/fog_volumes.rs.html#77))

```rust
74fn rotate_camera(mut cameras: Query<&mut Transform, With<Camera3d>>) {
75    for mut camera_transform in cameras.iter_mut() {
76        *camera_transform =
77            Transform::from_translation(Quat::from_rotation_y(0.01) * camera_transform.translation)
78                .looking_at(vec3(0.0, 0.5, 0.0), Vec3::Y);
79    }
80}
```

Additional examples can be found in:  

*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#341)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#165)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#194-196)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#80)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#33)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#57)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#30)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#383)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#44)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#174)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#618-620)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#124)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#236)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#103)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#40)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#250)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#381)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#83)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#209)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#45)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#41)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#419)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#81)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#158)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#39)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#65)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#174)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#50)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#47)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#446)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#243)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#275)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#43)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#47)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#117)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#85)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#329)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#84)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#93)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#168)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#421)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#55)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#93)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#33)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#57)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#111)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#135)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#42)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#95)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#221)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#72)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#203)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#145)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#290)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#25)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#205)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#68)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#200)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#149)

#### pub const fn [from\_rotation](#method.from_rotation)(rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Creates a new [`Transform`](struct.Transform.html "struct bevy::prelude::Transform"), with `rotation`. Translation will be 0 and scale 1 on all axes.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([lines 358-363](../../src/occlusion_culling/occlusion_culling.rs.html#358-363))

```rust
355fn spawn_light(commands: &mut Commands) {
356    commands
357        .spawn(DirectionalLight::default())
358        .insert(Transform::from_rotation(Quat::from_euler(
359            EulerRot::ZYX,
360            0.0,
361            PI * -0.15,
362            PI * -0.15,
363        )));
364}
```

Hide additional examples

examples/3d/mirror.rs ([line 228](../../src/mirror/mirror.rs.html#228))

```rust
220fn spawn_ground_plane(
221    commands: &mut Commands,
222    meshes: &mut Assets<Mesh>,
223    standard_materials: &mut Assets<StandardMaterial>,
224) {
225    commands.spawn((
226        Mesh3d(meshes.add(Circle::new(200.0))),
227        MeshMaterial3d(standard_materials.add(Color::from(GREEN))),
228        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2))
229            .with_translation(vec3(-25.0, 0.0, 0.0)),
230    ));
231}
```

examples/gltf/query\_gltf\_primitives.rs ([line 61](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#61))

```rust
54fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
55    commands.spawn((
56        Camera3d::default(),
57        Transform::from_xyz(4.0, 4.0, 12.0).looking_at(Vec3::new(0.0, 0.0, 0.5), Vec3::Y),
58    ));
59
60    commands.spawn((
61        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
62        DirectionalLight::default(),
63    ));
64
65    commands.spawn(WorldAssetRoot(asset_server.load(
66        GltfAssetLabel::Scene(0).from_asset("models/GltfPrimitives/gltf_primitives.glb"),
67    )));
68}
```

examples/app/externally\_driven\_headless\_renderer.rs ([line 138](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#138))

```rust
130fn spawn_test_scene(
131    mut commands: Commands,
132    mut meshes: ResMut<Assets<Mesh>>,
133    mut materials: ResMut<Assets<StandardMaterial>>,
134) {
135    commands.spawn((
136        Mesh3d(meshes.add(Circle::new(4.0))),
137        MeshMaterial3d(materials.add(Color::WHITE)),
138        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
139    ));
140    commands.spawn((
141        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
142        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
143        Transform::from_xyz(0.0, 1.0, 0.0),
144    ));
145    commands.spawn((
146        PointLight {
147            shadow_maps_enabled: true,
148            ..default()
149        },
150        Transform::from_xyz(4.0, 8.0, 4.0),
151    ));
152}
```

examples/asset/multi\_asset\_sync.rs ([line 205](../../src/multi_asset_sync/multi_asset_sync.rs.html#205))

```rust
188fn setup_scene(
189    mut commands: Commands,
190    mut meshes: ResMut<Assets<Mesh>>,
191    mut materials: ResMut<Assets<StandardMaterial>>,
192) {
193    // Camera
194    commands.spawn((
195        Camera3d::default(),
196        Transform::from_xyz(10.0, 10.0, 15.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
197    ));
198
199    // Light
200    commands.spawn((
201        DirectionalLight {
202            shadow_maps_enabled: true,
203            ..default()
204        },
205        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
206    ));
207
208    // Plane
209    commands.spawn((
210        Mesh3d(meshes.add(Plane3d::default().mesh().size(50000.0, 50000.0))),
211        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.2, 0.2))),
212        Loading,
213    ));
214}
```

examples/transforms/scale.rs ([line 46](../../src/scale/scale.rs.html#46))

```rust
37fn setup(
38    mut commands: Commands,
39    mut meshes: ResMut<Assets<Mesh>>,
40    mut materials: ResMut<Assets<StandardMaterial>>,
41) {
42    // Spawn a cube to scale.
43    commands.spawn((
44        Mesh3d(meshes.add(Cuboid::default())),
45        MeshMaterial3d(materials.add(Color::WHITE)),
46        Transform::from_rotation(Quat::from_rotation_y(PI / 4.0)),
47        Scaling::new(),
48    ));
49
50    // Spawn a camera looking at the entities to show what's happening in this example.
51    commands.spawn((
52        Camera3d::default(),
53        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
54    ));
55
56    // Add a light source for better 3d visibility.
57    commands.spawn((
58        DirectionalLight::default(),
59        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
60    ));
61}
```

Additional examples can be found in:  

*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#140)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#49)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#194-199)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#120)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#253)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#133)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#367)
*   [examples/remote/server.rs](../../src/server/server.rs.html#35)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#153)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#126)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#70)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#68)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#328)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#108)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#67)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#84)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#98)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#43)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#65)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#67)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#201)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#177)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#76)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#114)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#43)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#191)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#104)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#152)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#44)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#65)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#91)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#451)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#58)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#146)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#110)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#65)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#49)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#36)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#126-131)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#230)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#72)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#188)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#166)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#159)

#### pub const fn [from\_scale](#method.from_scale)(scale: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Creates a new [`Transform`](struct.Transform.html "struct bevy::prelude::Transform"), with `scale`. Translation will be 0 and rotation 0 on all axes.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([line 286](../../src/irradiance_volumes/irradiance_volumes.rs.html#286))

```rust
282fn spawn_fox(commands: &mut Commands, assets: &ExampleAssets) {
283    commands.spawn((
284        WorldAssetRoot(assets.fox.clone()),
285        Visibility::Hidden,
286        Transform::from_scale(Vec3::splat(FOX_SCALE)),
287        MainObject,
288    ));
289}
290
291fn spawn_text(commands: &mut Commands, app_status: &AppStatus) {
292    commands.spawn((
293        app_status.create_text(),
294        Node {
295            position_type: PositionType::Absolute,
296            bottom: px(12),
297            left: px(12),
298            ..default()
299        },
300    ));
301}
302
303// A system that updates the help text.
304fn update_text(mut text_query: Query<&mut Text>, app_status: Res<AppStatus>) {
305    for mut text in text_query.iter_mut() {
306        *text = app_status.create_text();
307    }
308}
309
310impl AppStatus {
311    // Constructs the help text at the bottom of the screen based on the
312    // application status.
313    fn create_text(&self) -> Text {
314        let irradiance_volume_help_text = if self.irradiance_volume_present {
315            DISABLE_IRRADIANCE_VOLUME_HELP_TEXT
316        } else {
317            ENABLE_IRRADIANCE_VOLUME_HELP_TEXT
318        };
319
320        let voxels_help_text = if self.voxels_visible {
321            HIDE_VOXELS_HELP_TEXT
322        } else {
323            SHOW_VOXELS_HELP_TEXT
324        };
325
326        let rotation_help_text = if self.rotating {
327            STOP_ROTATION_HELP_TEXT
328        } else {
329            START_ROTATION_HELP_TEXT
330        };
331
332        let switch_mesh_help_text = match self.model {
333            ExampleModel::Sphere => SWITCH_TO_FOX_HELP_TEXT,
334            ExampleModel::Fox => SWITCH_TO_SPHERE_HELP_TEXT,
335        };
336
337        format!(
338            "{CLICK_TO_MOVE_HELP_TEXT}\n\
339            {voxels_help_text}\n\
340            {irradiance_volume_help_text}\n\
341            {rotation_help_text}\n\
342            {switch_mesh_help_text}"
343        )
344        .into()
345    }
346}
347
348// Rotates the camera a bit every frame.
349fn rotate_camera(
350    mut camera_query: Query<&mut Transform, With<Camera3d>>,
351    time: Res<Time>,
352    app_status: Res<AppStatus>,
353) {
354    if !app_status.rotating {
355        return;
356    }
357
358    for mut transform in camera_query.iter_mut() {
359        transform.translation = Vec2::from_angle(ROTATION_SPEED * time.delta_secs())
360            .rotate(transform.translation.xz())
361            .extend(transform.translation.y)
362            .xzy();
363        transform.look_at(Vec3::ZERO, Vec3::Y);
364    }
365}
366
367// Toggles between the unskinned sphere model and the skinned fox model if the
368// user requests it.
369fn change_main_object(
370    keyboard: Res<ButtonInput<KeyCode>>,
371    mut app_status: ResMut<AppStatus>,
372    mut sphere_query: Query<
373        &mut Visibility,
374        (With<MainObject>, With<Mesh3d>, Without<WorldAssetRoot>),
375    >,
376    mut fox_query: Query<&mut Visibility, (With<MainObject>, With<WorldAssetRoot>)>,
377) {
378    if !keyboard.just_pressed(KeyCode::Tab) {
379        return;
380    }
381    let Some(mut sphere_visibility) = sphere_query.iter_mut().next() else {
382        return;
383    };
384    let Some(mut fox_visibility) = fox_query.iter_mut().next() else {
385        return;
386    };
387
388    match app_status.model {
389        ExampleModel::Sphere => {
390            *sphere_visibility = Visibility::Hidden;
391            *fox_visibility = Visibility::Visible;
392            app_status.model = ExampleModel::Fox;
393        }
394        ExampleModel::Fox => {
395            *sphere_visibility = Visibility::Visible;
396            *fox_visibility = Visibility::Hidden;
397            app_status.model = ExampleModel::Sphere;
398        }
399    }
400}
401
402impl Default for AppStatus {
403    fn default() -> Self {
404        Self {
405            irradiance_volume_present: true,
406            rotating: true,
407            model: ExampleModel::Sphere,
408            voxels_visible: false,
409        }
410    }
411}
412
413// Turns on and off the irradiance volume as requested by the user.
414fn toggle_irradiance_volumes(
415    mut commands: Commands,
416    keyboard: Res<ButtonInput<KeyCode>>,
417    light_probe_query: Query<Entity, With<LightProbe>>,
418    mut app_status: ResMut<AppStatus>,
419    assets: Res<ExampleAssets>,
420    mut ambient_light: ResMut<GlobalAmbientLight>,
421) {
422    if !keyboard.just_pressed(KeyCode::Space) {
423        return;
424    };
425
426    let Some(light_probe) = light_probe_query.iter().next() else {
427        return;
428    };
429
430    if app_status.irradiance_volume_present {
431        commands.entity(light_probe).remove::<IrradianceVolume>();
432        ambient_light.brightness = AMBIENT_LIGHT_BRIGHTNESS * IRRADIANCE_VOLUME_INTENSITY;
433        app_status.irradiance_volume_present = false;
434    } else {
435        commands.entity(light_probe).insert(IrradianceVolume {
436            voxels: assets.irradiance_volume.clone(),
437            intensity: IRRADIANCE_VOLUME_INTENSITY,
438            ..default()
439        });
440        ambient_light.brightness = 0.0;
441        app_status.irradiance_volume_present = true;
442    }
443}
444
445fn toggle_rotation(keyboard: Res<ButtonInput<KeyCode>>, mut app_status: ResMut<AppStatus>) {
446    if keyboard.just_pressed(KeyCode::Enter) {
447        app_status.rotating = !app_status.rotating;
448    }
449}
450
451// Handles clicks on the plane that reposition the object.
452fn handle_mouse_clicks(
453    buttons: Res<ButtonInput<MouseButton>>,
454    windows: Query<&Window, With<PrimaryWindow>>,
455    cameras: Query<(&Camera, &GlobalTransform)>,
456    mut main_objects: Query<&mut Transform, With<MainObject>>,
457) {
458    if !buttons.pressed(MouseButton::Left) {
459        return;
460    }
461    let Some(mouse_position) = windows.iter().next().and_then(Window::cursor_position) else {
462        return;
463    };
464    let Some((camera, camera_transform)) = cameras.iter().next() else {
465        return;
466    };
467
468    // Figure out where the user clicked on the plane.
469    let Ok(ray) = camera.viewport_to_world(camera_transform, mouse_position) else {
470        return;
471    };
472    let Some(plane_intersection) =
473        ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
474    else {
475        return;
476    };
477    // Move all the main objects.
478    for mut transform in main_objects.iter_mut() {
479        transform.translation = vec3(
480            plane_intersection.x,
481            transform.translation.y,
482            plane_intersection.z,
483        );
484    }
485}
486
487impl FromWorld for ExampleAssets {
488    fn from_world(world: &mut World) -> Self {
489        let fox_animation =
490            world.load_asset(GltfAssetLabel::Animation(1).from_asset("models/animated/Fox.glb"));
491        let (fox_animation_graph, fox_animation_node) =
492            AnimationGraph::from_clip(fox_animation.clone());
493
494        ExampleAssets {
495            main_sphere: world.add_asset(Sphere::default().mesh().uv(32, 18)),
496            fox: world.load_asset(GltfAssetLabel::Scene(0).from_asset("models/animated/Fox.glb")),
497            main_sphere_material: world.add_asset(Color::from(SILVER)),
498            main_scene: world.load_asset(
499                GltfAssetLabel::Scene(0)
500                    .from_asset("models/IrradianceVolumeExample/IrradianceVolumeExample.glb"),
501            ),
502            irradiance_volume: world.load_asset("irradiance_volumes/Example.vxgi.ktx2"),
503            fox_animation_graph: world.add_asset(fox_animation_graph),
504            fox_animation_node,
505            voxel_cube: world.add_asset(Cuboid::default()),
506            // Just use a specular map for the skybox since it's not too blurry.
507            // In reality you wouldn't do this--you'd use a real skybox texture--but
508            // reusing the textures like this saves space in the Bevy repository.
509            skybox: world.load_asset("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
510        }
511    }
512}
513
514// Plays the animation on the fox.
515fn play_animations(
516    mut commands: Commands,
517    assets: Res<ExampleAssets>,
518    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
519) {
520    for (entity, mut player) in players.iter_mut() {
521        commands
522            .entity(entity)
523            .insert(AnimationGraphHandle(assets.fox_animation_graph.clone()));
524        player.play(assets.fox_animation_node).repeat();
525    }
526}
527
528fn create_cubes(
529    image_assets: Res<Assets<Image>>,
530    mut commands: Commands,
531    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
532    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
533    voxel_cubes: Query<Entity, With<VoxelCube>>,
534    example_assets: Res<ExampleAssets>,
535    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
536) {
537    // If voxel cubes have already been spawned, don't do anything.
538    if !voxel_cubes.is_empty() {
539        return;
540    }
541
542    let Some(voxel_cube_parent) = voxel_cube_parents.iter().next() else {
543        return;
544    };
545
546    for (irradiance_volume, global_transform) in irradiance_volumes.iter() {
547        let Some(image) = image_assets.get(&irradiance_volume.voxels) else {
548            continue;
549        };
550
551        let resolution = image.texture_descriptor.size;
552
553        let voxel_cube_material = voxel_visualization_material_assets.add(ExtendedMaterial {
554            base: StandardMaterial::from(Color::from(RED)),
555            extension: VoxelVisualizationExtension {
556                irradiance_volume_info: VoxelVisualizationIrradianceVolumeInfo {
557                    world_from_voxel: VOXEL_FROM_WORLD.inverse(),
558                    voxel_from_world: VOXEL_FROM_WORLD,
559                    resolution: uvec3(
560                        resolution.width,
561                        resolution.height,
562                        resolution.depth_or_array_layers,
563                    ),
564                    intensity: IRRADIANCE_VOLUME_INTENSITY,
565                },
566            },
567        });
568
569        let scale = vec3(
570            1.0 / resolution.width as f32,
571            1.0 / resolution.height as f32,
572            1.0 / resolution.depth_or_array_layers as f32,
573        );
574
575        // Spawn a cube for each voxel.
576        for z in 0..resolution.depth_or_array_layers {
577            for y in 0..resolution.height {
578                for x in 0..resolution.width {
579                    let uvw = (uvec3(x, y, z).as_vec3() + 0.5) * scale - 0.5;
580                    let pos = global_transform.transform_point(uvw);
581                    let voxel_cube = commands
582                        .spawn((
583                            Mesh3d(example_assets.voxel_cube.clone()),
584                            MeshMaterial3d(voxel_cube_material.clone()),
585                            Transform::from_scale(Vec3::splat(VOXEL_CUBE_SCALE))
586                                .with_translation(pos),
587                        ))
588                        .insert(VoxelCube)
589                        .insert(NotShadowCaster)
590                        .id();
591
592                    commands.entity(voxel_cube_parent).add_child(voxel_cube);
593                }
594            }
595        }
596    }
597}
```

Hide additional examples

examples/gltf/gltf\_extension\_mesh\_2d.rs ([line 62](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#62))

```rust
56fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
57    commands.spawn((
58        WorldAssetRoot(
59            asset_server
60                .load(GltfAssetLabel::Scene(0).from_asset("models/barycentric/barycentric.gltf")),
61        ),
62        Transform::from_scale(150. * Vec3::ONE),
63    ));
64    commands.spawn(Camera2d);
65}
```

examples/3d/ssr.rs ([line 284](../../src/ssr/ssr.rs.html#284))

```rust
278fn spawn_flight_helmet(commands: &mut Commands, asset_server: &AssetServer) {
279    commands.spawn((
280        WorldAssetRoot(
281            asset_server
282                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
283        ),
284        Transform::from_scale(Vec3::splat(2.5)),
285        FlightHelmetModel,
286        Visibility::Hidden,
287    ));
288}
289
290// Spawns the row of capsules.
291fn spawn_capsules(
292    commands: &mut Commands,
293    meshes: &mut Assets<Mesh>,
294    standard_materials: &mut Assets<StandardMaterial>,
295) {
296    let capsule_mesh = meshes.add(Capsule3d::new(0.4, 0.5));
297    let parent = commands
298        .spawn((
299            Transform::from_xyz(0.0, 0.5, 0.0),
300            Visibility::Hidden,
301            CapsulesParent,
302        ))
303        .id();
304
305    for i in 0..5 {
306        let roughness = i as f32 * 0.25;
307        let child = commands
308            .spawn((
309                Mesh3d(capsule_mesh.clone()),
310                MeshMaterial3d(standard_materials.add(StandardMaterial {
311                    base_color: Color::BLACK,
312                    perceptual_roughness: roughness.max(0.08),
313                    ..default()
314                })),
315                Transform::from_xyz(i as f32 * 1.1 - (1.1 * 2.0), 0.5, 0.0),
316                CapsuleModel,
317            ))
318            .id();
319        commands.entity(parent).add_child(child);
320    }
321}
322
323// Spawns the metallic base.
324fn spawn_metallic_base(
325    commands: &mut Commands,
326    meshes: &mut Assets<Mesh>,
327    standard_materials: &mut Assets<StandardMaterial>,
328) {
329    commands.spawn((
330        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
331        MeshMaterial3d(standard_materials.add(StandardMaterial {
332            base_color: Color::from(bevy::color::palettes::css::DARK_GRAY),
333            metallic: 1.0,
334            perceptual_roughness: 0.3,
335            ..default()
336        })),
337        Transform::from_scale(Vec3::splat(100.0)),
338        MetallicBaseModel,
339        Visibility::Hidden,
340    ));
341}
342
343// Spawns the non-metallic base.
344fn spawn_non_metallic_base(
345    commands: &mut Commands,
346    meshes: &mut Assets<Mesh>,
347    standard_materials: &mut Assets<StandardMaterial>,
348) {
349    commands.spawn((
350        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
351        MeshMaterial3d(standard_materials.add(StandardMaterial {
352            base_color: Color::from(bevy::color::palettes::css::RED),
353            metallic: 0.0,
354            perceptual_roughness: 0.2,
355            ..default()
356        })),
357        Transform::from_scale(Vec3::splat(100.0)),
358        RedPlaneBaseModel,
359        Visibility::Hidden,
360    ));
361}
362
363// Spawns the water plane.
364fn spawn_water(
365    commands: &mut Commands,
366    asset_server: &AssetServer,
367    meshes: &mut Assets<Mesh>,
368    water_materials: &mut Assets<ExtendedMaterial<StandardMaterial, Water>>,
369) {
370    commands.spawn((
371        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
372        MeshMaterial3d(
373            water_materials.add(ExtendedMaterial {
374                base: StandardMaterial {
375                    base_color: BLACK.into(),
376                    perceptual_roughness: 0.09,
377                    ..default()
378                },
379                extension: Water {
380                    normals: asset_server
381                        .load_builder()
382                        .with_settings::<ImageLoaderSettings>(|settings| {
383                            settings.is_srgb = false;
384                            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
385                                address_mode_u: ImageAddressMode::Repeat,
386                                address_mode_v: ImageAddressMode::Repeat,
387                                mag_filter: ImageFilterMode::Linear,
388                                min_filter: ImageFilterMode::Linear,
389                                ..default()
390                            });
391                        })
392                        .load("textures/water_normals.png"),
393                    // These water settings are just random values to create some
394                    // variety.
395                    settings: WaterSettings {
396                        octave_vectors: [
397                            vec4(0.080, 0.059, 0.073, -0.062),
398                            vec4(0.153, 0.138, -0.149, -0.195),
399                        ],
400                        octave_scales: vec4(1.0, 2.1, 7.9, 14.9) * 5.0,
401                        octave_strengths: vec4(0.16, 0.18, 0.093, 0.044),
402                    },
403                },
404            }),
405        ),
406        Transform::from_scale(Vec3::splat(100.0)),
407        WaterModel,
408    ));
409}
```

examples/movement/physics\_in\_fixed\_timestep.rs ([line 179](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#179))

```rust
175fn spawn_player(mut commands: Commands) {
176    commands.spawn((Camera3d::default(), CameraSensitivity::default()));
177    commands.spawn((
178        Name::new("Player"),
179        Transform::from_scale(Vec3::splat(0.3)),
180        AccumulatedInput::default(),
181        Velocity::default(),
182        PhysicalTranslation::default(),
183        PreviousPhysicalTranslation::default(),
184    ));
185}
```

examples/gltf/custom\_gltf\_vertex\_attribute.rs ([line 62](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#62))

```rust
46fn setup(
47    mut commands: Commands,
48    asset_server: Res<AssetServer>,
49    mut materials: ResMut<Assets<CustomMaterial>>,
50) {
51    // Add a mesh loaded from a glTF file. This mesh has data for `ATTRIBUTE_BARYCENTRIC`.
52    let mesh = asset_server.load(
53        GltfAssetLabel::Primitive {
54            mesh: 0,
55            primitive: 0,
56        }
57        .from_asset("models/barycentric/barycentric.gltf"),
58    );
59    commands.spawn((
60        Mesh2d(mesh),
61        MeshMaterial2d(materials.add(CustomMaterial {})),
62        Transform::from_scale(150.0 * Vec3::ONE),
63    ));
64
65    commands.spawn(Camera2d);
66}
```

examples/3d/reflection\_probes.rs ([line 166](../../src/reflection_probes/reflection_probes.rs.html#166))

```rust
156fn spawn_reflection_probe(commands: &mut Commands, cubemaps: &Cubemaps) {
157    commands.spawn((
158        LightProbe::default(),
159        EnvironmentMapLight {
160            diffuse_map: cubemaps.diffuse_environment_map.clone(),
161            specular_map: cubemaps.specular_reflection_probe.clone(),
162            intensity: ENV_MAP_INTENSITY,
163            ..default()
164        },
165        // 2.0 because the sphere's radius is 1.0 and we want to fully enclose it.
166        Transform::from_scale(Vec3::splat(2.0)),
167        // Disable parallax correction because the reflected scene is quite
168        // distant.
169        ParallaxCorrection::None,
170    ));
171}
```

Additional examples can be found in:  

*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#149)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#20)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#68)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#63)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#245)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#146)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#299)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#82)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#27)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#329)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#237)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#87)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#123)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#121)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#156)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#57)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#56)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#40)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#107)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#93)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#159)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#179)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#238)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#170)

#### pub fn [from\_isometry](#method.from_isometry)(iso: [Isometry3d](struct.Isometry3d.html "struct bevy::prelude::Isometry3d")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Creates a new [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") that is equivalent to the given [isometry](struct.Isometry3d.html "struct bevy::prelude::Isometry3d").

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#187)

#### pub fn [looking\_at](#method.looking_at)(self, target: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3"), up: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") with a new rotation so that [`Transform::forward`](struct.Transform.html#method.forward "method bevy::prelude::Transform::forward") points towards the `target` position and [`Transform::up`](struct.Transform.html#method.up "method bevy::prelude::Transform::up") points towards `up`.

In some cases it’s not possible to construct a rotation. Another axis will be picked in those cases:

*   if `target` is the same as the transform translation, `Vec3::Z` is used instead
*   if `up` fails converting to `Dir3` (e.g if it is `Vec3::ZERO`), `Dir3::Y` is used instead
*   if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the “right” direction

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/3d/clustered\_decals.rs ([line 204](../../src/clustered_decals/clustered_decals.rs.html#204))

```rust
201fn spawn_light(commands: &mut Commands) {
202    commands.spawn((
203        DirectionalLight::default(),
204        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
205    ));
206}
207
208/// Spawns the camera.
209fn spawn_camera(commands: &mut Commands) {
210    commands
211        .spawn(Camera3d::default())
212        .insert(Transform::from_xyz(0.0, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
213        // Tag the camera with `Selection::Camera`.
214        .insert(Selection::Camera);
215}
```

Hide additional examples

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 53](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#53))

```rust
50fn setup(mut commands: Commands) {
51    commands.spawn((
52        Camera3d::default(),
53        Transform::from_xyz(0.0, 7.5, 18.0).looking_at(Vec3::new(0.0, 5.5, 0.0), Vec3::Y),
54    ));
55}
```

examples/3d/mixed\_lighting.rs ([line 160](../../src/mixed_lighting/mixed_lighting.rs.html#160))

```rust
157fn spawn_camera(commands: &mut Commands) {
158    commands
159        .spawn(Camera3d::default())
160        .insert(Transform::from_xyz(-0.7, 0.7, 1.0).looking_at(vec3(0.0, 0.3, 0.0), Vec3::Y));
161}
```

examples/3d/clustered\_decal\_maps.rs ([line 243](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#243))

```rust
240fn spawn_camera(commands: &mut Commands) {
241    commands.spawn((
242        Camera3d::default(),
243        Transform::from_xyz(2.0, 0.0, -7.0).looking_at(Vec3::ZERO, Vec3::Y),
244        Hdr,
245    ));
246}
```

examples/3d/pccm.rs ([line 98](../../src/pccm/pccm.rs.html#98))

```rust
94fn spawn_camera(commands: &mut Commands) {
95    commands.spawn((
96        Camera3d::default(),
97        FreeCamera::default(),
98        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::new(0.0, -2.5, 0.0), Dir3::Y),
99        Hdr,
100    ));
101}
```

examples/3d/occlusion\_culling.rs ([line 370](../../src/occlusion_culling/occlusion_culling.rs.html#370))

```rust
367fn spawn_camera(commands: &mut Commands) {
368    commands
369        .spawn(Camera3d::default())
370        .insert(Transform::from_xyz(0.0, 0.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
371        .insert(DepthPrepass)
372        .insert(OcclusionCulling);
373}
```

Additional examples can be found in:  

*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#128)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#157)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#237)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#17)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#306)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#85)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#129)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#24)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#103)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#197)
*   [examples/shader/animate\_shader.rs](../../src/animate_shader/animate_shader.rs.html#33)
*   [examples/async\_tasks/async\_compute.rs](../../src/async_compute/async_compute.rs.html#159)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#57)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#69)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#203)
*   [examples/shader/fallback\_image.rs](../../src/fallback_image/fallback_image.rs.html#43)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#202)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#33)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#57)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#38)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#39)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#61)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#30)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#196)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#238)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#326)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#37)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#53)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#155)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#55)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#54)
*   [examples/3d/3d\_scene.rs](../../src/3d_scene/3d_scene.rs.html#35)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#57)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#133)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#29)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#51)
*   [examples/shader/shader\_defs.rs](../../src/shader_defs/shader_defs.rs.html#53)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#22)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#109)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#140)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#55)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#229)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#83)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#47)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#76)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#54)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#80)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#24)
*   [examples/remote/server.rs](../../src/server/server.rs.html#64)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#128)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#103)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#77)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#21)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#250)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#63)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#99)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#243)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#21)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#43)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#63)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#97)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#86)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#139)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#34)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#108)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#39)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#20)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#79)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#48)
*   [examples/3d/lines.rs](../../src/lines/lines.rs.html#62)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#63)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#181)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#30)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#51)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#49)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#419)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#104)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#58)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#82)
*   [examples/shader/extended\_material.rs](../../src/extended_material/extended_material.rs.html#54)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#50)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#54)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#82)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#46)
*   [examples/3d/orthographic.rs](../../src/orthographic/orthographic.rs.html#28)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#82)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#97)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#21)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#24)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#212)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#79)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#61)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#447)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#198)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#32)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#146)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#103)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#62)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#74)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#61)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#72)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#86)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#64)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#63)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#84)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#165)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#71)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#51)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#91)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#76)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#103)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#116)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#34)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#464)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#54)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#58)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#49)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#94)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#174)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#50)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#118)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#158)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#79)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#116)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#95)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#95)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#105)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#102)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#132)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#42)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#58)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#43)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#98)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#85)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#221)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#49)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#47)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#72)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#144)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#154)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#221)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#155)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#206)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#37)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#112)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#204)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#48)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#114)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#151)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#32)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#158)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#31)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#302)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#201)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#201)

#### pub fn [looking\_to](#method.looking_to)( self, direction: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, up: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, ) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") with a new rotation so that [`Transform::forward`](struct.Transform.html#method.forward "method bevy::prelude::Transform::forward") points in the given `direction` and [`Transform::up`](struct.Transform.html#method.up "method bevy::prelude::Transform::up") points towards `up`.

In some cases it’s not possible to construct a rotation. Another axis will be picked in those cases:

*   if `direction` fails converting to `Dir3` (e.g if it is `Vec3::ZERO`), `Dir3::Z` is used instead
*   if `up` fails converting to `Dir3`, `Dir3::Y` is used instead
*   if `direction` is parallel with `up`, an orthogonal vector is used as the “right” direction

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 68](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#68))

```rust
63fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
64    commands.spawn((
65        PendingScene(asset_server.load("models/animated/Fox.glb")),
66        Transform::from_xyz(1.3, 4.3, 0.0)
67            .with_scale(Vec3::splat(0.08))
68            .looking_to(-Vec3::X, Vec3::Y),
69    ));
70}
```

Hide additional examples

examples/3d/clustered\_decals.rs ([line 343](../../src/clustered_decals/clustered_decals.rs.html#343))

```rust
338fn calculate_initial_decal_transform(start: Vec3, looking_at: Vec3, size: Vec2) -> Transform {
339    let direction = looking_at - start;
340    let center = start + direction * 0.5;
341    Transform::from_translation(center)
342        .with_scale((size * 0.5).extend(direction.length()))
343        .looking_to(direction, Vec3::Y)
344}
```

examples/scene/world\_serialization.rs ([line 133](../../src/world_serialization/world_serialization.rs.html#133))

```rust
125fn load_world_system(mut commands: Commands, asset_server: Res<AssetServer>) {
126    commands.spawn(DynamicWorldRoot(asset_server.load(WORLD_FILE_PATH)));
127    commands.spawn((
128        Camera3d::default(),
129        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::new(0.0, 0.25, 0.0), Vec3::Y),
130    ));
131    commands.spawn((
132        DirectionalLight::default(),
133        Transform::default().looking_to(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
134    ));
135}
```

examples/camera/free\_camera\_controller.rs ([line 79](../../src/free_camera_controller/free_camera_controller.rs.html#79))

```rust
76fn spawn_camera(mut commands: Commands) {
77    commands.spawn((
78        Camera3d::default(),
79        Transform::from_xyz(0.0, 1.0, 0.0).looking_to(Vec3::X, Vec3::Y),
80        // This component stores all camera settings and state, which is used by the FreeCameraPlugin to
81        // control it. These properties can be changed at runtime, but beware the controller system is
82        // constantly using and modifying those values unless the enabled field is false.
83        FreeCamera {
84            sensitivity: 0.2,
85            friction: 25.0,
86            walk_speed: 3.0,
87            run_speed: 9.0,
88            ..default()
89        },
90    ));
91}
```

examples/movement/physics\_in\_fixed\_timestep.rs ([line 219](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#219))

```rust
188fn spawn_environment(
189    mut commands: Commands,
190    mut meshes: ResMut<Assets<Mesh>>,
191    mut materials: ResMut<Assets<StandardMaterial>>,
192) {
193    let sphere_material = materials.add(Color::from(tailwind::SKY_200));
194    let sphere_mesh = meshes.add(Sphere::new(0.3));
195    let spheres_in_x = 6;
196    let spheres_in_y = 4;
197    let spheres_in_z = 10;
198    let distance = 3.0;
199    for x in 0..spheres_in_x {
200        for y in 0..spheres_in_y {
201            for z in 0..spheres_in_z {
202                let translation = Vec3::new(
203                    x as f32 * distance - (spheres_in_x as f32 - 1.0) * distance / 2.0,
204                    y as f32 * distance - (spheres_in_y as f32 - 1.0) * distance / 2.0,
205                    z as f32 * distance - (spheres_in_z as f32 - 1.0) * distance / 2.0,
206                );
207                commands.spawn((
208                    Name::new("Sphere"),
209                    Transform::from_translation(translation),
210                    Mesh3d(sphere_mesh.clone()),
211                    MeshMaterial3d(sphere_material.clone()),
212                ));
213            }
214        }
215    }
216
217    commands.spawn((
218        DirectionalLight::default(),
219        Transform::default().looking_to(Vec3::new(-1.0, -3.0, 0.5), Vec3::Y),
220    ));
221}
```

examples/asset/generated\_assets.rs ([line 23](../../src/generated_assets/generated_assets.rs.html#23))

```rust
13fn setup(
14    mut commands: Commands,
15    asset_server: Res<AssetServer>,
16    mut materials: ResMut<Assets<StandardMaterial>>,
17    meshes: Res<Assets<Mesh>>,
18) {
19    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 5.0)));
20
21    commands.spawn((
22        DirectionalLight::default(),
23        Transform::default().looking_to(Dir3::new(Vec3::new(-1.0, -1.0, -1.0)).unwrap(), Dir3::Y),
24    ));
25
26    // The simplest way to generate an asset is to add it directly to the `Assets`.
27    let material_handle = materials.add(StandardMaterial::default());
28
29    commands.spawn((
30        Transform::from_xyz(-2.0, 0.0, 0.0),
31        MeshMaterial3d(material_handle.clone()),
32        // Alternatively, `add_async` creates a task that runs your async function. Once it
33        // completes, the asset is added to the `Assets`. This is "deferred" meaning that the asset
34        // may take a frame to be added after the task completes.
35        Mesh3d(asset_server.add_async(generate_mesh_async())),
36    ));
37
38    // The last way to generate assets is to reserve a handle, and then use `Assets::insert` to
39    // populate the asset later. In this example, the `generate_mesh_system` system runs to populate
40    // the mesh.
41    let mesh_handle = meshes.reserve_handle();
42    commands.insert_resource(HandleToGenerate(mesh_handle.clone()));
43    commands.spawn((
44        Transform::from_xyz(2.0, 0.0, 0.0)
45            .with_rotation(Quat::from_rotation_x(50.0f32.to_radians())),
46        Mesh3d(mesh_handle),
47        MeshMaterial3d(material_handle),
48    ));
49}
```

Additional examples can be found in:  

*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#441)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#72)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#331)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#223-229)

#### pub fn [aligned\_by](#method.aligned_by)( self, main\_axis: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, main\_direction: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, secondary\_axis: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, secondary\_direction: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, ) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") so that the `main_axis` vector, reinterpreted in local coordinates, points in the given `main_direction`, while `secondary_axis` points towards `secondary_direction`. For example, if a spaceship model has its nose pointing in the X-direction in its own local coordinates and its dorsal fin pointing in the Y-direction, then `align(Dir3::X, v, Dir3::Y, w)` will make the spaceship’s nose point in the direction of `v`, while the dorsal fin does its best to point in the direction `w`.

In some cases a rotation cannot be constructed. Another axis will be picked in those cases:

*   if `main_axis` or `main_direction` fail converting to `Dir3` (e.g are zero), `Dir3::X` takes their place
*   if `secondary_axis` or `secondary_direction` fail converting, `Dir3::Y` takes their place
*   if `main_axis` is parallel with `secondary_axis` or `main_direction` is parallel with `secondary_direction`, a rotation is constructed which takes `main_axis` to `main_direction` along a great circle, ignoring the secondary counterparts

See [`Transform::align`](struct.Transform.html#method.align "method bevy::prelude::Transform::align") for additional details.

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/transforms/align.rs ([line 226](../../src/align/align.rs.html#226))

```rust
224fn random_axes_target_alignment(random_axes: &RandomAxes) -> Transform {
225    let RandomAxes(first, second) = random_axes;
226    Transform::IDENTITY.aligned_by(Vec3::NEG_Z, *first, Vec3::X, *second)
227}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#242)

#### pub const fn [with\_translation](#method.with_translation)(self, translation: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") with a new translation.

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 229](../../src/mirror/mirror.rs.html#229))

```rust
220fn spawn_ground_plane(
221    commands: &mut Commands,
222    meshes: &mut Assets<Mesh>,
223    standard_materials: &mut Assets<StandardMaterial>,
224) {
225    commands.spawn((
226        Mesh3d(meshes.add(Circle::new(200.0))),
227        MeshMaterial3d(standard_materials.add(Color::from(GREEN))),
228        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2))
229            .with_translation(vec3(-25.0, 0.0, 0.0)),
230    ));
231}
232
233/// Creates the initial image that the mirror camera will render the mirror
234/// world to.
235fn create_mirror_texture_resource(
236    commands: &mut Commands,
237    windows_query: &Query<&Window>,
238    images: &mut Assets<Image>,
239) -> Handle<Image> {
240    let window = windows_query.iter().next().expect("No window found");
241    let window_size = uvec2(window.physical_width(), window.physical_height());
242    let image = create_mirror_texture_image(images, window_size);
243    commands.insert_resource(MirrorImage(image.clone()));
244    image
245}
246
247/// Spawns the camera that renders the mirror world.
248fn spawn_mirror_camera(
249    commands: &mut Commands,
250    camera_transform: &Transform,
251    camera_projection: &PerspectiveProjection,
252    mirror_transform: &Transform,
253    mirror_render_target: Handle<Image>,
254) {
255    let (mirror_camera_transform, mirror_camera_projection) =
256        calculate_mirror_camera_transform_and_projection(
257            camera_transform,
258            camera_projection,
259            mirror_transform,
260        );
261
262    commands.spawn((
263        Camera3d::default(),
264        Camera {
265            order: -1,
266            // Reflecting the model across the mirror will flip the winding of
267            // all the polygons. Therefore, in order to properly backface cull,
268            // we need to turn on `invert_culling`.
269            invert_culling: true,
270            ..default()
271        },
272        RenderTarget::Image(mirror_render_target.clone().into()),
273        mirror_camera_transform,
274        Projection::Perspective(mirror_camera_projection),
275        MirrorCamera,
276    ));
277}
278
279/// Spawns the animated fox.
280///
281/// Note that this doesn't play the animation; that's handled in
282/// [`play_fox_animation`].
283fn spawn_fox(commands: &mut Commands, asset_server: &AssetServer) {
284    commands.spawn((
285        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(FOX_ASSET_PATH))),
286        Transform::from_xyz(-50.0, 0.0, -100.0),
287    ));
288}
289
290/// Spawns the mirror plane mesh and returns its transform.
291fn spawn_mirror(
292    commands: &mut Commands,
293    meshes: &mut Assets<Mesh>,
294    screen_space_texture_materials: &mut Assets<
295        ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>,
296    >,
297    mirror_render_target: Handle<Image>,
298) -> Transform {
299    let mirror_transform = Transform::from_scale(vec3(300.0, 1.0, 150.0))
300        .with_rotation(Quat::from_rotation_x(MIRROR_ROTATION_ANGLE))
301        .with_translation(MIRROR_POSITION);
302
303    commands.spawn((
304        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
305        MeshMaterial3d(screen_space_texture_materials.add(ExtendedMaterial {
306            base: StandardMaterial {
307                base_color: Color::BLACK,
308                emissive: Color::WHITE.into(),
309                emissive_texture: Some(mirror_render_target),
310                perceptual_roughness: 0.0,
311                metallic: 1.0,
312                ..default()
313            },
314            extension: ScreenSpaceTextureExtension { dummy: 0.0 },
315        })),
316        mirror_transform,
317        Mirror,
318    ));
319
320    mirror_transform
321}
```

Hide additional examples

examples/ecs/parallel\_query.rs ([line 21](../../src/parallel_query/parallel_query.rs.html#21))

```rust
10fn spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
11    commands.spawn(Camera2d);
12    let texture = asset_server.load("branding/icon.png");
13
14    // We're seeding the PRNG here to make this example deterministic for testing purposes.
15    // This isn't strictly required in practical use unless you need your app to be deterministic.
16    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
17    for z in 0..128 {
18        commands.spawn((
19            Sprite::from_image(texture.clone()),
20            Transform::from_scale(Vec3::splat(0.1))
21                .with_translation(Vec2::splat(0.0).extend(z as f32)),
22            Velocity(20.0 * Vec2::new(rng.random::<f32>() - 0.5, rng.random::<f32>() - 0.5)),
23        ));
24    }
25}
```

examples/3d/pcss.rs ([line 200](../../src/pcss/pcss.rs.html#200))

```rust
187fn spawn_light(commands: &mut Commands, app_status: &AppStatus) {
188    // Because this light can become a directional light, point light, or spot
189    // light depending on the settings, we add the union of the components
190    // necessary for this light to behave as all three of those.
191    commands
192        .spawn((
193            create_directional_light(app_status),
194            Transform::from_rotation(Quat::from_array([
195                0.6539259,
196                -0.34646285,
197                0.36505926,
198                -0.5648683,
199            ]))
200            .with_translation(vec3(57.693, 34.334, -6.422)),
201        ))
202        // These two are needed for point lights.
203        .insert(CubemapVisibleEntities::default())
204        .insert(CubemapFrusta::default())
205        // These two are needed for spot lights.
206        .insert(VisibleMeshEntities::default())
207        .insert(Frustum::default());
208}
```

examples/3d/light\_probe\_blending.rs ([line 347](../../src/light_probe_blending/light_probe_blending.rs.html#347))

```rust
317fn spawn_light_probes(commands: &mut Commands, asset_server: &AssetServer) {
318    // Spawn the first room's light probe.
319    commands.spawn((
320        LightProbe {
321            falloff: Vec3::splat(LIGHT_PROBE_FALLOFF),
322        },
323        EnvironmentMapLight {
324            diffuse_map: asset_server.load(get_web_asset_url("diffuse_room1.ktx2")),
325            specular_map: asset_server.load(get_web_asset_url("specular_room1.ktx2")),
326            intensity: LIGHT_PROBE_INTENSITY,
327            ..default()
328        },
329        Transform::from_scale(vec3(1.0, -1.0, 1.0) * LIGHT_PROBE_SIDE_LENGTH)
330            .with_rotation(Quat::from_rotation_x(PI)),
331        ParallaxCorrection::Custom(Vec3::splat(LIGHT_PROBE_PARALLAX_CORRECTION_SIDE_LENGTH)),
332    ));
333
334    // Spawn the second room's light probe.
335    commands.spawn((
336        LightProbe {
337            falloff: Vec3::splat(LIGHT_PROBE_FALLOFF),
338        },
339        EnvironmentMapLight {
340            diffuse_map: asset_server.load(get_web_asset_url("diffuse_room2.ktx2")),
341            specular_map: asset_server.load(get_web_asset_url("specular_room2.ktx2")),
342            intensity: LIGHT_PROBE_INTENSITY,
343            ..default()
344        },
345        Transform::from_scale(vec3(1.0, -1.0, 1.0) * LIGHT_PROBE_SIDE_LENGTH)
346            .with_rotation(Quat::from_rotation_x(PI))
347            .with_translation(vec3(0.0, 0.0, -ROOM_SEPARATION)),
348        ParallaxCorrection::Custom(Vec3::splat(LIGHT_PROBE_PARALLAX_CORRECTION_SIDE_LENGTH)),
349    ));
350}
```

examples/3d/atmosphere.rs ([line 237](../../src/atmosphere/atmosphere.rs.html#237))

```rust
212fn setup_terrain_scene(
213    mut commands: Commands,
214    mut meshes: ResMut<Assets<Mesh>>,
215    mut water_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, Water>>>,
216    asset_server: Res<AssetServer>,
217) {
218    // Sun
219    commands.spawn((
220        DirectionalLight {
221            shadow_maps_enabled: true,
222            // lux::RAW_SUNLIGHT is recommended for use with this feature, since
223            // other values approximate sunlight *post-scattering* in various
224            // conditions. RAW_SUNLIGHT in comparison is the illuminance of the
225            // sun unfiltered by the atmosphere, so it is the proper input for
226            // sunlight to be filtered by the atmosphere.
227            illuminance: lux::RAW_SUNLIGHT,
228            ..default()
229        },
230        Transform::from_xyz(1.0, 0.4, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
231        VolumetricLight,
232    ));
233
234    // spawn the fog volume
235    commands.spawn((
236        FogVolume::default(),
237        Transform::from_scale(Vec3::new(10.0, 1.0, 10.0)).with_translation(Vec3::Y * 0.5),
238    ));
239
240    // Terrain
241    commands.spawn((
242        Terrain,
243        WorldAssetRoot(
244            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/terrain/terrain.glb")),
245        ),
246        Transform::from_xyz(-1.0, 0.0, -0.5)
247            .with_scale(Vec3::splat(0.5))
248            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
249    ));
250
251    spawn_water(
252        &mut commands,
253        &asset_server,
254        &mut meshes,
255        &mut water_materials,
256    );
257}
```

examples/3d/fog.rs ([line 87](../../src/fog/fog.rs.html#87))

```rust
56fn setup_pyramid_scene(
57    mut commands: Commands,
58    mut meshes: ResMut<Assets<Mesh>>,
59    mut materials: ResMut<Assets<StandardMaterial>>,
60) {
61    let stone = materials.add(StandardMaterial {
62        base_color: Srgba::hex("28221B").unwrap().into(),
63        perceptual_roughness: 1.0,
64        ..default()
65    });
66
67    // pillars
68    for (x, z) in &[(-1.5, -1.5), (1.5, -1.5), (1.5, 1.5), (-1.5, 1.5)] {
69        commands.spawn((
70            Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
71            MeshMaterial3d(stone.clone()),
72            Transform::from_xyz(*x, 1.5, *z),
73        ));
74    }
75
76    // orb
77    commands.spawn((
78        Mesh3d(meshes.add(Sphere::default())),
79        MeshMaterial3d(materials.add(StandardMaterial {
80            base_color: Srgba::hex("126212CC").unwrap().into(),
81            reflectance: 1.0,
82            perceptual_roughness: 0.0,
83            metallic: 0.5,
84            alpha_mode: AlphaMode::Blend,
85            ..default()
86        })),
87        Transform::from_scale(Vec3::splat(1.75)).with_translation(Vec3::new(0.0, 4.0, 0.0)),
88        NotShadowCaster,
89        NotShadowReceiver,
90    ));
91
92    // steps
93    for i in 0..50 {
94        let half_size = i as f32 / 2.0 + 3.0;
95        let y = -i as f32 / 2.0;
96        commands.spawn((
97            Mesh3d(meshes.add(Cuboid::new(2.0 * half_size, 0.5, 2.0 * half_size))),
98            MeshMaterial3d(stone.clone()),
99            Transform::from_xyz(0.0, y + 0.25, 0.0),
100        ));
101    }
102
103    // sky
104    commands.spawn((
105        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.0))),
106        MeshMaterial3d(materials.add(StandardMaterial {
107            base_color: Srgba::hex("888888").unwrap().into(),
108            unlit: true,
109            cull_mode: None,
110            ..default()
111        })),
112        Transform::from_scale(Vec3::splat(1_000_000.0)),
113    ));
114
115    // light
116    commands.spawn((
117        PointLight {
118            shadow_maps_enabled: true,
119            ..default()
120        },
121        Transform::from_xyz(0.0, 1.0, 0.0),
122    ));
123}
```

Additional examples can be found in:  

*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#123)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#586)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#92)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#103)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#316)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#250)

#### pub const fn [with\_rotation](#method.with_rotation)(self, rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") with a new rotation.

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/3d/post\_processing.rs ([line 123](../../src/post_processing/post_processing.rs.html#123))

```rust
111fn spawn_scene(commands: &mut Commands, asset_server: &AssetServer) {
112    // Spawn the main scene.
113    commands.spawn(WorldAssetRoot(asset_server.load(
114        GltfAssetLabel::Scene(0).from_asset("models/TonemappingTest/TonemappingTest.gltf"),
115    )));
116
117    // Spawn the flight helmet.
118    commands.spawn((
119        WorldAssetRoot(
120            asset_server
121                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
122        ),
123        Transform::from_xyz(0.5, 0.0, -0.5).with_rotation(Quat::from_rotation_y(-0.15 * PI)),
124    ));
125
126    // Spawn the light.
127    commands.spawn((
128        DirectionalLight {
129            illuminance: 15000.0,
130            shadow_maps_enabled: true,
131            ..default()
132        },
133        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
134        CascadeShadowConfigBuilder {
135            maximum_distance: 3.0,
136            first_cascade_far_bound: 0.9,
137            ..default()
138        }
139        .build(),
140    ));
141}
```

Hide additional examples

examples/3d/color\_grading.rs ([line 357](../../src/color_grading/color_grading.rs.html#357))

```rust
345fn add_basic_scene(commands: &mut Commands, asset_server: &AssetServer) {
346    // Spawn the main scene.
347    commands.spawn(WorldAssetRoot(asset_server.load(
348        GltfAssetLabel::Scene(0).from_asset("models/TonemappingTest/TonemappingTest.gltf"),
349    )));
350
351    // Spawn the flight helmet.
352    commands.spawn((
353        WorldAssetRoot(
354            asset_server
355                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
356        ),
357        Transform::from_xyz(0.5, 0.0, -0.5).with_rotation(Quat::from_rotation_y(-0.15 * PI)),
358    ));
359
360    // Spawn the light.
361    commands.spawn((
362        DirectionalLight {
363            illuminance: 15000.0,
364            shadow_maps_enabled: true,
365            ..default()
366        },
367        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
368        CascadeShadowConfigBuilder {
369            maximum_distance: 3.0,
370            first_cascade_far_bound: 0.9,
371            ..default()
372        }
373        .build(),
374    ));
375}
```

examples/3d/mirror.rs ([line 300](../../src/mirror/mirror.rs.html#300))

```rust
291fn spawn_mirror(
292    commands: &mut Commands,
293    meshes: &mut Assets<Mesh>,
294    screen_space_texture_materials: &mut Assets<
295        ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>,
296    >,
297    mirror_render_target: Handle<Image>,
298) -> Transform {
299    let mirror_transform = Transform::from_scale(vec3(300.0, 1.0, 150.0))
300        .with_rotation(Quat::from_rotation_x(MIRROR_ROTATION_ANGLE))
301        .with_translation(MIRROR_POSITION);
302
303    commands.spawn((
304        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
305        MeshMaterial3d(screen_space_texture_materials.add(ExtendedMaterial {
306            base: StandardMaterial {
307                base_color: Color::BLACK,
308                emissive: Color::WHITE.into(),
309                emissive_texture: Some(mirror_render_target),
310                perceptual_roughness: 0.0,
311                metallic: 1.0,
312                ..default()
313            },
314            extension: ScreenSpaceTextureExtension { dummy: 0.0 },
315        })),
316        mirror_transform,
317        Mirror,
318    ));
319
320    mirror_transform
321}
```

examples/3d/pcss.rs ([lines 162-164](../../src/pcss/pcss.rs.html#162-164))

```rust
158fn spawn_camera(commands: &mut Commands, asset_server: &AssetServer) {
159    commands
160        .spawn((
161            Camera3d::default(),
162            Transform::from_xyz(-12.912 * 0.7, 4.466 * 0.7, -10.624 * 0.7).with_rotation(
163                Quat::from_euler(EulerRot::YXZ, -134.76 / 180.0 * PI, -0.175, 0.0),
164            ),
165            #[cfg(feature = "free_camera")]
166            FreeCamera::default(),
167        ))
168        .insert(ShadowFilteringMethod::Gaussian)
169        // `TemporalJitter` is needed for TAA. Note that it does nothing without
170        // `TemporalAntiAliasSettings`.
171        .insert(TemporalJitter::default())
172        // We want MSAA off for TAA to work properly.
173        .insert(Msaa::Off)
174        // The depth prepass is needed for TAA.
175        .insert(DepthPrepass)
176        // The motion vector prepass is needed for TAA.
177        .insert(MotionVectorPrepass)
178        // Add a nice skybox.
179        .insert(Skybox {
180            image: Some(asset_server.load("environment_maps/sky_skybox.ktx2")),
181            brightness: 500.0,
182            rotation: Quat::IDENTITY,
183        });
184}
```

examples/3d/tonemapping.rs ([line 115](../../src/tonemapping/tonemapping.rs.html#115))

```rust
100fn setup_basic_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
101    // Main scene
102    commands.spawn((
103        WorldAssetRoot(asset_server.load(
104            GltfAssetLabel::Scene(0).from_asset("models/TonemappingTest/TonemappingTest.gltf"),
105        )),
106        SceneNumber(1),
107    ));
108
109    // Flight Helmet
110    commands.spawn((
111        WorldAssetRoot(
112            asset_server
113                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
114        ),
115        Transform::from_xyz(0.5, 0.0, -0.5).with_rotation(Quat::from_rotation_y(-0.15 * PI)),
116        SceneNumber(1),
117    ));
118
119    // light
120    commands.spawn((
121        DirectionalLight {
122            illuminance: 15_000.,
123            shadow_maps_enabled: true,
124            ..default()
125        },
126        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
127        CascadeShadowConfigBuilder {
128            maximum_distance: 3.0,
129            first_cascade_far_bound: 0.9,
130            ..default()
131        }
132        .build(),
133        SceneNumber(1),
134    ));
135}
```

examples/3d/skybox.rs ([line 68](../../src/skybox/skybox.rs.html#68))

```rust
61fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
62    // directional 'sun' light
63    commands.spawn((
64        DirectionalLight {
65            illuminance: 32000.0,
66            ..default()
67        },
68        Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
69    ));
70
71    let skybox_handle = asset_server.load(CUBEMAPS[0].0);
72    // camera
73    commands.spawn((
74        Camera3d::default(),
75        Msaa::Off,
76        #[cfg(any(feature = "webgpu", not(target_arch = "wasm32")))]
77        TemporalAntiAliasing::default(),
78        ScreenSpaceAmbientOcclusion::default(),
79        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
80        FreeCamera::default(),
81        Skybox {
82            image: Some(skybox_handle.clone()),
83            brightness: 1000.0,
84            ..default()
85        },
86    ));
87
88    // ambient light
89    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
90    // environment map, use an appropriate color and brightness to match
91    commands.insert_resource(GlobalAmbientLight {
92        color: Color::srgb_u8(210, 220, 240),
93        brightness: 1.0,
94        ..default()
95    });
96
97    commands.insert_resource(Cubemap {
98        is_loaded: false,
99        index: 0,
100        image_handle: skybox_handle,
101    });
102}
```

Additional examples can be found in:  

*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#82)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#330)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#248)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#45)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#226)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#64)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#262)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#76)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#59)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#145)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#74-77)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#81)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#98)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#104)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#217)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#89)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#104)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#147)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#185)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#143)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#197)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#76-81)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#258)

#### pub const fn [with\_scale](#method.with_scale)(self, scale: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") with a new scale.

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 67](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#67))

```rust
63fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
64    commands.spawn((
65        PendingScene(asset_server.load("models/animated/Fox.glb")),
66        Transform::from_xyz(1.3, 4.3, 0.0)
67            .with_scale(Vec3::splat(0.08))
68            .looking_to(-Vec3::X, Vec3::Y),
69    ));
70}
```

Hide additional examples

examples/3d/clustered\_decals.rs ([line 342](../../src/clustered_decals/clustered_decals.rs.html#342))

```rust
338fn calculate_initial_decal_transform(start: Vec3, looking_at: Vec3, size: Vec2) -> Transform {
339    let direction = looking_at - start;
340    let center = start + direction * 0.5;
341    Transform::from_translation(center)
342        .with_scale((size * 0.5).extend(direction.length()))
343        .looking_to(direction, Vec3::Y)
344}
```

examples/3d/irradiance\_volumes.rs ([line 273](../../src/irradiance_volumes/irradiance_volumes.rs.html#273))

```rust
268fn spawn_sphere(commands: &mut Commands, assets: &ExampleAssets) {
269    commands
270        .spawn((
271            Mesh3d(assets.main_sphere.clone()),
272            MeshMaterial3d(assets.main_sphere_material.clone()),
273            Transform::from_xyz(0.0, SPHERE_SCALE, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
274        ))
275        .insert(MainObject);
276}
```

examples/2d/mesh2d.rs ([line 22](../../src/mesh2d/mesh2d.rs.html#22))

```rust
12fn setup(
13    mut commands: Commands,
14    mut meshes: ResMut<Assets<Mesh>>,
15    mut materials: ResMut<Assets<ColorMaterial>>,
16) {
17    commands.spawn(Camera2d);
18
19    commands.spawn((
20        Mesh2d(meshes.add(Rectangle::default())),
21        MeshMaterial2d(materials.add(Color::from(PURPLE))),
22        Transform::default().with_scale(Vec3::splat(128.)),
23    ));
24}
```

examples/2d/pixel\_grid\_snap.rs ([line 78](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#78))

```rust
70fn setup_mesh(
71    mut commands: Commands,
72    mut meshes: ResMut<Assets<Mesh>>,
73    mut materials: ResMut<Assets<ColorMaterial>>,
74) {
75    commands.spawn((
76        Mesh2d(meshes.add(Capsule2d::default())),
77        MeshMaterial2d(materials.add(Color::BLACK)),
78        Transform::from_xyz(25., 0., 2.).with_scale(Vec3::splat(32.)),
79        Rotate,
80        PIXEL_PERFECT_LAYERS,
81    ));
82}
```

examples/shader/shader\_material\_2d.rs ([line 41](../../src/shader_material_2d/shader_material_2d.rs.html#41))

```rust
25fn setup(
26    mut commands: Commands,
27    mut meshes: ResMut<Assets<Mesh>>,
28    mut materials: ResMut<Assets<CustomMaterial>>,
29    asset_server: Res<AssetServer>,
30) {
31    // camera
32    commands.spawn(Camera2d);
33
34    // quad
35    commands.spawn((
36        Mesh2d(meshes.add(Rectangle::default())),
37        MeshMaterial2d(materials.add(CustomMaterial {
38            color: LinearRgba::BLUE,
39            color_texture: Some(asset_server.load("branding/icon.png")),
40        })),
41        Transform::default().with_scale(Vec3::splat(128.)),
42    ));
43}
```

Additional examples can be found in:  

*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#230)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#383)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#621)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#88)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#121)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#381)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#41)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#33)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#247)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#225)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#58)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#82)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#253)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#85)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#330)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#77)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#80)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#112)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#135)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#91)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#204)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#147)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#184)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#273-277)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#141)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#266)

#### pub fn [to\_matrix](#method.to_matrix)(&self) -> [Mat4](struct.Mat4.html "struct bevy::prelude::Mat4")

Computes the 3d affine transformation matrix from this transform’s translation, rotation, and scale.

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 355](../../src/mirror/mirror.rs.html#355))

```rust
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#273)

#### pub fn [compute\_affine](#method.compute_affine)(&self) -> [Affine3A](../math/struct.Affine3A.html "struct bevy::math::Affine3A")

Returns the 3d affine transformation matrix from this transforms translation, rotation, and scale.

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 368](../../src/mirror/mirror.rs.html#368))

```rust
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#279)

#### pub fn [local\_x](#method.local_x)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Get the unit vector in the local `X` direction.

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/transforms/translation.rs ([line 68](../../src/translation/translation.rs.html#68))

```rust
62fn move_cube(mut cubes: Query<(&mut Transform, &mut Movable)>, timer: Res<Time>) {
63    for (mut transform, mut cube) in &mut cubes {
64        // Check if the entity moved too far from its spawn, if so invert the moving direction.
65        if (cube.spawn - transform.translation).length() > cube.max_distance {
66            cube.speed *= -1.0;
67        }
68        let direction = transform.local_x();
69        transform.translation += direction * cube.speed * timer.delta_secs();
70    }
71}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#286)

#### pub fn [left](#method.left)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Equivalent to [`-local_x()`](struct.Transform.html#method.local_x "method bevy::prelude::Transform::local_x")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#292)

#### pub fn [right](#method.right)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Equivalent to [`local_x()`](struct.Transform.html#method.local_x "method bevy::prelude::Transform::local_x")

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

examples/ecs/fallible\_params.rs ([line 128](../../src/fallible_params/fallible_params.rs.html#128))

```rust
124fn move_targets(mut enemies: Populated<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
125    for (mut transform, mut target) in &mut *enemies {
126        target.rotation += target.rotation_speed * time.delta_secs();
127        transform.rotation = Quat::from_rotation_z(target.rotation);
128        let offset = transform.right() * target.radius;
129        transform.translation = target.origin.extend(0.0) + offset;
130    }
131}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#298)

#### pub fn [local\_y](#method.local_y)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Get the unit vector in the local `Y` direction.

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

examples/transforms/transform.rs ([line 114](../../src/transform/transform.rs.html#114))

```rust
101fn rotate_cube(
102    mut cubes: Query<(&mut Transform, &mut CubeState), Without<Center>>,
103    center_spheres: Query<&Transform, With<Center>>,
104    timer: Res<Time>,
105) {
106    // Calculate the point to circle around. (The position of the center_sphere)
107    let mut center: Vec3 = Vec3::ZERO;
108    for sphere in &center_spheres {
109        center += sphere.translation;
110    }
111    // Update the rotation of the cube(s).
112    for (mut transform, cube) in &mut cubes {
113        // Calculate the rotation of the cube if it would be looking at the sphere in the center.
114        let look_at_sphere = transform.looking_at(center, *transform.local_y());
115        // Interpolate between the current rotation and the fully turned rotation
116        // when looking at the sphere, with a given turn speed to get a smooth motion.
117        // With higher speed the curvature of the orbit would be smaller.
118        let incremental_turn_weight = cube.turn_speed * timer.delta_secs();
119        let old_rotation = transform.rotation;
120        transform.rotation = old_rotation.lerp(look_at_sphere.rotation, incremental_turn_weight);
121    }
122}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#305)

#### pub fn [up](#method.up)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Equivalent to [`local_y()`](struct.Transform.html#method.local_y "method bevy::prelude::Transform::local_y")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#311)

#### pub fn [down](#method.down)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Equivalent to [`-local_y()`](struct.Transform.html#method.local_y "method bevy::prelude::Transform::local_y")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#317)

#### pub fn [local\_z](#method.local_z)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Get the unit vector in the local `Z` direction.

##### [Examples found in repository](#scraped-examples-16)[?](../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 662](../../src/ssr/ssr.rs.html#662))

```rust
634fn move_camera(
635    keyboard_input: Res<ButtonInput<KeyCode>>,
636    mut mouse_wheel_reader: MessageReader<MouseWheel>,
637    mut cameras: Query<&mut Transform, With<Camera>>,
638) {
639    let (mut distance_delta, mut theta_delta) = (0.0, 0.0);
640
641    // Handle keyboard events.
642    if keyboard_input.pressed(KeyCode::KeyW) {
643        distance_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;
644    }
645    if keyboard_input.pressed(KeyCode::KeyS) {
646        distance_delta += CAMERA_KEYBOARD_ZOOM_SPEED;
647    }
648    if keyboard_input.pressed(KeyCode::KeyA) {
649        theta_delta += CAMERA_KEYBOARD_ORBIT_SPEED;
650    }
651    if keyboard_input.pressed(KeyCode::KeyD) {
652        theta_delta -= CAMERA_KEYBOARD_ORBIT_SPEED;
653    }
654
655    // Handle mouse events.
656    for mouse_wheel in mouse_wheel_reader.read() {
657        distance_delta -= mouse_wheel.y * CAMERA_MOUSE_WHEEL_ZOOM_SPEED;
658    }
659
660    // Update transforms.
661    for mut camera_transform in cameras.iter_mut() {
662        let local_z = camera_transform.local_z().as_vec3().normalize_or_zero();
663        if distance_delta != 0.0 {
664            camera_transform.translation = (camera_transform.translation.length() + distance_delta)
665                .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end)
666                * local_z;
667        }
668        if theta_delta != 0.0 {
669            camera_transform
670                .translate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, theta_delta));
671            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
672        }
673    }
674}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#324)

#### pub fn [forward](#method.forward)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Equivalent to [`-local_z()`](struct.Transform.html#method.local_z "method bevy::prelude::Transform::local_z")

##### [Examples found in repository](#scraped-examples-17)[?](../../scrape-examples-help.html)

examples/transforms/transform.rs ([line 93](../../src/transform/transform.rs.html#93))

```rust
90fn move_cube(mut cubes: Query<(&mut Transform, &mut CubeState)>, timer: Res<Time>) {
91    for (mut transform, cube) in &mut cubes {
92        // Move the cube forward smoothly at a given move_speed.
93        let forward = transform.forward();
94        transform.translation += forward * cube.move_speed * timer.delta_secs();
95    }
96}
```

Hide additional examples

examples/3d/tonemapping.rs ([line 144](../../src/tonemapping/tonemapping.rs.html#144))

```rust
137fn setup_color_gradient_scene(
138    mut commands: Commands,
139    mut meshes: ResMut<Assets<Mesh>>,
140    mut materials: ResMut<Assets<ColorGradientMaterial>>,
141    camera_transform: Res<CameraTransform>,
142) {
143    let mut transform = camera_transform.0;
144    transform.translation += *transform.forward();
145
146    commands.spawn((
147        Mesh3d(meshes.add(Rectangle::new(0.7, 0.7))),
148        MeshMaterial3d(materials.add(ColorGradientMaterial {})),
149        transform,
150        Visibility::Hidden,
151        SceneNumber(2),
152    ));
153}
154
155fn setup_image_viewer_scene(
156    mut commands: Commands,
157    mut meshes: ResMut<Assets<Mesh>>,
158    mut materials: ResMut<Assets<StandardMaterial>>,
159    camera_transform: Res<CameraTransform>,
160) {
161    let mut transform = camera_transform.0;
162    transform.translation += *transform.forward();
163
164    // exr/hdr viewer (exr requires enabling bevy feature)
165    commands.spawn((
166        Mesh3d(meshes.add(Rectangle::default())),
167        MeshMaterial3d(materials.add(StandardMaterial {
168            base_color_texture: None,
169            unlit: true,
170            ..default()
171        })),
172        transform,
173        Visibility::Hidden,
174        SceneNumber(3),
175        HDRViewer,
176    ));
177
178    commands.spawn((
179        Text::new("Drag and drop an HDR or EXR file"),
180        TextFont {
181            font_size: FontSize::Px(36.0),
182            ..default()
183        },
184        TextColor(Color::BLACK),
185        TextLayout::justify(Justify::Center),
186        Node {
187            align_self: AlignSelf::Center,
188            margin: UiRect::all(auto()),
189            ..default()
190        },
191        SceneNumber(3),
192        Visibility::Hidden,
193    ));
194}
```

examples/3d/motion\_blur.rs ([line 347](../../src/motion_blur/motion_blur.rs.html#347))

```rust
330fn move_camera(
331    camera: Single<(&mut Transform, &mut Projection), Without<CameraTracked>>,
332    tracked: Single<&Transform, With<CameraTracked>>,
333    mode: Res<CameraMode>,
334) {
335    let (mut transform, mut projection) = camera.into_inner();
336    match *mode {
337        CameraMode::Track => {
338            transform.look_at(tracked.translation, Vec3::Y);
339            transform.translation = Vec3::new(15.0, -0.5, 0.0);
340            if let Projection::Perspective(perspective) = &mut *projection {
341                perspective.fov = 0.05;
342            }
343        }
344        CameraMode::Chase => {
345            transform.translation =
346                tracked.translation + Vec3::new(0.0, 0.15, 0.0) + tracked.back() * 0.6;
347            transform.look_to(tracked.forward(), Vec3::Y);
348            if let Projection::Perspective(perspective) = &mut *projection {
349                perspective.fov = 1.0;
350            }
351        }
352    }
353}
```

examples/camera/camera\_orbit.rs ([line 140](../../src/camera_orbit/camera_orbit.rs.html#140))

```rust
99fn orbit(
100    mut camera: Single<&mut Transform, With<Camera>>,
101    camera_settings: Res<CameraSettings>,
102    mouse_buttons: Res<ButtonInput<MouseButton>>,
103    mouse_motion: Res<AccumulatedMouseMotion>,
104    time: Res<Time>,
105) {
106    let delta = mouse_motion.delta;
107    let mut delta_roll = 0.0;
108
109    if mouse_buttons.pressed(MouseButton::Left) {
110        delta_roll -= 1.0;
111    }
112    if mouse_buttons.pressed(MouseButton::Right) {
113        delta_roll += 1.0;
114    }
115
116    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
117    // as we are already receiving the full movement since the last frame was rendered. Multiplying
118    // by delta time here would make the movement slower that it should be.
119    let delta_pitch = delta.y * camera_settings.pitch_speed;
120    let delta_yaw = delta.x * camera_settings.yaw_speed;
121
122    // Conversely, we DO need to factor in delta time for mouse button inputs.
123    delta_roll *= camera_settings.roll_speed * time.delta_secs();
124
125    // Obtain the existing pitch, yaw, and roll values from the transform.
126    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
127
128    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
129    let pitch = (pitch + delta_pitch).clamp(
130        camera_settings.pitch_range.start,
131        camera_settings.pitch_range.end,
132    );
133    let roll = roll + delta_roll;
134    let yaw = yaw + delta_yaw;
135    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
136
137    // Adjust the translation to maintain the correct orientation toward the orbit target.
138    // In our example it's a static target, but this could easily be customized.
139    let target = Vec3::ZERO;
140    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
141}
```

examples/3d/mirror.rs ([line 569](../../src/mirror/mirror.rs.html#569))

```rust
529fn move_camera_on_mouse_down(
530    mut main_cameras_query: Query<&mut Transform, (With<Camera>, Without<MirrorCamera>)>,
531    interactions_query: Query<&Interaction, With<RadioButton>>,
532    mouse_buttons: Res<ButtonInput<MouseButton>>,
533    mouse_motion: Res<AccumulatedMouseMotion>,
534    app_status: Res<AppStatus>,
535) {
536    // Only process the mouse motion if the left mouse button is pressed, the
537    // mouse action is set to move the fox, and the pointer isn't over a UI
538    // widget.
539    if app_status.drag_action != DragAction::MoveCamera
540        || !mouse_buttons.pressed(MouseButton::Left)
541        || interactions_query
542            .iter()
543            .any(|interaction| *interaction != Interaction::None)
544    {
545        return;
546    }
547
548    let delta = mouse_motion.delta;
549
550    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
551    // as we are already receiving the full movement since the last frame was rendered. Multiplying
552    // by delta time here would make the movement slower that it should be.
553    let delta_pitch = delta.y * CAMERA_PITCH_SPEED;
554    let delta_yaw = delta.x * CAMERA_YAW_SPEED;
555
556    for mut main_camera_transform in &mut main_cameras_query {
557        // Obtain the existing pitch and yaw values from the transform.
558        let (yaw, pitch, _) = main_camera_transform.rotation.to_euler(EulerRot::YXZ);
559
560        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
561        let pitch = (pitch + delta_pitch).clamp(-CAMERA_PITCH_LIMIT, CAMERA_PITCH_LIMIT);
562        let yaw = yaw + delta_yaw;
563        main_camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
564
565        // Adjust the translation to maintain the correct orientation toward the orbit target.
566        // In our example it's a static target, but this could easily be customized.
567        let target = Vec3::ZERO;
568        main_camera_transform.translation =
569            target - main_camera_transform.forward() * CAMERA_ORBIT_DISTANCE;
570    }
571}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#330)

#### pub fn [back](#method.back)(&self) -> [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

Equivalent to [`local_z()`](struct.Transform.html#method.local_z "method bevy::prelude::Transform::local_z")

##### [Examples found in repository](#scraped-examples-18)[?](../../scrape-examples-help.html)

examples/3d/motion\_blur.rs ([line 346](../../src/motion_blur/motion_blur.rs.html#346))

```rust
330fn move_camera(
331    camera: Single<(&mut Transform, &mut Projection), Without<CameraTracked>>,
332    tracked: Single<&Transform, With<CameraTracked>>,
333    mode: Res<CameraMode>,
334) {
335    let (mut transform, mut projection) = camera.into_inner();
336    match *mode {
337        CameraMode::Track => {
338            transform.look_at(tracked.translation, Vec3::Y);
339            transform.translation = Vec3::new(15.0, -0.5, 0.0);
340            if let Projection::Perspective(perspective) = &mut *projection {
341                perspective.fov = 0.05;
342            }
343        }
344        CameraMode::Chase => {
345            transform.translation =
346                tracked.translation + Vec3::new(0.0, 0.15, 0.0) + tracked.back() * 0.6;
347            transform.look_to(tracked.forward(), Vec3::Y);
348            if let Projection::Perspective(perspective) = &mut *projection {
349                perspective.fov = 1.0;
350            }
351        }
352    }
353}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#344)

#### pub fn [rotate](#method.rotate)(&mut self, rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat"))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") by the given rotation.

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the `rotation` is relative to the rotation of the parent.

##### Examples

*   [`3d_rotation`](https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs)

##### [Examples found in repository](#scraped-examples-19)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_animated\_sprite\_meshes.rs ([line 102](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#102))

```rust
101fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
102    camera_transform.rotate(Quat::from_rotation_z(time.delta_secs() * 0.5));
103    **camera_transform = **camera_transform
104        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
105}
```

Hide additional examples

examples/stress\_tests/many\_animated\_sprites.rs ([line 100](../../src/many_animated_sprites/many_animated_sprites.rs.html#100))

```rust
99fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
100    camera_transform.rotate(Quat::from_rotation_z(time.delta_secs() * 0.5));
101    **camera_transform = **camera_transform
102        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
103}
```

examples/3d/occlusion\_culling.rs ([lines 345-350](../../src/occlusion_culling/occlusion_culling.rs.html#345-350))

```rust
343fn spin_large_cube(mut large_cubes: Query<&mut Transform, With<LargeCube>>) {
344    for mut transform in &mut large_cubes {
345        transform.rotate(Quat::from_euler(
346            EulerRot::XYZ,
347            0.13 * ROTATION_SPEED,
348            0.29 * ROTATION_SPEED,
349            0.35 * ROTATION_SPEED,
350        ));
351    }
352}
```

examples/shader/shader\_material\_wesl.rs ([line 87](../../src/shader_material_wesl/shader_material_wesl.rs.html#87))

```rust
73fn update(
74    time: Res<Time>,
75    mut query: Query<(&MeshMaterial3d<CustomMaterial>, &mut Transform)>,
76    mut materials: ResMut<Assets<CustomMaterial>>,
77    keys: Res<ButtonInput<KeyCode>>,
78) {
79    for (material, mut transform) in query.iter_mut() {
80        let mut material = materials.get_mut(material).unwrap();
81        material.time.x = time.elapsed_secs();
82        if keys.just_pressed(KeyCode::Space) {
83            material.party_mode = !material.party_mode;
84        }
85
86        if material.party_mode {
87            transform.rotate(Quat::from_rotation_y(0.005));
88        }
89    }
90}
```

examples/3d/transmission.rs ([line 607](../../src/transmission/transmission.rs.html#607))

```rust
594fn flicker_system(
595    mut flame: Single<&mut Transform, (With<Flicker>, With<Mesh3d>)>,
596    light: Single<(&mut PointLight, &mut Transform), (With<Flicker>, Without<Mesh3d>)>,
597    time: Res<Time>,
598) {
599    let s = time.elapsed_secs();
600    let a = ops::cos(s * 6.0) * 0.0125 + ops::cos(s * 4.0) * 0.025;
601    let b = ops::cos(s * 5.0) * 0.0125 + ops::cos(s * 3.0) * 0.025;
602    let c = ops::cos(s * 7.0) * 0.0125 + ops::cos(s * 2.0) * 0.025;
603    let (mut light, mut light_transform) = light.into_inner();
604    light.intensity = 4_000.0 + 3000.0 * (a + b + c);
605    flame.translation = Vec3::new(-1.0, 1.23, 0.0);
606    flame.look_at(Vec3::new(-1.0 - c, 1.7 - b, 0.0 - a), Vec3::X);
607    flame.rotate(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, PI / 2.0));
608    light_transform.translation = Vec3::new(-1.0 - c, 1.7, 0.0 - a);
609    flame.translation = Vec3::new(-1.0 - c, 1.23, 0.0 - a);
610}
```

examples/3d/volumetric\_fog.rs ([line 192](../../src/volumetric_fog/volumetric_fog.rs.html#192))

```rust
168fn move_directional_light(
169    input: Res<ButtonInput<KeyCode>>,
170    mut directional_lights: Query<&mut Transform, With<DirectionalLight>>,
171) {
172    let mut delta_theta = Vec2::ZERO;
173    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
174        delta_theta.y += DIRECTIONAL_LIGHT_MOVEMENT_SPEED;
175    }
176    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
177        delta_theta.y -= DIRECTIONAL_LIGHT_MOVEMENT_SPEED;
178    }
179    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
180        delta_theta.x += DIRECTIONAL_LIGHT_MOVEMENT_SPEED;
181    }
182    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
183        delta_theta.x -= DIRECTIONAL_LIGHT_MOVEMENT_SPEED;
184    }
185
186    if delta_theta == Vec2::ZERO {
187        return;
188    }
189
190    let delta_quat = Quat::from_euler(EulerRot::XZY, delta_theta.y, 0.0, delta_theta.x);
191    for mut transform in directional_lights.iter_mut() {
192        transform.rotate(delta_quat);
193    }
194}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#359)

#### pub fn [rotate\_axis](#method.rotate_axis)(&mut self, axis: [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3"), angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around the given `axis` by `angle` (in radians).

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the `axis` is relative to the rotation of the parent.

##### Warning

If you pass in an `axis` based on the current rotation (e.g. obtained via [`Transform::local_x`](struct.Transform.html#method.local_x "method bevy::prelude::Transform::local_x")), floating point errors can accumulate exponentially when applying rotations repeatedly this way. This will result in a denormalized rotation. In this case, it is recommended to normalize the [`Transform::rotation`](struct.Transform.html#structfield.rotation "field bevy::prelude::Transform::rotation") after each call to this method.

##### [Examples found in repository](#scraped-examples-20)[?](../../scrape-examples-help.html)

examples/ecs/fallible\_params.rs ([line 159](../../src/fallible_params/fallible_params.rs.html#159))

```rust
136fn track_targets(
137    // `Single` ensures the system runs ONLY when exactly one matching entity exists.
138    mut player: Single<(&mut Transform, &Player)>,
139    // `Option<Single>` never prevents the system from running, but will be `None` if there is not exactly one matching entity.
140    enemy: Option<Single<&Transform, (With<Enemy>, Without<Player>)>>,
141    time: Res<Time>,
142) {
143    let (player_transform, player) = &mut *player;
144    if let Some(enemy_transform) = enemy {
145        // Enemy found, rotate and move towards it.
146        let delta = enemy_transform.translation - player_transform.translation;
147        let distance = delta.length();
148        let front = delta / distance;
149        let up = Vec3::Z;
150        let side = front.cross(up);
151        player_transform.rotation = Quat::from_mat3(&Mat3::from_cols(side, front, up));
152        let max_step = distance - player.min_follow_radius;
153        if 0.0 < max_step {
154            let velocity = (player.speed * time.delta_secs()).min(max_step);
155            player_transform.translation += front * velocity;
156        }
157    } else {
158        // 0 or multiple enemies found, keep searching.
159        player_transform.rotate_axis(Dir3::Z, player.rotation_speed * time.delta_secs());
160    }
161}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#373)

#### pub fn [rotate\_x](#method.rotate_x)(&mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around the `X` axis by `angle` (in radians).

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the axis is relative to the rotation of the parent.

##### [Examples found in repository](#scraped-examples-21)[?](../../scrape-examples-help.html)

examples/3d/parenting.rs ([line 21](../../src/parenting/parenting.rs.html#21))

```rust
19fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
20    for mut transform in &mut query {
21        transform.rotate_x(3.0 * time.delta_secs());
22    }
23}
```

Hide additional examples

tests/window/desktop\_request\_redraw.rs ([line 99](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#99))

```rust
97fn update(time: Res<Time>, mut query: Query<&mut Transform, With<AnimationActive>>) {
98    if let Ok(mut transform) = query.single_mut() {
99        transform.rotate_x(time.delta_secs().min(1.0 / 60.0));
100    }
101}
```

examples/stress\_tests/many\_lights.rs ([line 138](../../src/many_lights/many_lights.rs.html#138))

```rust
135fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
136    let delta = time.delta_secs() * 0.15;
137    camera_transform.rotate_z(delta);
138    camera_transform.rotate_x(delta);
139}
```

examples/shader\_advanced/custom\_post\_processing.rs ([line 283](../../src/custom_post_processing/custom_post_processing.rs.html#283))

```rust
281fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
282    for mut transform in &mut query {
283        transform.rotate_x(0.55 * time.delta_secs());
284        transform.rotate_z(0.15 * time.delta_secs());
285    }
286}
```

examples/picking/mesh\_picking.rs ([line 196](../../src/mesh_picking/mesh_picking.rs.html#196))

```rust
193fn rotate_on_drag(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
194    let mut transform = transforms.get_mut(drag.entity).unwrap();
195    transform.rotate_y(drag.delta.x * 0.02);
196    transform.rotate_x(drag.delta.y * 0.02);
197}
```

examples/3d/render\_to\_texture.rs ([line 112](../../src/render_to_texture/render_to_texture.rs.html#112))

```rust
110fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
111    for mut transform in &mut query {
112        transform.rotate_x(1.5 * time.delta_secs());
113        transform.rotate_z(1.3 * time.delta_secs());
114    }
115}
116
117/// Rotates the outer cube (main pass)
118fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {
119    for mut transform in &mut query {
120        transform.rotate_x(1.0 * time.delta_secs());
121        transform.rotate_y(0.7 * time.delta_secs());
122    }
123}
```

Additional examples can be found in:  

*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#107)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#83)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#164)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#294)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#139)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#105)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#314)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#559)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#85)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#73)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#381)

#### pub fn [rotate\_y](#method.rotate_y)(&mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around the `Y` axis by `angle` (in radians).

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the axis is relative to the rotation of the parent.

##### [Examples found in repository](#scraped-examples-22)[?](../../scrape-examples-help.html)

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
191
192/// An observer to rotate an entity when it is dragged
193fn rotate_on_drag(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
194    let mut transform = transforms.get_mut(drag.entity).unwrap();
195    transform.rotate_y(drag.delta.x * 0.02);
196    transform.rotate_x(drag.delta.y * 0.02);
197}
```

examples/3d/occlusion\_culling.rs ([line 335](../../src/occlusion_culling/occlusion_culling.rs.html#335))

```rust
333fn spin_small_cubes(mut sphere_parents: Query<&mut Transform, With<SphereParent>>) {
334    for mut sphere_parent_transform in &mut sphere_parents {
335        sphere_parent_transform.rotate_y(ROTATION_SPEED);
336    }
337}
```

examples/3d/skybox.rs ([line 183](../../src/skybox/skybox.rs.html#183))

```rust
178fn animate_light_direction(
179    time: Res<Time>,
180    mut query: Query<&mut Transform, With<DirectionalLight>>,
181) {
182    for mut transform in &mut query {
183        transform.rotate_y(time.delta_secs() * 0.5);
184    }
185}
```

examples/math/custom\_primitives.rs ([line 317](../../src/custom_primitives/custom_primitives.rs.html#317))

```rust
313fn rotate_3d_shapes(mut shapes: Query<&mut Transform, With<Shape3d>>, time: Res<Time>) {
314    let delta_seconds = time.delta_secs();
315
316    for mut transform in shapes.iter_mut() {
317        transform.rotate_y(delta_seconds);
318    }
319}
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#603)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#121)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#106)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#82)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#165)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#208)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#295)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#104)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#370)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#285)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#53)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#185)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#177)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#90)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#281)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#196)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#86)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#389)

#### pub fn [rotate\_z](#method.rotate_z)(&mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around the `Z` axis by `angle` (in radians).

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the axis is relative to the rotation of the parent.

##### [Examples found in repository](#scraped-examples-23)[?](../../scrape-examples-help.html)

examples/2d/2d\_shapes.rs ([line 168](../../src/2d_shapes/2d_shapes.rs.html#168))

```rust
166fn rotate(mut query: Query<&mut Transform, With<Mesh2d>>, time: Res<Time>) {
167    for mut transform in &mut query {
168        transform.rotate_z(time.delta_secs() / 2.0);
169    }
170}
```

Hide additional examples

examples/2d/pixel\_grid\_snap.rs ([line 140](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#140))

```rust
137fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<Rotate>>) {
138    for mut transform in &mut transforms {
139        let dt = time.delta_secs();
140        transform.rotate_z(dt);
141    }
142}
```

examples/stress\_tests/many\_lights.rs ([line 137](../../src/many_lights/many_lights.rs.html#137))

```rust
135fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
136    let delta = time.delta_secs() * 0.15;
137    camera_transform.rotate_z(delta);
138    camera_transform.rotate_x(delta);
139}
```

examples/shader\_advanced/custom\_post\_processing.rs ([line 284](../../src/custom_post_processing/custom_post_processing.rs.html#284))

```rust
281fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
282    for mut transform in &mut query {
283        transform.rotate_x(0.55 * time.delta_secs());
284        transform.rotate_z(0.15 * time.delta_secs());
285    }
286}
```

examples/3d/render\_to\_texture.rs ([line 113](../../src/render_to_texture/render_to_texture.rs.html#113))

```rust
110fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
111    for mut transform in &mut query {
112        transform.rotate_x(1.5 * time.delta_secs());
113        transform.rotate_z(1.3 * time.delta_secs());
114    }
115}
```

examples/stress\_tests/many\_sprite\_meshes.rs ([line 106](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#106))

```rust
105fn move_camera(time: Res<Time>, mut camera_transform: Single<&mut Transform, With<Camera>>) {
106    camera_transform.rotate_z(time.delta_secs() * 0.5);
107    **camera_transform = **camera_transform
108        * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_secs());
109}
```

Additional examples can be found in:  

*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#104)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#311)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#296)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#558)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#163)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#95)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#70)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#135)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#122)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#61)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#397)

#### pub fn [rotate\_local](#method.rotate_local)(&mut self, rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat"))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") by the given `rotation`.

The `rotation` is relative to this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform")’s current rotation.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#410)

#### pub fn [rotate\_local\_axis](#method.rotate_local_axis)(&mut self, axis: [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3"), angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around its local `axis` by `angle` (in radians).

##### Warning

If you pass in an `axis` based on the current rotation (e.g. obtained via [`Transform::local_x`](struct.Transform.html#method.local_x "method bevy::prelude::Transform::local_x")), floating point errors can accumulate exponentially when applying rotations repeatedly this way. This will result in a denormalized rotation. In this case, it is recommended to normalize the [`Transform::rotation`](struct.Transform.html#structfield.rotation "field bevy::prelude::Transform::rotation") after each call to this method.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#422)

#### pub fn [rotate\_local\_x](#method.rotate_local_x)(&mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around its local `X` axis by `angle` (in radians).

##### [Examples found in repository](#scraped-examples-24)[?](../../scrape-examples-help.html)

examples/3d/parallax\_mapping.rs ([line 158](../../src/parallax_mapping/parallax_mapping.rs.html#158))

```rust
155fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>) {
156    for (mut transform, spin) in query.iter_mut() {
157        transform.rotate_local_y(spin.speed * time.delta_secs());
158        transform.rotate_local_x(spin.speed * time.delta_secs());
159        transform.rotate_local_z(-spin.speed * time.delta_secs());
160    }
161}
```

Hide additional examples

examples/3d/deferred\_rendering.rs ([line 266](../../src/deferred_rendering/deferred_rendering.rs.html#266))

```rust
260fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>, pause: Res<Pause>) {
261    if pause.0 {
262        return;
263    }
264    for (mut transform, spin) in query.iter_mut() {
265        transform.rotate_local_y(spin.speed * time.delta_secs());
266        transform.rotate_local_x(spin.speed * time.delta_secs());
267        transform.rotate_local_z(-spin.speed * time.delta_secs());
268    }
269}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#428)

#### pub fn [rotate\_local\_y](#method.rotate_local_y)(&mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around its local `Y` axis by `angle` (in radians).

##### [Examples found in repository](#scraped-examples-25)[?](../../scrape-examples-help.html)

examples/window/low\_power.rs ([line 140](../../src/low_power/low_power.rs.html#140))

```rust
134    pub(crate) fn rotate_cube(
135        time: Res<Time>,
136        mut cube_transform: Query<&mut Transform, With<Rotator>>,
137    ) {
138        for mut transform in &mut cube_transform {
139            transform.rotate_x(time.delta_secs());
140            transform.rotate_local_y(time.delta_secs());
141        }
142    }
```

Hide additional examples

examples/3d/parallax\_mapping.rs ([line 157](../../src/parallax_mapping/parallax_mapping.rs.html#157))

```rust
155fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>) {
156    for (mut transform, spin) in query.iter_mut() {
157        transform.rotate_local_y(spin.speed * time.delta_secs());
158        transform.rotate_local_x(spin.speed * time.delta_secs());
159        transform.rotate_local_z(-spin.speed * time.delta_secs());
160    }
161}
```

examples/3d/deferred\_rendering.rs ([line 265](../../src/deferred_rendering/deferred_rendering.rs.html#265))

```rust
260fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>, pause: Res<Pause>) {
261    if pause.0 {
262        return;
263    }
264    for (mut transform, spin) in query.iter_mut() {
265        transform.rotate_local_y(spin.speed * time.delta_secs());
266        transform.rotate_local_x(spin.speed * time.delta_secs());
267        transform.rotate_local_z(-spin.speed * time.delta_secs());
268    }
269}
```

examples/3d/motion\_blur.rs ([line 325](../../src/motion_blur/motion_blur.rs.html#325))

```rust
300fn move_cars(
301    time: Res<Time>,
302    mut movables: Query<(&mut Transform, &Moves, &Children)>,
303    mut spins: Query<&mut Transform, (Without<Moves>, With<Rotates>)>,
304) {
305    for (mut transform, moves, children) in &mut movables {
306        let time = time.elapsed_secs() * 0.25;
307        let t = time + 0.5 * moves.0;
308        let dx = ops::cos(t);
309        let dz = -ops::sin(3.0 * t);
310        let speed_variation = (dx * dx + dz * dz).sqrt() * 0.15;
311        let t = t + speed_variation;
312        let prev = transform.translation;
313        transform.translation.x = race_track_pos(0.0, t).x;
314        transform.translation.z = race_track_pos(0.0, t).y;
315        transform.translation.y = -0.59;
316        let delta = transform.translation - prev;
317        transform.look_to(delta, Vec3::Y);
318        for child in children.iter() {
319            let Ok(mut wheel) = spins.get_mut(child) else {
320                continue;
321            };
322            let radius = wheel.scale.x;
323            let circumference = 2.0 * std::f32::consts::PI * radius;
324            let angle = delta.length() / circumference * std::f32::consts::PI * 2.0;
325            wheel.rotate_local_y(angle);
326        }
327    }
328}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#434)

#### pub fn [rotate\_local\_z](#method.rotate_local_z)(&mut self, angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around its local `Z` axis by `angle` (in radians).

##### [Examples found in repository](#scraped-examples-26)[?](../../scrape-examples-help.html)

examples/3d/parallax\_mapping.rs ([line 159](../../src/parallax_mapping/parallax_mapping.rs.html#159))

```rust
155fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>) {
156    for (mut transform, spin) in query.iter_mut() {
157        transform.rotate_local_y(spin.speed * time.delta_secs());
158        transform.rotate_local_x(spin.speed * time.delta_secs());
159        transform.rotate_local_z(-spin.speed * time.delta_secs());
160    }
161}
```

Hide additional examples

examples/3d/deferred\_rendering.rs ([line 267](../../src/deferred_rendering/deferred_rendering.rs.html#267))

```rust
260fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>, pause: Res<Pause>) {
261    if pause.0 {
262        return;
263    }
264    for (mut transform, spin) in query.iter_mut() {
265        transform.rotate_local_y(spin.speed * time.delta_secs());
266        transform.rotate_local_x(spin.speed * time.delta_secs());
267        transform.rotate_local_z(-spin.speed * time.delta_secs());
268    }
269}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#442)

#### pub fn [translate\_around](#method.translate_around)(&mut self, point: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3"), rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat"))

Translates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around a `point` in space.

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the `point` is relative to the [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") of the parent.

##### [Examples found in repository](#scraped-examples-27)[?](../../scrape-examples-help.html)

examples/3d/ssr.rs ([line 670](../../src/ssr/ssr.rs.html#670))

```rust
634fn move_camera(
635    keyboard_input: Res<ButtonInput<KeyCode>>,
636    mut mouse_wheel_reader: MessageReader<MouseWheel>,
637    mut cameras: Query<&mut Transform, With<Camera>>,
638) {
639    let (mut distance_delta, mut theta_delta) = (0.0, 0.0);
640
641    // Handle keyboard events.
642    if keyboard_input.pressed(KeyCode::KeyW) {
643        distance_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;
644    }
645    if keyboard_input.pressed(KeyCode::KeyS) {
646        distance_delta += CAMERA_KEYBOARD_ZOOM_SPEED;
647    }
648    if keyboard_input.pressed(KeyCode::KeyA) {
649        theta_delta += CAMERA_KEYBOARD_ORBIT_SPEED;
650    }
651    if keyboard_input.pressed(KeyCode::KeyD) {
652        theta_delta -= CAMERA_KEYBOARD_ORBIT_SPEED;
653    }
654
655    // Handle mouse events.
656    for mouse_wheel in mouse_wheel_reader.read() {
657        distance_delta -= mouse_wheel.y * CAMERA_MOUSE_WHEEL_ZOOM_SPEED;
658    }
659
660    // Update transforms.
661    for mut camera_transform in cameras.iter_mut() {
662        let local_z = camera_transform.local_z().as_vec3().normalize_or_zero();
663        if distance_delta != 0.0 {
664            camera_transform.translation = (camera_transform.translation.length() + distance_delta)
665                .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end)
666                * local_z;
667        }
668        if theta_delta != 0.0 {
669            camera_transform
670                .translate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, theta_delta));
671            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
672        }
673    }
674}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#450)

#### pub fn [rotate\_around](#method.rotate_around)(&mut self, point: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3"), rotation: [Quat](struct.Quat.html "struct bevy::prelude::Quat"))

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") around a `point` in space.

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has a parent, the `point` is relative to the [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") of the parent.

##### [Examples found in repository](#scraped-examples-28)[?](../../scrape-examples-help.html)

examples/gizmos/light\_gizmos.rs ([line 151](../../src/light_gizmos/light_gizmos.rs.html#151))

```rust
150fn rotate_camera(mut transform: Single<&mut Transform, With<Camera>>, time: Res<Time>) {
151    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs() / 2.));
152}
```

Hide additional examples

examples/stress\_tests/many\_cameras\_lights.rs ([line 101](../../src/many_cameras_lights/many_cameras_lights.rs.html#101))

```rust
99fn rotate_cameras(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
100    for mut transform in query.iter_mut() {
101        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs()));
102    }
103}
```

examples/shader/shader\_material\_screenspace\_texture.rs ([lines 53-56](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#53-56))

```rust
52fn rotate_camera(mut cam_transform: Single<&mut Transform, With<MainCamera>>, time: Res<Time>) {
53    cam_transform.rotate_around(
54        Vec3::ZERO,
55        Quat::from_axis_angle(Vec3::Y, 45f32.to_radians() * time.delta_secs()),
56    );
57    cam_transform.look_at(Vec3::ZERO, Vec3::Y);
58}
```

examples/3d/contact\_shadows.rs ([line 271](../../src/contact_shadows/contact_shadows.rs.html#271))

```rust
262fn rotate_light(
263    mut lights: Query<&mut Transform, With<LightContainer>>,
264    app_status: Res<AppStatus>,
265) {
266    if app_status.light_rotation != LightRotation::Rotating {
267        return;
268    }
269
270    for mut transform in lights.iter_mut() {
271        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(LIGHT_ROTATION_SPEED));
272    }
273}
```

examples/3d/spotlight.rs ([line 196](../../src/spotlight/spotlight.rs.html#196))

```rust
188fn rotation(
189    mut transform: Single<&mut Transform, With<Camera>>,
190    input: Res<ButtonInput<KeyCode>>,
191    time: Res<Time>,
192) {
193    let delta = time.delta_secs();
194
195    if input.pressed(KeyCode::ArrowLeft) {
196        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(delta));
197    } else if input.pressed(KeyCode::ArrowRight) {
198        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-delta));
199    }
200}
```

examples/transforms/align.rs ([line 211](../../src/align/align.rs.html#211))

```rust
191fn handle_mouse(
192    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
193    mut mouse_button_inputs: MessageReader<MouseButtonInput>,
194    mut camera_transform: Single<&mut Transform, With<Camera>>,
195    mut mouse_pressed: ResMut<MousePressed>,
196) {
197    // Store left-pressed state in the MousePressed resource
198    for mouse_button_input in mouse_button_inputs.read() {
199        if mouse_button_input.button != MouseButton::Left {
200            continue;
201        }
202        *mouse_pressed = MousePressed(mouse_button_input.state.is_pressed());
203    }
204
205    // If the mouse is not pressed, just ignore motion events
206    if !mouse_pressed.0 {
207        return;
208    }
209    if accumulated_mouse_motion.delta != Vec2::ZERO {
210        let displacement = accumulated_mouse_motion.delta.x;
211        camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-displacement / 75.));
212    }
213}
```

Additional examples can be found in:  

*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#241)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#200)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#177)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#318)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#539-542)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#463)

#### pub fn [look\_at](#method.look_at)(&mut self, target: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3"), up: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>)

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") so that [`Transform::forward`](struct.Transform.html#method.forward "method bevy::prelude::Transform::forward") points towards the `target` position, and [`Transform::up`](struct.Transform.html#method.up "method bevy::prelude::Transform::up") points towards `up`.

In some cases it’s not possible to construct a rotation. Another axis will be picked in those cases:

*   if `target` is the same as the transform translation, `Vec3::Z` is used instead
*   if `up` fails converting to `Dir3` (e.g if it is `Vec3::ZERO`), `Dir3::Y` is used instead
*   if the resulting forward direction is parallel with `up`, an orthogonal vector is used as the “right” direction

##### [Examples found in repository](#scraped-examples-29)[?](../../scrape-examples-help.html)

examples/shader/shader\_material\_screenspace\_texture.rs ([line 57](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#57))

```rust
52fn rotate_camera(mut cam_transform: Single<&mut Transform, With<MainCamera>>, time: Res<Time>) {
53    cam_transform.rotate_around(
54        Vec3::ZERO,
55        Quat::from_axis_angle(Vec3::Y, 45f32.to_radians() * time.delta_secs()),
56    );
57    cam_transform.look_at(Vec3::ZERO, Vec3::Y);
58}
```

Hide additional examples

examples/3d/specular\_tint.rs ([line 139](../../src/specular_tint/specular_tint.rs.html#139))

```rust
135fn rotate_camera(mut cameras: Query<&mut Transform, With<Camera3d>>) {
136    for mut camera_transform in cameras.iter_mut() {
137        camera_transform.translation =
138            Quat::from_rotation_y(ROTATION_SPEED) * camera_transform.translation;
139        camera_transform.look_at(Vec3::ZERO, Vec3::Y);
140    }
141}
```

examples/3d/anisotropy.rs ([line 189](../../src/anisotropy/anisotropy.rs.html#189))

```rust
182fn animate_light(
183    mut lights: Query<&mut Transform, Or<(With<DirectionalLight>, With<PointLight>)>>,
184    time: Res<Time>,
185) {
186    let now = time.elapsed_secs();
187    for mut transform in lights.iter_mut() {
188        transform.translation = vec3(ops::cos(now), 1.0, ops::sin(now)) * vec3(3.0, 4.0, 3.0);
189        transform.look_at(Vec3::ZERO, Vec3::Y);
190    }
191}
```

examples/3d/clearcoat.rs ([line 254](../../src/clearcoat/clearcoat.rs.html#254))

```rust
243fn animate_light(
244    mut lights: Query<&mut Transform, Or<(With<PointLight>, With<DirectionalLight>)>>,
245    time: Res<Time>,
246) {
247    let now = time.elapsed_secs();
248    for mut transform in lights.iter_mut() {
249        transform.translation = vec3(
250            ops::sin(now * 1.4),
251            ops::cos(now * 1.0),
252            ops::cos(now * 0.6),
253        ) * vec3(3.0, 4.0, 3.0);
254        transform.look_at(Vec3::ZERO, Vec3::Y);
255    }
256}
```

examples/3d/reflection\_probes.rs ([line 357](../../src/reflection_probes/reflection_probes.rs.html#357))

```rust
343fn rotate_camera(
344    time: Res<Time>,
345    mut camera_query: Query<&mut Transform, With<Camera3d>>,
346    app_status: Res<AppStatus>,
347) {
348    if !app_status.rotating {
349        return;
350    }
351
352    for mut transform in camera_query.iter_mut() {
353        transform.translation = Vec2::from_angle(time.delta_secs() * PI / 5.0)
354            .rotate(transform.translation.xz())
355            .extend(transform.translation.y)
356            .xzy();
357        transform.look_at(Vec3::ZERO, Vec3::Y);
358    }
359}
```

examples/3d/irradiance\_volumes.rs ([line 363](../../src/irradiance_volumes/irradiance_volumes.rs.html#363))

```rust
349fn rotate_camera(
350    mut camera_query: Query<&mut Transform, With<Camera3d>>,
351    time: Res<Time>,
352    app_status: Res<AppStatus>,
353) {
354    if !app_status.rotating {
355        return;
356    }
357
358    for mut transform in camera_query.iter_mut() {
359        transform.translation = Vec2::from_angle(ROTATION_SPEED * time.delta_secs())
360            .rotate(transform.translation.xz())
361            .extend(transform.translation.y)
362            .xzy();
363        transform.look_at(Vec3::ZERO, Vec3::Y);
364    }
365}
```

Additional examples can be found in:  

*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#606)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#338)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#197)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#671)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#410)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#274)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#429)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#511)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#475)

#### pub fn [look\_to](#method.look_to)(&mut self, direction: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, up: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>)

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") so that [`Transform::forward`](struct.Transform.html#method.forward "method bevy::prelude::Transform::forward") points in the given `direction` and [`Transform::up`](struct.Transform.html#method.up "method bevy::prelude::Transform::up") points towards `up`.

In some cases it’s not possible to construct a rotation. Another axis will be picked in those cases:

*   if `direction` fails converting to `Dir3` (e.g if it is `Vec3::ZERO`), `Dir3::NEG_Z` is used instead
*   if `up` fails converting to `Dir3`, `Dir3::Y` is used instead
*   if `direction` is parallel with `up`, an orthogonal vector is used as the “right” direction

##### [Examples found in repository](#scraped-examples-30)[?](../../scrape-examples-help.html)

examples/3d/generate\_custom\_mesh.rs ([line 100](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#100))

```rust
71fn input_handler(
72    keyboard_input: Res<ButtonInput<KeyCode>>,
73    mesh_query: Query<&Mesh3d, With<CustomUV>>,
74    mut meshes: ResMut<Assets<Mesh>>,
75    mut query: Query<&mut Transform, With<CustomUV>>,
76    time: Res<Time>,
77) {
78    if keyboard_input.just_pressed(KeyCode::Space) {
79        let mesh_handle = mesh_query.single().expect("Query not successful");
80        let mesh = meshes.get_mut(mesh_handle).unwrap();
81        toggle_texture(mesh.into_inner());
82    }
83    if keyboard_input.pressed(KeyCode::KeyX) {
84        for mut transform in &mut query {
85            transform.rotate_x(time.delta_secs() / 1.2);
86        }
87    }
88    if keyboard_input.pressed(KeyCode::KeyY) {
89        for mut transform in &mut query {
90            transform.rotate_y(time.delta_secs() / 1.2);
91        }
92    }
93    if keyboard_input.pressed(KeyCode::KeyZ) {
94        for mut transform in &mut query {
95            transform.rotate_z(time.delta_secs() / 1.2);
96        }
97    }
98    if keyboard_input.pressed(KeyCode::KeyR) {
99        for mut transform in &mut query {
100            transform.look_to(Vec3::NEG_Z, Vec3::Y);
101        }
102    }
103}
```

Hide additional examples

examples/3d/motion\_blur.rs ([line 317](../../src/motion_blur/motion_blur.rs.html#317))

```rust
300fn move_cars(
301    time: Res<Time>,
302    mut movables: Query<(&mut Transform, &Moves, &Children)>,
303    mut spins: Query<&mut Transform, (Without<Moves>, With<Rotates>)>,
304) {
305    for (mut transform, moves, children) in &mut movables {
306        let time = time.elapsed_secs() * 0.25;
307        let t = time + 0.5 * moves.0;
308        let dx = ops::cos(t);
309        let dz = -ops::sin(3.0 * t);
310        let speed_variation = (dx * dx + dz * dz).sqrt() * 0.15;
311        let t = t + speed_variation;
312        let prev = transform.translation;
313        transform.translation.x = race_track_pos(0.0, t).x;
314        transform.translation.z = race_track_pos(0.0, t).y;
315        transform.translation.y = -0.59;
316        let delta = transform.translation - prev;
317        transform.look_to(delta, Vec3::Y);
318        for child in children.iter() {
319            let Ok(mut wheel) = spins.get_mut(child) else {
320                continue;
321            };
322            let radius = wheel.scale.x;
323            let circumference = 2.0 * std::f32::consts::PI * radius;
324            let angle = delta.length() / circumference * std::f32::consts::PI * 2.0;
325            wheel.rotate_local_y(angle);
326        }
327    }
328}
329
330fn move_camera(
331    camera: Single<(&mut Transform, &mut Projection), Without<CameraTracked>>,
332    tracked: Single<&Transform, With<CameraTracked>>,
333    mode: Res<CameraMode>,
334) {
335    let (mut transform, mut projection) = camera.into_inner();
336    match *mode {
337        CameraMode::Track => {
338            transform.look_at(tracked.translation, Vec3::Y);
339            transform.translation = Vec3::new(15.0, -0.5, 0.0);
340            if let Projection::Perspective(perspective) = &mut *projection {
341                perspective.fov = 0.05;
342            }
343        }
344        CameraMode::Chase => {
345            transform.translation =
346                tracked.translation + Vec3::new(0.0, 0.15, 0.0) + tracked.back() * 0.6;
347            transform.look_to(tracked.forward(), Vec3::Y);
348            if let Projection::Perspective(perspective) = &mut *projection {
349                perspective.fov = 1.0;
350            }
351        }
352    }
353}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#529-535)

#### pub fn [align](#method.align)( &mut self, main\_axis: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, main\_direction: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, secondary\_axis: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, secondary\_direction: impl [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\>, )

Rotates this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") so that the `main_axis` vector, reinterpreted in local coordinates, points in the given `main_direction`, while `secondary_axis` points towards `secondary_direction`.

For example, if a spaceship model has its nose pointing in the X-direction in its own local coordinates and its dorsal fin pointing in the Y-direction, then `align(Dir3::X, v, Dir3::Y, w)` will make the spaceship’s nose point in the direction of `v`, while the dorsal fin does its best to point in the direction `w`.

More precisely, the [`Transform::rotation`](struct.Transform.html#structfield.rotation "field bevy::prelude::Transform::rotation") produced will be such that:

*   applying it to `main_axis` results in `main_direction`
*   applying it to `secondary_axis` produces a vector that lies in the half-plane generated by `main_direction` and `secondary_direction` (with positive contribution by `secondary_direction`)

[`Transform::look_to`](struct.Transform.html#method.look_to "method bevy::prelude::Transform::look_to") is recovered, for instance, when `main_axis` is `Dir3::NEG_Z` (the [`Transform::forward`](struct.Transform.html#method.forward "method bevy::prelude::Transform::forward") direction in the default orientation) and `secondary_axis` is `Dir3::Y` (the [`Transform::up`](struct.Transform.html#method.up "method bevy::prelude::Transform::up") direction in the default orientation). (Failure cases may differ somewhat.)

In some cases a rotation cannot be constructed. Another axis will be picked in those cases:

*   if `main_axis` or `main_direction` fail converting to `Dir3` (e.g are zero), `Dir3::X` takes their place
*   if `secondary_axis` or `secondary_direction` fail converting, `Dir3::Y` takes their place
*   if `main_axis` is parallel with `secondary_axis` or `main_direction` is parallel with `secondary_direction`, a rotation is constructed which takes `main_axis` to `main_direction` along a great circle, ignoring the secondary counterparts

Example

```rust
t1.align(Dir3::X, Dir3::Y, Vec3::new(1., 1., 0.), Dir3::Z);
let main_axis_image = t1.rotation * Dir3::X;
let secondary_axis_image = t1.rotation * Vec3::new(1., 1., 0.);
assert!(main_axis_image.abs_diff_eq(Vec3::Y, 1e-5));
assert!(secondary_axis_image.abs_diff_eq(Vec3::new(0., 1., 1.), 1e-5));

t1.align(Vec3::ZERO, Dir3::Z, Vec3::ZERO, Dir3::X);
t2.align(Dir3::X, Dir3::Z, Dir3::Y, Dir3::X);
assert_eq!(t1.rotation, t2.rotation);

t1.align(Dir3::X, Dir3::Z, Dir3::X, Dir3::Y);
assert_eq!(t1.rotation, Quat::from_rotation_arc(Vec3::X, Vec3::Z));
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#571)

#### pub fn [mul\_transform](#method.mul_transform)(&self, transform: [Transform](struct.Transform.html "struct bevy::prelude::Transform")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Multiplies `self` with `transform` component by component, returning the resulting [`Transform`](struct.Transform.html "struct bevy::prelude::Transform")

##### [Examples found in repository](#scraped-examples-31)[?](../../scrape-examples-help.html)

examples/3d/order\_independent\_transparency.rs ([line 192](../../src/order_independent_transparency/order_independent_transparency.rs.html#192))

```rust
183fn spawn_quads(
184    commands: &mut Commands,
185    meshes: &mut Assets<Mesh>,
186    materials: &mut Assets<StandardMaterial>,
187) {
188    let quad_handle = meshes.add(Rectangle::new(3.0, 3.0).mesh());
189    let render_layers = RenderLayers::layer(1);
190    let xform = |x, y, z| {
191        Transform::from_rotation(Quat::from_rotation_y(0.5))
192            .mul_transform(Transform::from_xyz(x, y, z))
193    };
194    commands.spawn((
195        Mesh3d(quad_handle.clone()),
196        MeshMaterial3d(materials.add(StandardMaterial {
197            base_color: RED.with_alpha(0.5).into(),
198            alpha_mode: AlphaMode::Blend,
199            ..default()
200        })),
201        xform(1.0, -0.1, 0.),
202        render_layers.clone(),
203    ));
204    commands.spawn((
205        Mesh3d(quad_handle.clone()),
206        MeshMaterial3d(materials.add(StandardMaterial {
207            base_color: BLUE.with_alpha(0.8).into(),
208            alpha_mode: AlphaMode::Blend,
209            ..default()
210        })),
211        xform(0.5, 0.2, -0.5),
212        render_layers.clone(),
213    ));
214    commands.spawn((
215        Mesh3d(quad_handle.clone()),
216        MeshMaterial3d(materials.add(StandardMaterial {
217            base_color: GREEN.with_green(1.0).with_alpha(0.5).into(),
218            alpha_mode: AlphaMode::Blend,
219            ..default()
220        })),
221        xform(0.0, 0.4, -1.),
222        render_layers.clone(),
223    ));
224    commands.spawn((
225        Mesh3d(quad_handle.clone()),
226        MeshMaterial3d(materials.add(StandardMaterial {
227            base_color: YELLOW.with_alpha(0.3).into(),
228            alpha_mode: AlphaMode::Blend,
229            ..default()
230        })),
231        xform(-0.5, 0.6, -1.1),
232        render_layers.clone(),
233    ));
234    commands.spawn((
235        Mesh3d(quad_handle.clone()),
236        MeshMaterial3d(materials.add(StandardMaterial {
237            base_color: BLUE.with_alpha(0.2).into(),
238            alpha_mode: AlphaMode::Blend,
239            ..default()
240        })),
241        xform(-0.8, 0.8, -1.2),
242        render_layers.clone(),
243    ));
244}
```

Hide additional examples

examples/stress\_tests/many\_cubes.rs ([line 202](../../src/many_cubes/many_cubes.rs.html#202))

```rust
161fn setup(
162    mut commands: Commands,
163    args: Res<Args>,
164    mesh_assets: ResMut<Assets<Mesh>>,
165    material_assets: ResMut<Assets<StandardMaterial>>,
166    images: ResMut<Assets<Image>>,
167) {
168    warn!(include_str!("warning_string.txt"));
169
170    let args = args.into_inner();
171    let images = images.into_inner();
172    let material_assets = material_assets.into_inner();
173    let mesh_assets = mesh_assets.into_inner();
174
175    let meshes = init_meshes(args, mesh_assets);
176
177    let material_textures = init_textures(args, images);
178    let materials = init_materials(args, &material_textures, material_assets);
179
180    // We're seeding the PRNG here to make this example deterministic for testing purposes.
181    // This isn't strictly required in practical use unless you need your app to be deterministic.
182    let mut material_rng = ChaCha8Rng::seed_from_u64(42);
183    match args.layout {
184        Layout::Sphere => {
185            // NOTE: This pattern is good for testing performance of culling as it provides roughly
186            // the same number of visible meshes regardless of the viewing angle.
187            let n_points: usize = args.instance_count;
188            // NOTE: f64 is used to avoid precision issues that produce visual artifacts in the distribution
189            let radius = WIDTH as f64 * 2.5;
190            let golden_ratio = 0.5f64 * (1.0f64 + 5.0f64.sqrt());
191            for i in 0..n_points {
192                let spherical_polar_theta_phi =
193                    fibonacci_spiral_on_sphere(golden_ratio, i, n_points);
194                let unit_sphere_p = spherical_polar_to_cartesian(spherical_polar_theta_phi);
195                let (mesh, transform) = meshes.choose(&mut material_rng).unwrap();
196                commands
197                    .spawn((
198                        Mesh3d(mesh.clone()),
199                        MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
200                        Transform::from_translation((radius * unit_sphere_p).as_vec3())
201                            .looking_at(Vec3::ZERO, Vec3::Y)
202                            .mul_transform(*transform),
203                    ))
204                    .insert_if(NoFrustumCulling, || args.no_frustum_culling)
205                    .insert_if(NoAutomaticBatching, || args.no_automatic_batching)
206                    .insert_if(NoCpuCulling, || args.no_cpu_culling);
207            }
208
209            // camera
210            let mut camera = commands.spawn(Camera3d::default());
211            if args.no_indirect_drawing {
212                camera.insert(NoIndirectDrawing);
213            }
214            if args.no_cpu_culling {
215                camera.insert(NoCpuCulling);
216            }
217            if args.motion_blur {
218                camera.insert((
219                    MotionBlur {
220                        // Use an unrealistically large shutter angle so that motion blur is clearly visible.
221                        shutter_angle: 3.0,
222                        ..Default::default()
223                    },
224                    // MSAA and MotionBlur are not compatible on WebGL.
225                    #[cfg(all(
226                        feature = "webgl2",
227                        target_arch = "wasm32",
228                        not(feature = "webgpu")
229                    ))]
230                    Msaa::Off,
231                ));
232            }
233
234            // Inside-out box around the meshes onto which shadows are cast (though you cannot see them...)
235            commands.spawn((
236                Mesh3d(mesh_assets.add(Cuboid::from_size(Vec3::splat(radius as f32 * 2.2)))),
237                MeshMaterial3d(material_assets.add(StandardMaterial::from(Color::WHITE))),
238                Transform::from_scale(-Vec3::ONE),
239                NotShadowCaster,
240            ));
241        }
242        Layout::Cube => {
243            // NOTE: This pattern is good for demonstrating that frustum culling is working correctly
244            // as the number of visible meshes rises and falls depending on the viewing angle.
245            let scale = 2.5;
246
247            // Scale the width and height by the same factor so that we have the
248            // right number of instances.
249            // Because of the moiré pattern check and the fact that we're
250            // spawning 4 instances per trip around the inner loop below, we're
251            // solving the following equation for the factor variable:
252            //
253            //      4 * (9/10 * factor * width * 9/10 * factor * height) = count
254            //
255            // The solution is the value below.
256            let factor = (5.0 / 9.0) * sqrt(args.instance_count as f32)
257                / (sqrt(HEIGHT as f32) * sqrt(WIDTH as f32));
258            let dimensions = (vec2(WIDTH as f32, HEIGHT as f32) * factor)
259                .ceil()
260                .as_uvec2();
261
262            for x in 0..dimensions.x {
263                for y in 0..dimensions.y {
264                    // introduce spaces to break any kind of moiré pattern
265                    if x % 10 == 0 || y % 10 == 0 {
266                        continue;
267                    }
268                    // cube
269                    commands
270                        .spawn((
271                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
272                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
273                            Transform::from_xyz((x as f32) * scale, (y as f32) * scale, 0.0),
274                        ))
275                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
276                    commands
277                        .spawn((
278                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
279                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
280                            Transform::from_xyz(
281                                (x as f32) * scale,
282                                dimensions.y as f32 * scale,
283                                (y as f32) * scale,
284                            ),
285                        ))
286                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
287                    commands
288                        .spawn((
289                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
290                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
291                            Transform::from_xyz((x as f32) * scale, 0.0, (y as f32) * scale),
292                        ))
293                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
294                    commands
295                        .spawn((
296                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
297                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
298                            Transform::from_xyz(0.0, (x as f32) * scale, (y as f32) * scale),
299                        ))
300                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
301                }
302            }
303            // camera
304            let center = 0.5
305                * scale
306                * Vec3::new(
307                    dimensions.x as f32,
308                    dimensions.y as f32,
309                    dimensions.x as f32,
310                );
311            commands.spawn((Camera3d::default(), Transform::from_translation(center)));
312            // Inside-out box around the meshes onto which shadows are cast (though you cannot see them...)
313            commands.spawn((
314                Mesh3d(mesh_assets.add(Cuboid::from_size(2.0 * 1.1 * center))),
315                MeshMaterial3d(material_assets.add(StandardMaterial::from(Color::WHITE))),
316                Transform::from_scale(-Vec3::ONE).with_translation(center),
317                NotShadowCaster,
318            ));
319        }
320        Layout::Dense => {
321            // NOTE: This pattern is good for demonstrating a dense configuration of cubes
322            // overlapping each other, all within the camera frustum.
323            let count = args.instance_count;
324            let size = cbrt(count as f32).round();
325            let gap = 1.25;
326
327            for i in 0..count {
328                let x = i as f32 % size;
329                let y = (i as f32 / size) % size;
330                let z = i as f32 / (size * size);
331                let pos = Vec3::new(x * gap, y * gap, z * gap);
332                commands
333                    .spawn((
334                        Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
335                        MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
336                        Transform::from_translation(pos),
337                    ))
338                    .insert_if(NoCpuCulling, || args.no_cpu_culling);
339            }
340
341            // camera
342            commands.spawn((
343                Camera3d::default(),
344                Transform::from_xyz(100.0, 90.0, 100.0)
345                    .looking_at(Vec3::new(0.0, -10.0, 0.0), Vec3::Y),
346            ));
347        }
348    }
349
350    commands.spawn((
351        DirectionalLight {
352            shadow_maps_enabled: args.shadows,
353            ..default()
354        },
355        Transform::IDENTITY.looking_at(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
356    ));
357}
```

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#594)

#### pub fn [transform\_point](#method.transform_point)(&self, point: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

Transforms the given `point`, applying scale, rotation and translation.

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") has an ancestor entity with a [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") component, [`Transform::transform_point`](struct.Transform.html#method.transform_point "method bevy::prelude::Transform::transform_point") will transform a point in local space into its parent transform’s space.

If this [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") does not have a parent, [`Transform::transform_point`](struct.Transform.html#method.transform_point "method bevy::prelude::Transform::transform_point") will transform a point in local space into worldspace coordinates.

If you always want to transform a point in local space to worldspace, or if you need the inverse transformations, see [`GlobalTransform::transform_point()`](struct.GlobalTransform.html#method.transform_point "method bevy::prelude::GlobalTransform::transform_point").

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#606)

#### pub fn [is\_finite](#method.is_finite)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, translation, rotation and scale all are finite. If any of them contains a `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#614)

#### pub fn [to\_isometry](#method.to_isometry)(&self) -> [Isometry3d](struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

Get the [isometry](struct.Isometry3d.html "struct bevy::prelude::Isometry3d") defined by this transform’s rotation and translation, ignoring scale.

##### [Examples found in repository](#scraped-examples-32)[?](../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 332](../../src/custom_primitives/custom_primitives.rs.html#332))

```rust
322fn bounding_shapes_3d(
323    shapes: Query<&Transform, With<Shape3d>>,
324    mut gizmos: Gizmos,
325    bounding_shape: Res<State<BoundingShape>>,
326) {
327    for transform in shapes.iter() {
328        match bounding_shape.get() {
329            BoundingShape::None => (),
330            BoundingShape::BoundingBox => {
331                // Get the AABB of the extrusion with the rotation and translation of the mesh.
332                let aabb = EXTRUSION.aabb_3d(transform.to_isometry());
333
334                gizmos.primitive_3d(
335                    &Cuboid::from_size(Vec3::from(aabb.half_size()) * 2.),
336                    aabb.center(),
337                    WHITE,
338                );
339            }
340            BoundingShape::BoundingSphere => {
341                // Get the bounding sphere of the extrusion with the rotation and translation of the mesh.
342                let bounding_sphere = EXTRUSION.bounding_sphere(transform.to_isometry());
343
344                gizmos.sphere(bounding_sphere.center(), bounding_sphere.radius(), WHITE);
345            }
346        }
347    }
348}
```

## Trait Implementations

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#133)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#134)

#### fn [interpolate](trait.Animatable.html#tymethod.interpolate)(a: &[Transform](struct.Transform.html "struct bevy::prelude::Transform"), b: &[Transform](struct.Transform.html "struct bevy::prelude::Transform"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Interpolates between `a` and `b` with an interpolation factor of `time`. [Read more](trait.Animatable.html#tymethod.interpolate)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#142)

#### fn [blend](trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](struct.BlendInput.html "struct bevy::prelude::BlendInput")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>>) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Blends one or more values together. [Read more](trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#74)

### impl [Component](trait.Component.html "trait bevy::prelude::Component") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

where [Transform](struct.Transform.html "struct bevy::prelude::Transform"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

**Required Components**: [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform"), [`TransformTreeChanged`](struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged").

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#74)

#### const [STORAGE\_TYPE](trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::Table

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#74)

#### type [Mutability](trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](trait.Component.html "trait bevy::prelude::Component"). [Read more](trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#74)

#### fn [register\_required\_components](trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#74)

#### fn [clone\_behavior](trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#74)

#### fn [relationship\_accessor](trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>>

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

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#619)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#620)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#70)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

where [Transform](struct.Transform.html "struct bevy::prelude::Transform"): [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#70)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Transform](struct.Transform.html "struct bevy::prelude::Transform"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#627)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")\> for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

The transform is expected to be non-degenerate and without shearing, or the output will be invalid.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#628)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(transform: [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")) -> [Transform](struct.Transform.html "struct bevy::prelude::Transform")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#308)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\> for [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#309)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(transform: [Transform](struct.Transform.html "struct bevy::prelude::Transform")) -> [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Transform](struct.Transform.html "struct bevy::prelude::Transform")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Transform](struct.Transform.html "struct bevy::prelude::Transform") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Transform](struct.Transform.html "struct bevy::prelude::Transform"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#633)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#634)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Transform](struct.Transform.html "struct bevy::prelude::Transform")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#636)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, transform: [Transform](struct.Transform.html "struct bevy::prelude::Transform")) -> <[Transform](struct.Transform.html "struct bevy::prelude::Transform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#641)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")\> for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#642)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#645)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)( self, global\_transform: [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform"), ) -> <[Transform](struct.Transform.html "struct bevy::prelude::Transform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2554)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")\> for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2555)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2557)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")) -> <[Transform](struct.Transform.html "struct bevy::prelude::Transform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#329)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\> for [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#330)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#333)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)( self, transform: [Transform](struct.Transform.html "struct bevy::prelude::Transform"), ) -> <[GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#650)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#651)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#653)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, value: [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[Transform](struct.Transform.html "struct bevy::prelude::Transform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Transform](struct.Transform.html "struct bevy::prelude::Transform")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#80)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#80)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#80)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[Transform](struct.Transform.html "struct bevy::prelude::Transform")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#70)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#70)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [field](trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [field\_mut](trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [field\_at](trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [field\_at\_mut](trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [name\_at](trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [index\_of\_name](trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [field\_len](trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [iter\_fields](trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [to\_dynamic\_struct](trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#69)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#11)

### impl [TransformPoint](trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#13)

#### fn [transform\_point](trait.TransformPoint.html#tymethod.transform_point)(&self, point: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

Transform a point.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

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

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

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

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

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

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#686)

### impl<T> [ErasedComponentTemplate](../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#687-691)

#### unsafe fn [apply](../scene/trait.ErasedComponentTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, bundle\_writer: &mut [BundleWriter](../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../scene/trait.ErasedComponentTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#701)

#### fn [clone\_template](../scene/trait.ErasedComponentTemplate.html#tymethod.clone_template)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedComponentTemplate](../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate")\>

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

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

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