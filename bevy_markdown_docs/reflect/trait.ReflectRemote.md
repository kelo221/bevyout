[bevy](../index.html)::[reflect](index.html)

# Trait ReflectRemote 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#47)

```rust
pub trait ReflectRemote: Reflect {
    type Remote;

    // Required methods
    fn as_remote(&self) -> &Self::Remote;
    fn as_remote_mut(&mut self) -> &mut Self::Remote;
    fn into_remote(self) -> Self::Remote;
    fn as_wrapper(remote: &Self::Remote) -> &Self;
    fn as_wrapper_mut(remote: &mut Self::Remote) -> &mut Self;
    fn into_wrapper(remote: Self::Remote) -> Self;
}
```

Marks a type as a [reflectable](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") wrapper for a remote type.

This allows types from external libraries (remote types) to be included in reflection.

## Safety

It is highly recommended to avoid implementing this trait manually and instead use the [`#[reflect_remote]`](attr.reflect_remote.html "attr bevy::reflect::reflect_remote") attribute macro. This is because the trait tends to rely on [`transmute`](https://doc.rust-lang.org/nightly/core/intrinsics/fn.transmute.html "fn core::intrinsics::transmute"), which is [very unsafe](https://doc.rust-lang.org/1.71.0/nomicon/transmutes.html).

The macro will ensure that the following safety requirements are met:

*   `Self` is a single-field tuple struct (i.e. a newtype) containing the remote type.
*   `Self` is `#[repr(transparent)]` over the remote type.

Additionally, the macro will automatically generate [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") and [`FromReflect`](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") implementations, along with compile-time assertions to validate that the safety requirements have been met.

## Example

```rust
use bevy_reflect_derive::{reflect_remote, Reflect};

mod some_lib {
  pub struct TheirType {
    pub value: u32
  }
}

#[reflect_remote(some_lib::TheirType)]
struct MyType {
  pub value: u32
}

#[derive(Reflect)]
struct MyStruct {
  #[reflect(remote = MyType)]
  data: some_lib::TheirType,
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#49)

#### type [Remote](#associatedtype.Remote)

The remote type this type represents via reflection.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#52)

#### fn [as\_remote](#tymethod.as_remote)(&self) -> &Self::[Remote](trait.ReflectRemote.html#associatedtype.Remote "type bevy::reflect::ReflectRemote::Remote")

Converts a reference of this wrapper to a reference of its remote type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#54)

#### fn [as\_remote\_mut](#tymethod.as_remote_mut)(&mut self) -> &mut Self::[Remote](trait.ReflectRemote.html#associatedtype.Remote "type bevy::reflect::ReflectRemote::Remote")

Converts a mutable reference of this wrapper to a mutable reference of its remote type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#56)

#### fn [into\_remote](#tymethod.into_remote)(self) -> Self::[Remote](trait.ReflectRemote.html#associatedtype.Remote "type bevy::reflect::ReflectRemote::Remote")

Converts this wrapper into its remote type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#59)

#### fn [as\_wrapper](#tymethod.as_wrapper)(remote: &Self::[Remote](trait.ReflectRemote.html#associatedtype.Remote "type bevy::reflect::ReflectRemote::Remote")) -> &Self

Converts a reference of the remote type to a reference of this wrapper.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#61)

#### fn [as\_wrapper\_mut](#tymethod.as_wrapper_mut)(remote: &mut Self::[Remote](trait.ReflectRemote.html#associatedtype.Remote "type bevy::reflect::ReflectRemote::Remote")) -> &mut Self

Converts a mutable reference of the remote type to a mutable reference of this wrapper.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/remote.rs.html#63)

#### fn [into\_wrapper](#tymethod.into_wrapper)(remote: Self::[Remote](trait.ReflectRemote.html#associatedtype.Remote "type bevy::reflect::ReflectRemote::Remote")) -> Self

Converts the remote type into this wrapper.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors