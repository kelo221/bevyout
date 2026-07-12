[bevy](../../../index.html)::[render](../../index.html)::[render\_resource](../index.html)

# Module encase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/mod.rs.html#68)

## Modules

[internal](internal/index.html "mod bevy::render::render_resource::encase::internal")

[matrix](matrix/index.html "mod bevy::render::render_resource::encase::matrix")

Module containing items necessary to implement `ShaderType` for matrices

[rts\_array](rts_array/index.html "mod bevy::render::render_resource::encase::rts_array")

Module containing items necessary to implement `ShaderType` for runtime-sized arrays

[vector](vector/index.html "mod bevy::render::render_resource::encase::vector")

Module containing items necessary to implement `ShaderType` for vectors

## Macros

[impl\_matrix](macro.impl_matrix.html "macro bevy::render::render_resource::encase::impl_matrix")

Used to implement `ShaderType` for the given matrix type

[impl\_rts\_array](macro.impl_rts_array.html "macro bevy::render::render_resource::encase::impl_rts_array")

Used to implement `ShaderType` for the given runtime-sized array type

[impl\_vector](macro.impl_vector.html "macro bevy::render::render_resource::encase::impl_vector")

Used to implement `ShaderType` for the given vector type

[impl\_wrapper](macro.impl_wrapper.html "macro bevy::render::render_resource::encase::impl_wrapper")

Used to implement `ShaderType` for the given wrapper type

## Structs

[ArrayLength](struct.ArrayLength.html "struct bevy::render::render_resource::encase::ArrayLength")

Helper type meant to be used together with the [`ShaderType`](https://docs.rs/encase_derive/0.12.0/x86_64-unknown-linux-gnu/encase_derive/derive.ShaderType.html "derive encase_derive::ShaderType") derive macro

[DynamicStorageBuffer](struct.DynamicStorageBuffer.html "struct bevy::render::render_resource::encase::DynamicStorageBuffer")

Dynamic storage buffer wrapper facilitating RW operations

[DynamicUniformBuffer](struct.DynamicUniformBuffer.html "struct bevy::render::render_resource::encase::DynamicUniformBuffer")

Dynamic uniform buffer wrapper facilitating RW operations

[StorageBuffer](struct.StorageBuffer.html "struct bevy::render::render_resource::encase::StorageBuffer")

Storage buffer wrapper facilitating RW operations

[UniformBuffer](struct.UniformBuffer.html "struct bevy::render::render_resource::encase::UniformBuffer")

Uniform buffer wrapper facilitating RW operations

## Traits

[CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor")

Trait implemented for [WGSL runtime-sized arrays](https://gpuweb.github.io/gpuweb/wgsl/#runtime-sized) and [WGSL structs containing runtime-sized arrays](https://gpuweb.github.io/gpuweb/wgsl/#struct-types) (non fixed-footprint types)

[ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::encase::ShaderSize")

Trait implemented for all [WGSL fixed-footprint types](https://gpuweb.github.io/gpuweb/wgsl/#fixed-footprint-types)

[ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::encase::ShaderType")

Base trait for all [WGSL host-shareable types](https://gpuweb.github.io/gpuweb/wgsl/#host-shareable-types)

## Derive Macros

[ShaderType](derive.ShaderType.html "derive bevy::render::render_resource::encase::ShaderType")