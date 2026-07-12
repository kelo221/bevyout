[bevy](../index.html)

# Crate gizmos 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#1-363)

This crate adds an immediate mode drawing api to Bevy for visual debugging.

## Example

```rust
fn system(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, GREEN);
}
```

See the documentation on [Gizmos](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos") for more examples.

## Modules

[aabb](aabb/index.html "mod bevy::gizmos::aabb")

A module adding debug visualization of [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb")s.

[arcs](arcs/index.html "mod bevy::gizmos::arcs")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Arcs

[arrows](arrows/index.html "mod bevy::gizmos::arrows")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Arrows

[circles](circles/index.html "mod bevy::gizmos::circles")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Circles

[config](config/index.html "mod bevy::gizmos::config")

A module for the [`GizmoConfig<T>`](../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource").

[cross](cross/index.html "mod bevy::gizmos::cross")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Crosses

[curves](curves/index.html "mod bevy::gizmos::curves")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Curves

[frustum](frustum/index.html "mod bevy::gizmos::frustum")

Module for the drawing of [`Frustum`](../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")s.

[gizmos](gizmos/index.html "mod bevy::gizmos::gizmos")

A module for the [`Gizmos`](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos") [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

[grid](grid/index.html "mod bevy::gizmos::grid")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Grids

[prelude](prelude/index.html "mod bevy::gizmos::prelude")

The gizmos prelude.

[primitives](primitives/index.html "mod bevy::gizmos::primitives")

A module for rendering each of the 2D and 3D [`bevy_math::primitives`](../math/primitives/index.html "mod bevy::math::primitives") with [`Gizmos`](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos").

[retained](retained/index.html "mod bevy::gizmos::retained")

This module is for ‘retained’ alternatives to the ‘immediate mode’ [`Gizmos`](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos") system parameter.

[rounded\_box](rounded_box/index.html "mod bevy::gizmos::rounded_box")

Additional [`GizmoBuffer`](gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer") Functions – Rounded cuboids and rectangles

[skinned\_mesh\_bounds](skinned_mesh_bounds/index.html "mod bevy::gizmos::skinned_mesh_bounds")`bevy_mesh`

A module adding debug visualization of [`DynamicSkinnedMeshBounds`](../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds").

[stroke\_text](stroke_text/index.html "mod bevy::gizmos::stroke_text")

This module draws text gizmos using a stroke font.

[transform\_gizmo](transform_gizmo/index.html "mod bevy::gizmos::transform_gizmo")

Interactive transform gizmo for translating, rotating, and scaling entities.

## Macros

[resolve\_gizmo\_camera](macro.resolve_gizmo_camera.html "macro bevy::gizmos::resolve_gizmo_camera")

Resolves which camera the gizmo should use.

## Structs

[GizmoAsset](struct.GizmoAsset.html "struct bevy::gizmos::GizmoAsset")

A collection of gizmos.

[GizmoHandles](struct.GizmoHandles.html "struct bevy::gizmos::GizmoHandles")

Holds handles to the line gizmos for each gizmo configuration group

[GizmoMeshSystems](struct.GizmoMeshSystems.html "struct bevy::gizmos::GizmoMeshSystems")

System set for updating the rendering meshes for drawing gizmos.

[GizmoPlugin](struct.GizmoPlugin.html "struct bevy::gizmos::GizmoPlugin")

A [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that provides an immediate mode drawing api for visual debugging.

## Traits

[AppGizmoBuilder](trait.AppGizmoBuilder.html "trait bevy::gizmos::AppGizmoBuilder")

A extension trait adding `App::init_gizmo_group` and `App::insert_gizmo_config`.

## Functions

[clear\_gizmo\_context](fn.clear_gizmo_context.html "fn bevy::gizmos::clear_gizmo_context")

Clear out the contextual gizmos.

[collect\_requested\_gizmos](fn.collect_requested_gizmos.html "fn bevy::gizmos::collect_requested_gizmos")

Collect the requested gizmos into a specific clear context.

[color\_from\_entity](fn.color_from_entity.html "fn bevy::gizmos::color_from_entity")

Generates a random, well-dispersed color seeded by the provided `Entity`.

[end\_gizmo\_context](fn.end_gizmo_context.html "fn bevy::gizmos::end_gizmo_context")

End this gizmo clearing context.

[propagate\_gizmos](fn.propagate_gizmos.html "fn bevy::gizmos::propagate_gizmos")

Propagate the contextual gizmo into the `Update` storage for rendering.

[start\_gizmo\_context](fn.start_gizmo_context.html "fn bevy::gizmos::start_gizmo_context")

Start a new gizmo clearing context.