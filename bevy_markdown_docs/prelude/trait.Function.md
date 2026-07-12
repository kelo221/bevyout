[bevy](../index.html)::[prelude](index.html)

# Trait Function 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/function.rs.html#38)

```rust
pub trait Function: PartialReflect + Debug {
    // Required methods
    fn name(&self) -> Option<&Cow<'static, str>>;
    fn info(&self) -> &FunctionInfo;
    fn reflect_call<'a>(
        &self,
        args: ArgList<'a>,
    ) -> Result<Return<'a>, FunctionError>;
    fn to_dynamic_function(&self) -> DynamicFunction<'static>;

    // Provided method
    fn arg_count(&self) -> ArgCount { ... }
}
```

A trait used to power [function-like](../reflect/func/index.html "mod bevy::reflect::func") operations via [reflection](trait.Reflect.html "trait bevy::prelude::Reflect").

This trait allows types to be called like regular functions with [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect")\-based [arguments](../reflect/func/args/index.html "mod bevy::reflect::func::args") and return values.

By default, this trait is currently only implemented for [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction"), however, it is possible to implement this trait for custom function-like types.

## Example

```rust
fn add(a: i32, b: i32) -> i32 {
   a + b
}

let func: Box<dyn Function> = Box::new(add.into_function());
let args = ArgList::new().with_owned(25_i32).with_owned(75_i32);
let value = func.reflect_call(args).unwrap().unwrap_owned();
assert_eq!(value.try_take::<i32>().unwrap(), 100);
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/function.rs.html#48)

#### fn [name](#tymethod.name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>

The name of the function, if any.

For [`DynamicFunctions`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") created using [`IntoFunction`](trait.IntoFunction.html "trait bevy::prelude::IntoFunction"), the default name will always be the full path to the function as returned by [`core::any::type_name`](https://doc.rust-lang.org/nightly/core/any/fn.type_name.html "fn core::any::type_name"), unless the function is a closure, anonymous function, or function pointer, in which case the name will be `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/function.rs.html#61)

#### fn [info](#tymethod.info)(&self) -> &[FunctionInfo](../reflect/func/struct.FunctionInfo.html "struct bevy::reflect::func::FunctionInfo")

The [`FunctionInfo`](../reflect/func/struct.FunctionInfo.html "struct bevy::reflect::func::FunctionInfo") for this function.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/function.rs.html#64)

#### fn [reflect\_call](#tymethod.reflect_call)<'a>( &self, args: [ArgList](../reflect/func/struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'a>, [FunctionError](../reflect/func/enum.FunctionError.html "enum bevy::reflect::func::FunctionError")\>

Call this function with the given arguments.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/function.rs.html#67)

#### fn [to\_dynamic\_function](#tymethod.to_dynamic_function)(&self) -> [DynamicFunction](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'static>

Creates a new [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") from this function.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/function.rs.html#56)

#### fn [arg\_count](#method.arg_count)(&self) -> [ArgCount](../reflect/func/args/struct.ArgCount.html "struct bevy::reflect::func::args::ArgCount")

Returns the number of arguments the function expects.

For [overloaded](../reflect/func/index.html#overloading-functions "mod bevy::reflect::func") functions that can have a variable number of arguments, this will contain the full set of counts for all signatures.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/dynamic_function.rs.html#348)

### impl [Function](trait.Function.html "trait bevy::prelude::Function") for [DynamicFunction](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction")<'static>