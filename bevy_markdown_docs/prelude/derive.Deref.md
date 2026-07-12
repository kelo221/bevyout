[bevy](../index.html)::[prelude](index.html)

# Derive Macro Deref 

[Source](https://docs.rs/bevy_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_derive/lib.rs.html#100)

```rust
#[derive(Deref)]
{
    // Attributes available to this derive:
    #[deref]
}
```

Implements [`Deref`](std::ops::Deref) for structs. This is especially useful when utilizing the [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern.

For single-field structs, the implementation automatically uses that field. For multi-field structs, you must specify which field to use with the `#[deref]` attribute.

If you need [`DerefMut`](std::ops::DerefMut) as well, consider using the other [derive](derive.DerefMut.html "derive bevy::prelude::DerefMut") macro alongside this one.

## Example

### Tuple Structs

Using a single-field struct:

```rust
use bevy_derive::Deref;

#[derive(Deref)]
struct MyNewtype(String);

let foo = MyNewtype(String::from("Hello"));
assert_eq!("Hello", *foo);
```

Using a multi-field struct:

```rust
use bevy_derive::Deref;

#[derive(Deref)]
struct MyStruct<T>(#[deref] String, PhantomData<T>);

let foo = MyStruct(String::from("Hello"), PhantomData::<usize>);
assert_eq!("Hello", *foo);
```

### Named Structs

Using a single-field struct:

```rust
use bevy_derive::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct MyStruct {
  value: String,
}

let foo = MyStruct {
  value: String::from("Hello")
};
assert_eq!("Hello", *foo);
```

Using a multi-field struct:

```rust
use bevy_derive::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct MyStruct<T> {
  #[deref]
  value: String,
  _phantom: PhantomData<T>,
}

let foo = MyStruct {
  value:String::from("Hello"),
  _phantom:PhantomData::<usize>
};
assert_eq!("Hello", *foo);
```