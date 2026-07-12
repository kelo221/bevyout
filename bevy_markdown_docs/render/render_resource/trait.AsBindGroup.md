[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait AsBindGroup 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#500)

```rust
pub trait AsBindGroup {
    type Data: Send + Sync;
    type Param: SystemParam + 'static;

    // Required methods
    fn label() -> &'static str;
    fn bind_group_data(&self) -> Self::Data;
    fn unprepared_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
        force_no_bindless: bool,
    ) -> Result<UnpreparedBindGroup, AsBindGroupError>;
    fn bind_group_layout_entries(
        render_device: &RenderDevice,
        force_no_bindless: bool,
    ) -> Vec<BindGroupLayoutEntry>
       where Self: Sized;

    // Provided methods
    fn bindless_slot_count() -> Option<BindlessSlabResourceLimit> { ... }
    fn bindless_supported(_: &RenderDevice) -> bool { ... }
    fn as_bind_group(
        &self,
        layout_descriptor: &BindGroupLayoutDescriptor,
        render_device: &RenderDevice,
        pipeline_cache: &PipelineCache,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) -> Result<PreparedBindGroup, AsBindGroupError> { ... }
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout
       where Self: Sized { ... }
    fn bind_group_layout_descriptor(
        render_device: &RenderDevice,
    ) -> BindGroupLayoutDescriptor
       where Self: Sized { ... }
    fn bindless_descriptor() -> Option<BindlessDescriptor> { ... }
}
```

Converts a value to a [`BindGroup`](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup") with a given [`BindGroupLayout`](struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout"), which can then be used in Bevy shaders. This trait can be derived (and generally should be). Read on for details and examples.

This is an opinionated trait that is intended to make it easy to generically convert a type into a [`BindGroup`](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup"). It provides access to specific render resources, such as [`RenderAssets<GpuImage>`](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets") and [`crate::texture::FallbackImage`](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage"). If a type has a [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle"), these can be used to retrieve the corresponding [`Texture`](struct.Texture.html "struct bevy::render::render_resource::Texture") resource.

[`AsBindGroup::as_bind_group`](trait.AsBindGroup.html#method.as_bind_group "method bevy::render::render_resource::AsBindGroup::as_bind_group") is intended to be called once, then the result cached somewhere. It is generally ok to do “expensive” work here, such as creating a [`Buffer`](struct.Buffer.html "struct bevy::render::render_resource::Buffer") for a uniform.

If for some reason a [`BindGroup`](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup") cannot be created yet (for example, the [`Texture`](struct.Texture.html "struct bevy::render::render_resource::Texture") for an [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") hasn’t loaded yet), just return [`AsBindGroupError::RetryNextUpdate`](enum.AsBindGroupError.html#variant.RetryNextUpdate "variant bevy::render::render_resource::AsBindGroupError::RetryNextUpdate"), which signals that the caller should retry again later.

## Deriving

This trait can be derived. Field attributes like `uniform` and `texture` are used to define which fields should be bindings, what their binding type is, and what index they should be bound at:

```rust
#[derive(AsBindGroup)]
struct CoolMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
    #[storage(3, read_only)]
    storage_buffer: Handle<ShaderBuffer>,
    #[storage(4, read_only, buffer)]
    raw_buffer: Buffer,
    #[storage_texture(5)]
    storage_texture: Handle<Image>,
}
```

In WGSL shaders, the binding would look like this:

```
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<storage> storage_buffer: array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<storage> raw_buffer: array<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var storage_texture: texture_storage_2d<rgba8unorm, read_write>;
```

Note that the “group” index is determined by the usage context. It is not defined in [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup"). For example, in Bevy material bind groups are generally bound to group 2.

The following field-level attributes are supported:

### `uniform(BINDING_INDEX)`

*   The field will be converted to a shader-compatible type using the [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") trait, written to a [`Buffer`](struct.Buffer.html "struct bevy::render::render_resource::Buffer"), and bound as a uniform. [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") is implemented for most math types already, such as [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32"), [`Vec4`](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), and [`LinearRgba`](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba"). It can also be derived for custom structs.

### `texture(BINDING_INDEX, arguments)`

*   This field’s [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") will be used to look up the matching [`Texture`](struct.Texture.html "struct bevy::render::render_resource::Texture") GPU resource, which will be bound as a texture in shaders. The field will be assumed to implement [`Into<Option<Handle<Image>>>`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into"). In practice, most fields should be a [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") or [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"). If the value of an [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the [`crate::texture::FallbackImage`](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage") resource will be used instead. This attribute can be used in conjunction with a `sampler` binding attribute (with a different binding index) if a binding of the sampler for the [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") is also required.

| Arguments | Values | Default |
| --- | --- | --- |
| `dimension` = “…” | `"1d"`, `"2d"`, `"2d_array"`, `"3d"`, `"cube"`, `"cube_array"` | `"2d"` |
| `sample_type` = “…” | `"float"`, `"depth"`, `"s_int"` or `"u_int"` | `"float"` |
| `filterable` = … | `true`, `false` | `true` |
| `multisampled` = … | `true`, `false` | `false` |
| `visibility(...)` | `all`, `none`, or a list-combination of `vertex`, `fragment`, `compute` | `vertex`, `fragment` |

### `storage_texture(BINDING_INDEX, arguments)`

*   This field’s [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") will be used to look up the matching [`Texture`](struct.Texture.html "struct bevy::render::render_resource::Texture") GPU resource, which will be bound as a storage texture in shaders. The field will be assumed to implement [`Into<Option<Handle<Image>>>`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into"). In practice, most fields should be a [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") or [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"). If the value of an [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the [`crate::texture::FallbackImage`](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage") resource will be used instead.

| Arguments | Values | Default |
| --- | --- | --- |
| `dimension` = “…” | `"1d"`, `"2d"`, `"2d_array"`, `"3d"`, `"cube"`, `"cube_array"` | `"2d"` |
| `image_format` = … | any member of [`TextureFormat`](enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") | `Rgba8Unorm` |
| `access` = … | any member of [`StorageTextureAccess`](enum.StorageTextureAccess.html "enum bevy::render::render_resource::StorageTextureAccess") | `ReadWrite` |
| `visibility(...)` | `all`, `none`, or a list-combination of `vertex`, `fragment`, `compute` | `compute` |

### `sampler(BINDING_INDEX, arguments)`

*   This field’s [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") will be used to look up the matching [`Sampler`](struct.Sampler.html "struct bevy::render::render_resource::Sampler") GPU resource, which will be bound as a sampler in shaders. The field will be assumed to implement [`Into<Option<Handle<Image>>>`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into"). In practice, most fields should be a [`Handle<Image>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") or [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"). If the value of an [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the [`crate::texture::FallbackImage`](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage") resource will be used instead. This attribute can be used in conjunction with a `texture` binding attribute (with a different binding index) if a binding of the texture for the [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") is also required.

| Arguments | Values | Default |
| --- | --- | --- |
| `sampler_type` = “…” | `"filtering"`, `"non_filtering"`, `"comparison"`. | `"filtering"` |
| `visibility(...)` | `all`, `none`, or a list-combination of `vertex`, `fragment`, `compute` | `vertex`, `fragment` |

### `storage(BINDING_INDEX, arguments)`

*   The field’s [`Handle<Storage>`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") will be used to look up the matching [`Buffer`](struct.Buffer.html "struct bevy::render::render_resource::Buffer") GPU resource, which will be bound as a storage buffer in shaders. If the `storage` attribute is used, the field is expected a raw buffer, and the buffer will be bound as a storage buffer in shaders. In bindless mode, `binding_array()` argument that specifies the binding number of the resulting storage buffer binding array must be present.

| Arguments | Values | Default |
| --- | --- | --- |
| `visibility(...)` | `all`, `none`, or a list-combination of `vertex`, `fragment`, `compute` | `vertex`, `fragment` |
| `read_only` | if present then value is true, otherwise false | `false` |
| `buffer` | if present then the field will be assumed to be a raw wgpu buffer |  |
| `binding_array(...)` | the binding number of the binding array, for bindless mode | bindless mode disabled |

Note that fields without field-level binding attributes will be ignored.

```rust
#[derive(AsBindGroup)]
struct CoolMaterial {
    #[uniform(0)]
    color: LinearRgba,
    this_field_is_ignored: String,
}
```

As mentioned above, [`Option<Handle<Image>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") is also supported:

```rust
#[derive(AsBindGroup)]
struct CoolMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}
```

This is useful if you want a texture to be optional. When the value is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the [`crate::texture::FallbackImage`](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage") will be used for the binding instead, which defaults to “pure white”.

Field uniforms with the same index will be combined into a single binding:

```rust
#[derive(AsBindGroup)]
struct CoolMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(0)]
    roughness: f32,
}
```

In WGSL shaders, the binding would look like this:

```
struct CoolMaterial {
    color: vec4<f32>,
    roughness: f32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: CoolMaterial;
```

Some less common scenarios will require “struct-level” attributes. These are the currently supported struct-level attributes:

### `uniform(BINDING_INDEX, ConvertedShaderType)`

*   This also creates a [`Buffer`](struct.Buffer.html "struct bevy::render::render_resource::Buffer") using [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") and binds it as a uniform, much like the field-level `uniform` attribute. The difference is that the entire [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") value is converted to `ConvertedShaderType`, which must implement [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), instead of a specific field implementing [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"). This is useful if more complicated conversion logic is required, or when using bindless mode (see below). The conversion is done using the [`AsBindGroupShaderType<ConvertedShaderType>`](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType") trait, which is automatically implemented if `&Self` implements [`Into<ConvertedShaderType>`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into"). Outside of bindless mode, only use [`AsBindGroupShaderType`](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType") if access to resources like [`RenderAssets<GpuImage>`](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets") is required.
    
*   In bindless mode (see `bindless(COUNT)`), this attribute becomes `uniform(BINDLESS_INDEX, ConvertedShaderType, binding_array(BINDING_INDEX))`. The resulting uniform buffers will be available in the shader as a binding array at the given `BINDING_INDEX`. The `BINDLESS_INDEX` specifies the offset of the buffer in the bindless index table.
    
    For example, suppose that the material slot is stored in a variable named `slot`, the bindless index table is named `material_indices`, and that the first field (index 0) of the bindless index table type is named `material`. Then specifying `#[uniform(0, StandardMaterialUniform, binding_array(10)]` will create a binding array buffer declared in the shader as `var<storage> material_array: binding_array<StandardMaterialUniform>` and accessible as `material_array[material_indices[slot].material]`.
    

### `data(BINDING_INDEX, ConvertedShaderType, binding_array(BINDING_INDEX))`

*   This is very similar to `uniform(BINDING_INDEX, ConvertedShaderType, binding_array(BINDING_INDEX)` and in fact is identical if bindless mode isn’t being used. The difference is that, in bindless mode, the `data` attribute produces a single buffer containing an array, not an array of buffers. For example, suppose you had the following declaration:

[ⓘ](# "This example is not tested")

```rust
#[uniform(0, StandardMaterialUniform, binding_array(10))]
struct StandardMaterial { ... }
```

In bindless mode, this will produce a binding matching the following WGSL declaration:

```
@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<storage> material_array: binding_array<StandardMaterial>;
```

On the other hand, if you write this declaration:

[ⓘ](# "This example is not tested")

```rust
#[data(0, StandardMaterialUniform, binding_array(10))]
struct StandardMaterial { ... }
```

Then Bevy produces a binding that matches this WGSL declaration instead:

```
@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<storage> material_array: array<StandardMaterial>;
```

*   Just as with the structure-level `uniform` attribute, Bevy converts the entire [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") to `ConvertedShaderType`, using the [`AsBindGroupShaderType<ConvertedShaderType>`](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType") trait.
    
*   In non-bindless mode, the structure-level `data` attribute is the same as the structure-level `uniform` attribute and produces a single uniform buffer in the shader. The above example would result in a binding that looks like this in WGSL in non-bindless mode:
    

```
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: StandardMaterial;
```

*   For efficiency reasons, `data` is generally preferred over `uniform` unless you need to place your data in individual buffers.

### `bind_group_data(DataType)`

*   The [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") type will be converted to some `DataType` using [`Into<DataType>`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") and stored as [`AsBindGroup::Data`](trait.AsBindGroup.html#associatedtype.Data "associated type bevy::render::render_resource::AsBindGroup::Data") as part of the [`AsBindGroup::as_bind_group`](trait.AsBindGroup.html#method.as_bind_group "method bevy::render::render_resource::AsBindGroup::as_bind_group") call. This is useful if data needs to be stored alongside the generated bind group, such as a unique identifier for a material’s bind group. The most common use case for this attribute is “shader pipeline specialization”. See [`SpecializedRenderPipeline`](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline").

### `bindless`

*   This switch enables _bindless resources_, which changes the way Bevy supplies resources (textures, and samplers) to the shader. When bindless resources are enabled, and the current platform supports them, Bevy will allocate textures, and samplers into _binding arrays_, separated based on type and will supply your shader with indices into those arrays.
*   Bindless textures and samplers are placed into the appropriate global array defined in `bevy_render::bindless` (`bindless.wgsl`).
*   Bevy doesn’t currently support bindless buffers, except for those created with the `uniform(BINDLESS_INDEX, ConvertedShaderType, binding_array(BINDING_INDEX))` attribute. If you need to include a buffer in your object, and you can’t create the data in that buffer with the `uniform` attribute, consider a non-bindless object instead.
*   If bindless mode is enabled, the `BINDLESS` definition will be available. Because not all platforms support bindless resources, you should check for the presence of this definition via `#ifdef` and fall back to standard bindings if it isn’t present.
*   By default, in bindless mode, binding 0 becomes the _bindless index table_, which is an array of structures, each of which contains as many fields of type `u32` as the highest binding number in the structure annotated with `#[derive(AsBindGroup)]`. Again by default, the _i_th field of the bindless index table contains the index of the resource with binding _i_ within the appropriate binding array.
*   In the case of materials, the index of the applicable table within the bindless index table list corresponding to the mesh currently being drawn can be retrieved with `mesh[in.instance_index].material_and_lightmap_bind_group_slot & 0xffffu`.
*   You can limit the size of the bindless slabs to N resources with the `limit(N)` declaration. For example, `#[bindless(limit(16))]` ensures that each slab will have no more than 16 total resources in it. If you don’t specify a limit, Bevy automatically picks a reasonable one for the current platform.
*   The `index_table(range(M..N), binding(B))` declaration allows you to customize the layout of the bindless index table. This is useful for materials that are composed of multiple bind groups, such as `ExtendedMaterial`. In such cases, there will be multiple bindless index tables, so they can’t both be assigned to binding 0 or their bindings will conflict.
    *   The `binding(B)` attribute of the `index_table` attribute allows you to customize the binding (`@binding(B)`, in the shader) at which the index table will be bound.
    *   The `range(M, N)` attribute of the `index_table` attribute allows you to change the mapping from the field index in the bindless index table to the bindless index. Instead of the field at index $i$ being mapped to the bindless index $i$, with the `range(M, N)` attribute the field at index $i$ in the bindless index table is mapped to the bindless index $i$ + M. The size of the index table will be set to N - M. Note that this may result in the table being too small to contain all the bindless bindings.
*   The purpose of bindless mode is to improve performance by reducing state changes. By grouping resources together into binding arrays, Bevy doesn’t have to modify GPU state as often, decreasing API and driver overhead.
*   See the `shaders/shader_material_bindless` example for an example of how to use bindless mode. See the `shaders/extended_material_bindless` example for a more exotic example of bindless mode that demonstrates the `index_table` attribute.
*   The following diagram illustrates how bindless mode works using a subset of `StandardMaterial`:

```
Shader Bindings                          Sampler Binding Array
    +----+-----------------------------+     +-----------+-----------+-----+
+---|  0 | material_indices            |  +->| sampler 0 | sampler 1 | ... |
|   +----+-----------------------------+  |  +-----------+-----------+-----+
|   |  1 | bindless_samplers_filtering +--+        ^
|   +----+-----------------------------+           +-------------------------------+
|   | .. |            ...              |                                           |
|   +----+-----------------------------+      Texture Binding Array                |
|   |  5 | bindless_textures_2d        +--+  +-----------+-----------+-----+       |
|   +----+-----------------------------+  +->| texture 0 | texture 1 | ... |       |
|   | .. |            ...              |     +-----------+-----------+-----+       |
|   +----+-----------------------------+           ^                               |
|   + 10 | material_array              +--+        +---------------------------+   |
|   +----+-----------------------------+  |                                    |   |
|                                         |   Buffer Binding Array             |   |
|                                         |  +----------+----------+-----+     |   |
|                                         +->| buffer 0 | buffer 1 | ... |     |   |
|    Material Bindless Indices               +----------+----------+-----+     |   |
|   +----+-----------------------------+          ^                            |   |
+-->|  0 | material                    +----------+                            |   |
    +----+-----------------------------+                                       |   |
    |  1 | base_color_texture          +---------------------------------------+   |
    +----+-----------------------------+                                           |
    |  2 | base_color_sampler          +-------------------------------------------+
    +----+-----------------------------+
    | .. |            ...              |
    +----+-----------------------------+
```

The previous `CoolMaterial` example illustrating “combining multiple field-level uniform attributes with the same binding index” can also be equivalently represented with a single struct-level uniform attribute:

```rust
#[derive(AsBindGroup)]
#[uniform(0, CoolMaterialUniform)]
struct CoolMaterial {
    color: LinearRgba,
    roughness: f32,
}

#[derive(ShaderType)]
struct CoolMaterialUniform {
    color: LinearRgba,
    roughness: f32,
}

impl From<&CoolMaterial> for CoolMaterialUniform {
    fn from(material: &CoolMaterial) -> CoolMaterialUniform {
        CoolMaterialUniform {
            color: material.color,
            roughness: material.roughness,
        }
    }
}
```

Setting `bind_group_data` looks like this:

```rust
#[derive(AsBindGroup)]
#[bind_group_data(CoolMaterialKey)]
struct CoolMaterial {
    #[uniform(0)]
    color: LinearRgba,
    is_shaded: bool,
}

// Materials keys are intended to be small, cheap to hash, and
// uniquely identify a specific material permutation.
#[repr(C)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct CoolMaterialKey {
    is_shaded: bool,
}

impl From<&CoolMaterial> for CoolMaterialKey {
    fn from(material: &CoolMaterial) -> CoolMaterialKey {
        CoolMaterialKey {
            is_shaded: material.is_shaded,
        }
    }
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#502)

#### type [Data](#associatedtype.Data): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

Data that will be stored alongside the “prepared” bind group.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#504)

#### type [Param](#associatedtype.Param): [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") + 'static

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#527)

#### fn [label](#tymethod.label)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

label

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#558)

#### fn [bind\_group\_data](#tymethod.bind_group_data)(&self) -> Self::[Data](trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#571-577)

#### fn [unprepared\_bind\_group](#tymethod.unprepared_bind_group)( &self, layout: &[BindGroupLayout](struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout"), render\_device: &[RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"), param: &mut <Self::[Param](trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, force\_no\_bindless: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnpreparedBindGroup](struct.UnpreparedBindGroup.html "struct bevy::render::render_resource::UnpreparedBindGroup"), [AsBindGroupError](enum.AsBindGroupError.html "enum bevy::render::render_resource::AsBindGroupError")\>

Returns a vec of (binding index, `OwnedBindingResource`).

In cases where `OwnedBindingResource` is not available (as for bindless texture arrays currently), an implementor may return `AsBindGroupError::CreateBindGroupDirectly` from this function and instead define `as_bind_group` directly. This may prevent certain features, such as bindless mode, from working correctly.

Set `force_no_bindless` to true to require that bindless textures _not_ be used. `ExtendedMaterial` uses this in order to ensure that the base material doesn’t use bindless mode if the extension doesn’t.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#609-614)

#### fn [bind\_group\_layout\_entries](#tymethod.bind_group_layout_entries)( render\_device: &[RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"), force\_no\_bindless: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[BindGroupLayoutEntry](struct.BindGroupLayoutEntry.html "struct bevy::render::render_resource::BindGroupLayoutEntry")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a vec of bind group layout entries.

Set `force_no_bindless` to true to require that bindless textures _not_ be used. `ExtendedMaterial` uses this in order to ensure that the base material doesn’t use bindless mode if the extension doesn’t.

## Provided Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#513)

#### fn [bindless\_slot\_count](#method.bindless_slot_count)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BindlessSlabResourceLimit](enum.BindlessSlabResourceLimit.html "enum bevy::render::render_resource::BindlessSlabResourceLimit")\>

The number of slots per bind group, if bindless mode is enabled.

If this bind group doesn’t use bindless, then this will be `None`.

Note that the _actual_ slot count may be different from this value, due to platform limitations. For example, if bindless resources aren’t supported on this platform, the actual slot count will be 1.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#522)

#### fn [bindless\_supported](#method.bindless_supported)(\_: &[RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

True if the hardware _actually_ supports bindless textures for this type, taking the device and driver capabilities into account.

If this type doesn’t use bindless textures, then the return value from this function is meaningless.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#530-536)

#### fn [as\_bind\_group](#method.as_bind_group)( &self, layout\_descriptor: &[BindGroupLayoutDescriptor](../../material/descriptor/struct.BindGroupLayoutDescriptor.html "struct bevy::material::descriptor::BindGroupLayoutDescriptor"), render\_device: &[RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"), pipeline\_cache: &[PipelineCache](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache"), param: &mut <Self::[Param](trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[PreparedBindGroup](struct.PreparedBindGroup.html "struct bevy::render::render_resource::PreparedBindGroup"), [AsBindGroupError](enum.AsBindGroupError.html "enum bevy::render::render_resource::AsBindGroupError")\>

Creates a bind group for `self` matching the layout defined in [`AsBindGroup::bind_group_layout`](trait.AsBindGroup.html#method.bind_group_layout "associated function bevy::render::render_resource::AsBindGroup::bind_group_layout").

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#581-583)

#### fn [bind\_group\_layout](#method.bind_group_layout)(render\_device: &[RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")) -> [BindGroupLayout](struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates the bind group layout matching all bind groups returned by [`AsBindGroup::as_bind_group`](trait.AsBindGroup.html#method.as_bind_group "method bevy::render::render_resource::AsBindGroup::as_bind_group")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#594-596)

#### fn [bind\_group\_layout\_descriptor](#method.bind_group_layout_descriptor)( render\_device: &[RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"), ) -> [BindGroupLayoutDescriptor](../../material/descriptor/struct.BindGroupLayoutDescriptor.html "struct bevy::material::descriptor::BindGroupLayoutDescriptor")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates the bind group layout descriptor matching all bind groups returned by [`AsBindGroup::as_bind_group`](trait.AsBindGroup.html#method.as_bind_group "method bevy::render::render_resource::AsBindGroup::as_bind_group") TODO: we only need `RenderDevice` to determine if bindless is supported

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#616)

#### fn [bindless\_descriptor](#method.bindless_descriptor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BindlessDescriptor](struct.BindlessDescriptor.html "struct bevy::render::render_resource::BindlessDescriptor")\>

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [ColorMaterial](../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

#### type [Data](#associatedtype.Data) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [FallbackImage](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")\>>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#82)

### impl [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [ForwardDecalMaterialExt](../../pbr/decal/struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#82)

#### type [Data](#associatedtype.Data) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#82)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [FallbackImage](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")\>>)

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#80)

### impl [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [FrametimeGraphMaterial](../../dev_tools/frame_time_graph/struct.FrametimeGraphMaterial.html "struct bevy::dev_tools::frame_time_graph::FrametimeGraphMaterial")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#80)

#### type [Data](#associatedtype.Data) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#80)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [FallbackImage](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")\>>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [SpriteMaterial](../../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

#### type [Data](#associatedtype.Data) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [FallbackImage](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")\>>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [StandardMaterial](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

#### type [Data](#associatedtype.Data) = [StandardMaterialKey](../../pbr/struct.StandardMaterialKey.html "struct bevy::pbr::StandardMaterialKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [FallbackImage](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")\>>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#27)

### impl [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [TilemapChunkMaterial](../../sprite_render/struct.TilemapChunkMaterial.html "struct bevy::sprite_render::TilemapChunkMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#27)

#### type [Data](#associatedtype.Data) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#27)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [FallbackImage](../texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")\>>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#174)

### impl<B, E> [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") for [ExtendedMaterial](../../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../../prelude/trait.Material.html "trait bevy::prelude::Material"), E: [MaterialExtension](../../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#175)

#### type [Data](#associatedtype.Data) = [MaterialExtensionBindGroupData](../../pbr/struct.MaterialExtensionBindGroupData.html "struct bevy::pbr::MaterialExtensionBindGroupData")<<B as [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"), <E as [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#176)

#### type [Param](#associatedtype.Param) = (<B as [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Param](trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param"), <E as [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Param](trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param"))