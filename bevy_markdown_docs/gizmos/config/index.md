[bevy](../../index.html)::[gizmos](../index.html)

# Module config 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#29)

A module for the [`GizmoConfig<T>`](../../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

## Structs

[DefaultGizmoConfigGroup](struct.DefaultGizmoConfigGroup.html "struct bevy::gizmos::config::DefaultGizmoConfigGroup")

The default gizmo config group.

[ErasedGizmoConfigGroup](struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

Used when the gizmo config group needs to be type-erased. Also used for retained gizmos, which can’t have a gizmo config group.

[GizmoConfig](struct.GizmoConfig.html "struct bevy::gizmos::config::GizmoConfig")

A struct that stores configuration for gizmos.

[GizmoConfigStore](struct.GizmoConfigStore.html "struct bevy::gizmos::config::GizmoConfigStore")

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") storing [`GizmoConfig`](../../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") and [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") structs

[GizmoLineConfig](struct.GizmoLineConfig.html "struct bevy::gizmos::config::GizmoLineConfig")

A struct that stores configuration for gizmos.

[GizmoMeshConfig](struct.GizmoMeshConfig.html "struct bevy::gizmos::config::GizmoMeshConfig")

Configuration for gizmo meshes.

[GizmoMeshConfigTemplate](struct.GizmoMeshConfigTemplate.html "struct bevy::gizmos::config::GizmoMeshConfigTemplate")

## Enums

[GizmoLineJoint](enum.GizmoLineJoint.html "enum bevy::gizmos::config::GizmoLineJoint")

An enum configuring how line joints will be drawn.

[GizmoLineStyle](enum.GizmoLineStyle.html "enum bevy::gizmos::config::GizmoLineStyle")

An enum used to configure the style of gizmo lines, similar to CSS line-style

## Traits

[GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::gizmos::config::GizmoConfigGroup")

A trait used to create gizmo configs groups.

## Derive Macros

[GizmoConfigGroup](derive.GizmoConfigGroup.html "derive bevy::gizmos::config::GizmoConfigGroup")

Implements the [`GizmoConfigGroup`](../../prelude/derive.GizmoConfigGroup.html "derive bevy::prelude::GizmoConfigGroup") trait for a gizmo config group type.