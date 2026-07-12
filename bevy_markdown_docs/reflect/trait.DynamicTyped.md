[bevy](../index.html)::[reflect](index.html)

# Trait DynamicTyped 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#160)

```rust
pub trait DynamicTyped {
    // Required method
    fn reflect_type_info(&self) -> &'static TypeInfo;
}
```

Dynamic dispatch for [`Typed`](trait.Typed.html "trait bevy::reflect::Typed").

Since this is a supertrait of [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") its methods can be called on a `dyn Reflect`.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#162)

#### fn [reflect\_type\_info](#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](trait.Typed.html "trait bevy::reflect::Typed"),