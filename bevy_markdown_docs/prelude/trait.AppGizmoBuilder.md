[bevy](../index.html)::[prelude](index.html)

# Trait AppGizmoBuilder 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#126)

```rust
pub trait AppGizmoBuilder {
    // Required methods
    fn init_gizmo_group<Config>(&mut self) -> &mut Self
       where Config: GizmoConfigGroup;
    fn insert_gizmo_config<Config>(
        &mut self,
        group: Config,
        config: GizmoConfig,
    ) -> &mut Self
       where Config: GizmoConfigGroup;
}
```

A extension trait adding `App::init_gizmo_group` and `App::insert_gizmo_config`.

## Required Methods

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#130)

#### fn [init\_gizmo\_group](#tymethod.init_gizmo_group)<Config>(&mut self) -> &mut Self

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"),

Registers [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") in the app enabling the use of [Gizmos<Config>](struct.Gizmos.html "struct bevy::prelude::Gizmos").

Configurations can be set using the [`GizmoConfigStore`](struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore") [`Resource`](trait.Resource.html "trait bevy::prelude::Resource").

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#135-139)

#### fn [insert\_gizmo\_config](#tymethod.insert_gizmo_config)<Config>( &mut self, group: Config, config: [GizmoConfig](struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig"), ) -> &mut Self

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"),

Insert a [`GizmoConfig`](struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") into a specific [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup").

This method should be preferred over [`AppGizmoBuilder::init_gizmo_group`](trait.AppGizmoBuilder.html#tymethod.init_gizmo_group "method bevy::prelude::AppGizmoBuilder::init_gizmo_group") if and only if you need to configure fields upon initialization.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#142)

### impl [AppGizmoBuilder](trait.AppGizmoBuilder.html "trait bevy::prelude::AppGizmoBuilder") for [App](struct.App.html "struct bevy::prelude::App")