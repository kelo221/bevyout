[bevy](../index.html)

# Crate light 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#1-787)

Provides component types for lighting a bevy scene. This includes the usual directional, point, and spot lights, as well as light probes, atmosphere, other volumetrics, and shadow configuration.

## Modules

[atmosphere](atmosphere/index.html "mod bevy::light::atmosphere")

Provides types to specify atmosphere lighting, scattering terms, etc.

[cascade](cascade/index.html "mod bevy::light::cascade")

Provides shadow cascade configuration and construction helpers.

[cluster](cluster/index.html "mod bevy::light::cluster")

Spatial clustering of objects to accelerate rendering performance.

[gizmos](gizmos/index.html "mod bevy::light::gizmos")`bevy_gizmos`

Provides gizmo drawing for visualizing light positions. A module adding debug visualization of [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s, [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s, [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s and [`RectLight`](../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")s.

[prelude](prelude/index.html "mod bevy::light::prelude")

The light prelude.

## Structs

[AmbientLight](struct.AmbientLight.html "struct bevy::light::AmbientLight")

An ambient light, which lights the entire scene equally.

[Atmosphere](struct.Atmosphere.html "struct bevy::light::Atmosphere")

Atmosphere for one planet. The entity’s [`GlobalTransform`](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is the planet center in world space.

[AtmosphereEnvironmentMapLight](struct.AtmosphereEnvironmentMapLight.html "struct bevy::light::AtmosphereEnvironmentMapLight")

Lets the atmosphere contribute environment lighting (reflections and ambient diffuse) to your scene.

[CascadeShadowConfig](struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig")

Controls how cascaded shadow mapping works. Prefer using [`CascadeShadowConfigBuilder`](struct.CascadeShadowConfigBuilder.html "struct bevy::light::CascadeShadowConfigBuilder") to construct an instance.

[CascadeShadowConfigBuilder](struct.CascadeShadowConfigBuilder.html "struct bevy::light::CascadeShadowConfigBuilder")

Builder for [`CascadeShadowConfig`](struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig").

[Cascades](struct.Cascades.html "struct bevy::light::Cascades")

A [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")’s per-view list of [`Cascade`](cascade/struct.Cascade.html "struct bevy::light::cascade::Cascade")s.

[ClusteredDecal](struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

An object that projects a decal onto surfaces within its bounds.

[DirectionalLight](struct.DirectionalLight.html "struct bevy::light::DirectionalLight")

A Directional light.

[DirectionalLightShadowMap](struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

Controls the resolution of [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") and [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight") shadow maps.

[DirectionalLightTexture](struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

Add to a [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") to add a light texture effect. A texture mask is applied to the light source to modulate its intensity,  
simulating patterns like window shadows, gobo/cookie effects, or soft falloffs.

[EnvironmentMapLight](struct.EnvironmentMapLight.html "struct bevy::light::EnvironmentMapLight")

A pair of cubemap textures that represent the surroundings of a specific area in space.

[FogVolume](struct.FogVolume.html "struct bevy::light::FogVolume")

A unit cube of fog at the origin. Can be positioned and scaled with a [`Transform`](../prelude/struct.Transform.html "struct bevy::prelude::Transform"). Only visible by cameras with a [`VolumetricFog`](struct.VolumetricFog.html "struct bevy::light::VolumetricFog") component when lit by a directional light with [`VolumetricLight`](struct.VolumetricLight.html "struct bevy::light::VolumetricLight").

[GeneratedEnvironmentMapLight](struct.GeneratedEnvironmentMapLight.html "struct bevy::light::GeneratedEnvironmentMapLight")

A generated environment map that is filtered at runtime.

[GlobalAmbientLight](struct.GlobalAmbientLight.html "struct bevy::light::GlobalAmbientLight")

The global ambient light, which lights the entire scene equally.

[IrradianceVolume](struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

The component that defines an irradiance volume.

[LightPlugin](struct.LightPlugin.html "struct bevy::light::LightPlugin")

Sets up all the light visibility and clustering infrastructure needed for rendering lights.

[LightProbe](struct.LightProbe.html "struct bevy::light::LightProbe")

A marker component for a light probe, which is a cuboid region that provides global illumination to all fragments inside it.

[NotShadowCaster](struct.NotShadowCaster.html "struct bevy::light::NotShadowCaster")

Add this component to make a [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") not cast shadows.

[NotShadowReceiver](struct.NotShadowReceiver.html "struct bevy::light::NotShadowReceiver")

Add this component to make a [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") not receive shadows.

[PointLight](struct.PointLight.html "struct bevy::light::PointLight")

A light that emits light in all directions from a central point.

[PointLightShadowMap](struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

Controls the resolution of [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight") shadow maps.

[PointLightTexture](struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

Add to a [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight") to add a light texture effect. A texture mask is applied to the light source to modulate its intensity,  
simulating patterns like window shadows, gobo/cookie effects, or soft falloffs.

[RectLight](struct.RectLight.html "struct bevy::light::RectLight")

A rectangular area light.

[Skybox](struct.Skybox.html "struct bevy::light::Skybox")

Adds a skybox to a 3D camera, based on a cubemap texture.

[SpotLight](struct.SpotLight.html "struct bevy::light::SpotLight")

A light that emits light in a given direction from a central point.

[SpotLightTexture](struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

Add to a [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight") to add a light texture effect. A texture mask is applied to the light source to modulate its intensity,  
simulating patterns like window shadows, gobo/cookie effects, or soft falloffs.

[SunDisk](struct.SunDisk.html "struct bevy::light::SunDisk")

Add to a [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") to control rendering of the visible solar disk in the sky. Affects only the disk’s appearance, not the light’s illuminance or shadows. Requires a `bevy::pbr::Atmosphere` component on a [`Camera3d`](../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") to have any effect.

[TransmittedShadowReceiver](struct.TransmittedShadowReceiver.html "struct bevy::light::TransmittedShadowReceiver")

Add this component to make a [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") using a PBR material with `StandardMaterial::diffuse_transmission > 0.0` receive shadows on its diffuse transmission lobe. (i.e. its “backside”)

[VolumetricFog](struct.VolumetricFog.html "struct bevy::light::VolumetricFog")

When placed on a [`bevy_camera::Camera3d`](../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d"), enables volumetric fog and volumetric lighting, also known as light shafts or god rays.

[VolumetricLight](struct.VolumetricLight.html "struct bevy::light::VolumetricLight")

Add this component to a [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") with a shadow map (`shadow_maps_enabled: true`) to make volumetric fog interact with it.

## Enums

[ParallaxCorrection](enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")

Add this component to a reflection probe to customize _parallax correction_.

[ShadowFilteringMethod](enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

Add this component to a [`Camera3d`](../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") to control how to anti-alias shadow edges.

[SimulationLightSystems](enum.SimulationLightSystems.html "enum bevy::light::SimulationLightSystems")

System sets used to run light-related systems.

## Functions

[automatically\_add\_parallax\_correction\_components](fn.automatically_add_parallax_correction_components.html "fn bevy::light::automatically_add_parallax_correction_components")

A system that automatically adds a [`ParallaxCorrection::Auto`](enum.ParallaxCorrection.html#variant.Auto "variant bevy::light::ParallaxCorrection::Auto") component to any reflection probe that doesn’t already have a [`ParallaxCorrection`](enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection") component.

[check\_dir\_light\_mesh\_visibility](fn.check_dir_light_mesh_visibility.html "fn bevy::light::check_dir_light_mesh_visibility")

Updates the visibility for [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s so that shadow map rendering can work.

[check\_point\_light\_mesh\_visibility](fn.check_point_light_mesh_visibility.html "fn bevy::light::check_point_light_mesh_visibility")

Updates the visibility for [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s and [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s so that shadow map rendering can work.

[get\_shadow\_lod\_origin](fn.get_shadow_lod_origin.html "fn bevy::light::get_shadow_lod_origin")

Determines the LOD origin for spot and point light shadow maps.

[orthonormalize](fn.orthonormalize.html "fn bevy::light::orthonormalize")

Constructs a right-handed orthonormal basis from a given unit Z vector.

[spot\_light\_clip\_from\_view](fn.spot_light_clip_from_view.html "fn bevy::light::spot_light_clip_from_view")

Creates the projection matrix that transforms the light’s view space into the light’s clip space.

[spot\_light\_world\_from\_view](fn.spot_light_world_from_view.html "fn bevy::light::spot_light_world_from_view")

Constructs a right-handed orthonormal basis with translation, using only the forward direction and translation of a given [`GlobalTransform`](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

[update\_directional\_light\_frusta](fn.update_directional_light_frusta.html "fn bevy::light::update_directional_light_frusta")

Updates the frusta for all visible shadow mapped [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s.

[update\_point\_light\_frusta](fn.update_point_light_frusta.html "fn bevy::light::update_point_light_frusta")

Updates the frusta for all visible shadow mapped [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s.

[update\_spot\_light\_frusta](fn.update_spot_light_frusta.html "fn bevy::light::update_spot_light_frusta")

Updates the frusta for all visible shadow mapped [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s.

## Type Aliases

[WithLight](type.WithLight.html "type bevy::light::WithLight")

A convenient alias for `Or<(With<PointLight>, With<SpotLight>, With<DirectionalLight>, With<RectLight>)>`, for use with [`bevy_camera::visibility::VisibleEntities`](../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities").