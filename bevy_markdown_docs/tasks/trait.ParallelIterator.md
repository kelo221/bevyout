[bevy](../index.html)::[tasks](index.html)

# Trait ParallelIterator 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#15)

```rust
pub trait ParallelIterator<BatchIter>: Sized + Sendwhere
    BatchIter: Iterator + Send,{
    // Required method
    fn next_batch(&mut self) -> Option<BatchIter>;

    // Provided methods
    fn size_hint(&self) -> (usize, Option<usize>) { ... }
    fn count(self, pool: &TaskPool) -> usize { ... }
    fn last(self, _pool: &TaskPool) -> Option<<BatchIter as Iterator>::Item> { ... }
    fn nth(
        self,
        _pool: &TaskPool,
        n: usize,
    ) -> Option<<BatchIter as Iterator>::Item> { ... }
    fn chain<U>(self, other: U) -> Chain<Self, U>
       where U: ParallelIterator<BatchIter> { ... }
    fn map<T, F>(self, f: F) -> Map<Self, F>
       where F: FnMut(<BatchIter as Iterator>::Item) -> T + Send + Clone { ... }
    fn for_each<F>(self, pool: &TaskPool, f: F)
       where F: FnMut(<BatchIter as Iterator>::Item) + Send + Clone + Sync { ... }
    fn filter<F>(self, predicate: F) -> Filter<Self, F>
       where F: FnMut(&<BatchIter as Iterator>::Item) -> bool { ... }
    fn filter_map<R, F>(self, f: F) -> FilterMap<Self, F>
       where F: FnMut(<BatchIter as Iterator>::Item) -> Option<R> { ... }
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, F>
       where F: FnMut(<BatchIter as Iterator>::Item) -> U,
             U: IntoIterator { ... }
    fn flatten(self) -> Flatten<Self>
       where <BatchIter as Iterator>::Item: IntoIterator { ... }
    fn fuse(self) -> Fuse<Self> { ... }
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
       where F: FnMut(&<BatchIter as Iterator>::Item) { ... }
    fn by_ref(&mut self) -> &mut Self { ... }
    fn collect<C>(self, pool: &TaskPool) -> C
       where C: FromIterator<<BatchIter as Iterator>::Item>,
             <BatchIter as Iterator>::Item: Send + 'static { ... }
    fn partition<C, F>(self, pool: &TaskPool, f: F) -> (C, C)
       where C: Default + Extend<<BatchIter as Iterator>::Item> + Send,
             F: FnMut(&<BatchIter as Iterator>::Item) -> bool + Send + Sync + Clone,
             <BatchIter as Iterator>::Item: Send + 'static { ... }
    fn fold<C, F, D>(self, pool: &TaskPool, init: C, f: F) -> Vec<C>
       where F: FnMut(C, <BatchIter as Iterator>::Item) -> C + Send + Sync + Clone,
             C: Clone + Send + Sync + 'static { ... }
    fn all<F>(self, pool: &TaskPool, f: F) -> bool
       where F: FnMut(<BatchIter as Iterator>::Item) -> bool + Send + Sync + Clone { ... }
    fn any<F>(self, pool: &TaskPool, f: F) -> bool
       where F: FnMut(<BatchIter as Iterator>::Item) -> bool + Send + Sync + Clone { ... }
    fn position<F>(self, pool: &TaskPool, f: F) -> Option<usize>
       where F: FnMut(<BatchIter as Iterator>::Item) -> bool + Send + Sync + Clone { ... }
    fn max(self, pool: &TaskPool) -> Option<<BatchIter as Iterator>::Item>
       where <BatchIter as Iterator>::Item: Ord + Send + 'static { ... }
    fn min(self, pool: &TaskPool) -> Option<<BatchIter as Iterator>::Item>
       where <BatchIter as Iterator>::Item: Ord + Send + 'static { ... }
    fn max_by_key<R, F>(
        self,
        pool: &TaskPool,
        f: F,
    ) -> Option<<BatchIter as Iterator>::Item>
       where R: Ord,
             F: FnMut(&<BatchIter as Iterator>::Item) -> R + Send + Sync + Clone,
             <BatchIter as Iterator>::Item: Send + 'static { ... }
    fn max_by<F>(
        self,
        pool: &TaskPool,
        f: F,
    ) -> Option<<BatchIter as Iterator>::Item>
       where F: FnMut(&<BatchIter as Iterator>::Item, &<BatchIter as Iterator>::Item) -> Ordering + Send + Sync + Clone,
             <BatchIter as Iterator>::Item: Send + 'static { ... }
    fn min_by_key<R, F>(
        self,
        pool: &TaskPool,
        f: F,
    ) -> Option<<BatchIter as Iterator>::Item>
       where R: Ord,
             F: FnMut(&<BatchIter as Iterator>::Item) -> R + Send + Sync + Clone,
             <BatchIter as Iterator>::Item: Send + 'static { ... }
    fn min_by<F>(
        self,
        pool: &TaskPool,
        f: F,
    ) -> Option<<BatchIter as Iterator>::Item>
       where F: FnMut(&<BatchIter as Iterator>::Item, &<BatchIter as Iterator>::Item) -> Ordering + Send + Sync + Clone,
             <BatchIter as Iterator>::Item: Send + 'static { ... }
    fn copied<'a, T>(self) -> Copied<Self>
       where Self: ParallelIterator<BatchIter>,
             T: 'a + Copy { ... }
    fn cloned<'a, T>(self) -> Cloned<Self>
       where Self: ParallelIterator<BatchIter>,
             T: 'a + Copy { ... }
    fn cycle(self) -> Cycle<Self>
       where Self: Clone { ... }
    fn sum<S, R>(self, pool: &TaskPool) -> R
       where S: Sum<<BatchIter as Iterator>::Item> + Send + 'static,
             R: Sum<S> { ... }
    fn product<S, R>(self, pool: &TaskPool) -> R
       where S: Product<<BatchIter as Iterator>::Item> + Send + 'static,
             R: Product<S> { ... }
}
```

[`ParallelIterator`](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator") closely emulates the `std::iter::Iterator` interface. However, it uses `bevy_task` to compute batches in parallel.

Note that the overhead of [`ParallelIterator`](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator") is high relative to some workloads. In particular, if the batch size is too small or task being run in parallel is inexpensive, _a [`ParallelIterator`](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator") could take longer than a normal [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")_. Therefore, you should profile your code before using [`ParallelIterator`](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator").

## Required Methods

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#24)

#### fn [next\_batch](#tymethod.next_batch)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<BatchIter>

Returns the next batch of items for processing.

Each batch is an iterator with items of the same type as the [`ParallelIterator`](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator"). Returns `None` when there are no batches left.

## Provided Methods

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#30)

#### fn [size\_hint](#method.size_hint)(&self) -> ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>)

Returns the bounds on the remaining number of items in the parallel iterator.

See [`Iterator::size_hint()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#37)

#### fn [count](#method.count)(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Consumes the parallel iterator and returns the number of items.

See [`Iterator::count()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#50)

#### fn [last](#method.last)(self, \_pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

Consumes the parallel iterator and returns the last item.

See [`Iterator::last()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#62)

#### fn [nth](#method.nth)( self, \_pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

Consumes the parallel iterator and returns the nth item.

See [`Iterator::nth()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#80-82)

#### fn [chain](#method.chain)<U>(self, other: U) -> Chain<Self, U>

where U: [ParallelIterator](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator")<BatchIter>,

Takes two parallel iterators and returns a parallel iterators over both in sequence.

See [`Iterator::chain()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#95-97)

#### fn [map](#method.map)<T, F>(self, f: F) -> Map<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> T + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Takes a closure and creates a parallel iterator which calls that closure on each item.

See [`Iterator::map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#105-107)

#### fn [for\_each](#method.for_each)<F>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

Calls a closure on each item of a parallel iterator.

See [`Iterator::for_each()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#123-125)

#### fn [filter](#method.filter)<F>(self, predicate: F) -> Filter<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Creates a parallel iterator which uses a closure to determine if an element should be yielded.

See [`Iterator::filter()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#136-138)

#### fn [filter\_map](#method.filter_map)<R, F>(self, f: F) -> FilterMap<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<R>,

Creates a parallel iterator that both filters and maps.

See [`Iterator::filter_map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#147-150)

#### fn [flat\_map](#method.flat_map)<U, F>(self, f: F) -> FlatMap<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> U, U: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Creates a parallel iterator that works like map, but flattens nested structure.

See [`Iterator::flat_map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#158-160)

#### fn [flatten](#method.flatten)(self) -> Flatten<Self>

where <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Creates a parallel iterator that flattens nested structure.

See [`Iterator::flatten()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#168)

#### fn [fuse](#method.fuse)(self) -> Fuse<Self>

Creates a parallel iterator which ends after the first None.

See [`Iterator::fuse()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#176-178)

#### fn [inspect](#method.inspect)<F>(self, f: F) -> Inspect<Self, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

Does something with each item of a parallel iterator, passing the value on.

See [`Iterator::inspect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#186)

#### fn [by\_ref](#method.by_ref)(&mut self) -> &mut Self

Borrows a parallel iterator, rather than consuming it.

See [`Iterator::by_ref()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#194-197)

#### fn [collect](#method.collect)<C>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> C

where C: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Transforms a parallel iterator into a collection.

See [`Iterator::collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#213-217)

#### fn [partition](#method.partition)<C, F>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F) -> [(C, C)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Consumes a parallel iterator, creating two collections from it.

See [`Iterator::partition()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#241-244)

#### fn [fold](#method.fold)<C, F, D>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), init: C, f: F) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<C>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(C, <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> C + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), C: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Repeatedly applies a function to items of each batch of a parallel iterator, producing a Vec of final values.

_Note that this folds each batch independently and returns a Vec of results (in batch order)._

See [`Iterator::fold()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#260-262)

#### fn [all](#method.all)<F>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Tests if every element of the parallel iterator matches a predicate.

_Note that all is **not** short circuiting._

See [`Iterator::all()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#279-281)

#### fn [any](#method.any)<F>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Tests if any element of the parallel iterator matches a predicate.

_Note that any is **not** short circuiting._

See [`Iterator::any()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#299-301)

#### fn [position](#method.position)<F>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Searches for an element in a parallel iterator, returning its index.

_Note that position consumes the whole iterator._

See [`Iterator::position()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.position)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#332-334)

#### fn [max](#method.max)(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Returns the maximum item of a parallel iterator.

See [`Iterator::max()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#349-351)

#### fn [min](#method.min)(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Returns the minimum item of a parallel iterator.

See [`Iterator::min()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#366-370)

#### fn [max\_by\_key](#method.max_by_key)<R, F>( self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Returns the item that gives the maximum value from the specified function.

See [`Iterator::max_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#387-390)

#### fn [max\_by](#method.max_by)<F>( self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Returns the item that gives the maximum value with respect to the specified comparison function.

See [`Iterator::max_by()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#406-410)

#### fn [min\_by\_key](#method.min_by_key)<R, F>( self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where R: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> R + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Returns the item that gives the minimum value from the specified function.

See [`Iterator::min_by_key()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#427-430)

#### fn [min\_by](#method.min_by)<F>( self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool"), f: F, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"), &<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), <BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

Returns the item that gives the minimum value with respect to the specified comparison function.

See [`Iterator::min_by()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#446-449)

#### fn [copied](#method.copied)<'a, T>(self) -> Copied<Self>

where Self: [ParallelIterator](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator")<BatchIter>, T: 'a + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

Creates a parallel iterator which copies all of its items.

See [`Iterator::copied()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#457-460)

#### fn [cloned](#method.cloned)<'a, T>(self) -> Cloned<Self>

where Self: [ParallelIterator](trait.ParallelIterator.html "trait bevy::tasks::ParallelIterator")<BatchIter>, T: 'a + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

Creates a parallel iterator which clones all of its items.

See [`Iterator::cloned()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#468-470)

#### fn [cycle](#method.cycle)(self) -> Cycle<Self>

where Self: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Repeats a parallel iterator endlessly.

See [`Iterator::cycle()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cycle)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#481-484)

#### fn [sum](#method.sum)<S, R>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> R

where S: [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, R: [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<S>,

Sums the items of a parallel iterator.

See [`Iterator::sum()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/iter/mod.rs.html#498-501)

#### fn [product](#method.product)<S, R>(self, pool: &[TaskPool](struct.TaskPool.html "struct bevy::tasks::TaskPool")) -> R

where S: [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<<BatchIter as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, R: [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<S>,

Multiplies all the items of a parallel iterator.

See [`Iterator::product()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors