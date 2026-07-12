[bevy](../../index.html)::[reflect](../index.html)::[prelude](index.html)

# Trait IntoFunctionMut 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/into_function_mut.rs.html#25)

```rust
pub trait IntoFunctionMut<'env, Marker> {
    // Required method
    fn into_function_mut(self) -> DynamicFunctionMut<'env>;
}
```

A trait for types that can be converted into a [`DynamicFunctionMut`](../func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

This trait is automatically implemented for any type that implements [`ReflectFnMut`](../func/trait.ReflectFnMut.html "trait bevy::reflect::func::ReflectFnMut") and [`TypedFunction`](../func/trait.TypedFunction.html "trait bevy::reflect::func::TypedFunction").

This trait can be seen as a superset of [`IntoFunction`](../../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction").

See the [module-level documentation](../func/index.html "mod bevy::reflect::func") for more information.

## Trait Parameters

This trait has a `Marker` type parameter that is used to get around issues with [unconstrained type parameters](https://doc.rust-lang.org/error_codes/E0207.html) when defining impls with generic arguments or return types. This `Marker` can be any type, provided it doesn’t conflict with other implementations.

Additionally, it has a lifetime parameter, `'env`, that is used to bound the lifetime of the function. For named functions and some closures, this will end up just being `'static`, however, closures that borrow from their environment will have a lifetime bound to that environment.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/into_function_mut.rs.html#27)

#### fn [into\_function\_mut](#tymethod.into_function_mut)(self) -> [DynamicFunctionMut](../func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut")<'env>

Converts [`Self`](../../prelude/trait.IntoFunctionMut.html "trait bevy::prelude::IntoFunctionMut") into a [`DynamicFunctionMut`](../func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/into_function_mut.rs.html#30-32)

### impl<'env, F, Marker1, Marker2> [IntoFunctionMut](../../prelude/trait.IntoFunctionMut.html "trait bevy::prelude::IntoFunctionMut")<'env, [(Marker1, Marker2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for F

where F: [ReflectFnMut](../func/trait.ReflectFnMut.html "trait bevy::reflect::func::ReflectFnMut")<'env, Marker1> + [TypedFunction](../func/trait.TypedFunction.html "trait bevy::reflect::func::TypedFunction")<Marker2> + 'env,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/dynamic_function.rs.html#470)

### impl<'env> [IntoFunctionMut](../../prelude/trait.IntoFunctionMut.html "trait bevy::prelude::IntoFunctionMut")<'env, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [DynamicFunction](../func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'env>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/dynamic_function_mut.rs.html#371)

### impl<'env> [IntoFunctionMut](../../prelude/trait.IntoFunctionMut.html "trait bevy::prelude::IntoFunctionMut")<'env, [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [DynamicFunctionMut](../func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut")<'env>