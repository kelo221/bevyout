[bevy](../../index.html)::[ecs](../index.html)

# Module observer 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#45)

Observers are a push-based tool for responding to [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event")s. The [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") component holds a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System") that runs whenever a matching [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") is triggered.

See [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") and [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") for in-depth documentation and usage examples.

## Structs

[CachedComponentObservers](struct.CachedComponentObservers.html "struct bevy::ecs::observer::CachedComponentObservers")

Collection of [`ObserverRunner`](type.ObserverRunner.html "type bevy::ecs::observer::ObserverRunner") for [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") registered to a particular event targeted at a specific component.

[CachedObservers](struct.CachedObservers.html "struct bevy::ecs::observer::CachedObservers")

Collection of [`ObserverRunner`](type.ObserverRunner.html "type bevy::ecs::observer::ObserverRunner") for [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") registered to a particular event.

[ObservedBy](struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy")

Tracks a list of entity observers for the [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") [`ObservedBy`](struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy") is added to.

[Observer](struct.Observer.html "struct bevy::ecs::observer::Observer")

An [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") system. Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to turn it into an “observer”.

[ObserverDescriptor](struct.ObserverDescriptor.html "struct bevy::ecs::observer::ObserverDescriptor")

Store information about what an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") observes.

[ObserverWithCondition](struct.ObserverWithCondition.html "struct bevy::ecs::observer::ObserverWithCondition")

An observer system with run conditions that preserves event type information.

[Observers](struct.Observers.html "struct bevy::ecs::observer::Observers")

An internal lookup table tracking all of the observers in the world.

[On](struct.On.html "struct bevy::ecs::observer::On")

A [system parameter](../system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") used by an observer to process events. See [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") and [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") for examples.

[TriggerContext](struct.TriggerContext.html "struct bevy::ecs::observer::TriggerContext")

Metadata about a specific [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") that triggered an observer.

## Traits

[IntoEntityObserver](trait.IntoEntityObserver.html "trait bevy::ecs::observer::IntoEntityObserver")

Trait for types that can be converted into an entity-targeting [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer").

[IntoObserver](trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")

Trait for types that can be converted into an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer").

[ObserverSystemExt](trait.ObserverSystemExt.html "trait bevy::ecs::observer::ObserverSystemExt")

Extension trait for adding run conditions to observer systems.

## Type Aliases

[ObserverMap](type.ObserverMap.html "type bevy::ecs::observer::ObserverMap")

Map between an observer entity and its [`ObserverRunner`](type.ObserverRunner.html "type bevy::ecs::observer::ObserverRunner")

[ObserverRunner](type.ObserverRunner.html "type bevy::ecs::observer::ObserverRunner")

Type for function that is run when an observer is triggered.