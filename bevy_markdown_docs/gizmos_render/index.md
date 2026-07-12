[bevy](../index.html)

# Crate gizmos\_render 

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#1-645)

This crate renders `bevy_gizmos` with `bevy_render`.

## Modules

[retained](retained/index.html "mod bevy::gizmos_render::retained")

This module is for ‘retained’ alternatives to the ‘immediate mode’ [`Gizmos`](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos") system parameter.

[transform\_gizmo\_render](transform_gizmo_render/index.html "mod bevy::gizmos_render::transform_gizmo_render")`bevy_pbr`

Mesh-based rendering for the transform gizmo.

## Structs

[GizmoRenderPlugin](struct.GizmoRenderPlugin.html "struct bevy::gizmos_render::GizmoRenderPlugin")

A [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that provides an immediate mode drawing api for visual debugging.

[LineGizmoEntities](struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

Holds entities that the gizmo render phase items are associated with.

## Enums

[GizmoRenderSystems](enum.GizmoRenderSystems.html "enum bevy::gizmos_render::GizmoRenderSystems")

System set label for the systems handling the rendering of gizmos.