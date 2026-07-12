[bevy](../../index.html)::[light](../index.html)

# Module cascade 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#47)

Provides shadow cascade configuration and construction helpers.

## Structs

[Cascade](struct.Cascade.html "struct bevy::light::cascade::Cascade")

A single cascade of a view’s shadow map cascade. Several of these are used to cover most of the view to ensure most geometry gets shadows, with some overlap for blending at cascade transitions. Farther away cascades are larger and have a lower effective shadowmap texel per world unit resolution. All cascades have the same pixel dimensions however.

[CascadeShadowConfig](struct.CascadeShadowConfig.html "struct bevy::light::cascade::CascadeShadowConfig")

Controls how cascaded shadow mapping works. Prefer using [`CascadeShadowConfigBuilder`](../struct.CascadeShadowConfigBuilder.html "struct bevy::light::CascadeShadowConfigBuilder") to construct an instance.

[CascadeShadowConfigBuilder](struct.CascadeShadowConfigBuilder.html "struct bevy::light::cascade::CascadeShadowConfigBuilder")

Builder for [`CascadeShadowConfig`](../struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig").

[Cascades](struct.Cascades.html "struct bevy::light::cascade::Cascades")

A [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")’s per-view list of [`Cascade`](struct.Cascade.html "struct bevy::light::cascade::Cascade")s.

## Functions

[build\_directional\_light\_cascades](fn.build_directional_light_cascades.html "fn bevy::light::cascade::build_directional_light_cascades")

Sets up [`Cascades`](../struct.Cascades.html "struct bevy::light::Cascades") for all shadow mapped [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s.