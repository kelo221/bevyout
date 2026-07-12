[bevy](../index.html)::[pbr](index.html)

# Trait MaterialExtension 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#33)

```rust
pub trait MaterialExtension:
    Sized
    + Asset
    + AsBindGroup
    + Clone {
    // Provided methods
    fn vertex_shader() -> ShaderRef { ... }
    fn fragment_shader() -> ShaderRef { ... }
    fn alpha_mode() -> Option<AlphaMode> { ... }
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
        pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> { ... }
}
```

A subset of the `Material` trait for defining extensions to a base `Material`, such as the builtin `StandardMaterial`.

A user type implementing the trait should be used as the `E` generic param in an `ExtendedMaterial` struct.

## Provided Methods

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#36)

#### fn [vertex\_shader](#method.vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the base material mesh vertex shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#42)

#### fn [fragment\_shader](#method.fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the base material mesh fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#47)

#### fn [alpha\_mode](#method.alpha_mode)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[AlphaMode](../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#54)

#### fn [enable\_prepass](#method.enable_prepass)() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Controls if the prepass is enabled for the Material. For more information about what a prepass is, see the [`bevy_core_pipeline::prepass`](../core_pipeline/prepass/index.html "mod bevy::core_pipeline::prepass") docs.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#60)

#### fn [enable\_shadows](#method.enable_shadows)() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Controls if shadows are enabled for the Material.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#66)

#### fn [prepass\_vertex\_shader](#method.prepass_vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s prepass vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the base material prepass vertex shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#72)

#### fn [prepass\_fragment\_shader](#method.prepass_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s prepass fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the base material prepass fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#78)

#### fn [deferred\_vertex\_shader](#method.deferred_vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s deferred vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the base material deferred vertex shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#84)

#### fn [deferred\_fragment\_shader](#method.deferred_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s prepass fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the base material deferred fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#91)

#### fn [meshlet\_mesh\_fragment\_shader](#method.meshlet_mesh_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Available on **crate feature `meshlet`** only.

Returns this material’s [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default meshlet mesh fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#98)

#### fn [meshlet\_mesh\_prepass\_fragment\_shader](#method.meshlet_mesh_prepass_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Available on **crate feature `meshlet`** only.

Returns this material’s [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") prepass fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default meshlet mesh prepass fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#105)

#### fn [meshlet\_mesh\_deferred\_fragment\_shader](#method.meshlet_mesh_deferred_fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Available on **crate feature `meshlet`** only.

Returns this material’s [`crate::meshlet::MeshletMesh`](experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh") deferred fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default meshlet mesh deferred fragment shader will be used.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#117-122)

#### fn [specialize](#method.specialize)( pipeline: &[MaterialExtensionPipeline](struct.MaterialExtensionPipeline.html "struct bevy::pbr::MaterialExtensionPipeline"), descriptor: &mut [RenderPipelineDescriptor](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor"), layout: &[MeshVertexBufferLayoutRef](../mesh/struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef"), key: [MaterialExtensionKey](struct.MaterialExtensionKey.html "struct bevy::pbr::MaterialExtensionKey")<Self>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SpecializedMeshPipelineError](../material/specialize/enum.SpecializedMeshPipelineError.html "enum bevy::material::specialize::SpecializedMeshPipelineError")\>

Customizes the default [`RenderPipelineDescriptor`](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor") for a specific entity using the entity’s [`MaterialPipelineKey`](struct.MaterialPipelineKey.html "struct bevy::pbr::MaterialPipelineKey") and [`MeshVertexBufferLayoutRef`](../mesh/struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef") as input. Specialization for the base material is applied before this function is called.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#113)

### impl [MaterialExtension](trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension") for [ForwardDecalMaterialExt](decal/struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")