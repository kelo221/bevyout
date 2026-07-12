[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Struct ComputePass 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#13)

```rust
pub struct ComputePass<'encoder> { /* private fields */ }
```

In-progress recording of a compute pass.

It can be created with [`CommandEncoder::begin_compute_pass`](struct.CommandEncoder.html#method.begin_compute_pass "method bevy::render::render_resource::CommandEncoder::begin_compute_pass").

Corresponds to [WebGPU `GPUComputePassEncoder`](https://gpuweb.github.io/gpuweb/#compute-pass-encoder).

## Implementations

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#30)

### impl [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#43)

#### pub fn [forget\_lifetime](#method.forget_lifetime)(self) -> [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'static>

Drops the lifetime relationship to the parent command encoder, making usage of the encoder while this pass is recorded a run-time error instead.

Attention: As long as the compute pass has not been ended, any mutating operation on the parent command encoder will cause a run-time error and invalidate it! By default, the lifetime constraint prevents this, but it can be useful to handle this at run time, such as when storing the pass and encoder in the same data structure.

This operation has no effect on pass recording. It’s a safe operation, since [`CommandEncoder`](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder") is in a locked state as long as the pass is active regardless of the lifetime constraint or its absence.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#57-59)

#### pub fn [set\_bind\_group](#method.set_bind_group)<'a, BG>( &mut self, index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), bind\_group: BG, offsets: &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\], )

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [BindGroup](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/bind_group/struct.BindGroup.html "struct wgpu::api::bind_group::BindGroup")\>: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<BG>,

Sets the active bind group for a given bind group index. The bind group layout in the active pipeline when the `dispatch()` function is called must match the layout of this bind group.

If the bind group have dynamic offsets, provide them in the binding order. These offsets have to be aligned to [`Limits::min_uniform_buffer_offset_alignment`](struct.WgpuLimits.html#structfield.min_uniform_buffer_offset_alignment "field bevy::render::render_resource::WgpuLimits::min_uniform_buffer_offset_alignment") or [`Limits::min_storage_buffer_offset_alignment`](struct.WgpuLimits.html#structfield.min_storage_buffer_offset_alignment "field bevy::render::render_resource::WgpuLimits::min_storage_buffer_offset_alignment") appropriately.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([line 211](../../../src/gpu_readback/gpu_readback.rs.html#211))

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

examples/shader/compute\_shader\_game\_of\_life.rs ([line 274](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#274))

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

examples/shader\_advanced/compute\_mesh.rs ([line 312](../../../src/compute_mesh/compute_mesh.rs.html#312))

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#67)

#### pub fn [set\_pipeline](#method.set_pipeline)(&mut self, pipeline: &[ComputePipeline](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/compute_pipeline/struct.ComputePipeline.html "struct wgpu::api::compute_pipeline::ComputePipeline"))

Sets the active compute pipeline.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([line 212](../../../src/gpu_readback/gpu_readback.rs.html#212))

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

examples/shader/compute\_shader\_game\_of\_life.rs ([line 275](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#275))

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

examples/app/render\_recovery.rs ([line 206](../../../src/render_recovery/render_recovery.rs.html#206))

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

examples/shader\_advanced/compute\_mesh.rs ([line 313](../../../src/compute_mesh/compute_mesh.rs.html#313))

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#72)

#### pub fn [insert\_debug\_marker](#method.insert_debug_marker)(&mut self, label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Inserts debug marker.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#77)

#### pub fn [push\_debug\_group](#method.push_debug_group)(&mut self, label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Start record commands and group it into debug marker group.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/compute\_mesh.rs ([line 310](../../../src/compute_mesh/compute_mesh.rs.html#310))

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#82)

#### pub fn [pop\_debug\_group](#method.pop_debug_group)(&mut self)

Stops command recording and creates debug group.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/compute\_mesh.rs ([line 318](../../../src/compute_mesh/compute_mesh.rs.html#318))

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#89)

#### pub fn [dispatch\_workgroups](#method.dispatch_workgroups)(&mut self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Dispatches compute work operations.

`x`, `y` and `z` denote the number of work groups to dispatch in each dimension.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([line 213](../../../src/gpu_readback/gpu_readback.rs.html#213))

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

examples/shader/compute\_shader\_game\_of\_life.rs ([line 276](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#276))

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

examples/app/render\_recovery.rs ([line 207](../../../src/render_recovery/render_recovery.rs.html#207))

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

examples/shader\_advanced/compute\_mesh.rs ([line 316](../../../src/compute_mesh/compute_mesh.rs.html#316))

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#96-100)

#### pub fn [dispatch\_workgroups\_indirect](#method.dispatch_workgroups_indirect)( &mut self, indirect\_buffer: &[Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer"), indirect\_offset: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), )

Dispatches compute work operations, based on the contents of the `indirect_buffer`.

The structure expected in `indirect_buffer` must conform to [`DispatchIndirectArgs`](struct.DispatchIndirectArgs.html "struct bevy::render::render_resource::DispatchIndirectArgs").

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#105)

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#105)

#### pub fn [on\_submitted\_work\_done](#method.on_submitted_work_done)(&self, callback: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static)

Registers a callback that is invoked when this command buffer’s work finishes executing on the GPU. When this callback runs, all mapped-buffer callbacks registered for the same submission are guaranteed to have been called.

For the callback to run, either [`queue.submit(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.Queue.html#method.submit "method wgpu::api::queue::Queue::submit"), [`instance.poll_all(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/instance/struct.Instance.html#method.poll_all "method wgpu::api::instance::Instance::poll_all"), or [`device.poll(..)`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.poll "method wgpu::api::device::Device::poll") must be called elsewhere in the runtime, possibly integrated into an event loop or run on a separate thread.

The callback runs on the thread that first calls one of the above functions after the GPU work completes. There are no restrictions on the code you can run in the callback; however, on native the polling call will not return until the callback finishes, so keep callbacks short (set flags, send messages, etc.).

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#115)

### impl [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[`Features::IMMEDIATES`](struct.WgpuFeatures.html#associatedconstant.IMMEDIATES "associated constant bevy::render::render_resource::WgpuFeatures::IMMEDIATES") must be enabled on the device in order to call these functions.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#124)

#### pub fn [set\_immediates](#method.set_immediates)(&mut self, offset: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), data: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Set immediate data for subsequent dispatch calls.

Write the bytes in `data` at offset `offset` within immediate data storage. Both `offset` and the length of `data` must be multiples of [`crate::IMMEDIATE_DATA_ALIGNMENT`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/constant.IMMEDIATE_DATA_ALIGNMENT.html "constant wgpu_types::IMMEDIATE_DATA_ALIGNMENT"), which is always 4.

For example, if `offset` is `4` and `data` is eight bytes long, this call will write `data` to bytes `4..12` of immediate data storage.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#130)

### impl [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[`Features::TIMESTAMP_QUERY_INSIDE_PASSES`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY_INSIDE_PASSES "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY_INSIDE_PASSES") must be enabled on the device in order to call these functions.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#137)

#### pub fn [write\_timestamp](#method.write_timestamp)(&mut self, query\_set: &[QuerySet](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet"), query\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Issue a timestamp command at this point in the queue. The timestamp will be written to the specified query set, at the specified index.

Must be multiplied by [`Queue::get_timestamp_period`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.Queue.html#method.get_timestamp_period "method wgpu::api::queue::Queue::get_timestamp_period") to get the value in nanoseconds. Absolute values have no meaning, but timestamps can be subtracted to get the time it takes for a string of operations to complete.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#143)

### impl [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[`Features::PIPELINE_STATISTICS_QUERY`](struct.WgpuFeatures.html#associatedconstant.PIPELINE_STATISTICS_QUERY "associated constant bevy::render::render_resource::WgpuFeatures::PIPELINE_STATISTICS_QUERY") must be enabled on the device in order to call these functions.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#151)

#### pub fn [begin\_pipeline\_statistics\_query](#method.begin_pipeline_statistics_query)( &mut self, query\_set: &[QuerySet](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet"), query\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), )

Start a pipeline statistics query on this compute pass. It can be ended with `end_pipeline_statistics_query`. Pipeline statistics queries may not be nested.

The amount of information collected by this query, and the space occupied in the query set, is determined by the [`PipelineStatisticsTypes`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/struct.PipelineStatisticsTypes.html "struct wgpu_types::PipelineStatisticsTypes") the query set was created with. `query_index` is the index of the first query result slot that will be written to, and `query_set` must have sufficient size to hold all results written starting at that slot.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#158)

#### pub fn [end\_pipeline\_statistics\_query](#method.end_pipeline_statistics_query)(&mut self)

End the pipeline statistics query on this compute pass. It can be started with `begin_pipeline_statistics_query`. Pipeline statistics queries may not be nested.

## Trait Implementations

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#12)

### impl<'encoder> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#12)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<H>(&self, state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

### impl [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

#### fn [cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)(&self, other: &[ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")

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

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

### impl [PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/compute_pass.rs.html#28)

#### fn [partial\_cmp](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)(&self, other: &[ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'\_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

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

### impl<'encoder> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

### impl<'encoder> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

### impl<'encoder> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

### impl<'encoder> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

### impl<'encoder> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

### impl<'encoder> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

### impl<'encoder> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")<'encoder>

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