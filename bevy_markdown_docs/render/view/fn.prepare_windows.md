[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function prepare\_windows 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/mod.rs.html#244-250)

```rust
pub fn prepare_windows(
    windows: ResMut<'_, ExtractedWindows>,
    window_surfaces: ResMut<'_, WindowSurfaces>,
    render_device: Res<'_, RenderDevice>,
    sorted_cameras: Res<'_, SortedCameras>,
    render_instance: Res<'_, RenderInstance>,
)
```

(re)configures window surfaces, and obtains a swapchain texture for rendering.

NOTE: `get_current_texture` in `prepare_windows` can take a long time if the GPU workload is the performance bottleneck. This can be seen in profiles as multiple prepare-set systems all taking an unusually long time to complete, and all finishing at about the same time as the `prepare_windows` system. Improvements in bevy are planned to avoid this happening when it should not but it will still happen as it is easy for a user to create a large GPU workload relative to the GPU performance and/or CPU workload. This can be caused by many reasons, but several of them are:

*   GPU workload is more than your current GPU can manage
*   Error / performance bug in your custom shaders
*   wgpu was unable to detect a proper GPU hardware-accelerated device given the chosen [`Backends`](../settings/struct.Backends.html "struct bevy::render::settings::Backends"), [`WgpuLimits`](../render_resource/struct.WgpuLimits.html "struct bevy::render::render_resource::WgpuLimits"), and/or [`WgpuFeatures`](../render_resource/struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"). For example, on Windows currently `DirectX 11` is not supported by wgpu 0.12 and so if your GPU/drivers do not support Vulkan, it may be that a software renderer called “Microsoft Basic Render Driver” using `DirectX 12` will be chosen and performance will be very poor. This is visible in a log message that is output during renderer initialization. Another alternative is to try to use [`ANGLE`](https://github.com/gfx-rs/wgpu#angle) and [`Backends::GL`](../settings/struct.Backends.html#associatedconstant.GL "associated constant bevy::render::settings::Backends::GL") with the `gles` feature enabled if your GPU/drivers support `OpenGL 4.3` / `OpenGL ES 3.0` or later.