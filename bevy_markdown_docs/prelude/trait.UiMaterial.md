[bevy](../index.html)::[prelude](index.html)

# Trait UiMaterial 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#102)

```rust
pub trait UiMaterial:
    Sized
    + AsBindGroup
    + Asset
    + Clone {
    // Provided methods
    fn vertex_shader() -> ShaderRef { ... }
    fn fragment_shader() -> ShaderRef { ... }
    fn stack_z_offset() -> f32 { ... }
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        key: UiMaterialKey<Self>,
    ) { ... }
}
```

Materials are used alongside [`UiMaterialPlugin`](struct.UiMaterialPlugin.html "struct bevy::prelude::UiMaterialPlugin") and [`MaterialNode`](struct.MaterialNode.html "struct bevy::prelude::MaterialNode") to spawn entities that are rendered with a specific [`UiMaterial`](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") type. They serve as an easy to use high level way to render `Node` entities with custom shader logic.

`UiMaterials` must implement [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") to define how data will be transferred to the GPU and bound in shaders. [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") can be derived, which makes generating bindings straightforward. See the [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") docs for details.

Materials must also implement [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") so they can be treated as such.

If you are only using the fragment shader, make sure your shader imports the `UiVertexOutput` from `bevy_ui::ui_vertex_output` and uses it as the input of your fragment shader like the example below does.

## Example

Here is a simple [`UiMaterial`](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") implementation. The [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derive has many features. To see what else is available, check out the [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") documentation.

```rust
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
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

// All functions on `UiMaterial` have default impls. You only need to implement the
// functions that are relevant for your material.
impl UiMaterial for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// Spawn an entity using `CustomMaterial`.
fn setup(mut commands: Commands, mut materials: ResMut<Assets<CustomMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn((
        MaterialNode(materials.add(CustomMaterial {
            color: LinearRgba::RED,
            color_texture: asset_server.load("some_image.png"),
        })),
        Node {
            width: Val::Percent(100.0),
            ..Default::default()
        },
    ));
}
```

In WGSL shaders, the material’s binding would look like this:

If you only use the fragment shader make sure to import `UiVertexOutput` from `bevy_ui::ui_vertex_output` in your wgsl shader. Also note that bind group 0 is always bound to the [`View Uniform`](../render/view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform") and the [`Globals Uniform`](../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform").

```
#import bevy_ui::ui_vertex_output UiVertexOutput

struct CustomMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var color_texture: texture_2d<f32>;
@group(1) @binding(2)
var color_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {

}
```

## Provided Methods

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#105)

#### fn [vertex\_shader](#method.vertex_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this materials vertex shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default UI vertex shader will be used.

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#111)

#### fn [fragment\_shader](#method.fragment_shader)() -> [ShaderRef](../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

Returns this materials fragment shader. If [`ShaderRef::Default`](../shader/enum.ShaderRef.html#variant.Default "variant bevy::shader::ShaderRef::Default") is returned, the default UI fragment shader will be used.

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#115)

#### fn [stack\_z\_offset](#method.stack_z_offset)() -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#124)

#### fn [specialize](#method.specialize)( descriptor: &mut [RenderPipelineDescriptor](../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor"), key: [UiMaterialKey](struct.UiMaterialKey.html "struct bevy::prelude::UiMaterialKey")<Self>, )

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#92)

### impl [UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") for [FrametimeGraphMaterial](../dev_tools/frame_time_graph/struct.FrametimeGraphMaterial.html "struct bevy::dev_tools::frame_time_graph::FrametimeGraphMaterial")