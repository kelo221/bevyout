[bevy](../index.html)::[reflect](index.html)

# Trait TypeData 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#804)

```rust
pub trait TypeData:
    Downcast
    + Send
    + Sync {
    // Required method
    fn clone_type_data(&self) -> Box<dyn TypeData>;
}
```

A trait used to type-erase type metadata.

Type data can be registered to the [`TypeRegistry`](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") and stored on a type’s [`TypeRegistration`](struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration").

While type data is often generated using the [`#[reflect_trait]`](../prelude/attr.reflect_trait.html "attr bevy::prelude::reflect_trait") macro, almost any type that implements [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") can be considered “type data”. This is because it has a blanket implementation over all `T` where `T: Clone + Send + Sync + 'static`.

See the [crate-level documentation](index.html "mod bevy::reflect") for more information on type data and type registration.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#806)

#### fn [clone\_type\_data](#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

## Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#809)

### impl dyn [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#809)

#### pub fn [is](#method.is)<\_\_T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where \_\_T: [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData"),

Returns true if the trait object wraps an object of type `__T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#809)

#### pub fn [downcast](#method.downcast)<\_\_T>( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<\_\_T>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")\>>

where \_\_T: [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData"),

Returns a boxed object from a boxed trait object if the underlying object is of type `__T`. Returns the original boxed trait if it isn’t.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#809)

#### pub fn [downcast\_rc](#method.downcast_rc)<\_\_T>( self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<\_\_T>, [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")\>>

where \_\_T: [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData"),

Returns an `Rc`\-ed object from an `Rc`\-ed trait object if the underlying object is of type `__T`. Returns the original `Rc`\-ed trait if it isn’t.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#809)

#### pub fn [downcast\_ref](#method.downcast_ref)<\_\_T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&\_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData"),

Returns a reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#809)

#### pub fn [downcast\_mut](#method.downcast_mut)<\_\_T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut \_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData"),

Returns a mutable reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),