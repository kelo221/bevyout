[bevy](../../index.html)::[render](../index.html)

# Module slab\_allocator 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#61)

A general-purpose allocator that manages a set of GPU buffer slabs.

## Structs

[AllocationStage](struct.AllocationStage.html "struct bevy::render::slab_allocator::AllocationStage")

An object that allows batched allocation.

[DeallocationStage](struct.DeallocationStage.html "struct bevy::render::slab_allocator::DeallocationStage")

An object that enables batched deallocation.

[GeneralSlab](struct.GeneralSlab.html "struct bevy::render::slab_allocator::GeneralSlab")

A resizable slab that can contain multiple objects.

[LargeObjectSlab](struct.LargeObjectSlab.html "struct bevy::render::slab_allocator::LargeObjectSlab")

A slab that contains a single object.

[SlabAllocationBufferSlice](struct.SlabAllocationBufferSlice.html "struct bevy::render::slab_allocator::SlabAllocationBufferSlice")

The hardware buffer that slab-allocated data lives in, as well as the range within that buffer.

[SlabAllocator](struct.SlabAllocator.html "struct bevy::render::slab_allocator::SlabAllocator")

A general-purpose allocator that manages a set of GPU buffer slabs.

[SlabAllocatorSettings](struct.SlabAllocatorSettings.html "struct bevy::render::slab_allocator::SlabAllocatorSettings")

Tunable parameters that customize the behavior of the allocator.

[SlabId](struct.SlabId.html "struct bevy::render::slab_allocator::SlabId")

The index of a single slab.

[SlabToReallocate](struct.SlabToReallocate.html "struct bevy::render::slab_allocator::SlabToReallocate")

Holds information about a slab that’s scheduled to be allocated or reallocated.

## Enums

[Slab](enum.Slab.html "enum bevy::render::slab_allocator::Slab")

Data for a single slab.

## Traits

[SlabItem](trait.SlabItem.html "trait bevy::render::slab_allocator::SlabItem")

Describes the type of the data that a [`SlabAllocator`](struct.SlabAllocator.html "struct bevy::render::slab_allocator::SlabAllocator") will store.

[SlabItemLayout](trait.SlabItemLayout.html "trait bevy::render::slab_allocator::SlabItemLayout")

A trait that defines information necessary to determine the size and alignment of objects within a slab.