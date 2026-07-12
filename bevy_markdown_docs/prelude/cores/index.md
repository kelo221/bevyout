[bevy](../../index.html)::[prelude](../index.html)

# Module cores 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#289)

Core data structures to be used internally in Curve implementations, encapsulating storage and access patterns for reuse.

The `Core` types here expose their fields publicly so that it is easier to manipulate and extend them, but in doing so, you must maintain the invariants of those fields yourself. The provided methods all maintain the invariants, so this is only a concern if you manually mutate the fields.

## Structs

[ChunkedUnevenCore](struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")`alloc`

The data core of a curve using uneven samples (i.e. keyframes), where each sample time yields some fixed number of values — the [sampling width](struct.ChunkedUnevenCore.html#method.width "method bevy::prelude::cores::ChunkedUnevenCore::width"). This may serve as storage for curves that yield vectors or iterators, and in some cases, it may be useful for cache locality if the sample type can effectively be encoded as a fixed-length slice of values.

[EvenCore](struct.EvenCore.html "struct bevy::prelude::cores::EvenCore")`alloc`

The data core of a curve derived from evenly-spaced samples. The intention is to use this in addition to explicit or inferred interpolation information in user-space in order to implement curves using [`domain`](../struct.EvenCore.html#method.domain "method bevy::prelude::EvenCore::domain") and [`sample_with`](../struct.EvenCore.html#method.sample_with "method bevy::prelude::EvenCore::sample_with").

[UnevenCore](struct.UnevenCore.html "struct bevy::prelude::cores::UnevenCore")`alloc`

The data core of a curve defined by unevenly-spaced samples or keyframes. The intention is to use this in concert with implicitly or explicitly-defined interpolation in user-space in order to implement the curve interface using [`domain`](../struct.UnevenCore.html#method.domain "method bevy::prelude::UnevenCore::domain") and [`sample_with`](../struct.UnevenCore.html#method.sample_with "method bevy::prelude::UnevenCore::sample_with").

## Enums

[ChunkedUnevenCoreError](enum.ChunkedUnevenCoreError.html "enum bevy::prelude::cores::ChunkedUnevenCoreError")

An error that indicates that a [`ChunkedUnevenCore`](struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore") could not be formed.

[EvenCoreError](enum.EvenCoreError.html "enum bevy::prelude::cores::EvenCoreError")

An error indicating that an [`EvenCore`](../struct.EvenCore.html "struct bevy::prelude::EvenCore") could not be constructed.

[InterpolationDatum](enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")

This type expresses the relationship of a value to a fixed collection of values. It is a kind of summary used intermediately by sampling operations.

[UnevenCoreError](enum.UnevenCoreError.html "enum bevy::prelude::cores::UnevenCoreError")

An error indicating that an [`UnevenCore`](../struct.UnevenCore.html "struct bevy::prelude::UnevenCore") could not be constructed.

## Functions

[even\_interp](fn.even_interp.html "fn bevy::prelude::cores::even_interp")

Given a domain and a number of samples taken over that interval, return an [`InterpolationDatum`](enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum") that governs how samples are extracted relative to the stored data.

[uneven\_interp](fn.uneven_interp.html "fn bevy::prelude::cores::uneven_interp")

Given a list of `times` and a target value, get the interpolation relationship for the target value in terms of the indices of the starting list. In a sense, this encapsulates the heart of uneven/keyframe sampling.