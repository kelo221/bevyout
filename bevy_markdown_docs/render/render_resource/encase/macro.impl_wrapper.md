[bevy](../../../index.html)::[render](../../index.html)::[render\_resource](../index.html)::[encase](index.html)

# Macro impl\_wrapper 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#12)

```rust
macro_rules! impl_wrapper {
    ($type:ty; using $($using:tt)*) => { ... };
    ($type:ty; ($($generics:tt)*); using $($using:tt)*) => { ... };
}
```

Used to implement `ShaderType` for the given wrapper type

## Args

*   `$type` the type (representing a wrapper) for which `ShaderType` will be implemented for
    
*   `$generics` \[optional\] generics that will be passed into the `impl< >`
    
*   `$using` \[optional\] can be any combination of `Ref{ X } Mut{ X } From{ X }` (where `X` denotes a possible function call)