[bevy](../../index.html)::[animation](../index.html)

# Module animatable 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#12)

Traits and type for interpolating between values.

## Structs

[BlendInput](struct.BlendInput.html "struct bevy::animation::animatable::BlendInput")

An individual input for [`Animatable::blend`](../../prelude/trait.Animatable.html#tymethod.blend "associated function bevy::prelude::Animatable::blend").

## Traits

[Animatable](trait.Animatable.html "trait bevy::animation::animatable::Animatable")

An animatable value type.

## Functions

[interpolate\_with\_cubic\_bezier](fn.interpolate_with_cubic_bezier.html "fn bevy::animation::animatable::interpolate_with_cubic_bezier")

Evaluates a cubic Bézier curve at a value `t`, given two endpoints and the derivatives at those endpoints.