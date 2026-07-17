# DynamicLighting source-to-port matrix

Frozen baselines and licensing are recorded in `NOTICE.md`. A row is complete
only when its Rust/WGSL destination and the listed Unity-generated fixture are
covered by an automated test.

## Temporal effects

| Original symbol | Upstream source | Rust destination | GPU destination | Golden fixture |
| --- | --- | --- | --- | --- |
| `Steady = 0` | `DynamicLightManager.UpdateLightEffects` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Pulse = 1` | `LightEffectPulse.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Random = 2` | `DynamicLightManager.UpdateLightEffects` | `core/runtime.rs` | uploaded `intensity` | `unity_multilight_random_v1.json` |
| `Strobe = 3` | `DynamicLightManager.UpdateLightEffects` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Flicker = 4` | `DynamicLightManager.UpdateLightEffects` | `core/runtime.rs` | uploaded `intensity` | `unity_multilight_random_v1.json` |
| `FluorescentStarter = 5` | `LightEffectFluorescentStarter.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `FluorescentClicker = 6` | `LightEffectFluorescentClicker.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `FluorescentRandom = 7` | `LightEffectFluorescentRandom.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_multilight_random_v1.json` |
| `Candle = 8` | `LightEffectCandle.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Pulsar = 9` | `LightEffectPulsar.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Fire = 10` | `LightEffectFire.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Generator = 11` | `LightEffectGenerator.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Lightning = 12` | `LightEffectLightning.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Cloudy = 13` | `LightEffectCloudy.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |
| `Overcast = 14` | `LightEffectOvercast.cs` | `core/runtime.rs` | uploaded `intensity` | `unity_effects_v1.json` |

## Spatial types

| Original symbol | Upstream function | Rust/GPU type | WGSL destination | Golden fixture |
| --- | --- | --- | --- | --- |
| `Point = 0` | radial default | `DynamicLightType::Point` | `calculate_point` | `unity_spatial_v1.json` |
| `Spot = 1` | `calculate_spotlight` | `DynamicLightType::Spot` | `calculate_spotlight` | `unity_spatial_v1.json` |
| `Discoball = 2` | `calculate_discoball` | `DynamicLightType::Discoball` | `calculate_discoball` | `unity_spatial_v1.json` |
| `Wave = 3` | `calculate_wave` | `DynamicLightType::Wave` | `calculate_wave` | `unity_spatial_v1.json` |
| `Interference = 4` | `calculate_interference` | `DynamicLightType::Interference` | `calculate_interference` | `unity_spatial_v1.json` |
| `Rotor = 5` | `calculate_rotor` | `DynamicLightType::Rotor` | `calculate_rotor` | `unity_spatial_v1.json` |
| `Shock = 6` | `calculate_shock` | `DynamicLightType::Shock` | `calculate_shock` | `unity_spatial_v1.json` |
| `Disco = 7` | `calculate_disco` | `DynamicLightType::Disco` | `calculate_disco` | `unity_spatial_v1.json` |

## Authoring fields and GPU ABI

| Original field/default | Source | Rust destination | GPU mapping/test |
| --- | --- | --- | --- |
| color `white`, intensity `2`, radius `4` | `DynamicLight.cs` | `DynamicLightConfig` | `color`, `intensity`, `radius_squared` |
| falloff `0` | `DynamicLight.cs` / `calculate_attenuation` | `DynamicLightConfig::falloff` | `radius * falloff^2` |
| spot inner `26 deg`, outer `30 deg` | `DynamicLight.cs` | spatial parameters | `cos(inner)`, `cos(outer)` |
| wave speed/frequency `1`, offset `0` | `DynamicLight.cs` | spatial parameters | `gp_float_1/2` by type |
| rotor center `0.1` | `DynamicLight.cs` | spatial parameters | `gp_float_3` |
| disco vertical speed `1` | `DynamicLight.cs` | spatial parameters | `gp_float_3` |
| pulse speed `1`, modifier `0.25`, offset `0` | `DynamicLight.cs` | effect parameters | temporal runtime |
| fixed step `1/30 s` | `DynamicLightCache.cs`, `MathEx.FixedTimestep` | `FixedTimestep` | temporal runtime |
| bounce color `white/alpha 0`, modifier `1`, intensity `1` | `DynamicLight.cs` | bounce parameters | `bounce_color` |
| type bits `type << 6` | `DynamicLighting.cginc` | `packed_channel` | ABI/channel tests |
| realtime bit `1 << 5` | `DynamicLighting.cginc` | feature flags | ABI/channel tests |
| shadow bit `1 << 15`, cookie bit `1 << 16` | `DynamicLighting.cginc` | feature flags | ABI/channel tests |
| seven 16-byte `ShaderDynamicLight` blocks | `ShaderDynamicLight.cs` | `render/gpu.rs` | size/alignment/offset tests |

## Volumetric fog

| Original symbol | Upstream source | Rust destination | GPU destination / fixture |
| --- | --- | --- | --- |
| `None = 0` | `DynamicLightVolumetricType.cs` | `DynamicLightVolumetricType::None` | filtered before upload / `unity_volumetric_v1.json` |
| `Sphere = 1` | `DynamicLightingPostProcessing.shader` | volumetric config/packing | sphere branch / `unity_volumetric_v1.json` |
| `Box = 2` | `DynamicLightManager.PostProcessing.cs` | transform-scale packing | depth-limited ray/box branch / `unity_volumetric_v1.json` |
| `ConeZ = 3` | `DynamicLightManager.PostProcessing.cs` | forward-axis packing | depth-limited ray/cone branch / `unity_volumetric_v1.json` |
| `ConeY = 4` | `DynamicLightManager.PostProcessing.cs` | up-axis packing | depth-limited ray/cone branch / `unity_volumetric_v1.json` |
| radius `4`, thickness `1`, intensity `0.75`, visibility `2` | `DynamicLight.cs` | `DynamicLightVolumetricParameters` | recycled 112-byte fields / defaults and packing tests |
| temporal fog intensity | `DynamicLightManager.UpdateLightEffects` | shared `DynamicLightRuntime` multiplier | `volumetric_intensity` / strobe Cucumber scenario |
| screen blend and maximum opacity | `DynamicLightingPostProcessing.shader` | Unity reference exporter | `dynamic_lighting_volumetric.wgsl` / live on/off capture |

## Shadow boundary

`bevy_bridge/shadow_proxy.rs` is the only DynamicLighting file allowed to
mutate a Bevy `PointLight`. Its black proxy has constant intensity, so it
contributes no built-in direct light while Bevy renders the realtime cubemap.
`render/gpu.rs` maps that proxy's stable render-light index into
`shadow_cubemap_index`, and `dynamic_lighting_pass.wgsl` performs the comparison
sample before applying the custom surface contribution. Removing or hiding the
proxy restores visibility to 1.0 and leaves the custom light active.

## Source files executed or translated

- `AlpacaIT.DynamicLighting/Scripts/Core/DynamicLightManager.cs`
- `AlpacaIT.DynamicLighting/Scripts/Core/LightEffects/*.cs`
- `AlpacaIT.DynamicLighting/Scripts/Core/ShaderDynamicLight.cs`
- `AlpacaIT.DynamicLighting/Scripts/Lighting/DynamicLight.cs`
- `AlpacaIT.DynamicLighting/Scripts/Lighting/DynamicLightCache.cs`
- `AlpacaIT.DynamicLighting/Scripts/Lighting/DynamicLightEffect.cs`
- `AlpacaIT.DynamicLighting/Scripts/Lighting/DynamicLightType.cs`
- `AlpacaIT.DynamicLighting/Scripts/Lighting/DynamicLightVolumetricType.cs`
- `AlpacaIT.DynamicLighting/Scripts/Core/DynamicLightManager.PostProcessing.cs`
- `AlpacaIT.DynamicLighting/Scripts/Utilities/MathEx.cs` (`FixedTimestep`)
- `AlpacaIT.DynamicLighting/Shaders/DynamicLighting.cginc`
- `AlpacaIT.DynamicLighting/Shaders/DynamicLightingPostProcessing.shader`
