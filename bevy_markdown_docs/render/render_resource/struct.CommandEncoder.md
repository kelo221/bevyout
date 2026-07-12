[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Struct CommandEncoder 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#22)

```rust
pub struct CommandEncoder { /* private fields */ }
```

Encodes a series of GPU operations.

A command encoder can record [`RenderPass`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass")es, [`ComputePass`](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")es, and transfer operations between driver-managed resources like [`Buffer`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer")s and [`Texture`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")s.

When finished recording, call [`CommandEncoder::finish`](struct.CommandEncoder.html#method.finish "method bevy::render::render_resource::CommandEncoder::finish") to obtain a [`CommandBuffer`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/command_buffer/struct.CommandBuffer.html "struct wgpu::api::command_buffer::CommandBuffer") which may be submitted for execution.

Corresponds to [WebGPU `GPUCommandEncoder`](https://gpuweb.github.io/gpuweb/#command-encoder).

## Implementations

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#58)

### impl [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#60)

#### pub fn [finish](#method.finish)(self) -> [CommandBuffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/command_buffer/struct.CommandBuffer.html "struct wgpu::api::command_buffer::CommandBuffer")

Finishes recording and returns a [`CommandBuffer`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/command_buffer/struct.CommandBuffer.html "struct wgpu::api::command_buffer::CommandBuffer") that can be submitted for execution.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([line 370](../../../src/headless_renderer/headless_renderer.rs.html#370))

```rust
320fn image_copy_driver(
321    render_context: RenderContext,
322    image_copiers: Res<ImageCopiers>,
323    render_queue: Res<RenderQueue>,
324    gpu_images: Res<RenderAssets<bevy::render::texture::GpuImage>>,
325) {
326    for image_copier in image_copiers.iter() {
327        if !image_copier.enabled() {
328            continue;
329        }
330
331        let src_image = gpu_images.get(&image_copier.src_image).unwrap();
332
333        let mut encoder = render_context
334            .render_device()
335            .create_command_encoder(&CommandEncoderDescriptor::default());
336
337        let block_dimensions = src_image.texture_descriptor.format.block_dimensions();
338        let block_size = src_image
339            .texture_descriptor
340            .format
341            .block_copy_size(None)
342            .unwrap();
343
344        // Calculating correct size of image row because
345        // copy_texture_to_buffer can copy image only by rows aligned wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
346        // That's why image in buffer can be little bit wider
347        // This should be taken into account at copy from buffer stage
348        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(
349            (src_image.texture_descriptor.size.width as usize / block_dimensions.0 as usize)
350                * block_size as usize,
351        );
352
353        encoder.copy_texture_to_buffer(
354            src_image.texture.as_image_copy(),
355            TexelCopyBufferInfo {
356                buffer: &image_copier.buffer,
357                layout: TexelCopyBufferLayout {
358                    offset: 0,
359                    bytes_per_row: Some(
360                        std::num::NonZero::<u32>::new(padded_bytes_per_row as u32)
361                            .unwrap()
362                            .into(),
363                    ),
364                    rows_per_image: None,
365                },
366            },
367            src_image.texture_descriptor.size,
368        );
369
370        render_queue.submit(std::iter::once(encoder.finish()));
371    }
372}
```

Hide additional examples

examples/app/render\_recovery.rs ([line 210](../../../src/render_recovery/render_recovery.rs.html#210))

```rust
145fn cause_error(error: If<Res<RenderError>>, device: Res<RenderDevice>, queue: Res<RenderQueue>) {
146    match **error {
147        RenderError::None => {}
148        RenderError::OutOfMemory => {
149            let mut textures = Vec::new();
150            for _ in 0..64 {
151                textures.push(device.create_texture(&TextureDescriptor {
152                    label: None,
153                    size: Extent3d {
154                        width: 8192,
155                        height: 8192,
156                        depth_or_array_layers: 1,
157                    },
158                    mip_level_count: 1,
159                    sample_count: 1,
160                    dimension: TextureDimension::D2,
161                    format: TextureFormat::Rgba16Float,
162                    usage: TextureUsages::RENDER_ATTACHMENT,
163                    view_formats: &[],
164                }));
165            }
166        }
167        RenderError::Validation => {
168            device.create_buffer(&BufferDescriptor {
169                label: None,
170                size: 1 << 63,
171                usage: BufferUsages::COPY_SRC,
172                mapped_at_creation: false,
173            });
174        }
175        RenderError::DeviceLost => {
176            device.wgpu_device().destroy();
177            device.poll(PollType::wait_indefinitely()).unwrap();
178        }
179        RenderError::Loop => {
180            let sm = device.create_and_validate_shader_module(ShaderModuleDescriptor {
181                label: Some("shader"),
182                source: ShaderSource::Wgsl(
183                    "@compute @workgroup_size(1, 1, 1) fn main() { loop { workgroupBarrier(); } }"
184                        .into(),
185                ),
186            });
187
188            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
189                label: Some("pipeline_layout"),
190                bind_group_layouts: &[],
191                immediate_size: 0,
192            });
193
194            let pipeline = device.create_compute_pipeline(&RawComputePipelineDescriptor {
195                label: Some("pipeline"),
196                layout: Some(&pipeline_layout),
197                module: &sm,
198                entry_point: Some("main"),
199                compilation_options: Default::default(),
200                cache: None,
201            });
202
203            let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
204            {
205                let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
206                cpass.set_pipeline(&pipeline);
207                cpass.dispatch_workgroups(1, 1, 1);
208            }
209            device.poll(PollType::wait_indefinitely()).unwrap();
210            queue.submit([encoder.finish()]);
211        }
212    }
213}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#76-79)

#### pub fn [begin\_render\_pass](#method.begin_render_pass)<'encoder>( &'encoder mut self, desc: &[RenderPassDescriptor](struct.RenderPassDescriptor.html "struct bevy::render::render_resource::RenderPassDescriptor")<'\_>, ) -> [RenderPass](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass")<'encoder>

Begins recording of a render pass.

This function returns a [`RenderPass`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass") object which records a single render pass.

As long as the returned [`RenderPass`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass") has not ended, any mutating operation on this command encoder causes an error and invalidates it. Note that the `'encoder` lifetime relationship protects against this, but it is possible to opt out of it by calling [`RenderPass::forget_lifetime`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html#method.forget_lifetime "method wgpu::api::render_pass::RenderPass::forget_lifetime"). This can be useful for runtime handling of the encoder->pass dependency e.g. when pass and encoder are stored in the same data structure.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/custom\_post\_processing.rs ([lines 142-156](../../../src/custom_post_processing/custom_post_processing.rs.html#142-156))

```rust
75fn post_process_system(
76    view: ViewQuery<(
77        &ViewTarget,
78        &PostProcessSettings,
79        &DynamicUniformIndex<PostProcessSettings>,
80    )>,
81    post_process_pipeline: Option<Res<PostProcessPipeline>>,
82    pipeline_cache: Res<PipelineCache>,
83    settings_uniforms: Res<ComponentUniforms<PostProcessSettings>>,
84    mut cache: Local<PostProcessBindGroupCache>,
85    mut ctx: RenderContext,
86) {
87    let Some(post_process_pipeline) = post_process_pipeline else {
88        return;
89    };
90
91    let (view_target, _post_process_settings, settings_index) = view.into_inner();
92
93    let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
94    else {
95        return;
96    };
97
98    let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
99        return;
100    };
101
102    // This will start a new "post process write", obtaining two texture
103    // views from the view target - a `source` and a `destination`.
104    // `source` is the "current" main texture and you _must_ write into
105    // `destination` because calling `post_process_write()` on the
106    // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
107    // texture to the `destination` texture. Failing to do so will cause
108    // the current main texture information to be lost.
109    let post_process = view_target.post_process_write();
110
111    let bind_group = match &mut cache.cached {
112        Some((texture_id, bind_group)) if post_process.source.id() == *texture_id => bind_group,
113        cached => {
114            // The bind_group gets created each frame.
115            //
116            // Normally, you would create a bind_group in the Queue set,
117            // but this doesn't work with the post_process_write().
118            // The reason it doesn't work is because each post_process_write will alternate the source/destination.
119            // The only way to have the correct source/destination for the bind_group
120            // is to make sure you get it during the node execution.
121            let bind_group = ctx.render_device().create_bind_group(
122                "post_process_bind_group",
123                &pipeline_cache.get_bind_group_layout(&post_process_pipeline.layout),
124                // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
125                &BindGroupEntries::sequential((
126                    // Make sure to use the source view
127                    post_process.source,
128                    // Use the sampler created for the pipeline
129                    &post_process_pipeline.sampler,
130                    // Set the settings binding
131                    settings_binding.clone(),
132                )),
133            );
134
135            let (_, bind_group) = cached.insert((post_process.source.id(), bind_group));
136            bind_group
137        }
138    };
139
140    let mut render_pass = ctx
141        .command_encoder()
142        .begin_render_pass(&RenderPassDescriptor {
143            label: Some("post_process_pass"),
144            color_attachments: &[Some(RenderPassColorAttachment {
145                // We need to specify the post process destination view here
146                // to make sure we write to the appropriate texture.
147                view: post_process.destination,
148                depth_slice: None,
149                resolve_target: None,
150                ops: Operations::default(),
151            })],
152            depth_stencil_attachment: None,
153            timestamp_writes: None,
154            occlusion_query_set: None,
155            multiview_mask: None,
156        });
157
158    render_pass.set_pipeline(pipeline);
159    // By passing in the index of the post process settings on this view, we ensure
160    // that in the event that multiple settings were sent to the GPU (as would be the
161    // case with multiple cameras), we use the correct one.
162    render_pass.set_bind_group(0, bind_group, &[settings_index.index()]);
163    render_pass.draw(0..3, 0..1);
164}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#98-101)

#### pub fn [begin\_compute\_pass](#method.begin_compute_pass)<'encoder>( &'encoder mut self, desc: &[ComputePassDescriptor](struct.ComputePassDescriptor.html "struct bevy::render::render_resource::ComputePassDescriptor")<'\_>, ) -> [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

Begins recording of a compute pass.

This function returns a [`ComputePass`](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass") object which records a single compute pass.

As long as the returned [`ComputePass`](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass") has not ended, any mutating operation on this command encoder causes an error and invalidates it. Note that the `'encoder` lifetime relationship protects against this, but it is possible to opt out of it by calling [`ComputePass::forget_lifetime`](struct.ComputePass.html#method.forget_lifetime "method bevy::render::render_resource::ComputePass::forget_lifetime"). This can be useful for runtime handling of the encoder->pass dependency e.g. when pass and encoder are stored in the same data structure.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([lines 206-209](../../../src/gpu_readback/gpu_readback.rs.html#206-209))

```rust
196fn compute(
197    mut render_context: RenderContext,
198    pipeline_cache: Res<PipelineCache>,
199    pipeline: Res<ComputePipeline>,
200    bind_group: Res<GpuBufferBindGroup>,
201) {
202    if let Some(init_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
203        let mut pass =
204            render_context
205                .command_encoder()
206                .begin_compute_pass(&ComputePassDescriptor {
207                    label: Some("GPU readback compute pass"),
208                    ..default()
209                });
210
211        pass.set_bind_group(0, &bind_group.0, &[]);
212        pass.set_pipeline(init_pipeline);
213        pass.dispatch_workgroups(BUFFER_LEN as u32, 1, 1);
214    }
215}
```

Hide additional examples

examples/shader/compute\_shader\_game\_of\_life.rs ([line 265](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#265))

```rust
256fn game_of_life(
257    mut render_context: RenderContext,
258    bind_groups: Res<GameOfLifeImageBindGroups>,
259    pipeline_cache: Res<PipelineCache>,
260    pipeline: Res<GameOfLifePipeline>,
261    state: Res<GameOfLifeState>,
262) {
263    let mut pass = render_context
264        .command_encoder()
265        .begin_compute_pass(&ComputePassDescriptor::default());
266
267    // select the pipeline based on the current state
268    match *state {
269        GameOfLifeState::Loading => {}
270        GameOfLifeState::Init => {
271            let init_pipeline = pipeline_cache
272                .get_compute_pipeline(pipeline.init_pipeline)
273                .unwrap();
274            pass.set_bind_group(0, &bind_groups.0[0], &[]);
275            pass.set_pipeline(init_pipeline);
276            pass.dispatch_workgroups(SIZE.x / WORKGROUP_SIZE, SIZE.y / WORKGROUP_SIZE, 1);
277        }
278        GameOfLifeState::Update(index) => {
279            let update_pipeline = pipeline_cache
280                .get_compute_pipeline(pipeline.update_pipeline)
281                .unwrap();
282            pass.set_bind_group(0, &bind_groups.0[index], &[]);
283            pass.set_pipeline(update_pipeline);
284            pass.dispatch_workgroups(SIZE.x / WORKGROUP_SIZE, SIZE.y / WORKGROUP_SIZE, 1);
285        }
286    }
287}
```

examples/app/render\_recovery.rs ([line 205](../../../src/render_recovery/render_recovery.rs.html#205))

```rust
145fn cause_error(error: If<Res<RenderError>>, device: Res<RenderDevice>, queue: Res<RenderQueue>) {
146    match **error {
147        RenderError::None => {}
148        RenderError::OutOfMemory => {
149            let mut textures = Vec::new();
150            for _ in 0..64 {
151                textures.push(device.create_texture(&TextureDescriptor {
152                    label: None,
153                    size: Extent3d {
154                        width: 8192,
155                        height: 8192,
156                        depth_or_array_layers: 1,
157                    },
158                    mip_level_count: 1,
159                    sample_count: 1,
160                    dimension: TextureDimension::D2,
161                    format: TextureFormat::Rgba16Float,
162                    usage: TextureUsages::RENDER_ATTACHMENT,
163                    view_formats: &[],
164                }));
165            }
166        }
167        RenderError::Validation => {
168            device.create_buffer(&BufferDescriptor {
169                label: None,
170                size: 1 << 63,
171                usage: BufferUsages::COPY_SRC,
172                mapped_at_creation: false,
173            });
174        }
175        RenderError::DeviceLost => {
176            device.wgpu_device().destroy();
177            device.poll(PollType::wait_indefinitely()).unwrap();
178        }
179        RenderError::Loop => {
180            let sm = device.create_and_validate_shader_module(ShaderModuleDescriptor {
181                label: Some("shader"),
182                source: ShaderSource::Wgsl(
183                    "@compute @workgroup_size(1, 1, 1) fn main() { loop { workgroupBarrier(); } }"
184                        .into(),
185                ),
186            });
187
188            let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
189                label: Some("pipeline_layout"),
190                bind_group_layouts: &[],
191                immediate_size: 0,
192            });
193
194            let pipeline = device.create_compute_pipeline(&RawComputePipelineDescriptor {
195                label: Some("pipeline"),
196                layout: Some(&pipeline_layout),
197                module: &sm,
198                entry_point: Some("main"),
199                compilation_options: Default::default(),
200                cache: None,
201            });
202
203            let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
204            {
205                let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
206                cpass.set_pipeline(&pipeline);
207                cpass.dispatch_workgroups(1, 1, 1);
208            }
209            device.poll(PollType::wait_indefinitely()).unwrap();
210            queue.submit([encoder.finish()]);
211        }
212    }
213}
```

examples/shader\_advanced/compute\_mesh.rs ([lines 306-309](../../../src/compute_mesh/compute_mesh.rs.html#306-309))

```rust
253fn compute_mesh(
254    mut render_context: RenderContext,
255    chunks: Res<ChunksToProcess>,
256    mesh_allocator: Res<MeshAllocator>,
257    pipeline_cache: Res<PipelineCache>,
258    pipeline: Res<ComputePipeline>,
259    render_queue: Res<RenderQueue>,
260) {
261    let Some(init_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) else {
262        return;
263    };
264
265    for mesh_id in &chunks.0 {
266        info!(?mesh_id, "processing mesh");
267
268        // the mesh_allocator holds slabs of meshes, so the buffers we get here
269        // can contain more data than just the mesh we're asking for.
270        // That's why there is a range field.
271        // You should *not* touch data in these buffers that is outside of the range.
272        let vertex_buffer_slice = mesh_allocator.mesh_vertex_slice(mesh_id).unwrap();
273        let index_buffer_slice = mesh_allocator.mesh_index_slice(mesh_id).unwrap();
274
275        let first = DataRanges {
276            // there are 8 vertex data values (pos, normal, uv) per vertex
277            // and the vertex_buffer_slice.range.start is in "vertex elements"
278            // which includes all of that data, so each index is worth 8 indices
279            // to our shader code.
280            vertex_start: vertex_buffer_slice.range.start * 8,
281            vertex_end: vertex_buffer_slice.range.end * 8,
282            // but each vertex index is a single value, so the index of the
283            // vertex indices is exactly what the value is
284            index_start: index_buffer_slice.range.start,
285            index_end: index_buffer_slice.range.end,
286        };
287
288        let mut uniforms = UniformBuffer::from(first);
289        uniforms.write_buffer(render_context.render_device(), &render_queue);
290
291        // pass in the full mesh_allocator slabs as well as the first index
292        // offsets for the vertex and index buffers
293        let bind_group = render_context.render_device().create_bind_group(
294            None,
295            &pipeline_cache.get_bind_group_layout(&pipeline.layout),
296            &BindGroupEntries::sequential((
297                &uniforms,
298                vertex_buffer_slice.buffer.as_entire_buffer_binding(),
299                index_buffer_slice.buffer.as_entire_buffer_binding(),
300            )),
301        );
302
303        let mut pass =
304            render_context
305                .command_encoder()
306                .begin_compute_pass(&ComputePassDescriptor {
307                    label: Some("Mesh generation compute pass"),
308                    ..default()
309                });
310        pass.push_debug_group("compute_mesh");
311
312        pass.set_bind_group(0, &bind_group, &[]);
313        pass.set_pipeline(init_pipeline);
314        // we only dispatch 1,1,1 workgroup here, but a real compute shader
315        // would take advantage of more and larger size workgroups
316        pass.dispatch_workgroups(1, 1, 1);
317
318        pass.pop_debug_group();
319    }
320}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#117-124)

#### pub fn [copy\_buffer\_to\_buffer](#method.copy_buffer_to_buffer)( &mut self, source: &[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer"), source\_offset: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), destination: &[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer"), destination\_offset: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), copy\_size: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>, )

Copy data from one buffer to another.

##### Panics

*   Buffer offsets or copy size not a multiple of [`COPY_BUFFER_ALIGNMENT`](constant.COPY_BUFFER_ALIGNMENT.html "constant bevy::render::render_resource::COPY_BUFFER_ALIGNMENT").
*   Copy would overrun buffer.
*   Copy within the same buffer.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([lines 423-429](../../../src/occlusion_culling/occlusion_culling.rs.html#423-429))

```rust
388fn readback_indirect_parameters_node(
389    mut render_context: RenderContext,
390    indirect_parameters_buffers: Res<IndirectParametersBuffers>,
391    indirect_parameters_mapping_buffers: Res<IndirectParametersStagingBuffers>,
392) {
393    // Get the indirect parameters buffers corresponding to the opaque 3D
394    // phase, since all our meshes are in that phase.
395    let Some(phase_indirect_parameters_buffers) =
396        indirect_parameters_buffers.get(&TypeId::of::<Opaque3d>())
397    else {
398        return;
399    };
400
401    // Grab both the buffers we're copying from and the staging buffers
402    // we're copying to. Remember that we can't map the indirect parameters
403    // buffers directly, so we have to copy their contents to a staging
404    // buffer.
405    let (
406        Some(indexed_data_buffer),
407        Some(indexed_batch_sets_buffer),
408        Some(indirect_parameters_staging_data_buffer),
409        Some(indirect_parameters_staging_batch_sets_buffer),
410    ) = (
411        phase_indirect_parameters_buffers.indexed.data_buffer(),
412        phase_indirect_parameters_buffers
413            .indexed
414            .batch_sets_buffer(),
415        indirect_parameters_mapping_buffers.data.as_ref(),
416        indirect_parameters_mapping_buffers.batch_sets.as_ref(),
417    )
418    else {
419        return;
420    };
421
422    // Copy from the indirect parameters buffers to the staging buffers.
423    render_context.command_encoder().copy_buffer_to_buffer(
424        indexed_data_buffer,
425        0,
426        indirect_parameters_staging_data_buffer,
427        0,
428        indexed_data_buffer.size(),
429    );
430    render_context.command_encoder().copy_buffer_to_buffer(
431        indexed_batch_sets_buffer,
432        0,
433        indirect_parameters_staging_batch_sets_buffer,
434        0,
435        indexed_batch_sets_buffer.size(),
436    );
437}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#135-140)

#### pub fn [copy\_buffer\_to\_texture](#method.copy_buffer_to_texture)( &mut self, source: [TexelCopyBufferInfo](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TexelCopyBufferInfo.html "struct wgpu_types::texture::TexelCopyBufferInfo")<&[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer")\>, destination: [TexelCopyTextureInfo](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TexelCopyTextureInfo.html "struct wgpu_types::texture::TexelCopyTextureInfo")<&[Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")\>, copy\_size: [Extent3d](struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), )

Copy data from a buffer to a texture.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#146-151)

#### pub fn [copy\_texture\_to\_buffer](#method.copy_texture_to_buffer)( &mut self, source: [TexelCopyTextureInfo](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TexelCopyTextureInfo.html "struct wgpu_types::texture::TexelCopyTextureInfo")<&[Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")\>, destination: [TexelCopyBufferInfo](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TexelCopyBufferInfo.html "struct wgpu_types::texture::TexelCopyBufferInfo")<&[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer")\>, copy\_size: [Extent3d](struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), )

Copy data from a texture to a buffer.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([lines 353-368](../../../src/headless_renderer/headless_renderer.rs.html#353-368))

```rust
320fn image_copy_driver(
321    render_context: RenderContext,
322    image_copiers: Res<ImageCopiers>,
323    render_queue: Res<RenderQueue>,
324    gpu_images: Res<RenderAssets<bevy::render::texture::GpuImage>>,
325) {
326    for image_copier in image_copiers.iter() {
327        if !image_copier.enabled() {
328            continue;
329        }
330
331        let src_image = gpu_images.get(&image_copier.src_image).unwrap();
332
333        let mut encoder = render_context
334            .render_device()
335            .create_command_encoder(&CommandEncoderDescriptor::default());
336
337        let block_dimensions = src_image.texture_descriptor.format.block_dimensions();
338        let block_size = src_image
339            .texture_descriptor
340            .format
341            .block_copy_size(None)
342            .unwrap();
343
344        // Calculating correct size of image row because
345        // copy_texture_to_buffer can copy image only by rows aligned wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
346        // That's why image in buffer can be little bit wider
347        // This should be taken into account at copy from buffer stage
348        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(
349            (src_image.texture_descriptor.size.width as usize / block_dimensions.0 as usize)
350                * block_size as usize,
351        );
352
353        encoder.copy_texture_to_buffer(
354            src_image.texture.as_image_copy(),
355            TexelCopyBufferInfo {
356                buffer: &image_copier.buffer,
357                layout: TexelCopyBufferLayout {
358                    offset: 0,
359                    bytes_per_row: Some(
360                        std::num::NonZero::<u32>::new(padded_bytes_per_row as u32)
361                            .unwrap()
362                            .into(),
363                    ),
364                    rows_per_image: None,
365                },
366            },
367            src_image.texture_descriptor.size,
368        );
369
370        render_queue.submit(std::iter::once(encoder.finish()));
371    }
372}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#163-168)

#### pub fn [copy\_texture\_to\_texture](#method.copy_texture_to_texture)( &mut self, source: [TexelCopyTextureInfo](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TexelCopyTextureInfo.html "struct wgpu_types::texture::TexelCopyTextureInfo")<&[Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")\>, destination: [TexelCopyTextureInfo](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TexelCopyTextureInfo.html "struct wgpu_types::texture::TexelCopyTextureInfo")<&[Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")\>, copy\_size: [Extent3d](struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), )

Copy data from one texture to another.

##### Panics

*   Textures are not the same type
*   If a depth texture, or a multisampled texture, the entire texture must be copied
*   Copy would overrun either texture

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/render\_depth\_to\_texture.rs ([lines 153-171](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#153-171))

```rust
127fn copy_depth_texture_system(
128    view: ViewQuery<(&ExtractedCamera, &ViewDepthTexture)>,
129    demo_depth_texture: Option<Res<DemoDepthTexture>>,
130    image_assets: Res<RenderAssets<GpuImage>>,
131    mut ctx: RenderContext,
132) {
133    let Some(demo_depth_texture) = demo_depth_texture else {
134        return;
135    };
136
137    let (camera, depth_texture) = view.into_inner();
138
139    // Make sure we only run on the depth-only camera.
140    // We could make a marker component for that camera and extract it to
141    // the render world, but using `order` as a tag to tell the main camera
142    // and the depth-only camera apart works in a pinch.
143    if camera.order >= 0 {
144        return;
145    }
146
147    let Some(demo_depth_image) = image_assets.get(demo_depth_texture.0.id()) else {
148        return;
149    };
150
151    let command_encoder = ctx.command_encoder();
152    command_encoder.push_debug_group("copy depth to demo texture");
153    command_encoder.copy_texture_to_texture(
154        TexelCopyTextureInfo {
155            texture: &depth_texture.texture,
156            mip_level: 0,
157            origin: Origin3d::default(),
158            aspect: TextureAspect::DepthOnly,
159        },
160        TexelCopyTextureInfo {
161            texture: &demo_depth_image.texture,
162            mip_level: 0,
163            origin: Origin3d::default(),
164            aspect: TextureAspect::DepthOnly,
165        },
166        Extent3d {
167            width: DEPTH_TEXTURE_SIZE,
168            height: DEPTH_TEXTURE_SIZE,
169            depth_or_array_layers: 1,
170        },
171    );
172    command_encoder.pop_debug_group();
173}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#186)

#### pub fn [clear\_texture](#method.clear_texture)( &mut self, texture: &[Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture"), subresource\_range: &[ImageSubresourceRange](struct.ImageSubresourceRange.html "struct bevy::render::render_resource::ImageSubresourceRange"), )

Clears texture to zero.

Note that unlike with clear\_buffer, `COPY_DST` usage is not required.

##### Implementation notes

*   implemented either via buffer copies and render/depth target clear, path depends on texture usages
*   behaves like texture zero init, but is performed immediately (clearing is _not_ delayed via marking it as uninitialized)

##### Panics

*   `CLEAR_TEXTURE` extension not enabled
*   Range is out of bounds

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#196-201)

#### pub fn [clear\_buffer](#method.clear_buffer)(&mut self, buffer: &[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer"), offset: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), size: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>)

Clears buffer to zero.

##### Panics

*   Buffer does not have `COPY_DST` usage.
*   Range is out of bounds

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#206)

#### pub fn [insert\_debug\_marker](#method.insert_debug_marker)(&mut self, label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Inserts debug marker.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#211)

#### pub fn [push\_debug\_group](#method.push_debug_group)(&mut self, label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Start record commands and group it into debug marker group.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 152](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#152))

```rust
127fn copy_depth_texture_system(
128    view: ViewQuery<(&ExtractedCamera, &ViewDepthTexture)>,
129    demo_depth_texture: Option<Res<DemoDepthTexture>>,
130    image_assets: Res<RenderAssets<GpuImage>>,
131    mut ctx: RenderContext,
132) {
133    let Some(demo_depth_texture) = demo_depth_texture else {
134        return;
135    };
136
137    let (camera, depth_texture) = view.into_inner();
138
139    // Make sure we only run on the depth-only camera.
140    // We could make a marker component for that camera and extract it to
141    // the render world, but using `order` as a tag to tell the main camera
142    // and the depth-only camera apart works in a pinch.
143    if camera.order >= 0 {
144        return;
145    }
146
147    let Some(demo_depth_image) = image_assets.get(demo_depth_texture.0.id()) else {
148        return;
149    };
150
151    let command_encoder = ctx.command_encoder();
152    command_encoder.push_debug_group("copy depth to demo texture");
153    command_encoder.copy_texture_to_texture(
154        TexelCopyTextureInfo {
155            texture: &depth_texture.texture,
156            mip_level: 0,
157            origin: Origin3d::default(),
158            aspect: TextureAspect::DepthOnly,
159        },
160        TexelCopyTextureInfo {
161            texture: &demo_depth_image.texture,
162            mip_level: 0,
163            origin: Origin3d::default(),
164            aspect: TextureAspect::DepthOnly,
165        },
166        Extent3d {
167            width: DEPTH_TEXTURE_SIZE,
168            height: DEPTH_TEXTURE_SIZE,
169            depth_or_array_layers: 1,
170        },
171    );
172    command_encoder.pop_debug_group();
173}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#216)

#### pub fn [pop\_debug\_group](#method.pop_debug_group)(&mut self)

Stops command recording and creates debug group.

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 172](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#172))

```rust
127fn copy_depth_texture_system(
128    view: ViewQuery<(&ExtractedCamera, &ViewDepthTexture)>,
129    demo_depth_texture: Option<Res<DemoDepthTexture>>,
130    image_assets: Res<RenderAssets<GpuImage>>,
131    mut ctx: RenderContext,
132) {
133    let Some(demo_depth_texture) = demo_depth_texture else {
134        return;
135    };
136
137    let (camera, depth_texture) = view.into_inner();
138
139    // Make sure we only run on the depth-only camera.
140    // We could make a marker component for that camera and extract it to
141    // the render world, but using `order` as a tag to tell the main camera
142    // and the depth-only camera apart works in a pinch.
143    if camera.order >= 0 {
144        return;
145    }
146
147    let Some(demo_depth_image) = image_assets.get(demo_depth_texture.0.id()) else {
148        return;
149    };
150
151    let command_encoder = ctx.command_encoder();
152    command_encoder.push_debug_group("copy depth to demo texture");
153    command_encoder.copy_texture_to_texture(
154        TexelCopyTextureInfo {
155            texture: &depth_texture.texture,
156            mip_level: 0,
157            origin: Origin3d::default(),
158            aspect: TextureAspect::DepthOnly,
159        },
160        TexelCopyTextureInfo {
161            texture: &demo_depth_image.texture,
162            mip_level: 0,
163            origin: Origin3d::default(),
164            aspect: TextureAspect::DepthOnly,
165        },
166        Extent3d {
167            width: DEPTH_TEXTURE_SIZE,
168            height: DEPTH_TEXTURE_SIZE,
169            depth_or_array_layers: 1,
170        },
171    );
172    command_encoder.pop_debug_group();
173}
```

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#233-239)

#### pub fn [resolve\_query\_set](#method.resolve_query_set)( &mut self, query\_set: &[QuerySet](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet"), query\_range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>, destination: &[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer"), destination\_offset: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), )

Copies query results stored in `query_set` into `destination` so that they can be read by compute shaders or buffer operations.

*   `query_range` is the range of query result indices to copy from `query_set`. Occlusion and timestamp queries occupy 1 result index each; for pipeline statistics queries, see [`PipelineStatisticsTypes`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/struct.PipelineStatisticsTypes.html "struct wgpu_types::PipelineStatisticsTypes").
*   `destination_offset` is the offset within `destination` to start writing at. It must be a multiple of [`QUERY_RESOLVE_BUFFER_ALIGNMENT`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/constant.QUERY_RESOLVE_BUFFER_ALIGNMENT.html "constant wgpu_types::QUERY_RESOLVE_BUFFER_ALIGNMENT").

The length of the data written to `destination` will be 8 bytes ([`QUERY_SIZE`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/constant.QUERY_SIZE.html "constant wgpu_types::QUERY_SIZE")) times the number of elements in `query_range`.

For further information about using queries, see [`QuerySet`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet").

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#249)

#### pub fn [map\_buffer\_on\_submit](#method.map_buffer_on_submit)<S>( &self, buffer: &[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer"), mode: [MapMode](enum.MapMode.html "enum bevy::render::render_resource::MapMode"), bounds: S, callback: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BufferAsyncError](struct.BufferAsyncError.html "struct bevy::render::render_resource::BufferAsyncError")\>) + [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + 'static, )

where S: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>,

On submission, maps the buffer to host (CPU) memory, making it available for reading or writing via [`get_mapped_range()`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html#method.get_mapped_range "method wgpu::api::buffer::Buffer::get_mapped_range"). The buffer becomes accessible once the `callback` is invoked with [`Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok").

Use this when you need to submit work that uses the buffer before mapping it. Because that submission must happen before calling `map_async`, this method schedules the mapping for after submission, avoiding extra calls to [`Buffer::map_async()`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html#method.map_async "method wgpu::api::buffer::Buffer::map_async") or [`BufferSlice::map_async()`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html#method.map_async "method wgpu::api::buffer::BufferSlice::map_async") and letting you start the mapping from a more convenient place.

For the callback to run, either [`queue.submit(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.Queue.html#method.submit "method wgpu::api::queue::Queue::submit"), [`instance.poll_all(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/instance/struct.Instance.html#method.poll_all "method wgpu::api::instance::Instance::poll_all"), or [`device.poll(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.poll "method wgpu::api::device::Device::poll") must be called elsewhere in the runtime, possibly integrated into an event loop or run on a separate thread.

The callback runs on the thread that first calls one of the above functions after the GPU work completes. There are no restrictions on the code you can run in the callback; however, on native the polling call will not return until the callback finishes, so keep callbacks short (set flags, send messages, etc.).

While a buffer is mapped, it cannot be used by other commands; at any time, either the GPU or the CPU has exclusive access to the buffer’s contents.

##### Panics

*   If `bounds` is outside the bounds of `buffer`.
*   If `bounds` has a length less than 1.

##### Panics During Submit

*   If the buffer is already mapped.
*   If the buffer’s [`BufferUsages`](struct.BufferUsages.html "struct bevy::render::render_resource::BufferUsages") do not allow the requested [`MapMode`](enum.MapMode.html "enum bevy::render::render_resource::MapMode").
*   If `bounds` is outside of the bounds of `buffer`.
*   If `bounds` does not start at a multiple of [`MAP_ALIGNMENT`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/constant.MAP_ALIGNMENT.html "constant wgpu_types::MAP_ALIGNMENT").
*   If `bounds` has a length that is not a multiple of 4 greater than 0.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#249)

#### pub fn [on\_submitted\_work\_done](#method.on_submitted_work_done)(&self, callback: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static)

Registers a callback that is invoked when this command buffer’s work finishes executing on the GPU. When this callback runs, all mapped-buffer callbacks registered for the same submission are guaranteed to have been called.

For the callback to run, either [`queue.submit(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.Queue.html#method.submit "method wgpu::api::queue::Queue::submit"), [`instance.poll_all(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/instance/struct.Instance.html#method.poll_all "method wgpu::api::instance::Instance::poll_all"), or [`device.poll(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.poll "method wgpu::api::device::Device::poll") must be called elsewhere in the runtime, possibly integrated into an event loop or run on a separate thread.

The callback runs on the thread that first calls one of the above functions after the GPU work completes. There are no restrictions on the code you can run in the callback; however, on native the polling call will not return until the callback finishes, so keep callbacks short (set flags, send messages, etc.).

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#277-280)

#### pub unsafe fn [as\_hal\_mut](#method.as_hal_mut)<A, F, R>( &mut self, hal\_command\_encoder\_callback: F, ) -> R

where A: [Api](https://docs.rs/wgpu-hal/29.0.3/x86_64-unknown-linux-gnu/wgpu_hal/trait.Api.html "trait wgpu_hal::Api"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut <A as [Api](https://docs.rs/wgpu-hal/29.0.3/x86_64-unknown-linux-gnu/wgpu_hal/trait.Api.html "trait wgpu_hal::Api")\>::[CommandEncoder](https://docs.rs/wgpu-hal/29.0.3/x86_64-unknown-linux-gnu/wgpu_hal/trait.Api.html#associatedtype.CommandEncoder "type wgpu_hal::Api::CommandEncoder")\>) -> R,

Available on **`wgpu_core`** only.

Get the [`wgpu_hal`](https://docs.rs/wgpu-hal/29.0.3/x86_64-unknown-linux-gnu/wgpu_hal/index.html "mod wgpu_hal") command encoder from this `CommandEncoder`.

The returned command encoder will be ready to record onto.

##### Errors

This method will pass in [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if:

*   The encoder is not from the backend specified by `A`.
*   The encoder is from the `webgpu` or `custom` backend.

##### Types

The callback argument depends on the backend:

*   [`hal::api::Vulkan`](https://docs.rs/wgpu-hal/29.0.3/x86_64-unknown-linux-gnu/wgpu_hal/vulkan/struct.Api.html "struct wgpu_hal::vulkan::Api") uses [`hal::vulkan::CommandEncoder`](https://docs.rs/wgpu-hal/29.0.3/x86_64-unknown-linux-gnu/wgpu_hal/vulkan/struct.CommandEncoder.html "struct wgpu_hal::vulkan::CommandEncoder")
*   `hal::api::Metal` uses `hal::metal::CommandEncoder`
*   `hal::api::Dx12` uses `hal::dx12::CommandEncoder`
*   `hal::api::Gles` uses `hal::gles::CommandEncoder`

##### Safety

*   The raw handle obtained from the `A::CommandEncoder` must not be manually destroyed.
*   You must not end the command buffer; wgpu will do it when you call finish.
*   The wgpu command encoder must not be interacted with in any way while recording is happening to the wgpu\_hal or backend command encoder.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#300)

### impl [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[`Features::TIMESTAMP_QUERY_INSIDE_ENCODERS`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY_INSIDE_ENCODERS "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY_INSIDE_ENCODERS") must be enabled on the device in order to call these functions.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#313)

#### pub fn [write\_timestamp](#method.write_timestamp)(&mut self, query\_set: &[QuerySet](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet"), query\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Issue a timestamp command at this point in the queue. The timestamp will be written to the specified query set, at the specified index.

Must be multiplied by [`Queue::get_timestamp_period`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.Queue.html#method.get_timestamp_period "method wgpu::api::queue::Queue::get_timestamp_period") to get the value in nanoseconds. Absolute values have no meaning, but timestamps can be subtracted to get the time it takes for a string of operations to complete.

Attention: Since commands within a command recorder may be reordered, there is no strict guarantee that timestamps are taken after all commands recorded so far and all before all commands recorded after. This may depend both on the backend and the driver.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#319)

### impl [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[`Features::EXPERIMENTAL_RAY_QUERY`](struct.WgpuFeatures.html#associatedconstant.EXPERIMENTAL_RAY_QUERY "associated constant bevy::render::render_resource::WgpuFeatures::EXPERIMENTAL_RAY_QUERY") must be enabled on the device in order to call these functions.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#336-340)

#### pub unsafe fn [mark\_acceleration\_structures\_built](#method.mark_acceleration_structures_built)<'a>( &self, blas: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [Blas](struct.Blas.html "struct bevy::render::render_resource::Blas")\>, tlas: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [Tlas](struct.Tlas.html "struct bevy::render::render_resource::Tlas")\>, )

When encoding the acceleration structure build with the raw Hal encoder (obtained from [`CommandEncoder::as_hal_mut`](struct.CommandEncoder.html#method.as_hal_mut "method bevy::render::render_resource::CommandEncoder::as_hal_mut")), this function marks the acceleration structures as having been built.

This function must only be used with the raw encoder API. When using the wgpu encoding API, acceleration structure build is tracked automatically.

##### Panics

*   If the encoder is being used with the wgpu encoding API.

##### Safety

*   All acceleration structures must have been build in this command encoder.
*   All BLASes inputted must have been built before all TLASes that were inputted here and which use them.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#375-379)

#### pub fn [build\_acceleration\_structures](#method.build_acceleration_structures)<'a>( &mut self, blas: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [BlasBuildEntry](struct.BlasBuildEntry.html "struct bevy::render::render_resource::BlasBuildEntry")<'a>>, tlas: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = &'a [Tlas](struct.Tlas.html "struct bevy::render::render_resource::Tlas")\>, )

Build bottom and top level acceleration structures.

Builds the BLASes then the TLASes, but does _**not**_ build the BLASes into the TLASes, that must be done by setting a TLAS instance in the TLAS package to one that contains the BLAS (and with an appropriate transform)

##### Validation

*   blas: Iterator of bottom level acceleration structure entries to build. For each entry, the provided size descriptor must be strictly smaller or equal to the descriptor given at BLAS creation, this means:
    *   Less or equal number of geometries
    *   Same kind of geometry (with index buffer or without) (same vertex/index format)
    *   Same flags
    *   Less or equal number of vertices
    *   Less or equal number of indices (if applicable)
*   tlas: iterator of top level acceleration structure packages to build For each entry:
    *   Each BLAS in each TLAS instance must have been being built in the current call or in a previous call to `build_acceleration_structures` or `build_acceleration_structures_unsafe_tlas`
    *   The number of TLAS instances must be less than or equal to the max number of tlas instances when creating (if creating a package with `TlasPackage::new()` this is already satisfied)

If the device the command encoder is created from does not have [Features::EXPERIMENTAL\_RAY\_QUERY](struct.WgpuFeatures.html#associatedconstant.EXPERIMENTAL_RAY_QUERY "associated constant bevy::render::render_resource::WgpuFeatures::EXPERIMENTAL_RAY_QUERY") enabled then a validation error is generated

A bottom level acceleration structure may be build and used as a reference in a top level acceleration structure in the same invocation of this function.

##### Bind group usage

When a top level acceleration structure is used in a bind group, some validation takes place:

*   The top level acceleration structure is valid and has been built.
*   All the bottom level acceleration structures referenced by the top level acceleration structure are valid and have been built prior, or at same time as the containing top level acceleration structure.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#431-435)

#### pub fn [transition\_resources](#method.transition_resources)<'a>( &mut self, buffer\_transitions: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BufferTransition](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/buffer/struct.BufferTransition.html "struct wgpu_types::buffer::BufferTransition")<&'a [Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer")\>>, texture\_transitions: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [TextureTransition](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureTransition.html "struct wgpu_types::texture::TextureTransition")<&'a [Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")\>>, )

Transition resources to an underlying hal resource state.

This is an advanced, native-only API (no-op on web) that has two main use cases:

##### Batching Barriers

Wgpu does not have a global view of the frame when recording command buffers. When you submit multiple command buffers in a single queue submission, wgpu may need to record and insert new command buffers (holding 1 or more barrier commands) in between the user-supplied command buffers in order to ensure that resources are transitioned to the correct state for the start of the next user-supplied command buffer.

Wgpu does not currently attempt to batch multiple of these generated command buffers/barriers together, which may lead to suboptimal barrier placement.

Consider the following scenario, where the user does `queue.submit(&[a, b, c])`:

*   CommandBuffer A: Use resource X as a render pass attachment
*   CommandBuffer B: Use resource Y as a render pass attachment
*   CommandBuffer C: Use resources X and Y in a bind group

At submission time, wgpu will record and insert some new command buffers, resulting in a submission that looks like `queue.submit(&[0, a, 1, b, 2, c])`:

*   CommandBuffer 0: Barrier to transition resource X from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR\_TARGET
*   CommandBuffer A: Use resource X as a render pass attachment
*   CommandBuffer 1: Barrier to transition resource Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR\_TARGET
*   CommandBuffer B: Use resource Y as a render pass attachment
*   CommandBuffer 2: Barrier to transition resources X and Y from TextureUses::COLOR\_TARGET to TextureUses::RESOURCE
*   CommandBuffer C: Use resources X and Y in a bind group

To prevent this, after profiling their app, an advanced user might choose to instead do `queue.submit(&[a, b, c])`:

*   CommandBuffer A:
    *   Use [`CommandEncoder::transition_resources`](struct.CommandEncoder.html#method.transition_resources "method bevy::render::render_resource::CommandEncoder::transition_resources") to transition resources X and Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR\_TARGET
    *   Use resource X as a render pass attachment
*   CommandBuffer B: Use resource Y as a render pass attachment
*   CommandBuffer C:
    *   Use [`CommandEncoder::transition_resources`](struct.CommandEncoder.html#method.transition_resources "method bevy::render::render_resource::CommandEncoder::transition_resources") to transition resources X and Y from TextureUses::COLOR\_TARGET to TextureUses::RESOURCE
    *   Use resources X and Y in a bind group

At submission time, wgpu will record and insert some new command buffers, resulting in a submission that looks like `queue.submit(&[0, a, b, 1, c])`:

*   CommandBuffer 0: Barrier to transition resources X and Y from TextureUses::RESOURCE (from last frame) to TextureUses::COLOR\_TARGET
*   CommandBuffer A: Use resource X as a render pass attachment
*   CommandBuffer B: Use resource Y as a render pass attachment
*   CommandBuffer 1: Barrier to transition resources X and Y from TextureUses::COLOR\_TARGET to TextureUses::RESOURCE
*   CommandBuffer C: Use resources X and Y in a bind group

Which eliminates the extra command buffer and barrier between command buffers A and B.

##### Native Interoperability

A user wanting to interoperate with the underlying native graphics APIs (Vulkan, DirectX12, Metal, etc) can use this API to generate barriers between wgpu commands and the native API commands, for synchronization and resource state transition purposes.

## Trait Implementations

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#21)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#21)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

### impl [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1034-1036)

#### fn [max](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1073-1075)

#### fn [min](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)(self, other: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1099-1101)

#### fn [clamp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)(self, min: Self, max: Self) -> Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#29)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1410)

#### fn [lt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1428)

#### fn [le](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1446)

#### fn [gt](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1464)

#### fn [ge](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

## Auto Trait Implementations

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#104-107)

### impl<Q, K> [Comparable](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html "trait equivalent::Comparable")<K> for Q

where Q: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#110)

#### fn [compare](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/equivalent/trait.Comparable.html#tymethod.compare)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

Compare self to `key` and return their ordering.

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

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

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

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}