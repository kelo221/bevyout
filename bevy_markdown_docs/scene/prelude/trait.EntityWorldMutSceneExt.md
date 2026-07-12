[bevy](../../index.html)::[scene](../index.html)::[prelude](index.html)

# Trait EntityWorldMutSceneExt 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#413)

```rust
pub trait EntityWorldMutSceneExt {
    // Required methods
    fn queue_spawn_related_scenes<T>(self, scenes: impl SceneList) -> Self
       where T: RelationshipTarget;
    fn apply_scene<S>(&mut self, scene: S) -> Result<(), SpawnSceneError>
       where S: Scene;
    fn queue_apply_scene<S>(&mut self, scene: S)
       where S: Scene;
}
```

Adds scene functionality to [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#452)

#### fn [queue\_spawn\_related\_scenes](#tymethod.queue_spawn_related_scenes)<T>(self, scenes: impl [SceneList](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")) -> Self

where T: [RelationshipTarget](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Spawns a [`SceneList`](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), where each entity is related to the current entity using [`RelationshipTarget::Relationship`](../../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship").

This will evaluate the `scene_list`’s dependencies (via [`SceneList::register_dependencies`](../../prelude/trait.SceneList.html#tymethod.register_dependencies "method bevy::prelude::SceneList::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene list will be spawned this frame.

```rust
#[derive(Component, FromTemplate)]
enum Team {
    #[default]
    Red,
    Blue,
}

world.spawn_empty().queue_spawn_related_scenes::<Children>(bsn_list! {
    (
        #Player1
        Team::Red
    ),
    (
        #Player2
        Team::Blue
    )
});
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#465)

#### fn [apply\_scene](#tymethod.apply_scene)<S>(&mut self, scene: S) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SpawnSceneError](../enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError")\>

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Applies the given [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") to the current entity immediately. This will resolve the Scene (using [`Scene::resolve`](../../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will return a [`SpawnSceneError`](../enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError"). If resolving the [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned.

If resolving and spawning is successful, the entity will contain the full contents of the spawned scene.

This will write directly on top of any existing components on the entity. [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is generally used as a spawning mechanism, so for most things, prefer using [`World::spawn_scene`](../../prelude/struct.World.html#method.spawn_scene "method bevy::prelude::World::spawn_scene").

See [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

If your scene has a dependency that might not be loaded yet (for example, it includes a `.bsn` asset file), consider using [`World::queue_spawn_scene`](../../prelude/struct.World.html#method.queue_spawn_scene "method bevy::prelude::World::queue_spawn_scene"). Note that the .bsn file format is not yet released.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#474)

#### fn [queue\_apply\_scene](#tymethod.queue_apply_scene)<S>(&mut self, scene: S)

where S: [Scene](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be applied. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](../../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene will be spawned this frame. This will write directly on top of any existing components on the entity. [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") is generally used as a spawning mechanism, so for most things, prefer using [`World::queue_spawn_scene`](../../prelude/struct.World.html#method.queue_spawn_scene "method bevy::prelude::World::queue_spawn_scene").

See [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#477)

### impl [EntityWorldMutSceneExt](../../prelude/trait.EntityWorldMutSceneExt.html "trait bevy::prelude::EntityWorldMutSceneExt") for [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>