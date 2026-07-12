[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Function check\_visibility\_gpu\_culling 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#884-892)

```rust
pub fn check_visibility_gpu_culling(
    query: Query<'_, '_, (&mut ViewVisibility, &InheritedVisibility), (With<NoCpuCulling>, Or<(Changed<InheritedVisibility>, Added<NoCpuCulling>)>)>,
)
```

Updates the visibility of entities marked with [`NoCpuCulling`](struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling").

In this case, the [`ViewVisibility`](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility") of each such mesh simply becomes equal to its [`InheritedVisibility`](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility"), as the CPU has been instructed to perform no other checks. For performance, we avoid examining any entity that hasn’t changed its inherited visibility.