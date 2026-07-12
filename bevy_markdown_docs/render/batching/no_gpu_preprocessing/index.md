[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)

# Module no\_gpu\_preprocessing 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#21)

Batching functionality when GPU preprocessing isn’t in use.

## Structs

[BatchedInstanceBuffer](struct.BatchedInstanceBuffer.html "struct bevy::render::batching::no_gpu_preprocessing::BatchedInstanceBuffer")

The GPU buffers holding the data needed to render batches.

## Functions

[batch\_and\_prepare\_binned\_render\_phase](fn.batch_and_prepare_binned_render_phase.html "fn bevy::render::batching::no_gpu_preprocessing::batch_and_prepare_binned_render_phase")

Creates batches for a render phase that uses bins, when GPU batch data building isn’t in use.

[batch\_and\_prepare\_sorted\_render\_phase](fn.batch_and_prepare_sorted_render_phase.html "fn bevy::render::batching::no_gpu_preprocessing::batch_and_prepare_sorted_render_phase")

Batch the items in a sorted render phase, when GPU instance buffer building isn’t in use. This means comparing metadata needed to draw each phase item and trying to combine the draws into a batch.

[clear\_batched\_cpu\_instance\_buffers](fn.clear_batched_cpu_instance_buffers.html "fn bevy::render::batching::no_gpu_preprocessing::clear_batched_cpu_instance_buffers")

A system that clears out the [`BatchedInstanceBuffer`](struct.BatchedInstanceBuffer.html "struct bevy::render::batching::no_gpu_preprocessing::BatchedInstanceBuffer") for the frame.

[write\_batched\_instance\_buffer](fn.write_batched_instance_buffer.html "fn bevy::render::batching::no_gpu_preprocessing::write_batched_instance_buffer")

Writes the instance buffer data to the GPU.