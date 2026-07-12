[bevy](../index.html)::[reflect](index.html)

# Attribute Macro reflect\_remote 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#636)

```rust
#[reflect_remote]
```

Generates a wrapper type that can be used to “derive `Reflect`” for remote types.

This works by wrapping the remote type in a generated wrapper that has the `#[repr(transparent)]` attribute. This allows the two types to be safely [transmuted](std::mem::transmute) back-and-forth.

## Defining the Wrapper

Before defining the wrapper type, please note that it is _required_ that all fields of the remote type are public. The generated code will, at times, need to access or mutate them, and we do not currently have a way to assign getters/setters to each field (but this may change in the future).

The wrapper definition should match the remote type 1-to-1. This includes the naming and ordering of the fields and variants.

Generics and lifetimes do _not_ need to have the same names, however, they _do_ need to follow the same order. Additionally, whether generics are inlined or placed in a where clause should not matter.

Lastly, all macros and doc-comments should be placed **below** this attribute. If they are placed above, they will not be properly passed to the generated wrapper type.

## Example

Given a remote type, `RemoteType`:

```rust
#[derive(Default)]
struct RemoteType<T>
where
  T: Default + Clone,
{
  pub foo: T,
  pub bar: usize
}
```

We would define our wrapper type as such:

[ⓘ](# "This example is not tested")

```rust
use external_crate::RemoteType;

#[reflect_remote(RemoteType<T>)]
#[derive(Default)]
pub struct WrapperType<T: Default + Clone> {
  pub foo: T,
  pub bar: usize
}
```

Apart from all the reflection trait implementations, this generates something like the following:

[ⓘ](# "This example is not tested")

```rust
use external_crate::RemoteType;

#[derive(Default)]
#[repr(transparent)]
pub struct Wrapper<T: Default + Clone>(RemoteType<T>);
```

## Usage as a Field

You can tell `Reflect` to use a remote type’s wrapper internally on fields of a struct or enum. This allows the real type to be used as usual while `Reflect` handles everything internally. To do this, add the `#[reflect(remote = path::to::MyType)]` attribute to your field:

[ⓘ](# "This example is not tested")

```rust
#[derive(Reflect)]
struct SomeStruct {
  #[reflect(remote = RemoteTypeWrapper)]
  data: RemoteType
}
```

### Safety

When using the `#[reflect(remote = path::to::MyType)]` field attribute, be sure you are defining the correct wrapper type. Internally, this field will be unsafely [transmuted](std::mem::transmute), and is only sound if using a wrapper generated for the remote type. This also means keeping your wrapper definitions up-to-date with the remote types.