[bevy](../../index.html)::[animation](../index.html)::[prelude](index.html)

# Trait AnimationCompatibleCurve 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#277)

```rust
pub trait AnimationCompatibleCurve<T>:
    Curve<T>
    + Debug
    + Clone
    + Reflectable { }
```

This trait collects the additional requirements on top of [`Curve<T>`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") needed for a curve to be used as an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#279)

### impl<T, C> [AnimationCompatibleCurve](../../prelude/trait.AnimationCompatibleCurve.html "trait bevy::prelude::AnimationCompatibleCurve")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable"),