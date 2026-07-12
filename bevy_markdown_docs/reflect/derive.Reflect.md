[bevy](../index.html)::[reflect](index.html)

# Derive Macro Reflect 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#402)

```rust
#[derive(Reflect)]
{
    // Attributes available to this derive:
    #[reflect]
    #[type_path]
    #[type_name]
}
```

The main derive macro used by `bevy_reflect` for deriving its `Reflect` trait.

This macro can be used on all structs and enums (unions are not supported). It will automatically generate implementations for `Reflect`, `Typed`, `GetTypeRegistration`, and `FromReflect`. And, depending on the item’s structure, will either implement `Struct`, `TupleStruct`, or `Enum`.

See the [`FromReflect`](../prelude/derive.FromReflect.html "derive bevy::prelude::FromReflect") derive macro for more information on how to customize the [`FromReflect`](../prelude/derive.FromReflect.html "derive bevy::prelude::FromReflect") implementation. To implement [`FromReflect`](../prelude/derive.FromReflect.html "derive bevy::prelude::FromReflect") manually while deriving [`Reflect`](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect"), [opt out](#reflectfrom_reflect--false) of the default implementation.

## Container Attributes

This macro comes with some helper attributes that can be added to the container item in order to provide additional functionality or alter the generated implementations.

In addition to those listed, this macro can also use the attributes for [`TypePath`](../prelude/derive.TypePath.html "derive bevy::prelude::TypePath") derives.

### `#[reflect(Ident)]`

The `#[reflect(Ident)]` attribute is used to add type data registrations to the `GetTypeRegistration` implementation corresponding to the given identifier, prepended by `Reflect`.

For example, `#[reflect(Foo, Bar)]` would add two registrations: one for `ReflectFoo` and another for `ReflectBar`. This assumes these types are indeed in-scope wherever this macro is called.

This is often used with traits that have been marked by the [`#[reflect_trait]`](../prelude/attr.reflect_trait.html "attr bevy::prelude::reflect_trait") macro in order to register the type’s implementation of that trait.

#### Default Registrations

The following types are automatically registered when deriving `Reflect`:

*   `ReflectFromReflect` (unless opting out of `FromReflect`)
*   `SerializationData`
*   `ReflectFromPtr`

#### Special Identifiers

There are a few “special” identifiers that work a bit differently:

*   `#[reflect(Clone)]` will force the implementation of `Reflect::reflect_clone` to rely on the type’s \[`Clone`\] implementation. A custom implementation may be provided using `#[reflect(Clone(my_clone_func))]` where `my_clone_func` is the path to a function matching the signature: `(&Self) -> Self`.
*   `#[reflect(Debug)]` will force the implementation of `Reflect::debug` to rely on the type’s \[`Debug`\] implementation. A custom implementation may be provided using `#[reflect(Debug(my_debug_func))]` where `my_debug_func` is the path to a function matching the signature: `(&Self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`.
*   `#[reflect(PartialEq)]` will force the implementation of `Reflect::reflect_partial_eq` to rely on the type’s \[`PartialEq`\] implementation. A custom implementation may be provided using `#[reflect(PartialEq(my_partial_eq_func))]` where `my_partial_eq_func` is the path to a function matching the signature: `(&Self, value: &dyn #bevy_reflect_path::Reflect) -> bool`.
*   `#[reflect(PartialOrd)]` will force the implementation of `PartialReflect::reflect_partial_cmp` to rely on the type’s \[`PartialOrd`\] implementation. A custom implementation may be provided using `#[reflect(PartialOrd(my_partial_cmp_fn))]` where `my_partial_cmp_fn` is the path to a function matching the signature: `(&Self, value: &dyn #bevy_reflect_path::PartialReflect) -> Option<::core::cmp::Ordering>`.
*   `#[reflect(Hash)]` will force the implementation of `Reflect::reflect_hash` to rely on the type’s \[`Hash`\] implementation. A custom implementation may be provided using `#[reflect(Hash(my_hash_func))]` where `my_hash_func` is the path to a function matching the signature: `(&Self) -> u64`.
*   `#[reflect(Default)]` will register the `ReflectDefault` type data as normal. However, it will also affect how certain other operations are performed in order to improve performance and/or robustness. An example of where this is used is in the [`FromReflect`](../prelude/derive.FromReflect.html "derive bevy::prelude::FromReflect") derive macro, where adding this attribute will cause the `FromReflect` implementation to create a base value using its \[`Default`\] implementation avoiding issues with ignored fields (for structs and tuple structs only).

### `#[reflect(opaque)]`

The `#[reflect(opaque)]` attribute denotes that the item should implement `Reflect` as an opaque type, hiding its structure and fields from the reflection API. This means that it will forgo implementing `Struct`, `TupleStruct`, or `Enum`.

Furthermore, it requires that the type implements \[`Clone`\]. If planning to serialize this type using the reflection serializers, then the `Serialize` and `Deserialize` traits will need to be implemented and registered as well.

### `#[reflect(from_reflect = false)]`

This attribute will opt-out of the default `FromReflect` implementation.

This is useful for when a type can’t or shouldn’t implement `FromReflect`, or if a manual implementation is desired.

Note that in the latter case, `ReflectFromReflect` will no longer be automatically registered.

### `#[reflect(type_path = false)]`

This attribute will opt-out of the default `TypePath` implementation.

This is useful for when a type can’t or shouldn’t implement `TypePath`, or if a manual implementation is desired.

### `#[reflect(no_field_bounds)]`

This attribute will opt-out of the default trait bounds added to all field types for the generated reflection trait impls.

Normally, all fields will have the bounds `TypePath`, and either `FromReflect` or `Reflect` depending on if `#[reflect(from_reflect = false)]` is used. However, this might not always be desirable, and so this attribute may be used to remove those bounds.

#### Example

If a type is recursive the default bounds will cause an overflow error when building:

[ⓘ](# "This example is not tested")

```rust
#[derive(Reflect)] // ERROR: overflow evaluating the requirement `Foo: FromReflect`
struct Foo {
  foo: Vec<Foo>,
}

// Generates a where clause like:
// impl bevy_reflect::Reflect for Foo
// where
//   Foo: Any + Send + Sync,
//   Vec<Foo>: FromReflect + TypePath + MaybeTyped + RegisterForReflection,
```

In this case, `Foo` is given the bounds `Vec<Foo>: FromReflect + ...`, which requires that `Foo` implements `FromReflect`, which requires that `Vec<Foo>` implements `FromReflect`, and so on, resulting in the error.

To fix this, we can add `#[reflect(no_field_bounds)]` to `Foo` to remove the bounds on `Vec<Foo>`:

[ⓘ](# "This example is not tested")

```rust
#[derive(Reflect)]
#[reflect(no_field_bounds)]
struct Foo {
  foo: Vec<Foo>,
}

// Generates a where clause like:
// impl bevy_reflect::Reflect for Foo
// where
//   Self: Any + Send + Sync,
```

### `#[reflect(where T: Trait, U::Assoc: Trait, ...)]`

This attribute can be used to add additional bounds to the generated reflection trait impls.

This is useful for when a type needs certain bounds only applied to the reflection impls that are not otherwise automatically added by the derive macro.

#### Example

In the example below, we want to enforce that `T::Assoc: List` is required in order for `Foo<T>` to be reflectable, but we don’t want it to prevent `Foo<T>` from being used in places where `T::Assoc: List` is not required.

[ⓘ](# "This example is not tested")

```rust
trait Trait {
  type Assoc;
}

#[derive(Reflect)]
#[reflect(where T::Assoc: List)]
struct Foo<T: Trait> where T::Assoc: Default {
  value: T::Assoc,
}

// Generates a where clause like:
//
// impl<T: Trait> bevy_reflect::Reflect for Foo<T>
// where
//   Foo<T>: Any + Send + Sync,
//   T::Assoc: Default,
//   T: TypePath,
//   T::Assoc: FromReflect + TypePath + MaybeTyped + RegisterForReflection,
//   T::Assoc: List,
// {/* ... */}
```

### `#[reflect(@...)]`

This attribute can be used to register custom attributes to the type’s `TypeInfo`.

It accepts any expression after the `@` symbol that resolves to a value which implements `Reflect`.

Any number of custom attributes may be registered, however, each the type of each attribute must be unique. If two attributes of the same type are registered, the last one will overwrite the first.

#### Example

[ⓘ](# "This example is not tested")

```rust
#[derive(Reflect)]
struct Required;

#[derive(Reflect)]
struct EditorTooltip(String);

impl EditorTooltip {
  fn new(text: &str) -> Self {
    Self(text.to_string())
  }
}

#[derive(Reflect)]
// Specify a "required" status and tooltip:
#[reflect(@Required, @EditorTooltip::new("An ID is required!"))]
struct Id(u8);
```

### `#[reflect(no_auto_register)]`

This attribute will opt-out of the automatic reflect type registration.

All non-generic types annotated with `#[derive(Reflect)]` are usually automatically registered on app startup. If this behavior is not desired, this attribute may be used to disable it for the annotated type.

## Field Attributes

Along with the container attributes, this macro comes with some attributes that may be applied to the contained fields themselves.

### `#[reflect(ignore)]`

This attribute simply marks a field to be ignored by the reflection API.

This allows fields to completely opt-out of reflection, which may be useful for maintaining invariants, keeping certain data private, or allowing the use of types that do not implement `Reflect` within the container.

### `#[reflect(skip_serializing)]`

This works similar to `#[reflect(ignore)]`, but rather than opting out of _all_ of reflection, it simply opts the field out of both serialization and deserialization. This can be useful when a field should be accessible via reflection, but may not make sense in a serialized form, such as computed data.

What this does is register the `SerializationData` type within the `GetTypeRegistration` implementation, which will be used by the reflection serializers to determine whether or not the field is serializable.

### `#[reflect(clone)]`

This attribute affects the `Reflect::reflect_clone` implementation.

Without this attribute, the implementation will rely on the field’s own `Reflect::reflect_clone` implementation. When this attribute is present, the implementation will instead use the field’s `Clone` implementation directly.

The attribute may also take the path to a custom function like `#[reflect(clone = "path::to::my_clone_func")]`, where `my_clone_func` matches the signature `(&Self) -> Self`.

This attribute does nothing if the containing struct/enum has the `#[reflect(Clone)]` attribute.

### `#[reflect(@...)]`

This attribute can be used to register custom attributes to the field’s `TypeInfo`.

It accepts any expression after the `@` symbol that resolves to a value which implements `Reflect`.

Any number of custom attributes may be registered, however, each the type of each attribute must be unique. If two attributes of the same type are registered, the last one will overwrite the first.

#### Example

[ⓘ](# "This example is not tested")

```rust
#[derive(Reflect)]
struct EditorTooltip(String);

impl EditorTooltip {
  fn new(text: &str) -> Self {
    Self(text.to_string())
  }
}

#[derive(Reflect)]
struct Slider {
  // Specify a custom range and tooltip:
  #[reflect(@0.0..=1.0, @EditorTooltip::new("Must be between 0 and 1"))]
  value: f32,
}
```