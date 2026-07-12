[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)

# Module stream 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/lib.rs.html#59)

Combinators for the [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") trait.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![1, 2, 3]);

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(3));
assert_eq!(s.next().await, None);
```

## Structs

[AllFuture](struct.AllFuture.html "struct bevy::tasks::futures_lite::stream::AllFuture")

Future for the [`StreamExt::all()`](../trait.StreamExt.html#method.all "method bevy::tasks::futures_lite::StreamExt::all") method.

[AnyFuture](struct.AnyFuture.html "struct bevy::tasks::futures_lite::stream::AnyFuture")

Future for the [`StreamExt::any()`](../trait.StreamExt.html#method.any "method bevy::tasks::futures_lite::StreamExt::any") method.

[BlockOn](struct.BlockOn.html "struct bevy::tasks::futures_lite::stream::BlockOn")

Iterator for the [`block_on()`](fn.block_on.html "fn bevy::tasks::futures_lite::stream::block_on") function.

[Chain](struct.Chain.html "struct bevy::tasks::futures_lite::stream::Chain")

Stream for the [`StreamExt::chain()`](../trait.StreamExt.html#method.chain "method bevy::tasks::futures_lite::StreamExt::chain") method.

[Cloned](struct.Cloned.html "struct bevy::tasks::futures_lite::stream::Cloned")

Stream for the [`StreamExt::cloned()`](../trait.StreamExt.html#method.cloned "method bevy::tasks::futures_lite::StreamExt::cloned") method.

[CollectFuture](struct.CollectFuture.html "struct bevy::tasks::futures_lite::stream::CollectFuture")

Future for the [`StreamExt::collect()`](../trait.StreamExt.html#method.collect "method bevy::tasks::futures_lite::StreamExt::collect") method.

[Copied](struct.Copied.html "struct bevy::tasks::futures_lite::stream::Copied")

Stream for the [`StreamExt::copied()`](../trait.StreamExt.html#method.copied "method bevy::tasks::futures_lite::StreamExt::copied") method.

[CountFuture](struct.CountFuture.html "struct bevy::tasks::futures_lite::stream::CountFuture")

Future for the [`StreamExt::count()`](../trait.StreamExt.html#method.count "method bevy::tasks::futures_lite::StreamExt::count") method.

[Cycle](struct.Cycle.html "struct bevy::tasks::futures_lite::stream::Cycle")

Stream for the [`StreamExt::cycle()`](../trait.StreamExt.html#method.cycle "method bevy::tasks::futures_lite::StreamExt::cycle") method.

[Drain](struct.Drain.html "struct bevy::tasks::futures_lite::stream::Drain")

Stream for the [`StreamExt::drain()`](../trait.StreamExt.html#method.drain "method bevy::tasks::futures_lite::StreamExt::drain") method.

[Empty](struct.Empty.html "struct bevy::tasks::futures_lite::stream::Empty")

Stream for the [`empty()`](fn.empty.html "fn bevy::tasks::futures_lite::stream::empty") function.

[Enumerate](struct.Enumerate.html "struct bevy::tasks::futures_lite::stream::Enumerate")

Stream for the [`StreamExt::enumerate()`](../trait.StreamExt.html#method.enumerate "method bevy::tasks::futures_lite::StreamExt::enumerate") method.

[Filter](struct.Filter.html "struct bevy::tasks::futures_lite::stream::Filter")

Stream for the [`StreamExt::filter()`](../trait.StreamExt.html#method.filter "method bevy::tasks::futures_lite::StreamExt::filter") method.

[FilterMap](struct.FilterMap.html "struct bevy::tasks::futures_lite::stream::FilterMap")

Stream for the [`StreamExt::filter_map()`](../trait.StreamExt.html#method.filter_map "method bevy::tasks::futures_lite::StreamExt::filter_map") method.

[FindFuture](struct.FindFuture.html "struct bevy::tasks::futures_lite::stream::FindFuture")

Future for the [`StreamExt::find()`](../trait.StreamExt.html#method.find "method bevy::tasks::futures_lite::StreamExt::find") method.

[FindMapFuture](struct.FindMapFuture.html "struct bevy::tasks::futures_lite::stream::FindMapFuture")

Future for the [`StreamExt::find_map()`](../trait.StreamExt.html#method.find_map "method bevy::tasks::futures_lite::StreamExt::find_map") method.

[FlatMap](struct.FlatMap.html "struct bevy::tasks::futures_lite::stream::FlatMap")

Stream for the [`StreamExt::flat_map()`](../trait.StreamExt.html#method.flat_map "method bevy::tasks::futures_lite::StreamExt::flat_map") method.

[Flatten](struct.Flatten.html "struct bevy::tasks::futures_lite::stream::Flatten")

Stream for the [`StreamExt::flatten()`](../trait.StreamExt.html#method.flatten "method bevy::tasks::futures_lite::StreamExt::flatten") method.

[FoldFuture](struct.FoldFuture.html "struct bevy::tasks::futures_lite::stream::FoldFuture")

Future for the [`StreamExt::fold()`](../trait.StreamExt.html#method.fold "method bevy::tasks::futures_lite::StreamExt::fold") method.

[ForEachFuture](struct.ForEachFuture.html "struct bevy::tasks::futures_lite::stream::ForEachFuture")

Future for the [`StreamExt::for_each()`](../trait.StreamExt.html#method.for_each "method bevy::tasks::futures_lite::StreamExt::for_each") method.

[Fuse](struct.Fuse.html "struct bevy::tasks::futures_lite::stream::Fuse")

Stream for the [`StreamExt::fuse()`](../trait.StreamExt.html#method.fuse "method bevy::tasks::futures_lite::StreamExt::fuse") method.

[Inspect](struct.Inspect.html "struct bevy::tasks::futures_lite::stream::Inspect")

Stream for the [`StreamExt::inspect()`](../trait.StreamExt.html#method.inspect "method bevy::tasks::futures_lite::StreamExt::inspect") method.

[Iter](struct.Iter.html "struct bevy::tasks::futures_lite::stream::Iter")

Stream for the [`iter()`](fn.iter.html "fn bevy::tasks::futures_lite::stream::iter") function.

[LastFuture](struct.LastFuture.html "struct bevy::tasks::futures_lite::stream::LastFuture")

Future for the [`StreamExt::last()`](../trait.StreamExt.html#method.last "method bevy::tasks::futures_lite::StreamExt::last") method.

[Map](struct.Map.html "struct bevy::tasks::futures_lite::stream::Map")

Stream for the [`StreamExt::map()`](../trait.StreamExt.html#method.map "method bevy::tasks::futures_lite::StreamExt::map") method.

[MapWhile](struct.MapWhile.html "struct bevy::tasks::futures_lite::stream::MapWhile")

Stream for the [`StreamExt::map_while()`](../trait.StreamExt.html#method.map_while "method bevy::tasks::futures_lite::StreamExt::map_while") method.

[NextFuture](struct.NextFuture.html "struct bevy::tasks::futures_lite::stream::NextFuture")

Future for the [`StreamExt::next()`](../trait.StreamExt.html#method.next "method bevy::tasks::futures_lite::StreamExt::next") method.

[NthFuture](struct.NthFuture.html "struct bevy::tasks::futures_lite::stream::NthFuture")

Future for the [`StreamExt::nth()`](../trait.StreamExt.html#method.nth "method bevy::tasks::futures_lite::StreamExt::nth") method.

[Once](struct.Once.html "struct bevy::tasks::futures_lite::stream::Once")

Stream for the [`once()`](fn.once.html "fn bevy::tasks::futures_lite::stream::once") function.

[OnceFuture](struct.OnceFuture.html "struct bevy::tasks::futures_lite::stream::OnceFuture")

Stream for the [`once_future()`](fn.once_future.html "fn bevy::tasks::futures_lite::stream::once_future") function.

[Or](struct.Or.html "struct bevy::tasks::futures_lite::stream::Or")

Stream for the [`or()`](fn.or.html "fn bevy::tasks::futures_lite::stream::or") function and the [`StreamExt::or()`](../trait.StreamExt.html#method.or "method bevy::tasks::futures_lite::StreamExt::or") method.

[PartitionFuture](struct.PartitionFuture.html "struct bevy::tasks::futures_lite::stream::PartitionFuture")

Future for the [`StreamExt::partition()`](../trait.StreamExt.html#method.partition "method bevy::tasks::futures_lite::StreamExt::partition") method.

[Pending](struct.Pending.html "struct bevy::tasks::futures_lite::stream::Pending")

Stream for the [`pending()`](fn.pending.html "fn bevy::tasks::futures_lite::stream::pending") function.

[PollFn](struct.PollFn.html "struct bevy::tasks::futures_lite::stream::PollFn")

Stream for the [`poll_fn()`](fn.poll_fn.html "fn bevy::tasks::futures_lite::stream::poll_fn") function.

[PositionFuture](struct.PositionFuture.html "struct bevy::tasks::futures_lite::stream::PositionFuture")

Future for the [`StreamExt::position()`](../trait.StreamExt.html#method.position "method bevy::tasks::futures_lite::StreamExt::position") method.

[Race](struct.Race.html "struct bevy::tasks::futures_lite::stream::Race")

Stream for the [`race()`](fn.race.html "fn bevy::tasks::futures_lite::stream::race") function and the [`StreamExt::race()`](../trait.StreamExt.html#method.race "method bevy::tasks::futures_lite::StreamExt::race") method.

[Repeat](struct.Repeat.html "struct bevy::tasks::futures_lite::stream::Repeat")

Stream for the [`repeat()`](fn.repeat.html "fn bevy::tasks::futures_lite::stream::repeat") function.

[RepeatWith](struct.RepeatWith.html "struct bevy::tasks::futures_lite::stream::RepeatWith")

Stream for the [`repeat_with()`](fn.repeat_with.html "fn bevy::tasks::futures_lite::stream::repeat_with") function.

[Scan](struct.Scan.html "struct bevy::tasks::futures_lite::stream::Scan")

Stream for the [`StreamExt::scan()`](../trait.StreamExt.html#method.scan "method bevy::tasks::futures_lite::StreamExt::scan") method.

[Skip](struct.Skip.html "struct bevy::tasks::futures_lite::stream::Skip")

Stream for the [`StreamExt::skip()`](../trait.StreamExt.html#method.skip "method bevy::tasks::futures_lite::StreamExt::skip") method.

[SkipWhile](struct.SkipWhile.html "struct bevy::tasks::futures_lite::stream::SkipWhile")

Stream for the [`StreamExt::skip_while()`](../trait.StreamExt.html#method.skip_while "method bevy::tasks::futures_lite::StreamExt::skip_while") method.

[StepBy](struct.StepBy.html "struct bevy::tasks::futures_lite::stream::StepBy")

Stream for the [`StreamExt::step_by()`](../trait.StreamExt.html#method.step_by "method bevy::tasks::futures_lite::StreamExt::step_by") method.

[StopAfterFuture](struct.StopAfterFuture.html "struct bevy::tasks::futures_lite::stream::StopAfterFuture")

Stream for the [`stop_after_future()`](fn.stop_after_future.html "fn bevy::tasks::futures_lite::stream::stop_after_future") function.

[Take](struct.Take.html "struct bevy::tasks::futures_lite::stream::Take")

Stream for the [`StreamExt::take()`](../trait.StreamExt.html#method.take "method bevy::tasks::futures_lite::StreamExt::take") method.

[TakeWhile](struct.TakeWhile.html "struct bevy::tasks::futures_lite::stream::TakeWhile")

Stream for the [`StreamExt::take_while()`](../trait.StreamExt.html#method.take_while "method bevy::tasks::futures_lite::StreamExt::take_while") method.

[Then](struct.Then.html "struct bevy::tasks::futures_lite::stream::Then")

Stream for the [`StreamExt::then()`](../trait.StreamExt.html#method.then "method bevy::tasks::futures_lite::StreamExt::then") method.

[TryCollectFuture](struct.TryCollectFuture.html "struct bevy::tasks::futures_lite::stream::TryCollectFuture")

Future for the [`StreamExt::try_collect()`](../trait.StreamExt.html#method.try_collect "method bevy::tasks::futures_lite::StreamExt::try_collect") method.

[TryFoldFuture](struct.TryFoldFuture.html "struct bevy::tasks::futures_lite::stream::TryFoldFuture")

Future for the [`StreamExt::try_fold()`](../trait.StreamExt.html#method.try_fold "method bevy::tasks::futures_lite::StreamExt::try_fold") method.

[TryForEachFuture](struct.TryForEachFuture.html "struct bevy::tasks::futures_lite::stream::TryForEachFuture")

Future for the [`StreamExt::try_for_each()`](../trait.StreamExt.html#method.try_for_each "method bevy::tasks::futures_lite::StreamExt::try_for_each") method.

[TryNextFuture](struct.TryNextFuture.html "struct bevy::tasks::futures_lite::stream::TryNextFuture")

Future for the [`StreamExt::try_next()`](../trait.StreamExt.html#method.try_next "method bevy::tasks::futures_lite::StreamExt::try_next") method.

[TryUnfold](struct.TryUnfold.html "struct bevy::tasks::futures_lite::stream::TryUnfold")

Stream for the [`try_unfold()`](fn.try_unfold.html "fn bevy::tasks::futures_lite::stream::try_unfold") function.

[Unfold](struct.Unfold.html "struct bevy::tasks::futures_lite::stream::Unfold")

Stream for the [`unfold()`](fn.unfold.html "fn bevy::tasks::futures_lite::stream::unfold") function.

[UnzipFuture](struct.UnzipFuture.html "struct bevy::tasks::futures_lite::stream::UnzipFuture")

Future for the [`StreamExt::unzip()`](../trait.StreamExt.html#method.unzip "method bevy::tasks::futures_lite::StreamExt::unzip") method.

[Zip](struct.Zip.html "struct bevy::tasks::futures_lite::stream::Zip")

Stream for the [`StreamExt::zip()`](../trait.StreamExt.html#method.zip "method bevy::tasks::futures_lite::StreamExt::zip") method.

## Traits

[Stream](trait.Stream.html "trait bevy::tasks::futures_lite::stream::Stream")

A stream of values produced asynchronously.

[StreamExt](trait.StreamExt.html "trait bevy::tasks::futures_lite::stream::StreamExt")

Extension trait for [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream").

## Functions

[block\_on](fn.block_on.html "fn bevy::tasks::futures_lite::stream::block_on")`std`

Converts a stream into a blocking iterator.

[empty](fn.empty.html "fn bevy::tasks::futures_lite::stream::empty")

Creates an empty stream.

[iter](fn.iter.html "fn bevy::tasks::futures_lite::stream::iter")

Creates a stream from an iterator.

[once](fn.once.html "fn bevy::tasks::futures_lite::stream::once")

Creates a stream that yields a single item.

[once\_future](fn.once_future.html "fn bevy::tasks::futures_lite::stream::once_future")

Creates a stream that invokes the given future as its first item, and then produces no more items.

[or](fn.or.html "fn bevy::tasks::futures_lite::stream::or")

Merges two streams, preferring items from `stream1` whenever both streams are ready.

[pending](fn.pending.html "fn bevy::tasks::futures_lite::stream::pending")

Creates a stream that is always pending.

[poll\_fn](fn.poll_fn.html "fn bevy::tasks::futures_lite::stream::poll_fn")

Creates a stream from a function returning [`Poll`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll").

[race](fn.race.html "fn bevy::tasks::futures_lite::stream::race")`race` and `std`

Merges two streams, with no preference for either stream when both are ready.

[race\_with\_seed](fn.race_with_seed.html "fn bevy::tasks::futures_lite::stream::race_with_seed")`race`

Races two streams, but with a user-provided seed for randomness.

[repeat](fn.repeat.html "fn bevy::tasks::futures_lite::stream::repeat")

Creates an infinite stream that yields the same item repeatedly.

[repeat\_with](fn.repeat_with.html "fn bevy::tasks::futures_lite::stream::repeat_with")

Creates an infinite stream from a closure that generates items.

[stop\_after\_future](fn.stop_after_future.html "fn bevy::tasks::futures_lite::stream::stop_after_future")

Take elements from this stream until the provided future resolves.

[try\_unfold](fn.try_unfold.html "fn bevy::tasks::futures_lite::stream::try_unfold")

Creates a stream from a seed value and a fallible async closure operating on it.

[unfold](fn.unfold.html "fn bevy::tasks::futures_lite::stream::unfold")

Creates a stream from a seed value and an async closure operating on it.

## Type Aliases

[Boxed](type.Boxed.html "type bevy::tasks::futures_lite::stream::Boxed")`alloc`

Type alias for `Pin<Box<dyn Stream<Item = T> + Send + 'static>>`.

[BoxedLocal](type.BoxedLocal.html "type bevy::tasks::futures_lite::stream::BoxedLocal")`alloc`

Type alias for `Pin<Box<dyn Stream<Item = T> + 'static>>`.