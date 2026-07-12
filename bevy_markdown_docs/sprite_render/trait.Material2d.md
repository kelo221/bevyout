[bevy](../index.html)::[sprite\_render](index.html)

# Trait Material2d 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#136)

```rust
pub trait Material2d:
    Sized
    + AsBindGroup
    + Asset
    + Clone {
    // Provided methods
    fn vertex_shader() -> ShaderRef { ... }
    fn fragment_shader() -> ShaderRef { ... }
    fn depth_bias(&self) -> f32 { ... }
    fn alpha_mode(&self) -> AlphaMode2d { ... }
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> { ... }
}
```

Materials are used alongside [`Material2dPlugin`](struct.Material2dPlugin.html "struct bevy::sprite_render::Material2dPlugin"), [`Mesh2d`](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d"), and [`MeshMaterial2d`](../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d") to spawn entities that are rendered with a specific [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") type. They serve as an easy to use high level way to render [`Mesh2d`](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d") entities with custom shader logic.

Materials must implement [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") to define how data will be transferred to the GPU and bound in shaders. [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") can be derived, which makes generating bindings straightforward. See the [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") docs for details.

## Example

Here is a simple [`Material2d`](trait.Material2d.html "trait bevy::sprite_render::Material2d") implementation. The [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derive has many features. To see what else is available, check out the [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") documentation.

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

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// Spawn an entity with a mesh using `CustomMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(CustomMaterial {
            color: RED.into(),
            color_texture: asset_server.load("some_image.png"),
        })),
    ));
}
```

In WGSL shaders, the material’s binding would look like this:

```
struct CustomMaterial {
    color: vec4<f32>,
}

@group(2) @binding(0) var<uniform> material: CustomMaterial;
@group(2) @binding(1) var color_texture: texture_2d<f32>;
@group(2) @binding(2) var color_sampler: sampler;
```

## Provided Methods

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#139)

#### fn [vertex\_shader](#method.vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default mesh vertex shader will be used.

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#145)

#### fn [fragment\_shader](#method.fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this material’s fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default mesh fragment shader will be used.

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#151)

#### fn [depth\_bias](#method.depth_bias)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Add a bias to the view depth of the mesh which can be used to force a specific render order.

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#155)

#### fn [alpha\_mode](#method.alpha_mode)(&self) -> [AlphaMode2d](enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#165-169)

#### fn [specialize](#method.specialize)( descriptor: &mut [RenderPipelineDescriptor](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor"), layout: &[MeshVertexBufferLayoutRef](../mesh/struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef"), key: [Material2dKey](struct.Material2dKey.html "struct bevy::sprite_render::Material2dKey")<Self>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SpecializedMeshPipelineError](../material/specialize/enum.SpecializedMeshPipelineError.html "enum bevy::material::specialize::SpecializedMeshPipelineError")\>

Customizes the default [`RenderPipelineDescriptor`](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#147)

### impl [Material2d](trait.Material2d.html "trait bevy::sprite_render::Material2d") for [ColorMaterial](../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#313)

### impl [Material2d](trait.Material2d.html "trait bevy::sprite_render::Material2d") for [SpriteMaterial](../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#39)

### impl [Material2d](trait.Material2d.html "trait bevy::sprite_render::Material2d") for [TilemapChunkMaterial](struct.TilemapChunkMaterial.html "struct bevy::sprite_render::TilemapChunkMaterial")