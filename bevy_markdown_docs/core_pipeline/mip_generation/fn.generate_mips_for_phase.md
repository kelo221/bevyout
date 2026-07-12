[bevy](../../index.html)::[core\_pipeline](../index.html)::[mip\_generation](index.html)

# Function generate\_mips\_for\_phase 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#377-384)

```rust
pub fn generate_mips_for_phase(
    phase_id: MipGenerationPhaseId,
    mip_generation_jobs: &MipGenerationJobs,
    pipeline_cache: &PipelineCache,
    mip_generation_bind_groups: &MipGenerationPipelines,
    gpu_images: &RenderAssets<GpuImage>,
    ctx: &mut RenderContext<'_, '_>,
)
```

Generates mipmaps for all images in a [`MipGenerationPhaseId`](struct.MipGenerationPhaseId.html "struct bevy::core_pipeline::mip_generation::MipGenerationPhaseId").

This function should be called from within a render system to generate mipmaps for all images that have been enqueued for the specified phase. The phased nature of mipmap generation allows precise control over the time when mipmaps are generated for each image. Your system should be ordered so that the mipmaps will be generated after any passes that _write_ to the images in question but before any shaders that _read_ from those images execute.

See `dynamic_mip_generation` for an example of use.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/2d/dynamic\_mip\_generation.rs ([lines 270-277](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#270-277))

```rust
260fn generate_mips_for_example(
261    mip_generation_jobs: Res<MipGenerationJobs>,
262    pipeline_cache: Res<PipelineCache>,
263    mip_generation_pipelines: Option<Res<MipGenerationPipelines>>,
264    gpu_images: Res<RenderAssets<GpuImage>>,
265    mut ctx: RenderContext,
266) {
267    let Some(mip_generation_pipelines) = mip_generation_pipelines else {
268        return;
269    };
270    generate_mips_for_phase(
271        MIP_GENERATION_PHASE_ID,
272        &mip_generation_jobs,
273        &pipeline_cache,
274        &mip_generation_pipelines,
275        &gpu_images,
276        &mut ctx,
277    );
278}
```