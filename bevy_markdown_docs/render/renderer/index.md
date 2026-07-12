[bevy](../../index.html)::[render](../index.html)

# Module renderer 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#59)

## Modules

[raw\_vulkan\_init](raw_vulkan_init/index.html "mod bevy::render::renderer::raw_vulkan_init")`raw_vulkan_init`

## Structs

[CurrentView](struct.CurrentView.html "struct bevy::render::renderer::CurrentView")

The entity corresponding to the current view being rendered.

[FlushCommands](struct.FlushCommands.html "struct bevy::render::renderer::FlushCommands")

A system parameter that can be used to explicitly flush pending command buffers to the render queue. This is typically not necessary, as command buffers are automatically flushed at the end of each render system. However, in some cases it may be useful to flush command buffers earlier.

[PendingCommandBuffers](struct.PendingCommandBuffers.html "struct bevy::render::renderer::PendingCommandBuffers")

A resource that holds command buffers and encoders that are pending submission to the render queue.

[RenderAdapter](struct.RenderAdapter.html "struct bevy::render::renderer::RenderAdapter")

The handle to the physical device being used for rendering. See [`Adapter`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/adapter/struct.Adapter.html "struct wgpu::api::adapter::Adapter") for more info.

[RenderAdapterInfo](struct.RenderAdapterInfo.html "struct bevy::render::renderer::RenderAdapterInfo")

The [`AdapterInfo`](../render_resource/struct.WgpuAdapterInfo.html "struct bevy::render::render_resource::WgpuAdapterInfo") of the adapter in use by the renderer.

[RenderContext](struct.RenderContext.html "struct bevy::render::renderer::RenderContext")

A system parameter that provides access to a command encoder and render device for issuing rendering commands inside any system running beneath the root [`super::RenderGraph`](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph") schedule in the [`super::render_system`](fn.render_system.html "fn bevy::render::renderer::render_system") system.

[RenderContextState](struct.RenderContextState.html "struct bevy::render::renderer::RenderContextState")

A resource that holds the current render context state, including command encoder and command buffers. This is used internally by the [`RenderContext`](struct.RenderContext.html "struct bevy::render::renderer::RenderContext") system parameter. Implements [`SystemBuffer`](../../ecs/system/trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer") to flush command buffers at the end of each render system in topological system order.

[RenderDevice](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

This GPU device is responsible for the creation of most rendering and compute resources.

[RenderGraph](struct.RenderGraph.html "struct bevy::render::renderer::RenderGraph")

Schedule label for the root render graph schedule. This schedule runs once per frame in the [`render_system`](fn.render_system.html "fn bevy::render::renderer::render_system") system and is responsible for driving the entire rendering process.

[RenderInstance](struct.RenderInstance.html "struct bevy::render::renderer::RenderInstance")

The GPU instance is used to initialize the [`RenderQueue`](struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue") and [`RenderDevice`](struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"), as well as to create [`WindowSurfaces`](../view/struct.WindowSurfaces.html "struct bevy::render::view::WindowSurfaces").

[RenderQueue](struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")

This queue is used to enqueue tasks for the GPU to execute asynchronously.

[ViewQuery](struct.ViewQuery.html "struct bevy::render::renderer::ViewQuery")

A query that fetches components for the entity corresponding to the current view being rendered, as defined by the [`CurrentView`](struct.CurrentView.html "struct bevy::render::renderer::CurrentView") resource, equivalent to `query.get(current_view.entity())`.

[WgpuWrapper](struct.WgpuWrapper.html "struct bevy::render::renderer::WgpuWrapper")

A wrapper to safely make `wgpu` types Send / Sync on web with atomics enabled.

## Enums

[RenderGraphSystems](enum.RenderGraphSystems.html "enum bevy::render::renderer::RenderGraphSystems")

System sets for the root [`RenderGraph`](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph") schedule.

## Functions

[initialize\_renderer](fn.initialize_renderer.html "fn bevy::render::renderer::initialize_renderer")

Initializes the renderer by retrieving and preparing the GPU instance, device and queue for the specified backend.

[render\_system](fn.render_system.html "fn bevy::render::renderer::render_system")

The main render system that drives the rendering process. This system runs the [`RenderGraph`](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph") schedule, runs any finalization commands like screenshot captures and GPU readbacks, and calls present on swap chains that need to be presented.