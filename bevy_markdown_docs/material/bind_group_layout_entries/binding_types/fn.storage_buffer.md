[bevy](../../../index.html)::[material](../../index.html)::[bind\_group\_layout\_entries](../index.html)::[binding\_types](index.html)

# Function storage\_buffer 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#376)

```rust
pub fn storage_buffer<T>(
    has_dynamic_offset: bool,
) -> BindGroupLayoutEntryBuilderwhere
    T: ShaderType,
```

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([line 181](../../../../src/gpu_readback/gpu_readback.rs.html#181))

```rust
171fn init_compute_pipeline(
172    mut commands: Commands,
173    asset_server: Res<AssetServer>,
174    pipeline_cache: Res<PipelineCache>,
175) {
176    let layout = BindGroupLayoutDescriptor::new(
177        "",
178        &BindGroupLayoutEntries::sequential(
179            ShaderStages::COMPUTE,
180            (
181                storage_buffer::<Vec<u32>>(false),
182                texture_storage_2d(TextureFormat::R32Uint, StorageTextureAccess::WriteOnly),
183            ),
184        ),
185    );
186    let shader = asset_server.load(SHADER_ASSET_PATH);
187    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
188        label: Some("GPU readback compute shader".into()),
189        layout: vec![layout.clone()],
190        shader: shader.clone(),
191        ..default()
192    });
193    commands.insert_resource(ComputePipeline { layout, pipeline });
194}
```

Hide additional examples

examples/shader\_advanced/compute\_mesh.rs ([line 227](../../../../src/compute_mesh/compute_mesh.rs.html#227))

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