[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Function check\_visibility\_cpu\_culling 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#748-774)

```rust
pub fn check_visibility_cpu_culling(
    thread_queues: Local<'_, Parallel<IndexMap<TypeId, Vec<Entity>, NoOpHash>>>,
    view_query: Query<'_, '_, (Entity, &mut VisibleEntities, &Frustum, Option<&RenderLayers>, &Camera, Has<NoCpuCulling>)>,
    visible_aabb_query: Query<'_, '_, (Entity, &InheritedVisibility, &mut ViewVisibility, Option<&VisibilityClass>, Option<&RenderLayers>, Option<&Aabb>, Option<&Sphere>, &GlobalTransform, Has<NoFrustumCulling>, Has<VisibilityRange>), Without<NoCpuCulling>>,
    visible_entity_ranges: Option<Res<'_, VisibleEntityRanges>>,
)
```

System updating the visibility of entities, other than those that have opted out of CPU culling, each frame.

The system is part of the [`VisibilitySystems::CheckVisibility`](enum.VisibilitySystems.html#variant.CheckVisibility "variant bevy::camera::visibility::VisibilitySystems::CheckVisibility") set. Each frame, it updates the [`ViewVisibility`](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility") of all entities, and for each view also compute the [`VisibleEntities`](struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities") for that view.

To ensure that an entity is checked for visibility, make sure that it has a [`VisibilityClass`](struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass") component and that that component is nonempty.