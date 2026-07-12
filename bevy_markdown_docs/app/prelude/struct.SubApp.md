[bevy](../../index.html)::[app](../index.html)::[prelude](index.html)

# Struct SubApp 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#65)

```rust
pub struct SubApp {
    pub update_schedule: Option<Interned<dyn ScheduleLabel>>,
    /* private fields */
}
```

A secondary application with its own [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"). These can run independently of each other.

These are useful for situations where certain processes (e.g. a render thread) need to be kept separate from the main application.

## Example

```rust
#[derive(Resource, Default)]
struct Val(pub i32);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
struct ExampleApp;

// Create an app with a certain resource.
let mut app = App::new();
app.insert_resource(Val(10));

// Create a sub-app with the same resource and a single schedule.
let mut sub_app = SubApp::new();
sub_app.update_schedule = Some(Main.intern());
sub_app.insert_resource(Val(100));

// Setup an extract function to copy the resource's value in the main world.
sub_app.set_extract(|main_world, sub_world| {
    sub_world.resource_mut::<Val>().0 = main_world.resource::<Val>().0;
});

// Schedule a system that will verify extraction is working.
sub_app.add_systems(Main, |counter: Res<Val>| {
    // The value will be copied during extraction, so we should see 10 instead of 100.
    assert_eq!(counter.0, 10);
});

// Add the sub-app to the main app.
app.insert_sub_app(ExampleApp, sub_app);

// Update the application once (using the default runner).
app.run();
```

## Fields

`update_schedule: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")>>`

The schedule that will be run by [`update`](../../prelude/struct.SubApp.html#method.update "method bevy::prelude::SubApp::update").

## Implementations

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#105)

### impl [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#107)

#### pub fn [new](#method.new)() -> [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Returns a default, empty [`SubApp`](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#124)

#### pub fn [world](#method.world)(&self) -> &[World](../../prelude/struct.World.html "struct bevy::prelude::World")

Returns a reference to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 110](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#110))

```rust
105    fn update(&mut self) {
106        self.0.update();
107        // Wait for frame to finish rendering by wait polling the device
108        self.0
109            .main
110            .world()
111            .resource::<RenderDevice>()
112            .wgpu_device()
113            .poll(PollType::Wait {
114                submission_index: None,
115                timeout: None,
116            })
117            .unwrap();
118    }
```

Hide additional examples

tests/ecs/ambiguity\_detection.rs ([line 93](../../../src/ambiguity_detection/ambiguity_detection.rs.html#93))

```rust
91fn count_ambiguities(sub_app: &mut SubApp) -> AmbiguitiesCount {
92    let schedule_labels = sub_app
93        .world()
94        .resource::<Schedules>()
95        .iter()
96        .map(|(_, schedule)| schedule.label())
97        .collect::<Vec<_>>();
98    let mut ambiguities = <HashMap<_, _>>::default();
99    for label in schedule_labels {
100        let ambiguities_in_schedule =
101            sub_app
102                .world_mut()
103                .schedule_scope(label, |world, schedule| {
104                    schedule.initialize(world).unwrap().unwrap();
105                    schedule.graph().conflicting_systems().len()
106                });
107        ambiguities.insert(label, ambiguities_in_schedule);
108    }
109    AmbiguitiesCount(ambiguities)
110}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#129)

#### pub fn [world\_mut](#method.world_mut)(&mut self) -> &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")

Returns a mutable reference to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/compute\_mesh.rs ([line 67](../../../src/compute_mesh/compute_mesh.rs.html#67))

```rust
62    fn finish(&self, app: &mut App) {
63        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
64            return;
65        };
66        render_app
67            .world_mut()
68            .resource_mut::<MeshAllocatorSettings>()
69            // This allows using the mesh allocator slabs as
70            // storage buffers directly in the compute shader.
71            // Which means that we can write from our compute
72            // shader directly to the allocated mesh slabs.
73            .extra_buffer_usages = BufferUsages::STORAGE;
74    }
```

Hide additional examples

examples/app/externally\_driven\_headless\_renderer.rs ([line 90](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#90))

```rust
75    fn new_render_target(&mut self, width: u32, height: u32) -> RenderTarget {
76        let mut target = Image::new_uninit(
77            Extent3d {
78                width,
79                height,
80                depth_or_array_layers: 1,
81            },
82            TextureDimension::D2,
83            TextureFormat::Rgba8UnormSrgb,
84            RenderAssetUsages::RENDER_WORLD,
85        );
86        // We're going to render to this image, mark it as such
87        target.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
88        self.0
89            .main
90            .world_mut()
91            .resource_mut::<Assets<Image>>()
92            .add(target)
93            .into()
94    }
95
96    fn spawn_camera(&mut self, target: RenderTarget) -> Entity {
97        self.0
98            .main
99            .world_mut()
100            .spawn((Camera3d::default(), target, Transform::IDENTITY))
101            .id()
102    }
103
104    // Run one world update and wait for rendering to finish.
105    fn update(&mut self) {
106        self.0.update();
107        // Wait for frame to finish rendering by wait polling the device
108        self.0
109            .main
110            .world()
111            .resource::<RenderDevice>()
112            .wgpu_device()
113            .poll(PollType::Wait {
114                submission_index: None,
115                timeout: None,
116            })
117            .unwrap();
118    }
119
120    // Schedules a screenshot to be captured on the next update.
121    fn screenshot(&mut self, target: RenderTarget, i: u32) {
122        self.0
123            .main
124            .world_mut()
125            .spawn(Screenshot::image(target.as_image().unwrap().clone()))
126            .observe(save_to_disk(format!("test_images/screenshot{i}.png")));
127    }
```

tests/ecs/ambiguity\_detection.rs ([line 71](../../../src/ambiguity_detection/ambiguity_detection.rs.html#71))

```rust
70fn configure_ambiguity_detection(sub_app: &mut SubApp) {
71    let mut schedules = sub_app.world_mut().resource_mut::<Schedules>();
72    for (_, schedule) in schedules.iter_mut() {
73        schedule.set_build_settings(ScheduleBuildSettings {
74            // NOTE: you can change this to `LogLevel::Ignore` to easily see the current number of ambiguities.
75            ambiguity_detection: LogLevel::Warn,
76            // With auto-inserted apply_deferred stages, these can cause two ambiguous systems to
77            // become accidentally ordered by one of the apply_deferred stages. Disabling requires
78            // us to meet a higher bar. We don't just want no ambiguities - we also don't want
79            // changes to systems or the auto-insert code from "creating" new ambiguities (by
80            // reordering the graph). However, the cost is that the graph is no longer runnable,
81            // since Bevy crates often rely on auto-insert apply_deferred to not panic (e.g.,
82            // because a resource wasn't inserted).
83            auto_insert_apply_deferred: false,
84            use_shortnames: false,
85            ..default()
86        });
87    }
88}
89
90/// Returns the number of conflicting systems per schedule.
91fn count_ambiguities(sub_app: &mut SubApp) -> AmbiguitiesCount {
92    let schedule_labels = sub_app
93        .world()
94        .resource::<Schedules>()
95        .iter()
96        .map(|(_, schedule)| schedule.label())
97        .collect::<Vec<_>>();
98    let mut ambiguities = <HashMap<_, _>>::default();
99    for label in schedule_labels {
100        let ambiguities_in_schedule =
101            sub_app
102                .world_mut()
103                .schedule_scope(label, |world, schedule| {
104                    schedule.initialize(world).unwrap().unwrap();
105                    schedule.graph().conflicting_systems().len()
106                });
107        ambiguities.insert(label, ambiguities_in_schedule);
108    }
109    AmbiguitiesCount(ambiguities)
110}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#136)

#### pub fn [run\_default\_schedule](#method.run_default_schedule)(&mut self)

Runs the default schedule.

Does not clear internal trackers used for change detection.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#147)

#### pub fn [update](#method.update)(&mut self)

Runs the default schedule and updates internal component trackers.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#156)

#### pub fn [extract](#method.extract)(&mut self, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Extracts data from `world` into the app’s world using the registered extract method.

**Note:** There is no default extract method. Calling `extract` does nothing if [`set_extract`](../../prelude/struct.SubApp.html#method.set_extract "method bevy::prelude::SubApp::set_extract") has not been called.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#165-167)

#### pub fn [set\_extract](#method.set_extract)<F>(&mut self, extract: F) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Sets the method that will be called by [`extract`](../../prelude/struct.SubApp.html#method.extract "method bevy::prelude::SubApp::extract").

The first argument is the `World` to extract data from, the second argument is the app `World`.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#198)

#### pub fn [take\_extract](#method.take_extract)( &mut self, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>>

Take the function that will be called by [`extract`](../../prelude/struct.SubApp.html#method.extract "method bevy::prelude::SubApp::extract") out of the app, if any was set, and replace it with `None`.

If you use Bevy, `bevy_render` will set a default extract function used to extract data from the main world into the render world as part of the Extract phase. In that case, you cannot replace it with your own function. Instead, take the Bevy default function with this, and install your own instead which calls the Bevy default.

```rust
let mut default_fn = app.take_extract();
app.set_extract(move |main, render| {
    // Do pre-extract custom logic
    // [...]

    // Call Bevy's default, which executes the Extract phase
    if let Some(f) = default_fn.as_mut() {
        f(main, render);
    }

    // Do post-extract custom logic
    // [...]
});
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#203)

#### pub fn [insert\_resource](#method.insert_resource)<R>(&mut self, resource: R) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

See [`App::insert_resource`](../../prelude/struct.App.html#method.insert_resource "method bevy::prelude::App::insert_resource").

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([line 213](../../../src/headless_renderer/headless_renderer.rs.html#213))

```rust
205    fn build(&self, app: &mut App) {
206        let (s, r) = crossbeam_channel::unbounded();
207
208        let render_app = app
209            .insert_resource(MainWorldReceiver(r))
210            .sub_app_mut(RenderApp);
211
212        render_app
213            .insert_resource(RenderWorldSender(s))
214            // Make ImageCopiers accessible in RenderWorld system and plugin
215            .add_systems(ExtractSchedule, image_copy_extract)
216            // Receives image data from buffer to channel
217            // so we need to run it after the render graph is done
218            .add_systems(
219                Render,
220                receive_image_from_buffer.after(RenderSystems::Render),
221            )
222            .add_systems(RenderGraph, image_copy_driver);
223    }
```

Hide additional examples

examples/ecs/extraction.rs ([line 66](../../../src/extraction/extraction.rs.html#66))

```rust
46fn main() {
47    let mut app = App::new();
48
49    // Main World
50    app.insert_resource(WorldName("Main World".into()))
51        .add_plugins((
52            DefaultPlugins,
53            // Plugin for automatically extracting A.
54            ExtractComponentPlugin::<A>::default(),
55        ))
56        .add_message::<ExtractMessage>()
57        .add_systems(Startup, setup)
58        .add_systems(Update, (set_time, trigger_extraction, display_state));
59
60    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
61        return;
62    };
63
64    // Render World
65    render_app
66        .insert_resource(WorldName("Render World".into()))
67        .add_systems(ExtractSchedule, extract_components)
68        .add_systems(Render, display_state);
69
70    app.run();
71}
```

examples/2d/mesh2d\_manual.rs ([line 312](../../../src/mesh2d_manual/mesh2d_manual.rs.html#312))

```rust
300    fn build(&self, app: &mut App) {
301        // Load our custom shader
302        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
303        // Here, we construct and add the shader asset manually. There are many ways to load this
304        // shader, including `embedded_asset`/`load_embedded_asset`.
305        let shader = shaders.add(Shader::from_wgsl(COLORED_MESH2D_SHADER, file!()));
306
307        app.add_plugins(SyncComponentPlugin::<ColoredMesh2d>::default());
308
309        // Register our custom draw function, and add our render systems
310        app.get_sub_app_mut(RenderApp)
311            .unwrap()
312            .insert_resource(ColoredMesh2dShader(shader))
313            .add_render_command::<Transparent2d, DrawColoredMesh2d>()
314            .init_resource::<SpecializedRenderPipelines<ColoredMesh2dPipeline>>()
315            .init_resource::<RenderColoredMesh2dInstances>()
316            .add_systems(
317                RenderStartup,
318                init_colored_mesh_2d_pipeline.after(init_mesh_2d_pipeline),
319            )
320            .add_systems(
321                ExtractSchedule,
322                extract_colored_mesh2d.after(extract_mesh2d),
323            )
324            .add_systems(
325                Render,
326                queue_colored_mesh2d.in_set(RenderSystems::QueueMeshes),
327            );
328    }
```

examples/3d/occlusion\_culling.rs ([line 214](../../../src/occlusion_culling/occlusion_culling.rs.html#214))

```rust
197    fn build(&self, app: &mut App) {
198        // Create the `SavedIndirectParameters` resource that we're going to use
199        // to communicate between the thread that the GPU-to-CPU readback
200        // callback runs on and the main application threads. This resource is
201        // atomically reference counted. We store one reference to the
202        // `SavedIndirectParameters` in the main app and another reference in
203        // the render app.
204        let saved_indirect_parameters = SavedIndirectParameters::new();
205        app.insert_resource(saved_indirect_parameters.clone());
206
207        // Fetch the render app.
208        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
209            return;
210        };
211
212        render_app
213            // Insert another reference to the `SavedIndirectParameters`.
214            .insert_resource(saved_indirect_parameters)
215            // Setup the parameters in RenderStartup.
216            .add_systems(RenderStartup, init_saved_indirect_parameters)
217            .init_resource::<IndirectParametersStagingBuffers>()
218            .add_systems(ExtractSchedule, readback_indirect_parameters)
219            .add_systems(
220                Render,
221                create_indirect_parameters_staging_buffers
222                    .in_set(RenderSystems::PrepareResourcesFlush),
223            )
224            .add_systems(
225                Core3d,
226                // Add the node that allows us to read the indirect parameters back
227                // from the GPU to the CPU, which allows us to determine how many
228                // meshes were culled.
229                readback_indirect_parameters_node
230                    // We read back the indirect parameters any time after
231                    // `MainPass`. Readback doesn't particularly need to execute
232                    // before PostProcess, but we order it that way anyway.
233                    .after(Core3dSystems::MainPass)
234                    .before(Core3dSystems::PostProcess),
235            );
236    }
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#209)

#### pub fn [init\_resource](#method.init_resource)<R>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

See [`App::init_resource`](../../prelude/struct.App.html#method.init_resource "method bevy::prelude::App::init_resource").

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/compute\_mesh.rs ([line 57](../../../src/compute_mesh/compute_mesh.rs.html#57))

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

examples/shader\_advanced/custom\_shader\_instancing.rs ([line 114](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#114))

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

examples/shader\_advanced/custom\_phase\_item.rs ([line 172](../../../src/custom_phase_item/custom_phase_item.rs.html#172))

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

examples/shader/compute\_shader\_game\_of\_life.rs ([line 103](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#103))

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

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 115](../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#115))

```rust
106    fn build(&self, app: &mut App) {
107        app.add_plugins(ExtractComponentPlugin::<CustomRenderedEntity>::default());
108
109        // We make sure to add these to the render app, not the main app.
110        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
111            return;
112        };
113        render_app
114            // This is needed to tell bevy about your custom pipeline
115            .init_resource::<SpecializedMeshPipelines<CustomMeshPipeline>>()
116            .init_resource::<PendingCustomMeshQueues>()
117            // We need to use a custom draw command so we need to register it
118            .add_render_command::<Opaque3d, DrawSpecializedPipelineCommands>()
119            .add_systems(
120                RenderStartup,
121                init_custom_mesh_pipeline.after(MeshPipelineSystems),
122            )
123            .add_systems(
124                Render,
125                queue_custom_mesh_pipeline.in_set(RenderSystems::Queue),
126            );
127    }
```

examples/2d/mesh2d\_manual.rs ([line 314](../../../src/mesh2d_manual/mesh2d_manual.rs.html#314))

```rust
300    fn build(&self, app: &mut App) {
301        // Load our custom shader
302        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
303        // Here, we construct and add the shader asset manually. There are many ways to load this
304        // shader, including `embedded_asset`/`load_embedded_asset`.
305        let shader = shaders.add(Shader::from_wgsl(COLORED_MESH2D_SHADER, file!()));
306
307        app.add_plugins(SyncComponentPlugin::<ColoredMesh2d>::default());
308
309        // Register our custom draw function, and add our render systems
310        app.get_sub_app_mut(RenderApp)
311            .unwrap()
312            .insert_resource(ColoredMesh2dShader(shader))
313            .add_render_command::<Transparent2d, DrawColoredMesh2d>()
314            .init_resource::<SpecializedRenderPipelines<ColoredMesh2dPipeline>>()
315            .init_resource::<RenderColoredMesh2dInstances>()
316            .add_systems(
317                RenderStartup,
318                init_colored_mesh_2d_pipeline.after(init_mesh_2d_pipeline),
319            )
320            .add_systems(
321                ExtractSchedule,
322                extract_colored_mesh2d.after(extract_mesh2d),
323            )
324            .add_systems(
325                Render,
326                queue_colored_mesh2d.in_set(RenderSystems::QueueMeshes),
327            );
328    }
```

Additional examples can be found in:  

*   [examples/shader\_advanced/custom\_render\_phase.rs](../../../src/custom_render_phase/custom_render_phase.rs.html#129)
*   [tests/ecs/ambiguity\_detection.rs](../../../src/ambiguity_detection/ambiguity_detection.rs.html#51)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#217)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#215-219)

#### pub fn [add\_systems](#method.add_systems)<M>( &mut self, schedule: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), systems: impl [IntoScheduleConfigs](../../prelude/trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>, M>, ) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::add_systems`](../../prelude/struct.App.html#method.add_systems "method bevy::prelude::App::add_systems").

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/texture\_binding\_array.rs ([line 49](../../../src/texture_binding_array/texture_binding_array.rs.html#49))

```rust
44    fn build(&self, app: &mut App) {
45        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
46            return;
47        };
48
49        render_app.add_systems(RenderStartup, verify_required_features);
50    }
```

Hide additional examples

examples/stress\_tests/many\_lights.rs ([lines 158-161](../../../src/many_lights/many_lights.rs.html#158-161))

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

examples/app/render\_recovery.rs ([line 30](../../../src/render_recovery/render_recovery.rs.html#30))

```rust
20fn main() {
21    let mut app = App::new();
22    app.add_plugins((
23        DefaultPlugins,
24        ExtractResourcePlugin::<RenderError>::default(),
25    ))
26    .add_systems(Startup, setup)
27    .add_systems(Update, (update_camera, input))
28    .init_resource::<RenderError>()
29    .sub_app_mut(RenderApp)
30    .add_systems(Render, cause_error);
31    app.run();
32}
```

examples/shader\_advanced/compute\_mesh.rs ([line 58](../../../src/compute_mesh/compute_mesh.rs.html#58))

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

examples/showcase/loading\_screen.rs ([line 293](../../../src/loading_screen/loading_screen.rs.html#293))

```rust
284        fn build(&self, app: &mut App) {
285            app.insert_resource(PipelinesReady::default());
286
287            // In order to gain access to the pipelines status, we have to
288            // go into the `RenderApp`, grab the resource from the main App
289            // and then update the pipelines status from there.
290            // Writing between these Apps can only be done through the
291            // `ExtractSchedule`.
292            app.sub_app_mut(RenderApp)
293                .add_systems(ExtractSchedule, update_pipelines_ready);
294        }
```

examples/shader/gpu\_readback.rs ([line 49](../../../src/gpu_readback/gpu_readback.rs.html#49))

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

Additional examples can be found in:  

*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#115-118)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../../src/custom_phase_item/custom_phase_item.rs.html#175-178)
*   [examples/app/headless\_renderer.rs](../../../src/headless_renderer/headless_renderer.rs.html#215)
*   [examples/ecs/extraction.rs](../../../src/extraction/extraction.rs.html#67)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#117-122)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#104)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#119-122)
*   [examples/shader\_advanced/manual\_material.rs](../../../src/manual_material/manual_material.rs.html#65)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#62)
*   [examples/2d/mesh2d\_manual.rs](../../../src/mesh2d_manual/mesh2d_manual.rs.html#316-319)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../../src/custom_render_phase/custom_render_phase.rs.html#134-137)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#250)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#216)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#227-232)

#### pub fn [remove\_systems\_in\_set](#method.remove_systems_in_set)<M>( &mut self, schedule: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), set: impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>, policy: [ScheduleCleanupPolicy](../../ecs/schedule/enum.ScheduleCleanupPolicy.html "enum bevy::ecs::schedule::ScheduleCleanupPolicy"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [ScheduleError](../../ecs/schedule/enum.ScheduleError.html "enum bevy::ecs::schedule::ScheduleError")\>

See [`App::remove_systems_in_set`](../../prelude/struct.App.html#method.remove_systems_in_set "method bevy::prelude::App::remove_systems_in_set")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#239-245)

#### pub fn [register\_system](#method.register_system)<I, O, M>( &mut self, system: impl [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemId](../../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

See [`App::register_system`](../../prelude/struct.App.html#method.register_system "method bevy::prelude::App::register_system").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#251-257)

#### pub fn [register\_tracked\_system](#method.register_tracked_system)<I, O, M>( &mut self, system: impl [IntoSystem](../../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemHandle](../../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")<I, O>

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

See [`App::register_tracked_system`](../../prelude/struct.App.html#method.register_tracked_system "method bevy::prelude::App::register_tracked_system").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#264-268)

#### pub fn [configure\_sets](#method.configure_sets)<M>( &mut self, schedule: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), sets: impl [IntoScheduleConfigs](../../prelude/trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>, M>, ) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::configure_sets`](../../prelude/struct.App.html#method.configure_sets "method bevy::prelude::App::configure_sets").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#275)

#### pub fn [add\_schedule](#method.add_schedule)(&mut self, schedule: [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::add_schedule`](../../prelude/struct.App.html#method.add_schedule "method bevy::prelude::App::add_schedule").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#291)

#### pub fn [init\_schedule](#method.init_schedule)(&mut self, label: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::init_schedule`](../../prelude/struct.App.html#method.init_schedule "method bevy::prelude::App::init_schedule").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#301)

#### pub fn [get\_schedule](#method.get_schedule)(&self, label: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")\>

See [`App::get_schedule`](../../prelude/struct.App.html#method.get_schedule "method bevy::prelude::App::get_schedule").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#307)

#### pub fn [get\_schedule\_mut](#method.get_schedule_mut)( &mut self, label: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")\>

See [`App::get_schedule_mut`](../../prelude/struct.App.html#method.get_schedule_mut "method bevy::prelude::App::get_schedule_mut").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#315-319)

#### pub fn [edit\_schedule](#method.edit_schedule)( &mut self, label: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), f: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")), ) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::edit_schedule`](../../prelude/struct.App.html#method.edit_schedule "method bevy::prelude::App::edit_schedule").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#333-336)

#### pub fn [configure\_schedules](#method.configure_schedules)( &mut self, schedule\_build\_settings: [ScheduleBuildSettings](../../ecs/schedule/struct.ScheduleBuildSettings.html "struct bevy::ecs::schedule::ScheduleBuildSettings"), ) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::configure_schedules`](../../prelude/struct.App.html#method.configure_schedules "method bevy::prelude::App::configure_schedules").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#344)

#### pub fn [allow\_ambiguous\_component](#method.allow_ambiguous_component)<T>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

See [`App::allow_ambiguous_component`](../../prelude/struct.App.html#method.allow_ambiguous_component "method bevy::prelude::App::allow_ambiguous_component").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#350)

#### pub fn [allow\_ambiguous\_resource](#method.allow_ambiguous_resource)<T>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

See [`App::allow_ambiguous_resource`](../../prelude/struct.App.html#method.allow_ambiguous_resource "method bevy::prelude::App::allow_ambiguous_resource").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#357-365)

#### pub fn [ignore\_ambiguity](#method.ignore_ambiguity)<M1, M2, S1, S2>( &mut self, schedule: impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), a: S1, b: S2, ) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S1: [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M1>, S2: [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M2>,

See [`App::ignore_ambiguity`](../../prelude/struct.App.html#method.ignore_ambiguity "method bevy::prelude::App::ignore_ambiguity").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#376)

#### pub fn [add\_observer](#method.add_observer)<M>(&mut self, observer: impl [IntoObserver](../../ecs/observer/trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")<M>) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::add_observer`](../../prelude/struct.App.html#method.add_observer "method bevy::prelude::App::add_observer").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#382-384)

#### pub fn [add\_message](#method.add_message)<T>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

See [`App::add_message`](../../prelude/struct.App.html#method.add_message "method bevy::prelude::App::add_message").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#394)

#### pub fn [add\_plugins](#method.add_plugins)<M>(&mut self, plugins: impl [Plugins](../trait.Plugins.html "trait bevy::app::Plugins")<M>) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

See [`App::add_plugins`](../../prelude/struct.App.html#method.add_plugins "method bevy::prelude::App::add_plugins").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#400-402)

#### pub fn [is\_plugin\_added](#method.is_plugin_added)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Plugin](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

See [`App::is_plugin_added`](../../prelude/struct.App.html#method.is_plugin_added "method bevy::prelude::App::is_plugin_added").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#408-410)

#### pub fn [get\_added\_plugins](#method.get_added_plugins)<T>(&self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Plugin](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

See [`App::get_added_plugins`](../../prelude/struct.App.html#method.get_added_plugins "method bevy::prelude::App::get_added_plugins").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#425)

#### pub fn [plugins\_state](#method.plugins_state)(&mut self) -> [PluginsState](../enum.PluginsState.html "enum bevy::app::PluginsState")

Return the state of plugins.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#446)

#### pub fn [finish](#method.finish)(&mut self)

Runs [`Plugin::finish`](../../prelude/trait.Plugin.html#method.finish "method bevy::prelude::Plugin::finish") for each plugin.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#463)

#### pub fn [cleanup](#method.cleanup)(&mut self)

Runs [`Plugin::cleanup`](../../prelude/trait.Plugin.html#method.cleanup "method bevy::prelude::Plugin::cleanup") for each plugin.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#481)

#### pub fn [register\_type](#method.register_type)<T>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

Available on **crate feature `bevy_reflect`** only.

See [`App::register_type`](../../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#489-494)

#### pub fn [register\_type\_data](#method.register_type_data)<T, D>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), D: [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") + [FromType](../../reflect/trait.FromType.html "trait bevy::reflect::FromType")<T>,

Available on **crate feature `bevy_reflect`** only.

See [`App::register_type_data`](../../prelude/struct.App.html#method.register_type_data "method bevy::prelude::App::register_type_data").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#502-506)

#### pub fn [register\_type\_conversion](#method.register_type_conversion)<T, U, F>(&mut self, function: F) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), U: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, T> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Available on **crate feature `bevy_reflect`** only.

See [`App::register_type_conversion`](../../prelude/struct.App.html#method.register_type_conversion "method bevy::prelude::App::register_type_conversion").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#517-520)

#### pub fn [register\_into\_type\_conversion](#method.register_into_type_conversion)<T, U>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), U: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

Available on **crate feature `bevy_reflect`** only.

See [`App::register_into_type_conversion`](../../prelude/struct.App.html#method.register_into_type_conversion "method bevy::prelude::App::register_into_type_conversion").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#529-531)

#### pub fn [register\_function](#method.register_function)<F, Marker>(&mut self, function: F) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where F: [IntoFunction](../../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction")<'static, Marker> + 'static,

Available on **crate feature `reflect_functions`** only.

See [`App::register_function`](../../prelude/struct.App.html#method.register_function "method bevy::prelude::App::register_function").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#540-546)

#### pub fn [register\_function\_with\_name](#method.register_function_with_name)<F, Marker>( &mut self, name: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, function: F, ) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where F: [IntoFunction](../../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction")<'static, Marker> + 'static,

Available on **crate feature `reflect_functions`** only.

See [`App::register_function_with_name`](../../prelude/struct.App.html#method.register_function_with_name "method bevy::prelude::App::register_function_with_name").

## Trait Implementations

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#360)

### impl [AddRenderCommand](../../render/render_phase/trait.AddRenderCommand.html "trait bevy::render::render_phase::AddRenderCommand") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#361-365)

#### fn [add\_render\_command](../../render/render_phase/trait.AddRenderCommand.html#tymethod.add_render_command)<P, C>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where P: [PhaseItem](../../render/render_phase/trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), C: [RenderCommand](../../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <C as [RenderCommand](../../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](../../render/render_phase/trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param"): [ReadOnlySystemParam](../../ecs/system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

Adds the [`RenderCommand`](../../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") for the specified render phase to the app.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#95)

### impl [AppExtStates](../../prelude/trait.AppExtStates.html "trait bevy::prelude::AppExtStates") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#96)

#### fn [init\_state](../../prelude/trait.AppExtStates.html#tymethod.init_state)<S>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes a [`State`](../../prelude/struct.State.html "struct bevy::prelude::State") with standard starting values. [Read more](../../prelude/trait.AppExtStates.html#tymethod.init_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#122)

#### fn [insert\_state](../../prelude/trait.AppExtStates.html#tymethod.insert_state)<S>(&mut self, state: S) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Inserts a specific [`State`](../../prelude/struct.State.html "struct bevy::prelude::State") to the current [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") and overrides any [`State`](../../prelude/struct.State.html "struct bevy::prelude::State") previously added of the same type. [Read more](../../prelude/trait.AppExtStates.html#tymethod.insert_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#157)

#### fn [add\_computed\_state](../../prelude/trait.AppExtStates.html#tymethod.add_computed_state)<S>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S: [ComputedStates](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"),

Sets up a type implementing [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"). [Read more](../../prelude/trait.AppExtStates.html#tymethod.add_computed_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#186)

#### fn [add\_sub\_state](../../prelude/trait.AppExtStates.html#tymethod.add_sub_state)<S>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S: [SubStates](../../prelude/trait.SubStates.html "trait bevy::prelude::SubStates"),

Sets up a type implementing [`SubStates`](../../prelude/trait.SubStates.html "trait bevy::prelude::SubStates"). [Read more](../../prelude/trait.AppExtStates.html#tymethod.add_sub_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#218-220)

#### fn [register\_type\_state](../../prelude/trait.AppExtStates.html#tymethod.register_type_state)<S>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

Available on **crate feature `bevy_reflect`** only.

Registers the state type `T` using [`App::register_type`](../../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type"), and adds [`ReflectState`](../../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") type data to `T` in the type registry. [Read more](../../prelude/trait.AppExtStates.html#tymethod.register_type_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#230-232)

#### fn [register\_type\_mutable\_state](../../prelude/trait.AppExtStates.html#tymethod.register_type_mutable_state)<S>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

Available on **crate feature `bevy_reflect`** only.

Registers the state type `T` using [`App::register_type`](../../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type"), and adds [`crate::reflect::ReflectState`](../../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") and [`crate::reflect::ReflectFreelyMutableState`](../../prelude/struct.ReflectFreelyMutableState.html "struct bevy::prelude::ReflectFreelyMutableState") type data to `T` in the type registry. [Read more](../../prelude/trait.AppExtStates.html#tymethod.register_type_mutable_state)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#83)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#84)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#89)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/sub_app.rs.html#90)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#236)

### impl [GpuResourceAppExt](../../render/trait.GpuResourceAppExt.html "trait bevy::render::GpuResourceAppExt") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#237)

#### fn [init\_gpu\_resource](../../render/trait.GpuResourceAppExt.html#tymethod.init_gpu_resource)<R>(&mut self) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where R: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Causes the provided GPU resource to be re-initialized during [`RenderStartup`](../../render/struct.RenderStartup.html "struct bevy::render::RenderStartup"). [Read more](../../render/trait.GpuResourceAppExt.html#tymethod.init_gpu_resource)

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#423)

### impl [RegisterDiagnostic](../../diagnostic/trait.RegisterDiagnostic.html "trait bevy::diagnostic::RegisterDiagnostic") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#424)

#### fn [register\_diagnostic](../../diagnostic/trait.RegisterDiagnostic.html#tymethod.register_diagnostic)(&mut self, diagnostic: [Diagnostic](../../diagnostic/struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic")) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Register a new [`Diagnostic`](../../diagnostic/struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic") with an [`App`](../../prelude/struct.App.html "struct bevy::prelude::App"). [Read more](../../diagnostic/trait.RegisterDiagnostic.html#tymethod.register_diagnostic)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#172)

### impl [StateScopedMessagesAppExt](../../prelude/trait.StateScopedMessagesAppExt.html "trait bevy::prelude::StateScopedMessagesAppExt") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#173)

#### fn [clear\_messages\_on\_exit](../../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_exit)<M>(&mut self, state: impl [States](../../prelude/trait.States.html "trait bevy::prelude::States")) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Clears a [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") when exiting the specified `state`. [Read more](../../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_exit)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#178)

#### fn [clear\_messages\_on\_enter](../../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_enter)<M>(&mut self, state: impl [States](../../prelude/trait.States.html "trait bevy::prelude::States")) -> &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Clears a [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") when entering the specified `state`. [Read more](../../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_enter)

## Auto Trait Implementations

### impl ![Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

### impl ![Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

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

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

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

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}