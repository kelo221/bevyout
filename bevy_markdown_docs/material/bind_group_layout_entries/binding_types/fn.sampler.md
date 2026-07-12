[bevy](../../../index.html)::[material](../../index.html)::[bind\_group\_layout\_entries](../index.html)::[binding\_types](index.html)

# Function sampler 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#539)

```rust
pub fn sampler(
    sampler_binding_type: SamplerBindingType,
) -> BindGroupLayoutEntryBuilder
```

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/shader\_advanced/manual\_material.rs ([line 90](../../../../src/manual_material/manual_material.rs.html#90))

```rust
79fn init_image_material_resources(
80    mut commands: Commands,
81    render_device: Res<RenderDevice>,
82    mut bind_group_allocators: ResMut<MaterialBindGroupAllocators>,
83) {
84    let bind_group_layout = BindGroupLayoutDescriptor::new(
85        "image_material_layout",
86        &BindGroupLayoutEntries::sequential(
87            ShaderStages::FRAGMENT,
88            (
89                texture_2d(TextureSampleType::Float { filterable: false }),
90                sampler(SamplerBindingType::NonFiltering),
91            ),
92        ),
93    );
94    let sampler = render_device.create_sampler(&SamplerDescriptor::default());
95    commands.insert_resource(ImageMaterialBindGroupLayout(bind_group_layout.clone()));
96    commands.insert_resource(ImageMaterialBindGroupSampler(sampler));
97
98    bind_group_allocators.insert(
99        TypeId::of::<ImageMaterial>(),
100        MaterialBindGroupAllocator::new(
101            &render_device,
102            "image_material_allocator",
103            None,
104            bind_group_layout,
105            None,
106        ),
107    );
108}
```

Hide additional examples

examples/shader\_advanced/texture\_binding\_array.rs ([line 189](../../../../src/texture_binding_array/texture_binding_array.rs.html#189))

```rust
159    fn bind_group_layout_entries(_: &RenderDevice, _: bool) -> Vec<BindGroupLayoutEntry>
160    where
161        Self: Sized,
162    {
163        BindGroupLayoutEntries::with_indices(
164            // The layout entries will only be visible in the fragment stage
165            ShaderStages::FRAGMENT,
166            (
167                // Screen texture
168                //
169                // @group(#{MATERIAL_BIND_GROUP}) @binding(0) var textures: binding_array<texture_2d<f32>>;
170                (
171                    0,
172                    texture_2d(TextureSampleType::Float { filterable: true })
173                        .count(NonZero::<u32>::new(MAX_TEXTURE_COUNT as u32).unwrap()),
174                ),
175                // Sampler
176                //
177                // @group(#{MATERIAL_BIND_GROUP}) @binding(1) var nearest_sampler: sampler;
178                //
179                // Note: as with textures, multiple samplers can also be bound
180                // onto one binding slot:
181                //
182                // ```
183                // sampler(SamplerBindingType::Filtering)
184                //     .count(NonZero::<u32>::new(MAX_TEXTURE_COUNT as u32).unwrap()),
185                // ```
186                //
187                // One may need to pay attention to the limit of sampler binding
188                // amount on some platforms.
189                (1, sampler(SamplerBindingType::Filtering)),
190            ),
191        )
192        .to_vec()
193    }
```

examples/shader\_advanced/custom\_post\_processing.rs ([line 191](../../../../src/custom_post_processing/custom_post_processing.rs.html#191))

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