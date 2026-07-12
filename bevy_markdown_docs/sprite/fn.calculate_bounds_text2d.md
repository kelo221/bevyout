[bevy](../index.html)::[sprite](index.html)

# Function calculate\_bounds\_text2d 

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#346-358)

```rust
pub fn calculate_bounds_text2d(
    commands: Commands<'_, '_>,
    text_to_update_aabb: Query<'_, '_, (Entity, &TextLayoutInfo, &Anchor, &TextBounds, Option<&mut Aabb>), (Changed<TextLayoutInfo>, Without<NoFrustumCulling>)>,
)
```

System calculating and inserting an [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component to entities with some [`TextLayoutInfo`](../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo") and [`Anchor`](struct.Anchor.html "struct bevy::sprite::Anchor") components, and without a [`NoFrustumCulling`](../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling") component.

Used in system set [`VisibilitySystems::CalculateBounds`](../camera/visibility/enum.VisibilitySystems.html#variant.CalculateBounds "variant bevy::camera::visibility::VisibilitySystems::CalculateBounds").