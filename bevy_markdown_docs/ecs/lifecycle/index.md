[bevy](../../index.html)::[ecs](../index.html)

# Module lifecycle 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#41)

This module contains various tools to allow you to react to component insertion or removal, as well as entity spawning and despawning.

There are four main ways to react to these lifecycle events:

1.  Using component hooks, which act as inherent constructors and destructors for components.
2.  Using [observers](../observer/index.html "mod bevy::ecs::observer"), which are a user-extensible way to respond to events, including component lifecycle events.
3.  Using the [`RemovedComponents`](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents") system parameter, which offers an event-style interface.
4.  Using the [`Added`](../../prelude/struct.Added.html "struct bevy::prelude::Added") query filter, which checks each component to see if it has been added since the last time a system ran.

## Types of lifecycle events

There are five types of lifecycle events, split into two categories. First, we have lifecycle events that are triggered when a component is added to an entity:

*   [`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add"): Triggered when a component is added to an entity that did not already have it.
*   [`Insert`](../../prelude/struct.Insert.html "struct bevy::prelude::Insert"): Triggered when a component is added to an entity, regardless of whether it already had it.

When both events occur, [`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add") hooks are evaluated before [`Insert`](../../prelude/struct.Insert.html "struct bevy::prelude::Insert").

Next, we have lifecycle events that are triggered when a component is removed from an entity:

*   [`Discard`](../../prelude/struct.Discard.html "struct bevy::prelude::Discard"): Triggered when a component is removed from an entity, regardless if it is then replaced with a new value.
*   [`Remove`](../../prelude/struct.Remove.html "struct bevy::prelude::Remove"): Triggered when a component is removed from an entity and not replaced, before the component is removed.
*   [`Despawn`](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn"): Triggered for each component on an entity when it is despawned.

[`Discard`](../../prelude/struct.Discard.html "struct bevy::prelude::Discard") hooks are evaluated before [`Remove`](../../prelude/struct.Remove.html "struct bevy::prelude::Remove"), then finally [`Despawn`](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn") hooks are evaluated.

[`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add") and [`Remove`](../../prelude/struct.Remove.html "struct bevy::prelude::Remove") are counterparts: they are only triggered when a component is added or removed from an entity in such a way as to cause a change in the component’s presence on that entity. Similarly, [`Insert`](../../prelude/struct.Insert.html "struct bevy::prelude::Insert") and [`Discard`](../../prelude/struct.Discard.html "struct bevy::prelude::Discard") are counterparts: they are triggered when a component is added or overwritten on an entity, regardless of whether this results in a change in the component’s presence on that entity.

To reliably synchronize data structures using with component lifecycle events, you can combine [`Insert`](../../prelude/struct.Insert.html "struct bevy::prelude::Insert") and [`Discard`](../../prelude/struct.Discard.html "struct bevy::prelude::Discard") to fully capture any changes to the data. This is particularly useful in combination with immutable components, to avoid any lifecycle-bypassing mutations.

### Lifecycle events and component types

Despite the absence of generics, each lifecycle event is associated with a specific component. When defining a component hook for a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") type, that component is used. When observers watch lifecycle events, the `B: Bundle` generic is used.

Each of these lifecycle events also corresponds to a fixed [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), which are assigned during [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") initialization. For example, [`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add") corresponds to [`ADD`](constant.ADD.html "constant bevy::ecs::lifecycle::ADD"). This is used to skip [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") lookups in hot paths.

## Structs

[Add](struct.Add.html "struct bevy::ecs::lifecycle::Add")

Trigger emitted when a component is inserted onto an entity that does not already have that component. Runs before `Insert`. See [`ComponentHooks::on_add`](struct.ComponentHooks.html#method.on_add "method bevy::ecs::lifecycle::ComponentHooks::on_add") for more information.

[ComponentHooks](struct.ComponentHooks.html "struct bevy::ecs::lifecycle::ComponentHooks")

[`World`](../../prelude/struct.World.html "struct bevy::prelude::World")\-mutating functions that run as part of lifecycle events of a [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component").

[Despawn](struct.Despawn.html "struct bevy::ecs::lifecycle::Despawn")

[`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") emitted for each component on an entity when it is despawned. See [`ComponentHooks::on_despawn`](struct.ComponentHooks.html#method.on_despawn "method bevy::ecs::lifecycle::ComponentHooks::on_despawn") for more information.

[Discard](struct.Discard.html "struct bevy::ecs::lifecycle::Discard")

Trigger emitted when a component is removed from an entity, regardless of whether or not it is later replaced.

[HookContext](struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext")

Context provided to a [`ComponentHook`](type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook").

[Insert](struct.Insert.html "struct bevy::ecs::lifecycle::Insert")

Trigger emitted when a component is inserted, regardless of whether or not the entity already had that component. Runs after `Add`, if it ran. See [`ComponentHooks::on_insert`](struct.ComponentHooks.html#method.on_insert "method bevy::ecs::lifecycle::ComponentHooks::on_insert") for more information.

[Remove](struct.Remove.html "struct bevy::ecs::lifecycle::Remove")

Trigger emitted when a component is removed from an entity, and runs before the component is removed, so you can still access the component data. See [`ComponentHooks::on_remove`](struct.ComponentHooks.html#method.on_remove "method bevy::ecs::lifecycle::ComponentHooks::on_remove") for more information.

[RemovedComponentEntity](struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

Wrapper around [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") for [`RemovedComponents`](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents"). Internally, `RemovedComponents` uses these as an [`Messages<RemovedComponentEntity>`](../../prelude/struct.Messages.html "struct bevy::prelude::Messages").

[RemovedComponentMessages](struct.RemovedComponentMessages.html "struct bevy::ecs::lifecycle::RemovedComponentMessages")

Stores the [`RemovedComponents`](../../prelude/struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents") event buffers for all types of component in a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[RemovedComponentReader](struct.RemovedComponentReader.html "struct bevy::ecs::lifecycle::RemovedComponentReader")

Wrapper around a [`MessageCursor<RemovedComponentEntity>`](../message/struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor") so that we can differentiate messages between components.

[RemovedComponents](struct.RemovedComponents.html "struct bevy::ecs::lifecycle::RemovedComponents")

A [`SystemParam`](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that yields entities that had their `T` [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") removed or have been despawned with it.

## Constants

[ADD](constant.ADD.html "constant bevy::ecs::lifecycle::ADD")

[`EventKey`](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for [`Add`](../../prelude/struct.Add.html "struct bevy::prelude::Add")

[DESPAWN](constant.DESPAWN.html "constant bevy::ecs::lifecycle::DESPAWN")

[`EventKey`](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for [`Despawn`](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn")

[DISCARD](constant.DISCARD.html "constant bevy::ecs::lifecycle::DISCARD")

[`EventKey`](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for [`Discard`](../../prelude/struct.Discard.html "struct bevy::prelude::Discard")

[INSERT](constant.INSERT.html "constant bevy::ecs::lifecycle::INSERT")

[`EventKey`](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for [`Insert`](../../prelude/struct.Insert.html "struct bevy::prelude::Insert")

[REMOVE](constant.REMOVE.html "constant bevy::ecs::lifecycle::REMOVE")

[`EventKey`](../event/struct.EventKey.html "struct bevy::ecs::event::EventKey") for [`Remove`](../../prelude/struct.Remove.html "struct bevy::prelude::Remove")

## Type Aliases

[ComponentHook](type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook")

The type used for [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") lifecycle hooks such as `on_add`, `on_insert` or `on_remove`.

[RemovedIter](type.RemovedIter.html "type bevy::ecs::lifecycle::RemovedIter")

Iterator over entities that had a specific component removed.

[RemovedIterWithId](type.RemovedIterWithId.html "type bevy::ecs::lifecycle::RemovedIterWithId")

Iterator over entities that had a specific component removed.