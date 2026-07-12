[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Function check\_visibility\_ranges 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#230-238)

```rust
pub fn check_visibility_ranges(
    visible_entity_ranges: ResMut<'_, VisibleEntityRanges>,
    view_query: Query<'_, '_, (Entity, &GlobalTransform), Or<(With<Camera>, With<ShadowLodOrigin>)>>,
    par_local: Local<'_, Parallel<Vec<(Entity, u32)>>>,
    entity_query: Query<'_, '_, (Entity, &GlobalTransform, Option<&Aabb>, &VisibilityRange), Without<NoCpuCulling>>,
)
```

Checks all entities against all views in order to determine which entities with [`VisibilityRange`](struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")s are potentially visible.

This only checks distance from the camera and doesn’t frustum or occlusion cull.