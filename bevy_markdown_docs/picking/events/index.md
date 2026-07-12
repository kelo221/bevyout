[bevy](../../index.html)::[picking](../index.html)

# Module events 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#160)

This module defines a stateful set of interaction events driven by the `PointerInput` stream and the hover state of each Pointer.

## Usage

To receive events from this module, you must use an [`Observer`](../../prelude/struct.Observer.html "struct bevy::prelude::Observer") or [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader") with [`Pointer<E>`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events. The simplest example, registering a callback when an entity is hovered over by a pointer, looks like this:

```rust
world.spawn_empty()
    .observe(|event: On<Pointer<Over>>| {
        println!("I am being hovered over");
    });
```

Observers give us three important properties:

1.  They allow for attaching event handlers to specific entities,
2.  they allow events to bubble up the entity hierarchy,
3.  and they allow events of different types to be called in a specific order.

The order in which interaction events are received is extremely important, and you can read more about it on the docs for the dispatcher system: [`pointer_events`](../../prelude/fn.pointer_events.html "fn bevy::prelude::pointer_events"). This system runs in [`PreUpdate`](../../prelude/struct.PreUpdate.html "struct bevy::prelude::PreUpdate") in [`PickingSystems::Hover`](../enum.PickingSystems.html#variant.Hover "variant bevy::picking::PickingSystems::Hover"). All pointer-event observers resolve during the sync point between [`pointer_events`](../../prelude/fn.pointer_events.html "fn bevy::prelude::pointer_events") and [`update_interactions`](../hover/fn.update_interactions.html "fn bevy::picking::hover::update_interactions").

## Events Types

The events this module defines fall into a few broad categories:

*   Hovering and movement: [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over"), [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter"), [`Move`](../../prelude/struct.Move.html "struct bevy::prelude::Move"), [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave"), and [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out").
*   Clicking and pressing: [`Press`](../../prelude/struct.Press.html "struct bevy::prelude::Press"), [`Release`](../../prelude/struct.Release.html "struct bevy::prelude::Release"), and [`Click`](../../prelude/struct.Click.html "struct bevy::prelude::Click").
*   Dragging and dropping: [`DragStart`](../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart"), [`Drag`](../../prelude/struct.Drag.html "struct bevy::prelude::Drag"), [`DragEnd`](../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd"), [`DragEnter`](../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter"), [`DragOver`](../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver"), [`DragDrop`](../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop"), [`DragLeave`](../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave").

When received by an observer, these events will always be wrapped by the [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") type, which contains general metadata about the pointer event.

## Structs

[Cancel](struct.Cancel.html "struct bevy::picking::events::Cancel")

Fires when a pointer is canceled, and its current interaction state is dropped.

[Click](struct.Click.html "struct bevy::picking::events::Click")

Fires when a pointer sends a pointer pressed event followed by a pointer released event, with the same [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") for both events.

[Drag](struct.Drag.html "struct bevy::picking::events::Drag")

Fires while the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") is being dragged.

[DragDrop](struct.DragDrop.html "struct bevy::picking::events::DragDrop")

Fires when a pointer drops the `dropped` entity onto the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragEnd](struct.DragEnd.html "struct bevy::picking::events::DragEnd")

Fires when a pointer is dragging the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") and a pointer released event is received.

[DragEnter](struct.DragEnter.html "struct bevy::picking::events::DragEnter")

Fires when a pointer dragging the `dragged` entity enters the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target")

[DragEntry](struct.DragEntry.html "struct bevy::picking::events::DragEntry")

Dragging state.

[DragLeave](struct.DragLeave.html "struct bevy::picking::events::DragLeave")

Fires when a pointer dragging the `dragged` entity leaves the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragOver](struct.DragOver.html "struct bevy::picking::events::DragOver")

Fires while the `dragged` entity is being dragged over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragStart](struct.DragStart.html "struct bevy::picking::events::DragStart")

Fires when the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") receives a pointer pressed event followed by a pointer move event.

[Enter](struct.Enter.html "struct bevy::picking::events::Enter")

Fires when a pointer crosses into the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over"), this event bubbles up through a subset of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship).

[HoveredEntityAncestors](struct.HoveredEntityAncestors.html "struct bevy::picking::events::HoveredEntityAncestors")

A cache map containing the ancestry of hovered entities

[Leave](struct.Leave.html "struct bevy::picking::events::Leave")

Fires when a pointer crosses out of the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out"), this event bubbles up through a subset of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship).

[Move](struct.Move.html "struct bevy::picking::events::Move")

Fires while a pointer is moving over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Out](struct.Out.html "struct bevy::picking::events::Out")

Fires when a pointer crosses out of the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave"), this event bubbles up to all of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) without restriction. Refer to [`pointer_events`](../../prelude/fn.pointer_events.html "fn bevy::prelude::pointer_events") for more information on how these events are triggered. Refer to [`PointerTraversal`](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal") for how [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events are propagated.

[Over](struct.Over.html "struct bevy::picking::events::Over")

Fires when a pointer crosses into the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter"), this event bubbles up to all of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) without restriction. Refer to [`pointer_events`](../../prelude/fn.pointer_events.html "fn bevy::prelude::pointer_events") for more information on how these events are triggered. Refer to [`PointerTraversal`](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal") for how [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events are propagated.

[PickingMessageWriters](struct.PickingMessageWriters.html "struct bevy::picking::events::PickingMessageWriters")

A helper system param for accessing the picking event writers.

[Pointer](struct.Pointer.html "struct bevy::picking::events::Pointer")

Stores the common data needed for all pointer events.

[PointerButtonState](struct.PointerButtonState.html "struct bevy::picking::events::PointerButtonState")

An entry in the cache that drives the `pointer_events` system, storing additional data about pointer button presses.

[PointerState](struct.PointerState.html "struct bevy::picking::events::PointerState")

State for all pointers.

[PointerTraversal](struct.PointerTraversal.html "struct bevy::picking::events::PointerTraversal")

A traversal query (i.e. it implements [`Traversal`](../../ecs/traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal")) intended for use with [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events.

[PointerTraversalItem](struct.PointerTraversalItem.html "struct bevy::picking::events::PointerTraversalItem")

Automatically generated [`WorldQuery`](../../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`PointerTraversal`](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal"), returned when iterating over query results.

[Press](struct.Press.html "struct bevy::picking::events::Press")

Fires when a pointer button is pressed over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Release](struct.Release.html "struct bevy::picking::events::Release")

Fires when a pointer button is released over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Scroll](struct.Scroll.html "struct bevy::picking::events::Scroll")

Fires while a pointer is scrolling over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

## Functions

[pointer\_events](fn.pointer_events.html "fn bevy::picking::events::pointer_events")

Dispatches interaction events to the target entities.