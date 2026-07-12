[bevy](../../index.html)::[light](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#73)

The light prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AmbientLight](struct.AmbientLight.html "struct bevy::light::prelude::AmbientLight")

An ambient light, which lights the entire scene equally.

[DirectionalLight](struct.DirectionalLight.html "struct bevy::light::prelude::DirectionalLight")

A Directional light.

[EnvironmentMapLight](struct.EnvironmentMapLight.html "struct bevy::light::prelude::EnvironmentMapLight")

A pair of cubemap textures that represent the surroundings of a specific area in space.

[GeneratedEnvironmentMapLight](struct.GeneratedEnvironmentMapLight.html "struct bevy::light::prelude::GeneratedEnvironmentMapLight")

A generated environment map that is filtered at runtime.

[GlobalAmbientLight](struct.GlobalAmbientLight.html "struct bevy::light::prelude::GlobalAmbientLight")

The global ambient light, which lights the entire scene equally.

[LightGizmoConfigGroup](struct.LightGizmoConfigGroup.html "struct bevy::light::prelude::LightGizmoConfigGroup")

The [`GizmoConfigGroup`](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used to configure the visualization of lights.

[LightProbe](struct.LightProbe.html "struct bevy::light::prelude::LightProbe")

A marker component for a light probe, which is a cuboid region that provides global illumination to all fragments inside it.

[PointLight](struct.PointLight.html "struct bevy::light::prelude::PointLight")

A light that emits light in all directions from a central point.

[RectLight](struct.RectLight.html "struct bevy::light::prelude::RectLight")

A rectangular area light.

[ShowLightGizmo](struct.ShowLightGizmo.html "struct bevy::light::prelude::ShowLightGizmo")

Add this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") to an entity to draw any of its lights components ([`PointLight`](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight"), [`SpotLight`](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight"), [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") and [`RectLight`](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")).

[SpotLight](struct.SpotLight.html "struct bevy::light::prelude::SpotLight")

A light that emits light in a given direction from a central point.

## Enums

[LightGizmoColor](enum.LightGizmoColor.html "enum bevy::light::prelude::LightGizmoColor")

Configures how a color is attributed to a light gizmo.