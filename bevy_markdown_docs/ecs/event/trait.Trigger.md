[bevy](../../index.html)::[ecs](../index.html)::[event](index.html)

# Trait Trigger 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#38)

```rust
pub unsafe trait Trigger<E>where
    E: Event,{
    // Required method
    unsafe fn trigger(
        &mut self,
        world: DeferredWorld<'_>,
        observers: &CachedObservers,
        trigger_context: &TriggerContext,
        event: &mut E,
    );
}
```

[`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") determines _how_ an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") is triggered when [`World::trigger`](../../prelude/struct.World.html#method.trigger "method bevy::prelude::World::trigger") is called. This decides which [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s will run, what data gets passed to them, and the order they will be executed in.

Implementing [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") is “advanced-level” territory, and is generally unnecessary unless you are developing highly specialized [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") trigger logic.

Bevy comes with a number of built-in [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") implementations (see their documentation for more info):

*   [`GlobalTrigger`](struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger"): The [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") derive defaults to using this
*   [`EntityTrigger`](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger"): The [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") derive defaults to using this
*   [`PropagateEntityTrigger`](struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger"): The [`EntityEvent`](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") derive uses this when propagation is enabled.
*   [`EntityComponentsTrigger`](struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger"): Used by Bevy’s [component lifecycle events](../lifecycle/index.html "mod bevy::ecs::lifecycle").

## Safety

Implementing this properly is _advanced_ soundness territory! Implementers must abide by the following:

*   The `E`’ [`Event::Trigger`](../../prelude/trait.Event.html#associatedtype.Trigger "associated type bevy::prelude::Event::Trigger") must be constrained to the implemented [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") type, as part of the implementation. This prevents other [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") implementations from directly deferring to your implementation, which is a very easy soundness misstep, as most [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") implementations will invoke observers that are developed _for their specific [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") type_. Without this constraint, something like [`GlobalTrigger`](struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger") could be called for _any_ [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") type, even one that expects a different [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") type. This would result in an unsound cast of [`GlobalTrigger`](struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger") reference. This is not expressed as an explicit type constraint,, as the `for<'a> Event::Trigger<'a>` lifetime can mismatch explicit lifetimes in some impls.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#49-55)

#### unsafe fn [trigger](#tymethod.trigger)( &mut self, world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, observers: &[CachedObservers](../observer/struct.CachedObservers.html "struct bevy::ecs::observer::CachedObservers"), trigger\_context: &[TriggerContext](../observer/struct.TriggerContext.html "struct bevy::ecs::observer::TriggerContext"), event: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html), )

Trigger the given `event`, running every [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") that matches the `event`, as defined by this [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") and the state stored on `self`.

##### Safety

*   The [`CachedObservers`](../observer/struct.CachedObservers.html "struct bevy::ecs::observer::CachedObservers") `observers` must come from the [`DeferredWorld`](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld") `world`
*   [`TriggerContext`](../observer/struct.TriggerContext.html "struct bevy::ecs::observer::TriggerContext") must contain an [`EventKey`](struct.EventKey.html "struct bevy::ecs::event::EventKey") that matches the `E` [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event") type
*   `observers` must correspond to observers compatible with the event type `E`
*   Read and abide by the “Safety” section defined in the top-level [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") docs. Calling this function is unintuitively risky. _Do not use it directly unless you know what you are doing_. Importantly, this should only be called for an `event` whose [`Event::Trigger`](../../prelude/trait.Event.html#associatedtype.Trigger "associated type bevy::prelude::Event::Trigger") matches this trigger.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#427-428)

### impl<'a, E> [Trigger](trait.Trigger.html "trait bevy::ecs::event::Trigger")<E> for [EntityComponentsTrigger](struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent")<Trigger<'a> = [EntityComponentsTrigger](struct.EntityComponentsTrigger.html "struct bevy::ecs::event::EntityComponentsTrigger")<'a>> + [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_event.rs.html#34-35)

### impl<E> [Trigger](trait.Trigger.html "trait bevy::ecs::event::Trigger")<E> for [AnimationEventTrigger](../../animation/struct.AnimationEventTrigger.html "struct bevy::animation::AnimationEventTrigger")

where E: [AnimationEvent](../../animation/trait.AnimationEvent.html "trait bevy::animation::AnimationEvent")<Trigger<'a> = [AnimationEventTrigger](../../animation/struct.AnimationEventTrigger.html "struct bevy::animation::AnimationEventTrigger")\> + for<'a> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#138)

### impl<E> [Trigger](trait.Trigger.html "trait bevy::ecs::event::Trigger")<E> for [EntityTrigger](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent")<Trigger<'a> = [EntityTrigger](struct.EntityTrigger.html "struct bevy::ecs::event::EntityTrigger")\> + for<'a> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#68)

### impl<E> [Trigger](trait.Trigger.html "trait bevy::ecs::event::Trigger")<E> for [GlobalTrigger](struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger")

where E: for<'a> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event")<Trigger<'a> = [GlobalTrigger](struct.GlobalTrigger.html "struct bevy::ecs::event::GlobalTrigger")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#270-274)

### impl<const AUTO\_PROPAGATE: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), E, T> [Trigger](trait.Trigger.html "trait bevy::ecs::event::Trigger")<E> for [PropagateEntityTrigger](struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<AUTO\_PROPAGATE, E, T>

where E: [EntityEvent](../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent")<Trigger<'a> = [PropagateEntityTrigger](struct.PropagateEntityTrigger.html "struct bevy::ecs::event::PropagateEntityTrigger")<AUTO\_PROPAGATE, E, T>> + [SetEntityEventTarget](trait.SetEntityEventTarget.html "trait bevy::ecs::event::SetEntityEventTarget") + for<'a> [Event](../../prelude/trait.Event.html "trait bevy::prelude::Event"), T: [Traversal](../traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal")<E>,