[bevy](../../index.html)::[ecs](../index.html)::[event](index.html)

# Function trigger\_entity\_internal 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/event/trigger.rs.html#176-183)

```rust
pub unsafe fn trigger_entity_internal(
    world: DeferredWorld<'_>,
    observers: &CachedObservers,
    event: PtrMut<'_>,
    trigger: PtrMut<'_>,
    target_entity: Entity,
    trigger_context: &TriggerContext,
)
```

Trigger observers watching for the given entity event. The `target_entity` should match the [`EntityEvent::event_target`](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") on `event` for logical correctness.

## Safety

*   `observers` must come from the `world` [`DeferredWorld`](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld"), and correspond to observers that match the `event` type
*   `event` must point to an [`Event`](../../prelude/trait.Event.html "trait bevy::prelude::Event")
*   `trigger` must correspond to the [`Event::Trigger`](../../prelude/trait.Event.html#associatedtype.Trigger "associated type bevy::prelude::Event::Trigger") type expected by the `event`
*   `trigger_context`’s [`TriggerContext::event_key`](../observer/struct.TriggerContext.html#structfield.event_key "field bevy::ecs::observer::TriggerContext::event_key") must correspond to the `event` type.
*   Read, understand, and abide by the [`Trigger`](trait.Trigger.html "trait bevy::ecs::event::Trigger") safety documentation