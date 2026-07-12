[bevy](../../../index.html)::[material](../../index.html)::[bind\_group\_layout\_entries](../index.html)::[binding\_types](index.html)

# Function texture\_storage\_2d 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#543-546)

```rust
pub fn texture_storage_2d(
    format: TextureFormat,
    access: StorageTextureAccess,
) -> BindGroupLayoutEntryBuilder
```

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/shader/gpu\_readback.rs ([line 182](../../../../src/gpu_readback/gpu_readback.rs.html#182))

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

examples/shader/compute\_shader\_game\_of\_life.rs ([line 184](../../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#184))

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