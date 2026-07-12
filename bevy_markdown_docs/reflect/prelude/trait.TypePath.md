[bevy](../../index.html)::[reflect](../index.html)::[prelude](index.html)

# Trait TypePath 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#86)

```rust
pub trait TypePath: 'static {
    // Required methods
    fn type_path() -> &'static str;
    fn short_type_path() -> &'static str;

    // Provided methods
    fn type_ident() -> Option<&'static str> { ... }
    fn crate_name() -> Option<&'static str> { ... }
    fn module_path() -> Option<&'static str> { ... }
}
```

A static accessor to type paths and names.

The engine uses this trait over [`core::any::type_name`](https://doc.rust-lang.org/nightly/core/any/fn.type_name.html "fn core::any::type_name") for stability and flexibility.

This trait is automatically implemented by the `#[derive(Reflect)]` macro and allows type path information to be processed without an instance of that type.

Implementors may have difficulty in generating references with static lifetimes. Luckily, this crate comes with some [utility](../utility/index.html "mod bevy::reflect::utility") structs, to make generating these statics much simpler.

## Stability

Certain parts of the engine, e.g. [(de)serialization](../serde/struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer"), rely on type paths as identifiers for matching dynamic values to concrete types.

Using [`core::any::type_name`](https://doc.rust-lang.org/nightly/core/any/fn.type_name.html "fn core::any::type_name"), a scene containing `my_crate::foo::MyComponent` would break, failing to deserialize if the component was moved from the `foo` module to the `bar` module, becoming `my_crate::bar::MyComponent`. This trait, through attributes when deriving itself or [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"), can ensure breaking changes are avoidable.

The only external factor we rely on for stability when deriving is the [`module_path!`](https://doc.rust-lang.org/nightly/core/macro.module_path.html "macro core::module_path") macro, only if the derive does not provide a `#[type_path = "..."]` attribute.

## Anonymity

Some methods on this trait return `Option<&'static str>` over `&'static str` because not all types define all parts of a type path, for example the array type `[T; N]`.

Such types are ‘anonymous’ in that they have only a defined [`type_path`](../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") and [`short_type_path`](../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path") and the methods [`crate_name`](../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name"), [`module_path`](../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path") and [`type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident") all return `None`.

Primitives are treated like anonymous types, except they also have a defined [`type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

## Example

```rust
use bevy_reflect::TypePath;

// This type path will not change with compiler versions or recompiles,
// although it will not be the same if the definition is moved.
#[derive(TypePath)]
struct NonStableTypePath;

// This type path will never change, even if the definition is moved.
#[derive(TypePath)]
#[type_path = "my_crate::foo"]
struct StableTypePath;

// Type paths can have any number of path segments.
#[derive(TypePath)]
#[type_path = "my_crate::foo::bar::baz"]
struct DeeplyNestedStableTypePath;

// Including just a crate name!
#[derive(TypePath)]
#[type_path = "my_crate"]
struct ShallowStableTypePath;

// We can also rename the identifier/name of types.
#[derive(TypePath)]
#[type_path = "my_crate::foo"]
#[type_name = "RenamedStableTypePath"]
struct NamedStableTypePath;

// Generics are also supported.
#[derive(TypePath)]
#[type_path = "my_crate::foo"]
struct StableGenericTypePath<T, const N: usize>([T; N]);
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#92)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type.

Generic parameter types are also fully expanded.

For `Option<Vec<usize>>`, this is `"std::option::Option<std::vec::Vec<usize>>"`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#99)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type.

Generic parameter types are also shortened.

For `Option<Vec<usize>>`, this is `"Option<Vec<usize>>"`.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#108)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath").

Primitive types will return [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some").

For `Option<Vec<usize>>`, this is `"Option"`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#117)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath").

For `Option<Vec<usize>>`, this is `"core"`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#126)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath").

For `Option<Vec<usize>>`, this is `"std::option"`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for &'static [Location](https://doc.rust-lang.org/nightly/core/panic/location/struct.Location.html "struct core::panic::location::Location")<'static>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#17)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#21)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#771-777)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#771-777)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#771-777)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FixedState](https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.FixedState.html "struct foldhash::quality::FixedState")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#7)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#7)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#7)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#7)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#7)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#300)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#300)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#300)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#300)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#300)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#300)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RandomState](https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html "struct foldhash::quality::RandomState")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#8)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#8)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#8)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#8)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#8)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RandomState](https://doc.rust-lang.org/nightly/std/hash/random/struct.RandomState.html "struct std::hash::random::RandomState")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#13)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#13)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#13)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#13)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#13)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#318)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#318)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#318)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#318)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#318)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#318)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#22)

### impl<'a, T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, T>

where 'a: 'static, T: [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'a, T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#22)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#22)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#22)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#22)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#22)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#6)

### impl<'a> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FoldHasher](https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.FoldHasher.html "struct foldhash::quality::FoldHasher")<'a>

where [FoldHasher](https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.FoldHasher.html "struct foldhash::quality::FoldHasher")<'a>: 'static,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#6)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#6)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#6)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#6)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#6)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/hash.rs.html#3)

### impl<H> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BuildHasherDefault](https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html "struct core::hash::BuildHasherDefault")<H>

where [BuildHasherDefault](https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html "struct core::hash::BuildHasherDefault")<H>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), H: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/hash.rs.html#3)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/hash.rs.html#3)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/hash.rs.html#3)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/hash.rs.html#3)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/hash.rs.html#3)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#14)

### impl<K, V, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>

where [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), K: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#14)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#14)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#14)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#14)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#14)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#280)

### impl<K, V, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>

where [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), K: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#280)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#280)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#280)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#280)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#280)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#229)

### impl<K, V> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>

where [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), K: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#229)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#229)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#229)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#229)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#229)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

### impl<N, E, Ix> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>

where N: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), E: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), Ix: [IndexType](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/trait.IndexType.html "trait petgraph::graph_impl::IndexType") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#771-777)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [(P₁, P₂, …, Pₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where P: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

This trait is implemented for tuples up to 13 items long.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#771-777)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#771-777)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

### impl<T, E> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), E: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#499)

### impl<T, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>

where [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#499)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#499)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#499)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#499)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#499)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#627)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#628)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#633)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#669)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [&'static T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#670)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#675)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#681)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [&'static mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#682)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#687)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#208)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>

where T: [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#208)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#208)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#208)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#208)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#208)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#18)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#18)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#18)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#18)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#18)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#18)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#654-656)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#658)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#663)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#10)

### impl<V, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>

where [HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#10)

#### fn [type\_path](#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#10)

#### fn [short\_type\_path](#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#10)

#### fn [type\_ident](#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#10)

#### fn [crate\_name](#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#10)

#### fn [module\_path](#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#62)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Aabb](../../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#42)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Aabb2d](../../math/bounding/struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#48)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Aabb3d](../../math/bounding/struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#112)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AabbCast2d](../../math/bounding/struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#109)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AabbCast3d](../../math/bounding/struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#43)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AabbGizmoConfigGroup](../../prelude/struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#110)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AccessibilityRequested](../../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#251)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AccessibilitySystems](../../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/accessibility.rs.html#185)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AccessibleLabel](../../prelude/struct.AccessibleLabel.html "struct bevy::prelude::AccessibleLabel")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#210)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AccumulatedMouseMotion](../../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#231)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AccumulatedMouseScroll](../../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#208)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AcquireFocus](../../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Activate](../../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#33)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ActivateOnPress](../../ui_widgets/struct.ActivateOnPress.html "struct bevy::ui_widgets::ActivateOnPress")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ActiveAnimation](../../animation/struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#49)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ActiveDescendant](../../ui_widgets/struct.ActiveDescendant.html "struct bevy::ui_widgets::ActiveDescendant")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#333)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Add](../../prelude/struct.Add.html "struct bevy::prelude::Add")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Affine2](../../math/struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#416-423)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Affine3](../../math/struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#424-431)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Affine3A](../../math/struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1055)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AlignContent](../../prelude/enum.AlignContent.html "enum bevy::prelude::AlignContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#895)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AlignItems](../../prelude/enum.AlignItems.html "enum bevy::prelude::AlignItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#975)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AlignSelf](../../prelude/enum.AlignSelf.html "enum bevy::prelude::AlignSelf")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/alpha.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AlphaMode](../../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#245)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AlphaMode2d](../../sprite_render/enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#9)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AmbientLight](../../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#254)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Anchor](../../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AngularColorStop](../../prelude/struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#213)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimatedBy](../../animation/struct.AnimatedBy.html "struct bevy::animation::AnimatedBy")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationClip](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationGraph](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#244)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationGraphAssetLoader](../../prelude/struct.AnimationGraphAssetLoader.html "struct bevy::prelude::AnimationGraphAssetLoader")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationGraphHandle](../../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#169)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationGraphNode](../../prelude/struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#211)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationNodeType](../../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#730)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationPlayer](../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#184)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationTargetId](../../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#54)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationTransition](../../prelude/struct.AnimationTransition.html "struct bevy::prelude::AnimationTransition")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimationTransitions](../../prelude/struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#955)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Annulus](../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#745)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnnulusMeshBuilder](../../mesh/struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1565)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AppExit](../../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#453)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AppLifecycle](../../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#117)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Arc2d](../../prelude/struct.Arc2d.html "struct bevy::prelude::Arc2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#14)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AspectRatio](../../math/struct.AspectRatio.html "struct bevy::math::AspectRatio")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AssetIndex](../../asset/struct.AssetIndex.html "struct bevy::asset::AssetIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#414)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AtmosphereMode](../../pbr/enum.AtmosphereMode.html "enum bevy::pbr::AtmosphereMode")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#286)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AtmosphereSettings](../../pbr/struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#38)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AudioLoader](../../audio/struct.AudioLoader.html "struct bevy::audio::AudioLoader")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AudioSource](../../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#105)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AutoDirectionalNavigation](../../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AutoExposureCompensationCurve](../../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/autofocus.rs.html#20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AutoFocus](../../input_focus/struct.AutoFocus.html "struct bevy::input_focus::AutoFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#90)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AutoNavigationConfig](../../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#984)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AxisSettings](../../input/gamepad/struct.AxisSettings.html "struct bevy::input::gamepad::AxisSettings")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#287-294)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BVec2](../../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#295-303)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#304-313)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BVec4](../../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#502-508)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#509-515)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BVec4A](../../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Backfaces](../../picking/mesh_picking/ray_cast/enum.Backfaces.html "enum bevy::picking::mesh_picking::ray_cast::Backfaces")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2222)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BackgroundColor](../../prelude/struct.BackgroundColor.html "struct bevy::prelude::BackgroundColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#526)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BackgroundGradient](../../prelude/struct.BackgroundGradient.html "struct bevy::prelude::BackgroundGradient")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#11-18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BlendState](../../render/render_resource/struct.BlendState.html "struct bevy::render::render_resource::BlendState")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#30)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Bloom](../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#216)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BloomCompositeMode](../../post_process/bloom/enum.BloomCompositeMode.html "enum bevy::post_process::bloom::BloomCompositeMode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#199)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BloomPrefilter](../../post_process/bloom/struct.BloomPrefilter.html "struct bevy::post_process::bloom::BloomPrefilter")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2249)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BorderColor](../../prelude/struct.BorderColor.html "struct bevy::prelude::BorderColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#542)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BorderGradient](../../prelude/struct.BorderGradient.html "struct bevy::prelude::BorderGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BorderRadius](../../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/border_rect.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BorderRect](../../prelude/struct.BorderRect.html "struct bevy::prelude::BorderRect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#478)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoundingCircle](../../math/bounding/struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#150)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoundingCircleCast](../../math/bounding/struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#504)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoundingSphere](../../math/bounding/struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#154)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoundingSphereCast](../../math/bounding/struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2831)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoxShadow](../../prelude/struct.BoxShadow.html "struct bevy::prelude::BoxShadow")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#186)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoxShadowSamples](../../prelude/struct.BoxShadowSamples.html "struct bevy::prelude::BoxShadowSamples")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1181)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [BoxSizing](../../prelude/enum.BoxSizing.html "enum bevy::prelude::BoxSizing")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/button.rs.html#6)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::prelude::[Button](../../prelude/struct.Button.html "struct bevy::prelude::Button")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::ui\_widgets::[Button](../../ui_widgets/struct.Button.html "struct bevy::ui_widgets::Button")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1412)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ButtonAxisSettings](../../input/gamepad/struct.ButtonAxisSettings.html "struct bevy::input::gamepad::ButtonAxisSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#820)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ButtonSettings](../../input/gamepad/struct.ButtonSettings.html "struct bevy::input::gamepad::ButtonSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#172)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ButtonState](../../input/enum.ButtonState.html "enum bevy::input::ButtonState")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ButtonVariant](../../feathers/controls/enum.ButtonVariant.html "enum bevy::feathers::controls::ButtonVariant")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2407)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CalculatedClip](../../prelude/struct.CalculatedClip.html "struct bevy::prelude::CalculatedClip")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#374)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#9)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Camera2d](../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Camera3d](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#58)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Camera3dDepthLoadOp](../../camera/enum.Camera3dDepthLoadOp.html "enum bevy::camera::Camera3dDepthLoadOp")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#41)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Camera3dDepthTextureUsage](../../camera/struct.Camera3dDepthTextureUsage.html "struct bevy::camera::Camera3dDepthTextureUsage")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#1044)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CameraMainTextureUsages](../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#860)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CameraOutputMode](../../camera/enum.CameraOutputMode.html "enum bevy::camera::CameraOutputMode")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#176)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CameraRenderGraph](../../render/camera/struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#178)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cancel](../../prelude/struct.Cancel.html "struct bevy::prelude::Cancel")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2183)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Capsule2d](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1121)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Capsule2dMeshBuilder](../../mesh/struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#856)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Capsule3d](../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Capsule3dMeshBuilder](../../mesh/struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CapsuleUvProfile](../../mesh/enum.CapsuleUvProfile.html "enum bevy::mesh::CapsuleUvProfile")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#179)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cascade](../../light/cascade/struct.Cascade.html "struct bevy::light::cascade::Cascade")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#24)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CascadeShadowConfig](../../light/struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#167)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cascades](../../light/struct.Cascades.html "struct bevy::light::Cascades")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#443)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CascadesFrusta](../../camera/primitives/struct.CascadesFrusta.html "struct bevy::camera::primitives::CascadesFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#460)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CascadesVisibleEntities](../../camera/visibility/struct.CascadesVisibleEntities.html "struct bevy::camera::visibility::CascadesVisibleEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#49)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Checkable](../../ui/struct.Checkable.html "struct bevy::ui::Checkable")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Checkbox](../../ui_widgets/struct.Checkbox.html "struct bevy::ui_widgets::Checkbox")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#54)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Checked](../../ui/struct.Checked.html "struct bevy::ui::Checked")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#149)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Children](../../prelude/struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#43)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ChromaticAberration](../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#29)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Circle](../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CircleMeshBuilder](../../mesh/struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#106)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CircularMeshUvMode](../../mesh/enum.CircularMeshUvMode.html "enum bevy::mesh::CircularMeshUvMode")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#285)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CircularSector](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#128)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CircularSectorMeshBuilder](../../mesh/struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#437)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CircularSegment](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#266)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CircularSegmentMeshBuilder](../../mesh/struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ClearColor](../../prelude/struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ClearColorConfig](../../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#309)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Click](../../prelude/struct.Click.html "struct bevy::prelude::Click")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#105)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ClusterConfig](../../light/cluster/enum.ClusterConfig.html "enum bevy::light::cluster::ClusterConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#82)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ClusterFarZMode](../../light/cluster/enum.ClusterFarZMode.html "enum bevy::light::cluster::ClusterFarZMode")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ClusterZConfig](../../light/cluster/struct.ClusterZConfig.html "struct bevy::light::cluster::ClusterZConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#229)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ClusteredDecal](../../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#47)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorChannel](../../feathers/controls/enum.ColorChannel.html "enum bevy::feathers::controls::ColorChannel")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#399)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorGrading](../../render/view/struct.ColorGrading.html "struct bevy::render::view::ColorGrading")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#428)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorGradingGlobal](../../render/view/struct.ColorGradingGlobal.html "struct bevy::render::view::ColorGradingGlobal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#494)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorGradingSection](../../render/view/struct.ColorGradingSection.html "struct bevy::render::view::ColorGradingSection")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorMaterial](../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#68)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorPlaneValue](../../feathers/controls/struct.ColorPlaneValue.html "struct bevy::feathers::controls::ColorPlaneValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#187)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorSlider](../../feathers/controls/struct.ColorSlider.html "struct bevy::feathers::controls::ColorSlider")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#10)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorStop](../../prelude/struct.ColorStop.html "struct bevy::prelude::ColorStop")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#40)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorSwatchFg](../../feathers/controls/struct.ColorSwatchFg.html "struct bevy::feathers::controls::ColorSwatchFg")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#33)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorSwatchValue](../../feathers/controls/struct.ColorSwatchValue.html "struct bevy::feathers::controls::ColorSwatchValue")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#132)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CompassOctant](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#25)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CompassQuadrant](../../math/enum.CompassQuadrant.html "enum bevy::math::CompassQuadrant")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/info.rs.html#178)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#136)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComponentTicks](../../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1295)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CompositeAlphaMode](../../window/enum.CompositeAlphaMode.html "enum bevy::window::CompositeAlphaMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#92)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CompositingSpace](../../prelude/enum.CompositingSpace.html "enum bevy::prelude::CompositingSpace")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#12)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CompressedImageSaver](../../image/struct.CompressedImageSaver.html "struct bevy::image::CompressedImageSaver")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CompressedImageSaverError](../../image/enum.CompressedImageSaverError.html "enum bevy::image::CompressedImageSaverError")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#217)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComputedCameraValues](../../camera/struct.ComputedCameraValues.html "struct bevy::camera::ComputedCameraValues")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#26)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComputedStackIndex](../../ui/struct.ComputedStackIndex.html "struct bevy::ui::ComputedStackIndex")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#37)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3036)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComputedUiRenderTargetInfo](../../prelude/struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3014)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#927)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cone](../../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConeAnchor](../../mesh/enum.ConeAnchor.html "enum bevy::mesh::ConeAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConeMeshBuilder](../../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#410)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConicGradient](../../prelude/struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1010)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConicalFrustum](../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConicalFrustumMeshBuilder](../../mesh/struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ContactShadows](../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#139)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ContentSize](../../ui/struct.ContentSize.html "struct bevy::ui::ContentSize")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#37)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ContrastAdaptiveSharpening](../../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ControlOrientation](../../ui_widgets/enum.ControlOrientation.html "enum bevy::ui_widgets::ControlOrientation")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1950)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConvexPolygon](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#413)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConvexPolygonMeshBuilder](../../mesh/struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#392)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubemapFrusta](../../camera/primitives/struct.CubemapFrusta.html "struct bevy::camera::primitives::CubemapFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#408)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubemapLayout](../../camera/primitives/enum.CubemapLayout.html "enum bevy::camera::primitives::CubemapLayout")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#435)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubemapVisibleEntities](../../camera/visibility/struct.CubemapVisibleEntities.html "struct bevy::camera::visibility::CubemapVisibleEntities")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicRotationCurve](../../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#684)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cuboid](../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CuboidMeshBuilder](../../mesh/struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#209)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CursorEntered](../../prelude/struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1076)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CursorGrabMode](../../window/enum.CursorGrabMode.html "enum bevy::window::CursorGrabMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/mod.rs.html#24)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CursorIcon](../../window/enum.CursorIcon.html "enum bevy::window::CursorIcon")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#226)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CursorLeft](../../prelude/struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#184)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CursorMoved](../../prelude/struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#744)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CursorOptions](../../window/struct.CursorOptions.html "struct bevy::window::CursorOptions")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#71)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CustomCursor](../../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#15)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CustomCursorImage](../../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#55)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CustomCursorUrl](../../window/struct.CustomCursorUrl.html "struct bevy::window::CustomCursorUrl")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#109)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CustomProjection](../../camera/struct.CustomProjection.html "struct bevy::camera::CustomProjection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#777)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Cylinder](../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CylinderAnchor](../../mesh/enum.CylinderAnchor.html "enum bevy::mesh::CylinderAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CylinderMeshBuilder](../../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#433-440)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DAffine2](../../math/struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#441-448)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DAffine3](../../math/struct.DAffine3.html "struct bevy::math::DAffine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#380-387)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DMat2](../../math/struct.DMat2.html "struct bevy::math::DMat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DMat3](../../math/struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#397-406)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DMat4](../../math/struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#460-469)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DQuat](../../math/struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#315-322)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DVec2](../../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DVec3](../../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DVec4](../../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DefaultCursor](../../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#84)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DefaultGizmoConfigGroup](../../prelude/struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DefaultOpaqueRendererMethod](../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#172)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DefaultQueryFilters](../../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#232)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DefaultSpatialScale](../../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#82)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DeferredPrepass](../../core_pipeline/prepass/struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#93)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DeferredPrepassDoubleBuffer](../../core_pipeline/prepass/struct.DeferredPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#133)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DelayedCommandQueue](../../time/struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#66)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DenoiseCas](../../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#75)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DepthOfField](../../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#119)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DepthOfFieldMode](../../post_process/dof/enum.DepthOfFieldMode.html "enum bevy::post_process::dof::DepthOfFieldMode")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#62)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DepthPrepass](../../core_pipeline/prepass/struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#87)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DepthPrepassDoubleBuffer](../../core_pipeline/prepass/struct.DepthPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#388)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Despawn](../../prelude/struct.Despawn.html "struct bevy::prelude::Despawn")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#88)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#399)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Dir3](../../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1053)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Dir4](../../math/struct.Dir4.html "struct bevy::math::Dir4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#803)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Dir3A](../../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#61)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DirectionalLight](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#191)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DirectionalLightShadowMap](../../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DirectionalLightTexture](../../light/struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#251)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DirectionalNavigationMap](../../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#354)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DirectlyHovered](../../picking/hover/struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#131)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Disabled](../../ecs/entity_disabling/struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#361)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Discard](../../prelude/struct.Discard.html "struct bevy::prelude::Discard")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1147)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Display](../../prelude/enum.Display.html "enum bevy::prelude::Display")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#66)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DoubleTapGesture](../../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#348)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Drag](../../prelude/struct.Drag.html "struct bevy::prelude::Drag")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#421)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragDrop](../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#370)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragEnd](../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#385)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragEnter](../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#433)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragEntry](../../prelude/struct.DragEntry.html "struct bevy::prelude::DragEntry")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#409)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragLeave](../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#397)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragOver](../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#338)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DragStart](../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#350)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicArray](../array/struct.DynamicArray.html "struct bevy::reflect::array::DynamicArray")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/dynamic_enum.rs.html#422)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicEnum](../enums/struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#339)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicList](../list/struct.DynamicList.html "struct bevy::reflect::list::DynamicList")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#409)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicMap](../map/struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#335)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicSet](../set/struct.DynamicSet.html "struct bevy::reflect::set::DynamicSet")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#329)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicSkinnedMeshBounds](../../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#543)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicStruct](../structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#362)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicTuple](../tuple/struct.DynamicTuple.html "struct bevy::reflect::tuple::DynamicTuple")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#379)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicTupleStruct](../tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/dynamic_world.rs.html#24)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicWorld](../../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicWorldRoot](../../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#431)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EaseFunction](../../prelude/enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#804)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Ellipse](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#556)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EllipseMeshBuilder](../../mesh/struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1432)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EnabledButtons](../../window/struct.EnabledButtons.html "struct bevy::window::EnabledButtons")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#223)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Enter](../../prelude/struct.Enter.html "struct bevy::prelude::Enter")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#414)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityCursor](../../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#248)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityGeneration](../../ecs/entity/struct.EntityGeneration.html "struct bevy::ecs::entity::EntityGeneration")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityHash](../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityHashSet](../../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#147)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityIndex](../../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#29)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityIndexSet](../../ecs/entity/struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EnvironmentMapLight](../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#90)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ErasedGizmoConfigGroup](../../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#471-500)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EulerRot](../../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#229)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Exposure](../../camera/struct.Exposure.html "struct bevy::camera::Exposure")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#10)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ExrTextureLoader](../../image/struct.ExrTextureLoader.html "struct bevy::image::ExrTextureLoader")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#24)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ExrTextureLoaderError](../../image/enum.ExrTextureLoaderError.html "enum bevy::image::ExrTextureLoaderError")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#59)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersButton](../../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#48)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersCheckbox](../../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersColorPlane](../../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#162)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersColorSlider](../../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersColorSwatch](../../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersDisclosureToggle](../../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersListRow](../../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersListView](../../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersMenu](../../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#141)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersMenuButton](../../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersMenuDivider](../../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#250)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersMenuItem](../../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersMenuPopup](../../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#55)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersNumberInput](../../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#47)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersRadio](../../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersScrollbar](../../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersSlider](../../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#85)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersTextInput](../../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersTextInputContainer](../../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersToggleSwitch](../../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#126)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FeathersToolButton](../../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#376)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FileDragAndDrop](../../prelude/enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#68)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Fixed](../../prelude/struct.Fixed.html "struct bevy::prelude::Fixed")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#4)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FixedHasher](../../platform/hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#4)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::platform::hash::[FixedState](../../platform/hash/struct.FixedState.html "struct bevy::platform::hash::FixedState")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1206)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FlexDirection](../../prelude/enum.FlexDirection.html "enum bevy::prelude::FlexDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1478)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FlexWrap](../../prelude/enum.FlexWrap.html "enum bevy::prelude::FlexWrap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/float_ord.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#15)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusCause](../../input_focus/enum.FocusCause.html "enum bevy::input_focus::FocusCause")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#33)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusGained](../../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusIndicator](../../feathers/focus/struct.FocusIndicator.html "struct bevy::feathers::focus::FocusIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#50)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusLost](../../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#101)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#30)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusWithinIndicator](../../feathers/focus/struct.FocusWithinIndicator.html "struct bevy::feathers::focus::FocusWithinIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#469)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusableArea](../../input_focus/directional_navigation/struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#100)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FogFalloff](../../prelude/enum.FogFalloff.html "enum bevy::prelude::FogFalloff")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#75)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FogVolume](../../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font.rs.html#28)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Font](../../prelude/struct.Font.html "struct bevy::prelude::Font")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#728)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontFeatureTag](../../text/struct.FontFeatureTag.html "struct bevy::text::FontFeatureTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#839)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontFeatures](../../text/struct.FontFeatures.html "struct bevy::text::FontFeatures")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1199)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontHinting](../../prelude/enum.FontHinting.html "enum bevy::prelude::FontHinting")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#6)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontLoader](../../text/struct.FontLoader.html "struct bevy::text::FontLoader")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#486)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontSize](../../prelude/enum.FontSize.html "enum bevy::prelude::FontSize")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1179)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontSmoothing](../../prelude/enum.FontSmoothing.html "enum bevy::prelude::FontSmoothing")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontSource](../../prelude/enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#704)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontStyle](../../prelude/enum.FontStyle.html "enum bevy::prelude::FontStyle")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#913)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontVariationTag](../../text/struct.FontVariationTag.html "struct bevy::text::FontVariationTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#960)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontVariations](../../text/struct.FontVariations.html "struct bevy::text::FontVariations")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#596)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontWeight](../../prelude/struct.FontWeight.html "struct bevy::prelude::FontWeight")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#659)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FontWidth](../../prelude/struct.FontWidth.html "struct bevy::prelude::FontWidth")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#73)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ForceTouch](../../input/touch/enum.ForceTouch.html "enum bevy::input::touch::ForceTouch")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#62)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ForwardDecal](../../pbr/decal/struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#82)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ForwardDecalMaterialExt](../../pbr/decal/struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#108)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FpsOverlayConfig](../../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#139)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FrameTimeGraphConfig](../../dev_tools/fps_overlay/struct.FrameTimeGraphConfig.html "struct bevy::dev_tools::fps_overlay::FrameTimeGraphConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#80)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FrametimeGraphMaterial](../../dev_tools/frame_time_graph/struct.FrametimeGraphMaterial.html "struct bevy::dev_tools::frame_time_graph::FrametimeGraphMaterial")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#247)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Frustum](../../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#78)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FrustumGizmoConfigGroup](../../prelude/struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#371)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Gamepad](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#664)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadAxis](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#258)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadAxisChangedEvent](../../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#572)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadButton](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#222)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadButtonChangedEvent](../../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#190)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadButtonStateChangedEvent](../../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1554)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadConnection](../../input/gamepad/enum.GamepadConnection.html "enum bevy::input::gamepad::GamepadConnection")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#151)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadConnectionEvent](../../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#38)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadEvent](../../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#710)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadInput](../../input/gamepad/enum.GamepadInput.html "enum bevy::input::gamepad::GamepadInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1688)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadRumbleIntensity](../../input/gamepad/struct.GamepadRumbleIntensity.html "struct bevy::input::gamepad::GamepadRumbleIntensity")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1778)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadRumbleRequest](../../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#736)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GamepadSettings](../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GeneratedEnvironmentMapLight](../../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GhostNode](../../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Gizmo](../../prelude/struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#325)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoAsset](../../prelude/struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#206)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoConfig](../../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#97)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoConfigStore](../../prelude/struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#246)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoLineConfig](../../prelude/struct.GizmoLineConfig.html "struct bevy::prelude::GizmoLineConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#19)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoLineJoint](../../prelude/enum.GizmoLineJoint.html "enum bevy::prelude::GizmoLineJoint")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#37)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoLineStyle](../../prelude/enum.GizmoLineStyle.html "enum bevy::prelude::GizmoLineStyle")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#60)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalAmbientLight](../../prelude/struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalTransform](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#107)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalUiDebugOptions](../../prelude/struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalVolume](../../prelude/struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2448)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalZIndex](../../prelude/struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlobalsUniform](../../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Gltf](../../prelude/struct.Gltf.html "struct bevy::prelude::Gltf")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#266)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfExtras](../../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#143)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfLoader](../../gltf/struct.GltfLoader.html "struct bevy::gltf::GltfLoader")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/material.rs.html#13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfMaterial](../../gltf/struct.GltfMaterial.html "struct bevy::gltf::GltfMaterial")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#334)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfMaterialExtras](../../gltf/struct.GltfMaterialExtras.html "struct bevy::gltf::GltfMaterialExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#344)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfMaterialName](../../gltf/struct.GltfMaterialName.html "struct bevy::gltf::GltfMaterialName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#55)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfMesh](../../gltf/struct.GltfMesh.html "struct bevy::gltf::GltfMesh")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#309)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfMeshExtras](../../gltf/struct.GltfMeshExtras.html "struct bevy::gltf::GltfMeshExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#319)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfMeshName](../../gltf/struct.GltfMeshName.html "struct bevy::gltf::GltfMeshName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#97)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfNode](../../gltf/struct.GltfNode.html "struct bevy::gltf::GltfNode")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#163)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfPrimitive](../../gltf/struct.GltfPrimitive.html "struct bevy::gltf::GltfPrimitive")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#284)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfSceneExtras](../../gltf/struct.GltfSceneExtras.html "struct bevy::gltf::GltfSceneExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#294)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfSceneName](../../gltf/struct.GltfSceneName.html "struct bevy::gltf::GltfSceneName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#222)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GltfSkin](../../gltf/struct.GltfSkin.html "struct bevy::gltf::GltfSkin")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#32)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlyphAtlasInfo](../../text/struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#51)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GlyphAtlasLocation](../../text/struct.GlyphAtlasLocation.html "struct bevy::text::GlyphAtlasLocation")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GpuAtmosphereSettings](../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#457)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Gradient](../../prelude/enum.Gradient.html "enum bevy::prelude::Gradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1512)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GridAutoFlow](../../prelude/enum.GridAutoFlow.html "enum bevy::prelude::GridAutoFlow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2020)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GridPlacement](../../prelude/struct.GridPlacement.html "struct bevy::prelude::GridPlacement")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GridTrack](../../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1768)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GridTrackRepetition](../../prelude/enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/half_space.rs.html#36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HalfSpace](../../prelude/struct.HalfSpace.html "struct bevy::prelude::HalfSpace")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#60)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HashedStr](../../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#87)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Hdr](../../camera/struct.Hdr.html "struct bevy::camera::Hdr")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#11)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HdrTextureLoader](../../image/struct.HdrTextureLoader.html "struct bevy::image::HdrTextureLoader")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#133)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HitData](../../picking/backend/struct.HitData.html "struct bevy::picking::backend::HitData")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#336)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Hovered](../../picking/hover/struct.Hovered.html "struct bevy::picking::hover::Hovered")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#48-55)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I8Vec2](../../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#57-65)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I8Vec3](../../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#67-76)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I8Vec4](../../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#78-85)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I16Vec2](../../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#87-95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I16Vec3](../../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#97-106)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I16Vec4](../../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#108-115)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I64Vec2](../../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#117-125)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I64Vec3](../../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#127-136)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [I64Vec4](../../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/irect.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IRect](../../prelude/struct.IRect.html "struct bevy::prelude::IRect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#20-27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#28-36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IVec4](../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#436)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IgnoreScroll](../../prelude/struct.IgnoreScroll.html "struct bevy::prelude::IgnoreScroll")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#723)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageAddressMode](../../image/enum.ImageAddressMode.html "enum bevy::image::ImageAddressMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#776)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageCompareFunction](../../image/enum.ImageCompareFunction.html "enum bevy::image::ImageCompareFunction")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#757)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageFilterMode](../../image/enum.ImageFilterMode.html "enum bevy::image::ImageFilterMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageLoader](../../image/struct.ImageLoader.html "struct bevy::image::ImageLoader")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageNode](../../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#192)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageNodeSize](../../ui/widget/struct.ImageNodeSize.html "struct bevy::ui::widget::ImageNodeSize")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#983)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageRenderTarget](../../camera/struct.ImageRenderTarget.html "struct bevy::camera::ImageRenderTarget")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#673)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageSampler](../../image/enum.ImageSampler.html "enum bevy::image::ImageSampler")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#804)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageSamplerBorderColor](../../image/enum.ImageSamplerBorderColor.html "enum bevy::image::ImageSamplerBorderColor")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#830)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageSamplerDescriptor](../../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ImageSaver](../../image/struct.ImageSaver.html "struct bevy::image::ImageSaver")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#247)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Ime](../../prelude/enum.Ime.html "enum bevy::prelude::Ime")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/index.rs.html#83)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Indices](../../mesh/enum.Indices.html "enum bevy::mesh::Indices")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#89)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InfiniteGrid](../../dev_tools/infinite_grid/struct.InfiniteGrid.html "struct bevy::dev_tools::infinite_grid::InfiniteGrid")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#105)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InfiniteGridSettings](../../dev_tools/infinite_grid/struct.InfiniteGridSettings.html "struct bevy::dev_tools::infinite_grid::InfiniteGridSettings")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#180)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InfinitePlane3d](../../prelude/struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InheritableFont](../../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#106)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InheritableThemeTextColor](../../feathers/theme/struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#162)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#875)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InlineDirection](../../prelude/enum.InlineDirection.html "enum bevy::prelude::InlineDirection")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#100)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InputFocus](../../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#173)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InputFocusVisible](../../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#346)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Insert](../../prelude/struct.Insert.html "struct bevy::prelude::Insert")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#50)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InstanceId](../../world_serialization/struct.InstanceId.html "struct bevy::world_serialization::InstanceId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/time.rs.html#3-5)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Instant](../../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#44)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InteractionDisabled](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1098)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InternalWindowState](../../window/struct.InternalWindowState.html "struct bevy::window::InternalWindowState")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#634)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InterpolationColorSpace](../../prelude/enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/interval.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IrradianceVolume](../../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2978)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IsDefaultUiCamera](../../prelude/struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/resource.rs.html#121)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IsResource](../../ecs/resource/struct.IsResource.html "struct bevy::ecs::resource::IsResource")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#90)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#368)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [JointAabb](../../mesh/skinning/struct.JointAabb.html "struct bevy::mesh::skinning::JointAabb")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#336)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [JointIndex](../../mesh/skinning/struct.JointIndex.html "struct bevy::mesh::skinning::JointIndex")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#346)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [JumpAt](../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#230)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Justify](../../prelude/enum.Justify.html "enum bevy::prelude::Justify")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1102)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [JustifyContent](../../prelude/enum.JustifyContent.html "enum bevy::prelude::JustifyContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#938)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [JustifyItems](../../prelude/enum.JustifyItems.html "enum bevy::prelude::JustifyItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1018)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [JustifySelf](../../prelude/enum.JustifySelf.html "enum bevy::prelude::JustifySelf")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#804)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Key](../../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#262)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [KeyCode](../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#152)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [KeyboardFocusLost](../../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#103)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [KeyboardInput](../../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/label.rs.html#5)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Label](../../prelude/struct.Label.html "struct bevy::prelude::Label")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2903)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LayoutConfig](../../prelude/struct.LayoutConfig.html "struct bevy::prelude::LayoutConfig")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#273)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Leave](../../prelude/struct.Leave.html "struct bevy::prelude::Leave")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1039)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LetterSpacing](../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#151)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LightGizmoColor](../../prelude/enum.LightGizmoColor.html "enum bevy::prelude::LightGizmoColor")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#166)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LightGizmoConfigGroup](../../prelude/struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#71)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LightProbe](../../prelude/struct.LightProbe.html "struct bevy::prelude::LightProbe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Lightmap](../../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1234)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Line2d](../../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#357)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Line3d](../../prelude/struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1112)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LineBreak](../../prelude/enum.LineBreak.html "enum bevy::prelude::LineBreak")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LineGizmoEntities](../../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1011)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LineHeight](../../text/enum.LineHeight.html "enum bevy::text::LineHeight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#227)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LinearGradient](../../prelude/struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#39)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ListItem](../../ui_widgets/struct.ListItem.html "struct bevy::ui_widgets::ListItem")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/folder.rs.html#11)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LoadedFolder](../../asset/struct.LoadedFolder.html "struct bevy::asset::LoadedFolder")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#97)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LoadedUntypedAsset](../../asset/struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#210)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::picking::pointer::[Location](../../picking/pointer/struct.Location.html "struct bevy::picking::pointer::Location")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#158)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#142)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MainPassResolutionOverride](../../camera/struct.MainPassResolutionOverride.html "struct bevy::camera::MainPassResolutionOverride")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#156)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ManageAccessibilityUpdates](../../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#976)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ManualTextureViewHandle](../../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#343-350)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#351-359)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#360-368)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#276)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MaterialBindGroupIndex](../../pbr/struct.MaterialBindGroupIndex.html "struct bevy::pbr::MaterialBindGroupIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#294)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MaterialBindGroupSlot](../../pbr/struct.MaterialBindGroupSlot.html "struct bevy::pbr::MaterialBindGroupSlot")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#259)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MaterialBindingId](../../pbr/struct.MaterialBindingId.html "struct bevy::pbr::MaterialBindingId")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1569)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MaxTrackSizingFunction](../../prelude/enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#61)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuAction](../../ui_widgets/enum.MenuAction.html "enum bevy::ui_widgets::MenuAction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#414)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuButton](../../ui_widgets/struct.MenuButton.html "struct bevy::ui_widgets::MenuButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#79)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuEvent](../../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#139)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuFocusState](../../ui_widgets/enum.MenuFocusState.html "enum bevy::ui_widgets::MenuFocusState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#133)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuItem](../../ui_widgets/struct.MenuItem.html "struct bevy::ui_widgets::MenuItem")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#91)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuLayout](../../ui_widgets/enum.MenuLayout.html "enum bevy::ui_widgets::MenuLayout")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#123)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MenuPopup](../../ui_widgets/struct.MenuPopup.html "struct bevy::ui_widgets::MenuPopup")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mesh2d](../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mesh2dWireframe](../../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mesh3d](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mesh3dWireframe](../../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#118)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshMorphWeights](../../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#33)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshPickingCamera](../../prelude/struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#38)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshPickingSettings](../../prelude/struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#154)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshTag](../../mesh/struct.MeshTag.html "struct bevy::mesh::MeshTag")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#40)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshletMesh](../../pbr/experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshletMesh3d](../../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#195)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshletMeshLoader](../../pbr/experimental/meshlet/struct.MeshletMeshLoader.html "struct bevy::pbr::experimental::meshlet::MeshletMeshLoader")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#148)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshletMeshSaver](../../pbr/experimental/meshlet/struct.MeshletMeshSaver.html "struct bevy::pbr::experimental::meshlet::MeshletMeshSaver")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1540)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MinTrackSizingFunction](../../prelude/enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#805)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MipBias](../../render/camera/struct.MipBias.html "struct bevy::render::camera::MipBias")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#24)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Monitor](../../window/struct.Monitor.html "struct bevy::window::Monitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1147)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MonitorSelection](../../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MorphAttributes](../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#79)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MorphWeights](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#73)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MotionBlur](../../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#76)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MotionVectorPrepass](../../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#64)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MouseButton](../../prelude/enum.MouseButton.html "enum bevy::prelude::MouseButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MouseButtonInput](../../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#99)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MouseMotion](../../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#121)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MouseScrollUnit](../../input/mouse/enum.MouseScrollUnit.html "enum bevy::input::mouse::MouseScrollUnit")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#160)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MouseWheel](../../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#323)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Move](../../prelude/struct.Move.html "struct bevy::prelude::Move")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#231)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#29)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MsaaWriteback](../../prelude/enum.MsaaWriteback.html "enum bevy::prelude::MsaaWriteback")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Name](../../prelude/struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#758)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NativeKey](../../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#220)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NativeKeyCode](../../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#107)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NavAction](../../input_focus/tab_navigation/enum.NavAction.html "enum bevy::input_focus::tab_navigation::NavAction")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NavNeighbor](../../input_focus/directional_navigation/enum.NavNeighbor.html "enum bevy::input_focus::directional_navigation::NavNeighbor")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#187)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NavNeighbors](../../input_focus/directional_navigation/struct.NavNeighbors.html "struct bevy::input_focus::directional_navigation::NavNeighbors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#550)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NoAutoAabb](../../camera/visibility/struct.NoAutoAabb.html "struct bevy::camera::visibility::NoAutoAabb")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#55)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NoBackgroundMotionVectors](../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#316)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NoFrustumCulling](../../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#3)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NoOpHash](../../platform/hash/struct.NoOpHash.html "struct bevy::platform::hash::NoOpHash")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#868)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NoWireframe](../../pbr/wireframe/struct.NoWireframe.html "struct bevy::pbr::wireframe::NoWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#418)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NoWireframe2d](../../sprite_render/struct.NoWireframe2d.html "struct bevy::sprite_render::NoWireframe2d")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#471)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Node](../../prelude/struct.Node.html "struct bevy::prelude::Node")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#156)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NodeImageMode](../../prelude/enum.NodeImageMode.html "enum bevy::prelude::NodeImageMode")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#15-22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NonNilUuid](../../asset/uuid/struct.NonNilUuid.html "struct bevy::asset::uuid::NonNilUuid")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#68)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NormalPrepass](../../core_pipeline/prepass/struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#940)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NormalizedRenderTarget](../../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#105)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NormalizedWindowRef](../../window/struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#256)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NotShadowCaster](../../light/struct.NotShadowCaster.html "struct bevy::light::NotShadowCaster")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#264)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NotShadowReceiver](../../light/struct.NotShadowReceiver.html "struct bevy::light::NotShadowReceiver")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#131)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NumberFormat](../../feathers/controls/enum.NumberFormat.html "enum bevy::feathers::controls::NumberFormat")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#146)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NumberInputValue](../../feathers/controls/enum.NumberInputValue.html "enum bevy::feathers::controls::NumberInputValue")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#501)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ObservedBy](../../ecs/observer/struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OcclusionCulling](../../render/occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#298)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OffsetAccess](../struct.OffsetAccess.html "struct bevy::reflect::OffsetAccess")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/opaque.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OpaqueRendererMethod](../../material/enum.OpaqueRendererMethod.html "enum bevy::material::OpaqueRendererMethod")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#578)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OrthographicProjection](../../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#240)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Out](../../prelude/struct.Out.html "struct bevy::prelude::Out")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2456)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OuterColor](../../prelude/struct.OuterColor.html "struct bevy::prelude::OuterColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2315)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Outline](../../prelude/struct.Outline.html "struct bevy::prelude::Outline")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#190)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Over](../../prelude/struct.Over.html "struct bevy::prelude::Over")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Overflow](../../prelude/struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1347)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OverflowAxis](../../prelude/enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1381)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OverflowClipMargin](../../prelude/struct.OverflowClipMargin.html "struct bevy::prelude::OverflowClipMargin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2416)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OverrideClip](../../prelude/struct.OverrideClip.html "struct bevy::prelude::OverrideClip")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#47)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [OverrideCursor](../../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#84)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PanGesture](../../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#410)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ParallaxCorrection](../../light/enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/parallax.rs.html#14)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ParallaxMappingMethod](../../prelude/enum.ParallaxMappingMethod.html "enum bevy::prelude::ParallaxMappingMethod")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#367)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ParsedPath](../struct.ParsedPath.html "struct bevy::reflect::ParsedPath")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#5)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PassHash](../../platform/hash/struct.PassHash.html "struct bevy::platform::hash::PassHash")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/pathtracer/mod.rs.html#63)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Pathtracer](../../solari/pathtracer/struct.Pathtracer.html "struct bevy::solari::pathtracer::Pathtracer")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#281)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PerspectiveProjection](../../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#196)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#224)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PickingInteraction](../../picking/hover/enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#296)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PickingSettings](../../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#25)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PinchGesture](../../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/pitch.rs.html#10)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Pitch](../../prelude/struct.Pitch.html "struct bevy::prelude::Pitch")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1192)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Plane2d](../../prelude/struct.Plane2d.html "struct bevy::prelude::Plane2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#96)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Plane3d](../../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PlaneMeshBuilder](../../mesh/struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#9)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PlaybackMode](../../audio/enum.PlaybackMode.html "enum bevy::audio::PlaybackMode")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#33)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PlaybackSettings](../../prelude/struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#38)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointLight](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#177)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointLightShadowMap](../../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointLightTexture](../../light/struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#248)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerAction](../../picking/pointer/enum.PointerAction.html "enum bevy::picking::pointer::PointerAction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerButton](../../prelude/enum.PointerButton.html "enum bevy::prelude::PointerButton")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#91)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerHits](../../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerId](../../picking/pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#278)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerInput](../../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#42)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerInputSettings](../../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#71)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerInteraction](../../picking/pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#178)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerLocation](../../picking/pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#114)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PointerPress](../../picking/pointer/struct.PointerPress.html "struct bevy::picking::pointer::PointerPress")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1894)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Polygon](../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1566)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#701)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Polyline2dMeshBuilder](../../mesh/struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#624)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Polyline3d](../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#84)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Popover](../../ui_widgets/popover/struct.Popover.html "struct bevy::ui_widgets::popover::Popover")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#52)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PopoverAlign](../../ui_widgets/popover/enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#69)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PopoverPlacement](../../ui_widgets/popover/struct.PopoverPlacement.html "struct bevy::ui_widgets::popover::PopoverPlacement")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PopoverSide](../../ui_widgets/popover/enum.PopoverSide.html "enum bevy::ui_widgets::popover::PopoverSide")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1453)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PositionType](../../prelude/enum.PositionType.html "enum bevy::prelude::PositionType")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PositionedGlyph](../../text/struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PreeditCursor](../../text/struct.PreeditCursor.html "struct bevy::text::PreeditCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1214)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PresentMode](../../window/enum.PresentMode.html "enum bevy::window::PresentMode")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#286)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Press](../../prelude/struct.Press.html "struct bevy::prelude::Press")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#149)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PressDirection](../../picking/pointer/enum.PressDirection.html "enum bevy::picking::pointer::PressDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#44)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Pressed](../../ui/struct.Pressed.html "struct bevy::ui::Pressed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PrimaryMonitor](../../window/struct.PrimaryMonitor.html "struct bevy::window::PrimaryMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PrimaryWindow](../../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#214)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Projection](../../prelude/enum.Projection.html "enum bevy::prelude::Projection")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#361)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RadialGradient](../../prelude/struct.RadialGradient.html "struct bevy::prelude::RadialGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#558)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RadialGradientShape](../../prelude/enum.RadialGradientShape.html "enum bevy::prelude::RadialGradientShape")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#58)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RadioButton](../../ui_widgets/struct.RadioButton.html "struct bevy::ui_widgets::RadioButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#40)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RadioGroup](../../ui_widgets/struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#5)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::platform::hash::[RandomState](../../platform/hash/struct.RandomState.html "struct bevy::platform::hash::RandomState")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#118)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RawGamepadAxisChangedEvent](../../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#86)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RawGamepadButtonChangedEvent](../../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#65)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RawGamepadEvent](../../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Ray2d](../../prelude/struct.Ray2d.html "struct bevy::prelude::Ray2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#74)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Ray3d](../../prelude/struct.Ray3d.html "struct bevy::prelude::Ray3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#12)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RayCast2d](../../math/bounding/struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#12)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RayCast3d](../../math/bounding/struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#106)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RayCastBackfaces](../../prelude/struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RayCastVisibility](../../prelude/enum.RayCastVisibility.html "enum bevy::prelude::RayCastVisibility")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#245)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RayId](../../picking/backend/ray/struct.RayId.html "struct bevy::picking::backend::ray::RayId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RayMeshHit](../../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RaytracingMesh3d](../../solari/scene/struct.RaytracingMesh3d.html "struct bevy::solari::scene::RaytracingMesh3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ReadbackComplete](../../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#44)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Real](../../prelude/struct.Real.html "struct bevy::prelude::Real")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/rect.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Rect](../../prelude/struct.Rect.html "struct bevy::prelude::Rect")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/rect_light.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RectLight](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1801)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Rectangle](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1041)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RectangleMeshBuilder](../../mesh/struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2036)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RegularPolygon](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#482)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RegularPolygonMeshBuilder](../../mesh/struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#78)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#298)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Release](../../prelude/struct.Release.html "struct bevy::prelude::Release")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#376)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Remove](../../prelude/struct.Remove.html "struct bevy::prelude::Remove")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#399)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RemovedComponentEntity](../../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/render_asset.rs.html#29)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#319)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderDebugMode](../../dev_tools/render_debug/enum.RenderDebugMode.html "enum bevy::dev_tools::render_debug::RenderDebugMode")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#262)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderDebugOverlayEvent](../../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#129)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/render_layers.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderLayers](../../camera/visibility/struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#63)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderShadowMapVisibleEntities](../../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderTarget](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#196)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderTargetInfo](../../camera/struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#87)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RenderVisibleEntitiesClass](../../render/view/struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::RenderVisibleEntitiesClass")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#469)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RepeatAnimation](../../animation/enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RepeatedGridTrack](../../prelude/struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#53)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RequestRedraw](../../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2802)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ResolvedBorderRadius](../../prelude/struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1055)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Rhombus](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#878)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RhombusMeshBuilder](../../mesh/struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#120)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RootNonCameraView](../../core_pipeline/schedule/struct.RootNonCameraView.html "struct bevy::core_pipeline::schedule::RootNonCameraView")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#44)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Rot2](../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#47)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RotationGesture](../../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#501)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RunGeometry](../../text/struct.RunGeometry.html "struct bevy::text::RunGeometry")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#521)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScalingMode](../../camera/enum.ScalingMode.html "enum bevy::camera::ScalingMode")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#132)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScatteringMedium](../../light/atmosphere/struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SceneComponentInfo](../../scene/struct.SceneComponentInfo.html "struct bevy::scene::SceneComponentInfo")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SceneListPatch](../../scene/struct.SceneListPatch.html "struct bevy::scene::SceneListPatch")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#19)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScenePatch](../../scene/struct.ScenePatch.html "struct bevy::scene::ScenePatch")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SchemaTypesMetadata](../../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1478)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenEdge](../../window/enum.ScreenEdge.html "enum bevy::window::ScreenEdge")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#135)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenSpaceAmbientOcclusionQualityLevel](../../pbr/enum.ScreenSpaceAmbientOcclusionQualityLevel.html "enum bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#78)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenSpaceReflections](../../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#110)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenSpaceTransmissionQuality](../../pbr/enum.ScreenSpaceTransmissionQuality.html "enum bevy::pbr::ScreenSpaceTransmissionQuality")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#78)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Screenshot](../../render/view/window/screenshot/struct.Screenshot.html "struct bevy::render::view::window::screenshot::Screenshot")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScreenshotCaptured](../../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#455)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Scroll](../../prelude/struct.Scroll.html "struct bevy::prelude::Scroll")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollarea.rs.html#16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScrollArea](../../ui_widgets/struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#417)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScrollPosition](../../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Scrollbar](../../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#130)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScrollbarDragState](../../ui_widgets/struct.ScrollbarDragState.html "struct bevy::ui_widgets::ScrollbarDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#100)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ScrollbarThumb](../../ui_widgets/struct.ScrollbarThumb.html "struct bevy::ui_widgets::ScrollbarThumb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1254)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Segment2d](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#376)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Segment3d](../../prelude/struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#406)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SelectAllOnFocus](../../ui_widgets/struct.SelectAllOnFocus.html "struct bevy::ui_widgets::SelectAllOnFocus")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#29)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Sensitivity](../../anti_alias/fxaa/enum.Sensitivity.html "enum bevy::anti_alias::fxaa::Sensitivity")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SetChecked](../../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SetSliderValue](../../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Shader](../../prelude/struct.Shader.html "struct bevy::prelude::Shader")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShaderBuffer](../../render/storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#302)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShaderLoader](../../shader/struct.ShaderLoader.html "struct bevy::shader::ShaderLoader")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#283)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShadowFilteringMethod](../../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#854)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShadowLodOrigin](../../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2868)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShadowStyle](../../prelude/struct.ShadowStyle.html "struct bevy::prelude::ShadowStyle")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#61)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShowAabbGizmo](../../prelude/struct.ShowAabbGizmo.html "struct bevy::prelude::ShowAabbGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#96)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShowFrustumGizmo](../../prelude/struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#210)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShowLightGizmo](../../prelude/struct.ShowLightGizmo.html "struct bevy::prelude::ShowLightGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#76)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ShowSkinnedMeshBoundsGizmo](../../prelude/struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::prelude::ShowSkinnedMeshBoundsGizmo")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SimplifiedMesh](../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SkinnedMesh](../../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#88)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SkinnedMeshBounds](../../mesh/skinning/struct.SkinnedMeshBounds.html "struct bevy::mesh::skinning::SkinnedMeshBounds")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#52)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SkinnedMeshBoundsGizmoConfigGroup](../../prelude/struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SkinnedMeshInverseBindposes](../../mesh/skinning/struct.SkinnedMeshInverseBindposes.html "struct bevy::mesh::skinning::SkinnedMeshInverseBindposes")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Skybox](../../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliceScaleMode](../../prelude/enum.SliceScaleMode.html "enum bevy::prelude::SliceScaleMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#103)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Slider](../../ui_widgets/struct.Slider.html "struct bevy::ui_widgets::Slider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#147)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderBaseColor](../../feathers/controls/struct.SliderBaseColor.html "struct bevy::feathers::controls::SliderBaseColor")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#245)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderDragState](../../ui_widgets/struct.SliderDragState.html "struct bevy::ui_widgets::SliderDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#35)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderOrientation](../../ui_widgets/enum.SliderOrientation.html "enum bevy::ui_widgets::SliderOrientation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#233)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderPrecision](../../ui_widgets/struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#127)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderRange](../../ui_widgets/struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#214)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderStep](../../ui_widgets/struct.SliderStep.html "struct bevy::ui_widgets::SliderStep")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderThumb](../../ui_widgets/struct.SliderThumb.html "struct bevy::ui_widgets::SliderThumb")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#120)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderValue](../../ui_widgets/struct.SliderValue.html "struct bevy::ui_widgets::SliderValue")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#683)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SliderValueChange](../../ui_widgets/enum.SliderValueChange.html "enum bevy::ui_widgets::SliderValueChange")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#106)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SmaaPreset](../../anti_alias/smaa/enum.SmaaPreset.html "enum bevy::anti_alias::smaa::SmaaPreset")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/realtime/mod.rs.html#85)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SolariLighting](../../solari/realtime/struct.SolariLighting.html "struct bevy::solari::realtime::SolariLighting")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#170)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpatialListener](../../prelude/struct.SpatialListener.html "struct bevy::prelude::SpatialListener")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#203)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpatialScale](../../audio/struct.SpatialScale.html "struct bevy::audio::SpatialScale")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::prelude::[Sphere](../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#196)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::camera::primitives::[Sphere](../../camera/primitives/struct.Sphere.html "struct bevy::camera::primitives::Sphere")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SphereKind](../../mesh/enum.SphereKind.html "enum bevy::mesh::SphereKind")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#51)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SphereMeshBuilder](../../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpotLight](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpotLightTexture](../../light/struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Sprite](../../prelude/struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#178)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpriteAlphaMode](../../sprite/enum.SpriteAlphaMode.html "enum bevy::sprite::SpriteAlphaMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#166)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpriteImageMode](../../prelude/enum.SpriteImageMode.html "enum bevy::prelude::SpriteImageMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpriteMaterial](../../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpriteMesh](../../prelude/struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpritePickingCamera](../../prelude/struct.SpritePickingCamera.html "struct bevy::prelude::SpritePickingCamera")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#39)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpritePickingMode](../../prelude/enum.SpritePickingMode.html "enum bevy::prelude::SpritePickingMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#51)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpritePickingSettings](../../prelude/struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#214)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SpriteScalingMode](../../prelude/enum.SpriteScalingMode.html "enum bevy::prelude::SpriteScalingMode")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#20)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [StandardMaterial](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#88)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [StaticTransformOptimizations](../../prelude/enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/stopwatch.rs.html#31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Stopwatch](../../time/struct.Stopwatch.html "struct bevy::time::Stopwatch")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1132)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Strikethrough](../../prelude/struct.Strikethrough.html "struct bevy::prelude::Strikethrough")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1137)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [StrikethroughColor](../../prelude/struct.StrikethroughColor.html "struct bevy::prelude::StrikethroughColor")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [String](../../prelude/struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#83)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [StrongHandle](../../asset/struct.StrongHandle.html "struct bevy::asset::StrongHandle")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#174)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SubCameraView](../../camera/struct.SubCameraView.html "struct bevy::camera::SubCameraView")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#121)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SyncToRenderWorld](../../render/sync_world/struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/system_cursor.rs.html#89)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SystemCursorIcon](../../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#69)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TabGroup](../../input_focus/tab_navigation/struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#60)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TabIndex](../../input_focus/tab_navigation/struct.TabIndex.html "struct bevy::input_focus::tab_navigation::TabIndex")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#111)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TemporalAntiAliasing](../../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#780)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TemporalJitter](../../render/camera/struct.TemporalJitter.html "struct bevy::render::camera::TemporalJitter")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#190)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TemporaryRenderEntity](../../render/sync_world/struct.TemporaryRenderEntity.html "struct bevy::render::sync_world::TemporaryRenderEntity")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1433)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Tetrahedron](../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TetrahedronMeshBuilder](../../mesh/struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#97)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Text](../../prelude/struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#85)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Text2d](../../prelude/struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#141)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Text2dShadow](../../sprite/struct.Text2dShadow.html "struct bevy::sprite::Text2dShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1088)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextBackgroundColor](../../prelude/struct.TextBackgroundColor.html "struct bevy::prelude::TextBackgroundColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextBounds](../../text/struct.TextBounds.html "struct bevy::text::TextBounds")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1064)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextColor](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#25)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextEdit](../../text/enum.TextEdit.html "enum bevy::text::TextEdit")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextEntity](../../text/struct.TextEntity.html "struct bevy::text::TextEntity")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextFont](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#130)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextLayout](../../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#461)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#32)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextNodeFlags](../../ui/struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#32)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextScroll](../../ui/widget/struct.TextScroll.html "struct bevy::ui::widget::TextScroll")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#144)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextShadow](../../prelude/struct.TextShadow.html "struct bevy::prelude::TextShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#211)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextureAtlas](../../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextureAtlasLayout](../../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#3-10)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextureSlicer](../../prelude/struct.TextureSlicer.html "struct bevy::prelude::TextureSlicer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#90)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThemeBackgroundColor](../../feathers/theme/struct.ThemeBackgroundColor.html "struct bevy::feathers::theme::ThemeBackgroundColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#99)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThemeBorderColor](../../feathers/theme/struct.ThemeBorderColor.html "struct bevy::feathers::theme::ThemeBorderColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#50)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThemeProps](../../feathers/theme/struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#118)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThemeTextColor](../../feathers/theme/struct.ThemeTextColor.html "struct bevy::feathers::theme::ThemeTextColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThemeToken](../../feathers/theme/struct.ThemeToken.html "struct bevy::feathers::theme::ThemeToken")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#125)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThemedText](../../feathers/theme/struct.ThemedText.html "struct bevy::feathers::theme::ThemedText")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#298)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThreadedAnimationGraph](../../prelude/struct.ThreadedAnimationGraph.html "struct bevy::prelude::ThreadedAnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#288)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ThreadedAnimationGraphs](../../prelude/struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#15)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Tick](../../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#94)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TileData](../../sprite_render/struct.TileData.html "struct bevy::sprite_render::TileData")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tile_orientation.rs.html#37)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TileOrientation](../../sprite_render/enum.TileOrientation.html "enum bevy::sprite_render::TileOrientation")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TilemapChunk](../../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#27)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TilemapChunkMaterial](../../sprite_render/struct.TilemapChunkMaterial.html "struct bevy::sprite_render::TilemapChunkMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#46)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TilemapChunkMeshCache](../../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#130)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TilemapChunkTileData](../../sprite_render/struct.TilemapChunkTileData.html "struct bevy::sprite_render::TilemapChunkTileData")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Timer](../../prelude/struct.Timer.html "struct bevy::prelude::Timer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#492)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TimerMode](../../prelude/enum.TimerMode.html "enum bevy::prelude::TimerMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ToggleChecked](../../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1124)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Torus](../../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#8)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TorusMeshBuilder](../../mesh/struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#45)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TouchInput](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#123)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TouchPhase](../../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#61)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TrackClick](../../ui_widgets/enum.TrackClick.html "enum bevy::ui_widgets::TrackClick")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Transform](../../prelude/struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#123)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoAxis](../../prelude/enum.TransformGizmoAxis.html "enum bevy::prelude::TransformGizmoAxis")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoCamera](../../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#85)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoFocus](../../prelude/struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#101)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoMode](../../prelude/enum.TransformGizmoMode.html "enum bevy::prelude::TransformGizmoMode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#136)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoSettings](../../prelude/struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoSpace](../../prelude/enum.TransformGizmoSpace.html "enum bevy::prelude::TransformGizmoSpace")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#179)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformGizmoState](../../prelude/struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#666)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransformTreeChanged](../../prelude/struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#274)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TransmittedShadowReceiver](../../light/struct.TransmittedShadowReceiver.html "struct bevy::light::TransmittedShadowReceiver")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1627)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Triangle2d](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#964)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Triangle2dMeshBuilder](../../mesh/struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1236)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Triangle3d](../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#7)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Triangle3dMeshBuilder](../../mesh/struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#166-173)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U8Vec2](../../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#174-182)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U8Vec3](../../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#183-192)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U8Vec4](../../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#194-201)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U16Vec2](../../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#202-210)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U16Vec3](../../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#211-220)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U16Vec4](../../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#222-229)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U64Vec2](../../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#230-238)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U64Vec3](../../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#239-248)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [U64Vec4](../../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/urect.rs.html#21)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [URect](../../prelude/struct.URect.html "struct bevy::prelude::URect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#146-154)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiAntiAlias](../../prelude/enum.UiAntiAlias.html "enum bevy::prelude::UiAntiAlias")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#39)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiDebugOptions](../../prelude/struct.UiDebugOptions.html "struct bevy::prelude::UiDebugOptions")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#199)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#40)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiPickingCamera](../../prelude/struct.UiPickingCamera.html "struct bevy::prelude::UiPickingCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#45)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiPickingSettings](../../prelude/struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#993)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiPosition](../../prelude/struct.UiPosition.html "struct bevy::prelude::UiPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiRect](../../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#124)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiScale](../../prelude/struct.UiScale.html "struct bevy::prelude::UiScale")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#25)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiStack](../../ui/struct.UiStack.html "struct bevy::ui::UiStack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2936)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiTargetCamera](../../prelude/struct.UiTargetCamera.html "struct bevy::prelude::UiTargetCamera")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#59)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiTheme](../../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#122)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UiTransform](../../prelude/struct.UiTransform.html "struct bevy::prelude::UiTransform")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1154)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Underline](../../prelude/struct.Underline.html "struct bevy::prelude::Underline")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1159)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UnderlineColor](../../prelude/struct.UnderlineColor.html "struct bevy::prelude::UnderlineColor")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#167)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UntypedAssetId](../../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#474)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UntypedHandle](../../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UpdateNumberInput](../../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#4-13)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Uuid](../../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2531)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UvChannel](../../mesh/enum.UvChannel.html "enum bevy::mesh::UvChannel")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#15)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Val2](../../prelude/struct.Val2.html "struct bevy::prelude::Val2")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#80)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VariableCurve](../../prelude/struct.VariableCurve.html "struct bevy::prelude::VariableCurve")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#267-275)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#72)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VideoMode](../../window/struct.VideoMode.html "struct bevy::window::VideoMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1175)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VideoModeSelection](../../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/view_frustum.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ViewFrustum](../../prelude/struct.ViewFrustum.html "struct bevy::prelude::ViewFrustum")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#224)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ViewVisibility](../../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#60)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Viewport](../../camera/struct.Viewport.html "struct bevy::camera::Viewport")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#36)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ViewportNode](../../prelude/struct.ViewportNode.html "struct bevy::prelude::ViewportNode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#28)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vignette](../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#74)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Virtual](../../prelude/struct.Virtual.html "struct bevy::prelude::Virtual")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#80)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Visibility](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#208)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VisibilityClass](../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#78)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VisibilityRange](../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#342)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VisibleEntities](../../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#408)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VisibleMeshEntities](../../camera/visibility/struct.VisibleMeshEntities.html "struct bevy::camera::visibility::VisibleMeshEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1435)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VisualBox](../../prelude/enum.VisualBox.html "enum bevy::prelude::VisualBox")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#34)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Volume](../../audio/enum.Volume.html "enum bevy::audio::Volume")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#23)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VolumetricFog](../../light/struct.VolumetricFog.html "struct bevy::light::VolumetricFog")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#14)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VolumetricLight](../../light/struct.VolumetricLight.html "struct bevy::light::VolumetricLight")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#64)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WeightsCurveSample](../../prelude/struct.WeightsCurveSample.html "struct bevy::prelude::WeightsCurveSample")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#155)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#357)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowBackendScaleFactorChanged](../../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#95)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowCloseRequested](../../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#113)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowClosed](../../window/struct.WindowClosed.html "struct bevy::window::WindowClosed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#134)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowClosing](../../window/struct.WindowClosing.html "struct bevy::window::WindowClosing")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#69)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowCreated](../../window/struct.WindowCreated.html "struct bevy::window::WindowCreated")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#154)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowDestroyed](../../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#496)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowEvent](../../window/enum.WindowEvent.html "enum bevy::window::WindowEvent")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#292)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowFocused](../../window/struct.WindowFocused.html "struct bevy::window::WindowFocused")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1382)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowLevel](../../window/enum.WindowLevel.html "enum bevy::window::WindowLevel")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1334)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowMode](../../window/enum.WindowMode.html "enum bevy::window::WindowMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#412)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowMoved](../../prelude/struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#319)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowOccluded](../../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#796)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowPosition](../../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#64)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowRef](../../window/enum.WindowRef.html "enum bevy::window::WindowRef")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#675)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowResizeConstraints](../../prelude/struct.WindowResizeConstraints.html "struct bevy::prelude::WindowResizeConstraints")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowResized](../../window/struct.WindowResized.html "struct bevy::window::WindowResized")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#889)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowResolution](../../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#338)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowScaleFactorChanged](../../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1406)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowTheme](../../window/enum.WindowTheme.html "enum bevy::window::WindowTheme")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#434)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WindowThemeChanged](../../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#175)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WinitUserEvent](../../winit/enum.WinitUserEvent.html "enum bevy::winit::WinitUserEvent")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#199)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Wireframe](../../pbr/wireframe/struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#163)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Wireframe2d](../../sprite_render/struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#403)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Wireframe2dColor](../../sprite_render/struct.Wireframe2dColor.html "struct bevy::sprite_render::Wireframe2dColor")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Wireframe2dConfig](../../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Wireframe2dMaterial](../../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#843)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WireframeColor](../../pbr/wireframe/struct.WireframeColor.html "struct bevy::pbr::wireframe::WireframeColor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WireframeConfig](../../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#852)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WireframeLineWidth](../../pbr/wireframe/struct.WireframeLineWidth.html "struct bevy::pbr::wireframe::WireframeLineWidth")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WireframeMaterial](../../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#873)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WireframeTopology](../../pbr/wireframe/enum.WireframeTopology.html "enum bevy::pbr::wireframe::WireframeTopology")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset.rs.html#22)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WorldAsset](../../prelude/struct.WorldAsset.html "struct bevy::prelude::WorldAsset")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WorldAssetLoader](../../world_serialization/struct.WorldAssetLoader.html "struct bevy::world_serialization::WorldAssetLoader")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WorldAssetRoot](../../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WorldInstanceReady](../../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#17)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2438)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ZIndex](../../prelude/struct.ZIndex.html "struct bevy::prelude::ZIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#517)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#604)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/access.rs.html#16)

### impl<'a> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Access](../enum.Access.html "enum bevy::reflect::Access")<'a>

where [Access](../enum.Access.html "enum bevy::reflect::Access")<'a>: 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#54)

### impl<'a> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AssetPath](../../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>

where [AssetPath](../../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>: 'static,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/foldhash.rs.html#3)

### impl<'a> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::platform::hash::[DefaultHasher](../../platform/hash/struct.DefaultHasher.html "struct bevy::platform::hash::DefaultHasher")<'a>

where [FoldHasher](../../platform/hash/struct.DefaultHasher.html "struct bevy::platform::hash::DefaultHasher")<'a>: 'static,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/dynamic_function.rs.html#445)

### impl<'env> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DynamicFunction](../func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'env>

where [DynamicFunction](../func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'env>: 'static,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#303)

### impl<A> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimatableCurveEvaluator](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>

where A: [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AnimatableCurveEvaluator](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#49)

### impl<A> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AssetEvent](../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetEvent](../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#21)

### impl<A> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#132)

### impl<A> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#403)

### impl<A> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IdentityAssetTransformer](../../asset/transformer/struct.IdentityAssetTransformer.html "struct bevy::asset::transformer::IdentityAssetTransformer")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IdentityAssetTransformer](../../asset/transformer/struct.IdentityAssetTransformer.html "struct bevy::asset::transformer::IdentityAssetTransformer")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#172)

### impl<B, E> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ExtendedMaterial](../../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../../prelude/trait.Material.html "trait bevy::prelude::Material") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), E: [MaterialExtension](../../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ExtendedMaterial](../../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#96)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Inherited](../../app/struct.Inherited.html "struct bevy::app::Inherited")<C>

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Inherited](../../app/struct.Inherited.html "struct bevy::app::Inherited")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#70)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Propagate](../../app/struct.Propagate.html "struct bevy::app::Propagate")<C>

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Propagate](../../app/struct.Propagate.html "struct bevy::app::Propagate")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#78)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PropagateOver](../../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>

where [PropagateOver](../../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#83)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PropagateStop](../../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>

where [PropagateStop](../../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#152)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SampleDerivativeWrapper](../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

where [SampleDerivativeWrapper](../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#185)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SampleTwoDerivativesWrapper](../../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>

where [SampleTwoDerivativesWrapper](../../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#19)

### impl<C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WeightsCurve](../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>

where [WeightsCurve](../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#284)

### impl<Config, Clear> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GizmoBuffer](../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where [GizmoBuffer](../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Pointer](../../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_map.rs.html#13)

### impl<K, V, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::platform::collections::[HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>

where [HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), K: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#65)

### impl<L, T, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LoadTransformAndSave](../../asset/processor/struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave")<L, T, S>

where L: [AssetLoader](../../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [AssetTransformer](../../asset/transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer")<AssetInput = <L as [AssetLoader](../../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader")\>::[Asset](../../asset/trait.AssetLoader.html#associatedtype.Asset "type bevy::asset::AssetLoader::Asset")\> + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [AssetSaver](../../asset/saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")<Asset = <T as [AssetTransformer](../../asset/transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer")\>::[AssetOutput](../../asset/transformer/trait.AssetTransformer.html#associatedtype.AssetOutput "type bevy::asset::transformer::AssetTransformer::AssetOutput")\> + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [LoadTransformAndSave](../../asset/processor/struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave")<L, T, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#189)

### impl<M> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [FocusedInput](../../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#172)

### impl<M> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

### impl<M> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshMaterial2d](../../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

where M: [Material2d](../../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial2d](../../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

### impl<M> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MeshMaterial3d](../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](../../prelude/trait.Material.html "trait bevy::prelude::Material") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial3d](../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/mod.rs.html#117)

### impl<M> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MessageId](../../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MessageId](../../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/messages.rs.html#94)

### impl<M> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Messages](../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Messages](../../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#286)

### impl<P, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimatableCurve](../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>

where [AnimatableCurve](../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), P: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#434)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicBSpline](../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBSpline](../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#54)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicBezier](../../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBezier](../../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#272)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicCardinalSpline](../../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCardinalSpline](../../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1169)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicCurve](../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCurve](../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#144)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicHermite](../../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicHermite](../../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#611)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicNurbs](../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicNurbs](../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#946)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicSegment](../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicSegment](../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#837)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LinearSpline](../../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [LinearSpline](../../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1470)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RationalCurve](../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalCurve](../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1328)

### impl<P> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RationalSegment](../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalSegment](../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#482)

### impl<S, T, C, D> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ZipCurve](../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where [ZipCurve](../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), D: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#216-221)

### impl<S, T, C, F> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MapCurve](../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>

where S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), F: 'static,

Available on **crate feature `bevy_reflect`** only.

Note: This is not a fully stable implementation of `TypePath` due to usage of `type_name` for function members.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#229)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DespawnOnEnter](../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DespawnOnEnter](../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#148)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DespawnOnExit](../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DespawnOnExit](../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#66)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DespawnWhen](../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DespawnWhen](../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#476)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DisableOnEnter](../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DisableOnEnter](../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#395)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DisableOnExit](../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DisableOnExit](../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#313)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DisableWhen](../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DisableWhen](../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#723)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EnableOnEnter](../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EnableOnEnter](../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#642)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EnableOnExit](../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EnableOnExit](../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#560)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EnableWhen](../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EnableWhen](../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#178)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [NextState](../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [NextState](../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#131)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PreviousState](../../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [PreviousState](../../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#55)

### impl<S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [State](../../prelude/struct.State.html "struct bevy::prelude::State")<S>

where S: [States](../../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [State](../../prelude/struct.State.html "struct bevy::prelude::State")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

### impl<Source> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AudioPlayer](../../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

where [AudioPlayer](../../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Source: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Decodable](../../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#524)

### impl<T, C, D> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ChainCurve](../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where [ChainCurve](../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), D: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#787)

### impl<T, C, D> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ContinuationCurve](../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where [ContinuationCurve](../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), D: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#417)

### impl<T, C, D> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CurveReparamCurve](../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where [CurveReparamCurve](../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), D: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#315-319)

### impl<T, C, F> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ReparamCurve](../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), F: 'static,

Available on **crate feature `bevy_reflect`** only.

Note: This is not a fully stable implementation of `TypePath` due to usage of `type_name` for function members.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#677)

### impl<T, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ForeverCurve](../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where [ForeverCurve](../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#451)

### impl<T, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [GraphCurve](../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where [GraphCurve](../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#381)

### impl<T, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [LinearReparamCurve](../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where [LinearReparamCurve](../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#732)

### impl<T, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [PingPongCurve](../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where [PingPongCurve](../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#617)

### impl<T, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [RepeatCurve](../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where [RepeatCurve](../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#575)

### impl<T, C> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ReverseCurve](../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where [ReverseCurve](../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#111-114)

### impl<T, F> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [FunctionCurve](../../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), F: 'static,

Available on **crate feature `bevy_reflect`** only.

Note: This is not a fully stable implementation of `TypePath` due to usage of `type_name` for function members.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#7)

### impl<T, H> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Hashed](../../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), H: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Hashed](../../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#51-54)

### impl<T, I> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SampleCurve](../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), I: 'static,

Available on **crate feature `bevy_reflect`** only.

Note: This is not a fully stable implementation of `TypePath` due to usage of `type_name` for function members.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#211-214)

### impl<T, I> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UnevenSampleCurve](../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), I: 'static,

Available on **crate feature `bevy_reflect`** only.

Note: This is not a fully stable implementation of `TypePath` due to usage of `type_name` for function members.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#722)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [AnimatableKeyframeCurve](../../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>

where [AnimatableKeyframeCurve](../../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#3)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#301)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ArcMutexValue](../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>

where T: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ArcMutexValue](../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/axis.rs.html#16)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Axis](../../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>

where [Axis](../../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/button_input.rs.html#124)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ButtonInput](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ButtonInput](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#467)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ChunkedUnevenCore](../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>

where [ChunkedUnevenCore](../../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_gradient.rs.html#11)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ColorCurve](../../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>

where [ColorCurve](../../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#46)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ConstantCurve](../../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where [ConstantCurve](../../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#50)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [CubicKeyframeCurve](../../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>

where [CubicKeyframeCurve](../../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#298)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EasingCurve](../../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>

where [EasingCurve](../../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#122)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EvenCore](../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>

where [EvenCore](../../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#272)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [HandleTemplate](../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

where T: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HandleTemplate](../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#45)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>

where T: [Internable](../../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") + 'static + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Interned](../../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#25)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [InterpolationDatum](../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>

where [InterpolationDatum](../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/maybe_location.rs.html#20)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [MaybeLocation](../../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>

where [MaybeLocation](../../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#4)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mutex](../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>

where [Mutex](../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#139)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SampleAutoCurve](../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>

where [SampleAutoCurve](../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#12)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [SteppedKeyframeCurve](../../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>

where [SteppedKeyframeCurve](../../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Time](../../prelude/struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Time](../../prelude/struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#326)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UnevenCore](../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>

where [UnevenCore](../../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#314)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [UnevenSampleAutoCurve](../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>

where [UnevenSampleAutoCurve](../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [ValueChange](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

where [ValueChange](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/vec.rs.html#18)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VirtualKeyPressed](../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

where [VirtualKeyPressed](../../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#22)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [VirtualKeyboard](../../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [VirtualKeyboard](../../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#285)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WideCubicKeyframeCurve](../../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>

where [WideCubicKeyframeCurve](../../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#174)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WideLinearKeyframeCurve](../../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>

where [WideLinearKeyframeCurve](../../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#228)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WideSteppedKeyframeCurve](../../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>

where [WideSteppedKeyframeCurve](../../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#602)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

where [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#617)

### impl<T> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [WithTwoDerivatives](../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

where [WithTwoDerivatives](../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_set.rs.html#10)

### impl<V, S> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for bevy::platform::collections::[HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V, S>

where [HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V, S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#159)

### impl<V, W> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Sum](../../math/struct.Sum.html "struct bevy::math::Sum")<V, W>

where [Sum](../../math/struct.Sum.html "struct bevy::math::Sum")<V, W>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), W: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#19)

### impl<V> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityHashMap](../../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>

where [EntityHashMap](../../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [EntityIndexMap](../../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),