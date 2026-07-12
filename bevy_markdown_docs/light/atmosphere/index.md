[bevy](../../index.html)::[light](../index.html)

# Module atmosphere 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#43)

Provides types to specify atmosphere lighting, scattering terms, etc.

## Structs

[Atmosphere](struct.Atmosphere.html "struct bevy::light::atmosphere::Atmosphere")

Atmosphere for one planet. The entity’s [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is the planet center in world space.

[AtmosphereTemplate](struct.AtmosphereTemplate.html "struct bevy::light::atmosphere::AtmosphereTemplate")

[ScatteringMedium](struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium")

An asset that defines how a material scatters light.

[ScatteringTerm](struct.ScatteringTerm.html "struct bevy::light::atmosphere::ScatteringTerm")

An individual element of a [`ScatteringMedium`](struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium").

## Enums

[Falloff](enum.Falloff.html "enum bevy::light::atmosphere::Falloff")

Describes how the media in a [`ScatteringTerm`](struct.ScatteringTerm.html "struct bevy::light::atmosphere::ScatteringTerm") is distributed.

[PhaseFunction](enum.PhaseFunction.html "enum bevy::light::atmosphere::PhaseFunction")

Describes how a [`ScatteringTerm`](struct.ScatteringTerm.html "struct bevy::light::atmosphere::ScatteringTerm") scatters light in different directions.

## Functions

[extract\_chromatic\_phase\_textures](fn.extract_chromatic_phase_textures.html "fn bevy::light::atmosphere::extract_chromatic_phase_textures")

Resolves [`PhaseFunction::ChromaticTexture`](enum.PhaseFunction.html#variant.ChromaticTexture "variant bevy::light::atmosphere::PhaseFunction::ChromaticTexture") to [`PhaseFunction::ChromaticCurve`](enum.PhaseFunction.html#variant.ChromaticCurve "variant bevy::light::atmosphere::PhaseFunction::ChromaticCurve") when the image loads.