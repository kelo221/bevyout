[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Struct WgpuFeatures 

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

```rust
#[repr(C)]pub struct WgpuFeatures {
    pub features_wgpu: FeaturesWGPU,
    pub features_webgpu: FeaturesWebGPU,
}
```

Features that are not guaranteed to be supported.

These are either part of the webgpu standard, or are extension features supported by wgpu when targeting native.

If you want to use a feature, you need to first verify that the adapter supports the feature. If the adapter does not support the feature, requesting a device with it enabled will panic.

Corresponds to [WebGPU `GPUFeatureName`](https://gpuweb.github.io/gpuweb/#enumdef-gpufeaturename).

## Fields

`features_wgpu: [FeaturesWGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWGPU.html "struct wgpu_types::features::FeaturesWGPU")``features_webgpu: [FeaturesWebGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWebGPU.html "struct wgpu_types::features::FeaturesWebGPU")`

## Implementations

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_FLOAT32\_ATOMIC](#associatedconstant.SHADER_FLOAT32_ATOMIC): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use f32 atomic load, store, add, sub, and exchange.

Supported platforms:

*   Metal (with MSL 3.0+ and Apple7+/Mac2)
*   Vulkan (with [VK\_EXT\_shader\_atomic\_float](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float.html))

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_FORMAT\_16BIT\_NORM](#associatedconstant.TEXTURE_FORMAT_16BIT_NORM): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables normalized `16-bit` texture formats.

Supported platforms:

*   Vulkan
*   DX12
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_COMPRESSION\_ASTC\_HDR](#associatedconstant.TEXTURE_COMPRESSION_ASTC_HDR): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables ASTC HDR family of compressed textures.

Compressed textures sacrifice some quality in exchange for significantly reduced bandwidth usage.

Support for this feature guarantees availability of \[`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`\] for ASTC formats with the HDR channel type. [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`](struct.WgpuFeatures.html#associatedconstant.TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES "associated constant bevy::render::render_resource::WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES") may enable additional usages.

Supported Platforms:

*   Metal
*   Vulkan
*   OpenGL

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_ADAPTER\_SPECIFIC\_FORMAT\_FEATURES](#associatedconstant.TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables device specific texture format features.

See `TextureFormatFeatures` for a listing of the features in question.

By default only texture format properties as defined by the WebGPU specification are allowed. Enabling this feature flag extends the features of each format to the ones supported by the current device. Note that without this flag, read/write storage access is not allowed at all.

This extension does not enable additional formats.

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [PIPELINE\_STATISTICS\_QUERY](#associatedconstant.PIPELINE_STATISTICS_QUERY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables use of Pipeline Statistics Queries. These queries tell the count of various operations performed between the start and stop call. Call [`RenderPass::begin_pipeline_statistics_query`](../wgpu/struct.RenderPass.html#method.begin_pipeline_statistics_query) to start a query, then call [`RenderPass::end_pipeline_statistics_query`](../wgpu/struct.RenderPass.html#method.end_pipeline_statistics_query) to stop one.

They must be resolved using [`CommandEncoder::resolve_query_set`](../wgpu/struct.CommandEncoder.html#method.resolve_query_set) into a buffer. The rules on how these resolve into buffers are detailed in the documentation for [`PipelineStatisticsTypes`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/struct.PipelineStatisticsTypes.html "struct wgpu_types::PipelineStatisticsTypes").

Supported Platforms:

*   Vulkan
*   DX12

This is a native only feature with a [proposal](https://github.com/gpuweb/gpuweb/blob/0008bd30da2366af88180b511a5d0d0c1dffbc36/proposals/pipeline-statistics-query.md) for the web.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TIMESTAMP\_QUERY\_INSIDE\_ENCODERS](#associatedconstant.TIMESTAMP_QUERY_INSIDE_ENCODERS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for timestamp queries directly on command encoders.

Implies [`Features::TIMESTAMP_QUERY`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY") is supported.

Additionally allows for timestamp writes on command encoders using [`CommandEncoder::write_timestamp`](../wgpu/struct.CommandEncoder.html#method.write_timestamp).

Supported platforms:

*   Vulkan
*   DX12
*   Metal
*   OpenGL (with GL\_ARB\_timer\_query)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TIMESTAMP\_QUERY\_INSIDE\_PASSES](#associatedconstant.TIMESTAMP_QUERY_INSIDE_PASSES): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for timestamp queries directly on command encoders.

Implies [`Features::TIMESTAMP_QUERY`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY") & [`Features::TIMESTAMP_QUERY_INSIDE_ENCODERS`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY_INSIDE_ENCODERS "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY_INSIDE_ENCODERS") is supported.

Additionally allows for timestamp queries to be used inside render & compute passes using:

*   [`RenderPass::write_timestamp`](../wgpu/struct.RenderPass.html#method.write_timestamp)
*   [`ComputePass::write_timestamp`](../wgpu/struct.ComputePass.html#method.write_timestamp)

Supported platforms:

*   Vulkan
*   DX12
*   Metal (AMD & Intel, not Apple GPUs)
*   OpenGL (with GL\_ARB\_timer\_query)

This is generally not available on tile-based rasterization GPUs.

This is a native only feature with a [proposal](https://github.com/gpuweb/gpuweb/blob/0008bd30da2366af88180b511a5d0d0c1dffbc36/proposals/timestamp-query-inside-passes.md) for the web.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [MAPPABLE\_PRIMARY\_BUFFERS](#associatedconstant.MAPPABLE_PRIMARY_BUFFERS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Webgpu only allows the MAP\_READ and MAP\_WRITE buffer usage to be matched with COPY\_DST and COPY\_SRC respectively. This removes this requirement.

This is only beneficial on systems that share memory between CPU and GPU. If enabled on a system that doesn’t, this can severely hinder performance. Only use if you understand the consequences.

Supported platforms:

*   Vulkan
*   DX12
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_BINDING\_ARRAY](#associatedconstant.TEXTURE_BINDING_ARRAY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to create uniform arrays of textures in shaders:

ex.

*   `var textures: binding_array<texture_2d<f32>, 10>` (WGSL)
*   `uniform texture2D textures[10]` (GLSL)

If [`Features::STORAGE_RESOURCE_BINDING_ARRAY`](struct.WgpuFeatures.html#associatedconstant.STORAGE_RESOURCE_BINDING_ARRAY "associated constant bevy::render::render_resource::WgpuFeatures::STORAGE_RESOURCE_BINDING_ARRAY") is supported as well as this, the user may also create uniform arrays of storage textures.

ex.

*   `var textures: array<texture_storage_2d<r32float, write>, 10>` (WGSL)
*   `uniform image2D textures[10]` (GLSL)

This capability allows them to exist and to be indexed by dynamically uniform values.

Supported platforms:

*   DX12
*   Metal (with MSL 2.0+ on macOS 10.13+)
*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [BUFFER\_BINDING\_ARRAY](#associatedconstant.BUFFER_BINDING_ARRAY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to create arrays of buffers in shaders:

ex.

*   `var<uniform> buffer_array: array<MyBuffer, 10>` (WGSL)
*   `uniform myBuffer { ... } buffer_array[10]` (GLSL)

This capability allows them to exist and to be indexed by dynamically uniform values.

If [`Features::STORAGE_RESOURCE_BINDING_ARRAY`](struct.WgpuFeatures.html#associatedconstant.STORAGE_RESOURCE_BINDING_ARRAY "associated constant bevy::render::render_resource::WgpuFeatures::STORAGE_RESOURCE_BINDING_ARRAY") is supported as well as this, the user may also create arrays of storage buffers.

ex.

*   `var<storage> buffer_array: array<MyBuffer, 10>` (WGSL)
*   `buffer myBuffer { ... } buffer_array[10]` (GLSL)

Supported platforms:

*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [STORAGE\_RESOURCE\_BINDING\_ARRAY](#associatedconstant.STORAGE_RESOURCE_BINDING_ARRAY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to create uniform arrays of storage buffers or textures in shaders, if resp. [`Features::BUFFER_BINDING_ARRAY`](struct.WgpuFeatures.html#associatedconstant.BUFFER_BINDING_ARRAY "associated constant bevy::render::render_resource::WgpuFeatures::BUFFER_BINDING_ARRAY") or [`Features::TEXTURE_BINDING_ARRAY`](struct.WgpuFeatures.html#associatedconstant.TEXTURE_BINDING_ARRAY "associated constant bevy::render::render_resource::WgpuFeatures::TEXTURE_BINDING_ARRAY") is supported.

This capability allows them to exist and to be indexed by dynamically uniform values.

Supported platforms:

*   Metal (with MSL 2.2+ on macOS 10.13+)
*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SAMPLED\_TEXTURE\_AND\_STORAGE\_BUFFER\_ARRAY\_NON\_UNIFORM\_INDEXING](#associatedconstant.SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to index sampled texture and storage buffer resource arrays with dynamically non-uniform values:

ex. `texture_array[vertex_data]`

In order to use this capability, the corresponding GLSL extension must be enabled like so:

`#extension GL_EXT_nonuniform_qualifier : require`

and then used either as `nonuniformEXT` qualifier in variable declaration:

ex. `layout(location = 0) nonuniformEXT flat in int vertex_data;`

or as `nonuniformEXT` constructor:

ex. `texture_array[nonuniformEXT(vertex_data)]`

WGSL and HLSL do not need any extension.

Supported platforms:

*   DX12
*   Metal (with MSL 2.0+ on macOS 10.13+)
*   Vulkan 1.2+ (or VK\_EXT\_descriptor\_indexing)’s shaderSampledImageArrayNonUniformIndexing & shaderStorageBufferArrayNonUniformIndexing feature)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [STORAGE\_TEXTURE\_ARRAY\_NON\_UNIFORM\_INDEXING](#associatedconstant.STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to index storage texture resource arrays with dynamically non-uniform values:

ex. `texture_array[vertex_data]`

Supported platforms:

*   DX12
*   Metal (with MSL 2.0+ on macOS 10.13+)
*   Vulkan 1.2+ (or VK\_EXT\_descriptor\_indexing)’s shaderStorageTextureArrayNonUniformIndexing feature)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [PARTIALLY\_BOUND\_BINDING\_ARRAY](#associatedconstant.PARTIALLY_BOUND_BINDING_ARRAY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to create bind groups containing arrays with less bindings than the BindGroupLayout.

Supported platforms:

*   Vulkan
*   DX12

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [MULTI\_DRAW\_INDIRECT\_COUNT](#associatedconstant.MULTI_DRAW_INDIRECT_COUNT): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to call [`RenderPass::multi_draw_indirect_count`](../wgpu/struct.RenderPass.html#method.multi_draw_indirect_count) and [`RenderPass::multi_draw_indexed_indirect_count`](../wgpu/struct.RenderPass.html#method.multi_draw_indexed_indirect_count).

This allows the use of a buffer containing the actual number of draw calls. This feature being present also implies that all calls to [`RenderPass::multi_draw_indirect`](../wgpu/struct.RenderPass.html#method.multi_draw_indirect) and [`RenderPass::multi_draw_indexed_indirect`](../wgpu/struct.RenderPass.html#method.multi_draw_indexed_indirect) are not being emulated with a series of `draw_indirect` calls.

Supported platforms:

*   DX12
*   Vulkan 1.2+ (or VK\_KHR\_draw\_indirect\_count)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [ADDRESS\_MODE\_CLAMP\_TO\_ZERO](#associatedconstant.ADDRESS_MODE_CLAMP_TO_ZERO): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the use of [`AddressMode::ClampToBorder`](enum.AddressMode.html#variant.ClampToBorder "variant bevy::render::render_resource::AddressMode::ClampToBorder") with a border color of [`SamplerBorderColor::Zero`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/enum.SamplerBorderColor.html#variant.Zero "variant wgpu_types::texture::SamplerBorderColor::Zero").

Supported platforms:

*   DX12
*   Vulkan
*   Metal
*   OpenGL

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [ADDRESS\_MODE\_CLAMP\_TO\_BORDER](#associatedconstant.ADDRESS_MODE_CLAMP_TO_BORDER): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the use of [`AddressMode::ClampToBorder`](enum.AddressMode.html#variant.ClampToBorder "variant bevy::render::render_resource::AddressMode::ClampToBorder") with a border color other than [`SamplerBorderColor::Zero`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/enum.SamplerBorderColor.html#variant.Zero "variant wgpu_types::texture::SamplerBorderColor::Zero").

Supported platforms:

*   DX12
*   Vulkan
*   Metal (macOS 10.12+ only)
*   OpenGL

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [POLYGON\_MODE\_LINE](#associatedconstant.POLYGON_MODE_LINE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to set [`PolygonMode::Line`](enum.PolygonMode.html#variant.Line "variant bevy::render::render_resource::PolygonMode::Line") in [`PrimitiveState::polygon_mode`](struct.PrimitiveState.html "struct bevy::render::render_resource::PrimitiveState")

This allows drawing polygons/triangles as lines (wireframe) instead of filled

Supported platforms:

*   DX12
*   Vulkan
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [POLYGON\_MODE\_POINT](#associatedconstant.POLYGON_MODE_POINT): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to set [`PolygonMode::Point`](enum.PolygonMode.html#variant.Point "variant bevy::render::render_resource::PolygonMode::Point") in [`PrimitiveState::polygon_mode`](struct.PrimitiveState.html "struct bevy::render::render_resource::PrimitiveState")

This allows only drawing the vertices of polygons/triangles instead of filled

Supported platforms:

*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [CONSERVATIVE\_RASTERIZATION](#associatedconstant.CONSERVATIVE_RASTERIZATION): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to set a overestimation-conservative-rasterization in [`PrimitiveState::conservative`](struct.PrimitiveState.html#structfield.conservative "field bevy::render::render_resource::PrimitiveState::conservative")

Processing of degenerate triangles/lines is hardware specific. Only triangles are supported.

Supported platforms:

*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [VERTEX\_WRITABLE\_STORAGE](#associatedconstant.VERTEX_WRITABLE_STORAGE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables bindings of writable storage buffers and textures visible to vertex shaders.

Note: some (tiled-based) platforms do not support vertex shaders with any side-effects.

Supported Platforms:

*   All

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [CLEAR\_TEXTURE](#associatedconstant.CLEAR_TEXTURE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables clear to zero for textures.

Supported platforms:

*   All

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [MULTIVIEW](#associatedconstant.MULTIVIEW): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables multiview render passes and `builtin(view_index)` in vertex/mesh shaders.

Supported platforms:

*   Vulkan
*   Metal
*   DX12
*   OpenGL (web only)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [VERTEX\_ATTRIBUTE\_64BIT](#associatedconstant.VERTEX_ATTRIBUTE_64BIT): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables using 64-bit types for vertex attributes.

Requires SHADER\_FLOAT64.

Supported Platforms: N/A

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_ATOMIC](#associatedconstant.TEXTURE_ATOMIC): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables image atomic fetch add, and, xor, or, min, and max for R32Uint and R32Sint textures.

Supported platforms:

*   Vulkan
*   DX12
*   Metal (with MSL 3.1+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_FORMAT\_NV12](#associatedconstant.TEXTURE_FORMAT_NV12): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for creation of textures of format [`TextureFormat::NV12`](enum.TextureFormat.html#variant.NV12 "variant bevy::render::render_resource::TextureFormat::NV12")

Supported platforms:

*   DX12
*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_FORMAT\_P010](#associatedconstant.TEXTURE_FORMAT_P010): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for creation of textures of format [`TextureFormat::P010`](enum.TextureFormat.html#variant.P010 "variant bevy::render::render_resource::TextureFormat::P010")

Supported platforms:

*   DX12
*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXTERNAL\_TEXTURE](#associatedconstant.EXTERNAL_TEXTURE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for the creation and usage of `ExternalTexture`s, and bind group layouts containing external texture `BindingType`s.

Conceptually this should really be a [`crate::DownlevelFlags`](struct.DownlevelFlags.html "struct bevy::render::render_resource::DownlevelFlags") as it corresponds to WebGPU’s [`GPUExternalTexture`](https://www.w3.org/TR/webgpu/#gpuexternaltexture). However, the implementation is currently in-progress, and until it is complete we do not want applications to ignore adapters due to a missing downlevel flag, when they may not require this feature at all.

Supported platforms:

*   DX12
*   Metal

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXPERIMENTAL\_RAY\_QUERY](#associatedconstant.EXPERIMENTAL_RAY_QUERY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

_**THIS IS EXPERIMENTAL:**_ Features enabled by this may have major bugs in it and are expected to be subject to breaking changes, suggestions for the API exposed by this should be posted on [the ray-tracing issue](https://github.com/gfx-rs/wgpu/issues/1040)

Allows for the creation of ray-tracing queries within shaders.

Supported platforms:

*   Vulkan

This is a native-only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_F64](#associatedconstant.SHADER_F64): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables 64-bit floating point types in SPIR-V shaders.

Note: even when supported by GPU hardware, 64-bit floating point operations are frequently between 16 and 64 _times_ slower than equivalent operations on 32-bit floats.

Supported Platforms:

*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_I16](#associatedconstant.SHADER_I16): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use i16. Not currently supported in `naga`, only available through `spirv-passthrough`.

Supported platforms:

*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_EARLY\_DEPTH\_TEST](#associatedconstant.SHADER_EARLY_DEPTH_TEST): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use the `early_depth_test` attribute.

The attribute is applied to the fragment shader entry point. It can be used in two ways:

1.  Force early depth/stencil tests:
    
    *   `@early_depth_test(force)` (WGSL)
        
    *   `layout(early_fragment_tests) in;` (GLSL)
        
2.  Provide a conservative depth specifier that allows an additional early depth test under certain conditions:
    
    *   `@early_depth_test(greater_equal/less_equal/unchanged)` (WGSL)
        
    *   `layout(depth_<greater/less/unchanged>) out float gl_FragDepth;` (GLSL)
        

See [`EarlyDepthTest`](https://docs.rs/naga/latest/naga/ir/enum.EarlyDepthTest.html) for more details.

Supported platforms:

*   Vulkan
*   GLES 3.1+

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_INT64](#associatedconstant.SHADER_INT64): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use i64 and u64.

Supported platforms:

*   Vulkan
*   DX12 (DXC only)
*   Metal (with MSL 2.3+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SUBGROUP](#associatedconstant.SUBGROUP): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows compute and fragment shaders to use the subgroup operation built-ins and perform subgroup operations (except barriers).

Supported Platforms:

*   Vulkan
*   DX12
*   Metal

The `subgroups` feature has been added to WebGPU, but there may be differences between the standard and the `wgpu` implementation, so it remains a native-only feature in wgpu for now. See [https://github.com/gfx-rs/wgpu/issues/5555](https://github.com/gfx-rs/wgpu/issues/5555).

Because it is expected to move to the WebGPU feature set in the not-too-distant future, the name omits the `wgpu-` prefix.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SUBGROUP\_VERTEX](#associatedconstant.SUBGROUP_VERTEX): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows vertex shaders to use the subgroup operation built-ins and perform subgroup operations (except barriers).

Supported Platforms:

*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SUBGROUP\_BARRIER](#associatedconstant.SUBGROUP_BARRIER): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows compute shaders to use the subgroup barrier.

Requires [`Features::SUBGROUP`](struct.WgpuFeatures.html#associatedconstant.SUBGROUP "associated constant bevy::render::render_resource::WgpuFeatures::SUBGROUP"). Without it, enables nothing.

Supported Platforms:

*   Vulkan
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [PIPELINE\_CACHE](#associatedconstant.PIPELINE_CACHE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the use of pipeline cache objects

Supported platforms:

*   Vulkan

Unimplemented Platforms:

*   DX12
*   Metal

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_INT64\_ATOMIC\_MIN\_MAX](#associatedconstant.SHADER_INT64_ATOMIC_MIN_MAX): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use i64 and u64 atomic min and max.

Supported platforms:

*   Vulkan (with VK\_KHR\_shader\_atomic\_int64)
*   DX12 (with SM 6.6+)
*   Metal (with MSL 2.4+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_INT64\_ATOMIC\_ALL\_OPS](#associatedconstant.SHADER_INT64_ATOMIC_ALL_OPS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use all i64 and u64 atomic operations.

Supported platforms:

*   Vulkan (with VK\_KHR\_shader\_atomic\_int64)
*   DX12 (with SM 6.6+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [VULKAN\_GOOGLE\_DISPLAY\_TIMING](#associatedconstant.VULKAN_GOOGLE_DISPLAY_TIMING): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows using the [VK\_GOOGLE\_display\_timing](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html) Vulkan extension.

This is used for frame pacing to reduce latency, and is generally only available on Android.

This feature does not have a `wgpu`\-level API, and so users of wgpu wishing to use this functionality must access it using various `as_hal` functions, primarily [`Surface::as_hal()`](../wgpu/struct.Surface.html#method.as_hal), to then use.

Supported platforms:

*   Vulkan (with [VK\_GOOGLE\_display\_timing](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html))

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [VULKAN\_EXTERNAL\_MEMORY\_WIN32](#associatedconstant.VULKAN_EXTERNAL_MEMORY_WIN32): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows using the [VK\_KHR\_external\_memory\_win32](https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_memory_win32.html) Vulkan extension.

Supported platforms:

*   Vulkan (with [VK\_KHR\_external\_memory\_win32](https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_memory_win32.html))

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_INT64\_ATOMIC](#associatedconstant.TEXTURE_INT64_ATOMIC): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables R64Uint image atomic min and max.

Supported platforms:

*   Vulkan (with VK\_EXT\_shader\_image\_atomic\_int64)
*   DX12 (with SM 6.6+)
*   Metal (with MSL 3.1+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [UNIFORM\_BUFFER\_BINDING\_ARRAYS](#associatedconstant.UNIFORM_BUFFER_BINDING_ARRAYS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows uniform buffers to be bound as binding arrays.

This allows:

*   Shaders to contain `var<uniform> buffer: binding_array<UniformBuffer>;`
*   The `count` field of `BindGroupLayoutEntry`s with `Uniform` buffers, to be set to `Some`.

Supported platforms:

*   None ([https://github.com/gfx-rs/wgpu/issues/7149](https://github.com/gfx-rs/wgpu/issues/7149))

Potential Platforms:

*   DX12
*   Metal
*   Vulkan 1.2+ (or VK\_EXT\_descriptor\_indexing)’s `shaderUniformBufferArrayNonUniformIndexing` feature)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXPERIMENTAL\_MESH\_SHADER](#associatedconstant.EXPERIMENTAL_MESH_SHADER): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables mesh shaders and task shaders in mesh shader pipelines. This extension does NOT imply support for compiling mesh shaders at runtime.

Supported platforms:

*   Vulkan (with [VK\_EXT\_mesh\_shader](https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_mesh_shader.html))
*   DX12
*   Metal

Naga is only supported on vulkan. On other platforms you will have to use passthrough shaders.

It is recommended to use [`Device::create_shader_module_trusted`](https://docs.rs/wgpu/latest/wgpu/struct.Device.html#method.create_shader_module_trusted) with [`ShaderRuntimeChecks::unchecked()`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/shader/struct.ShaderRuntimeChecks.html#method.unchecked "associated function wgpu_types::shader::ShaderRuntimeChecks::unchecked") to avoid workgroup memory zero initialization, which can be expensive due to zero initialization being single-threaded currently.

Some Mesa drivers including LLVMPIPE but not RADV fail to run the naga generated code. [This may be our bug and will be investigated.](https://github.com/gfx-rs/wgpu/issues/8727) However, due to the nature of the failure, the fact that it is unique, and the random changes that make it go away, this is believed to be a Mesa bug. See [this Mesa issue.](https://gitlab.freedesktop.org/mesa/mesa/-/issues/14376)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXPERIMENTAL\_RAY\_HIT\_VERTEX\_RETURN](#associatedconstant.EXPERIMENTAL_RAY_HIT_VERTEX_RETURN): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

_**THIS IS EXPERIMENTAL:**_ Features enabled by this may have major bugs in them and are expected to be subject to breaking changes, suggestions for the API exposed by this should be posted on [the ray-tracing issue](https://github.com/gfx-rs/wgpu/issues/6762)

Allows for returning of the hit triangle’s vertex position when tracing with an acceleration structure marked with [`AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/struct.AccelerationStructureFlags.html#associatedconstant.ALLOW_RAY_HIT_VERTEX_RETURN "associated constant wgpu_types::ray_tracing::AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN").

Supported platforms:

*   Vulkan

This is a native only feature

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXPERIMENTAL\_MESH\_SHADER\_MULTIVIEW](#associatedconstant.EXPERIMENTAL_MESH_SHADER_MULTIVIEW): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables multiview in mesh shader pipelines

Supported platforms:

*   Vulkan (with [VK\_EXT\_mesh\_shader](https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_mesh_shader.html))

Potential Platforms:

*   DX12
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXTENDED\_ACCELERATION\_STRUCTURE\_VERTEX\_FORMATS](#associatedconstant.EXTENDED_ACCELERATION_STRUCTURE_VERTEX_FORMATS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows usage of additional vertex formats in [BlasTriangleGeometrySizeDescriptor::vertex\_format](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/struct.BlasTriangleGeometrySizeDescriptor.html "struct wgpu_types::ray_tracing::BlasTriangleGeometrySizeDescriptor")

Supported platforms

*   Vulkan
*   DX12

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [PASSTHROUGH\_SHADERS](#associatedconstant.PASSTHROUGH_SHADERS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables creating shaders from passthrough with reflection info (unsafe)

Allows using [`Device::create_shader_module_passthrough`](../wgpu/struct.Device.html#method.create_shader_module_passthrough). Shader code isn’t parsed or interpreted in any way. It is the user’s responsibility to ensure the code and reflection (if passed) are correct.

Supported platforms

*   Vulkan
*   DX12
*   Metal
*   WebGPU

Ideally, in the future, all platforms will be supported. For more info, see [this comment](https://github.com/gfx-rs/wgpu/issues/3103#issuecomment-2833058367).

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_BARYCENTRICS](#associatedconstant.SHADER_BARYCENTRICS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables shader barycentric coordinates.

Supported platforms:

*   Vulkan (with VK\_KHR\_fragment\_shader\_barycentric)
*   DX12 (with SM 6.1+)
*   Metal (with MSL 2.2+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SELECTIVE\_MULTIVIEW](#associatedconstant.SELECTIVE_MULTIVIEW): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables using multiview where not all texture array layers are rendered to in a single render pass/render pipeline. Making use of this feature also requires enabling `Features::MULTIVIEW`.

Supported platforms

*   Vulkan
*   DX12

While metal supports this in theory, the behavior of `view_index` differs from vulkan and dx12 so the feature isn’t exposed.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXPERIMENTAL\_MESH\_SHADER\_POINTS](#associatedconstant.EXPERIMENTAL_MESH_SHADER_POINTS): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables the use of point-primitive outputs from mesh shaders. Making use of this feature also requires enabling `Features::EXPERIMENTAL_MESH_SHADER`.

Supported platforms

*   Vulkan
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [MULTISAMPLE\_ARRAY](#associatedconstant.MULTISAMPLE_ARRAY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables creating texture arrays that are also multisampled.

Without this feature, you cannot create a texture that has both a `sample_count` higher than 1, and a `depth_or_array_layers` higher than 1.

Supported platforms:

*   Vulkan (except VK\_KHR\_portability\_subset if multisampleArrayImage is not available)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [EXPERIMENTAL\_COOPERATIVE\_MATRIX](#associatedconstant.EXPERIMENTAL_COOPERATIVE_MATRIX): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables cooperative matrix operations (also known as tensor cores on NVIDIA GPUs or simdgroup matrix operations on Apple GPUs).

Cooperative matrices allow a workgroup to collectively load, store, and perform matrix multiply-accumulate operations on small tiles of data, enabling hardware-accelerated matrix math.

**Current limitations:** The implementation currently only supports 8x8 f32 matrices. On Vulkan, support is determined by querying `vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR` for configurations matching 8x8x8 f32. Most Vulkan implementations (NVIDIA, AMD) primarily support f16 inputs at larger sizes (e.g., 16x16), so Vulkan support may be limited.

Supported platforms:

*   Metal (with MSL 2.3+ and Apple7+/Mac2+, using simdgroup matrix operations)
*   Vulkan (with [VK\_KHR\_cooperative\_matrix](https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_cooperative_matrix.html), if 8x8 f32 is supported)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_PER\_VERTEX](#associatedconstant.SHADER_PER_VERTEX): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables shader per-vertex attributes.

Supported platforms:

*   Vulkan (with VK\_KHR\_fragment\_shader\_barycentric)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_DRAW\_INDEX](#associatedconstant.SHADER_DRAW_INDEX): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables shader `draw_index` builtin.

Supported platforms:

*   GLES
*   Vulkan

Potential platforms:

*   DX12
*   Metal

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [ACCELERATION\_STRUCTURE\_BINDING\_ARRAY](#associatedconstant.ACCELERATION_STRUCTURE_BINDING_ARRAY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the user to create arrays of acceleration structures in shaders:

ex.

*   `var tlas: binding_array<acceleration_structure, 10>` (WGSL)

This capability allows them to exist and to be indexed by dynamically uniform values.

Supported platforms:

*   DX12
*   Vulkan

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [MEMORY\_DECORATION\_COHERENT](#associatedconstant.MEMORY_DECORATION_COHERENT): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables the `@coherent` memory decoration on storage buffer variables.

Backend mapping:

*   Vulkan
*   DX12
*   Metal (3.2+)
*   GLES (ES 3.1+ / GL 4.3+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [MEMORY\_DECORATION\_VOLATILE](#associatedconstant.MEMORY_DECORATION_VOLATILE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables the `@volatile` memory decoration on storage buffer variables.

Backend mapping:

*   Vulkan
*   GLES (ES 3.1+ / GL 4.3+)

This is a native only feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [DEPTH\_CLIP\_CONTROL](#associatedconstant.DEPTH_CLIP_CONTROL): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

By default, polygon depth is clipped to 0-1 range before/during rasterization. Anything outside of that range is rejected, and respective fragments are not touched.

With this extension, we can disabling clipping. That allows shadow map occluders to be rendered into a tighter depth range.

Supported platforms:

*   desktops
*   some mobile chips
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [DEPTH32FLOAT\_STENCIL8](#associatedconstant.DEPTH32FLOAT_STENCIL8): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for explicit creation of textures of format [`TextureFormat::Depth32FloatStencil8`](enum.TextureFormat.html#variant.Depth32FloatStencil8 "variant bevy::render::render_resource::TextureFormat::Depth32FloatStencil8")

Supported platforms:

*   Vulkan (mostly)
*   DX12
*   Metal
*   OpenGL
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_COMPRESSION\_BC](#associatedconstant.TEXTURE_COMPRESSION_BC): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables BCn family of compressed textures. All BCn textures use 4x4 pixel blocks with 8 or 16 bytes per block.

Compressed textures sacrifice some quality in exchange for significantly reduced bandwidth usage.

Support for this feature guarantees availability of \[`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`\] for BCn formats. [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`](struct.WgpuFeatures.html#associatedconstant.TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES "associated constant bevy::render::render_resource::WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES") may enable additional usages.

This feature guarantees availability of sliced-3d textures for BC formats when combined with TEXTURE\_COMPRESSION\_BC\_SLICED\_3D.

Supported Platforms:

*   desktops
*   Mobile (All Apple9 and some Apple7 and Apple8 devices)
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_COMPRESSION\_BC\_SLICED\_3D](#associatedconstant.TEXTURE_COMPRESSION_BC_SLICED_3D): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the 3d dimension for textures with BC compressed formats.

This feature must be used in combination with TEXTURE\_COMPRESSION\_BC to enable 3D textures with BC compression. It does not enable the BC formats by itself.

Supported Platforms:

*   desktops
*   Mobile (All Apple9 and some Apple7 and Apple8 devices)
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_COMPRESSION\_ETC2](#associatedconstant.TEXTURE_COMPRESSION_ETC2): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables ETC family of compressed textures. All ETC textures use 4x4 pixel blocks. ETC2 RGB and RGBA1 are 8 bytes per block. RTC2 RGBA8 and EAC are 16 bytes per block.

Compressed textures sacrifice some quality in exchange for significantly reduced bandwidth usage.

Support for this feature guarantees availability of \[`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`\] for ETC2 formats. [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`](struct.WgpuFeatures.html#associatedconstant.TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES "associated constant bevy::render::render_resource::WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES") may enable additional usages.

Supported Platforms:

*   Vulkan on Intel
*   Mobile (some)
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_COMPRESSION\_ASTC](#associatedconstant.TEXTURE_COMPRESSION_ASTC): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables ASTC family of compressed textures. ASTC textures use pixel blocks varying from 4x4 to 12x12. Blocks are always 16 bytes.

Compressed textures sacrifice some quality in exchange for significantly reduced bandwidth usage.

Support for this feature guarantees availability of \[`TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING`\] for ASTC formats with Unorm/UnormSrgb channel type. [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`](struct.WgpuFeatures.html#associatedconstant.TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES "associated constant bevy::render::render_resource::WgpuFeatures::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES") may enable additional usages.

This feature does not guarantee availability of sliced 3d textures for ASTC formats. If available, 3d support can be enabled by TEXTURE\_COMPRESSION\_ASTC\_SLICED\_3D feature.

Supported Platforms:

*   Vulkan on Intel
*   Mobile (some)
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TEXTURE\_COMPRESSION\_ASTC\_SLICED\_3D](#associatedconstant.TEXTURE_COMPRESSION_ASTC_SLICED_3D): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the 3d dimension for textures with ASTC compressed formats.

This feature must be used in combination with TEXTURE\_COMPRESSION\_ASTC to enable 3D textures with ASTC compression. It does not enable the ASTC formats by itself.

Supported Platforms:

*   Vulkan (some)
*   Metal on Apple3+
*   OpenGL/WebGL (some)
*   WebGPU

Not Supported:

*   DX12

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [TIMESTAMP\_QUERY](#associatedconstant.TIMESTAMP_QUERY): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables use of Timestamp Queries. These queries tell the current gpu timestamp when all work before the query is finished.

This feature allows the use of

*   [`RenderPassDescriptor::timestamp_writes`](../wgpu/struct.RenderPassDescriptor.html#structfield.timestamp_writes)
*   [`ComputePassDescriptor::timestamp_writes`](../wgpu/struct.ComputePassDescriptor.html#structfield.timestamp_writes) to write out timestamps.

For arbitrary timestamp write commands on encoders refer to [`Features::TIMESTAMP_QUERY_INSIDE_ENCODERS`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY_INSIDE_ENCODERS "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY_INSIDE_ENCODERS"). For arbitrary timestamp write commands on passes refer to [`Features::TIMESTAMP_QUERY_INSIDE_PASSES`](struct.WgpuFeatures.html#associatedconstant.TIMESTAMP_QUERY_INSIDE_PASSES "associated constant bevy::render::render_resource::WgpuFeatures::TIMESTAMP_QUERY_INSIDE_PASSES").

They must be resolved using [`CommandEncoder::resolve_query_set`](../wgpu/struct.CommandEncoder.html#method.resolve_query_set) into a buffer, then the result must be multiplied by the timestamp period [`Queue::get_timestamp_period`](../wgpu/struct.Queue.html#method.get_timestamp_period) to get the timestamp in nanoseconds. Multiple timestamps can then be diffed to get the time for operations between them to finish.

Supported Platforms:

*   Vulkan
*   DX12
*   Metal
*   OpenGL (with GL\_ARB\_timer\_query)
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [INDIRECT\_FIRST\_INSTANCE](#associatedconstant.INDIRECT_FIRST_INSTANCE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows non-zero value for the `first_instance` member in indirect draw calls.

If this feature is not enabled, and the `first_instance` member is non-zero, the behavior may be:

*   The draw call is ignored.
*   The draw call is executed as if the `first_instance` is zero.
*   The draw call is executed with the correct `first_instance` value.

Supported Platforms:

*   Vulkan (mostly)
*   DX12
*   Metal on Apple3+ or Mac1+
*   OpenGL (Desktop 4.2+ with ARB\_shader\_draw\_parameters only)
*   WebGPU

Not Supported:

*   OpenGL ES / WebGL

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [SHADER\_F16](#associatedconstant.SHADER_F16): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows shaders to use 16-bit floating point types. You may use them uniform buffers, storage buffers, and local variables. You may not use them in immediates.

In order to use this in WGSL shaders, you must add `enable f16;` to the top of your shader, before any global items.

Supported Platforms:

*   Vulkan
*   Metal
*   DX12
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [RG11B10UFLOAT\_RENDERABLE](#associatedconstant.RG11B10UFLOAT_RENDERABLE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows for usage of textures of format [`TextureFormat::Rg11b10Ufloat`](enum.TextureFormat.html#variant.Rg11b10Ufloat "variant bevy::render::render_resource::TextureFormat::Rg11b10Ufloat") as a render target

Supported platforms:

*   Vulkan
*   DX12
*   Metal
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [BGRA8UNORM\_STORAGE](#associatedconstant.BGRA8UNORM_STORAGE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the [`TextureUsages::STORAGE_BINDING`](struct.TextureUsages.html#associatedconstant.STORAGE_BINDING "associated constant bevy::render::render_resource::TextureUsages::STORAGE_BINDING") usage on textures with format [`TextureFormat::Bgra8Unorm`](enum.TextureFormat.html#variant.Bgra8Unorm "variant bevy::render::render_resource::TextureFormat::Bgra8Unorm")

Supported Platforms:

*   Vulkan
*   DX12
*   Metal
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [FLOAT32\_FILTERABLE](#associatedconstant.FLOAT32_FILTERABLE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows textures with formats “r32float”, “rg32float”, and “rgba32float” to be filterable.

Supported Platforms:

*   Vulkan (mainly on Desktop GPUs)
*   DX12
*   Metal on macOS or Apple9+ GPUs, optional on iOS/iPadOS with Apple7/8 GPUs
*   GL with one of `GL_ARB_color_buffer_float`/`GL_EXT_color_buffer_float`/`OES_texture_float_linear`
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [FLOAT32\_BLENDABLE](#associatedconstant.FLOAT32_BLENDABLE): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows textures with formats “r32float”, “rg32float”, and “rgba32float” to be blendable.

Supported Platforms:

*   Vulkan
*   WebGPU

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [DUAL\_SOURCE\_BLENDING](#associatedconstant.DUAL_SOURCE_BLENDING): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows two outputs from a shader to be used for blending. Note that dual-source blending doesn’t support multiple render targets.

For more info see the OpenGL ES extension GL\_EXT\_blend\_func\_extended.

Supported platforms:

*   OpenGL ES (with GL\_EXT\_blend\_func\_extended)
*   Metal (with MSL 1.2+)
*   Vulkan (with dualSrcBlend)
*   DX12
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [CLIP\_DISTANCES](#associatedconstant.CLIP_DISTANCES): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the use of `@builtin(clip_distances)` in WGSL.

Supported platforms:

*   Vulkan (mainly on Desktop GPUs)
*   Metal
*   GL (Desktop or `GL_EXT_clip_cull_distance`)
*   WebGPU

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [IMMEDIATES](#associatedconstant.IMMEDIATES): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Allows the use of immediate data: small, fast bits of memory that can be updated inside a [`RenderPass`](../wgpu/struct.RenderPass.html).

Allows the user to call [`RenderPass::set_immediates`](../wgpu/struct.RenderPass.html#method.set_immediates), provide a non-zero immediate data size to [`PipelineLayoutDescriptor`](../wgpu/struct.PipelineLayoutDescriptor.html), and provide a non-zero limit to [`Limits::max_immediate_size`](struct.WgpuLimits.html "struct bevy::render::render_resource::WgpuLimits").

A block of immediate data can be declared in WGSL with `var<immediate>`:

[ⓘ](# "This example is not tested")

```rust
struct Immediates { example: f32, }
var<immediate> c: Immediates;
```

In GLSL, this corresponds to `layout(immediates) uniform Name {..}`.

Supported platforms:

*   DX12
*   Vulkan
*   Metal
*   OpenGL (emulated with uniforms)
*   WebGPU

WebGPU support is currently a proposal and will be available in browsers in the future.

This is a web and native feature.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const [PRIMITIVE\_INDEX](#associatedconstant.PRIMITIVE_INDEX): [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Enables `builtin(primitive_index)` in fragment shaders.

Note: enables geometry processing for pipelines using the builtin. This may come with a significant performance impact on some hardware. Other pipelines are not affected.

Supported platforms:

*   Vulkan (with geometryShader)
*   DX12
*   Metal (some)
*   OpenGL (some)

This is a web and native feature. `primitive-index` is its WebGPU-defined name, and `shader-primitive-index` is accepted to remain compatible with previous wgpu behavior.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [bits](#method.bits)(&self) -> [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")

Gets the set flags as a container holding an array of bits.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [empty](#method.empty)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Returns self with no flags set.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [all](#method.all)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Returns self with all flags set.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [contains](#method.contains)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all the bits set in `other` are all set in `self`

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/texture\_binding\_array.rs ([line 82](../../../src/texture_binding_array/texture_binding_array.rs.html#82))

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

examples/3d/occlusion\_culling.rs ([line 143](../../../src/occlusion_culling/occlusion_culling.rs.html#143))

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

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [intersects](#method.intersects)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether any bit set in `self` matched any bit set in `other`.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [is\_empty](#method.is_empty)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether there is no flag set.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [is\_all](#method.is_all)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether the struct has all flags set.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [union](#method.union)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Bitwise or - `self | other`

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [intersection](#method.intersection)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Bitwise and - `self & other`

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [difference](#method.difference)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Bitwise and of the complement of other - `self & !other`

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [symmetric\_difference](#method.symmetric_difference)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Bitwise xor - `self ^ other`

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [complement](#method.complement)(self) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Bitwise not - `!self`

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [set](#method.set)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"), set: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Calls [`Self::insert`](struct.WgpuFeatures.html#method.insert "method bevy::render::render_resource::WgpuFeatures::insert") if `set` is true and otherwise calls [`Self::remove`](struct.WgpuFeatures.html#method.remove "method bevy::render::render_resource::WgpuFeatures::remove").

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [insert](#method.insert)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"))

Inserts specified flag(s) into self

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [remove](#method.remove)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"))

Removes specified flag(s) from self

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [toggle](#method.toggle)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"))

Toggles specified flag(s) in self

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [from\_bits](#method.from_bits)(bits: [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")\>

Takes in [`FeatureBits`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits") and returns None if there are invalid bits or otherwise Self with those bits set

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [from\_bits\_truncate](#method.from_bits_truncate)(bits: [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Takes in [`FeatureBits`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits") and returns Self with only valid bits (all other bits removed)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [from\_bits\_retain](#method.from_bits_retain)(bits: [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Takes in [`FeatureBits`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits") and returns Self with all bits that were set without removing invalid bits

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [from\_name](#method.from_name)(name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")\>

Takes in a bitflags flag name (in `SCREAMING_SNAKE_CASE`) and returns Self if it matches or none if the name does not match the name of any of the flags. Name is capitalisation dependent.

\[`impl FromStr`\] can be used to recognize kebab-case names, like are used in the WebGPU spec.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [from\_internal\_flags](#method.from_internal_flags)( features\_wgpu: [FeaturesWGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWGPU.html "struct wgpu_types::features::FeaturesWGPU"), features\_webgpu: [FeaturesWebGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWebGPU.html "struct wgpu_types::features::FeaturesWebGPU"), ) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Combines the features from the internal flags into the entire features struct

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [iter](#method.iter)(&self) -> [Iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")\> [ⓘ](#)

Returns an iterator over the set flags.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub const fn [iter\_names](#method.iter_names)(&self) -> [IterNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html "struct bitflags::iter::IterNames")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")\> [ⓘ](#)

Returns an iterator over the set flags and their names.

These are bitflags names in `SCREAMING_SNAKE_CASE`.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### pub fn [as\_str](#method.as_str)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

If the argument is a single [`Features`](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures") flag, returns the corresponding `kebab-case` feature name, otherwise `None`.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#1791)

### impl [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#1794)

#### pub const fn [all\_webgpu\_mask](#method.all_webgpu_mask)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Mask of all features which are part of the upstream WebGPU standard.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#1803)

#### pub const fn [all\_native\_mask](#method.all_native_mask)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Mask of all features that are only available when targeting native (not web).

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#1812)

#### pub const fn [all\_experimental\_mask](#method.all_experimental_mask)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Mask of all features which are experimental.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#1826)

#### pub fn [allowed\_vertex\_formats\_for\_blas](#method.allowed_vertex_formats_for_blas)(&self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[VertexFormat](../../mesh/enum.VertexFormat.html "enum bevy::mesh::VertexFormat")\>

Vertex formats allowed for creating and building BLASes

## Trait Implementations

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Available on **crate feature `serde`** only.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [Flags](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html "trait bitflags::traits::Flags") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### const [FLAGS](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedconstant.FLAGS): &'static \[[Flag](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/struct.Flag.html "struct bitflags::traits::Flag")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")\>\] = Features::FLAGS

The set of defined flags.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits) = [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")

The underlying bits type.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits)(&self) -> [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")

Get the underlying bits value. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [from\_bits\_retain](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.from_bits_retain)(bits: [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Convert from a bits value exactly.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [empty](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.empty)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Get a flags value with all bits unset.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [all](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all)() -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Get a flags value with all known bits set.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#158)

#### fn [all\_named](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all_named)() -> Self

Get a flags value with all bits from named flags set. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all_named)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#168)

#### fn [known\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.known_bits)(&self) -> Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")

Get the known bits from a flags value.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#173)

#### fn [unknown\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.unknown_bits)(&self) -> Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")

Get the unknown bits from a flags value.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#178)

#### fn [contains\_unknown\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.contains_unknown_bits)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method will return `true` if any unknown bits are set.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#190)

#### fn [from\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits)(bits: Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

Convert from a bits value. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#201)

#### fn [from\_bits\_truncate](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits_truncate)(bits: Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")) -> Self

Convert from a bits value, unsetting any unknown bits.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#212)

#### fn [from\_name](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name)(name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

Get a flags value with the bits of a flag with the given name set. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#231)

#### fn [iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter)(&self) -> [Iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter")<Self> [ⓘ](#)

Yield a set of contained flags values. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#239)

#### fn [iter\_names](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names)(&self) -> [IterNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html "struct bitflags::iter::IterNames")<Self> [ⓘ](#)

Yield a set of contained named flags values. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#244)

#### fn [iter\_defined\_names](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_defined_names)() -> [IterDefinedNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterDefinedNames.html "struct bitflags::iter::IterDefinedNames")<Self> [ⓘ](#)

Yield a set of all named flags defined by [`Self::FLAGS`](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedconstant.FLAGS "associated constant bitflags::traits::Flags::FLAGS").

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#252)

#### fn [iter\_equal\_names](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_equal_names)(&self) -> [IterEqualNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterEqualNames.html "struct bitflags::iter::IterEqualNames")<Self> [ⓘ](#)

Get an iterator over all defined names for this flags value. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_equal_names)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#257)

#### fn [is\_empty](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all bits in this flags value are unset.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#262)

#### fn [is\_all](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.is_all)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all known bits in this flags value are set.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#269-271)

#### fn [intersects](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.intersects)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Whether any set bits in `other` are also set in `self`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#277-279)

#### fn [contains](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.contains)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Whether all set bits in `other` are also set in `self`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#285-287)

#### fn [truncate](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.truncate)(&mut self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove any unknown bits from the flags.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#293-295)

#### fn [insert](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert)(&mut self, other: Self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#304-306)

#### fn [remove](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove)(&mut self, other: Self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The intersection of `self` with the complement of `other` (`&!`). [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#312-314)

#### fn [toggle](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.toggle)(&mut self, other: Self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#320-322)

#### fn [set](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.set)(&mut self, other: Self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Call [`Flags::insert`](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert "method bitflags::traits::Flags::insert") when `value` is `true` or [`Flags::remove`](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove "method bitflags::traits::Flags::remove") when `value` is `false`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#332-334)

#### fn [clear](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.clear)(&mut self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Unsets all bits in the flags.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#341)

#### fn [intersection](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.intersection)(self, other: Self) -> Self

The bitwise and (`&`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#347)

#### fn [union](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.union)(self, other: Self) -> Self

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#356)

#### fn [difference](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference)(self, other: Self) -> Self

The intersection of `self` with the complement of `other` (`&!`). [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#362)

#### fn [symmetric\_difference](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.symmetric_difference)(self, other: Self) -> Self

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#368)

#### fn [complement](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.complement)(self) -> Self

The bitwise negation (`!`) of the bits in `self`, truncating the result.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#565)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")\> for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#566)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [FeatureBits](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeatureBits.html "struct wgpu_types::features::FeatureBits")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Converts to this type from the input type.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[FeaturesWGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWGPU.html "struct wgpu_types::features::FeaturesWGPU")\> for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(features\_wgpu: [FeaturesWGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWGPU.html "struct wgpu_types::features::FeaturesWGPU")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Converts to this type from the input type.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[FeaturesWebGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWebGPU.html "struct wgpu_types::features::FeaturesWebGPU")\> for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(features\_webgpu: [FeaturesWebGPU](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/features/struct.FeaturesWebGPU.html "struct wgpu_types::features::FeaturesWebGPU")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Converts to this type from the input type.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

The associated error which can be returned from parsing.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [from\_str](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)(s: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures"), <[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures") as [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")\>::[Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "type core::str::traits::FromStr::Err")\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Available on **crate feature `serde`** only.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#590)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/features.rs.html#577-1789)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, other: [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")) -> [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Features](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

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

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

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

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

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

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

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

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)

### impl<T> [ToSmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html "trait smol_str::ToSmolStr") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)

#### fn [to\_smolstr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html#tymethod.to_smolstr)(&self) -> [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2900)

### impl<T> [ToString](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](../../prelude/trait.ToString.html#tymethod.to_string)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](../../prelude/trait.ToString.html#tymethod.to_string)

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<Features>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","Iter<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","IterDefinedNames<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterDefinedNames.html\\" title=\\"struct bitflags::iter::IterDefinedNames\\">IterDefinedNames</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterDefinedNames.html\\" title=\\"struct bitflags::iter::IterDefinedNames\\">IterDefinedNames</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, B);</div>","IterEqualNames<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterEqualNames.html\\" title=\\"struct bitflags::iter::IterEqualNames\\">IterEqualNames</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterEqualNames.html\\" title=\\"struct bitflags::iter::IterEqualNames\\">IterEqualNames</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","IterNames<Features>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, B);</div>","IterNames<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, B);</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}