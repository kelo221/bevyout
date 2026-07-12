[bevy](../index.html)::[scene](index.html)

# Trait CommandsSceneExt 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#229)

```rust
pub trait CommandsSceneExt {
    // Required methods
    fn spawn_scene<S>(&mut self, scene: S) -> EntityCommands<'_>
       where S: Scene;
    fn queue_spawn_scene<S>(&mut self, scene: S) -> EntityCommands<'_>
       where S: Scene;
    fn spawn_scene_list<L>(&mut self, scenes: L)
       where L: SceneList;
    fn queue_spawn_scene_list<L>(&mut self, scenes: L)
       where L: SceneList;
}
```

Adds scene spawning functionality to [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#263)

#### fn [spawn\_scene](#tymethod.spawn_scene)<S>(&mut self, scene: S) -> [EntityCommands](../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Spawns the given [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") as soon as [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands") are applied. This will resolve the Scene (using [`Scene::resolve`](../prelude/trait.Scene.html#tymethod.resolve "method bevy::prelude::Scene::resolve")). If that fails (for example, if there are dependencies that have not been loaded yet), it will log a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") as an error. If resolving the [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene will be spawned.

This is essentially a [`Command`](../prelude/trait.Command.html "trait bevy::prelude::Command") that runs [`World::spawn_scene`](../prelude/struct.World.html#method.spawn_scene "method bevy::prelude::World::spawn_scene").

See [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

If your scene has a dependency that might not be loaded yet (for example, it includes a `.bsn` asset file), consider using [`Commands::queue_spawn_scene`](../prelude/struct.Commands.html#method.queue_spawn_scene "method bevy::prelude::Commands::queue_spawn_scene"). Note that the `.bsn` file format is not yet released.

```rust
#[derive(Component, Default, Clone)]
struct Score(usize);

#[derive(Component, Default, Clone)]
struct Sword;

#[derive(Component, Default, Clone)]
struct Shield;

commands.spawn_scene(bsn! {
    #Player
    Score(0)
    Children [
        Sword,
        Shield,
    ]
});
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#298)

#### fn [queue\_spawn\_scene](#tymethod.queue_spawn_scene)<S>(&mut self, scene: S) -> [EntityCommands](../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands")<'\_>

where S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),

Queues the `scene` to be spawned. This will evaluate the `scene`’s dependencies (via [`Scene::register_dependencies`](../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene will be spawned this frame.

See [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

```rust
#[derive(Component, Default, Clone)]
struct Score(usize);

#[derive(Component, Default, Clone)]
struct Sword;

#[derive(Component, Default, Clone)]
struct Shield;

// This scene includes the "player.bsn" asset (note that the `.bsn` file format is not yet released). It will be spawned on the frame that "player.bsn"
// is fully loaded.
commands.queue_spawn_scene(bsn! {
    :"player.bsn"
    #Player
    Score(0)
    Children [
        Sword,
        Shield,
    ]
});
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#335)

#### fn [spawn\_scene\_list](#tymethod.spawn_scene_list)<L>(&mut self, scenes: L)

where L: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

Spawns the given [`SceneList`](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList") as soon as [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands") are applied. This will resolve the scene list (using [`SceneList::resolve_list`](../prelude/trait.SceneList.html#tymethod.resolve_list "method bevy::prelude::SceneList::resolve_list")). If that fails (for example, if there are dependencies that have not been loaded yet), it will log a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") as an error. If resolving the [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") is successful, the scene list will be spawned.

This is essentially a [`Command`](../prelude/trait.Command.html "trait bevy::prelude::Command") that performs [`World::spawn_scene_list`](../prelude/struct.World.html#method.spawn_scene_list "method bevy::prelude::World::spawn_scene_list").

See [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for the features of the scene system (and how to use it).

If your scene list has a dependency that might not be loaded yet (for example, it includes a `.bsn` asset file), consider using [`Commands::queue_spawn_scene_list`](../prelude/struct.Commands.html#method.queue_spawn_scene_list "method bevy::prelude::Commands::queue_spawn_scene_list").

```rust
#[derive(Component, FromTemplate)]
enum Team {
    #[default]
    Red,
    Blue,
}

// Note that the .bsn file format is not yet released.
commands.spawn_scene_list(bsn_list! {
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
});
```

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#369)

#### fn [queue\_spawn\_scene\_list](#tymethod.queue_spawn_scene_list)<L>(&mut self, scenes: L)

where L: [SceneList](../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"),

Queues the `scene_list` to be spawned. This will evaluate the `scene_list`’s dependencies (via [`Scene::register_dependencies`](../prelude/trait.Scene.html#method.register_dependencies "method bevy::prelude::Scene::register_dependencies")) and queue it to be resolved and spawned after all of the dependencies have been loaded. If a [`SpawnSceneError`](enum.SpawnSceneError.html "enum bevy::scene::SpawnSceneError") occurs, it will be logged as an error.

If the dependencies are already loaded (or there are no dependencies), then the scene will be spawned this frame.

```rust
#[derive(Component, FromTemplate)]
enum Team {
    #[default]
    Red,
    Blue,
}

// This scene list includes the "player.bsn" asset (note that the `.bsn` file format is not yet released). It will be spawned on the frame that "player.bsn"
// is loaded.
commands.queue_spawn_scene_list(bsn_list! [
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

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#372)

### impl<'w, 's> [CommandsSceneExt](../prelude/trait.CommandsSceneExt.html "trait bevy::prelude::CommandsSceneExt") for [Commands](../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>