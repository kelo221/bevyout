[bevy](../../index.html)::[picking](../index.html)

# Module hover 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#161)

Determines which entities are being hovered by which pointers.

The most important type in this module is the [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap"), which maps pointers to the entities they are hovering over.

## Structs

[DirectlyHovered](struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered")

A component that allows users to use regular Bevy change detection to determine when the pointer is directly hovering over an entity. Users should insert this component on an entity to indicate interest in knowing about hover state changes.

[HoverMap](struct.HoverMap.html "struct bevy::picking::hover::HoverMap")

The source of truth for all hover state. This is used to determine what events to send, and what state components should be in.

[Hovered](struct.Hovered.html "struct bevy::picking::hover::Hovered")

A component that allows users to use regular Bevy change detection to determine when the pointer enters or leaves an entity. Users should insert this component on an entity to indicate interest in knowing about hover state changes.

[PreviousHoverMap](struct.PreviousHoverMap.html "struct bevy::picking::hover::PreviousHoverMap")

The previous state of the hover map, used to track changes to hover state.

## Enums

[PickingInteraction](enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction")

A component that aggregates picking interaction state of this entity across all pointers.

## Functions

[generate\_hovermap](fn.generate_hovermap.html "fn bevy::picking::hover::generate_hovermap")

Coalesces all data from inputs and backends to generate a map of the currently hovered entities. This is the final focusing step to determine which entity the pointer is hovering over.

[update\_interactions](fn.update_interactions.html "fn bevy::picking::hover::update_interactions")

Uses [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap") changes to update [`PointerInteraction`](../pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction") and [`PickingInteraction`](enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction") components.

[update\_is\_directly\_hovered](fn.update_is_directly_hovered.html "fn bevy::picking::hover::update_is_directly_hovered")

Uses [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap") changes to update [`DirectlyHovered`](struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered") components.

[update\_is\_hovered](fn.update_is_hovered.html "fn bevy::picking::hover::update_is_hovered")

Uses [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap") changes to update [`Hovered`](struct.Hovered.html "struct bevy::picking::hover::Hovered") components.