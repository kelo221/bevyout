[bevy](../index.html)::[pbr](index.html)

# Trait Material 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#146)

```rust
pub trait Material:
    Sized
    + Asset
    + AsBindGroup
    + Clone {
    // Provided methods
    fn vertex_shader() -> ShaderRef { ... }
    fn fragment_shader() -> ShaderRef { ... }
    fn alpha_mode(&self) -> AlphaMode { ... }
    fn opaque_render_method(&self) -> OpaqueRendererMethod { ... }
    fn depth_bias(&self) -> f32 { ... }
    fn reads_view_transmission_texture(&self) -> bool { ... }
    fn enable_prepass() -> bool { ... }
    fn enable_shadows() -> bool { ... }
    fn prepass_vertex_shader() -> ShaderRef { ... }
    fn prepass_fragment_shader() -> ShaderRef { ... }
    fn deferred_vertex_shader() -> ShaderRef { ... }
    fn deferred_fragment_shader() -> ShaderRef { ... }
    fn meshlet_mesh_fragment_shader() -> ShaderRef { ... }
    fn meshlet_mesh_prepass_fragment_shader() -> ShaderRef { ... }
    fn meshlet_mesh_deferred_fragment_shader() -> ShaderRef { ... }
    fn specialize(
        pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> { ... }
}
```

Materials are used alongside [`MaterialPlugin`](../prelude/struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin"), [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d"), and [`MeshMaterial3d`](../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d") to spawn entities that are rendered with a specific [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") type. They serve as an easy to use high level way to render [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities with custom shader logic.

Materials must implement [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") to define how data will be transferred to the GPU and bound in shaders. [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") can be derived, which makes generating bindings straightforward. See the [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") docs for details.

## Example

Here is a simple [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") implementation. The [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derive has many features. To see what else is available, check out the [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") documentation.

```rust
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct CustomMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: LinearRgba,
    // Images can be bound as textures in shaders. If the Image's sampler is also needed, just
    // add the sampler attribute with a different binding index.
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

// All functions on `Material` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// Spawn an entity with a mesh using `CustomMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: RED.into(),
            color_texture: asset_server.load("some_image.png"),
        })),
    ));
}
```

In WGSL shaders, the material’s binding would look like this:

```
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var color_sampler: sampler;
```

## Provided Methods

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#149)

#### fn [vertex\_shader](#method.vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default mesh vertex shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#155)

#### fn [fragment\_shader](#method.fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default mesh fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#161)

#### fn [alpha\_mode](#method.alpha_mode)(&self) -> [AlphaMode](../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

Returns this material’s [`AlphaMode`](../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode"). Defaults to [`AlphaMode::Opaque`](../prelude/enum.AlphaMode.html#variant.Opaque "variant bevy::prelude::AlphaMode::Opaque").

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#169)

#### fn [opaque\_render\_method](#method.opaque_render_method)(&self) -> [OpaqueRendererMethod](../material/enum.OpaqueRendererMethod.html "enum bevy::material::OpaqueRendererMethod")

Returns if this material should be rendered by the deferred or forward renderer. for `AlphaMode::Opaque` or `AlphaMode::Mask` materials. If `OpaqueRendererMethod::Auto`, it will default to what is selected in the `DefaultOpaqueRendererMethod` resource.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#177)

#### fn [depth\_bias](#method.depth_bias)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Add a bias to the view depth of the mesh which can be used to force a specific render order. for meshes with similar depth, to avoid z-fighting. The bias is in depth-texture units so large values may be needed to overcome small depth differences.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#186)

#### fn [reads\_view\_transmission\_texture](#method.reads_view_transmission_texture)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether the material would like to read from [`ViewTransmissionTexture`](struct.ViewTransmissionTexture.html "struct bevy::pbr::ViewTransmissionTexture").

This allows taking color output from the [`Opaque3d`](../core_pipeline/core_3d/struct.Opaque3d.html "struct bevy::core_pipeline::core_3d::Opaque3d") pass as an input, (for screen-space transmission) but requires rendering to take place in a separate [`Transmissive3d`](struct.Transmissive3d.html "struct bevy::pbr::Transmissive3d") pass.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#193)

#### fn [enable\_prepass](#method.enable_prepass)() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Controls if the prepass is enabled for the Material. For more information about what a prepass is, see the [`bevy_core_pipeline::prepass`](../core_pipeline/prepass/index.html "mod bevy::core_pipeline::prepass") docs.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#199)

#### fn [enable\_shadows](#method.enable_shadows)() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Controls if shadows are enabled for the Material.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#208)

#### fn [prepass\_vertex\_shader](#method.prepass_vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s prepass vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default prepass vertex shader will be used.

This is used for the various [prepasses](../core_pipeline/prepass/index.html "mod bevy::core_pipeline::prepass") as well as for generating the depth maps required for shadow mapping.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#217)

#### fn [prepass\_fragment\_shader](#method.prepass_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s prepass fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default prepass fragment shader will be used.

This is used for the various [prepasses](../core_pipeline/prepass/index.html "mod bevy::core_pipeline::prepass") as well as for generating the depth maps required for shadow mapping.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#223)

#### fn [deferred\_vertex\_shader](#method.deferred_vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s deferred vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default deferred vertex shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#229)

#### fn [deferred\_fragment\_shader](#method.deferred_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s deferred fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default deferred fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#240)

#### fn [meshlet\_mesh\_fragment\_shader](#method.meshlet_mesh_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Available on **crate feature `meshlet`** only.

Returns this material’s [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default meshlet mesh fragment shader will be used.

This is part of an experimental feature, and is unnecessary to implement unless you are using `MeshletMesh`’s.

See [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") for limitations.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#251)

#### fn [meshlet\_mesh\_prepass\_fragment\_shader](#method.meshlet_mesh_prepass_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Available on **crate feature `meshlet`** only.

Returns this material’s [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") prepass fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default meshlet mesh prepass fragment shader will be used.

This is part of an experimental feature, and is unnecessary to implement unless you are using `MeshletMesh`’s.

See [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") for limitations.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#262)

#### fn [meshlet\_mesh\_deferred\_fragment\_shader](#method.meshlet_mesh_deferred_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Available on **crate feature `meshlet`** only.

Returns this material’s [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") deferred fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default meshlet mesh deferred fragment shader will be used.

This is part of an experimental feature, and is unnecessary to implement unless you are using `MeshletMesh`’s.

See [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") for limitations.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#273-278)

#### fn [specialize](#method.specialize)( pipeline: &[MaterialPipeline](struct.MaterialPipeline.html "struct bevy::pbr::MaterialPipeline"), descriptor: &mut [RenderPipelineDescriptor](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor"), layout: &[MeshVertexBufferLayoutRef](../mesh/struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef"), key: [MaterialPipelineKey](struct.MaterialPipelineKey.html "struct bevy::pbr::MaterialPipelineKey")<Self>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SpecializedMeshPipelineError](../material/specialize/enum.SpecializedMeshPipelineError.html "enum bevy::material::specialize::SpecializedMeshPipelineError")\>

Customizes the default [`RenderPipelineDescriptor`](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor") for a specific entity using the entity’s [`MaterialPipelineKey`](struct.MaterialPipelineKey.html "struct bevy::pbr::MaterialPipelineKey") and [`MeshVertexBufferLayoutRef`](../mesh/struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef") as input.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#1376)

### impl [Material](../prelude/trait.Material.html "trait bevy::prelude::Material") for [StandardMaterial](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#316)

### impl<B, E> [Material](../prelude/trait.Material.html "trait bevy::prelude::Material") for [ExtendedMaterial](struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material"), E: [MaterialExtension](trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension"),