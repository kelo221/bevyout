[bevy](../index.html)::[scene](index.html)

# Trait ErasedBundleTemplate 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#708)

```rust
pub trait ErasedBundleTemplate:
    Any
    + Send
    + Sync {
    // Required methods
    unsafe fn apply(
        &self,
        context: &mut TemplateContext<'_, '_>,
    ) -> Result<(), BevyError>;
    fn clone_template(&self) -> Box<dyn ErasedBundleTemplate>;
}
```

A type-erased, object-safe, downcastable version of [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that produces a [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), which will be added immediately to a given `entity`.

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#716)

#### unsafe fn [apply](#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`.

##### Safety

`bundle_writer` must always be used with the same World that is stored in `context`. This is intended to be used by a scene system in a scoped / controlled / easily verifiable context. If you are calling it outside of that context, you are almost certainly doing something wrong!

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#719)

#### fn [clone\_template](#tymethod.clone_template)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedBundleTemplate](trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#722)

### impl<T> [ErasedBundleTemplate](trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),