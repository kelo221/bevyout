[bevy](../../index.html)::[render](../index.html)

# Module extract\_component 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#44)

## Structs

[ComponentUniforms](struct.ComponentUniforms.html "struct bevy::render::extract_component::ComponentUniforms")

Stores all uniforms of the component type.

[DynamicUniformIndex](struct.DynamicUniformIndex.html "struct bevy::render::extract_component::DynamicUniformIndex")

Stores the index of a uniform inside of [`ComponentUniforms`](struct.ComponentUniforms.html "struct bevy::render::extract_component::ComponentUniforms").

[ExtractComponentPlugin](struct.ExtractComponentPlugin.html "struct bevy::render::extract_component::ExtractComponentPlugin")

This plugin extracts the components into the render world for synced entities. To do so, it sets up the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step for the specified [`ExtractComponent`](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent").

[UniformComponentPlugin](struct.UniformComponentPlugin.html "struct bevy::render::extract_component::UniformComponentPlugin")

This plugin prepares the components of the corresponding type for the GPU by transforming them into uniforms.

## Traits

[ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent")

Describes how a component gets extracted for rendering.

## Derive Macros

[ExtractComponent](derive.ExtractComponent.html "derive bevy::render::extract_component::ExtractComponent")

Implements `ExtractComponent` trait for a component.