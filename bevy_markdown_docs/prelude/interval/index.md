[bevy](../../index.html)::[prelude](../index.html)

# Module interval 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#292)

The [`Interval`](../struct.Interval.html "struct bevy::prelude::Interval") type for nonempty intervals used by the [`Curve`](../trait.Curve.html "trait bevy::prelude::Curve") trait.

## Structs

[Interval](struct.Interval.html "struct bevy::prelude::interval::Interval")

A nonempty closed interval, possibly unbounded in either direction.

[InvalidIntervalError](struct.InvalidIntervalError.html "struct bevy::prelude::interval::InvalidIntervalError")

An error that indicates that an operation would have returned an invalid [`Interval`](../struct.Interval.html "struct bevy::prelude::Interval").

[SpacedPointsError](struct.SpacedPointsError.html "struct bevy::prelude::interval::SpacedPointsError")

An error indicating that spaced points could not be extracted from an unbounded interval.

## Functions

[interval](fn.interval.html "fn bevy::prelude::interval::interval")

Create an [`Interval`](../struct.Interval.html "struct bevy::prelude::Interval") with a given `start` and `end`. Alias of [`Interval::new`](../struct.Interval.html#method.new "associated function bevy::prelude::Interval::new").