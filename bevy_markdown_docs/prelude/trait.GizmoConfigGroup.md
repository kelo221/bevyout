[bevy](../index.html)::[prelude](index.html)

# Trait GizmoConfigGroup 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#81)

```rust
pub trait GizmoConfigGroup:
    Reflect
    + TypePath
    + Default { }
```

A trait used to create gizmo configs groups.

Here you can store additional configuration for you gizmo group not covered by [`GizmoConfig`](struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

Make sure to derive [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") and register in the app using `app.init_gizmo_group::<T>()`

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#43)

### impl [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") for [AabbGizmoConfigGroup](struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

where [AabbGizmoConfigGroup](struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup"): [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#84)

### impl [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") for [DefaultGizmoConfigGroup](struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

where [DefaultGizmoConfigGroup](struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup"): [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#90)

### impl [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") for [ErasedGizmoConfigGroup](../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

where [ErasedGizmoConfigGroup](../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup"): [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#78)

### impl [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") for [FrustumGizmoConfigGroup](struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

where [FrustumGizmoConfigGroup](struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup"): [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#166)

### impl [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") for [LightGizmoConfigGroup](struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

where [LightGizmoConfigGroup](struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup"): [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#52)

### impl [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") for [SkinnedMeshBoundsGizmoConfigGroup](struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

where [SkinnedMeshBoundsGizmoConfigGroup](struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup"): [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),