[bevy](../../../index.html)::[render](../../index.html)::[mesh](../index.html)

# Module allocator 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/mod.rs.html#1)

Manages mesh vertex and index buffers.

## Structs

[ElementLayout](struct.ElementLayout.html "struct bevy::render::mesh::allocator::ElementLayout")

Information about the size of individual elements (vertices or indices) within a slab.

[MeshAllocationKey](struct.MeshAllocationKey.html "struct bevy::render::mesh::allocator::MeshAllocationKey")

The handle used to retrieve a single mesh allocation.

[MeshAllocator](struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")

Manages the assignment of mesh data to GPU buffers.

[MeshAllocatorPlugin](struct.MeshAllocatorPlugin.html "struct bevy::render::mesh::allocator::MeshAllocatorPlugin")

A plugin that manages GPU memory for mesh data.

[MeshAllocatorSettings](struct.MeshAllocatorSettings.html "struct bevy::render::mesh::allocator::MeshAllocatorSettings")

Tunable parameters that customize the behavior of the allocator.

[MeshSlabItem](struct.MeshSlabItem.html "struct bevy::render::mesh::allocator::MeshSlabItem")

The [`SlabItem`](../../slab_allocator/trait.SlabItem.html "trait bevy::render::slab_allocator::SlabItem") implementation that describes the information needed to allocate and free meshes.

[MeshSlabs](struct.MeshSlabs.html "struct bevy::render::mesh::allocator::MeshSlabs")

IDs of the slabs associated with a single mesh.

## Enums

[ElementClass](enum.ElementClass.html "enum bevy::render::mesh::allocator::ElementClass")

The type of element that a mesh slab can store.

## Functions

[allocate\_and\_free\_meshes](fn.allocate_and_free_meshes.html "fn bevy::render::mesh::allocator::allocate_and_free_meshes")

A system that processes newly-extracted or newly-removed meshes and writes their data into buffers or frees their data as appropriate.

## Type Aliases

[MeshBufferSlice](type.MeshBufferSlice.html "type bevy::render::mesh::allocator::MeshBufferSlice")

The slab buffer and location within that slab in which each mesh is allocated.

[MeshSlabId](type.MeshSlabId.html "type bevy::render::mesh::allocator::MeshSlabId")

The ID of a single slab.