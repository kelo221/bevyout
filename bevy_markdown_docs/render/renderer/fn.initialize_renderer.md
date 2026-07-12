[bevy](../../index.html)::[render](../index.html)::[renderer](index.html)

# Function initialize\_renderer 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#181-187)

```rust
pub async fn initialize_renderer(
    backends: Backends,
    primary_window: Option<RawHandleWrapperHolder>,
    options: &WgpuSettings,
    raw_vulkan_init_settings: RawVulkanInitSettings,
) -> RenderResources
```

Initializes the renderer by retrieving and preparing the GPU instance, device and queue for the specified backend.