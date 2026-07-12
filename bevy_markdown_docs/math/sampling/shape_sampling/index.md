[bevy](../../../index.html)::[math](../../index.html)::[sampling](../index.html)

# Module shape\_sampling 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/mod.rs.html#7)

Available on **crate feature `rand`** only.

The [`ShapeSample`](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") trait, allowing random sampling from geometric shapes.

At the most basic level, this allows sampling random points from the interior and boundary of geometric primitives. For example:

```rust
// Get some `RngExt`:
let mut rng: StdRng = rand::make_rng();
// Make a circle of radius 2:
let circle = Circle::new(2.0);
// Get a point inside this circle uniformly at random:
let interior_pt = circle.sample_interior(&mut rng);
// Get a point on the circle's boundary uniformly at random:
let boundary_pt = circle.sample_boundary(&mut rng);
```

For repeated sampling, `ShapeSample` also includes methods for accessing a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution"):

```rust
// Use a rectangle this time:
let rectangle = Rectangle::new(1.0, 2.0);
// Get an iterator that spits out random interior points:
let interior_iter = rectangle.interior_dist().sample_iter(&mut rng1);
// Collect random interior points from the iterator:
let interior_pts: Vec<Vec2> = interior_iter.take(1000).collect();
// Similarly, get an iterator over many random boundary points and collect them:
let boundary_pts: Vec<Vec2> = rectangle.boundary_dist().sample_iter(&mut rng2).take(1000).collect();
```

In any case, the [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") used as the source of randomness must be provided explicitly.

## Structs

[BoundaryOf](struct.BoundaryOf.html "struct bevy::math::sampling::shape_sampling::BoundaryOf")

A wrapper struct that allows boundary sampling from a [`ShapeSample`](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") type directly as a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution").

[InteriorOf](struct.InteriorOf.html "struct bevy::math::sampling::shape_sampling::InteriorOf")

A wrapper struct that allows interior sampling from a [`ShapeSample`](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") type directly as a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution").

## Traits

[ShapeSample](trait.ShapeSample.html "trait bevy::math::sampling::shape_sampling::ShapeSample")

Exposes methods to uniformly sample a variety of primitive shapes.