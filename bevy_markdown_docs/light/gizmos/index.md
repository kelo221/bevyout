[bevy](../../index.html)::[light](../index.html)

# Module gizmos 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#68)

Available on **crate feature `bevy_gizmos`** only.

Provides gizmo drawing for visualizing light positions. A module adding debug visualization of [`PointLight`](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s, [`SpotLight`](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s, [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s and [`RectLight`](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")s.

## Structs

[LightGizmoConfigGroup](struct.LightGizmoConfigGroup.html "struct bevy::light::gizmos::LightGizmoConfigGroup")

The [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used to configure the visualization of lights.

[LightGizmoPlugin](struct.LightGizmoPlugin.html "struct bevy::light::gizmos::LightGizmoPlugin")

A [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that provides visualization of [`PointLight`](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s, [`SpotLight`](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s, [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s and [`RectLight`](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")s for debugging.

[ShowLightGizmo](struct.ShowLightGizmo.html "struct bevy::light::gizmos::ShowLightGizmo")

Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an entity to draw any of its lights components ([`PointLight`](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight"), [`SpotLight`](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight"), [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") and [`RectLight`](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")).

## Enums

[LightGizmoColor](enum.LightGizmoColor.html "enum bevy::light::gizmos::LightGizmoColor")

Configures how a color is attributed to a light gizmo.