[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)

# Module matrix 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/lib.rs.html#131)

Module containing items necessary to implement `ShaderType` for matrices

## Macros

[impl\_matrix](macro.impl_matrix.html "macro bevy::render::render_resource::encase::matrix::impl_matrix")

Used to implement `ShaderType` for the given matrix type

## Traits

[AsMutMatrixParts](trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")

Enables writing to the matrix (via `&mut [[T; R]; C]`)

[AsRefMatrixParts](trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")

Enables reading from the matrix (via `&[[T; R]; C]`)

[FromMatrixParts](trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")

Enables the creation of a matrix (via `[[T; R]; C]`)

[MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar")