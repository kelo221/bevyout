[bevy](../../index.html)::[reflect](../index.html)::[func](index.html)

# Type Alias FunctionResult 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/error.rs.html#44)

```rust
pub type FunctionResult<'a> = Result<Return<'a>, FunctionError>;
```

Available on **crate feature `functions`** only.

The result of calling a [`DynamicFunction`](struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

Returns `Ok(value)` if the function was called successfully, where `value` is the [`Return`](enum.Return.html "enum bevy::reflect::func::Return") value of the function.

## Aliased Type

```rust
pub enum FunctionResult<'a> {
    Ok(Return<'a>),
    Err(FunctionError),
}
```

## Variants

1.0.0

### Ok([Return](enum.Return.html "enum bevy::reflect::func::Return")<'a>)

Contains the success value

1.0.0

### Err([FunctionError](enum.FunctionError.html "enum bevy::reflect::func::FunctionError"))

Contains the error value