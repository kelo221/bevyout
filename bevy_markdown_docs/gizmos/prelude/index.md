[bevy](../../index.html)::[gizmos](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#49)

The gizmos prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AabbGizmoConfigGroup](struct.AabbGizmoConfigGroup.html "struct bevy::gizmos::prelude::AabbGizmoConfigGroup")

The [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of [`Aabb`](../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") components on entities

[DefaultGizmoConfigGroup](struct.DefaultGizmoConfigGroup.html "struct bevy::gizmos::prelude::DefaultGizmoConfigGroup")

The default gizmo config group.

[FrustumGizmoConfigGroup](struct.FrustumGizmoConfigGroup.html "struct bevy::gizmos::prelude::FrustumGizmoConfigGroup")

The [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") components on entities

[Gizmo](struct.Gizmo.html "struct bevy::gizmos::prelude::Gizmo")

A component that draws the gizmos of a [`GizmoAsset`](../../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset").

[GizmoAsset](struct.GizmoAsset.html "struct bevy::gizmos::prelude::GizmoAsset")

A collection of gizmos.

[GizmoConfig](struct.GizmoConfig.html "struct bevy::gizmos::prelude::GizmoConfig")

A struct that stores configuration for gizmos.

[GizmoConfigStore](struct.GizmoConfigStore.html "struct bevy::gizmos::prelude::GizmoConfigStore")

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") storing [`GizmoConfig`](../../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") and [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") structs

[GizmoLineConfig](struct.GizmoLineConfig.html "struct bevy::gizmos::prelude::GizmoLineConfig")

A struct that stores configuration for gizmos.

[Gizmos](struct.Gizmos.html "struct bevy::gizmos::prelude::Gizmos")

A [`SystemParam`](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for drawing gizmos.

[ShowAabbGizmo](struct.ShowAabbGizmo.html "struct bevy::gizmos::prelude::ShowAabbGizmo")

Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`Aabb`](../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component.

[ShowFrustumGizmo](struct.ShowFrustumGizmo.html "struct bevy::gizmos::prelude::ShowFrustumGizmo")

Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") component.

[ShowSkinnedMeshBoundsGizmo](struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::gizmos::prelude::ShowSkinnedMeshBoundsGizmo")

Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`DynamicSkinnedMeshBounds`](../../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds") component.

[SkinnedMeshBoundsGizmoConfigGroup](struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::gizmos::prelude::SkinnedMeshBoundsGizmoConfigGroup")

The [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of entities with [`DynamicSkinnedMeshBounds`](../../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[TransformGizmoCamera](struct.TransformGizmoCamera.html "struct bevy::gizmos::prelude::TransformGizmoCamera")

Marker component for the camera the transform gizmo should use.

[TransformGizmoFocus](struct.TransformGizmoFocus.html "struct bevy::gizmos::prelude::TransformGizmoFocus")

Component that marks the entity the transform gizmo operates on.

[TransformGizmoPlugin](struct.TransformGizmoPlugin.html "struct bevy::gizmos::prelude::TransformGizmoPlugin")

Opt-in plugin that adds the interactive transform gizmo.

[TransformGizmoSettings](struct.TransformGizmoSettings.html "struct bevy::gizmos::prelude::TransformGizmoSettings")

Configuration and preferences for the transform gizmo.

[TransformGizmoState](struct.TransformGizmoState.html "struct bevy::gizmos::prelude::TransformGizmoState")

Runtime state of the transform gizmo (drag and hover).

[TransformGizmoSystems](struct.TransformGizmoSystems.html "struct bevy::gizmos::prelude::TransformGizmoSystems")

System set for the transform gizmo. All transform gizmo systems run in [`PostUpdate`](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate") within this set.

## Enums

[GizmoLineJoint](enum.GizmoLineJoint.html "enum bevy::gizmos::prelude::GizmoLineJoint")

An enum configuring how line joints will be drawn.

[GizmoLineStyle](enum.GizmoLineStyle.html "enum bevy::gizmos::prelude::GizmoLineStyle")

An enum used to configure the style of gizmo lines, similar to CSS line-style

[TransformGizmoAxis](enum.TransformGizmoAxis.html "enum bevy::gizmos::prelude::TransformGizmoAxis")

Which axis the user is interacting with.

[TransformGizmoMode](enum.TransformGizmoMode.html "enum bevy::gizmos::prelude::TransformGizmoMode")

Which manipulation mode the gizmo is in.

[TransformGizmoSpace](enum.TransformGizmoSpace.html "enum bevy::gizmos::prelude::TransformGizmoSpace")

Whether the gizmo transforms the object using world or local space axes.

## Traits

[AppGizmoBuilder](trait.AppGizmoBuilder.html "trait bevy::gizmos::prelude::AppGizmoBuilder")

A extension trait adding `App::init_gizmo_group` and `App::insert_gizmo_config`.

[GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::gizmos::prelude::GizmoConfigGroup")

A trait used to create gizmo configs groups.

[GizmoPrimitive2d](trait.GizmoPrimitive2d.html "trait bevy::gizmos::prelude::GizmoPrimitive2d")

A trait for rendering 2D geometric primitives (`P`) with [`GizmoBuffer`](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer").

[GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::gizmos::prelude::GizmoPrimitive3d")

A trait for rendering 3D geometric primitives (`P`) with [`GizmoBuffer`](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer").

## Functions

[gizmo](fn.gizmo.html "fn bevy::gizmos::prelude::gizmo")

A global gizmo context for use outside of bevy systems.

## Derive Macros

[GizmoConfigGroup](derive.GizmoConfigGroup.html "derive bevy::gizmos::prelude::GizmoConfigGroup")

Implements the [`GizmoConfigGroup`](../../prelude/derive.GizmoConfigGroup.html "derive bevy::prelude::GizmoConfigGroup") trait for a gizmo config group type.