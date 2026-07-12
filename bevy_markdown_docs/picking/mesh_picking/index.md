[bevy](../../index.html)::[picking](../index.html)

# Module mesh\_picking 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#164)

Available on **crate feature `mesh_picking`** only.

A [mesh ray casting](ray_cast/index.html "mod bevy::picking::mesh_picking::ray_cast") backend for [`bevy_picking`](../index.html "mod bevy::picking").

By default, all meshes that have [`bevy_asset::RenderAssetUsages::MAIN_WORLD`](../../asset/struct.RenderAssetUsages.html#associatedconstant.MAIN_WORLD "associated constant bevy::asset::RenderAssetUsages::MAIN_WORLD") are pickable. Picking can be disabled for individual entities by adding [`Pickable::IGNORE`](../../prelude/struct.Pickable.html#associatedconstant.IGNORE "associated constant bevy::prelude::Pickable::IGNORE").

To make mesh picking entirely opt-in, set [`MeshPickingSettings::require_markers`](../../prelude/struct.MeshPickingSettings.html#structfield.require_markers "field bevy::prelude::MeshPickingSettings::require_markers") to `true` and add [`MeshPickingCamera`](../../prelude/struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera") and [`Pickable`](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable") components to the desired camera and target entities.

To manually perform mesh ray casts independent of picking, use the [`MeshRayCast`](../../prelude/struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast") system parameter.

### Implementation Notes

*   The `position` reported in `HitData` is in world space. The `normal` is a vector pointing away from the face, it is not guaranteed to be normalized for scaled meshes.

## Modules

[ray\_cast](ray_cast/index.html "mod bevy::picking::mesh_picking::ray_cast")

Ray casting for meshes.

## Structs

[MeshPickingCamera](struct.MeshPickingCamera.html "struct bevy::picking::mesh_picking::MeshPickingCamera")

An optional component that marks cameras that should be used in the [`MeshPickingPlugin`](../../prelude/struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin").

[MeshPickingPlugin](struct.MeshPickingPlugin.html "struct bevy::picking::mesh_picking::MeshPickingPlugin")

Adds the mesh picking backend to your app.

[MeshPickingSettings](struct.MeshPickingSettings.html "struct bevy::picking::mesh_picking::MeshPickingSettings")

Runtime settings for the [`MeshPickingPlugin`](../../prelude/struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin").

## Functions

[update\_hits](fn.update_hits.html "fn bevy::picking::mesh_picking::update_hits")

Casts rays into the scene using [`MeshPickingSettings`](../../prelude/struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings") and sends [`PointerHits`](../backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits") events.