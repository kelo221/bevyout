[bevy](../index.html)::[pbr](index.html)

# Function extract\_atmosphere 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#210-214)

```rust
pub fn extract_atmosphere(
    commands: Commands<'_, '_>,
    atmosphere_entities: Extract<'_, '_, Query<'_, '_, (Entity, &Atmosphere, &GlobalTransform)>>,
    cameras: Extract<'_, '_, Query<'_, '_, (RenderEntity, &AtmosphereSettings, &GlobalTransform), With<Camera3d>>>,
)
```

For each camera with [`AtmosphereSettings`](struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings"), picks the nearest [`Atmosphere`](../light/struct.Atmosphere.html "struct bevy::light::Atmosphere") by world-space distance to its origin, copies it as [`ExtractedAtmosphere`](struct.ExtractedAtmosphere.html "struct bevy::pbr::ExtractedAtmosphere"), and builds [`GpuAtmosphereSettings`](struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings").