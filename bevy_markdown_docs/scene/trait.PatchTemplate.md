[bevy](../index.html)::[scene](index.html)

# Trait PatchTemplate 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#321)

```rust
pub trait PatchTemplate: Sized {
    // Required method
    fn patch_template<F>(func: F) -> TemplatePatch<F, Self>
       where F: FnOnce(&mut Self, &mut ResolveContext<'_>);
}
```

A helper function that returns a [`TemplatePatch`](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#323-324)

#### fn [patch\_template](#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, Self>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self, &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template"),