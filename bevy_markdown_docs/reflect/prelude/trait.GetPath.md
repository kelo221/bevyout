[bevy](../../index.html)::[reflect](../index.html)::[prelude](index.html)

# Trait GetPath 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#251)

```rust
pub trait GetPath: PartialReflect {
    // Provided methods
    fn reflect_path<'p>(
        &self,
        path: impl ReflectPath<'p>,
    ) -> Result<&(dyn PartialReflect + 'static), ReflectPathError<'p>> { ... }
    fn reflect_path_mut<'p>(
        &mut self,
        path: impl ReflectPath<'p>,
    ) -> Result<&mut (dyn PartialReflect + 'static), ReflectPathError<'p>> { ... }
    fn path<'p, T>(
        &self,
        path: impl ReflectPath<'p>,
    ) -> Result<&T, ReflectPathError<'p>>
       where T: Reflect { ... }
    fn path_mut<'p, T>(
        &mut self,
        path: impl ReflectPath<'p>,
    ) -> Result<&mut T, ReflectPathError<'p>>
       where T: Reflect { ... }
}
```

A trait which allows nested [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") values to be retrieved with path strings.

Using these functions repeatedly with the same string requires parsing the string every time. To avoid this cost, it’s recommended to construct a [`ParsedPath`](../struct.ParsedPath.html "struct bevy::reflect::ParsedPath") instead.

## Syntax

### Structs

Field paths for [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") elements use the standard Rust field access syntax of dot and field name: `.field_name`.

Additionally, struct fields may be accessed by their index within the struct’s definition. This is accomplished by using the hash symbol (`#`) in place of the standard dot: `#0`.

Accessing a struct’s field by index can speed up fetches at runtime due to the removed need for string matching. And while this can be more performant, it’s best to keep in mind the tradeoffs when utilizing such optimizations. For example, this can result in fairly fragile code as the string paths will need to be kept in sync with the struct definitions since the order of fields could be easily changed. Because of this, storing these kinds of paths in persistent storage (i.e. game assets) is strongly discouraged.

Note that a leading dot (`.`) or hash (`#`) token is implied for the first item in a path, and may therefore be omitted.

Additionally, an empty path may be used to get the struct itself.

#### Example

```rust
#[derive(Reflect, PartialEq, Debug)]
struct MyStruct {
  value: u32
}

let my_struct = MyStruct { value: 123 };
// Access via field name
assert_eq!(my_struct.path::<u32>(".value").unwrap(), &123);
// Access via field index
assert_eq!(my_struct.path::<u32>("#0").unwrap(), &123);
// Access self
assert_eq!(*my_struct.path::<MyStruct>("").unwrap(), my_struct);
```

### Tuples and Tuple Structs

[`Tuple`](../tuple/trait.Tuple.html "trait bevy::reflect::tuple::Tuple") and [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") elements also follow a conventional Rust syntax. Fields are accessed with a dot and the field index: `.0`.

Note that a leading dot (`.`) token is implied for the first item in a path, and may therefore be omitted.

#### Example

```rust
#[derive(Reflect)]
struct MyTupleStruct(u32);

let my_tuple_struct = MyTupleStruct(123);
assert_eq!(my_tuple_struct.path::<u32>(".0").unwrap(), &123);
```

### Lists and Arrays

[`List`](../list/trait.List.html "trait bevy::reflect::list::List") and [`Array`](../array/trait.Array.html "trait bevy::reflect::array::Array") elements are accessed with brackets: `[0]`.

#### Example

```rust
let my_list: Vec<u32> = vec![1, 2, 3];
assert_eq!(my_list.path::<u32>("[2]").unwrap(), &3);
```

### Enums

Pathing for [`Enum`](../enums/trait.Enum.html "trait bevy::reflect::enums::Enum") elements works a bit differently than in normal Rust. Usually, you would need to pattern match an enum, branching off on the desired variants. Paths used by this trait do not have any pattern matching capabilities; instead, they assume the variant is already known ahead of time.

The syntax used, therefore, depends on the variant being accessed:

*   Struct variants use the struct syntax (outlined above)
*   Tuple variants use the tuple syntax (outlined above)
*   Unit variants have no fields to access

If the variant cannot be known ahead of time, the path will need to be split up and proper enum pattern matching will need to be handled manually.

#### Example

```rust
#[derive(Reflect)]
enum MyEnum {
  Unit,
  Tuple(bool),
  Struct {
    value: u32
  }
}

let tuple_variant = MyEnum::Tuple(true);
assert_eq!(tuple_variant.path::<bool>(".0").unwrap(), &true);

let struct_variant = MyEnum::Struct { value: 123 };
// Access via field name
assert_eq!(struct_variant.path::<u32>(".value").unwrap(), &123);
// Access via field index
assert_eq!(struct_variant.path::<u32>("#0").unwrap(), &123);

// Error: Expected struct variant
assert!(matches!(tuple_variant.path::<u32>(".value"), Err(_)));
```

## Chaining

Using the aforementioned syntax, path items may be chained one after another to create a full path to a nested element.

### Example

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

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`.

To retrieve a statically typed reference, use [`path`](../../prelude/trait.GetPath.html#method.path "method bevy::prelude::GetPath::path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`.

To retrieve a statically typed mutable reference, use [`path_mut`](../../prelude/trait.GetPath.html#method.path_mut "method bevy::prelude::GetPath::path_mut").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](#method.path)<'p, T>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`.

This will automatically handle downcasting to type `T`. The downcast will fail if this value is not of type `T` (which may be the case when using dynamic types like [`DynamicStruct`](../structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")).

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`.

This will automatically handle downcasting to type `T`. The downcast will fail if this value is not of type `T` (which may be the case when using dynamic types like [`DynamicStruct`](../structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")).

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),