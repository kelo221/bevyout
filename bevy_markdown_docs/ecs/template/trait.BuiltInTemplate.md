[bevy](../../index.html)::[ecs](../index.html)::[template](index.html)

# Trait BuiltInTemplate 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#507)

```rust
pub trait BuiltInTemplate: Sized {
    type Template: Template;
}
```

Roughly equivalent to [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"), but does not have a blanket implementation for [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") types. This is generally used for common generic collection types like [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") and [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec"), which have [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") impls and therefore also pick up the [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") behavior. This is fine when the `T` in [`Option<T>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") is not “templated” (ex: does not have an explicit [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") derive). But if `T` is “templated”, such as [`Option<Handle<T>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"), then it would require a manual `#[template(OptionTemplate<HandleTemplate<T>>)]` field annotation. This isn’t fun to type out.

[`BuiltInTemplate`](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate") enables equivalent “template type inference”, by annotating a field with a type that implements [`BuiltInTemplate`](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate") with `#[template(built_in)]`.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#509)

#### type [Template](#associatedtype.Template): [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")

The template to consider the “built in” template for this type.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#512)

### impl<T> [BuiltInTemplate](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#513)

#### type [Template](#associatedtype.Template) = [OptionTemplate](enum.OptionTemplate.html "enum bevy::ecs::template::OptionTemplate")<<T as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")\>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#516)

### impl<T> [BuiltInTemplate](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#517)

#### type [Template](#associatedtype.Template) = [VecTemplate](struct.VecTemplate.html "struct bevy::ecs::template::VecTemplate")<<T as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")\>