[bevy](../index.html)::[sprite](index.html)

# Function calculate\_bounds\_2d 

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/lib.rs.html#118-159)

```rust
pub fn calculate_bounds_2d(
    commands: Commands<'_, '_>,
    meshes: Res<'_, Assets<Mesh>>,
    images: Res<'_, Assets<Image>>,
    atlases: Res<'_, Assets<TextureAtlasLayout>>,
    new_mesh_aabb: Query<'_, '_, (Entity, &Mesh2d), (Without<Aabb>, Without<NoFrustumCulling>, Without<NoAutoAabb>, Without<SpriteMesh>)>,
    update_mesh_aabb: Query<'_, '_, (&Mesh2d, &mut Aabb), (Or<(AssetChanged<Mesh2d>, Changed<Mesh2d>)>, Without<NoFrustumCulling>, Without<NoAutoAabb>, Without<SpriteMesh>, Without<Sprite>)>,
    new_sprite_aabb: Query<'_, '_, (Entity, &Sprite, &Anchor), (Without<Aabb>, Without<NoFrustumCulling>, Without<NoAutoAabb>)>,
    update_sprite_aabb: Query<'_, '_, (&Sprite, &mut Aabb, &Anchor), (Or<(Changed<Sprite>, Changed<Anchor>)>, Without<NoFrustumCulling>, Without<NoAutoAabb>, Without<Mesh2d>)>,
)
```

System calculating and inserting an [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component to entities with either:

*   a `Mesh2d` component,
*   a `Sprite` and `Handle<Image>` components, and without a [`NoFrustumCulling`](../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling") component.

Used in system set [`VisibilitySystems::CalculateBounds`](../camera/visibility/enum.VisibilitySystems.html#variant.CalculateBounds "variant bevy::camera::visibility::VisibilitySystems::CalculateBounds").