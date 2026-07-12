[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait SpecializedMeshPipeline 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#135)

```rust
pub trait SpecializedMeshPipeline {
    type Key: Clone + Hash + PartialEq + Eq;

    // Required method
    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError>;
}
```

A trait that allows constructing different variants of a render pipeline from a key and the particular mesh’s vertex buffer layout.

See [`SpecializedMeshPipelines`](struct.SpecializedMeshPipelines.html "struct bevy::render::render_resource::SpecializedMeshPipelines") for more info.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#137)

#### type [Key](#associatedtype.Key): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq")

The key that defines each “variant” of the render pipeline.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#143-147)

#### fn [specialize](#tymethod.specialize)( &self, key: Self::[Key](trait.SpecializedMeshPipeline.html#associatedtype.Key "type bevy::render::render_resource::SpecializedMeshPipeline::Key"), layout: &[MeshVertexBufferLayoutRef](../../mesh/struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RenderPipelineDescriptor](../../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor"), [SpecializedMeshPipelineError](../../material/specialize/enum.SpecializedMeshPipelineError.html "enum bevy::material::specialize::SpecializedMeshPipelineError")\>

Construct a new render pipeline based on the provided key and vertex layout.

The returned pipeline descriptor should have a single vertex buffer, which is derived from `layout`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#452)

### impl [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [MaterialPipelineSpecializer](../../pbr/struct.MaterialPipelineSpecializer.html "struct bevy::pbr::MaterialPipelineSpecializer")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#453)

#### type [Key](#associatedtype.Key) = [ErasedMaterialPipelineKey](../../material/key/struct.ErasedMaterialPipelineKey.html "struct bevy::material::key::ErasedMaterialPipelineKey")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#582)

### impl [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [Mesh2dPipeline](../../sprite_render/struct.Mesh2dPipeline.html "struct bevy::sprite_render::Mesh2dPipeline")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#583)

#### type [Key](#associatedtype.Key) = [Mesh2dPipelineKey](../../sprite_render/struct.Mesh2dPipelineKey.html "struct bevy::sprite_render::Mesh2dPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3281)

### impl [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [MeshPipeline](../../pbr/struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3282)

#### type [Key](#associatedtype.Key) = [MeshPipelineKey](../../pbr/struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#346)

### impl [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [PrepassPipelineSpecializer](../../pbr/struct.PrepassPipelineSpecializer.html "struct bevy::pbr::PrepassPipelineSpecializer")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#347)

#### type [Key](#associatedtype.Key) = [ErasedMaterialPipelineKey](../../material/key/struct.ErasedMaterialPipelineKey.html "struct bevy::material::key::ErasedMaterialPipelineKey")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#335)

### impl [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [Wireframe2dPipeline](../../sprite_render/struct.Wireframe2dPipeline.html "struct bevy::sprite_render::Wireframe2dPipeline")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#336)

#### type [Key](#associatedtype.Key) = [Mesh2dPipelineKey](../../sprite_render/struct.Mesh2dPipelineKey.html "struct bevy::sprite_render::Mesh2dPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#740)

### impl [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [Wireframe3dPipeline](../../pbr/wireframe/struct.Wireframe3dPipeline.html "struct bevy::pbr::wireframe::Wireframe3dPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#741)

#### type [Key](#associatedtype.Key) = [WireframePipelineKey](../../pbr/wireframe/struct.WireframePipelineKey.html "struct bevy::pbr::wireframe::WireframePipelineKey")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#457-459)

### impl<M> [SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline") for [Material2dPipeline](../../sprite_render/struct.Material2dPipeline.html "struct bevy::sprite_render::Material2dPipeline")<M>

where M: [Material2d](../../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"), <M as [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#461)

#### type [Key](#associatedtype.Key) = [Material2dKey](../../sprite_render/struct.Material2dKey.html "struct bevy::sprite_render::Material2dKey")<M>