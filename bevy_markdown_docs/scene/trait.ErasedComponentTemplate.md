[bevy](../index.html)::[scene](index.html)

# Trait ErasedComponentTemplate 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#668)

```rust
pub trait ErasedComponentTemplate:
    Any
    + Send
    + Sync {
    // Required methods
    unsafe fn apply(
        &self,
        context: &mut TemplateContext<'_, '_>,
        bundle_writer: &mut BundleWriter<'_>,
    ) -> Result<(), BevyError>;
    fn clone_template(&self) -> Box<dyn ErasedComponentTemplate>;
}
```

A type-erased, object-safe, downcastable version of [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that produces a [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component"), which will be added to the given [`BundleWriter`](../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#676-680)

#### unsafe fn [apply](#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, bundle\_writer: &mut [BundleWriter](../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`.

##### Safety

`bundle_writer` must always be used with the same World that is stored in `context`. This is intended to be used by a scene system in a scoped / controlled / easily verifiable context. If you are calling it outside of that context, you are almost certainly doing something wrong!

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#683)

#### fn [clone\_template](#tymethod.clone_template)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedComponentTemplate](trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#686)

### impl<T> [ErasedComponentTemplate](trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),