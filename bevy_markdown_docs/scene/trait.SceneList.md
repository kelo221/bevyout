[bevy](../index.html)::[scene](index.html)

# Trait SceneList 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#12)

```rust
pub trait SceneList: SceneListBox {
    // Required methods
    fn resolve_list(
        self,
        context: &mut ResolveContext<'_>,
        scenes: &mut Vec<ResolvedScene>,
    ) -> Result<(), ResolveSceneError>;
    fn register_dependencies(&self, dependencies: &mut SceneDependencies);
}
```

This behaves like a list of [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"), where each entry in the list is a new entity (see [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for more details).

[`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") is to [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity") as [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") is to [`Vec<Entity>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#15-19)

#### fn [resolve\_list](#tymethod.resolve_list)( self, context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

This will apply the changes described in this [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") to the given [`Vec<ResolvedScene>`](../prelude/struct.Vec.html "struct bevy::prelude::Vec"). This should not be called until all of the dependencies in [`Scene::register_dependencies`](../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies") have been loaded.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#25)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") can have [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") dependencies, which _must_ be loaded before calling [`SceneList::resolve_list`](../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list") or it might return a [`ResolveSceneError`](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")!

## Trait Implementations

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#572)

### impl<S> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[SceneListScope](struct.SceneListScope.html "struct bevy::scene::SceneListScope")<S>> for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")\>

where S: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#573)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [SceneListScope](struct.SceneListScope.html "struct bevy::scene::SceneListScope")<S>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")\>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#578)

### impl<S> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[SceneScope](struct.SceneScope.html "struct bevy::scene::SceneScope")<S>> for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")\>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#579)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [SceneScope](struct.SceneScope.html "struct bevy::scene::SceneScope")<S>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")\>

Converts to this type from the input type.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#236-238)

### impl<L> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<L>

where L: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#241-245)

#### fn [resolve\_list](#tymethod.resolve_list)( self, context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#251)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P6: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P7: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P8: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P9: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P10: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P11: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P6: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P7: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P8: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P9: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P10: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P6: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P7: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P8: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P9: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5, P6, P7, P8> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5, P6, P7, P8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P6: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P7: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P8: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5, P6, P7> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5, P6, P7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P6: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P7: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5, P6> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5, P6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P6: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4, P5> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4, P5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P5: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3, P4> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3, P4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P4: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2, P3> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2, P3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P3: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1, P2> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1, P2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P2: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0, P1> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0, P1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), P1: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

### impl<P0> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [(P0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where P0: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [resolve\_list](#tymethod.resolve_list)( self, \_context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, \_scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#140)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, \_dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#142)

### impl<S, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<[\[S; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#143-147)

#### fn [resolve\_list](#tymethod.resolve_list)( self, context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#156)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#163)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#164-168)

#### fn [resolve\_list](#tymethod.resolve_list)( self, context: &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>, scenes: &mut [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[ResolvedScene](struct.ResolvedScene.html "struct bevy::scene::ResolvedScene")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ResolveSceneError](enum.ResolveSceneError.html "enum bevy::scene::ResolveSceneError")\>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#175)

#### fn [register\_dependencies](#tymethod.register_dependencies)(&self, dependencies: &mut [SceneDependencies](struct.SceneDependencies.html "struct bevy::scene::SceneDependencies"))

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#203)

### impl [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")\>>

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#499)

### impl<L> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [SceneListScope](struct.SceneListScope.html "struct bevy::scene::SceneListScope")<L>

where L: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#90)

### impl<S> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [EntityScene](struct.EntityScene.html "struct bevy::scene::EntityScene")<S>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#222)

### impl<S> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [SceneScope](struct.SceneScope.html "struct bevy::scene::SceneScope")<S>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#182)

### impl<S> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<S>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_list.rs.html#71)

### impl<T> [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [SceneListBox](trait.SceneListBox.html "trait bevy::scene::SceneListBox") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),