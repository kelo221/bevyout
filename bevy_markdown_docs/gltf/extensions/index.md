[bevy](../../index.html)::[gltf](../index.html)

# Module extensions 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#1)

glTF extensions defined by the Khronos Group and other vendors

## Structs

[GltfExtensionHandlers](struct.GltfExtensionHandlers.html "struct bevy::gltf::extensions::GltfExtensionHandlers")

Stores the `ErasedGltfExtensionHandler` implementations so that they can be added by users and also passed to the glTF loader

## Traits

[ErasedGltfExtensionHandler](trait.ErasedGltfExtensionHandler.html "trait bevy::gltf::extensions::ErasedGltfExtensionHandler")

Type-erased version of [`GltfExtensionHandler`](trait.GltfExtensionHandler.html "trait bevy::gltf::extensions::GltfExtensionHandler"). This is used to store heterogeneous handlers in a collection.

[GltfExtensionHandler](trait.GltfExtensionHandler.html "trait bevy::gltf::extensions::GltfExtensionHandler")

glTF Extensions can attach data to any objects in a glTF file. This is done by inserting data in the `extensions` sub-object, and data in the extensions sub-object is keyed by the id of the extension. For example: `KHR_materials_variants`, `EXT_meshopt_compression`, or `BEVY_my_tool`