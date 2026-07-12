[bevy](../index.html)::[tasks](index.html)

# Trait ParallelSliceMut 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#105)

```rust
pub trait ParallelSliceMut<T>: AsMut<[T]>where
    T: Send,{
    // Provided methods
    fn par_chunk_map_mut<F, R>(
        &mut self,
        task_pool: &TaskPool,
        chunk_size: usize,
        f: F,
    ) -> Vec<R>
       where F: Fn(usize, &mut [T]) -> R + Send + Sync,
             R: Send + 'static { ... }
    fn par_splat_map_mut<F, R>(
        &mut self,
        task_pool: &TaskPool,
        max_tasks: Option<usize>,
        f: F,
    ) -> Vec<R>
       where F: Fn(usize, &mut [T]) -> R + Send + Sync,
             R: Send + 'static { ... }
}
```

Provides functions for mapping mutable slices across a provided [`TaskPool`](struct.TaskPool.html "struct bevy::tasks::TaskPool").

## Provided Methods

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#140-143)

#### fn [par\_chunk\_map\_mut](#method.par_chunk_map_mut)<F, R>( &mut self, task\_pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), chunk\_size: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), f: F, ) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<R>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> R + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), R: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Splits the slice in chunks of size `chunks_size` or less and maps the chunks in parallel across the provided `task_pool`. One task is spawned in the task pool for every chunk.

The iteration function takes the index of the chunk in the original slice as the first argument, and the chunk as the second argument.

Returns a `Vec` of the mapped results in the same order as the input.

##### Example

```rust
let task_pool = TaskPool::new();
let mut counts = (0..10000).collect::<Vec<u32>>();
let incremented = counts.par_chunk_map_mut(&task_pool, 100, |_index, chunk| {
  let mut results = Vec::new();
  for count in chunk {
    *count += 5;
    results.push(*count - 2);
  }
  results
});

assert_eq!(counts, (5..10005).collect::<Vec<u32>>());
```

##### See Also

[`ParallelSlice::par_chunk_map`](trait.ParallelSlice.html#method.par_chunk_map "method bevy::tasks::ParallelSlice::par_chunk_map") for mapping immutable slices. [`ParallelSliceMut::par_splat_map_mut`](trait.ParallelSliceMut.html#method.par_splat_map_mut "method bevy::tasks::ParallelSliceMut::par_splat_map_mut") for mapping when a specific chunk size is unknown.

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#190-198)

#### fn [par\_splat\_map\_mut](#method.par_splat_map_mut)<F, R>( &mut self, task\_pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), max\_tasks: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, f: F, ) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<R>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)) -> R + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), R: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Splits the slice into a maximum of `max_tasks` chunks, and maps the chunks in parallel across the provided `task_pool`. One task is spawned in the task pool for every chunk.

If `max_tasks` is `None`, this function will attempt to use one chunk per thread in `task_pool`.

The iteration function takes the index of the chunk in the original slice as the first argument, and the chunk as the second argument.

Returns a `Vec` of the mapped results in the same order as the input.

##### Example

```rust
let task_pool = TaskPool::new();
let mut counts = (0..10000).collect::<Vec<u32>>();
let incremented = counts.par_splat_map_mut(&task_pool, None, |_index, chunk| {
  let mut results = Vec::new();
  for count in chunk {
    *count += 5;
    results.push(*count - 2);
  }
  results
});

assert_eq!(counts, (5..10005).collect::<Vec<u32>>());
```

##### See Also

[`ParallelSlice::par_splat_map`](trait.ParallelSlice.html#method.par_splat_map "method bevy::tasks::ParallelSlice::par_splat_map") for mapping immutable slices. [`ParallelSliceMut::par_chunk_map_mut`](trait.ParallelSliceMut.html#method.par_chunk_map_mut "method bevy::tasks::ParallelSliceMut::par_chunk_map_mut") for mapping when a specific chunk size is desirable.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/slice.rs.html#213)

### impl<S, T> [ParallelSliceMut](trait.ParallelSliceMut.html "trait bevy::tasks::ParallelSliceMut")<T> for S

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), S: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>,