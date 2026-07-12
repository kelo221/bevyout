[bevy](../../index.html)::[scene](../index.html)::[prelude](index.html)

# Trait Scene 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#48)

```rust
pub trait Scene: SceneBox {
    // Required method
    fn resolve(
        self,
        context: &mut ResolveContext<'_>,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError>;

    // Provided method
    fn register_dependencies(&self, _dependencies: &mut SceneDependencies) { ... }
}
```

Conceptually, a [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") describes what a spawned [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") should look like. This often describes what [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s the entity should have.

[`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is _always_ a single top level [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") / root entity. For “lists of scenes” / multiple “root” entities, see [`SceneList`](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"). These are separate traits for logical reasons: [`World::spawn`](../../prelude/struct.World.html#method.spawn "method bevy::prelude::World::spawn") is a “single entity” action. Additionally, “scene caching” only makes sense when both scenes are “single root entities”. A good way to think of this is [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") vs [`Vec<Entity>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec"): these are different types with different APIs and semantics.

### Resolving Scenes

Functionally, a [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is something that can contribute to a [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") by calling [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve"). [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is inherently composable. A collection of [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")s is essentially a description of what a final [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") should look like. This is typically done with tuples of [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")s (which also implement [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")).

A [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") generally does one or more of the following to a [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"):

*   Adding a new [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template")
*   Editing an existing [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") (ex: “patching” [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") fields)
*   Adding one or more “related” [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")s, which will be spawned alongside the root [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") and “related” back to it with a [`Relationship`](../../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship").
*   Editing an existing “related” [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene").
*   Setting a [`ScenePatch`](../struct.ScenePatch.html "struct bevy::scene::ScenePatch") containing a cached [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") to apply first.

See [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") for more information on how it can be composed.

A [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") can have dependencies (defined in [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")), which _must_ be loaded before calling [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve"), or it might return a [`ResolveSceneError`](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError").

You generally don’t need to resolve [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")s yourself. Instead use APIs like [`World::spawn_scene`](../../prelude/trait.WorldSceneExt.html#tymethod.spawn_scene "method bevy::prelude::WorldSceneExt::spawn_scene") or [`World::queue_spawn_scene`](../../prelude/trait.WorldSceneExt.html#tymethod.queue_spawn_scene "method bevy::prelude::WorldSceneExt::queue_spawn_scene")

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#54-58)

#### fn [resolve](#tymethod.resolve)( self, context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

This will apply the changes described in this [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") to the given [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"). This should not be called until all of the dependencies in [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies") have been loaded. The scene system will generally call this method on behalf of developers.

[`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")s are free to modify [`ResolvedScene`](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene") in arbitrary ways. In the context of related entities, in general they should just be pushing new entities to the back of the list.

## Provided Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#65)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") can have [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") dependencies, which _must_ be loaded before calling [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve") / [`SceneList::resolve_list`](../../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list") or it might return a [`ResolveSceneError`](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")!

In most cases, the scene system will ensure [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve") / [`SceneList::resolve_list`](../../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list") is called _after_ these dependencies have been loaded.

## Trait Implementations

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#560)

### impl<S> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[SceneScope](../struct.SceneScope.html "struct bevy::scene::SceneScope")<S>> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")\>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#561)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [SceneScope](../struct.SceneScope.html "struct bevy::scene::SceneScope")<S>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene")\>

Converts to this type from the input type.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P6: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P7: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P8: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P9: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P10: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P11: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P6: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P7: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P8: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P9: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P10: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P6: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P7: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P8: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P9: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P6: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P7: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P8: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5, P6, P7> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P6: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P7: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5, P6> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P6: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4, P5> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P5: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3, P4> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P4: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2, P3> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P3: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1, P2> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P2: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0, P1> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), P1: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

### impl<P0> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [(P0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [resolve](#tymethod.resolve)( self, \_context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#212)

#### fn [register\_dependencies](#method.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#214-216)

### impl<S> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<S>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#219-223)

#### fn [resolve](#tymethod.resolve)( self, context: &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scene: &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](../enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#229)

#### fn [register\_dependencies](#method.register_dependencies)(&self, dependencies: &mut [SceneDependencies](../struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#406)

### impl [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [CachedSceneAsset](../struct.CachedSceneAsset.html "struct bevy::scene::CachedSceneAsset")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#464)

### impl [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [NameEntityReference](../struct.NameEntityReference.html "struct bevy::scene::NameEntityReference")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#428-429)

### impl<F, O> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [FnTemplate](../../ecs/template/struct.FnTemplate.html "struct bevy::ecs::template::FnTemplate")<F, O>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<O, [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, O: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#335-338)

### impl<F, T> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [TemplatePatch](../struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + 'static, <T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#614-615)

### impl<F> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [SceneFunction](../struct.SceneFunction.html "struct bevy::scene::SceneFunction")<F>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [ResolveContext](../struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, &mut [ResolvedScene](../struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#534-539)

### impl<I, E, B, M> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [OnTemplate](../struct.OnTemplate.html "struct bevy::scene::OnTemplate")<I, E, B, M>

where I: [IntoObserverSystem](../../ecs/system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), M: 'static,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#371)

### impl<R, L> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [RelatedScenes](../struct.RelatedScenes.html "struct bevy::scene::RelatedScenes")<R, L>

where R: [Relationship](../../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"), L: [SceneList](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#480)

### impl<S> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [SceneScope](../struct.SceneScope.html "struct bevy::scene::SceneScope")<S>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#110)

### impl<T> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [SceneBox](../trait.SceneBox.html "trait bevy::scene::SceneBox") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#600)

### impl<T> [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for [InitTemplate](../struct.InitTemplate.html "struct bevy::scene::InitTemplate")<T>

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),