[bevy](../../index.html)::[prelude](../index.html)

# Module sample\_curves 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#296)

Available on **crate feature `alloc`** only.

Sample-interpolated curves constructed using the [`Curve`](../trait.Curve.html "trait bevy::prelude::Curve") API.

## Structs

[SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::prelude::sample_curves::SampleAutoCurve")

A curve that is defined by neighbor interpolation over a set of evenly-spaced samples, interpolated automatically using [a particularly well-behaved interpolation](../trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[SampleCurve](struct.SampleCurve.html "struct bevy::prelude::sample_curves::SampleCurve")

A curve that is defined by explicit neighbor interpolation over a set of evenly-spaced samples.

[UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::prelude::sample_curves::UnevenSampleAutoCurve")

A curve that is defined by interpolation over unevenly spaced samples, interpolated automatically using [a particularly well-behaved interpolation](../trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::prelude::sample_curves::UnevenSampleCurve")

A curve that is defined by interpolation over unevenly spaced samples with explicit interpolation.