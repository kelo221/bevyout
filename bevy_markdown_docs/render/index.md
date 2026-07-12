[bevy](../index.html)

# Crate render 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#1-587)

## Useful Environment Variables

Both `bevy_render` and `wgpu` have a number of environment variable options for changing the runtime behavior of both crates. Many of these may be useful in development or release environments.

*   `WGPU_DEBUG=1` enables debug labels, which can be useful in release builds.
*   `WGPU_VALIDATION=0` disables validation layers. This can help with particularly spammy errors.
*   `WGPU_FORCE_FALLBACK_ADAPTER=1` attempts to force software rendering. This typically matches what is used in CI.
*   `WGPU_ADAPTER_NAME` allows selecting a specific adapter by name.
*   `WGPU_SETTINGS_PRIO=webgl2` uses webgl2 limits.
*   `WGPU_SETTINGS_PRIO=webgpu` uses webgpu limits.
*   `VERBOSE_SHADER_ERROR=1` prints more detailed information about WGSL compilation errors, such as shader defs and shader entrypoint.

## Modules

[batching](batching/index.html "mod bevy::render::batching")

[camera](camera/index.html "mod bevy::render::camera")

[diagnostic](diagnostic/index.html "mod bevy::render::diagnostic")

Infrastructure for recording render diagnostics.

[erased\_render\_asset](erased_render_asset/index.html "mod bevy::render::erased_render_asset")

[error\_handler](error_handler/index.html "mod bevy::render::error_handler")

[extract\_component](extract_component/index.html "mod bevy::render::extract_component")

[extract\_instances](extract_instances/index.html "mod bevy::render::extract_instances")

Convenience logic for turning components from the main world into extracted instances in the render world.

[extract\_plugin](extract_plugin/index.html "mod bevy::render::extract_plugin")

[extract\_resource](extract_resource/index.html "mod bevy::render::extract_resource")

[globals](globals/index.html "mod bevy::render::globals")

[gpu\_component\_array\_buffer](gpu_component_array_buffer/index.html "mod bevy::render::gpu_component_array_buffer")

[gpu\_readback](gpu_readback/index.html "mod bevy::render::gpu_readback")

[mesh](mesh/index.html "mod bevy::render::mesh")

[occlusion\_culling](occlusion_culling/index.html "mod bevy::render::occlusion_culling")

GPU occlusion culling.

[pipelined\_rendering](pipelined_rendering/index.html "mod bevy::render::pipelined_rendering")Non-WebAssembly

[prelude](prelude/index.html "mod bevy::render::prelude")

The render prelude.

[render\_asset](render_asset/index.html "mod bevy::render::render_asset")

[render\_phase](render_phase/index.html "mod bevy::render::render_phase")

The modular rendering abstraction responsible for queuing, preparing, sorting and drawing entities as part of separate render phases.

[render\_resource](render_resource/index.html "mod bevy::render::render_resource")

[renderer](renderer/index.html "mod bevy::render::renderer")

[settings](settings/index.html "mod bevy::render::settings")

[slab\_allocator](slab_allocator/index.html "mod bevy::render::slab_allocator")

A general-purpose allocator that manages a set of GPU buffer slabs.

[storage](storage/index.html "mod bevy::render::storage")

[sync\_component](sync_component/index.html "mod bevy::render::sync_component")

[sync\_world](sync_world/index.html "mod bevy::render::sync_world")

[texture](texture/index.html "mod bevy::render::texture")

[uniform](uniform/index.html "mod bevy::render::uniform")

[view](view/index.html "mod bevy::render::view")

## Macros

[impl\_atomic\_pod](macro.impl_atomic_pod.html "macro bevy::render::impl_atomic_pod")

A macro that generates a _blob_ type that allows a POD type to be updated in shared memory.

## Structs

[Extract](struct.Extract.html "struct bevy::render::Extract")

A helper for accessing [`MainWorld`](struct.MainWorld.html "struct bevy::render::MainWorld") content using a system parameter.

[ExtractSchedule](struct.ExtractSchedule.html "struct bevy::render::ExtractSchedule")

Schedule in which data from the main world is ‘extracted’ into the render world.

[MainWorld](struct.MainWorld.html "struct bevy::render::MainWorld")

The simulation [`World`](../prelude/struct.World.html "struct bevy::prelude::World") of the application, stored as a resource.

[Render](struct.Render.html "struct bevy::render::Render")

The main render schedule.

[RenderApp](struct.RenderApp.html "struct bevy::render::RenderApp")

A label for the rendering sub-app.

[RenderDebugFlags](struct.RenderDebugFlags.html "struct bevy::render::RenderDebugFlags")

Debugging flags that can optionally be set when constructing the renderer.

[RenderPlugin](struct.RenderPlugin.html "struct bevy::render::RenderPlugin")

Contains the default Bevy rendering backend based on wgpu.

[RenderScheduleOrder](struct.RenderScheduleOrder.html "struct bevy::render::RenderScheduleOrder")

Defines the schedules to be run for the rendering, including their order.

[RenderStartup](struct.RenderStartup.html "struct bevy::render::RenderStartup")

The startup schedule of the [`RenderApp`](struct.RenderApp.html "struct bevy::render::RenderApp"). This can potentially run multiple times, and not on a fresh render world. Every time a new [`RenderDevice`](renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice") is acquired, this schedule runs to initialize any gpu resources needed for rendering on it.

## Enums

[RenderSystems](enum.RenderSystems.html "enum bevy::render::RenderSystems")

The systems sets of the default [`App`](../prelude/struct.App.html "struct bevy::prelude::App") rendering schedule.

## Traits

[GpuResourceAppExt](trait.GpuResourceAppExt.html "trait bevy::render::GpuResourceAppExt")

Convenience methods for render-recovery-aware resource initialization.

## Functions

[get\_adreno\_model](fn.get_adreno_model.html "fn bevy::render::get_adreno_model")

If the [`RenderAdapterInfo`](renderer/struct.RenderAdapterInfo.html "struct bevy::render::renderer::RenderAdapterInfo") is a Qualcomm Adreno, returns its model number.

[get\_mali\_driver\_version](fn.get_mali_driver_version.html "fn bevy::render::get_mali_driver_version")

Get the Mali driver version if the adapter is a Mali GPU.

[get\_pixel10\_driver\_version](fn.get_pixel10_driver_version.html "fn bevy::render::get_pixel10_driver_version")

[init\_gpu\_resource](fn.init_gpu_resource.html "fn bevy::render::init_gpu_resource")

Constructs a `T` resource with `from_world` and inserts it.

[storage\_buffers\_are\_unsupported](fn.storage_buffers_are_unsupported.html "fn bevy::render::storage_buffers_are_unsupported")

Returns true if storage buffers are unsupported on this platform or false if they are supported.