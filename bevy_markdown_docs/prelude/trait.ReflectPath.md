[bevy](../index.html)::[prelude](index.html)

# Trait ReflectPath 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#50)

```rust
pub trait ReflectPath<'a>: Sized {
    // Required methods
    fn reflect_element(
        self,
        root: &(dyn PartialReflect + 'static),
    ) -> Result<&(dyn PartialReflect + 'static), ReflectPathError<'a>>;
    fn reflect_element_mut(
        self,
        root: &mut (dyn PartialReflect + 'static),
    ) -> Result<&mut (dyn PartialReflect + 'static), ReflectPathError<'a>>;

    // Provided methods
    fn element<T>(
        self,
        root: &(dyn PartialReflect + 'static),
    ) -> Result<&T, ReflectPathError<'a>>
       where T: Reflect { ... }
    fn element_mut<T>(
        self,
        root: &mut (dyn PartialReflect + 'static),
    ) -> Result<&mut T, ReflectPathError<'a>>
       where T: Reflect { ... }
}
```

Something that can be interpreted as a reflection path in [`GetPath`](trait.GetPath.html "trait bevy::prelude::GetPath").

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#55)

#### fn [reflect\_element](#tymethod.reflect_element)( self, root: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'a>>

Gets a reference to the specified element on the given [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") object.

See [`GetPath::reflect_path`](trait.GetPath.html#method.reflect_path "method bevy::prelude::GetPath::reflect_path") for more details, see [`element`](trait.ReflectPath.html#method.element "method bevy::prelude::ReflectPath::element") if you want a typed return value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#60-63)

#### fn [reflect\_element\_mut](#tymethod.reflect_element_mut)( self, root: &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'a>>

Gets a mutable reference to the specified element on the given [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") object.

See [`GetPath::reflect_path_mut`](trait.GetPath.html#method.reflect_path_mut "method bevy::prelude::GetPath::reflect_path_mut") for more details.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#68)

#### fn [element](#method.element)<T>( self, root: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'a>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a `&T` to the specified element on the given [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") object.

See [`GetPath::path`](trait.GetPath.html#method.path "method bevy::prelude::GetPath::path") for more details.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#78)

#### fn [element\_mut](#method.element_mut)<T>( self, root: &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'a>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a `&mut T` to the specified element on the given [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") object.

See [`GetPath::path_mut`](trait.GetPath.html#method.path_mut "method bevy::prelude::GetPath::path_mut") for more details.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#86)

### impl<'a> [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'a> for &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#87)

#### fn [reflect\_element](#tymethod.reflect_element)( self, root: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'a>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#94-97)

#### fn [reflect\_element\_mut](#tymethod.reflect_element_mut)( self, root: &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'a>>

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#444)

### impl<'a> [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'a> for &'a [ParsedPath](../reflect/struct.ParsedPath.html "struct bevy::reflect::ParsedPath")