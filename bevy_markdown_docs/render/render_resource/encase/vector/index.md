[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)

# Module vector 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/lib.rs.html#122)

Module containing items necessary to implement `ShaderType` for vectors

## Macros

[impl\_vector](macro.impl_vector.html "macro bevy::render::render_resource::encase::vector::impl_vector")

Used to implement `ShaderType` for the given vector type

## Traits

[AsMutVectorParts](trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")

Enables writing to the vector (via `&mut [T; N]`)

[AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")

Enables reading from the vector (via `&[T; N]`)

[FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")

Enables the creation of a vector (via `[T; N]`)

[VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar")