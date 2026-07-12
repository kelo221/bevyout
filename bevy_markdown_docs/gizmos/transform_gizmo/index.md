[bevy](../../index.html)::[gizmos](../index.html)

# Module transform\_gizmo 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#41)

Interactive transform gizmo for translating, rotating, and scaling entities.

This module provides an opt-in transform gizmo that renders visual handles on a focused entity, allowing the user to click-and-drag to translate, rotate, or scale it. The plugin does **not** handle keyboard input – users set [`TransformGizmoSettings::mode`](../../prelude/struct.TransformGizmoSettings.html#structfield.mode "field bevy::prelude::TransformGizmoSettings::mode") however they like (keyboard shortcuts, UI buttons, gamepad, etc.).

## Quick start

1.  Add [`TransformGizmoPlugin`](../../prelude/struct.TransformGizmoPlugin.html "struct bevy::prelude::TransformGizmoPlugin") to your app.
2.  Mark the camera with [`TransformGizmoCamera`](../../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera").
3.  Tag the entity you want to manipulate with [`TransformGizmoFocus`](../../prelude/struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus").

If there is exactly one camera in the world, the [`TransformGizmoCamera`](../../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera") marker is optional – the gizmo will use that camera automatically. When multiple cameras exist, the marker is required so the gizmo knows which one to use.

## Structs

[TransformGizmoCamera](struct.TransformGizmoCamera.html "struct bevy::gizmos::transform_gizmo::TransformGizmoCamera")

Marker component for the camera the transform gizmo should use.

[TransformGizmoFocus](struct.TransformGizmoFocus.html "struct bevy::gizmos::transform_gizmo::TransformGizmoFocus")

Component that marks the entity the transform gizmo operates on.

[TransformGizmoMeshMarker](struct.TransformGizmoMeshMarker.html "struct bevy::gizmos::transform_gizmo::TransformGizmoMeshMarker")

Marker component for individual gizmo mesh parts.

[TransformGizmoPlugin](struct.TransformGizmoPlugin.html "struct bevy::gizmos::transform_gizmo::TransformGizmoPlugin")

Opt-in plugin that adds the interactive transform gizmo.

[TransformGizmoRoot](struct.TransformGizmoRoot.html "struct bevy::gizmos::transform_gizmo::TransformGizmoRoot")

Marker component for the root entity of the gizmo mesh hierarchy.

[TransformGizmoSettings](struct.TransformGizmoSettings.html "struct bevy::gizmos::transform_gizmo::TransformGizmoSettings")

Configuration and preferences for the transform gizmo.

[TransformGizmoState](struct.TransformGizmoState.html "struct bevy::gizmos::transform_gizmo::TransformGizmoState")

Runtime state of the transform gizmo (drag and hover).

[TransformGizmoSystems](struct.TransformGizmoSystems.html "struct bevy::gizmos::transform_gizmo::TransformGizmoSystems")

System set for the transform gizmo. All transform gizmo systems run in [`PostUpdate`](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") within this set.

## Enums

[TransformGizmoAxis](enum.TransformGizmoAxis.html "enum bevy::gizmos::transform_gizmo::TransformGizmoAxis")

Which axis the user is interacting with.

[TransformGizmoMode](enum.TransformGizmoMode.html "enum bevy::gizmos::transform_gizmo::TransformGizmoMode")

Which manipulation mode the gizmo is in.

[TransformGizmoSpace](enum.TransformGizmoSpace.html "enum bevy::gizmos::transform_gizmo::TransformGizmoSpace")

Whether the gizmo transforms the object using world or local space axes.

## Constants

[AXIS\_HIT\_DISTANCE](constant.AXIS_HIT_DISTANCE.html "constant bevy::gizmos::transform_gizmo::AXIS_HIT_DISTANCE")

Default screen-space pixel distance threshold for hover detection.

[AXIS\_LENGTH](constant.AXIS_LENGTH.html "constant bevy::gizmos::transform_gizmo::AXIS_LENGTH")

Default length of each axis handle.

[AXIS\_START\_OFFSET](constant.AXIS_START_OFFSET.html "constant bevy::gizmos::transform_gizmo::AXIS_START_OFFSET")

Gap between the gizmo center and the start of each axis handle.

[AXIS\_TIP\_LENGTH](constant.AXIS_TIP_LENGTH.html "constant bevy::gizmos::transform_gizmo::AXIS_TIP_LENGTH")

Length of the arrow tip on translate handles.

[COLOR\_VIEW](constant.COLOR_VIEW.html "constant bevy::gizmos::transform_gizmo::COLOR_VIEW")

Color for the view-plane handle (white).

[COLOR\_X](constant.COLOR_X.html "constant bevy::gizmos::transform_gizmo::COLOR_X")

Color for the X axis (magenta-pink).

[COLOR\_Y](constant.COLOR_Y.html "constant bevy::gizmos::transform_gizmo::COLOR_Y")

Color for the Y axis (green).

[COLOR\_Z](constant.COLOR_Z.html "constant bevy::gizmos::transform_gizmo::COLOR_Z")

Color for the Z axis (blue).

[CONE\_HEIGHT](constant.CONE_HEIGHT.html "constant bevy::gizmos::transform_gizmo::CONE_HEIGHT")

Height of the cone mesh used for translate arrow tips.

[CONE\_RADIUS](constant.CONE_RADIUS.html "constant bevy::gizmos::transform_gizmo::CONE_RADIUS")

Radius of the cone mesh used for translate arrow tips.

[INACTIVE\_ALPHA](constant.INACTIVE_ALPHA.html "constant bevy::gizmos::transform_gizmo::INACTIVE_ALPHA")

Alpha value used for inactive (non-hovered) axes during a drag.

[ROTATE\_RING\_RADIUS](constant.ROTATE_RING_RADIUS.html "constant bevy::gizmos::transform_gizmo::ROTATE_RING_RADIUS")

Default radius of the rotation rings.

[SCALE\_CUBE\_SIZE](constant.SCALE_CUBE_SIZE.html "constant bevy::gizmos::transform_gizmo::SCALE_CUBE_SIZE")

Half-size of the scale cube tip.

[SHAFT\_LENGTH](constant.SHAFT_LENGTH.html "constant bevy::gizmos::transform_gizmo::SHAFT_LENGTH")

Height of the cylinder mesh used for axis shafts.

[SHAFT\_RADIUS](constant.SHAFT_RADIUS.html "constant bevy::gizmos::transform_gizmo::SHAFT_RADIUS")

Radius of the cylinder mesh used for axis shafts.

[VIEW\_CIRCLE\_MAJOR](constant.VIEW_CIRCLE_MAJOR.html "constant bevy::gizmos::transform_gizmo::VIEW_CIRCLE_MAJOR")

Major (ring) radius of the view-plane circle torus.

[VIEW\_CIRCLE\_MINOR](constant.VIEW_CIRCLE_MINOR.html "constant bevy::gizmos::transform_gizmo::VIEW_CIRCLE_MINOR")

Minor (tube) radius of the view-plane circle torus.

[VIEW\_RING\_MAJOR](constant.VIEW_RING_MAJOR.html "constant bevy::gizmos::transform_gizmo::VIEW_RING_MAJOR")

Major (ring) radius of the view-axis rotation ring torus.

[VIEW\_RING\_MINOR](constant.VIEW_RING_MINOR.html "constant bevy::gizmos::transform_gizmo::VIEW_RING_MINOR")

Minor (tube) radius of the view-axis rotation ring torus.

## Functions

[axis\_direction](fn.axis_direction.html "fn bevy::gizmos::transform_gizmo::axis_direction")

Get the world-space direction for a given axis.

[effective\_space](fn.effective_space.html "fn bevy::gizmos::transform_gizmo::effective_space")

Return the effective space for the gizmo: scale always uses local space.

[gizmo\_rotation](fn.gizmo_rotation.html "fn bevy::gizmos::transform_gizmo::gizmo_rotation")

Compute the gizmo rotation based on the space setting.

[intersect\_plane](fn.intersect_plane.html "fn bevy::gizmos::transform_gizmo::intersect_plane")

Intersect a ray with a plane defined by a normal and a point on the plane.

[point\_to\_ring\_screen\_dist](fn.point_to_ring_screen_dist.html "fn bevy::gizmos::transform_gizmo::point_to_ring_screen_dist")

Minimum screen-space distance from a cursor position to a 3D ring projected onto screen.

[point\_to\_segment\_dist](fn.point_to_segment_dist.html "fn bevy::gizmos::transform_gizmo::point_to_segment_dist")

Distance from a point to a line segment in 2D.

[translation\_plane\_normal](fn.translation_plane_normal.html "fn bevy::gizmos::transform_gizmo::translation_plane_normal")

Construct the constraint plane normal for axis translation/scale.