[bevy](../../index.html)::[tasks](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#112)

The tasks prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AsyncComputeTaskPool](struct.AsyncComputeTaskPool.html "struct bevy::tasks::prelude::AsyncComputeTaskPool")

A newtype for a task pool for CPU-intensive work that may span across multiple frames

[ComputeTaskPool](struct.ComputeTaskPool.html "struct bevy::tasks::prelude::ComputeTaskPool")

A newtype for a task pool for CPU-intensive work that must be completed to deliver the next frame

[IoTaskPool](struct.IoTaskPool.html "struct bevy::tasks::prelude::IoTaskPool")

A newtype for a task pool for IO-intensive work (i.e. tasks that spend very little time in a “woken” state)

## Traits

[ParallelIterator](trait.ParallelIterator.html "trait bevy::tasks::prelude::ParallelIterator")

[`ParallelIterator`](../trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator") closely emulates the `std::iter::Iterator` interface. However, it uses `bevy_task` to compute batches in parallel.

[ParallelSlice](trait.ParallelSlice.html "trait bevy::tasks::prelude::ParallelSlice")

Provides functions for mapping read-only slices across a provided [`TaskPool`](../struct.TaskPool.html "struct bevy::tasks::TaskPool").

[ParallelSliceMut](trait.ParallelSliceMut.html "trait bevy::tasks::prelude::ParallelSliceMut")

Provides functions for mapping mutable slices across a provided [`TaskPool`](../struct.TaskPool.html "struct bevy::tasks::TaskPool").

## Functions

[block\_on](fn.block_on.html "fn bevy::tasks::prelude::block_on")

Blocks the current thread on a future, processing I/O events when idle.