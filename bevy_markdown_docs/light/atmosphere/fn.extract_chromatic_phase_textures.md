[bevy](../../index.html)::[light](../index.html)::[atmosphere](index.html)

# Function extract\_chromatic\_phase\_textures 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#548-552)

```rust
pub fn extract_chromatic_phase_textures(
    reader: MessageReader<'_, '_, AssetEvent<Image>>,
    images: Res<'_, Assets<Image>>,
    scattering_media: ResMut<'_, Assets<ScatteringMedium>>,
)
```

Resolves [`PhaseFunction::ChromaticTexture`](enum.PhaseFunction.html#variant.ChromaticTexture "variant bevy::light::atmosphere::PhaseFunction::ChromaticTexture") to [`PhaseFunction::ChromaticCurve`](enum.PhaseFunction.html#variant.ChromaticCurve "variant bevy::light::atmosphere::PhaseFunction::ChromaticCurve") when the image loads.