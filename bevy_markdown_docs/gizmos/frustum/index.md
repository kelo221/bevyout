[bevy](../../index.html)::[gizmos](../index.html)

# Module frustum 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#32)

Module for the drawing of [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")s.

Camera entities are spawned with a [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") component, which describes a camera’s field of vision. With this module, a camera’s frustum can be drawn with gizmos. This is useful for debugging what a camera can see and what entities in the scene will be subject to the camera’s frustum culling, especially when combined with drawing [`Aabb`](../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") [`gizmos`](../aabb/index.html "mod bevy::gizmos::aabb").

There are two ways to enable gizmo drawing of a [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum"):

1.  The [`ShowFrustumGizmo`](../../prelude/struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo") component can be added to individual camera entities.

```rust
fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, ShowFrustumGizmo::default()));
}
```

2.  Setting the [`FrustumGizmoConfigGroup`](../../prelude/struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup") configuration’s `draw_all` field to `true` will draw every frustum. Note that this will include drawing `bevy_light` `SpotLight` [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")s.

```rust
fn turn_on_frustum_gizmos(mut config: ResMut<GizmoConfigStore>) {
   config.config_mut::<FrustumGizmoConfigGroup>().1.draw_all = true;
}
```

## Structs

[FrustumGizmoConfigGroup](struct.FrustumGizmoConfigGroup.html "struct bevy::gizmos::frustum::FrustumGizmoConfigGroup")

The [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") components on entities

[FrustumGizmoPlugin](struct.FrustumGizmoPlugin.html "struct bevy::gizmos::frustum::FrustumGizmoPlugin")

A [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that provides visualization of [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")s for debugging.

[FrustumGizmoSystems](struct.FrustumGizmoSystems.html "struct bevy::gizmos::frustum::FrustumGizmoSystems")

Frustum Gizmo system set. This exists in [`PostUpdate`](../../prelude/struct.PostUpdate.html "struct bevy::prelude::PostUpdate").

[ShowFrustumGizmo](struct.ShowFrustumGizmo.html "struct bevy::gizmos::frustum::ShowFrustumGizmo")

Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`Frustum`](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") component.