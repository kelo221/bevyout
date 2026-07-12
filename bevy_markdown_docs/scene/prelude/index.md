[bevy](../../index.html)::[scene](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/lib.rs.html#900)

The Bevy Scene prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Macros

[bsn\_list](macro.bsn_list.html "macro bevy::scene::prelude::bsn_list")

Creates a `SceneList` using BSN (Bevy Scene Notation) syntax.

## Structs

[ScenePatchInstance](struct.ScenePatchInstance.html "struct bevy::scene::prelude::ScenePatchInstance")

A component that, when added, will queue applying the given [`ScenePatch`](../struct.ScenePatch.html "struct bevy::scene::ScenePatch") after the scene and its dependencies have been loaded and resolved.

## Traits

[CommandsSceneExt](trait.CommandsSceneExt.html "trait bevy::scene::prelude::CommandsSceneExt")

Adds scene spawning functionality to [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands").

[EntityCommandsSceneExt](trait.EntityCommandsSceneExt.html "trait bevy::scene::prelude::EntityCommandsSceneExt")

Adds scene functionality to [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[EntityWorldMutSceneExt](trait.EntityWorldMutSceneExt.html "trait bevy::scene::prelude::EntityWorldMutSceneExt")

Adds scene functionality to [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::scene::prelude::PatchFromTemplate")

A helper function that returns a [`TemplatePatch`](../struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"). It will use [`FromTemplate::Template`](../../prelude/trait.FromTemplate.html#associatedtype.Template "associated type bevy::prelude::FromTemplate::Template") as the “patched template”.

[PatchTemplate](trait.PatchTemplate.html "trait bevy::scene::prelude::PatchTemplate")

A helper function that returns a [`TemplatePatch`](../struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Scene](trait.Scene.html "trait bevy::scene::prelude::Scene")

Conceptually, a [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") describes what a spawned [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") should look like. This often describes what [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s the entity should have.

[SceneComponent](trait.SceneComponent.html "trait bevy::scene::prelude::SceneComponent")

Implemented for [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s that have an associated [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), which can be constructed with [`Self::Props`](../../prelude/trait.SceneComponent.html#associatedtype.Props "associated type bevy::prelude::SceneComponent::Props").

[SceneList](trait.SceneList.html "trait bevy::scene::prelude::SceneList")

This behaves like a list of [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"), where each entry in the list is a new entity (see [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") for more details).

[SpawnListSystem](trait.SpawnListSystem.html "trait bevy::scene::prelude::SpawnListSystem")

Returns a system that spawns the given [`SceneList`](../../prelude/trait.SceneList.html "trait bevy::prelude::SceneList"). This should generally only be added to schedules that run once, such as [`Startup`](../../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[SpawnSystem](trait.SpawnSystem.html "trait bevy::scene::prelude::SpawnSystem")

Returns a system that spawns the given [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene"). This should generally only be added to schedules that run once, such as [`Startup`](../../prelude/struct.Startup.html "struct bevy::prelude::Startup").

[WorldSceneExt](trait.WorldSceneExt.html "trait bevy::scene::prelude::WorldSceneExt")

Adds scene spawning functionality to [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Functions

[on](fn.on.html "fn bevy::scene::prelude::on")

Returns an [`OnTemplate`](../struct.OnTemplate.html "struct bevy::scene::OnTemplate") that will create an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") of a given [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") on the current [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") entity.

[template\_value](fn.template_value.html "fn bevy::scene::prelude::template_value")

Returns a [`Scene`](../../prelude/trait.Scene.html "trait bevy::prelude::Scene") that completely overwrites the current value of a [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") `T` with the given `value`. The `value` is cloned each time the [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") is built.

## Derive Macros

[SceneComponent](derive.SceneComponent.html "derive bevy::scene::prelude::SceneComponent")