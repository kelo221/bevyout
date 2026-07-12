[bevy](../../index.html)::[gltf](../index.html)::[extensions](index.html)

# Trait ErasedGltfExtensionHandler 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#281)

```rust
pub trait ErasedGltfExtensionHandler:
    Send
    + Sync
    + 'static {
    // Required methods
    fn dyn_clone(&self) -> Box<dyn ErasedGltfExtensionHandler>;
    fn on_root(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf: &Gltf,
        settings: &GltfLoaderSettings,
    );
    fn on_animation(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_animation: &Animation<'_>,
        animation_clip: &mut AnimationClip,
    );
    fn on_animations_collected(
        &mut self,
        load_context: &mut LoadContext<'_>,
        animations: &[Handle<AnimationClip>],
        named_animations: &HashMap<Box<str>, Handle<AnimationClip>>,
        animation_roots: &HashSet<usize>,
    );
    fn on_texture(&mut self, gltf_texture: &Texture<'_>, texture: Handle<Image>);
    fn on_material(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_material: &Material<'_>,
        material: Handle<GltfMaterial>,
        material_asset: &GltfMaterial,
        material_label: &str,
    );
    fn on_gltf_primitive<'a>(
        &'a mut self,
        load_context: &'a mut LoadContext<'_>,
        gltf_document: &'a Gltf,
        gltf_mesh: &'a Mesh<'_>,
        gltf_primitive: &'a Primitive<'_>,
        buffer_data: &'a [Vec<u8>],
        custom_vertex_attributes: &'a HashMap<Box<str>, MeshVertexAttribute>,
        gltf_mesh_on_skinned_nodes: bool,
        gltf_mesh_on_non_skinned_nodes: bool,
        user_mesh: &'a mut Option<Mesh>,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = ()> + 'a>>;
    fn on_gltf_mesh(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_mesh: &Mesh<'_>,
        mesh: Handle<GltfMesh>,
    );
    fn on_spawn_mesh_and_material(
        &mut self,
        load_context: &mut LoadContext<'_>,
        primitive: &Primitive<'_>,
        mesh: &Mesh<'_>,
        material: &Material<'_>,
        entity: &mut EntityWorldMut<'_>,
        material_label: &str,
    );
    fn on_scene_completed(
        &mut self,
        load_context: &mut LoadContext<'_>,
        scene: &Scene<'_>,
        world_root_id: Entity,
        scene_world: &mut World,
    );
    fn on_gltf_node(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    );
    fn on_spawn_light_directional(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    );
    fn on_spawn_light_point(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    );
    fn on_spawn_light_spot(
        &mut self,
        load_context: &mut LoadContext<'_>,
        gltf_node: &Node<'_>,
        entity: &mut EntityWorldMut<'_>,
    );
}
```

Type-erased version of [`GltfExtensionHandler`](trait.GltfExtensionHandler.html "trait bevy::gltf::extensions::GltfExtensionHandler"). This is used to store heterogeneous handlers in a collection.

## Required Methods

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#283)

#### fn [dyn\_clone](#tymethod.dyn_clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedGltfExtensionHandler](trait.ErasedGltfExtensionHandler.html "trait bevy::gltf::extensions::ErasedGltfExtensionHandler")\>

Required for dyn cloning

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#287-292)

#### fn [on\_root](#tymethod.on_root)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf: &[Gltf](../gltf/struct.Gltf.html "struct bevy::gltf::gltf::Gltf"), settings: &[GltfLoaderSettings](../struct.GltfLoaderSettings.html "struct bevy::gltf::GltfLoaderSettings"), )

Called when the “global” data for an extension at the root of a glTF file is encountered.

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#296-301)

#### fn [on\_animation](#tymethod.on_animation)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_animation: &[Animation](../gltf/struct.Animation.html "struct bevy::gltf::gltf::Animation")<'\_>, animation\_clip: &mut [AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip"), )

Available on **crate feature `bevy_animation`** only.

Called when an individual animation is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#305-311)

#### fn [on\_animations\_collected](#tymethod.on_animations_collected)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, animations: &\[[Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")\>\], named\_animations: &[HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")\>>, animation\_roots: &[HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, )

Available on **crate feature `bevy_animation`** only.

Called when all animations have been collected.

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#314)

#### fn [on\_texture](#tymethod.on_texture)(&mut self, gltf\_texture: &[Texture](../gltf/struct.Texture.html "struct bevy::gltf::gltf::Texture")<'\_>, texture: [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>)

Called when an individual texture is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#317-324)

#### fn [on\_material](#tymethod.on_material)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_material: &[Material](../gltf/struct.Material.html "struct bevy::gltf::gltf::Material")<'\_>, material: [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[GltfMaterial](../struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial")\>, material\_asset: &[GltfMaterial](../struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial"), material\_label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), )

Called when an individual material is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#327-338)

#### fn [on\_gltf\_primitive](#tymethod.on_gltf_primitive)<'a>( &'a mut self, load\_context: &'a mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_document: &'a [Gltf](../gltf/struct.Gltf.html "struct bevy::gltf::gltf::Gltf"), gltf\_mesh: &'a [Mesh](../gltf/struct.Mesh.html "struct bevy::gltf::gltf::Mesh")<'\_>, gltf\_primitive: &'a [Primitive](../gltf/struct.Primitive.html "struct bevy::gltf::gltf::Primitive")<'\_>, buffer\_data: &'a \[[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>\], custom\_vertex\_attributes: &'a [HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>, [MeshVertexAttribute](../../mesh/struct.MeshVertexAttribute.html "struct bevy::mesh::MeshVertexAttribute")\>, gltf\_mesh\_on\_skinned\_nodes: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), gltf\_mesh\_on\_non\_skinned\_nodes: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), user\_mesh: &'a mut [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")\>, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> + 'a>>

Called when an individual glTF primitive is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#341-346)

#### fn [on\_gltf\_mesh](#tymethod.on_gltf_mesh)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_mesh: &[Mesh](../gltf/struct.Mesh.html "struct bevy::gltf::gltf::Mesh")<'\_>, mesh: [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[GltfMesh](../struct.GltfMesh.html "struct bevy::gltf::GltfMesh")\>, )

Called when an individual glTF Mesh is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#349-357)

#### fn [on\_spawn\_mesh\_and\_material](#tymethod.on_spawn_mesh_and_material)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, primitive: &[Primitive](../gltf/struct.Primitive.html "struct bevy::gltf::gltf::Primitive")<'\_>, mesh: &[Mesh](../gltf/struct.Mesh.html "struct bevy::gltf::gltf::Mesh")<'\_>, material: &[Material](../gltf/struct.Material.html "struct bevy::gltf::gltf::Material")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, material\_label: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), )

Called when mesh and material are spawned as a single Entity

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#360-366)

#### fn [on\_scene\_completed](#tymethod.on_scene_completed)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, scene: &[Scene](../gltf/struct.Scene.html "struct bevy::gltf::gltf::Scene")<'\_>, world\_root\_id: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), scene\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Called when an individual Scene is done processing

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#369-374)

#### fn [on\_gltf\_node](#tymethod.on_gltf_node)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called when a node is processed

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#377-382)

#### fn [on\_spawn\_light\_directional](#tymethod.on_spawn_light_directional)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called when a `DirectionalLight` node is spawned

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#385-390)

#### fn [on\_spawn\_light\_point](#tymethod.on_spawn_light_point)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called when a `PointLight` node is spawned

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#393-398)

#### fn [on\_spawn\_light\_spot](#tymethod.on_spawn_light_spot)( &mut self, load\_context: &mut [LoadContext](../../asset/struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, gltf\_node: &[Node](https://docs.rs/gltf/1.4.1/x86_64-unknown-linux-gnu/gltf/scene/struct.Node.html "struct gltf::scene::Node")<'\_>, entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Called when a `SpotLight` node is spawned

## Trait Implementations

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#569)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedGltfExtensionHandler](trait.ErasedGltfExtensionHandler.html "trait bevy::gltf::extensions::ErasedGltfExtensionHandler")\>

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#570)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedGltfExtensionHandler](trait.ErasedGltfExtensionHandler.html "trait bevy::gltf::extensions::ErasedGltfExtensionHandler")\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#401)

### impl<H> [ErasedGltfExtensionHandler](trait.ErasedGltfExtensionHandler.html "trait bevy::gltf::extensions::ErasedGltfExtensionHandler") for H

where H: [GltfExtensionHandler](trait.GltfExtensionHandler.html "trait bevy::gltf::extensions::GltfExtensionHandler"),