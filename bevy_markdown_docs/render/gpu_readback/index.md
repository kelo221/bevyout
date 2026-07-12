[bevy](../../index.html)::[render](../index.html)

# Module gpu\_readback 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#51)

## Structs

[GpuReadbackPlugin](struct.GpuReadbackPlugin.html "struct bevy::render::gpu_readback::GpuReadbackPlugin")

A plugin that enables reading back gpu buffers and textures to the cpu.

[ReadbackComplete](struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

An event that is triggered when a gpu readback is complete.

## Enums

[Readback](enum.Readback.html "enum bevy::render::gpu_readback::Readback")

A component that registers the wrapped handle for gpu readback, either a texture or a buffer.

[ReadbackTemplate](enum.ReadbackTemplate.html "enum bevy::render::gpu_readback::ReadbackTemplate")