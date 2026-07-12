[bevy](../index.html)

# Crate tasks 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#1-139)

## Bevy Tasks

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license) [![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy_tasks) [![Downloads](https://img.shields.io/crates/d/bevy_tasks.svg)](https://crates.io/crates/bevy_tasks) [![Docs](https://docs.rs/bevy_tasks/badge.svg)](https://docs.rs/bevy_tasks/latest/bevy_tasks/) [![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/bevy)

A refreshingly simple task executor for bevy. :)

This is a simple threadpool with minimal dependencies. The main usecase is a scoped fork-join, i.e. spawning tasks from a single thread and having that thread await the completion of those tasks. This is intended specifically for [`bevy`](https://bevy.org) as a lighter alternative to [`rayon`](https://github.com/rayon-rs/rayon) for this specific usecase. There are also utilities for generating the tasks from a slice of data. This library is intended for games and makes no attempt to ensure fairness or ordering of spawned tasks.

It is based on [`async-executor`](https://github.com/stjepang/async-executor), a lightweight executor that allows the end user to manage their own threads. `async-executor` is based on async-task, a core piece of async-std.

### Usage

In order to be able to optimize task execution in multi-threaded environments, bevy provides three different thread pools via which tasks of different kinds can be spawned. (The same API is used in single-threaded environments, even if execution is limited to a single thread. This currently applies to Wasm targets.) The determining factor for what kind of work should go in each pool is latency requirements:

*   For CPU-intensive work (tasks that generally spin until completion) we have a standard [`ComputeTaskPool`](struct.ComputeTaskPool.html "struct bevy::tasks::ComputeTaskPool") and an [`AsyncComputeTaskPool`](struct.AsyncComputeTaskPool.html "struct bevy::tasks::AsyncComputeTaskPool"). Work that does not need to be completed to present the next frame should go to the [`AsyncComputeTaskPool`](struct.AsyncComputeTaskPool.html "struct bevy::tasks::AsyncComputeTaskPool").
    
*   For IO-intensive work (tasks that spend very little time in a “woken” state) we have an [`IoTaskPool`](struct.IoTaskPool.html "struct bevy::tasks::IoTaskPool") whose tasks are expected to complete very quickly. Generally speaking, they should just await receiving data from somewhere (i.e. disk) and signal other systems when the data is ready for consumption. (likely via channels)
    

### `no_std` Support

To enable `no_std` support in this crate, you will need to disable default features, and enable the `edge_executor` and `critical-section` features.

## Modules

[cfg](cfg/index.html "mod bevy::tasks::cfg")

Configuration information for this crate.

[futures](futures/index.html "mod bevy::tasks::futures")

Utilities for working with [`Future`](futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future")s.

[futures\_lite](futures_lite/index.html "mod bevy::tasks::futures_lite")

Futures, streams, and async I/O combinators.

[prelude](prelude/index.html "mod bevy::tasks::prelude")

The tasks prelude.

## Structs

[AsyncComputeTaskPool](struct.AsyncComputeTaskPool.html "struct bevy::tasks::AsyncComputeTaskPool")

A newtype for a task pool for CPU-intensive work that may span across multiple frames

[ComputeTaskPool](struct.ComputeTaskPool.html "struct bevy::tasks::ComputeTaskPool")

A newtype for a task pool for CPU-intensive work that must be completed to deliver the next frame

[IoTaskPool](struct.IoTaskPool.html "struct bevy::tasks::IoTaskPool")

A newtype for a task pool for IO-intensive work (i.e. tasks that spend very little time in a “woken” state)

[Scope](struct.Scope.html "struct bevy::tasks::Scope")

A [`TaskPool`](struct.TaskPool.html "struct bevy::tasks::TaskPool") scope for running one or more non-`'static` futures.

[Task](struct.Task.html "struct bevy::tasks::Task")

A spawned task.

[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")

A thread pool for executing tasks.

[TaskPoolBuilder](struct.TaskPoolBuilder.html "struct bevy::tasks::TaskPoolBuilder")

Used to create a [`TaskPool`](struct.TaskPool.html "struct bevy::tasks::TaskPool")

[ThreadExecutor](struct.ThreadExecutor.html "struct bevy::tasks::ThreadExecutor")

An executor that can only be ticked on the thread it was instantiated on. But can spawn `Send` tasks from other threads.

[ThreadExecutorTicker](struct.ThreadExecutorTicker.html "struct bevy::tasks::ThreadExecutorTicker")

Used to tick the [`ThreadExecutor`](struct.ThreadExecutor.html "struct bevy::tasks::ThreadExecutor"). The executor does not make progress unless it is manually ticked on the thread it was created on.

## Traits

[ConditionalSend](trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend")

Use [`ConditionalSend`](trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") to mark an optional Send trait bound. Useful as on certain platforms (eg. Wasm), futures aren’t Send.

[ConditionalSendFuture](trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Use [`ConditionalSendFuture`](trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture") for a future with an optional Send trait bound, as on certain platforms (eg. Wasm), futures aren’t Send.

[ParallelIterator](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator")

[`ParallelIterator`](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator") closely emulates the `std::iter::Iterator` interface. However, it uses `bevy_task` to compute batches in parallel.

[ParallelSlice](trait.ParallelSlice.html "trait bevy::tasks::ParallelSlice")

Provides functions for mapping read-only slices across a provided [`TaskPool`](struct.TaskPool.html "struct bevy::tasks::TaskPool").

[ParallelSliceMut](trait.ParallelSliceMut.html "trait bevy::tasks::ParallelSliceMut")

Provides functions for mapping mutable slices across a provided [`TaskPool`](struct.TaskPool.html "struct bevy::tasks::TaskPool").

## Functions

[available\_parallelism](fn.available_parallelism.html "fn bevy::tasks::available_parallelism")

Gets the logical CPU core count available to the current process.

[block\_on](fn.block_on.html "fn bevy::tasks::block_on")

Blocks the current thread on a future, processing I/O events when idle.

[poll\_once](fn.poll_once.html "fn bevy::tasks::poll_once")

Polls a future just once and returns an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") with the result.

[tick\_global\_task\_pools\_on\_main\_thread](fn.tick_global_task_pools_on_main_thread.html "fn bevy::tasks::tick_global_task_pools_on_main_thread")

A function used by `bevy_app` to tick the global tasks pools on the main thread. This will run a maximum of 100 local tasks per executor per call to this function.

## Type Aliases

[BoxedFuture](type.BoxedFuture.html "type bevy::tasks::BoxedFuture")

An owned and dynamically typed Future used when you can’t statically type your result or need to add some indirection.