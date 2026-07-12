[bevy](../../../index.html)::[render](../../index.html)::[render\_resource](../index.html)::[encase](index.html)

# Macro impl\_rts\_array 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#84)

```rust
macro_rules! impl_rts_array {
    ($type:ty $( ; using $($using:tt)* )?) => { ... };
    ($type:ty; ($($generics:tt)*) $( ; using $($using:tt)* )?) => { ... };
}
```

Used to implement `ShaderType` for the given runtime-sized array type

The given runtime-sized array type should implement [`Length`](rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") and optionally [`Truncate`](rts_array/trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") depending on needed capability (they can also be derived via `$using`)

## Args

*   `$type` the type (representing a runtime-sized array) for which `ShaderType` will be implemented for
    
*   `$generics` \[optional\] generics that will be passed into the `impl< >`
    
*   `$using` \[optional\] can be any combination of `len truncate`