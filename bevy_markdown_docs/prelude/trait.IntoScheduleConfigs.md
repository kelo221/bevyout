[bevy](../index.html)::[prelude](index.html)

# Trait IntoScheduleConfigs 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#314-315)

```rust
pub trait IntoScheduleConfigs<T, Marker>: Sizedwhere
    T: Schedulable<Metadata = GraphInfo, GroupMetadata = Chain>,{
    // Required method
    fn into_configs(self) -> ScheduleConfigs<T>;

    // Provided methods
    fn in_set(self, set: impl SystemSet) -> ScheduleConfigs<T> { ... }
    fn before<M>(self, set: impl IntoSystemSet<M>) -> ScheduleConfigs<T> { ... }
    fn after<M>(self, set: impl IntoSystemSet<M>) -> ScheduleConfigs<T> { ... }
    fn before_ignore_deferred<M>(
        self,
        set: impl IntoSystemSet<M>,
    ) -> ScheduleConfigs<T> { ... }
    fn after_ignore_deferred<M>(
        self,
        set: impl IntoSystemSet<M>,
    ) -> ScheduleConfigs<T> { ... }
    fn distributive_run_if<M>(
        self,
        condition: impl SystemCondition<M> + Clone,
    ) -> ScheduleConfigs<T> { ... }
    fn run_if<M>(self, condition: impl SystemCondition<M>) -> ScheduleConfigs<T> { ... }
    fn ambiguous_with<M>(self, set: impl IntoSystemSet<M>) -> ScheduleConfigs<T> { ... }
    fn ambiguous_with_all(self) -> ScheduleConfigs<T> { ... }
    fn chain(self) -> ScheduleConfigs<T> { ... }
    fn chain_ignore_deferred(self) -> ScheduleConfigs<T> { ... }
}
```

Types that can convert into a [`ScheduleConfigs`](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

This trait is implemented for “systems” (functions whose arguments all implement [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")), or tuples thereof. It is a common entry point for system configurations.

## Usage notes

This trait should only be used as a bound for trait implementations or as an argument to a function. If system configs need to be returned from a function or stored somewhere, use [`ScheduleConfigs`](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs") instead of this trait.

## Examples

```rust
fn handle_input() {}

fn update_camera() {}
fn update_character() {}

app.add_systems(
    Update,
    (
        handle_input,
        (update_camera, update_character).after(handle_input)
    )
);
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#318)

#### fn [into\_configs](#tymethod.into_configs)(self) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Convert into a [`ScheduleConfigs`](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#322)

#### fn [in\_set](#method.in_set)(self, set: impl [SystemSet](trait.SystemSet.html "trait bevy::prelude::SystemSet")) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Add these systems to the provided `set`.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ui/render\_ui\_to\_texture.rs ([line 27](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#27))

```rust
22fn main() {
23    App::new()
24        .add_plugins(DefaultPlugins)
25        .add_systems(Startup, setup)
26        .add_systems(Update, rotator_system)
27        .add_systems(First, drive_diegetic_pointer.in_set(PickingSystems::Input))
28        .run();
29}
```

Hide additional examples

examples/stress\_tests/many\_lights.rs ([line 160](../../src/many_lights/many_lights.rs.html#160))

```rust
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
```

examples/shader/gpu\_readback.rs ([line 53](../../src/gpu_readback/gpu_readback.rs.html#53))

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

examples/shader\_advanced/custom\_shader\_instancing.rs ([line 122](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#122))

```rust
110    fn build(&self, app: &mut App) {
111        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
112        app.sub_app_mut(RenderApp)
113            .add_render_command::<Transparent3d, DrawCustom>()
114            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
115            .add_systems(
116                RenderStartup,
117                init_custom_pipeline.after(MeshPipelineSystems),
118            )
119            .add_systems(
120                Render,
121                (
122                    queue_custom.in_set(RenderSystems::QueueMeshes),
123                    prepare_instance_buffers.in_set(RenderSystems::PrepareResources),
124                ),
125            );
126    }
```

examples/shader\_advanced/custom\_phase\_item.rs ([line 177](../../src/custom_phase_item/custom_phase_item.rs.html#177))

```rust
164fn main() {
165    let mut app = App::new();
166    app.add_plugins(DefaultPlugins)
167        .add_plugins(ExtractComponentPlugin::<CustomRenderedEntity>::default())
168        .add_systems(Startup, setup);
169
170    // We make sure to add these to the render app, not the main app.
171    app.sub_app_mut(RenderApp)
172        .init_resource::<CustomPhasePipeline>()
173        .init_resource::<PendingCustomPhaseItemQueues>()
174        .add_render_command::<Opaque3d, DrawCustomPhaseItemCommands>()
175        .add_systems(
176            Render,
177            prepare_custom_phase_item_buffers.in_set(RenderSystems::Prepare),
178        )
179        .add_systems(Render, queue_custom_phase_item.in_set(RenderSystems::Queue));
180
181    app.run();
182}
```

examples/picking/custom\_hit\_data.rs ([line 41](../../src/custom_hit_data/custom_hit_data.rs.html#41))

```rust
25fn main() {
26    App::new()
27        .add_plugins((DefaultPlugins, MeshPickingPlugin))
28        .insert_resource(MeshPickingSettings {
29            require_markers: true,
30            ..default()
31        })
32        .insert_resource(PickingSettings {
33            is_window_picking_enabled: false,
34            ..default()
35        })
36        .init_resource::<HoveredTriangles>()
37        .add_systems(Startup, (setup_gizmos, setup_scene))
38        .add_systems(
39            PreUpdate,
40            (
41                custom_backend_system.in_set(PickingSystems::Backend),
42                cache_hovered_triangles.after(PickingSystems::Backend),
43            ),
44        )
45        .add_systems(Update, draw_hit_gizmos)
46        .run();
47}
```

Additional examples can be found in:  

*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#107)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#125)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#71)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#78)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#65)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#326)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#142)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#222)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#125)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#351)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#334)

#### fn [before](#method.before)<M>(self, set: impl [IntoSystemSet](trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Runs before all systems in `set`. If `self` has any systems that produce [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") or other [`Deferred`](struct.Deferred.html "struct bevy::prelude::Deferred") operations, all systems in `set` will see their effect.

If automatically inserting [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") like this isn’t desired, use [`before_ignore_deferred`](trait.IntoScheduleConfigs.html#method.before_ignore_deferred "method bevy::prelude::IntoScheduleConfigs::before_ignore_deferred") instead.

Calling [`.chain`](trait.IntoScheduleConfigs.html#method.chain "method bevy::prelude::IntoScheduleConfigs::chain") is often more convenient and ensures that all systems are added to the schedule. Please check the [caveats section of `.after`](trait.IntoScheduleConfigs.html#method.after "method bevy::prelude::IntoScheduleConfigs::after") for details.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/shader\_advanced/compute\_mesh.rs ([line 60](../../src/compute_mesh/compute_mesh.rs.html#60))

```rust
51    fn build(&self, app: &mut App) {
52        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
53            return;
54        };
55
56        render_app
57            .init_resource::<ChunksToProcess>()
58            .add_systems(RenderStartup, init_compute_pipeline)
59            .add_systems(Render, prepare_chunks)
60            .add_systems(RenderGraph, compute_mesh.before(camera_driver));
61    }
```

Hide additional examples

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 121](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#121))

```rust
101fn main() {
102    let mut app = App::new();
103
104    app.add_plugins(DefaultPlugins)
105        .add_plugins(MaterialPlugin::<ShowDepthTextureMaterial>::default())
106        .add_plugins(ExtractResourcePlugin::<DemoDepthTexture>::default())
107        .init_resource::<DemoDepthTexture>()
108        .add_systems(Startup, setup)
109        .add_systems(Update, rotate_cube)
110        .add_systems(Update, draw_camera_gizmo)
111        .add_systems(Update, move_camera);
112
113    let render_app = app
114        .get_sub_app_mut(RenderApp)
115        .expect("Render app should be present");
116
117    render_app.add_systems(
118        Core3d,
119        copy_depth_texture_system
120            .after(Core3dSystems::Prepass)
121            .before(Core3dSystems::MainPass),
122    );
123
124    app.run();
125}
```

examples/camera/2d\_screen\_shake.rs ([line 66](../../src/2d_screen_shake/2d_screen_shake.rs.html#66))

```rust
53fn main() {
54    App::new()
55        .add_plugins(DefaultPlugins)
56        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
57        // At the start of the frame, restore the camera's transform to its unshaken state.
58        .add_systems(PreUpdate, reset_transform)
59        .add_systems(
60            Update,
61            // Increase trauma when the space key is pressed.
62            increase_trauma.run_if(input_just_pressed(KeyCode::Space)),
63        )
64        // Just before the end of the frame, apply the shake.
65        // This is ordered so that the transform propagation produces correct values for the global transform, which is used by Bevy's rendering.
66        .add_systems(PostUpdate, shake_camera.before(TransformSystems::Propagate))
67        .run();
68}
```

examples/3d/reflection\_probes.rs ([line 80](../../src/reflection_probes/reflection_probes.rs.html#80))

```rust
70fn main() {
71    // Create the app.
72    App::new()
73        .add_plugins(DefaultPlugins)
74        .init_resource::<AppStatus>()
75        .init_resource::<Cubemaps>()
76        .add_systems(Startup, setup)
77        .add_systems(PreUpdate, add_environment_map_to_camera)
78        .add_systems(
79            Update,
80            change_reflection_type.before(generate_environment_map_light),
81        )
82        .add_systems(Update, toggle_rotation)
83        .add_systems(Update, change_sphere_roughness)
84        .add_systems(
85            Update,
86            rotate_camera
87                .after(toggle_rotation)
88                .after(change_reflection_type),
89        )
90        .add_systems(Update, update_text.after(rotate_camera))
91        .add_systems(Update, setup_environment_map_usage)
92        .run();
93}
```

examples/shader/compute\_shader\_game\_of\_life.rs ([line 110](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#110))

```rust
94    fn build(&self, app: &mut App) {
95        // Extract the game of life image resource from the main world into the render world
96        // for operation on by the compute shader and display on the sprite.
97        app.add_plugins((
98            ExtractResourcePlugin::<GameOfLifeImages>::default(),
99            ExtractResourcePlugin::<GameOfLifeUniforms>::default(),
100        ));
101        let render_app = app.sub_app_mut(RenderApp);
102        render_app
103            .init_resource::<GameOfLifeState>()
104            .add_systems(RenderStartup, init_game_of_life_pipeline)
105            .add_systems(
106                Render,
107                prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
108            )
109            .add_systems(Render, update.in_set(RenderSystems::Prepare))
110            .add_systems(RenderGraph, game_of_life.before(camera_driver));
111    }
```

examples/2d/dynamic\_mip\_generation.rs ([line 242](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#242))

```rust
208fn main() {
209    let mut app = App::new();
210    app.add_plugins((
211        DefaultPlugins.set(WindowPlugin {
212            primary_window: Some(Window {
213                title: "Bevy Dynamic Mipmap Generation Example".into(),
214                ..default()
215            }),
216            ..default()
217        }),
218        Material2dPlugin::<SingleMipLevelMaterial>::default(),
219    ))
220    .init_resource::<AppStatus>()
221    .init_resource::<AppAssets>()
222    .add_message::<RegenerateImage>()
223    .add_message::<WidgetClickEvent<AppSetting>>()
224    .add_systems(Startup, setup)
225    .add_systems(Update, animate_image_scale)
226    .add_systems(
227        Update,
228        (
229            widgets::handle_ui_interactions::<AppSetting>,
230            update_radio_buttons,
231        )
232            .chain(),
233    )
234    .add_systems(
235        Update,
236        (handle_window_resize_events, regenerate_image_when_requested).chain(),
237    )
238    .add_systems(
239        Update,
240        handle_app_setting_change
241            .after(widgets::handle_ui_interactions::<AppSetting>)
242            .before(regenerate_image_when_requested),
243    );
244
245    // Because `MipGenerationJobs` is part of the render app, we need to add the
246    // associated systems to that app, not the main one.
247
248    let render_app = app.get_sub_app_mut(RenderApp).expect("Need a render app");
249
250    render_app.add_systems(Core2d, generate_mips_for_example);
251
252    // Add the system that adds the image into the `MipGenerationJobs` list.
253    // Note that this must run as part of the extract schedule, because it needs
254    // access to resources from both the main world and the render world.
255    render_app.add_systems(ExtractSchedule, extract_mipmap_source_image);
256
257    app.run();
258}
```

Additional examples can be found in:  

*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#234)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#361)

#### fn [after](#method.after)<M>(self, set: impl [IntoSystemSet](trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run after all systems in `set`. If `set` has any systems that produce [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") or other [`Deferred`](struct.Deferred.html "struct bevy::prelude::Deferred") operations, all systems in `self` will see their effect.

If automatically inserting [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") like this isn’t desired, use [`after_ignore_deferred`](trait.IntoScheduleConfigs.html#method.after_ignore_deferred "method bevy::prelude::IntoScheduleConfigs::after_ignore_deferred") instead.

Calling [`.chain`](trait.IntoScheduleConfigs.html#method.chain "method bevy::prelude::IntoScheduleConfigs::chain") is often more convenient and ensures that all systems are added to the schedule.

##### Caveats

If you configure two [`System`](trait.System.html "trait bevy::prelude::System")s like `(GameSystem::A).after(GameSystem::B)` or `(GameSystem::A).before(GameSystem::B)`, the `GameSystem::B` will not be automatically scheduled.

This means that the system `GameSystem::A` and the system or systems in `GameSystem::B` will run independently of each other if `GameSystem::B` was never explicitly scheduled with [`configure_sets`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.configure_sets) If that is the case, `.after`/`.before` will not provide the desired behavior and the systems can run in parallel or in any order determined by the scheduler. Only use `after(GameSystem::B)` and `before(GameSystem::B)` when you know that `B` has already been scheduled for you, e.g. when it was provided by Bevy or a third-party dependency, or you manually scheduled it somewhere else in your app.

Another caveat is that if `GameSystem::B` is placed in a different schedule than `GameSystem::A`, any ordering calls between them—whether using `.before`, `.after`, or `.chain`—will be silently ignored.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/2d/2d\_viewport\_to\_world.rs ([line 18](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#18))

```rust
13fn main() {
14    App::new()
15        .add_plugins(DefaultPlugins)
16        .add_systems(Startup, setup)
17        .add_systems(FixedUpdate, controls)
18        .add_systems(PostUpdate, draw_cursor.after(TransformSystems::Propagate))
19        .run();
20}
```

Hide additional examples

examples/3d/skybox.rs ([line 47](../../src/skybox/skybox.rs.html#47))

```rust
38fn main() {
39    App::new()
40        .add_plugins(DefaultPlugins)
41        .add_plugins(FreeCameraPlugin)
42        .add_systems(Startup, setup)
43        .add_systems(
44            Update,
45            (
46                cycle_cubemap_asset,
47                asset_loaded.after(cycle_cubemap_asset),
48                animate_light_direction,
49            ),
50        )
51        .run();
52}
```

examples/ecs/one\_shot\_systems.rs ([line 24](../../src/one_shot_systems/one_shot_systems.rs.html#24))

```rust
16fn main() {
17    App::new()
18        .add_plugins(DefaultPlugins)
19        .add_systems(
20            Startup,
21            (
22                setup_ui,
23                setup_with_commands,
24                setup_with_world.after(setup_ui), // since we run `system_b` once in world it needs to run after `setup_ui`
25            ),
26        )
27        .add_systems(Update, (trigger_system, evaluate_callbacks).chain())
28        .run();
29}
```

examples/3d/post\_processing.rs ([line 61](../../src/post_processing/post_processing.rs.html#61))

```rust
51fn main() {
52    App::new()
53        .init_resource::<AppSettings>()
54        .add_plugins(DefaultPlugins)
55        .add_systems(Startup, setup)
56        .add_systems(Update, handle_keyboard_input)
57        .add_systems(
58            Update,
59            (update_chromatic_aberration_settings, update_help_text)
60                .run_if(resource_changed::<AppSettings>)
61                .after(handle_keyboard_input),
62        )
63        .run();
64}
```

examples/ui/ui\_scaling.rs ([line 20](../../src/ui_scaling/ui_scaling.rs.html#20))

```rust
9fn main() {
10    App::new()
11        .add_plugins(DefaultPlugins)
12        .insert_resource(TargetScale {
13            start_scale: 1.0,
14            target_scale: 1.0,
15            target_time: Timer::new(Duration::from_millis(SCALE_TIME), TimerMode::Once),
16        })
17        .add_systems(Startup, setup)
18        .add_systems(
19            Update,
20            (change_scaling, apply_scaling.after(change_scaling)),
21        )
22        .run();
23}
```

examples/async\_tasks/async\_channel\_pattern.rs ([line 32](../../src/async_channel_pattern/async_channel_pattern.rs.html#32))

```rust
22fn main() {
23    App::new()
24        .add_plugins(DefaultPlugins)
25        .add_systems(
26            Startup,
27            (
28                setup_env,
29                setup_assets,
30                setup_channel,
31                // Ensure the channel is set up before spawning tasks.
32                spawn_tasks.after(setup_channel),
33            ),
34        )
35        .add_systems(Update, (handle_finished_cubes, rotate_light))
36        .run();
37}
```

Additional examples can be found in:  

*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#117)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#42)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#70)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#220)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#120)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#72)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#87)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#41)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#43)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#41)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#102)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#51)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#49)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#121)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#69)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#56)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#32)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#135)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#142)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#161)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#132)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#46)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#318)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#76)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#128)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#136)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#189)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#241)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#233)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#172)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#44)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#359)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#18)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#369)

#### fn [before\_ignore\_deferred](#method.before_ignore_deferred)<M>( self, set: impl [IntoSystemSet](trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>, ) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run before all systems in `set`.

Unlike [`before`](trait.IntoScheduleConfigs.html#method.before "method bevy::prelude::IntoScheduleConfigs::before"), this will not cause the systems in `set` to wait for the deferred effects of `self` to be applied.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#377)

#### fn [after\_ignore\_deferred](#method.after_ignore_deferred)<M>( self, set: impl [IntoSystemSet](trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>, ) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run after all systems in `set`.

Unlike [`after`](trait.IntoScheduleConfigs.html#method.after "method bevy::prelude::IntoScheduleConfigs::after"), this will not wait for the deferred effects of systems in `set` to be applied.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#411-414)

#### fn [distributive\_run\_if](#method.distributive_run_if)<M>( self, condition: impl [SystemCondition](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), ) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Add a run condition to each contained system.

Each system will receive its own clone of the [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") and will only run if the `SystemCondition` is true.

Each individual condition will be evaluated at most once (per schedule run), right before the corresponding system prepares to run.

This is equivalent to calling [`run_if`](trait.IntoScheduleConfigs.html#method.run_if "method bevy::prelude::IntoScheduleConfigs::run_if") on each individual system, as shown below:

```rust
schedule.add_systems((a, b).distributive_run_if(condition));
schedule.add_systems((a.run_if(condition), b.run_if(condition)));
```

##### Note

Because the conditions are evaluated separately for each system, there is no guarantee that all evaluations in a single schedule run will yield the same result. If another system is run inbetween two evaluations it could cause the result of the condition to change.

Use [`run_if`](../ecs/schedule/enum.ScheduleConfigs.html#method.run_if "method bevy::ecs::schedule::ScheduleConfigs::run_if") on a [`SystemSet`](trait.SystemSet.html "trait bevy::prelude::SystemSet") if you want to make sure that either all or none of the systems are run, or you don’t want to evaluate the run condition for each contained system separately.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/picking/debug\_picking.rs ([lines 26-28](../../src/debug_picking/debug_picking.rs.html#26-28))

```rust
7fn main() {
8    App::new()
9        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
10            filter: "bevy_dev_tools=trace".into(), // Show picking logs trace level and up
11            ..default()
12        }))
13        .add_plugins((MeshPickingPlugin, DebugPickingPlugin))
14        .add_systems(Startup, setup_scene)
15        .insert_resource(DebugPickingMode::Normal)
16        // A system that cycles the debugging state when you press F3:
17        .add_systems(
18            PreUpdate,
19            (|mut mode: ResMut<DebugPickingMode>| {
20                *mode = match *mode {
21                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
22                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
23                    DebugPickingMode::Noisy => DebugPickingMode::Disabled,
24                }
25            })
26            .distributive_run_if(bevy::input::common_conditions::input_just_pressed(
27                KeyCode::F3,
28            )),
29        )
30        .run();
31}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#448)

#### fn [run\_if](#method.run_if)<M>(self, condition: impl [SystemCondition](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")<M>) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Run the systems only if the [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") is `true`.

The `SystemCondition` will be evaluated at most once (per schedule run), the first time a system in this set prepares to run.

If this set contains more than one system, calling `run_if` is equivalent to adding each system to a common set and configuring the run condition on that set, as shown below:

##### Examples

```rust
schedule.add_systems((a, b).run_if(condition));
schedule.add_systems((a, b).in_set(C)).configure_sets(C.run_if(condition));
```

##### Note

Because the condition will only be evaluated once, there is no guarantee that the condition is upheld after the first system has run. You need to make sure that no other systems that could invalidate the condition are scheduled inbetween the first and last run system.

Use [`distributive_run_if`](trait.IntoScheduleConfigs.html#method.distributive_run_if "method bevy::prelude::IntoScheduleConfigs::distributive_run_if") if you want the condition to be evaluated for each individual system, right before one is run.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/asset/generated\_assets.rs ([line 9](../../src/generated_assets/generated_assets.rs.html#9))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .add_systems(Update, generate_mesh_system.run_if(run_once))
10        .run();
11}
```

Hide additional examples

examples/usage/cooldown.rs ([line 16](../../src/cooldown/cooldown.rs.html#16))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins)
11        .add_systems(Startup, setup)
12        .add_systems(
13            Update,
14            (
15                activate_ability,
16                animate_cooldowns.run_if(any_with_component::<ActiveCooldown>),
17            ),
18        )
19        .run();
20}
```

examples/diagnostics/enabling\_disabling\_diagnostic.rs ([line 20](../../src/enabling_disabling_diagnostic/enabling_disabling_diagnostic.rs.html#20))

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

examples/remote/server.rs ([line 21](../../src/server/server.rs.html#21))

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

examples/asset/alter\_mesh.rs ([line 14](../../src/alter_mesh/alter_mesh.rs.html#14))

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

examples/asset/alter\_sprite.rs ([line 14](../../src/alter_sprite/alter_sprite.rs.html#14))

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

Additional examples can be found in:  

*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#17)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#14)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#60)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#63)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#34)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#23)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#31)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#18)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#36)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#39)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#55)
*   [examples/ecs/generic\_system.rs](../../src/generic_system/generic_system.rs.html#42)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#33)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#18)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#129)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#62)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#40)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#23)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#31)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#37)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#60)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#20)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#30)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#63)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#43)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#41)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#145)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#40)
*   [examples/state/states.rs](../../src/states/states.rs.html#21)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#21)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#49-52)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#62)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#45)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#72)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#187)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#129)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#20)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#454)

#### fn [ambiguous\_with](#method.ambiguous_with)<M>(self, set: impl [IntoSystemSet](trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Suppress warnings and errors that would result from these systems having ambiguities (conflicting access but indeterminate order) with systems in `set`.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/ecs/nondeterministic\_system\_order.rs ([line 53](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#53))

```rust
20fn main() {
21    App::new()
22        // We can modify the reporting strategy for system execution order ambiguities on a per-schedule basis.
23        // You must do this for each schedule you want to inspect; child schedules executed within an inspected
24        // schedule do not inherit this modification.
25        .edit_schedule(Update, |schedule| {
26            schedule.set_build_settings(ScheduleBuildSettings {
27                ambiguity_detection: LogLevel::Warn,
28                ..default()
29            });
30        })
31        .init_resource::<A>()
32        .init_resource::<B>()
33        .add_systems(
34            Update,
35            (
36                // This pair of systems has an ambiguous order,
37                // as their data access conflicts, and there's no order between them.
38                reads_a,
39                writes_a,
40                // This pair of systems has conflicting data access,
41                // but it's resolved with an explicit ordering:
42                // the .after relationship here means that we will always double after adding.
43                adds_one_to_b,
44                doubles_b.after(adds_one_to_b),
45                // This system isn't ambiguous with adds_one_to_b,
46                // due to the transitive ordering created by our constraints:
47                // if A is before B is before C, then A must be before C as well.
48                reads_b.after(doubles_b),
49                // This system will conflict with all of our writing systems
50                // but we've silenced its ambiguity with adds_one_to_b.
51                // This should only be done in the case of clear false positives:
52                // leave a comment in your code justifying the decision!
53                reads_a_and_b.ambiguous_with(adds_one_to_b),
54            ),
55        )
56        // Be mindful, internal ambiguities are reported too!
57        // If there are any ambiguities due solely to DefaultPlugins,
58        // or between DefaultPlugins and any of your third party plugins,
59        // please file a bug with the repo responsible!
60        // Only *you* can prevent nondeterministic bugs due to greedy parallelism.
61        .add_plugins(DefaultPlugins)
62        .run();
63}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#460)

#### fn [ambiguous\_with\_all](#method.ambiguous_with_all)(self) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Suppress warnings and errors that would result from these systems having ambiguities (conflicting access but indeterminate order) with any other system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#471)

#### fn [chain](#method.chain)(self) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Treat this collection as a sequence of systems.

Ordering constraints will be applied between the successive elements.

If the preceding node on an edge has deferred parameters, an [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") will be inserted on the edge. If this behavior is not desired consider using [`chain_ignore_deferred`](trait.IntoScheduleConfigs.html#method.chain_ignore_deferred "method bevy::prelude::IntoScheduleConfigs::chain_ignore_deferred") instead.

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/reflection/serialization.rs ([line 17](../../src/serialization/serialization.rs.html#17))

```rust
14fn main() {
15    App::new()
16        .add_plugins(DefaultPlugins)
17        .add_systems(Startup, (deserialize, serialize).chain())
18        .run();
19}
```

Hide additional examples

examples/gizmos/axes.rs ([line 14](../../src/axes/axes.rs.html#14))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .add_systems(Update, (move_cubes, draw_axes).chain())
15        .run();
16}
```

examples/movement/smooth\_follow.rs ([line 14](../../src/smooth_follow/smooth_follow.rs.html#14))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .add_systems(Update, (move_target, move_follower).chain())
15        .run();
16}
```

examples/ecs/contiguous\_query.rs ([line 52](../../src/contiguous_query/contiguous_query.rs.html#52))

```rust
49fn main() {
50    App::new()
51        .add_plugins(DefaultPlugins)
52        .add_systems(Update, (apply_health_decay, finish_off_first).chain())
53        .add_systems(Startup, setup)
54        .run();
55}
```

examples/camera/2d\_top\_down\_camera.rs ([line 27](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#27))

```rust
23fn main() {
24    App::new()
25        .add_plugins(DefaultPlugins)
26        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
27        .add_systems(Update, (move_player, update_camera).chain())
28        .run();
29}
```

examples/3d/motion\_blur.rs ([line 16](../../src/motion_blur/motion_blur.rs.html#16))

```rust
11fn main() {
12    let mut app = App::new();
13
14    app.add_plugins(DefaultPlugins)
15        .add_systems(Startup, (setup_camera, setup_scene, setup_ui))
16        .add_systems(Update, (keyboard_inputs, move_cars, move_camera).chain())
17        .run();
18}
```

Additional examples can be found in:  

*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#15)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#19)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#17)
*   [examples/ecs/custom\_executor.rs](../../src/custom_executor/custom_executor.rs.html#51)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#58)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#35)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#22)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#112)
*   [examples/ecs/custom\_query\_param.rs](../../src/custom_query_param/custom_query_param.rs.html#33)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#27)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#30)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#66)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#95)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#69)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#70)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#40)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#88)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#31)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#36)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#67)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#49)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#152)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#54)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#165)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#232)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#120)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#64)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#124)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#338)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#480)

#### fn [chain\_ignore\_deferred](#method.chain_ignore_deferred)(self) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

Treat this collection as a sequence of systems.

Ordering constraints will be applied between the successive elements.

Unlike [`chain`](trait.IntoScheduleConfigs.html#method.chain "method bevy::prelude::IntoScheduleConfigs::chain") this will **not** add [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred") on the edges.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#613-620)

### impl<P, S, T> [IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<T, (ScheduleConfigTupleMarker, P)> for [(S₁, S₂, …, Sₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where T: [Schedulable](../ecs/schedule/trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable")<Metadata = [GraphInfo](../ecs/schedule/struct.GraphInfo.html "struct bevy::ecs::schedule::GraphInfo"), GroupMetadata = [Chain](../ecs/schedule/enum.Chain.html "enum bevy::ecs::schedule::Chain")\>, S: [IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<T, P>,

This trait is implemented for tuples up to 20 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#613-620)

#### fn [into\_configs](#tymethod.into_configs)(self) -> [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#571)

### impl [IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [System](trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [System](trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#561-563)

### impl<F, Marker> [IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [System](trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>, Marker> for F

where F: [IntoSystem](trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Marker>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#577)

### impl<S> [IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](trait.SystemSet.html "trait bevy::prelude::SystemSet")\>, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for S

where S: [SystemSet](trait.SystemSet.html "trait bevy::prelude::SystemSet"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/config.rs.html#485-486)

### impl<T> [IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<T, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [ScheduleConfigs](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<T>

where T: [Schedulable](../ecs/schedule/trait.Schedulable.html "trait bevy::ecs::schedule::Schedulable")<Metadata = [GraphInfo](../ecs/schedule/struct.GraphInfo.html "struct bevy::ecs::schedule::GraphInfo"), GroupMetadata = [Chain](../ecs/schedule/enum.Chain.html "enum bevy::ecs::schedule::Chain")\>,