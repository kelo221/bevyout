[bevy](../../index.html)::[ecs](../index.html)

# Module event 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#37)

[`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") functionality.

## Structs

[EntityComponentsTrigger](struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")

An [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") that, in addition to behaving like a normal [`EntityTrigger`](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger"), _also_ runs observers that watch for components that match the slice of [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s referenced in [`EntityComponentsTrigger`](struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger"). This includes both _global_ observers of those components and “entity scoped” observers that watch the [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[EntityTrigger](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

An [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") that does two things:

[EventKey](struct.EventKey.html "struct bevy::ecs::event::EventKey")

A unique identifier for an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event"), used by [observers](../observer/index.html "mod bevy::ecs::observer").

[GlobalTrigger](struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger")

A [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") that runs _every_ “global” [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") (ex: registered via [`World::add_observer`](../../prelude/struct.World.html#method.add_observer "method bevy::prelude::World::add_observer")) that matches the given [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event").

[PropagateEntityTrigger](struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")

An [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") that behaves like [`EntityTrigger`](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger"), but “propagates” the event using an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") [`Traversal`](../traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal"). At each step in the propagation, the [`EntityTrigger`](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger") logic will be run, until [`PropagateEntityTrigger::propagate`](struct.PropagateEntityTrigger.html#structfield.propagate "field bevy::ecs::event::PropagateEntityTrigger::propagate") is false, or there are no entities left to traverse.

## Traits

[EntityEvent](trait.EntityEvent.html "trait bevy::ecs::event::EntityEvent")

An [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") is an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") that is triggered for a specific [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") entity:

[Event](trait.Event.html "trait bevy::ecs::event::Event")

An [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") is something that “happens” at a given moment.

[SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget")

A trait which is used to set the target of an [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent").

[Trigger](trait.Trigger.html "trait bevy::ecs::event::Trigger")

[`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") determines _how_ an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") is triggered when [`World::trigger`](../../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger") is called. This decides which [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s will run, what data gets passed to them, and the order they will be executed in.

## Functions

[trigger\_entity\_internal](fn.trigger_entity_internal.html "fn bevy::ecs::event::trigger_entity_internal")⚠

Trigger observers watching for the given entity event. The `target_entity` should match the [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") on `event` for logical correctness.

## Derive Macros

[EntityEvent](derive.EntityEvent.html "derive bevy::ecs::event::EntityEvent")

Cheat sheet for derive syntax, see full explanation on `EntityEvent` trait docs.

[Event](derive.Event.html "derive bevy::ecs::event::Event")

Implement the `Event` trait.