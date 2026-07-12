[bevy](../../index.html)::[prelude](../index.html)

# Module iterable 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#293)

Iterable curves, which sample in the form of an iterator in order to support `Vec`\-like output whose length cannot be known statically.

## Traits

[IterableCurve](trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve")

A curve which provides samples in the form of [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")s.