[bevy](../../index.html)::[render](../index.html)::[renderer](index.html)

# Struct RenderDevice 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#15)

```rust
pub struct RenderDevice { /* private fields */ }
```

This GPU device is responsible for the creation of most rendering and compute resources.

## Implementations

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#25)

### impl [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#26)

#### pub fn [new](#method.new)(device: [WgpuWrapper](struct.WgpuWrapper.html "struct bevy::render::renderer::WgpuWrapper")<[Device](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html "struct wgpu::api::device::Device")\>) -> [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#34)

#### pub fn [features](#method.features)(&self) -> [Features](../render_resource/struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

List all [`Features`](../render_resource/struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures") that may be used with this device.

Functions may panic if you use unsupported features.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/texture\_binding\_array.rs ([line 81](../../../src/texture_binding_array/texture_binding_array.rs.html#81))

```rust
77fn verify_required_features(render_device: Res<RenderDevice>) {
78    // Check if the device support the required feature. If not, exit the example. In a real
79    // application, you should setup a fallback for the missing feature
80    if !render_device
81        .features()
82        .contains(WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING)
83    {
84        error!(
85            "Render device doesn't support feature \
86SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING, \
87which is required for texture binding arrays"
88        );
89        exit(1);
90    }
91}
```

Hide additional examples

examples/3d/occlusion\_culling.rs ([line 142](../../../src/occlusion_culling/occlusion_culling.rs.html#142))

```rust
128fn init_saved_indirect_parameters(
129    render_device: Res<RenderDevice>,
130    gpu_preprocessing_support: Res<GpuPreprocessingSupport>,
131    saved_indirect_parameters: Res<SavedIndirectParameters>,
132) {
133    let mut saved_indirect_parameters = saved_indirect_parameters.0.lock().unwrap();
134    *saved_indirect_parameters = Some(SavedIndirectParametersData {
135        data: vec![],
136        count: 0,
137        occlusion_culling_supported: gpu_preprocessing_support.is_culling_supported(),
138        // In order to determine how many meshes were culled, we look at the indirect count buffer
139        // that Bevy only populates if the platform supports `multi_draw_indirect_count`. So, if we
140        // don't have that feature, then we don't bother to display how many meshes were culled.
141        occlusion_culling_introspection_supported: render_device
142            .features()
143            .contains(WgpuFeatures::MULTI_DRAW_INDIRECT_COUNT),
144    });
145}
```

examples/3d/skybox.rs ([line 123](../../../src/skybox/skybox.rs.html#123))

```rust
106fn cycle_cubemap_asset(
107    time: Res<Time>,
108    mut next_swap: Local<f32>,
109    mut cubemap: ResMut<Cubemap>,
110    asset_server: Res<AssetServer>,
111    render_device: Res<RenderDevice>,
112) {
113    let now = time.elapsed_secs();
114    if *next_swap == 0.0 {
115        *next_swap = now + CUBEMAP_SWAP_DELAY;
116        return;
117    } else if now < *next_swap {
118        return;
119    }
120    *next_swap += CUBEMAP_SWAP_DELAY;
121
122    let supported_compressed_formats =
123        CompressedImageFormats::from_features(render_device.features());
124
125    let mut new_index = cubemap.index;
126    for _ in 0..CUBEMAPS.len() {
127        new_index = (new_index + 1) % CUBEMAPS.len();
128        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
129            break;
130        }
131        info!(
132            "Skipping format which is not supported by current hardware: {:?}",
133            CUBEMAPS[new_index]
134        );
135    }
136
137    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
138    // is missing
139    if new_index == cubemap.index {
140        return;
141    }
142
143    cubemap.index = new_index;
144    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
145    cubemap.is_loaded = false;
146}
```

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#42)

#### pub fn [limits](#method.limits)(&self) -> [Limits](../render_resource/struct.WgpuLimits.html "struct bevy::render::render_resource::WgpuLimits")

List all [`Limits`](../render_resource/struct.WgpuLimits.html "struct bevy::render::render_resource::WgpuLimits") that were requested of this device.

If any of these limits are exceeded, functions may panic.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#55-58)

#### pub unsafe fn [create\_shader\_module](#method.create_shader_module)( &self, desc: [ShaderModuleDescriptor](../render_resource/struct.ShaderModuleDescriptor.html "struct bevy::render::render_resource::ShaderModuleDescriptor")<'\_>, ) -> [ShaderModule](../render_resource/struct.ShaderModule.html "struct bevy::render::render_resource::ShaderModule")

Creates a [`ShaderModule`](../render_resource/struct.ShaderModule.html "struct bevy::render::render_resource::ShaderModule") from either SPIR-V or WGSL source code.

##### Safety

Creates a shader module with user-customizable runtime checks which allows shaders to perform operations which can lead to undefined behavior like indexing out of bounds, To avoid UB, ensure any unchecked shaders are sound! This method should never be called for user-supplied shaders.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#100-103)

#### pub fn [create\_and\_validate\_shader\_module](#method.create_and_validate_shader_module)( &self, desc: [ShaderModuleDescriptor](../render_resource/struct.ShaderModuleDescriptor.html "struct bevy::render::render_resource::ShaderModuleDescriptor")<'\_>, ) -> [ShaderModule](../render_resource/struct.ShaderModule.html "struct bevy::render::render_resource::ShaderModule")

Creates and validates a [`ShaderModule`](../render_resource/struct.ShaderModule.html "struct bevy::render::render_resource::ShaderModule") from either SPIR-V or WGSL source code.

See [`ValidateShader`](../../shader/enum.ValidateShader.html "enum bevy::shader::ValidateShader") for more information on the tradeoffs involved with shader validation.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([lines 180-186](../../../src/render_recovery/render_recovery.rs.html#180-186))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#123)

#### pub fn [poll](#method.poll)( &self, maintain: [PollType](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/enum.PollType.html "enum wgpu_types::PollType")<[SubmissionIndex](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.SubmissionIndex.html "struct wgpu::api::queue::SubmissionIndex")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[PollStatus](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/enum.PollStatus.html "enum wgpu_types::PollStatus"), [PollError](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/enum.PollError.html "enum wgpu_types::PollError")\>

Check for resource cleanups and mapping callbacks.

Return `true` if the queue is empty, or `false` if there are more queue submissions still in flight. (Note that, unless access to the [`wgpu::Queue`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.Queue.html "struct wgpu::api::queue::Queue") is coordinated somehow, this information could be out of date by the time the caller receives it. `Queue`s can be shared between threads, so other threads could submit new work at any time.)

no-op on the web, device is automatically polled.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([line 177](../../../src/render_recovery/render_recovery.rs.html#177))

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

Hide additional examples

examples/app/headless\_renderer.rs ([line 429](../../../src/headless_renderer/headless_renderer.rs.html#429))

```rust
375fn receive_image_from_buffer(
376    image_copiers: Res<ImageCopiers>,
377    render_device: Res<RenderDevice>,
378    sender: Res<RenderWorldSender>,
379) {
380    for image_copier in image_copiers.0.iter() {
381        if !image_copier.enabled() {
382            continue;
383        }
384
385        // Finally time to get our data back from the gpu.
386        // First we get a buffer slice which represents a chunk of the buffer (which we
387        // can't access yet).
388        // We want the whole thing so use unbounded range.
389        let buffer_slice = image_copier.buffer.slice(..);
390
391        // Now things get complicated. WebGPU, for safety reasons, only allows either the GPU
392        // or CPU to access a buffer's contents at a time. We need to "map" the buffer which means
393        // flipping ownership of the buffer over to the CPU and making access legal. We do this
394        // with `BufferSlice::map_async`.
395        //
396        // The problem is that map_async is not an async function so we can't await it. What
397        // we need to do instead is pass in a closure that will be executed when the slice is
398        // either mapped or the mapping has failed.
399        //
400        // The problem with this is that we don't have a reliable way to wait in the main
401        // code for the buffer to be mapped and even worse, calling get_mapped_range or
402        // get_mapped_range_mut prematurely will cause a panic, not return an error.
403        //
404        // Using channels solves this as awaiting the receiving of a message from
405        // the passed closure will force the outside code to wait. It also doesn't hurt
406        // if the closure finishes before the outside code catches up as the message is
407        // buffered and receiving will just pick that up.
408        //
409        // It may also be worth noting that although on native, the usage of asynchronous
410        // channels is wholly unnecessary, for the sake of portability to Wasm
411        // we'll use async channels that work on both native and Wasm.
412
413        let (s, r) = crossbeam_channel::bounded(1);
414
415        // Maps the buffer so it can be read on the cpu
416        buffer_slice.map_async(MapMode::Read, move |r| match r {
417            // This will execute once the gpu is ready, so after the call to poll()
418            Ok(r) => s.send(r).expect("Failed to send map update"),
419            Err(err) => panic!("Failed to map buffer {err}"),
420        });
421
422        // In order for the mapping to be completed, one of three things must happen.
423        // One of those can be calling `Device::poll`. This isn't necessary on the web as devices
424        // are polled automatically but natively, we need to make sure this happens manually.
425        // `Maintain::Wait` will cause the thread to wait on native but not on WebGpu.
426
427        // This blocks until the gpu is done executing everything
428        render_device
429            .poll(PollType::wait_indefinitely())
430            .expect("Failed to poll device for map async");
431
432        // This blocks until the buffer is mapped
433        r.recv().expect("Failed to receive the map_async message");
434
435        // This could fail on app exit, if Main world clears resources (including receiver) while Render world still renders
436        let _ = sender.send(buffer_slice.get_mapped_range().to_vec());
437
438        // We need to make sure all `BufferView`'s are dropped before we do what we're about
439        // to do.
440        // Unmap so that we can copy to the staging buffer in the next iteration.
441        image_copier.buffer.unmap();
442    }
443}
```

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#129-132)

#### pub fn [create\_command\_encoder](#method.create_command_encoder)( &self, desc: &[CommandEncoderDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/struct.CommandEncoderDescriptor.html "struct wgpu_types::CommandEncoderDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, ) -> [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

Creates an empty [`CommandEncoder`](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder").

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([line 335](../../../src/headless_renderer/headless_renderer.rs.html#335))

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

examples/app/render\_recovery.rs ([line 203](../../../src/render_recovery/render_recovery.rs.html#203))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#138-141)

#### pub fn [create\_render\_bundle\_encoder](#method.create_render_bundle_encoder)( &self, desc: &[RenderBundleEncoderDescriptor](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_bundle_encoder/struct.RenderBundleEncoderDescriptor.html "struct wgpu::api::render_bundle_encoder::RenderBundleEncoderDescriptor")<'\_>, ) -> [RenderBundleEncoder](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_bundle_encoder/struct.RenderBundleEncoder.html "struct wgpu::api::render_bundle_encoder::RenderBundleEncoder")<'\_>

Creates an empty [`RenderBundleEncoder`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_bundle_encoder/struct.RenderBundleEncoder.html "struct wgpu::api::render_bundle_encoder::RenderBundleEncoder").

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#147-152)

#### pub fn [create\_bind\_group](#method.create_bind_group)<'a>( &self, label: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, layout: &'a [BindGroupLayout](../render_resource/struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout"), entries: &'a \[[BindGroupEntry](../render_resource/struct.BindGroupEntry.html "struct bevy::render::render_resource::BindGroupEntry")<'a>\], ) -> [BindGroup](../render_resource/struct.BindGroup.html "struct bevy::render::render_resource::BindGroup")

Creates a new [`BindGroup`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/bind_group/struct.BindGroup.html "struct wgpu::api::bind_group::BindGroup").

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([lines 154-161](../../../src/gpu_readback/gpu_readback.rs.html#154-161))

```rust
142fn prepare_bind_group(
143    mut commands: Commands,
144    pipeline: Res<ComputePipeline>,
145    render_device: Res<RenderDevice>,
146    pipeline_cache: Res<PipelineCache>,
147    buffer: Res<ReadbackBuffer>,
148    image: Res<ReadbackImage>,
149    buffers: Res<RenderAssets<GpuShaderBuffer>>,
150    images: Res<RenderAssets<GpuImage>>,
151) {
152    let buffer = buffers.get(&buffer.0).unwrap();
153    let image = images.get(&image.0).unwrap();
154    let bind_group = render_device.create_bind_group(
155        None,
156        &pipeline_cache.get_bind_group_layout(&pipeline.layout),
157        &BindGroupEntries::sequential((
158            buffer.buffer.as_entire_buffer_binding(),
159            image.texture_view.into_binding(),
160        )),
161    );
162    commands.insert_resource(GpuBufferBindGroup(bind_group));
163}
```

Hide additional examples

examples/shader\_advanced/texture\_binding\_array.rs ([lines 131-135](../../../src/texture_binding_array/texture_binding_array.rs.html#131-135))

```rust
103    fn as_bind_group(
104        &self,
105        layout: &BindGroupLayoutDescriptor,
106        render_device: &RenderDevice,
107        pipeline_cache: &PipelineCache,
108        (image_assets, fallback_image): &mut SystemParamItem<'_, '_, Self::Param>,
109    ) -> Result<PreparedBindGroup, AsBindGroupError> {
110        // retrieve the render resources from handles
111        let mut images = vec![];
112        for handle in self.textures.iter().take(MAX_TEXTURE_COUNT) {
113            match image_assets.get(handle) {
114                Some(image) => images.push(image),
115                None => return Err(AsBindGroupError::RetryNextUpdate),
116            }
117        }
118
119        let fallback_image = &fallback_image.d2;
120
121        let textures = vec![&fallback_image.texture_view; MAX_TEXTURE_COUNT];
122
123        // convert bevy's resource types to WGPU's references
124        let mut textures: Vec<_> = textures.into_iter().map(|texture| &**texture).collect();
125
126        // fill in up to the first `MAX_TEXTURE_COUNT` textures and samplers to the arrays
127        for (id, image) in images.into_iter().enumerate() {
128            textures[id] = &*image.texture_view;
129        }
130
131        let bind_group = render_device.create_bind_group(
132            Self::label(),
133            &pipeline_cache.get_bind_group_layout(layout),
134            &BindGroupEntries::sequential((&textures[..], &fallback_image.sampler)),
135        );
136
137        Ok(PreparedBindGroup {
138            bindings: BindingResources(vec![]),
139            bind_group,
140        })
141    }
```

examples/shader/compute\_shader\_game\_of\_life.rs ([lines 146-154](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#146-154))

```rust
128fn prepare_bind_group(
129    mut commands: Commands,
130    pipeline: Res<GameOfLifePipeline>,
131    gpu_images: Res<RenderAssets<GpuImage>>,
132    game_of_life_images: Res<GameOfLifeImages>,
133    game_of_life_uniforms: Res<GameOfLifeUniforms>,
134    render_device: Res<RenderDevice>,
135    pipeline_cache: Res<PipelineCache>,
136    queue: Res<RenderQueue>,
137) {
138    let view_a = gpu_images.get(&game_of_life_images.texture_a).unwrap();
139    let view_b = gpu_images.get(&game_of_life_images.texture_b).unwrap();
140
141    // Uniform buffer is used here to demonstrate how to set up a uniform in a compute shader
142    // Alternatives such as storage buffers or push constants may be more suitable for your use case
143    let mut uniform_buffer = UniformBuffer::from(game_of_life_uniforms.into_inner());
144    uniform_buffer.write_buffer(&render_device, &queue);
145
146    let bind_group_0 = render_device.create_bind_group(
147        None,
148        &pipeline_cache.get_bind_group_layout(&pipeline.texture_bind_group_layout),
149        &BindGroupEntries::sequential((
150            &view_a.texture_view,
151            &view_b.texture_view,
152            &uniform_buffer,
153        )),
154    );
155    let bind_group_1 = render_device.create_bind_group(
156        None,
157        &pipeline_cache.get_bind_group_layout(&pipeline.texture_bind_group_layout),
158        &BindGroupEntries::sequential((
159            &view_b.texture_view,
160            &view_a.texture_view,
161            &uniform_buffer,
162        )),
163    );
164    commands.insert_resource(GameOfLifeImageBindGroups([bind_group_0, bind_group_1]));
165}
```

examples/shader\_advanced/compute\_mesh.rs ([lines 293-301](../../../src/compute_mesh/compute_mesh.rs.html#293-301))

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

examples/shader\_advanced/custom\_post\_processing.rs ([lines 121-133](../../../src/custom_post_processing/custom_post_processing.rs.html#121-133))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#163-167)

#### pub fn [create\_bind\_group\_layout](#method.create_bind_group_layout)<'a>( &self, label: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, entries: &'a \[[BindGroupLayoutEntry](../render_resource/struct.BindGroupLayoutEntry.html "struct bevy::render::render_resource::BindGroupLayoutEntry")\], ) -> [BindGroupLayout](../render_resource/struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout")

Creates a [`BindGroupLayout`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/bind_group_layout/struct.BindGroupLayout.html "struct wgpu::api::bind_group_layout::BindGroupLayout").

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#179-182)

#### pub fn [create\_pipeline\_layout](#method.create_pipeline_layout)( &self, desc: &[PipelineLayoutDescriptor](../render_resource/struct.PipelineLayoutDescriptor.html "struct bevy::render::render_resource::PipelineLayoutDescriptor")<'\_>, ) -> [PipelineLayout](../render_resource/struct.PipelineLayout.html "struct bevy::render::render_resource::PipelineLayout")

Creates a [`PipelineLayout`](../render_resource/struct.PipelineLayout.html "struct bevy::render::render_resource::PipelineLayout").

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([lines 188-192](../../../src/render_recovery/render_recovery.rs.html#188-192))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#188)

#### pub fn [create\_render\_pipeline](#method.create_render_pipeline)( &self, desc: &[RenderPipelineDescriptor](../render_resource/struct.RawRenderPipelineDescriptor.html "struct bevy::render::render_resource::RawRenderPipelineDescriptor")<'\_>, ) -> [RenderPipeline](../render_resource/struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline")

Creates a [`RenderPipeline`](../render_resource/struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline").

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#195-198)

#### pub fn [create\_compute\_pipeline](#method.create_compute_pipeline)( &self, desc: &[ComputePipelineDescriptor](../render_resource/struct.RawComputePipelineDescriptor.html "struct bevy::render::render_resource::RawComputePipelineDescriptor")<'\_>, ) -> [ComputePipeline](../render_resource/struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline")

Creates a [`ComputePipeline`](../render_resource/struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline").

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([lines 194-201](../../../src/render_recovery/render_recovery.rs.html#194-201))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#204)

#### pub fn [create\_buffer](#method.create_buffer)(&self, desc: &[BufferDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/buffer/struct.BufferDescriptor.html "struct wgpu_types::buffer::BufferDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>) -> [Buffer](../render_resource/struct.Buffer.html "struct bevy::render::render_resource::Buffer")

Creates a [`Buffer`](../render_resource/struct.Buffer.html "struct bevy::render::render_resource::Buffer").

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([lines 293-298](../../../src/headless_renderer/headless_renderer.rs.html#293-298))

```rust
287    pub fn new(
288        src_image: Handle<Image>,
289        size: Extent3d,
290        render_device: &RenderDevice,
291    ) -> ImageCopier {
292        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(size.width as usize * 4);
293        let cpu_buffer = render_device.create_buffer(&BufferDescriptor {
294            label: None,
295            size: padded_bytes_per_row as u64 * size.height as u64,
296            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
297            mapped_at_creation: false,
298        });
299
300        ImageCopier {
301            buffer: cpu_buffer,
302            src_image,
303            enabled: Arc::new(AtomicBool::new(true)),
304        }
305    }
```

Hide additional examples

examples/3d/occlusion\_culling.rs ([lines 472-477](../../../src/occlusion_culling/occlusion_culling.rs.html#472-477))

```rust
448fn create_indirect_parameters_staging_buffers(
449    mut indirect_parameters_staging_buffers: ResMut<IndirectParametersStagingBuffers>,
450    indirect_parameters_buffers: Res<IndirectParametersBuffers>,
451    render_device: Res<RenderDevice>,
452) {
453    let Some(phase_indirect_parameters_buffers) =
454        indirect_parameters_buffers.get(&TypeId::of::<Opaque3d>())
455    else {
456        return;
457    };
458
459    // Fetch the indirect parameters buffers that we're going to copy from.
460    let (Some(indexed_data_buffer), Some(indexed_batch_set_buffer)) = (
461        phase_indirect_parameters_buffers.indexed.data_buffer(),
462        phase_indirect_parameters_buffers
463            .indexed
464            .batch_sets_buffer(),
465    ) else {
466        return;
467    };
468
469    // Build the staging buffers. Make sure they have the same sizes as the
470    // buffers we're copying from.
471    indirect_parameters_staging_buffers.data =
472        Some(render_device.create_buffer(&BufferDescriptor {
473            label: Some("indexed data staging buffer"),
474            size: indexed_data_buffer.size(),
475            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
476            mapped_at_creation: false,
477        }));
478    indirect_parameters_staging_buffers.batch_sets =
479        Some(render_device.create_buffer(&BufferDescriptor {
480            label: Some("indexed batch set staging buffer"),
481            size: indexed_batch_set_buffer.size(),
482            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
483            mapped_at_creation: false,
484        }));
485}
```

examples/app/render\_recovery.rs ([lines 168-173](../../../src/render_recovery/render_recovery.rs.html#168-173))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#210)

#### pub fn [create\_buffer\_with\_data](#method.create_buffer_with_data)(&self, desc: &[BufferInitDescriptor](../render_resource/struct.BufferInitDescriptor.html "struct bevy::render::render_resource::BufferInitDescriptor")<'\_>) -> [Buffer](../render_resource/struct.Buffer.html "struct bevy::render::render_resource::Buffer")

Creates a [`Buffer`](../render_resource/struct.Buffer.html "struct bevy::render::render_resource::Buffer") and initializes it with the specified data.

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/custom\_shader\_instancing.rs ([lines 220-224](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#220-224))

```rust
214fn prepare_instance_buffers(
215    mut commands: Commands,
216    query: Query<(Entity, &InstanceMaterialData)>,
217    render_device: Res<RenderDevice>,
218) {
219    for (entity, instance_data) in &query {
220        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
221            label: Some("instance data buffer"),
222            contents: bytemuck::cast_slice(instance_data.as_slice()),
223            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
224        });
225        commands.entity(entity).insert(InstanceBuffer {
226            buffer,
227            length: instance_data.len(),
228        });
229    }
230}
```

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#219-225)

#### pub fn [create\_texture\_with\_data](#method.create_texture_with_data)( &self, render\_queue: &[RenderQueue](struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue"), desc: &[TextureDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureDescriptor.html "struct wgpu_types::texture::TextureDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, &\[[TextureFormat](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")\]>, order: [TextureDataOrder](../render_resource/enum.TextureDataOrder.html "enum bevy::render::render_resource::TextureDataOrder"), data: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Texture](../render_resource/struct.Texture.html "struct bevy::render::render_resource::Texture")

Creates a new [`Texture`](../render_resource/struct.Texture.html "struct bevy::render::render_resource::Texture") and initializes it with the specified data.

`desc` specifies the general format of the texture. `data` is the raw data.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#235)

#### pub fn [create\_texture](#method.create_texture)( &self, desc: &[TextureDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureDescriptor.html "struct wgpu_types::texture::TextureDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, &\[[TextureFormat](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")\]>, ) -> [Texture](../render_resource/struct.Texture.html "struct bevy::render::render_resource::Texture")

Creates a new [`Texture`](../render_resource/struct.Texture.html "struct bevy::render::render_resource::Texture").

`desc` specifies the general format of the texture.

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([lines 151-164](../../../src/render_recovery/render_recovery.rs.html#151-164))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#243)

#### pub fn [create\_sampler](#method.create_sampler)(&self, desc: &[SamplerDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.SamplerDescriptor.html "struct wgpu_types::texture::SamplerDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>) -> [Sampler](../render_resource/struct.Sampler.html "struct bevy::render::render_resource::Sampler")

Creates a new [`Sampler`](../render_resource/struct.Sampler.html "struct bevy::render::render_resource::Sampler").

`desc` specifies the behavior of the sampler.

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/manual\_material.rs ([line 94](../../../src/manual_material/manual_material.rs.html#94))

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

examples/shader\_advanced/custom\_post\_processing.rs ([line 198](../../../src/custom_post_processing/custom_post_processing.rs.html#198))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#254)

#### pub fn [configure\_surface](#method.configure_surface)( &self, surface: &[Surface](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/surface/struct.Surface.html "struct wgpu::api::surface::Surface")<'\_>, config: &[SurfaceConfiguration](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/surface/struct.SurfaceConfiguration.html "struct wgpu_types::surface::SurfaceConfiguration")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[TextureFormat](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")\>>, )

Initializes [`Surface`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/surface/struct.Surface.html "struct wgpu::api::surface::Surface") for presentation.

##### Panics

*   A old [`SurfaceTexture`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/surface_texture/struct.SurfaceTexture.html "struct wgpu::api::surface_texture::SurfaceTexture") is still alive referencing an old surface.
*   Texture format requested is unsupported on the surface.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#259)

#### pub fn [wgpu\_device](#method.wgpu_device)(&self) -> &[Device](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html "struct wgpu::api::device::Device")

Returns the wgpu [`Device`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html "struct wgpu::api::device::Device").

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 112](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#112))

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

examples/app/render\_recovery.rs ([line 176](../../../src/render_recovery/render_recovery.rs.html#176))

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#263-268)

#### pub fn [map\_buffer](#method.map_buffer)( &self, buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, map\_mode: [MapMode](../render_resource/enum.MapMode.html "enum bevy::render::render_resource::MapMode"), callback: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BufferAsyncError](../render_resource/struct.BufferAsyncError.html "struct bevy::render::render_resource::BufferAsyncError")\>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, )

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#273)

#### pub const fn [align\_copy\_bytes\_per\_row](#method.align_copy_bytes_per_row)(row\_bytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

##### [Examples found in repository](#scraped-examples-12)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([line 292](../../../src/headless_renderer/headless_renderer.rs.html#292))

```rust
287    pub fn new(
288        src_image: Handle<Image>,
289        size: Extent3d,
290        render_device: &RenderDevice,
291    ) -> ImageCopier {
292        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(size.width as usize * 4);
293        let cpu_buffer = render_device.create_buffer(&BufferDescriptor {
294            label: None,
295            size: padded_bytes_per_row as u64 * size.height as u64,
296            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
297            mapped_at_creation: false,
298        });
299
300        ImageCopier {
301            buffer: cpu_buffer,
302            src_image,
303            enabled: Arc::new(AtomicBool::new(true)),
304        }
305    }
306
307    pub fn enabled(&self) -> bool {
308        self.enabled.load(Ordering::Relaxed)
309    }
310}
311
312/// Extracting `ImageCopier`s into render world, because `ImageCopyDriver` accesses them
313fn image_copy_extract(mut commands: Commands, image_copiers: Extract<Query<&ImageCopier>>) {
314    commands.insert_resource(ImageCopiers(
315        image_copiers.iter().cloned().collect::<Vec<ImageCopier>>(),
316    ));
317}
318
319// Copies image content from render target to buffer
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
373
374/// runs in render world after Render stage to send image from buffer via channel (receiver is in main world)
375fn receive_image_from_buffer(
376    image_copiers: Res<ImageCopiers>,
377    render_device: Res<RenderDevice>,
378    sender: Res<RenderWorldSender>,
379) {
380    for image_copier in image_copiers.0.iter() {
381        if !image_copier.enabled() {
382            continue;
383        }
384
385        // Finally time to get our data back from the gpu.
386        // First we get a buffer slice which represents a chunk of the buffer (which we
387        // can't access yet).
388        // We want the whole thing so use unbounded range.
389        let buffer_slice = image_copier.buffer.slice(..);
390
391        // Now things get complicated. WebGPU, for safety reasons, only allows either the GPU
392        // or CPU to access a buffer's contents at a time. We need to "map" the buffer which means
393        // flipping ownership of the buffer over to the CPU and making access legal. We do this
394        // with `BufferSlice::map_async`.
395        //
396        // The problem is that map_async is not an async function so we can't await it. What
397        // we need to do instead is pass in a closure that will be executed when the slice is
398        // either mapped or the mapping has failed.
399        //
400        // The problem with this is that we don't have a reliable way to wait in the main
401        // code for the buffer to be mapped and even worse, calling get_mapped_range or
402        // get_mapped_range_mut prematurely will cause a panic, not return an error.
403        //
404        // Using channels solves this as awaiting the receiving of a message from
405        // the passed closure will force the outside code to wait. It also doesn't hurt
406        // if the closure finishes before the outside code catches up as the message is
407        // buffered and receiving will just pick that up.
408        //
409        // It may also be worth noting that although on native, the usage of asynchronous
410        // channels is wholly unnecessary, for the sake of portability to Wasm
411        // we'll use async channels that work on both native and Wasm.
412
413        let (s, r) = crossbeam_channel::bounded(1);
414
415        // Maps the buffer so it can be read on the cpu
416        buffer_slice.map_async(MapMode::Read, move |r| match r {
417            // This will execute once the gpu is ready, so after the call to poll()
418            Ok(r) => s.send(r).expect("Failed to send map update"),
419            Err(err) => panic!("Failed to map buffer {err}"),
420        });
421
422        // In order for the mapping to be completed, one of three things must happen.
423        // One of those can be calling `Device::poll`. This isn't necessary on the web as devices
424        // are polled automatically but natively, we need to make sure this happens manually.
425        // `Maintain::Wait` will cause the thread to wait on native but not on WebGpu.
426
427        // This blocks until the gpu is done executing everything
428        render_device
429            .poll(PollType::wait_indefinitely())
430            .expect("Failed to poll device for map async");
431
432        // This blocks until the buffer is mapped
433        r.recv().expect("Failed to receive the map_async message");
434
435        // This could fail on app exit, if Main world clears resources (including receiver) while Render world still renders
436        let _ = sender.send(buffer_slice.get_mapped_range().to_vec());
437
438        // We need to make sure all `BufferView`'s are dropped before we do what we're about
439        // to do.
440        // Unmap so that we can copy to the staging buffer in the next iteration.
441        image_copier.buffer.unmap();
442    }
443}
444
445/// CPU-side image for saving
446#[derive(Component, Deref, DerefMut)]
447struct ImageToSave(Handle<Image>);
448
449// Takes from channel image content sent from render world and saves it to disk
450fn update(
451    images_to_save: Query<&ImageToSave>,
452    receiver: Res<MainWorldReceiver>,
453    mut images: ResMut<Assets<Image>>,
454    mut scene_controller: ResMut<SceneController>,
455    mut app_exit_writer: MessageWriter<AppExit>,
456    mut file_number: Local<u32>,
457) {
458    if let SceneState::Render(n) = scene_controller.state {
459        if n < 1 {
460            // We don't want to block the main world on this,
461            // so we use try_recv which attempts to receive without blocking
462            let mut image_data = Vec::new();
463            while let Ok(data) = receiver.try_recv() {
464                // image generation could be faster than saving to fs,
465                // that's why use only last of them
466                image_data = data;
467            }
468            if !image_data.is_empty() {
469                for image in images_to_save.iter() {
470                    // Fill correct data from channel to image
471                    let mut img_bytes = images.get_mut(image.id()).unwrap();
472
473                    // We need to ensure that this works regardless of the image dimensions
474                    // If the image became wider when copying from the texture to the buffer,
475                    // then the data is reduced to its original size when copying from the buffer to the image.
476                    let row_bytes = img_bytes.width() as usize
477                        * img_bytes.texture_descriptor.format.pixel_size().unwrap();
478                    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
479                    if row_bytes == aligned_row_bytes {
480                        img_bytes.data.as_mut().unwrap().clone_from(&image_data);
481                    } else {
482                        // shrink data to original image size
483                        img_bytes.data = Some(
484                            image_data
485                                .chunks(aligned_row_bytes)
486                                .take(img_bytes.height() as usize)
487                                .flat_map(|row| &row[..row_bytes.min(row.len())])
488                                .cloned()
489                                .collect(),
490                        );
491                    }
492
493                    // Create RGBA Image Buffer
494                    let img = match img_bytes.clone().try_into_dynamic() {
495                        Ok(img) => img.to_rgba8(),
496                        Err(e) => panic!("Failed to create image buffer {e:?}"),
497                    };
498
499                    // Prepare directory for images, test_images in bevy folder is used here for example
500                    // You should choose the path depending on your needs
501                    let images_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_images");
502                    info!("Saving image to: {images_dir:?}");
503                    std::fs::create_dir_all(&images_dir).unwrap();
504
505                    // Choose filename starting from 000.png
506                    let image_path = images_dir.join(format!("{:03}.png", file_number.deref()));
507                    *file_number.deref_mut() += 1;
508
509                    // Finally saving image to file, this heavy blocking operation is kept here
510                    // for example simplicity, but in real app you should move it to a separate task
511                    if let Err(e) = img.save(image_path) {
512                        panic!("Failed to save image: {e}");
513                    };
514                }
515                if scene_controller.single_image {
516                    app_exit_writer.write(AppExit::Success);
517                }
518            }
519        } else {
520            // clears channel for skipped frames
521            while receiver.try_recv().is_ok() {}
522            scene_controller.state = SceneState::Render(n - 1);
523        }
524    }
525}
```

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#284-287)

#### pub fn [get\_supported\_read\_only\_binding\_type](#method.get_supported_read_only_binding_type)( &self, buffers\_per\_shader\_stage: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> [BufferBindingType](../render_resource/enum.BufferBindingType.html "enum bevy::render::render_resource::BufferBindingType")

## Trait Implementations

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

### impl [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

where [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

#### const [STORAGE\_TYPE](../../prelude/trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::SparseSet

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

#### type [Mutability](../../prelude/trait.Component.html#associatedtype.Mutability) = [Mutable](../../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](../../prelude/trait.Component.html "trait bevy::prelude::Component"). [Read more](../../prelude/trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

#### fn [register\_required\_components](../../prelude/trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](../../prelude/trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

#### fn [clone\_behavior](../../prelude/trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](../../prelude/trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

#### fn [relationship\_accessor](../../prelude/trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>>

Returns [`ComponentRelationshipAccessor`](../../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](../../prelude/trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](../../prelude/trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](../../prelude/trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](../../prelude/trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](../../prelude/trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](../../prelude/trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](../../prelude/trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](../../prelude/trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#19)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Device](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html "struct wgpu::api::device::Device")\> for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#20)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(device: [Device](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html "struct wgpu::api::device::Device")) -> [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

### impl [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

where [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

## Auto Trait Implementations

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](../../prelude/trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](../../prelude/trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

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

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}