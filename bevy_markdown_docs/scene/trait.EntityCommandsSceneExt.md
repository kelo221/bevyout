[bevy](../index.html)::[scene](index.html)

# Trait EntityCommandsSceneExt 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#518)

```rust
pub trait EntityCommandsSceneExt {
    // Required methods
    fn queue_spawn_related_scenes<T>(
        &mut self,
        scenes: impl SceneList,
    ) -> &mut Self
       where T: RelationshipTarget;
    fn apply_scene<S>(&mut self, scene: S) -> &mut Self
       where S: Scene;
    fn queue_apply_scene<S>(&mut self, scene: S) -> &mut Self
       where S: Scene;
}
```

Adds scene functionality to [`EntityWorldMut`](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#552-555)

#### fn [queue\_spawn\_related\_scenes](#tymethod.queue_spawn_related_scenes)<T>(&mut self, scenes: impl [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList")) -> &mut Self

where T: [RelationshipTarget](../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"),

Spawns a [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"), where each entity is related to the current entity using [`RelationshipTarget::Relationship`](../prelude/trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship").

This will evaluate the `scene_list`’s dependencies (via [`SceneList::register_dependencies`](../prelude/trait.SceneList.html#tymethod.register_dependencies "method bevy::prelude::SceneList::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene list will be spawned this frame.

```rust
#[derive(Component, FromTemplate)]
enum Team {
    #[default]
    Red,
    Blue,
}

commands.spawn_empty().queue_spawn_related_scenes::<Children>(bsn_list! {
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

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#568)

#### fn [apply\_scene](#tymethod.apply_scene)<S>(&mut self, scene: S) -> &mut Self

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Applies the given [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") to the current entity as soon as [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands") are applied. This will resolve the Scene (using [`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will log a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") as an error. If resolving the [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned.

If resolving and spawning is successful, the entity will contain the full contents of the spawned scene.

This will write directly on top of any existing components on the entity. [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") is generally used as a spawning mechanism, so for most things, prefer using [`Commands::spawn_scene`](../prelude/struct.Commands.html#method.spawn_scene "method bevy::prelude::Commands::spawn_scene").

See [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

If your scene has a dependency that might not be loaded yet (for example, it includes a `.bsn` asset file), consider using [`Commands::spawn_scene`](../prelude/struct.Commands.html#method.spawn_scene "method bevy::prelude::Commands::spawn_scene"). Note that the .bsn file format is not yet released.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#577)

#### fn [queue\_apply\_scene](#tymethod.queue_apply_scene)<S>(&mut self, scene: S) -> &mut Self

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be applied. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene will be spawned this frame. This will write directly on top of any existing components on the entity. [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") is generally used as a spawning mechanism, so for most things, prefer using [`Commands::queue_spawn_scene`](../prelude/struct.Commands.html#method.queue_spawn_scene "method bevy::prelude::Commands::queue_spawn_scene").

See [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#580)

### impl [EntityCommandsSceneExt](../prelude/trait.EntityCommandsSceneExt.html "trait bevy::prelude::EntityCommandsSceneExt") for [EntityCommands](../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>