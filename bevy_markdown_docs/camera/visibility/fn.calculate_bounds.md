[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Function calculate\_bounds 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#557-576)

```rust
pub fn calculate_bounds(
    commands: Commands<'_, '_>,
    meshes: Res<'_, Assets<Mesh>>,
    new_aabb: Query<'_, '_, (Entity, &Mesh3d), (Without<Aabb>, Without<NoFrustumCulling>, Without<NoAutoAabb>)>,
    update_aabb: Query<'_, '_, (&Mesh3d, &mut Aabb), (Or<(AssetChanged<Mesh3d>, Changed<Mesh3d>)>, Without<NoFrustumCulling>, Without<NoAutoAabb>)>,
)
```

Computes and adds an [`Aabb`](../primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component to entities with a [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") component and without a [`NoFrustumCulling`](struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling") component.

This system is used in system set [`VisibilitySystems::CalculateBounds`](enum.VisibilitySystems.html#variant.CalculateBounds "variant bevy::camera::visibility::VisibilitySystems::CalculateBounds").