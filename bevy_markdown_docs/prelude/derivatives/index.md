[bevy](../../index.html)::[prelude](../index.html)

# Module derivatives 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#290)

This module holds traits related to extracting derivatives from curves. In applications, the derivatives of interest are chiefly the first and second; in this module, these are provided by the traits [`CurveWithDerivative`](trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative") and [`CurveWithTwoDerivatives`](trait.CurveWithTwoDerivatives.html "trait bevy::prelude::derivatives::CurveWithTwoDerivatives").

These take ownership of the curve they are used on by default, so that the resulting output may be used in more durable contexts. For example, `CurveWithDerivative<T>` is not dyn-compatible, but `Curve<WithDerivative<T>>` is, so if such a curve needs to be stored in a dynamic context, calling [`with_derivative`](trait.CurveWithDerivative.html#tymethod.with_derivative "method bevy::prelude::derivatives::CurveWithDerivative::with_derivative") and then placing the result in a `Box<Curve<WithDerivative<T>>>` is sensible.

On the other hand, in more transient contexts, consuming a value merely to sample derivatives is inconvenient, and in these cases, it is recommended to use [`by_ref`](../trait.CurveExt.html#method.by_ref "method bevy::prelude::CurveExt::by_ref") when possible to create a referential curve first, retaining liveness of the original.

This module also holds the [`SampleDerivative`](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative") and [`SampleTwoDerivatives`](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives") traits, which can be used to easily implement `CurveWithDerivative` and its counterpart.

## Modules

[adaptor\_impls](adaptor_impls/index.html "mod bevy::prelude::derivatives::adaptor_impls")

Implementations of derivatives on curve adaptors. These allow compositionality for derivatives.

## Structs

[SampleDerivativeWrapper](struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")

A wrapper that uses a [`SampleDerivative<T>`](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative") curve to produce a `Curve<WithDerivative<T>>`.

[SampleTwoDerivativesWrapper](struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")

A wrapper that uses a [`SampleTwoDerivatives<T>`](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives") curve to produce a `Curve<WithTwoDerivatives<T>>`.

## Traits

[CurveWithDerivative](trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative")

Trait for curves that have a well-defined notion of derivative, allowing for derivatives to be extracted along with values.

[CurveWithTwoDerivatives](trait.CurveWithTwoDerivatives.html "trait bevy::prelude::derivatives::CurveWithTwoDerivatives")

Trait for curves that have a well-defined notion of second derivative, allowing for two derivatives to be extracted along with values.

[SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")

A trait for curves that can sample derivatives in addition to values.

[SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")

A trait for curves that can sample two derivatives in addition to values.