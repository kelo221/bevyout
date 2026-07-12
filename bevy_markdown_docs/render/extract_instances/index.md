[bevy](../../index.html)::[render](../index.html)

# Module extract\_instances 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#45)

Convenience logic for turning components from the main world into extracted instances in the render world.

This is essentially the same as the `extract_component` module, but higher-performance because it avoids the ECS overhead.

## Structs

[ExtractInstancesPlugin](struct.ExtractInstancesPlugin.html "struct bevy::render::extract_instances::ExtractInstancesPlugin")

This plugin extracts one or more components into the “render world” as extracted instances.

[ExtractedInstances](struct.ExtractedInstances.html "struct bevy::render::extract_instances::ExtractedInstances")

Stores all extract instances of a type in the render world.

## Traits

[ExtractInstance](trait.ExtractInstance.html "trait bevy::render::extract_instances::ExtractInstance")

Describes how to extract data needed for rendering from a component or components.