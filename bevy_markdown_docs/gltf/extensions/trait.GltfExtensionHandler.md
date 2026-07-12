[bevy](../../index.html)::[gltf](../index.html)::[extensions](index.html)

# Trait GltfExtensionHandler 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#62)

```rust
pub trait GltfExtensionHandler:
    Send
    + Sync
    + 'static {
    // Required method
    fn dyn_clone(&self) -> Box<dyn ErasedGltfExtensionHandler>;

    // Provided methods
    fn on_root(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf: &Gltf,
        settings: &GltfLoaderSettings,
    ) { ... }
    fn on_animation(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_animation: &Animation<'_>,
        animation_clip: &mut AnimationClip,
    ) { ... }
    fn on_animations_collected(
        &mut self,
        load_context: &mut LoadContext<'_>,
        animations: &[Handle<AnimationClip>],
        named_animations: &HashMap<Box<str>, Handle<AnimationClip>>,
        animation_roots: &HashSet<usize>,
    ) { ... }
    fn on_texture(&mut self, gltf_texture: &Texture<'_>, texture: Handle<Image>) { ... }
    fn on_material(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_material: &Material<'_>,
        material: Handle<GltfMaterial>,
        material_asset: &GltfMaterial,
        material_label: &str,
    ) { ... }
    fn on_gltf_primitive(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_document: &Gltf,
        gltf_mesh: &Mesh<'_>,
        gltf_primitive: &Primitive<'_>,
        buffer_data: &[Vec<u8>],
        custom_vertex_attributes: &HashMap<Box<str>, MeshVertexAttribute>,
        gltf_mesh_on_skinned_nodes: bool,
        gltf_mesh_on_non_skinned_nodes: bool,
        user_mesh: &mut Option<Mesh>,
    ) -> impl ConditionalSendFuture { ... }
    fn on_gltf_mesh(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_mesh: &Mesh<'_>,
        mesh: Handle<GltfMesh>,
    ) { ... }
    fn on_spawn_mesh_and_material(
        &mut self,
        load_context: &mut LoadContext<'_>,
        primitive: &Primitive<'_>,
        mesh: &Mesh<'_>,
        material: &Material<'_>,
        entity: &mut EntityWorldMut<'_>,
        material_label: &str,
    ) { ... }
    fn on_scene_completed(
        &mut self,
        load_context: &mut LoadContext<'_>,
        scene: &Scene<'_>,
        world_root_id: Entity,
        scene_world: &mut World,
    ) { ... }
    fn on_gltf_node(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    ) { ... }
    fn on_spawn_light_directional(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    ) { ... }
    fn on_spawn_light_point(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    ) { ... }
    fn on_spawn_light_spot(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    ) { ... }
}
```

glTF Extensions can attach data to any objects in a glTF file. This is done by inserting data in the `extensions` sub-object, and data in the extensions sub-object is keyed by the id of the extension. For example: `KHR_materials_variants`, `EXT_meshopt_compression`, or `BEVY_my_tool`

A list of publicly known extensions and their ids can be found in the [KhronosGroup/glTF](https://github.com/KhronosGroup/glTF/blob/main/extensions/README.md) git repo. Vendors reserve prefixes, such as the `BEVY` prefix, which is also listed in the [KhronosGroup repo](https://github.com/KhronosGroup/glTF/blob/main/extensions/Prefixes.md).

The `GltfExtensionHandler` trait should be implemented to participate in processing glTF files as they load, and exposes glTF extension data via a series of hook callbacks.

The type a `GltfExtensionHandler` is implemented for can define data which will be cloned for each new glTF load. This enables stateful handling of glTF extension data during a single load.

When loading a glTF file, a glTF object that could contain extension data will cause the relevant hook to execute once per object. Each invocation will receive all extension data, which is required because many extensions require accessing data defined by other extensions.

The hooks are always called once, even if there is no extension data This is useful for scenarios where additional extension data isn’t required, but processing should still happen.

## Required Methods

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#64)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedGltfExtensionHandler](trait.ErasedGltfExtensionHandler.html "trait bevy::gltf::extensions::ErasedGltfExtensionHandler")\>

Required for dyn cloning

## Provided Methods

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#72-77)

#### fn [on\_root](#method.on_root)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf: &[Gltf](../gltf/struct.Gltf.html "struct bevy::gltf::gltf::Gltf"), settings: &[GltfLoaderSettings](../struct.GltfLoaderSettings.html "struct bevy::gltf::GltfLoaderSettings"), )

Called when the “global” data for an extension at the root of a glTF file is encountered.

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#86-91)

#### fn [on\_animation](#method.on_animation)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_animation: &[Animation](../gltf/struct.Animation.html "struct bevy::gltf::gltf::Animation")<'\_>, animation\_clip: &mut [AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip"), )

Available on **crate feature `bevy_animation`** only.

Called when an individual animation is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#103-109)

#### fn [on\_animations\_collected](#method.on_animations_collected)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, animations: &\[[Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")\>\], named\_animations: &[HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")\>>, animation\_roots: &[HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, )

Available on **crate feature `bevy_animation`** only.

Called when all animations have been collected. `animations` is the glTF ordered list of `Handle<AnimationClip>`s `named_animations` is a `HashMap` from animation name to `Handle<AnimationClip>` `animation_roots` is the glTF index of the animation root object

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#117)

#### fn [on\_texture](#method.on_texture)(&mut self, gltf\_texture: &[Texture](../gltf/struct.Texture.html "struct bevy::gltf::gltf::Texture")<'\_>, texture: [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>)

Called when an individual texture is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#124-131)

#### fn [on\_material](#method.on_material)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_material: &[Material](../gltf/struct.Material.html "struct bevy::gltf::gltf::Material")<'\_>, material: [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[GltfMaterial](../struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial")\>, material\_asset: &[GltfMaterial](../struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial"), material\_label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), )

Called when an individual material is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#162-173)

#### fn [on\_gltf\_primitive](#method.on_gltf_primitive)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_document: &[Gltf](../gltf/struct.Gltf.html "struct bevy::gltf::gltf::Gltf"), gltf\_mesh: &[Mesh](../gltf/struct.Mesh.html "struct bevy::gltf::gltf::Mesh")<'\_>, gltf\_primitive: &[Primitive](../gltf/struct.Primitive.html "struct bevy::gltf::gltf::Primitive")<'\_>, buffer\_data: &\[[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>\], custom\_vertex\_attributes: &[HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, [MeshVertexAttribute](../../mesh/struct.MeshVertexAttribute.html "struct bevy::mesh::MeshVertexAttribute")\>, gltf\_mesh\_on\_skinned\_nodes: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), gltf\_mesh\_on\_non\_skinned\_nodes: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), user\_mesh: &mut [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")\>, ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Called when an individual glTF primitive is processed glTF primitives are what become a Bevy `Mesh` This hook is useful for extensions that need to decompress or transform primitives and their associated glTF data.

`buffer_data` is a reference to all of the buffers from the glTF document, in order, after it has been loaded by Bevy. Extensions in glTF are allowed to add arbitrary buffers, so while this data is often vertex data, it can not be assumed to be vertex data.

`out_doc` is an optional `gltf::Document` which, if set, must contain a single `gltf::Mesh` with a single `gltf::Primitive`. This document is only used by Bevy for the processing of the relevant primitive and can not affect other processing.

`out_data` is a single buffer wrapped in a `Vec`, which mirrors the buffer structure of a loaded `gltf::Document`’s buffers, which is the same structure as `buffer_data`. The outer `Vec` must contain a single `Vec<u8>` of data, as only the first generated buffer is used. If set, the loader will use this modified buffer data instead of the original `buffer_data` to construct the Mesh.

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#182-187)

#### fn [on\_gltf\_mesh](#method.on_gltf_mesh)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_mesh: &[Mesh](../gltf/struct.Mesh.html "struct bevy::gltf::gltf::Mesh")<'\_>, mesh: [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[GltfMesh](../struct.GltfMesh.html "struct bevy::gltf::GltfMesh")\>, )

Called when an individual glTF Mesh is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#197-205)

#### fn [on\_spawn\_mesh\_and\_material](#method.on_spawn_mesh_and_material)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, primitive: &[Primitive](../gltf/struct.Primitive.html "struct bevy::gltf::gltf::Primitive")<'\_>, mesh: &[Mesh](../gltf/struct.Mesh.html "struct bevy::gltf::gltf::Mesh")<'\_>, material: &[Material](../gltf/struct.Material.html "struct bevy::gltf::gltf::Material")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, material\_label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), )

mesh and material are spawned as a single Entity, which means an extension would have to decide for itself how to merge the extension data.

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#213-219)

#### fn [on\_scene\_completed](#method.on_scene_completed)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, scene: &[Scene](../gltf/struct.Scene.html "struct bevy::gltf::gltf::Scene")<'\_>, world\_root\_id: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), scene\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Called when an individual Scene is done processing

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#227-232)

#### fn [on\_gltf\_node](#method.on_gltf_node)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called when a node is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#242-247)

#### fn [on\_spawn\_light\_directional](#method.on_spawn_light_directional)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called with a `DirectionalLight` node is spawned which is typically created as a result of `KHR_lights_punctual`

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#256-261)

#### fn [on\_spawn\_light\_point](#method.on_spawn_light_point)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called with a `PointLight` node is spawned which is typically created as a result of `KHR_lights_punctual`

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#270-275)

#### fn [on\_spawn\_light\_spot](#method.on_spawn_light_spot)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called with a `SpotLight` node is spawned which is typically created as a result of `KHR_lights_punctual`

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors