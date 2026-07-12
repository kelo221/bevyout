[bevy](../../index.html)::[render](../index.html)

# Module batching 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#39)

## Modules

[gpu\_preprocessing](gpu_preprocessing/index.html "mod bevy::render::batching::gpu_preprocessing")

Batching functionality when GPU preprocessing is in use.

[no\_gpu\_preprocessing](no_gpu_preprocessing/index.html "mod bevy::render::batching::no_gpu_preprocessing")

Batching functionality when GPU preprocessing isn’t in use.

## Structs

[NoAutomaticBatching](struct.NoAutomaticBatching.html "struct bevy::render::batching::NoAutomaticBatching")

Add this component to mesh entities to disable automatic batching

## Traits

[GetBatchData](trait.GetBatchData.html "trait bevy::render::batching::GetBatchData")

A trait to support getting data used for batching draw commands via phase items.

[GetFullBatchData](trait.GetFullBatchData.html "trait bevy::render::batching::GetFullBatchData")

A trait to support getting data used for batching draw commands via phase items.

## Functions

[sort\_binned\_render\_phase](fn.sort_binned_render_phase.html "fn bevy::render::batching::sort_binned_render_phase")

Sorts a render phase that uses bins.