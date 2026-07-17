# Isolated DynamicLighting slice

This module is the only home for the Henry00IS/AlpacaIT DynamicLighting port.
It is intentionally split into:

- `core/`: deterministic effect curves and light parameters, with no Bevy
  dependency;
- `bevy_bridge/`: the narrow ECS bridge that updates runtime point lights.

The intensity effects ported into the core are `Steady`, `Pulse`, `Random`,
`Strobe`, `Flicker`, `FluorescentStarter`, `FluorescentClicker`,
`FluorescentRandom`, `Candle`, `Pulsar`, `Fire`, `Generator`, `Lightning`,
`Cloudy`, and `Overcast`. Their noise is deterministic and seedable rather
than relying on Unity's global random state. The source spatial types
(`Point`, `Spot`, `Discoball`, `Wave`, `Interference`, `Rotor`, `Shock`, and
`Disco`) are represented as data contracts; their projection shaders are the
next bridge step.

`LightEffectState` also carries a default-on `bounce_multiplier` of `1.0`.
The existing prepared irradiance volume remains the offline one-diffuse-bounce
path; future iterations can move its channel/bounce payload into this slice
without leaking Unity types into the viewer.
