[bevy](../../index.html)::[gltf](../index.html)

# Module vertex\_attributes 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/lib.rs.html#136)

A set of utilities for accessing and converting vertex attribute data

## Enums

[AccessFailed](enum.AccessFailed.html "enum bevy::gltf::vertex_attributes::AccessFailed")

An error that occurs when accessing buffer data

[ConvertAttributeError](enum.ConvertAttributeError.html "enum bevy::gltf::vertex_attributes::ConvertAttributeError")

Errors that can occur during the `convert_attribute` function.

## Functions

[convert\_attribute](fn.convert_attribute.html "fn bevy::gltf::vertex_attributes::convert_attribute")

map glTF vertex attributes into their `MeshVertexAttribute` forms, optionally converting values if necessary.