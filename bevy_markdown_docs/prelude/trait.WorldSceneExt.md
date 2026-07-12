[bevy](../index.html)::[prelude](index.html)

# Trait WorldSceneExt 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#14)

```rust
pub trait WorldSceneExt {
    // Required methods
    fn spawn_scene<S>(
        &mut self,
        scene: S,
    ) -> Result<EntityWorldMut<'_>, SpawnSceneError>
       where S: Scene;
    fn queue_spawn_scene<S>(&mut self, scene: S) -> EntityWorldMut<'_>
       where S: Scene;
    fn spawn_scene_list<L>(
        &mut self,
        scenes: L,
    ) -> Result<Vec<Entity>, SpawnSceneError>
       where L: SceneList;
    fn queue_spawn_scene_list<L>(&mut self, scenes: L)
       where L: SceneList;
}
```

Adds scene spawning functionality to [`World`](struct.World.html "struct bevy::prelude::World").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#56)

#### fn [spawn\_scene](#tymethod.spawn_scene)<S>( &mut self, scene: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[EntityWorldMut](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, [SpawnSceneError](../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError")\>

where S: [Scene](trait.Scene.html "trait bevy::prelude::Scene"),

Spawns the given [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") immediately. This will resolve the Scene (using [`Scene::resolve`](trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will return a [`SpawnSceneError`](../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError"). If resolving the [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned.

If resolving and spawning is successful, it will return a new [`EntityWorldMut`](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") containing the full contents of the spawned scene.

See [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

If your scene has a dependency that might not be loaded yet (for example, it includes a `.bsn` asset file), consider using [`World::queue_spawn_scene`](struct.World.html#method.queue_spawn_scene "method bevy::prelude::World::queue_spawn_scene"). Note that the `.bsn` file format is not yet released.

```rust
#[derive(Component, Default, Clone)]
struct Score(usize);

#[derive(Component, Default, Clone)]
struct Sword;

#[derive(Component, Default, Clone)]
struct Shield;

world.spawn_scene(bsn! {
    #Player
    Score(0)
    Children [
        Sword,
        Shield,
    ]
}).unwrap();
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#99)

#### fn [queue\_spawn\_scene](#tymethod.queue_spawn_scene)<S>(&mut self, scene: S) -> [EntityWorldMut](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>

where S: [Scene](trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be spawned. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene will be spawned this frame.

See [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

```rust
#[derive(Component, Default, Clone)]
struct Score(usize);

#[derive(Component, Default, Clone)]
struct Sword;

#[derive(Component, Default, Clone)]
struct Shield;

// This scene includes the "player.bsn" asset (note that the `.bsn` file format is not yet released). It will be spawned on the frame that "player.bsn"
// is fully loaded.
world.queue_spawn_scene(bsn! {
    :"player.bsn"
    #Player
    Score(0)
    Children [
        Sword,
        Shield,
    ]
});
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#143-144)

#### fn [spawn\_scene\_list](#tymethod.spawn_scene_list)<L>( &mut self, scenes: L, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](struct.Vec.html "struct bevy::prelude::Vec")<[Entity](struct.Entity.html "struct bevy::prelude::Entity")\>, [SpawnSceneError](../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError")\>

where L: [SceneList](trait.SceneList.html "trait bevy::prelude::SceneList"),

Spawns the given [`SceneList`](trait.SceneList.html "trait bevy::prelude::SceneList") immediately. This will resolve the scene list (using [`SceneList::resolve_list`](trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list")). If that fails (for example, if there are dependencies that have not been loaded yet), it will return a [`SpawnSceneError`](../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError"). If resolving the [`SceneList`](trait.SceneList.html "trait bevy::prelude::SceneList") is successful, the scene list will be spawned.

If resolving and spawning is successful, it will return a [`Vec<Entity>`](struct.Vec.html "struct bevy::prelude::Vec") containing each entity described in the [`SceneList`](trait.SceneList.html "trait bevy::prelude::SceneList").

See [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

If your scene list has a dependency that might not be loaded yet (for example, it includes a `.bsn` asset file), consider using [`World::queue_spawn_scene_list`](struct.World.html#method.queue_spawn_scene_list "method bevy::prelude::World::queue_spawn_scene_list"). Note that the `.bsn` file format is not yet released.

```rust
#[derive(Component, FromTemplate)]
enum Team {
    #[default]
    Red,
    Blue,
}

world.spawn_scene_list(bsn_list! {
    (
        #Player1
        Team::Red
    ),
    (
        #Player2
        Team::Blue
    )
}).unwrap();
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#184)

#### fn [queue\_spawn\_scene\_list](#tymethod.queue_spawn_scene_list)<L>(&mut self, scenes: L)

where L: [SceneList](trait.SceneList.html "trait bevy::prelude::SceneList"),

Queues the `scene_list` to be spawned. This will evaluate the `scene_list`’s dependencies (via [`Scene::register_dependencies`](trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](../scene/enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene list will be spawned this frame.

```rust
#[derive(Component, FromTemplate)]
enum Team {
    #[default]
    Red,
    Blue,
}
// This scene list includes the "player.bsn" asset (note that the `.bsn` file format is not yet released). It will be spawned on the frame that "player.bsn"
// is loaded.
world.queue_spawn_scene_list(bsn_list! [
    (
        :"player.bsn"
        #Player1
        Team::Red
    ),
    (
        :"player.bsn"
        #Player2
        Team::Blue
    )
]);
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#187)

### impl [WorldSceneExt](trait.WorldSceneExt.html "trait bevy::prelude::WorldSceneExt") for [World](struct.World.html "struct bevy::prelude::World")