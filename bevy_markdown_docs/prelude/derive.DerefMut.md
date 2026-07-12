[bevy](../index.html)::[prelude](index.html)

# Derive Macro DerefMut 

[Source](https://docs.rs/bevy_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_derive/lib.rs.html#188)

```rust
#[derive(DerefMut)]
{
    // Attributes available to this derive:
    #[deref]
}
```

Implements [`DerefMut`](std::ops::DerefMut) for structs. This is especially useful when utilizing the [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern.

For single-field structs, the implementation automatically uses that field. For multi-field structs, you must specify which field to use with the `#[deref]` attribute.

[`DerefMut`](std::ops::DerefMut) requires a [`Deref`](std::ops::Deref) implementation. You can implement it manually or use Bevy’s [derive](derive.Deref.html "derive bevy::prelude::Deref") macro for convenience.

## Example

### Tuple Structs

Using a single-field struct:

```rust
use bevy_derive::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct MyNewtype(String);

let mut foo = MyNewtype(String::from("Hello"));
foo.push_str(" World!");
assert_eq!("Hello World!", *foo);
```

Using a multi-field struct:

```rust
use bevy_derive::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct MyStruct<T>(#[deref] String, PhantomData<T>);

let mut foo = MyStruct(String::from("Hello"), PhantomData::<usize>);
foo.push_str(" World!");
assert_eq!("Hello World!", *foo);
```

### Named Structs

Using a single-field struct:

```rust
use bevy_derive::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct MyStruct {
  value: String,
}

let mut foo = MyStruct {
  value: String::from("Hello")
};
foo.push_str(" World!");
assert_eq!("Hello World!", *foo);
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

let mut foo = MyStruct {
  value:String::from("Hello"),
  _phantom:PhantomData::<usize>
};
foo.push_str(" World!");
assert_eq!("Hello World!", *foo);
```