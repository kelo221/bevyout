[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Trait SetViewVisibility 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#277)

```rust
pub trait SetViewVisibility {
    // Required method
    fn set_visible(&mut self);
}
```

## Required Methods

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#287)

#### fn [set\_visible](#tymethod.set_visible)(&mut self)

Sets the visibility to `true` if not already visible, triggering change detection only when needed. This should not be considered reversible for a given frame, as this component tracks if the entity is visible in _any_ view.

You should only manually set this if you are defining a custom visibility system, in which case the system should be placed in the [`CheckVisibility`](enum.VisibilitySystems.html#variant.CheckVisibility "variant bevy::camera::visibility::VisibilitySystems::CheckVisibility") set. For normal user-defined entity visibility, see [`Visibility`](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#290)

### impl<'a> [SetViewVisibility](trait.SetViewVisibility.html "trait bevy::camera::visibility::SetViewVisibility") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'a, [ViewVisibility](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")\>