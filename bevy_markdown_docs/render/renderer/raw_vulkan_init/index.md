[bevy](../../../index.html)::[render](../../index.html)::[renderer](../index.html)

# Module raw\_vulkan\_init 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#2)

Available on **crate feature `raw_vulkan_init`** only.

## Structs

[AdditionalVulkanFeatures](struct.AdditionalVulkanFeatures.html "struct bevy::render::renderer::raw_vulkan_init::AdditionalVulkanFeatures")

A list of additional Vulkan features that are supported by the current wgpu instance / adapter. This is populated by callbacks defined in [`RawVulkanInitSettings`](struct.RawVulkanInitSettings.html "struct bevy::render::renderer::raw_vulkan_init::RawVulkanInitSettings")

[RawVulkanInitSettings](struct.RawVulkanInitSettings.html "struct bevy::render::renderer::raw_vulkan_init::RawVulkanInitSettings")

When the `raw_vulkan_init` feature is enabled, these settings will be used to configure the raw vulkan instance.