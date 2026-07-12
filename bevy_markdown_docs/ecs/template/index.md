[bevy](../../index.html)::[ecs](../index.html)

# Module template 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#55)

Functionality that relates to the [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") trait.

## Structs

[FnTemplate](struct.FnTemplate.html "struct bevy::ecs::template::FnTemplate")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") driven by a function that returns an output. This is used to create “free floating” templates without defining a new type. See [`template`](../../prelude/fn.template.html "fn bevy::prelude::template") for usage.

[InnerSceneEntityReference](struct.InnerSceneEntityReference.html "struct bevy::ecs::template::InnerSceneEntityReference")

The inner struct actually storing the unique index

[SceneEntityReference](struct.SceneEntityReference.html "struct bevy::ecs::template::SceneEntityReference")

A unique reference for a named entity in a scene. Usually used by `bevy_scene` in generated code

[SceneEntityReferences](struct.SceneEntityReferences.html "struct bevy::ecs::template::SceneEntityReferences")

Struct to store a mapping from [`SceneEntityReference`](struct.SceneEntityReference.html "struct bevy::ecs::template::SceneEntityReference") to [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") which are used for resolving `#Name` entity references in bsn! macros

[TemplateContext](struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")

The context used to apply the current [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"). This contains a reference to the entity that the template is being applied to (via an [`EntityWorldMut`](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")).

[TemplateTuple](struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")

A wrapper over a tuple of [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") implementations, which also implements [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"). This exists because [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") cannot be directly implemented for tuples of [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") implementations.

[VecTemplate](struct.VecTemplate.html "struct bevy::ecs::template::VecTemplate")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").

## Enums

[EntityTemplate](enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") reference to an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[OptionTemplate](enum.OptionTemplate.html "enum bevy::ecs::template::OptionTemplate")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

## Traits

[BuiltInTemplate](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate")

Roughly equivalent to [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"), but does not have a blanket implementation for [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") types. This is generally used for common generic collection types like [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") and [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec"), which have [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") impls and therefore also pick up the [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") behavior. This is fine when the `T` in [`Option<T>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") is not “templated” (ex: does not have an explicit [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") derive). But if `T` is “templated”, such as [`Option<Handle<T>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"), then it would require a manual `#[template(OptionTemplate<HandleTemplate<T>>)]` field annotation. This isn’t fun to type out.

[FromTemplate](trait.FromTemplate.html "trait bevy::ecs::template::FromTemplate")

[`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") is implemented for types that can be produced by a specific, canonical [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"). This creates a way to correlate to the [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") using the desired template output type. This is used by Bevy’s scene system.

[SpecializeFromTemplate](trait.SpecializeFromTemplate.html "trait bevy::ecs::template::SpecializeFromTemplate")

This is used to help improve error messages related to [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") specialization. Developers should generally just ignore this trait and read the error message when they encounter it.

[Template](trait.Template.html "trait bevy::ecs::template::Template")

A [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") is something that, given a spawn context (target [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), etc), can produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

## Functions

[template](fn.template.html "fn bevy::ecs::template::template")

Returns a “free floating” template for a given `func`. This prevents the need to define a custom type for one-off templates.

## Derive Macros

[FromTemplate](derive.FromTemplate.html "derive bevy::ecs::template::FromTemplate")

Derives `FromTemplate`.