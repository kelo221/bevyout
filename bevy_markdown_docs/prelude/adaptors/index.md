[bevy](../../index.html)::[prelude](../index.html)

# Module adaptors 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#288)

Adaptors used by the Curve API for transforming and combining curves together.

## Structs

[ChainCurve](struct.ChainCurve.html "struct bevy::prelude::adaptors::ChainCurve")

The curve that results from chaining one curve with another. The second curve is effectively reparametrized so that its start is at the end of the first.

[ConstantCurve](struct.ConstantCurve.html "struct bevy::prelude::adaptors::ConstantCurve")

A curve with a constant value over its domain.

[ContinuationCurve](struct.ContinuationCurve.html "struct bevy::prelude::adaptors::ContinuationCurve")

The curve that results from chaining two curves.

[CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::prelude::adaptors::CurveReparamCurve")

A curve that has been reparametrized by another curve, using that curve to transform the sample times before sampling. Curves of this type are produced by [`CurveExt::reparametrize_by_curve`](../trait.CurveExt.html#method.reparametrize_by_curve "method bevy::prelude::CurveExt::reparametrize_by_curve").

[ForeverCurve](struct.ForeverCurve.html "struct bevy::prelude::adaptors::ForeverCurve")

The curve that results from repeating a curve forever.

[FunctionCurve](struct.FunctionCurve.html "struct bevy::prelude::adaptors::FunctionCurve")

A curve defined by a function together with a fixed domain.

[GraphCurve](struct.GraphCurve.html "struct bevy::prelude::adaptors::GraphCurve")

A curve that is the graph of another curve over its parameter space. Curves of this type are produced by [`CurveExt::graph`](../trait.CurveExt.html#method.graph "method bevy::prelude::CurveExt::graph").

[LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::prelude::adaptors::LinearReparamCurve")

A curve that has had its domain changed by a linear reparameterization (stretching and scaling). Curves of this type are produced by [`CurveExt::reparametrize_linear`](../trait.CurveExt.html#method.reparametrize_linear "method bevy::prelude::CurveExt::reparametrize_linear").

[MapCurve](struct.MapCurve.html "struct bevy::prelude::adaptors::MapCurve")

A curve whose samples are defined by mapping samples from another curve through a given function. Curves of this type are produced by [`CurveExt::map`](../trait.CurveExt.html#method.map "method bevy::prelude::CurveExt::map").

[PingPongCurve](struct.PingPongCurve.html "struct bevy::prelude::adaptors::PingPongCurve")

The curve that results from chaining a curve with its reversed version. The transition point is guaranteed to make no jump.

[ReparamCurve](struct.ReparamCurve.html "struct bevy::prelude::adaptors::ReparamCurve")

A curve whose sample space is mapped onto that of some base curve’s before sampling. Curves of this type are produced by [`CurveExt::reparametrize`](../trait.CurveExt.html#method.reparametrize "method bevy::prelude::CurveExt::reparametrize").

[RepeatCurve](struct.RepeatCurve.html "struct bevy::prelude::adaptors::RepeatCurve")

The curve that results from repeating a curve `N` times.

[ReverseCurve](struct.ReverseCurve.html "struct bevy::prelude::adaptors::ReverseCurve")

The curve that results from reversing another.

[ZipCurve](struct.ZipCurve.html "struct bevy::prelude::adaptors::ZipCurve")

A curve that combines the output data from two constituent curves into a tuple output. Curves of this type are produced by [`CurveExt::zip`](../trait.CurveExt.html#method.zip "method bevy::prelude::CurveExt::zip").