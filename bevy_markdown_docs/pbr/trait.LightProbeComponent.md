[bevy](../index.html)::[pbr](index.html)

# Trait LightProbeComponent 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#302)

```rust
pub trait LightProbeComponent:
    Sized
    + Send
    + Sync
    + Component {
    type AssetId: Send + Sync + Clone + Eq + Hash;
    type ViewLightProbeInfo: Send + Sync + Default;
    type QueryData: ReadOnlyQueryData;

    // Required methods
    fn id(&self, image_assets: &RenderAssets<GpuImage>) -> Option<Self::AssetId>;
    fn intensity(&self) -> f32;
    fn flags(
        &self,
        query_components: &<Self::QueryData as QueryData>::Item<'_, '_>,
    ) -> RenderLightProbeFlags;
    fn create_render_view_light_probes(
        view_component: Option<&Self>,
        image_assets: &RenderAssets<GpuImage>,
    ) -> RenderViewLightProbes<Self>;

    // Provided methods
    fn get_world_from_light_matrix(
        &self,
        original_world_from_light: &Affine3A,
    ) -> Affine3A { ... }
    fn parallax_correction_bounds(
        &self,
        _query_components: &<Self::QueryData as QueryData>::Item<'_, '_>,
    ) -> Vec3 { ... }
}
```

A trait implemented by all components that represent light probes.

Currently, the two light probe types are [`EnvironmentMapLight`](../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight") and [`IrradianceVolume`](../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume"), for reflection probes and irradiance volumes respectively.

Most light probe systems are written to be generic over the type of light probe. This allows much of the code to be shared and enables easy addition of more light probe types (e.g. real-time reflection planes) in the future.

## Required Associated Types

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#309)

#### type [AssetId](#associatedtype.AssetId): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash")

Holds [`AssetId`](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")s of the texture or textures that this light probe references.

This can just be [`AssetId`](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId") if the light probe only references one texture. If it references multiple textures, it will be a structure containing those asset IDs.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#318)

#### type [ViewLightProbeInfo](#associatedtype.ViewLightProbeInfo): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default")

If the light probe can be attached to the view itself (as opposed to a cuboid region within the scene), this contains the information that will be passed to the GPU in order to render it. Otherwise, this will be `()`.

Currently, only reflection probes (i.e. [`EnvironmentMapLight`](../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")) can be attached directly to views.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#322)

#### type [QueryData](#associatedtype.QueryData): [ReadOnlyQueryData](../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")

Any additional query data needed to determine the [`RenderLightProbeFlags`](struct.RenderLightProbeFlags.html "struct bevy::pbr::RenderLightProbeFlags") for this light probe.

## Required Methods

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#326)

#### fn [id](#tymethod.id)(&self, image\_assets: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[AssetId](trait.LightProbeComponent.html#associatedtype.AssetId "type bevy::pbr::LightProbeComponent::AssetId")\>

Returns the asset ID or asset IDs of the texture or textures referenced by this light probe.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#332)

#### fn [intensity](#tymethod.intensity)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the intensity of this light probe.

This is a scaling factor that will be multiplied by the value or values sampled from the texture.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#336-339)

#### fn [flags](#tymethod.flags)( &self, query\_components: &<Self::[QueryData](trait.LightProbeComponent.html#associatedtype.QueryData "type bevy::pbr::LightProbeComponent::QueryData") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>, ) -> [RenderLightProbeFlags](struct.RenderLightProbeFlags.html "struct bevy::pbr::RenderLightProbeFlags")

Returns the appropriate value of [`RenderLightProbeFlags`](struct.RenderLightProbeFlags.html "struct bevy::pbr::RenderLightProbeFlags") for this component.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#345-348)

#### fn [create\_render\_view\_light\_probes](#tymethod.create_render_view_light_probes)( view\_component: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&Self>, image\_assets: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>, ) -> [RenderViewLightProbes](struct.RenderViewLightProbes.html "struct bevy::pbr::RenderViewLightProbes")<Self>

Creates an instance of [`RenderViewLightProbes`](struct.RenderViewLightProbes.html "struct bevy::pbr::RenderViewLightProbes") containing all the information needed to render this light probe.

This is called for every light probe in view every frame.

## Provided Methods

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#356)

#### fn [get\_world\_from\_light\_matrix](#method.get_world_from_light_matrix)( &self, original\_world\_from\_light: &[Affine3A](../math/struct.Affine3A.html "struct bevy::math::Affine3A"), ) -> [Affine3A](../math/struct.Affine3A.html "struct bevy::math::Affine3A")

Given the matrix value of the `GlobalTransform` of the light probe, returns the matrix that transforms world positions into light probe space.

The default implementation simply returns the matrix unchanged, but some light probes may want to perform other transforms.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#365-368)

#### fn [parallax\_correction\_bounds](#method.parallax_correction_bounds)( &self, \_query\_components: &<Self::[QueryData](trait.LightProbeComponent.html#associatedtype.QueryData "type bevy::pbr::LightProbeComponent::QueryData") as [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>, ) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the appropriate parallax correction bounds, as half extents in light probe space, for this component.

See the comments in [`bevy_light::ParallaxCorrection::Custom`](../light/enum.ParallaxCorrection.html#variant.Custom "variant bevy::light::ParallaxCorrection::Custom") for more details.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#243)

### impl [LightProbeComponent](trait.LightProbeComponent.html "trait bevy::pbr::LightProbeComponent") for [EnvironmentMapLight](../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#244)

#### type [AssetId](#associatedtype.AssetId) = [EnvironmentMapIds](environment_map/struct.EnvironmentMapIds.html "struct bevy::pbr::environment_map::EnvironmentMapIds")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#248)

#### type [ViewLightProbeInfo](#associatedtype.ViewLightProbeInfo) = [EnvironmentMapViewLightProbeInfo](environment_map/struct.EnvironmentMapViewLightProbeInfo.html "struct bevy::pbr::environment_map::EnvironmentMapViewLightProbeInfo")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/environment_map.rs.html#250)

#### type [QueryData](#associatedtype.QueryData) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ParallaxCorrection](../light/enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/irradiance_volume.rs.html#296)

### impl [LightProbeComponent](trait.LightProbeComponent.html "trait bevy::pbr::LightProbeComponent") for [IrradianceVolume](../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/irradiance_volume.rs.html#297)

#### type [AssetId](#associatedtype.AssetId) = [AssetId](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<[Image](../prelude/struct.Image.html "struct bevy::prelude::Image")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/irradiance_volume.rs.html#301)

#### type [ViewLightProbeInfo](#associatedtype.ViewLightProbeInfo) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/irradiance_volume.rs.html#303)

#### type [QueryData](#associatedtype.QueryData) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)