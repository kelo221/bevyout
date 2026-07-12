[bevy](../index.html)

# Crate shader 

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/lib.rs.html#1-39)

Provides [`Shader`](../prelude/struct.Shader.html "struct bevy::prelude::Shader") assets for Bevy.

## Modules

[prelude](prelude/index.html "mod bevy::shader::prelude")

The shader prelude.

## Macros

[load\_shader\_library](macro.load_shader_library.html "macro bevy::shader::load_shader_library")

Inline shader as an `embedded_asset` and load it permanently.

## Structs

[Shader](struct.Shader.html "struct bevy::shader::Shader")

An “unprocessed” shader. It can contain preprocessor directives and imports.

[ShaderCache](struct.ShaderCache.html "struct bevy::shader::ShaderCache")

A cache for shaders and shader imports, with asset state-tracking for waiting to load shaders until all imports are resolved.

[ShaderId](struct.ShaderId.html "struct bevy::shader::ShaderId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[ShaderLoader](struct.ShaderLoader.html "struct bevy::shader::ShaderLoader")

The [`AssetLoader`](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") responsible for loading unprocessed shader assets.

[ShaderResolver](struct.ShaderResolver.html "struct bevy::shader::ShaderResolver")`shader_format_wesl`

A Wesl import resolver. Maps module paths to actual Wesl shader source.

[ShaderSettings](struct.ShaderSettings.html "struct bevy::shader::ShaderSettings")

Settings for loading shaders.

## Enums

[ShaderCacheError](enum.ShaderCacheError.html "enum bevy::shader::ShaderCacheError")

Type of error returned by a `PipelineCache` when the creation of a GPU pipeline object failed.

[ShaderCacheSource](enum.ShaderCacheSource.html "enum bevy::shader::ShaderCacheSource")

Fully composed source code of a shader module, with all shader defs applied.

[ShaderDefVal](enum.ShaderDefVal.html "enum bevy::shader::ShaderDefVal")

A compile time shader value definition to be inlined into the shader source. Variant tuples contain the name of the definition, and the value.

[ShaderImport](enum.ShaderImport.html "enum bevy::shader::ShaderImport")

A shader import, described as either an asset path or an import path.

[ShaderLoaderError](enum.ShaderLoaderError.html "enum bevy::shader::ShaderLoaderError")

An error encountered while loading a shader’s source.

[ShaderRef](enum.ShaderRef.html "enum bevy::shader::ShaderRef")

A reference to a shader asset.

[Source](enum.Source.html "enum bevy::shader::Source")

Raw shader source code.

[ValidateShader](enum.ValidateShader.html "enum bevy::shader::ValidateShader")

Describes whether or not to perform runtime checks on shaders. Runtime checks can be enabled for safety at the cost of speed. By default no runtime checks will be performed.

## Type Aliases

[CachedPipelineId](type.CachedPipelineId.html "type bevy::shader::CachedPipelineId")

An id of a pipeline, typically in the [`PipelineCache`](https://docs.rs/bevy/latest/bevy/render/render_resource/struct.PipelineCache.html) Typically corresponds to a unique combination of [`Shader`](../prelude/struct.Shader.html "struct bevy::prelude::Shader") and [`ShaderDefVal`](enum.ShaderDefVal.html "enum bevy::shader::ShaderDefVal")s.