[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[vector](index.html)

# Macro impl\_vector 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/vector.rs.html#39)

```rust
macro_rules! impl_vector {
    ($n:literal, $type:ty $( ; using $($using:tt)* )?) => { ... };
    ($n:literal, $type:ty; ($($generics:tt)*) $( ; using $($using:tt)* )?) => { ... };
    ($n:literal, $type:ty, $el_ty:ty $( ; using $($using:tt)* )?) => { ... };
}
```

Used to implement `ShaderType` for the given vector type

The given vector type should implement any combination of [`AsRefVectorParts`](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts"), [`AsMutVectorParts`](trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts"), [`FromVectorParts`](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts") depending on needed capability (they can also be derived via `$using`)

## Args

*   `$n` nr of elements the given vector contains
    
*   `$type` the type (representing a vector) for which `ShaderType` will be implemented for
    
*   `$generics` \[optional\] generics that will be passed into the `impl< >`
    
*   `$el_type` \[optional\] inner element type of the vector (should implement [`VectorScalar`](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"))
    
*   `$using` \[optional\] can be any combination of `AsRef AsMut From`