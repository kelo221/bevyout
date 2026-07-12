[bevy](../../index.html)::[picking](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#176)

The picking prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Cancel](struct.Cancel.html "struct bevy::picking::prelude::Cancel")

Fires when a pointer is canceled, and its current interaction state is dropped.

[Click](struct.Click.html "struct bevy::picking::prelude::Click")

Fires when a pointer sends a pointer pressed event followed by a pointer released event, with the same [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") for both events.

[DefaultPickingPlugins](struct.DefaultPickingPlugins.html "struct bevy::picking::prelude::DefaultPickingPlugins")

One plugin that contains the [`PointerInputPlugin`](../../prelude/struct.PointerInputPlugin.html "struct bevy::prelude::PointerInputPlugin"), [`PickingPlugin`](../../prelude/struct.PickingPlugin.html "struct bevy::prelude::PickingPlugin") and the [`InteractionPlugin`](../../prelude/struct.InteractionPlugin.html "struct bevy::prelude::InteractionPlugin"), this is probably the plugin that will be most used.

[Drag](struct.Drag.html "struct bevy::picking::prelude::Drag")

Fires while the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") is being dragged.

[DragDrop](struct.DragDrop.html "struct bevy::picking::prelude::DragDrop")

Fires when a pointer drops the `dropped` entity onto the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragEnd](struct.DragEnd.html "struct bevy::picking::prelude::DragEnd")

Fires when a pointer is dragging the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") and a pointer released event is received.

[DragEnter](struct.DragEnter.html "struct bevy::picking::prelude::DragEnter")

Fires when a pointer dragging the `dragged` entity enters the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target")

[DragEntry](struct.DragEntry.html "struct bevy::picking::prelude::DragEntry")

Dragging state.

[DragLeave](struct.DragLeave.html "struct bevy::picking::prelude::DragLeave")

Fires when a pointer dragging the `dragged` entity leaves the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragOver](struct.DragOver.html "struct bevy::picking::prelude::DragOver")

Fires while the `dragged` entity is being dragged over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragStart](struct.DragStart.html "struct bevy::picking::prelude::DragStart")

Fires when the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") receives a pointer pressed event followed by a pointer move event.

[Enter](struct.Enter.html "struct bevy::picking::prelude::Enter")

Fires when a pointer crosses into the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over"), this event bubbles up through a subset of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship).

[HoveredEntityAncestors](struct.HoveredEntityAncestors.html "struct bevy::picking::prelude::HoveredEntityAncestors")

A cache map containing the ancestry of hovered entities

[InteractionPlugin](struct.InteractionPlugin.html "struct bevy::picking::prelude::InteractionPlugin")

Generates [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events and handles event bubbling.

[Leave](struct.Leave.html "struct bevy::picking::prelude::Leave")

Fires when a pointer crosses out of the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out"), this event bubbles up through a subset of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship).

[MeshPickingCamera](struct.MeshPickingCamera.html "struct bevy::picking::prelude::MeshPickingCamera")

An optional component that marks cameras that should be used in the [`MeshPickingPlugin`](../../prelude/struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin").

[MeshPickingPlugin](struct.MeshPickingPlugin.html "struct bevy::picking::prelude::MeshPickingPlugin")

Adds the mesh picking backend to your app.

[MeshPickingSettings](struct.MeshPickingSettings.html "struct bevy::picking::prelude::MeshPickingSettings")

Runtime settings for the [`MeshPickingPlugin`](../../prelude/struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin").

[MeshRayCast](struct.MeshRayCast.html "struct bevy::picking::prelude::MeshRayCast")

Add this ray casting [`SystemParam`](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") to your system to cast rays into the world with an immediate-mode API. Call `cast_ray` to immediately perform a ray cast and get a result.

[MeshRayCastSettings](struct.MeshRayCastSettings.html "struct bevy::picking::prelude::MeshRayCastSettings")

Settings for a ray cast.

[Move](struct.Move.html "struct bevy::picking::prelude::Move")

Fires while a pointer is moving over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Out](struct.Out.html "struct bevy::picking::prelude::Out")

Fires when a pointer crosses out of the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave"), this event bubbles up to all of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) without restriction. Refer to [`pointer_events`](../../prelude/fn.pointer_events.html "fn bevy::prelude::pointer_events") for more information on how these events are triggered. Refer to [`PointerTraversal`](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal") for how [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events are propagated.

[Over](struct.Over.html "struct bevy::picking::prelude::Over")

Fires when a pointer crosses into the bounds of a [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter"), this event bubbles up to all of the [target entity’s](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) without restriction. Refer to [`pointer_events`](../../prelude/fn.pointer_events.html "fn bevy::prelude::pointer_events") for more information on how these events are triggered. Refer to [`PointerTraversal`](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal") for how [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events are propagated.

[Pickable](struct.Pickable.html "struct bevy::picking::prelude::Pickable")

An optional component that marks an entity as usable by a backend, and overrides default picking behavior for an entity.

[PickingMessageWriters](struct.PickingMessageWriters.html "struct bevy::picking::prelude::PickingMessageWriters")

A helper system param for accessing the picking event writers.

[PickingPlugin](struct.PickingPlugin.html "struct bevy::picking::prelude::PickingPlugin")

This plugin sets up the core picking infrastructure. It receives input events, and provides the shared types used by other picking plugins.

[Pointer](struct.Pointer.html "struct bevy::picking::prelude::Pointer")

Stores the common data needed for all pointer events.

[PointerButtonState](struct.PointerButtonState.html "struct bevy::picking::prelude::PointerButtonState")

An entry in the cache that drives the `pointer_events` system, storing additional data about pointer button presses.

[PointerInputPlugin](struct.PointerInputPlugin.html "struct bevy::picking::prelude::PointerInputPlugin")

Adds mouse and touch inputs for picking pointers to your app. This is a default input plugin, that you can replace with your own plugin as needed.

[PointerState](struct.PointerState.html "struct bevy::picking::prelude::PointerState")

State for all pointers.

[PointerTraversal](struct.PointerTraversal.html "struct bevy::picking::prelude::PointerTraversal")

A traversal query (i.e. it implements [`Traversal`](../../ecs/traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal")) intended for use with [`Pointer`](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer") events.

[PointerTraversalItem](struct.PointerTraversalItem.html "struct bevy::picking::prelude::PointerTraversalItem")

Automatically generated [`WorldQuery`](../../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`PointerTraversal`](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal"), returned when iterating over query results.

[Press](struct.Press.html "struct bevy::picking::prelude::Press")

Fires when a pointer button is pressed over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[RayCastBackfaces](struct.RayCastBackfaces.html "struct bevy::picking::prelude::RayCastBackfaces")

Disables backface culling for [ray casts](../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast") on this entity.

[Release](struct.Release.html "struct bevy::picking::prelude::Release")

Fires when a pointer button is released over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Scroll](struct.Scroll.html "struct bevy::picking::prelude::Scroll")

Fires while a pointer is scrolling over the [target entity](../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

## Enums

[PointerButton](enum.PointerButton.html "enum bevy::picking::prelude::PointerButton")

The button that was just pressed or released

[RayCastVisibility](enum.RayCastVisibility.html "enum bevy::picking::prelude::RayCastVisibility")

How a ray cast should handle [`Visibility`](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility").

## Functions

[pointer\_events](fn.pointer_events.html "fn bevy::picking::prelude::pointer_events")

Dispatches interaction events to the target entities.