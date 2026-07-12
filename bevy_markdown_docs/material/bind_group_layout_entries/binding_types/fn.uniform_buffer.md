[bevy](../../../index.html)::[material](../../index.html)::[bind\_group\_layout\_entries](../index.html)::[binding\_types](index.html)

# Function uniform\_buffer 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#410)

```rust
pub fn uniform_buffer<T>(
    has_dynamic_offset: bool,
) -> BindGroupLayoutEntryBuilderwhere
    T: ShaderType,
```

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/shader\_advanced/compute\_mesh.rs ([line 225](../../../../src/compute_mesh/compute_mesh.rs.html#225))

```rust
214fn init_compute_pipeline(
215    mut commands: Commands,
216    asset_server: Res<AssetServer>,
217    pipeline_cache: Res<PipelineCache>,
218) {
219    let layout = BindGroupLayoutDescriptor::new(
220        "",
221        &BindGroupLayoutEntries::sequential(
222            ShaderStages::COMPUTE,
223            (
224                // offsets
225                uniform_buffer::<DataRanges>(false),
226                // vertices
227                storage_buffer::<Vec<u32>>(false),
228                // indices
229                storage_buffer::<Vec<u32>>(false),
230            ),
231        ),
232    );
233    let shader = asset_server.load(SHADER_ASSET_PATH);
234    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
235        label: Some("Mesh generation compute shader".into()),
236        layout: vec![layout.clone()],
237        shader: shader.clone(),
238        ..default()
239    });
240    commands.insert_resource(ComputePipeline { layout, pipeline });
241}
```

Hide additional examples

examples/shader/compute\_shader\_game\_of\_life.rs ([line 186](../../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#186))

```rust
174fn init_game_of_life_pipeline(
175    mut commands: Commands,
176    asset_server: Res<AssetServer>,
177    pipeline_cache: Res<PipelineCache>,
178) {
179    let texture_bind_group_layout = BindGroupLayoutDescriptor::new(
180        "GameOfLifeImages",
181        &BindGroupLayoutEntries::sequential(
182            ShaderStages::COMPUTE,
183            (
184                texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::ReadOnly),
185                texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::WriteOnly),
186                uniform_buffer::<GameOfLifeUniforms>(false),
187            ),
188        ),
189    );
190    let shader = asset_server.load(SHADER_ASSET_PATH);
191    let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
192        layout: vec![texture_bind_group_layout.clone()],
193        shader: shader.clone(),
194        entry_point: Some(Cow::from("init")),
195        ..default()
196    });
197    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
198        layout: vec![texture_bind_group_layout.clone()],
199        shader,
200        entry_point: Some(Cow::from("update")),
201        ..default()
202    });
203
204    commands.insert_resource(GameOfLifePipeline {
205        texture_bind_group_layout,
206        init_pipeline,
207        update_pipeline,
208    });
209}
```

examples/shader\_advanced/custom\_post\_processing.rs ([line 193](../../../../src/custom_post_processing/custom_post_processing.rs.html#193))

```rust
174fn init_post_process_pipeline(
175    mut commands: Commands,
176    render_device: Res<RenderDevice>,
177    asset_server: Res<AssetServer>,
178    fullscreen_shader: Res<FullscreenShader>,
179    pipeline_cache: Res<PipelineCache>,
180) {
181    // We need to define the bind group layout used for our pipeline
182    let layout = BindGroupLayoutDescriptor::new(
183        "post_process_bind_group_layout",
184        &BindGroupLayoutEntries::sequential(
185            // The layout entries will only be visible in the fragment stage
186            ShaderStages::FRAGMENT,
187            (
188                // The screen texture
189                texture_2d(TextureSampleType::Float { filterable: true }),
190                // The sampler that will be used to sample the screen texture
191                sampler(SamplerBindingType::Filtering),
192                // The settings uniform that will control the effect
193                uniform_buffer::<PostProcessSettings>(true),
194            ),
195        ),
196    );
197    // We can create the sampler here since it won't change at runtime and doesn't depend on the view
198    let sampler = render_device.create_sampler(&SamplerDescriptor::default());
199
200    // Get the shader handle
201    let shader = asset_server.load(SHADER_ASSET_PATH);
202    // This will setup a fullscreen triangle for the vertex state.
203    let vertex_state = fullscreen_shader.to_vertex_state();
204    let pipeline_id = pipeline_cache
205        // This will add the pipeline to the cache and queue its creation
206        .queue_render_pipeline(RenderPipelineDescriptor {
207            label: Some("post_process_pipeline".into()),
208            layout: vec![layout.clone()],
209            vertex: vertex_state,
210            fragment: Some(FragmentState {
211                shader,
212                // Make sure this matches the entry point of your shader.
213                // It can be anything as long as it matches here and in the shader.
214                targets: vec![Some(ColorTargetState {
215                    format: TextureFormat::Rgba8UnormSrgb,
216                    blend: None,
217                    write_mask: ColorWrites::ALL,
218                })],
219                ..default()
220            }),
221            ..default()
222        });
223    commands.insert_resource(PostProcessPipeline {
224        layout,
225        sampler,
226        pipeline_id,
227    });
228}
```