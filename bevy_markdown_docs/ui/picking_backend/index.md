[bevy](../../index.html)::[ui](../index.html)

# Module picking\_backend 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#23)

Available on **crate feature `bevy_picking`** only.

A picking backend for UI nodes.

## Usage

This backend does not require markers on cameras or entities to function. It will look for any pointers using the same render target as the UI camera, and run hit tests on the UI node tree.

### Important Note

This backend completely ignores [`FocusPolicy`](../enum.FocusPolicy.html "enum bevy::ui::FocusPolicy"). The design of `bevy_ui`’s focus systems and the picking plugin are not compatible. Instead, use the optional [`Pickable`](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable") component to override how an entity responds to picking focus. Nodes without the [`Pickable`](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable") component will still trigger events and block items below it from being hovered.

### Implementation Notes

*   `bevy_ui` can render on any camera with a flag, it is special, and is not tied to a particular camera.
*   To correctly sort picks, the order of `bevy_ui` is set to be the camera order plus 0.5.
*   The `position` reported in `HitData` is normalized relative to the node, with `(-0.5, -0.5, 0.)` at the top left and `(0.5, 0.5, 0.)` in the bottom right. Coordinates are relative to the entire node, not just the visible region. This backend does not provide a `normal`.

## Structs

[NodeQuery](struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

Main query from bevy’s `ui_focus_system`

[NodeQueryItem](struct.NodeQueryItem.html "struct bevy::ui::picking_backend::NodeQueryItem")

Automatically generated [`WorldQuery`](../../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`NodeQuery`](struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery"), returned when iterating over query results.

[NodeQueryReadOnly](struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

Automatically generated [`WorldQuery`](../../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") type for a read-only variant of [`NodeQuery`](struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery").

[NodeQueryReadOnlyItem](struct.NodeQueryReadOnlyItem.html "struct bevy::ui::picking_backend::NodeQueryReadOnlyItem")

Automatically generated [`WorldQuery`](../../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`NodeQueryReadOnly`](struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly"), returned when iterating over query results.

[UiPickingCamera](struct.UiPickingCamera.html "struct bevy::ui::picking_backend::UiPickingCamera")

An optional component that marks cameras that should be used in the [`UiPickingPlugin`](../../prelude/struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin").

[UiPickingPlugin](struct.UiPickingPlugin.html "struct bevy::ui::picking_backend::UiPickingPlugin")

A plugin that adds picking support for UI nodes.

[UiPickingSettings](struct.UiPickingSettings.html "struct bevy::ui::picking_backend::UiPickingSettings")

Runtime settings for the [`UiPickingPlugin`](../../prelude/struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin").

## Functions

[ui\_picking](fn.ui_picking.html "fn bevy::ui::picking_backend::ui_picking")

Computes the UI node entities under each pointer.