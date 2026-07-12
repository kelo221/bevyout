[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[matrix](index.html)

# Macro impl\_matrix 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#52)

```rust
macro_rules! impl_matrix {
    ($c:literal, $r:literal, $type:ty $( ; using $($using:tt)* )?) => { ... };
    ($c:literal, $r:literal, $type:ty; ($($generics:tt)*) $( ; using $($using:tt)* )?) => { ... };
    ($c:literal, $r:literal, $type:ty, $el_ty:ty $( ; using $($using:tt)* )?) => { ... };
}
```

Used to implement `ShaderType` for the given matrix type

The given matrix type should implement any combination of [`AsRefMatrixParts`](trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts"), [`AsMutMatrixParts`](trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts"), [`FromMatrixParts`](trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts") depending on needed capability (they can also be derived via `$using`)

## Args

*   `$c` nr of columns the given matrix contains
    
*   `$r` nr of rows the given matrix contains
    
*   `$type` the type (representing a matrix) for which `ShaderType` will be implemented for
    
*   `$generics` \[optional\] generics that will be passed into the `impl< >`
    
*   `$el_type` \[optional\] inner element type of the matrix (should implement [`MatrixScalar`](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"))
    
*   `$using` \[optional\] can be any combination of `AsRef AsMut From`