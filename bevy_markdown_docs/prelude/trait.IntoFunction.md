[bevy](../index.html)::[prelude](index.html)

# Trait IntoFunction 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/into_function.rs.html#22)

```rust
pub trait IntoFunction<'env, Marker> {
    // Required method
    fn into_function(self) -> DynamicFunction<'env>;
}
```

A trait for types that can be converted into a [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction").

This trait is automatically implemented for any type that implements [`ReflectFn`](../reflect/func/trait.ReflectFn.html "trait bevy::reflect::func::ReflectFn") and [`TypedFunction`](../reflect/func/trait.TypedFunction.html "trait bevy::reflect::func::TypedFunction").

See the [module-level documentation](../reflect/func/index.html "mod bevy::reflect::func") for more information.

## Trait Parameters

This trait has a `Marker` type parameter that is used to get around issues with [unconstrained type parameters](https://doc.rust-lang.org/error_codes/E0207.html) when defining impls with generic arguments or return types. This `Marker` can be any type, provided it doesn’t conflict with other implementations.

Additionally, it has a lifetime parameter, `'env`, that is used to bound the lifetime of the function. For named functions and some closures, this will end up just being `'static`, however, closures that borrow from their environment will have a lifetime bound to that environment.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/into_function.rs.html#24)

#### fn [into\_function](#tymethod.into_function)(self) -> [DynamicFunction](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'env>

Converts [`Self`](trait.IntoFunction.html "trait bevy::prelude::IntoFunction") into a [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/into_function.rs.html#27-29)

### impl<'env, F, Marker1, Marker2> [IntoFunction](trait.IntoFunction.html "trait bevy::prelude::IntoFunction")<'env, [(Marker1, Marker2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for F

where F: [ReflectFn](../reflect/func/trait.ReflectFn.html "trait bevy::reflect::func::ReflectFn")<'env, Marker1> + [TypedFunction](../reflect/func/trait.TypedFunction.html "trait bevy::reflect::func::TypedFunction")<Marker2> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'env,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/dynamic_function.rs.html#463)

### impl<'env> [IntoFunction](trait.IntoFunction.html "trait bevy::prelude::IntoFunction")<'env, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [DynamicFunction](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'env>