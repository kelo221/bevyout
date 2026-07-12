[bevy](../../index.html)::[camera](../index.html)

# Module visibility 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/lib.rs.html#7)

Components that control the visibility of entities.

### What is the difference between visibility components

There are three components that indicate various kinds of visibility modes of an entity:

*   [`Visibility`](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility")
*   [`InheritedVisibility`](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")
*   [`ViewVisibility`](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

[`Visibility`](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility") is the user-defined visibility. It is the only component that users should typically add to an entity[1](#fn1), the other two are then added automatically.

[`InheritedVisibility`](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility") is computed by propagation through the entity hierarchy. Entities with [`Visibility::Inherited`](../../prelude/enum.Visibility.html#variant.Inherited "variant bevy::prelude::Visibility::Inherited") copy the visibility of their parent entities. If they have no [`ChildOf`](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf") component, they are visible. The propagation is done in `visibility_propagate_system`, which runs in the [`PostUpdate`](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") schedule.

[`ViewVisibility`](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility") indicates whether the entity should be extracted for rendering. This component is recomputed in every frame in the [`PostUpdate`](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") schedule.

* * *

1.  If at all – most components that go together with [`Visibility`](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility") already [require](../../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") it, so users only need to explicitly add it if they wish to override the default value of [`Visibility::Inherited`](../../prelude/enum.Visibility.html#variant.Inherited "variant bevy::prelude::Visibility::Inherited"). [↩](#fnref1)
    

## Structs

[CascadesVisibleEntities](struct.CascadesVisibleEntities.html "struct bevy::camera::visibility::CascadesVisibleEntities")

[CubemapVisibleEntities](struct.CubemapVisibleEntities.html "struct bevy::camera::visibility::CubemapVisibleEntities")

[DynamicSkinnedMeshBounds](struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

Use this component to enable dynamic skinned mesh bounds. The [`Aabb`](../primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component of the skinned mesh will be automatically updated each frame based on the current joint transforms.

[InheritedVisibility](struct.InheritedVisibility.html "struct bevy::camera::visibility::InheritedVisibility")

Whether or not an entity is visible in the hierarchy.

[NoAutoAabb](struct.NoAutoAabb.html "struct bevy::camera::visibility::NoAutoAabb")

Add this component to an entity to prevent its `AABB` from being automatically recomputed.

[NoCpuCulling](struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling")

Use this component to opt-out of the built-in CPU frustum culling, see [`Frustum`](../primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum"). This can be attached to a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") or to individual entities.

[NoFrustumCulling](struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling")

Use this component to opt-out of built-in frustum culling for entities, see [`Frustum`](../primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum").

[RenderLayers](struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers")

Defines which rendering layers an entity belongs to.

[ViewVisibility](struct.ViewVisibility.html "struct bevy::camera::visibility::ViewVisibility")

Algorithmically computed indication of whether an entity is visible and should be extracted for rendering.

[VisibilityClass](struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass")

A bucket into which we group entities for the purposes of visibility.

[VisibilityPlugin](struct.VisibilityPlugin.html "struct bevy::camera::visibility::VisibilityPlugin")

[VisibilityRange](struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")

Specifies the range of distances that this entity must be from the camera in order to be rendered.

[VisibilityRangePlugin](struct.VisibilityRangePlugin.html "struct bevy::camera::visibility::VisibilityRangePlugin")

A plugin that enables [`VisibilityRange`](struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")s, which allow entities to be hidden or shown based on distance to the camera.

[VisibleEntities](struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities")

Collection of entities visible from the current view.

[VisibleEntityRanges](struct.VisibleEntityRanges.html "struct bevy::camera::visibility::VisibleEntityRanges")

Stores which entities are in within the [`VisibilityRange`](struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")s of views.

[VisibleMeshEntities](struct.VisibleMeshEntities.html "struct bevy::camera::visibility::VisibleMeshEntities")

Collection of mesh entities visible for 3D lighting.

## Enums

[Visibility](enum.Visibility.html "enum bevy::camera::visibility::Visibility")

User indication of whether an entity is visible. Propagates down the entity hierarchy.

[VisibilitySystems](enum.VisibilitySystems.html "enum bevy::camera::visibility::VisibilitySystems")

## Constants

[DEFAULT\_LAYERS](constant.DEFAULT_LAYERS.html "constant bevy::camera::visibility::DEFAULT_LAYERS")

## Traits

[SetViewVisibility](trait.SetViewVisibility.html "trait bevy::camera::visibility::SetViewVisibility")

## Functions

[add\_visibility\_class](fn.add_visibility_class.html "fn bevy::camera::visibility::add_visibility_class")

A generic component add hook that automatically adds the appropriate [`VisibilityClass`](struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass") to an entity.

[calculate\_bounds](fn.calculate_bounds.html "fn bevy::camera::visibility::calculate_bounds")

Computes and adds an [`Aabb`](../primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component to entities with a [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") component and without a [`NoFrustumCulling`](struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling") component.

[check\_visibility\_cpu\_culling](fn.check_visibility_cpu_culling.html "fn bevy::camera::visibility::check_visibility_cpu_culling")

System updating the visibility of entities, other than those that have opted out of CPU culling, each frame.

[check\_visibility\_gpu\_culling](fn.check_visibility_gpu_culling.html "fn bevy::camera::visibility::check_visibility_gpu_culling")

Updates the visibility of entities marked with [`NoCpuCulling`](struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling").

[check\_visibility\_ranges](fn.check_visibility_ranges.html "fn bevy::camera::visibility::check_visibility_ranges")

Checks all entities against all views in order to determine which entities with [`VisibilityRange`](struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")s are potentially visible.

[update\_frusta](fn.update_frusta.html "fn bevy::camera::visibility::update_frusta")

Updates [`Frustum`](../primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum").

## Type Aliases

[Layer](type.Layer.html "type bevy::camera::visibility::Layer")

An identifier for a rendering layer.