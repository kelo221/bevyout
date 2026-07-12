[bevy](../../index.html)::[render](../index.html)

# Module extract\_plugin 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#47)

## Structs

[ExtractPlugin](struct.ExtractPlugin.html "struct bevy::render::extract_plugin::ExtractPlugin")

Plugin that sets up the [`RenderApp`](../struct.RenderApp.html "struct bevy::render::RenderApp") and handles extracting data from the main world to the render world.

[ExtractSchedule](struct.ExtractSchedule.html "struct bevy::render::extract_plugin::ExtractSchedule")

Schedule in which data from the main world is ‘extracted’ into the render world.

[MainWorld](struct.MainWorld.html "struct bevy::render::extract_plugin::MainWorld")

The simulation [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") of the application, stored as a resource.

## Functions

[extract](fn.extract.html "fn bevy::render::extract_plugin::extract")

Executes the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step of the renderer. This updates the render world with the extracted ECS data of the current frame.