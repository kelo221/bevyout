[bevy](../index.html)::[scene](index.html)

# Trait PatchFromTemplate 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#301)

```rust
pub trait PatchFromTemplate {
    type Template;

    // Required method
    fn patch<F>(func: F) -> TemplatePatch<F, Self::Template>
       where F: FnOnce(&mut Self::Template, &mut ResolveContext<'_>);
}
```

A helper function that returns a [`TemplatePatch`](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`FromTemplate`](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"). It will use [`FromTemplate::Template`](../prelude/trait.FromTemplate.html#associatedtype.Template "associated type bevy::prelude::FromTemplate::Template") as the “patched template”.

## Required Associated Types

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#303)

#### type [Template](#associatedtype.Template)

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#306-308)

#### fn [patch](#tymethod.patch)<F>(func: F) -> [TemplatePatch](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, Self::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](#associatedtype.Template) = <G as [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")