[bevy](../index.html)

# Crate reflect 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#1-4291)

Reflection in Rust.

[Reflection](https://en.wikipedia.org/wiki/Reflective_programming) is a powerful tool provided within many programming languages that allows for meta-programming: using information _about_ the program to _affect_ the program. In other words, reflection allows us to inspect the program itself, its syntax, and its type information at runtime.

This crate adds this missing reflection functionality to Rust. Though it was made with the [Bevy](https://bevy.org/) game engine in mind, it’s a general-purpose solution that can be used in any Rust project.

At a very high level, this crate allows you to:

*   Dynamically interact with Rust values
*   Access type metadata at runtime
*   Serialize and deserialize (i.e. save and load) data

It’s important to note that because of missing features in Rust, there are some [limitations](#limitations) with this crate.

## The `Reflect` and `PartialReflect` traits

At the root of [`bevy_reflect`](index.html "mod bevy::reflect") is the [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") trait.

Its purpose is to allow dynamic [introspection](https://en.wikipedia.org/wiki/Type_introspection) of values, following Rust’s type system through a system of [subtraits](#the-reflection-subtraits).

Its primary purpose is to allow all implementors to be passed around as a `dyn PartialReflect` trait object in one of the following forms:

*   `&dyn PartialReflect`
*   `&mut dyn PartialReflect`
*   `Box<dyn PartialReflect>`

This allows values of types implementing `PartialReflect` to be operated upon completely dynamically (at a small [runtime cost](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)).

Building on `PartialReflect` is the [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") trait.

`PartialReflect` is a supertrait of `Reflect` so any type implementing `Reflect` implements `PartialReflect` by definition. `dyn Reflect` trait objects can be used similarly to `dyn PartialReflect`, but `Reflect` is also often used in trait bounds (like `T: Reflect`).

The distinction between `PartialReflect` and `Reflect` is summarized in the following:

*   `PartialReflect` is a trait for interacting with values under `bevy_reflect`’s data model. This means values implementing `PartialReflect` can be dynamically constructed and introspected.
*   The `Reflect` trait, however, ensures that the interface exposed by `PartialReflect` on types which additionally implement `Reflect` mirrors the structure of a single Rust type.
*   This means `dyn Reflect` trait objects can be directly downcast to concrete types, where `dyn PartialReflect` trait object cannot.
*   `Reflect`, since it provides a stronger type-correctness guarantee, is the trait used to interact with [the type registry](#type-registration).

### Converting between `PartialReflect` and `Reflect`

Since `T: Reflect` implies `T: PartialReflect`, conversion from a `dyn Reflect` to a `dyn PartialReflect` trait object (upcasting) is infallible and can be performed with one of the following methods. Note that these are temporary while [the language feature for dyn upcasting coercion](https://github.com/rust-lang/rust/issues/65991) is experimental:

*   [`PartialReflect::as_partial_reflect`](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect "method bevy::prelude::PartialReflect::as_partial_reflect") for `&dyn PartialReflect`
*   [`PartialReflect::as_partial_reflect_mut`](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut "method bevy::prelude::PartialReflect::as_partial_reflect_mut") for `&mut dyn PartialReflect`
*   [`PartialReflect::into_partial_reflect`](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect "method bevy::prelude::PartialReflect::into_partial_reflect") for `Box<dyn PartialReflect>`

For conversion in the other direction — downcasting `dyn PartialReflect` to `dyn Reflect` — there are fallible methods:

*   [`PartialReflect::try_as_reflect`](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect "method bevy::prelude::PartialReflect::try_as_reflect") for `&dyn Reflect`
*   [`PartialReflect::try_as_reflect_mut`](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut "method bevy::prelude::PartialReflect::try_as_reflect_mut") for `&mut dyn Reflect`
*   [`PartialReflect::try_into_reflect`](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect "method bevy::prelude::PartialReflect::try_into_reflect") for `Box<dyn Reflect>`

Additionally, [`FromReflect::from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") can be used to convert a `dyn PartialReflect` to a concrete type which implements `Reflect`.

## Implementing `Reflect`

Implementing `Reflect` (and `PartialReflect`) is easily done using the provided [derive macro](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect"):

```rust
#[derive(Reflect)]
struct MyStruct {
  foo: i32
}
```

This will automatically generate the implementation of `Reflect` for any struct or enum.

It will also generate other very important trait implementations used for reflection:

*   [`GetTypeRegistration`](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration")
*   [`Typed`](trait.Typed.html "trait bevy::reflect::Typed")
*   [`Struct`](../prelude/trait.Struct.html "trait bevy::prelude::Struct"), [`TupleStruct`](../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct"), or [`Enum`](enums/trait.Enum.html "trait bevy::reflect::enums::Enum") depending on the type

### Requirements

We can implement `Reflect` on any type that satisfies _both_ of the following conditions:

*   The type implements `Any`, `Send`, and `Sync`. For the `Any` requirement to be satisfied, the type itself must have a [`'static` lifetime](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound).
*   All fields and sub-elements themselves implement `Reflect` (see the [derive macro documentation](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") for details on how to ignore certain fields when deriving).

Additionally, using the derive macro on enums requires a third condition to be met:

*   All fields and sub-elements must implement [`FromReflect`](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect")— another important reflection trait discussed in a later section.

## The Reflection Subtraits

Since [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") is meant to cover any and every type, this crate also comes with a few more traits to accompany `PartialReflect` and provide more specific interactions. We refer to these traits as the _reflection subtraits_ since they all have `PartialReflect` as a supertrait. The current list of reflection subtraits include:

*   [`Tuple`](tuple/trait.Tuple.html "trait bevy::reflect::tuple::Tuple")
*   [`Array`](array/trait.Array.html "trait bevy::reflect::array::Array")
*   [`List`](list/trait.List.html "trait bevy::reflect::list::List")
*   [`Set`](set/trait.Set.html "trait bevy::reflect::set::Set")
*   [`Map`](map/trait.Map.html "trait bevy::reflect::map::Map")
*   [`Struct`](../prelude/trait.Struct.html "trait bevy::prelude::Struct")
*   [`TupleStruct`](../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct")
*   [`Enum`](enums/trait.Enum.html "trait bevy::reflect::enums::Enum")
*   [`Function`](../prelude/trait.Function.html "trait bevy::prelude::Function") (requires the `functions` feature)

As mentioned previously, the last three are automatically implemented by the [derive macro](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect").

Each of these traits come with their own methods specific to their respective category. For example, we can access our struct’s fields by name using the [`Struct::field`](../prelude/trait.Struct.html#tymethod.field "method bevy::prelude::Struct::field") method.

```rust
let my_struct: Box<dyn Struct> = Box::new(MyStruct {
  foo: 123
});
let foo: &dyn PartialReflect = my_struct.field("foo").unwrap();
assert_eq!(Some(&123), foo.try_downcast_ref::<i32>());
```

Since most data is passed around as `dyn PartialReflect` or `dyn Reflect` trait objects, the `PartialReflect` trait has methods for going to and from these subtraits.

[`PartialReflect::reflect_kind`](../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"), [`PartialReflect::reflect_ref`](../prelude/trait.PartialReflect.html#tymethod.reflect_ref "method bevy::prelude::PartialReflect::reflect_ref"), [`PartialReflect::reflect_mut`](../prelude/trait.PartialReflect.html#tymethod.reflect_mut "method bevy::prelude::PartialReflect::reflect_mut"), and [`PartialReflect::reflect_owned`](../prelude/trait.PartialReflect.html#tymethod.reflect_owned "method bevy::prelude::PartialReflect::reflect_owned") all return an enum that respectively contains zero-sized, immutable, mutable, and owned access to the type as a subtrait object.

For example, we can get out a `dyn Tuple` from our reflected tuple type using one of these methods.

```rust
let my_tuple: Box<dyn PartialReflect> = Box::new((1, 2, 3));
let my_tuple = my_tuple.reflect_ref().as_tuple().unwrap();
assert_eq!(3, my_tuple.field_len());
```

And to go back to a general-purpose `dyn PartialReflect`, we can just use the matching [`PartialReflect::as_partial_reflect`](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect "method bevy::prelude::PartialReflect::as_partial_reflect"), [`PartialReflect::as_partial_reflect_mut`](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut "method bevy::prelude::PartialReflect::as_partial_reflect_mut"), or [`PartialReflect::into_partial_reflect`](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect "method bevy::prelude::PartialReflect::into_partial_reflect") methods.

### Opaque Types

Some types don’t fall under a particular subtrait.

These types hide their internal structure to reflection, either because it is not possible, difficult, or not useful to reflect its internals. Such types are known as _opaque_ types.

This includes truly opaque types like `String` or `Instant`, but also includes all the primitive types (e.g. `bool`, `usize`, etc.) since they can’t be broken down any further.

## Dynamic Types

Each subtrait comes with a corresponding _dynamic_ type.

The available dynamic types are:

*   [`DynamicTuple`](tuple/struct.DynamicTuple.html "struct bevy::reflect::tuple::DynamicTuple")
*   [`DynamicArray`](array/struct.DynamicArray.html "struct bevy::reflect::array::DynamicArray")
*   [`DynamicList`](list/struct.DynamicList.html "struct bevy::reflect::list::DynamicList")
*   [`DynamicMap`](map/struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")
*   [`DynamicStruct`](structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")
*   [`DynamicTupleStruct`](tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")
*   [`DynamicEnum`](enums/struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

These dynamic types may contain any arbitrary reflected data.

```rust
let mut data = DynamicStruct::default();
data.insert("foo", 123_i32);
assert_eq!(Some(&123), data.field("foo").unwrap().try_downcast_ref::<i32>())
```

They are most commonly used as “proxies” for other types, where they contain the same data as— and therefore, represent— a concrete type. The [`PartialReflect::to_dynamic`](../prelude/trait.PartialReflect.html#method.to_dynamic "method bevy::prelude::PartialReflect::to_dynamic") method will return a dynamic type for all non-opaque types, allowing all types to essentially be “cloned” into a dynamic type. And since dynamic types themselves implement [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), we may pass them around just like most other reflected types.

```rust
let original: Box<dyn Reflect> = Box::new(MyStruct {
  foo: 123
});

// `dynamic` will be a `DynamicStruct` representing a `MyStruct`
let dynamic: Box<dyn PartialReflect> = original.to_dynamic();
assert!(dynamic.represents::<MyStruct>());
```

### Patching

These dynamic types come in handy when needing to apply multiple changes to another type. This is known as “patching” and is done using the [`PartialReflect::apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") and [`PartialReflect::try_apply`](../prelude/trait.PartialReflect.html#tymethod.try_apply "method bevy::prelude::PartialReflect::try_apply") methods.

```rust
let mut value = Some(123_i32);
let patch = DynamicEnum::new("None", ());
value.apply(&patch);
assert_eq!(None, value);
```

### `FromReflect`

It’s important to remember that dynamic types are _not_ the concrete type they may be representing. A common mistake is to treat them like such when trying to cast back to the original type or when trying to make use of a reflected trait which expects the actual type.

[ⓘ](# "This example panics")

```rust
let original: Box<dyn Reflect> = Box::new(MyStruct {
  foo: 123
});

let dynamic: Box<dyn PartialReflect> = original.to_dynamic();
let value = dynamic.try_take::<MyStruct>().unwrap(); // PANIC!
```

To resolve this issue, we’ll need to convert the dynamic type to the concrete one. This is where [`FromReflect`](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") comes in.

`FromReflect` is a trait that allows an instance of a type to be generated from a dynamic representation— even partial ones. And since the [`FromReflect::from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") method takes the data by reference, this can be used to effectively clone data (to an extent).

It is automatically implemented when [deriving `Reflect`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") on a type unless opted out of using `#[reflect(from_reflect = false)]` on the item.

```rust
#[derive(Reflect)]
struct MyStruct {
  foo: i32
}
let original: Box<dyn Reflect> = Box::new(MyStruct {
  foo: 123
});

let dynamic: Box<dyn PartialReflect> = original.to_dynamic();
let value = <MyStruct as FromReflect>::from_reflect(&*dynamic).unwrap(); // OK!
```

When deriving, all active fields and sub-elements must also implement `FromReflect`.

Fields can be given default values for when a field is missing in the passed value or even ignored. Ignored fields must either implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") or have a default function specified using `#[reflect(default = "path::to::function")]`.

See the [derive macro documentation](../prelude/derive.FromReflect.html "derive bevy::prelude::FromReflect") for details.

All primitives and simple types implement `FromReflect` by relying on their [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") implementation.

## Path navigation

The [`GetPath`](../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") trait allows accessing arbitrary nested fields of an [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") type.

Using `GetPath`, it is possible to use a path string to access a specific field of a reflected type.

```rust
#[derive(Reflect)]
struct MyStruct {
  value: Vec<Option<u32>>
}

let my_struct = MyStruct {
  value: vec![None, None, Some(123)],
};
assert_eq!(
  my_struct.path::<u32>(".value[2].0").unwrap(),
  &123,
);
```

## Type Registration

This crate also comes with a [`TypeRegistry`](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") that can be used to store and retrieve additional type metadata at runtime, such as helper types and trait implementations.

The [derive macro](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") for [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") also generates an implementation of the [`GetTypeRegistration`](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") trait, which is used by the registry to generate a [`TypeRegistration`](struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") struct for that type. We can then register additional [type data](trait.TypeData.html "trait bevy::reflect::TypeData") we want associated with that type.

For example, we can register [`ReflectDefault`](../prelude/struct.ReflectDefault.html "struct bevy::prelude::ReflectDefault") on our type so that its `Default` implementation may be used dynamically.

```rust
#[derive(Reflect, Default)]
struct MyStruct {
  foo: i32
}
let mut registry = TypeRegistry::empty();
registry.register::<MyStruct>();
registry.register_type_data::<MyStruct, ReflectDefault>();

let registration = registry.get(core::any::TypeId::of::<MyStruct>()).unwrap();
let reflect_default = registration.data::<ReflectDefault>().unwrap();

let new_value: Box<dyn Reflect> = reflect_default.default();
assert!(new_value.is::<MyStruct>());
```

Because this operation is so common, the derive macro actually has a shorthand for it. By using the `#[reflect(Trait)]` attribute, the derive macro will automatically register a matching, in-scope `ReflectTrait` type within the `GetTypeRegistration` implementation.

```rust
use bevy_reflect::prelude::{Reflect, ReflectDefault};

#[derive(Reflect, Default)]
#[reflect(Default)]
struct MyStruct {
  foo: i32
}
```

### Reflecting Traits

Type data doesn’t have to be tied to a trait, but it’s often extremely useful to create trait type data. These allow traits to be used directly on a `dyn Reflect` (and not a `dyn PartialReflect`) while utilizing the underlying type’s implementation.

For any [object-safe](https://doc.rust-lang.org/reference/items/traits.html#object-safety) trait, we can easily generate a corresponding `ReflectTrait` type for our trait using the [`#[reflect_trait]`](../prelude/attr.reflect_trait.html "attr bevy::prelude::reflect_trait") macro.

```rust
#[reflect_trait] // Generates a `ReflectMyTrait` type
pub trait MyTrait {}
impl<T: Reflect> MyTrait for T {}

let mut registry = TypeRegistry::new();
registry.register_type_data::<i32, ReflectMyTrait>();
```

The generated type data can be used to convert a valid `dyn Reflect` into a `dyn MyTrait`. See the [dynamic types example](https://github.com/bevyengine/bevy/blob/latest/examples/reflection/dynamic_types.rs) for more information and usage details.

## Serialization

By using reflection, we are also able to get serialization capabilities for free. In fact, using [`bevy_reflect`](index.html "mod bevy::reflect") can result in faster compile times and reduced code generation over directly deriving the [`serde`](https://docs.rs/serde/1.0.228/x86_64-unknown-linux-gnu/serde/index.html "mod serde") traits.

The way it works is by moving the serialization logic into common serializers and deserializers:

*   [`ReflectSerializer`](serde/struct.ReflectSerializer.html "struct bevy::reflect::serde::ReflectSerializer")
*   [`TypedReflectSerializer`](serde/struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer")
*   [`ReflectDeserializer`](serde/struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer")
*   [`TypedReflectDeserializer`](serde/struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer")

All of these structs require a reference to the [registry](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") so that [type information](enum.TypeInfo.html "enum bevy::reflect::TypeInfo") can be retrieved, as well as registered type data, such as [`ReflectSerialize`](../prelude/struct.ReflectSerialize.html "struct bevy::prelude::ReflectSerialize") and [`ReflectDeserialize`](../prelude/struct.ReflectDeserialize.html "struct bevy::prelude::ReflectDeserialize").

The general entry point are the “untyped” versions of these structs. These will automatically extract the type information and pass them into their respective “typed” version.

The output of the `ReflectSerializer` will be a map, where the key is the [type path](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") and the value is the serialized data. The `TypedReflectSerializer` will simply output the serialized data.

The `ReflectDeserializer` can be used to deserialize this map and return a `Box<dyn Reflect>`, where the underlying type will be a dynamic type representing some concrete type (except for opaque types).

Again, it’s important to remember that dynamic types may need to be converted to their concrete counterparts in order to be used in certain cases. This can be achieved using [`FromReflect`](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect").

```rust
#[derive(Reflect, PartialEq, Debug)]
struct MyStruct {
  foo: i32
}

let original_value = MyStruct {
  foo: 123
};

// Register
let mut registry = TypeRegistry::new();
registry.register::<MyStruct>();

// Serialize
let reflect_serializer = ReflectSerializer::new(original_value.as_partial_reflect(), &registry);
let serialized_value: String = ron::to_string(&reflect_serializer).unwrap();

// Deserialize
let reflect_deserializer = ReflectDeserializer::new(&registry);
let deserialized_value: Box<dyn PartialReflect> = reflect_deserializer.deserialize(
  &mut ron::Deserializer::from_str(&serialized_value).unwrap()
).unwrap();

// Convert
let converted_value = <MyStruct as FromReflect>::from_reflect(&*deserialized_value).unwrap();

assert_eq!(original_value, converted_value);
```

## Limitations

While this crate offers a lot in terms of adding reflection to Rust, it does come with some limitations that don’t make it as featureful as reflection in other programming languages.

### Non-Static Lifetimes

One of the most obvious limitations is the `'static` requirement. Rust requires fields to define a lifetime for referenced data, but [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") requires all types to have a `'static` lifetime. This makes it impossible to reflect any type with non-static borrowed data.

### Generic Function Reflection

Another limitation is the inability to reflect over generic functions directly. It can be done, but will typically require manual monomorphization (i.e. manually specifying the types the generic method can take).

## Features

### `bevy`

| Default | Dependencies |
| --- | --- |
| ❌ | [`bevy_math`](https://docs.rs/bevy_math/latest/bevy_math/), [`glam`](https://docs.rs/glam/latest/glam/), [`indexmap`](https://docs.rs/indexmap/latest/indexmap/), [`smallvec`](https://docs.rs/smallvec/latest/smallvec/) |

This feature makes it so that the appropriate reflection traits are implemented on all the types necessary for the [Bevy](https://bevy.org/) game engine. enables the optional dependencies: [`bevy_math`](https://docs.rs/bevy_math/latest/bevy_math/), [`glam`](https://docs.rs/glam/latest/glam/), [`indexmap`](https://docs.rs/indexmap/latest/indexmap/), and [`smallvec`](https://docs.rs/smallvec/latest/smallvec/). These dependencies are used by the [Bevy](https://bevy.org/) game engine and must define their reflection implementations within this crate due to Rust’s [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type:~:text=But%20we%20can%E2%80%99t,implementation%20to%20use.).

### `functions`

| Default | Dependencies |
| --- | --- |
| ❌ | [`bevy_reflect_derive/functions`](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/bevy_reflect_derive/index.html "mod bevy_reflect_derive") |

This feature allows creating a [`DynamicFunction`](func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut") from Rust functions. Dynamic functions can then be called with valid [`ArgList`](func/struct.ArgList.html "struct bevy::reflect::func::ArgList")s.

For more information, read the [`func`](func/index.html "mod bevy::reflect::func") module docs.

### `documentation`

| Default | Dependencies |
| --- | --- |
| ❌ | [`bevy_reflect_derive/documentation`](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/bevy_reflect_derive/index.html "mod bevy_reflect_derive") |

This feature enables capturing doc comments as strings for items that [derive `Reflect`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect"). Documentation information can then be accessed at runtime on the [`TypeInfo`](enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of that item.

This can be useful for generating documentation for scripting language interop or for displaying tooltips in an editor.

### `debug`

| Default | Dependencies |
| --- | --- |
| ✅ | `debug_stack` |

This feature enables useful debug features for reflection.

This includes the `debug_stack` feature, which enables capturing the type stack when serializing or deserializing a type and displaying it in error messages.

### `auto_register_inventory`/`auto_register_static`

| Default | Dependencies |
| --- | --- |
| ✅ | `bevy_reflect_derive/auto_register_inventory` |
| ❌ | `bevy_reflect_derive/auto_register_static` |

These features enable automatic registration of types that derive [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

*   `auto_register_inventory` uses `inventory` to collect types on supported platforms (Linux, macOS, iOS, FreeBSD, Android, Windows, WebAssembly).
*   `auto_register_static` uses platform-independent way to collect types, but requires additional setup and might slow down compilation, so it should only be used on platforms not supported by `inventory`. See documentation for [`load_type_registrations`](macro.load_type_registrations.html "macro bevy::reflect::load_type_registrations") macro for more info

When this feature is enabled `bevy_reflect` will automatically collects all types that derive [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") on app startup, and [`TypeRegistry::register_derived_types`](struct.TypeRegistry.html#method.register_derived_types "method bevy::reflect::TypeRegistry::register_derived_types") can be used to register these types at any point in the program. However, this does not apply to types with generics: their desired monomorphized representations must be registered manually.

## Modules

[access](access/index.html "mod bevy::reflect::access")

Representation for individual element accesses within a path.

[array](array/index.html "mod bevy::reflect::array")

Traits and types used to power [array-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) operations via reflection.

[attributes](attributes/index.html "mod bevy::reflect::attributes")

Types and functions for creating, manipulating and querying [`CustomAttributes`](attributes/struct.CustomAttributes.html "struct bevy::reflect::attributes::CustomAttributes").

[convert](convert/index.html "mod bevy::reflect::convert")

The [`ReflectConvert`](convert/struct.ReflectConvert.html "struct bevy::reflect::convert::ReflectConvert") type, which allows types to register conversions to and from one another.

[enums](enums/index.html "mod bevy::reflect::enums")

Traits and types used to power [enum-like](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) operations via reflection.

[erased\_serde](erased_serde/index.html "mod bevy::reflect::erased_serde")

[github](https://github.com/dtolnay/erased-serde) [crates-io](https://crates.io/crates/erased-serde) [docs-rs](https://docs.rs/erased-serde)

[func](func/index.html "mod bevy::reflect::func")`functions`

Reflection-based dynamic functions.

[list](list/index.html "mod bevy::reflect::list")

Traits and types used to power [list-like](https://doc.rust-lang.org/book/ch08-01-vectors.html) operations via reflection.

[map](map/index.html "mod bevy::reflect::map")

Traits and types used to power [map-like](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) operations via reflection.

[prelude](prelude/index.html "mod bevy::reflect::prelude")

The reflect prelude.

[serde](serde/index.html "mod bevy::reflect::serde")

Serde integration for reflected types.

[set](set/index.html "mod bevy::reflect::set")

A trait used to power [set-like](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html) operations via reflection.

[std\_traits](std_traits/index.html "mod bevy::reflect::std_traits")

Module containing the [`ReflectDefault`](../prelude/struct.ReflectDefault.html "struct bevy::prelude::ReflectDefault") type.

[structs](structs/index.html "mod bevy::reflect::structs")

Traits and types used to power [struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) operations via reflection.

[tuple](tuple/index.html "mod bevy::reflect::tuple")

Traits and types used to power [tuple-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) operations via reflection.

[tuple\_struct](tuple_struct/index.html "mod bevy::reflect::tuple_struct")

Traits and types used to power [tuple-struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) operations via reflection.

[utility](utility/index.html "mod bevy::reflect::utility")

Helpers for working with Bevy reflection.

## Macros

[hash\_error](macro.hash_error.html "macro bevy::reflect::hash_error")

Used to produce an error message when an attempt is made to hash a [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value that does not support hashing.

[impl\_from\_reflect\_opaque](macro.impl_from_reflect_opaque.html "macro bevy::reflect::impl_from_reflect_opaque")

A macro used to generate a `FromReflect` trait implementation for the given type.

[impl\_reflect](macro.impl_reflect.html "macro bevy::reflect::impl_reflect")

A replacement for `#[derive(Reflect)]` to be used with foreign types which the definitions of cannot be altered.

[impl\_reflect\_opaque](macro.impl_reflect_opaque.html "macro bevy::reflect::impl_reflect_opaque")

A macro used to generate reflection trait implementations for the given type.

[impl\_type\_path](macro.impl_type_path.html "macro bevy::reflect::impl_type_path")

A replacement for [deriving `TypePath`](../prelude/derive.TypePath.html "derive bevy::prelude::TypePath") for use on foreign types.

[load\_type\_registrations](macro.load_type_registrations.html "macro bevy::reflect::load_type_registrations")

Collects and loads type registrations when using `auto_register_static` feature.

## Structs

[AccessError](struct.AccessError.html "struct bevy::reflect::AccessError")

An error originating from an [`Access`](enum.Access.html "enum bevy::reflect::Access") of an element within a type.

[ConstParamInfo](struct.ConstParamInfo.html "struct bevy::reflect::ConstParamInfo")

Type information for a const generic parameter.

[Generics](struct.Generics.html "struct bevy::reflect::Generics")

The generic parameters of a type.

[NamedField](struct.NamedField.html "struct bevy::reflect::NamedField")

The named field of a reflected struct.

[OffsetAccess](struct.OffsetAccess.html "struct bevy::reflect::OffsetAccess")

An [`Access`](enum.Access.html "enum bevy::reflect::Access") combined with an `offset` for more helpful error reporting.

[OpaqueInfo](struct.OpaqueInfo.html "struct bevy::reflect::OpaqueInfo")

A container for compile-time info related to reflection-opaque types, including primitives.

[ParseError](struct.ParseError.html "struct bevy::reflect::ParseError")

An error that occurs when parsing reflect path strings.

[ParsedPath](struct.ParsedPath.html "struct bevy::reflect::ParsedPath")

A pre-parsed path to an element within a type.

[ReflectDeserialize](struct.ReflectDeserialize.html "struct bevy::reflect::ReflectDeserialize")

A struct used to deserialize reflected instances of a type.

[ReflectFromPtr](struct.ReflectFromPtr.html "struct bevy::reflect::ReflectFromPtr")

[`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") values are commonly used in situations where the actual types of values are not known at runtime. In such situations you might have access to a `*const ()` pointer that you know implements [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"), but have no way of turning it into a `&dyn Reflect`.

[ReflectFromReflect](struct.ReflectFromReflect.html "struct bevy::reflect::ReflectFromReflect")

Type data that represents the [`FromReflect`](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") trait and allows it to be used dynamically.

[ReflectKindMismatchError](struct.ReflectKindMismatchError.html "struct bevy::reflect::ReflectKindMismatchError")

Caused when a type was expected to be of a certain [kind](enum.ReflectKind.html "enum bevy::reflect::ReflectKind"), but was not.

[ReflectSerialize](struct.ReflectSerialize.html "struct bevy::reflect::ReflectSerialize")

A struct used to serialize reflected instances of a type.

[Type](struct.Type.html "struct bevy::reflect::Type")

The base representation of a Rust type.

[TypeParamInfo](struct.TypeParamInfo.html "struct bevy::reflect::TypeParamInfo")

Type information for a generic type parameter.

[TypePathTable](struct.TypePathTable.html "struct bevy::reflect::TypePathTable")

Provides dynamic access to all methods on [`TypePath`](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath").

[TypeRegistration](struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Runtime storage for type metadata, registered into the [`TypeRegistry`](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry").

[TypeRegistry](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")

A registry of [reflected](index.html "mod bevy::reflect") types.

[TypeRegistryArc](struct.TypeRegistryArc.html "struct bevy::reflect::TypeRegistryArc")

A synchronized wrapper around a [`TypeRegistry`](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry").

[UnnamedField](struct.UnnamedField.html "struct bevy::reflect::UnnamedField")

The unnamed field of a reflected tuple or tuple struct.

## Enums

[Access](enum.Access.html "enum bevy::reflect::Access")

A singular element access within a path. Multiple accesses can be combined into a [`ParsedPath`](struct.ParsedPath.html "struct bevy::reflect::ParsedPath").

[AccessErrorKind](enum.AccessErrorKind.html "enum bevy::reflect::AccessErrorKind")

The kind of [`AccessError`](struct.AccessError.html "struct bevy::reflect::AccessError"), along with some kind-specific information.

[ApplyError](enum.ApplyError.html "enum bevy::reflect::ApplyError")

A enumeration of all error outcomes that might happen when running [`try_apply`](../prelude/trait.PartialReflect.html#tymethod.try_apply "method bevy::prelude::PartialReflect::try_apply").

[FieldId](enum.FieldId.html "enum bevy::reflect::FieldId")

A representation of a field’s accessor.

[GenericInfo](enum.GenericInfo.html "enum bevy::reflect::GenericInfo")

An enum representing a generic parameter.

[ReflectCloneError](enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")

An error that occurs when cloning a type via [`PartialReflect::reflect_clone`](../prelude/trait.PartialReflect.html#method.reflect_clone "method bevy::prelude::PartialReflect::reflect_clone").

[ReflectKind](enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

An enumeration of the “kinds” of a reflected type.

[ReflectMut](enum.ReflectMut.html "enum bevy::reflect::ReflectMut")

A mutable enumeration of [“kinds”](enum.ReflectKind.html "enum bevy::reflect::ReflectKind") of a reflected type.

[ReflectOwned](enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

An owned enumeration of [“kinds”](enum.ReflectKind.html "enum bevy::reflect::ReflectKind") of a reflected type.

[ReflectPathError](enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")

An error returned from a failed path string query.

[ReflectRef](enum.ReflectRef.html "enum bevy::reflect::ReflectRef")

An immutable enumeration of [“kinds”](enum.ReflectKind.html "enum bevy::reflect::ReflectKind") of a reflected type.

[TypeInfo](enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Compile-time type information for various reflected types.

[TypeInfoError](enum.TypeInfoError.html "enum bevy::reflect::TypeInfoError")

A [`TypeInfo`](enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\-specific error.

## Traits

[DynamicTypePath](trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath")

Dynamic dispatch for [`TypePath`](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath").

[DynamicTyped](trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped")

Dynamic dispatch for [`Typed`](trait.Typed.html "trait bevy::reflect::Typed").

[FromReflect](trait.FromReflect.html "trait bevy::reflect::FromReflect")

A trait that enables types to be dynamically constructed from reflected data.

[FromType](trait.FromType.html "trait bevy::reflect::FromType")

Trait used to generate [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") for trait reflection.

[GetPath](trait.GetPath.html "trait bevy::reflect::GetPath")

A trait which allows nested [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") values to be retrieved with path strings.

[GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration")

A trait which allows a type to generate its [`TypeRegistration`](struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for registration into the [`TypeRegistry`](struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry").

[Is](trait.Is.html "trait bevy::reflect::Is")

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison.

[PartialReflect](trait.PartialReflect.html "trait bevy::reflect::PartialReflect")

The foundational trait of [`bevy_reflect`](index.html "mod bevy::reflect"), used for accessing and modifying data dynamically.

[Reflect](trait.Reflect.html "trait bevy::reflect::Reflect")

A core trait of [`bevy_reflect`](index.html "mod bevy::reflect"), used for downcasting to concrete types.

[ReflectPath](trait.ReflectPath.html "trait bevy::reflect::ReflectPath")

Something that can be interpreted as a reflection path in [`GetPath`](../prelude/trait.GetPath.html "trait bevy::prelude::GetPath").

[ReflectRemote](trait.ReflectRemote.html "trait bevy::reflect::ReflectRemote")

Marks a type as a [reflectable](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") wrapper for a remote type.

[Reflectable](trait.Reflectable.html "trait bevy::reflect::Reflectable")

A catch-all trait that is bound by the core reflection traits, useful to simplify reflection-based generic type bounds.

[TypeData](trait.TypeData.html "trait bevy::reflect::TypeData")

A trait used to type-erase type metadata.

[TypePath](trait.TypePath.html "trait bevy::reflect::TypePath")

A static accessor to type paths and names.

[Typed](trait.Typed.html "trait bevy::reflect::Typed")

A static accessor to compile-time type information.

## Attribute Macros

[reflect\_remote](attr.reflect_remote.html "attr bevy::reflect::reflect_remote")

Generates a wrapper type that can be used to “derive `Reflect`” for remote types.

[reflect\_trait](attr.reflect_trait.html "attr bevy::reflect::reflect_trait")

A macro that automatically generates type data for traits, which their implementors can then register.

## Derive Macros

[FromReflect](derive.FromReflect.html "derive bevy::reflect::FromReflect")

Derives the `FromReflect` trait.

[Reflect](derive.Reflect.html "derive bevy::reflect::Reflect")

The main derive macro used by `bevy_reflect` for deriving its `Reflect` trait.

[TypePath](derive.TypePath.html "derive bevy::reflect::TypePath")

Derives the `TypePath` trait, providing a stable alternative to \[`std::any::type_name`\].