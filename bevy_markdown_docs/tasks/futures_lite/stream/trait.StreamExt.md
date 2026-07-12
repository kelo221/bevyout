[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Trait StreamExt 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#834)

```rust
pub trait StreamExt: Stream {
    // Provided methods
    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
       where Self: Unpin { ... }
    fn next(&mut self) -> NextFuture<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn try_next<T, E>(&mut self) -> TryNextFuture<'_, Self> ⓘ
       where Self: Stream<Item = Result<T, E>> + Unpin { ... }
    fn count(self) -> CountFuture<Self> ⓘ
       where Self: Sized { ... }
    fn map<T, F>(self, f: F) -> Map<Self, F>
       where Self: Sized,
             F: FnMut(Self::Item) -> T { ... }
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
       where Self: Sized,
             U: Stream,
             F: FnMut(Self::Item) -> U { ... }
    fn flatten(self) -> Flatten<Self>
       where Self: Sized,
             Self::Item: Stream { ... }
    fn then<F, Fut>(self, f: F) -> Then<Self, F, Fut>
       where Self: Sized,
             F: FnMut(Self::Item) -> Fut,
             Fut: Future { ... }
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
       where Self: Sized,
             P: FnMut(&Self::Item) -> bool { ... }
    fn filter_map<T, F>(self, f: F) -> FilterMap<Self, F>
       where Self: Sized,
             F: FnMut(Self::Item) -> Option<T> { ... }
    fn take(self, n: usize) -> Take<Self>
       where Self: Sized { ... }
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
       where Self: Sized,
             P: FnMut(&Self::Item) -> bool { ... }
    fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>
       where Self: Sized,
             P: FnMut(Self::Item) -> Option<B> { ... }
    fn skip(self, n: usize) -> Skip<Self>
       where Self: Sized { ... }
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
       where Self: Sized,
             P: FnMut(&Self::Item) -> bool { ... }
    fn step_by(self, step: usize) -> StepBy<Self>
       where Self: Sized { ... }
    fn chain<U>(self, other: U) -> Chain<Self, U>
       where Self: Sized,
             U: Stream<Item = Self::Item> { ... }
    fn cloned<'a, T>(self) -> Cloned<Self>
       where Self: Sized + Stream<Item = &'a T>,
             T: Clone + 'a { ... }
    fn copied<'a, T>(self) -> Copied<Self>
       where Self: Sized + Stream<Item = &'a T>,
             T: Copy + 'a { ... }
    fn collect<C>(self) -> CollectFuture<Self, C> ⓘ
       where Self: Sized,
             C: Default + Extend<Self::Item> { ... }
    fn try_collect<T, E, C>(self) -> TryCollectFuture<Self, C> ⓘ
       where Self: Sized + Stream<Item = Result<T, E>>,
             C: Default + Extend<T> { ... }
    fn partition<B, P>(self, predicate: P) -> PartitionFuture<Self, P, B> ⓘ
       where Self: Sized,
             B: Default + Extend<Self::Item>,
             P: FnMut(&Self::Item) -> bool { ... }
    fn fold<T, F>(self, init: T, f: F) -> FoldFuture<Self, F, T> ⓘ
       where Self: Sized,
             F: FnMut(T, Self::Item) -> T { ... }
    fn try_fold<T, E, F, B>(
        &mut self,
        init: B,
        f: F,
    ) -> TryFoldFuture<'_, Self, F, B> ⓘ
       where Self: Sized + Stream<Item = Result<T, E>> + Unpin,
             F: FnMut(B, T) -> Result<B, E> { ... }
    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
       where Self: Sized,
             F: FnMut(&mut St, Self::Item) -> Option<B> { ... }
    fn fuse(self) -> Fuse<Self>
       where Self: Sized { ... }
    fn cycle(self) -> Cycle<Self>
       where Self: Sized + Clone { ... }
    fn enumerate(self) -> Enumerate<Self>
       where Self: Sized { ... }
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
       where Self: Sized,
             F: FnMut(&Self::Item) { ... }
    fn nth(&mut self, n: usize) -> NthFuture<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn last(self) -> LastFuture<Self> ⓘ
       where Self: Sized { ... }
    fn find<P>(&mut self, predicate: P) -> FindFuture<'_, Self, P> ⓘ
       where Self: Unpin,
             P: FnMut(&Self::Item) -> bool { ... }
    fn find_map<F, B>(&mut self, f: F) -> FindMapFuture<'_, Self, F> ⓘ
       where Self: Unpin,
             F: FnMut(Self::Item) -> Option<B> { ... }
    fn position<P>(&mut self, predicate: P) -> PositionFuture<'_, Self, P> ⓘ
       where Self: Unpin,
             P: FnMut(Self::Item) -> bool { ... }
    fn all<P>(&mut self, predicate: P) -> AllFuture<'_, Self, P> ⓘ
       where Self: Unpin,
             P: FnMut(Self::Item) -> bool { ... }
    fn any<P>(&mut self, predicate: P) -> AnyFuture<'_, Self, P> ⓘ
       where Self: Unpin,
             P: FnMut(Self::Item) -> bool { ... }
    fn for_each<F>(self, f: F) -> ForEachFuture<Self, F> ⓘ
       where Self: Sized,
             F: FnMut(Self::Item) { ... }
    fn try_for_each<F, E>(&mut self, f: F) -> TryForEachFuture<'_, Self, F> ⓘ
       where Self: Unpin,
             F: FnMut(Self::Item) -> Result<(), E> { ... }
    fn zip<U>(self, other: U) -> Zip<Self, U>
       where Self: Sized,
             U: Stream { ... }
    fn unzip<A, B, FromA, FromB>(self) -> UnzipFuture<Self, FromA, FromB> ⓘ
       where FromA: Default + Extend<A>,
             FromB: Default + Extend<B>,
             Self: Sized + Stream<Item = (A, B)> { ... }
    fn or<S>(self, other: S) -> Or<Self, S>
       where Self: Sized,
             S: Stream<Item = Self::Item> { ... }
    fn race<S>(self, other: S) -> Race<Self, S>
       where Self: Sized,
             S: Stream<Item = Self::Item> { ... }
    fn drain(&mut self) -> Drain<'_, Self> { ... }
    fn boxed<'a>(self) -> Pin<Box<dyn Stream<Item = Self::Item> + Send + 'a>>
       where Self: Sized + Send + 'a { ... }
    fn boxed_local<'a>(self) -> Pin<Box<dyn Stream<Item = Self::Item> + 'a>>
       where Self: Sized + 'a { ... }
}
```

Extension trait for [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream").

## Provided Methods

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#836-838)

#### fn [poll\_next](#method.poll_next)(&mut self, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>>

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

A convenience for calling [`Stream::poll_next()`](../trait.Stream.html#tymethod.poll_next "method bevy::tasks::futures_lite::Stream::poll_next") on `!`[`Unpin`](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") types.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#862-864)

#### fn [next](#method.next)(&mut self) -> [NextFuture](struct.NextFuture.html "struct bevy::tasks::futures_lite::stream::NextFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Retrieves the next item in the stream.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") when iteration is finished. Stream implementations may choose to or not to resume iteration after that.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(1..=3);

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(3));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#890-892)

#### fn [try\_next](#method.try_next)<T, E>(&mut self) -> [TryNextFuture](struct.TryNextFuture.html "struct bevy::tasks::futures_lite::stream::TryNextFuture")<'\_, Self> [ⓘ](#)

where Self: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Retrieves the next item in the stream.

This is similar to the [`next()`](../trait.StreamExt.html#method.next "method bevy::tasks::futures_lite::StreamExt::next") method, but returns `Result<Option<T>, E>` rather than `Option<Result<T, E>>`.

Note that `s.try_next().await` is equivalent to `s.next().await.transpose()`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![Ok(1), Ok(2), Err("error")]);

assert_eq!(s.try_next().await, Ok(Some(1)));
assert_eq!(s.try_next().await, Ok(Some(2)));
assert_eq!(s.try_next().await, Err("error"));
assert_eq!(s.try_next().await, Ok(None));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#912-914)

#### fn [count](#method.count)(self) -> [CountFuture](struct.CountFuture.html "struct bevy::tasks::futures_lite::stream::CountFuture")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Counts the number of items in the stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s1 = stream::iter(vec![0]);
let s2 = stream::iter(vec![1, 2, 3]);

assert_eq!(s1.count().await, 1);
assert_eq!(s2.count().await, 3);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#939-942)

#### fn [map](#method.map)<T, F>(self, f: F) -> [Map](struct.Map.html "struct bevy::tasks::futures_lite::stream::Map")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T,

Maps items of the stream to new values using a closure.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3]);
let mut s = s.map(|x| 2 * x);

assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(4));
assert_eq!(s.next().await, Some(6));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#965-969)

#### fn [flat\_map](#method.flat_map)<U, F>(self, f: F) -> [FlatMap](struct.FlatMap.html "struct bevy::tasks::futures_lite::stream::FlatMap")<Self, U, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> U,

Maps items to streams and then concatenates them.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let words = stream::iter(vec!["one", "two"]);

let s: String = words
    .flat_map(|s| stream::iter(s.chars()))
    .collect()
    .await;

assert_eq!(s, "onetwo");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#993-996)

#### fn [flatten](#method.flatten)(self) -> [Flatten](struct.Flatten.html "struct bevy::tasks::futures_lite::stream::Flatten")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

Concatenates inner streams.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s1 = stream::iter(vec![1, 2, 3]);
let s2 = stream::iter(vec![4, 5]);

let s = stream::iter(vec![s1, s2]);
let v: Vec<_> = s.flatten().collect().await;
assert_eq!(v, [1, 2, 3, 4, 5]);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1023-1027)

#### fn [then](#method.then)<F, Fut>(self, f: F) -> [Then](struct.Then.html "struct bevy::tasks::futures_lite::stream::Then")<Self, F, Fut>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

Maps items of the stream to new values using an async closure.

##### Examples

```rust
use futures_lite::pin;
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3]);
let mut s = s.then(|x| async move { 2 * x });

pin!(s);
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(4));
assert_eq!(s.next().await, Some(6));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1052-1055)

#### fn [filter](#method.filter)<P>(self, predicate: P) -> [Filter](struct.Filter.html "struct bevy::tasks::futures_lite::stream::Filter")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Keeps items of the stream for which `predicate` returns `true`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3, 4]);
let mut s = s.filter(|i| i % 2 == 0);

assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(4));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1080-1083)

#### fn [filter\_map](#method.filter_map)<T, F>(self, f: F) -> [FilterMap](struct.FilterMap.html "struct bevy::tasks::futures_lite::stream::FilterMap")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>,

Filters and maps items of the stream using a closure.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec!["1", "lol", "3", "NaN", "5"]);
let mut s = s.filter_map(|a| a.parse::<u32>().ok());

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(3));
assert_eq!(s.next().await, Some(5));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1103-1105)

#### fn [take](#method.take)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Take](struct.Take.html "struct bevy::tasks::futures_lite::stream::Take")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes only the first `n` items of the stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::repeat(7).take(2);

assert_eq!(s.next().await, Some(7));
assert_eq!(s.next().await, Some(7));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1126-1129)

#### fn [take\_while](#method.take_while)<P>(self, predicate: P) -> [TakeWhile](struct.TakeWhile.html "struct bevy::tasks::futures_lite::stream::TakeWhile")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Takes items while `predicate` returns `true`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3, 4]);
let mut s = s.take_while(|x| *x < 3);

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1161-1164)

#### fn [map\_while](#method.map_while)<B, P>(self, predicate: P) -> [MapWhile](struct.MapWhile.html "struct bevy::tasks::futures_lite::stream::MapWhile")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Maps items while `predicate` returns [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some").

This stream is not fused. After the predicate returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") the stream still contains remaining items that can be obtained by subsequent `next` calls. You can [`fuse`](../trait.StreamExt.html#method.fuse "method bevy::tasks::futures_lite::StreamExt::fuse") the stream if this behavior is undesirable.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 0, 3]);
let mut s = s.map_while(|x: u32| x.checked_sub(1));

assert_eq!(s.next().await, Some(0));
assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, None);

// Continue to iterate the stream.
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1187-1189)

#### fn [skip](#method.skip)(self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Skip](struct.Skip.html "struct bevy::tasks::futures_lite::stream::Skip")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Skips the first `n` items of the stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3]);
let mut s = s.skip(2);

assert_eq!(s.next().await, Some(3));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1210-1213)

#### fn [skip\_while](#method.skip_while)<P>(self, predicate: P) -> [SkipWhile](struct.SkipWhile.html "struct bevy::tasks::futures_lite::stream::SkipWhile")<Self, P>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Skips items while `predicate` returns `true`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![-1i32, 0, 1]);
let mut s = s.skip_while(|x| x.is_negative());

assert_eq!(s.next().await, Some(0));
assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1242-1244)

#### fn [step\_by](#method.step_by)(self, step: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [StepBy](struct.StepBy.html "struct bevy::tasks::futures_lite::stream::StepBy")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Yields every `step`th item.

##### Panics

This method will panic if the `step` is 0.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![0, 1, 2, 3, 4]);
let mut s = s.step_by(2);

assert_eq!(s.next().await, Some(0));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(4));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1273-1276)

#### fn [chain](#method.chain)<U>(self, other: U) -> [Chain](struct.Chain.html "struct bevy::tasks::futures_lite::stream::Chain")<Self, U>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Appends another stream to the end of this one.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s1 = stream::iter(vec![1, 2]);
let s2 = stream::iter(vec![7, 8]);
let mut s = s1.chain(s2);

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(7));
assert_eq!(s.next().await, Some(8));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1300-1303)

#### fn [cloned](#method.cloned)<'a, T>(self) -> [Cloned](struct.Cloned.html "struct bevy::tasks::futures_lite::stream::Cloned")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + 'a,

Clones all items.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![&1, &2]);
let mut s = s.cloned();

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1324-1327)

#### fn [copied](#method.copied)<'a, T>(self) -> [Copied](struct.Copied.html "struct bevy::tasks::futures_lite::stream::Copied")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + 'a,

Copies all items.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![&1, &2]);
let mut s = s.copied();

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1346-1349)

#### fn [collect](#method.collect)<C>(self) -> [CollectFuture](struct.CollectFuture.html "struct bevy::tasks::futures_lite::stream::CollectFuture")<Self, C> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Collects all items in the stream into a collection.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(1..=3);

let items: Vec<_> = s.collect().await;
assert_eq!(items, [1, 2, 3]);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1372-1375)

#### fn [try\_collect](#method.try_collect)<T, E, C>(self) -> [TryCollectFuture](struct.TryCollectFuture.html "struct bevy::tasks::futures_lite::stream::TryCollectFuture")<Self, C> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T>,

Collects all items in the fallible stream into a collection.

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![Ok(1), Err(2), Ok(3)]);
let res: Result<Vec<i32>, i32> = s.try_collect().await;
assert_eq!(res, Err(2));

let s = stream::iter(vec![Ok(1), Ok(2), Ok(3)]);
let res: Result<Vec<i32>, i32> = s.try_collect().await;
assert_eq!(res, Ok(vec![1, 2, 3]));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1399-1403)

#### fn [partition](#method.partition)<B, P>(self, predicate: P) -> [PartitionFuture](struct.PartitionFuture.html "struct bevy::tasks::futures_lite::stream::PartitionFuture")<Self, P, B> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Partitions items into those for which `predicate` is `true` and those for which it is `false`, and then collects them into two collections.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3]);
let (even, odd): (Vec<_>, Vec<_>) = s.partition(|&n| n % 2 == 0).await;

assert_eq!(even, &[2]);
assert_eq!(odd, &[1, 3]);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1429-1432)

#### fn [fold](#method.fold)<T, F>(self, init: T, f: F) -> [FoldFuture](struct.FoldFuture.html "struct bevy::tasks::futures_lite::stream::FoldFuture")<Self, F, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T,

Accumulates a computation over the stream.

The computation begins with the accumulator value set to `init`, and then applies `f` to the accumulator and each item in the stream. The final accumulator value is returned.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3]);
let sum = s.fold(0, |acc, x| acc + x).await;

assert_eq!(sum, 6);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1467-1470)

#### fn [try\_fold](#method.try_fold)<T, E, F, B>( &mut self, init: B, f: F, ) -> [TryFoldFuture](struct.TryFoldFuture.html "struct bevy::tasks::futures_lite::stream::TryFoldFuture")<'\_, Self, F, B> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<B, E>,

Accumulates a fallible computation over the stream.

The computation begins with the accumulator value set to `init`, and then applies `f` to the accumulator and each item in the stream. The final accumulator value is returned, or an error if `f` failed the computation.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![Ok(1), Ok(2), Ok(3)]);

let sum = s.try_fold(0, |acc, v| {
    if (acc + v) % 2 == 1 {
        Ok(acc + v)
    } else {
        Err("fail")
    }
})
.await;

assert_eq!(sum, Err("fail"));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1502-1505)

#### fn [scan](#method.scan)<St, B, F>(self, initial\_state: St, f: F) -> [Scan](struct.Scan.html "struct bevy::tasks::futures_lite::stream::Scan")<Self, St, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut St](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Maps items of the stream to new values using a state value and a closure.

Scanning begins with the initial state set to `initial_state`, and then applies `f` to the state and each item in the stream. The stream stops when `f` returns `None`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3]);
let mut s = s.scan(1, |state, x| {
    *state = *state * x;
    Some(-*state)
});

assert_eq!(s.next().await, Some(-1));
assert_eq!(s.next().await, Some(-2));
assert_eq!(s.next().await, Some(-6));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1528-1530)

#### fn [fuse](#method.fuse)(self) -> [Fuse](struct.Fuse.html "struct bevy::tasks::futures_lite::stream::Fuse")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fuses the stream so that it stops yielding items after the first [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::once(1).fuse();

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, None);
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1554-1556)

#### fn [cycle](#method.cycle)(self) -> [Cycle](struct.Cycle.html "struct bevy::tasks::futures_lite::stream::Cycle")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Repeats the stream from beginning to end, forever.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![1, 2]).cycle();

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1581-1583)

#### fn [enumerate](#method.enumerate)(self) -> [Enumerate](struct.Enumerate.html "struct bevy::tasks::futures_lite::stream::Enumerate")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Enumerates items, mapping them to `(index, item)`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec!['a', 'b', 'c']);
let mut s = s.enumerate();

assert_eq!(s.next().await, Some((0, 'a')));
assert_eq!(s.next().await, Some((1, 'b')));
assert_eq!(s.next().await, Some((2, 'c')));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1606-1609)

#### fn [inspect](#method.inspect)<F>(self, f: F) -> [Inspect](struct.Inspect.html "struct bevy::tasks::futures_lite::stream::Inspect")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")),

Calls a closure on each item and passes it on.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3, 4, 5]);

let sum = s
   .inspect(|x| println!("about to filter {}", x))
   .filter(|x| x % 2 == 0)
   .inspect(|x| println!("made it through filter: {}", x))
   .fold(0, |sum, i| sum + i)
   .await;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1631-1633)

#### fn [nth](#method.nth)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [NthFuture](struct.NthFuture.html "struct bevy::tasks::futures_lite::stream::NthFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Gets the `n`th item of the stream.

In the end, `n+1` items of the stream will be consumed.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![0, 1, 2, 3, 4, 5, 6, 7]);

assert_eq!(s.nth(2).await, Some(2));
assert_eq!(s.nth(2).await, Some(5));
assert_eq!(s.nth(2).await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1653-1655)

#### fn [last](#method.last)(self) -> [LastFuture](struct.LastFuture.html "struct bevy::tasks::futures_lite::stream::LastFuture")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns the last item in the stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![1, 2, 3, 4]);
assert_eq!(s.last().await, Some(4));

let s = stream::empty::<i32>();
assert_eq!(s.last().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1677-1680)

#### fn [find](#method.find)<P>(&mut self, predicate: P) -> [FindFuture](struct.FindFuture.html "struct bevy::tasks::futures_lite::stream::FindFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Finds the first item of the stream for which `predicate` returns `true`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![11, 12, 13, 14]);

assert_eq!(s.find(|x| *x % 2 == 0).await, Some(12));
assert_eq!(s.next().await, Some(13));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1702-1705)

#### fn [find\_map](#method.find_map)<F, B>(&mut self, f: F) -> [FindMapFuture](struct.FindMapFuture.html "struct bevy::tasks::futures_lite::stream::FindMapFuture")<'\_, Self, F> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

Applies a closure to items in the stream and returns the first [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some") result.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec!["lol", "NaN", "2", "5"]);
let number = s.find_map(|s| s.parse().ok()).await;

assert_eq!(number, Some(2));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1725-1728)

#### fn [position](#method.position)<P>(&mut self, predicate: P) -> [PositionFuture](struct.PositionFuture.html "struct bevy::tasks::futures_lite::stream::PositionFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Finds the index of the first item of the stream for which `predicate` returns `true`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![0, 1, 2, 3, 4, 5]);

assert_eq!(s.position(|x| x == 2).await, Some(2));
assert_eq!(s.position(|x| x == 3).await, Some(0));
assert_eq!(s.position(|x| x == 9).await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1757-1760)

#### fn [all](#method.all)<P>(&mut self, predicate: P) -> [AllFuture](struct.AllFuture.html "struct bevy::tasks::futures_lite::stream::AllFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if `predicate` returns `true` for all items in the stream.

The result is `true` for an empty stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![1, 2, 3]);
assert!(!s.all(|x| x % 2 == 0).await);

let mut s = stream::iter(vec![2, 4, 6, 8]);
assert!(s.all(|x| x % 2 == 0).await);

let mut s = stream::empty::<i32>();
assert!(s.all(|x| x % 2 == 0).await);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1788-1791)

#### fn [any](#method.any)<P>(&mut self, predicate: P) -> [AnyFuture](struct.AnyFuture.html "struct bevy::tasks::futures_lite::stream::AnyFuture")<'\_, Self, P> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Tests if `predicate` returns `true` for any item in the stream.

The result is `false` for an empty stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![1, 3, 5, 7]);
assert!(!s.any(|x| x % 2 == 0).await);

let mut s = stream::iter(vec![1, 2, 3]);
assert!(s.any(|x| x % 2 == 0).await);

let mut s = stream::empty::<i32>();
assert!(!s.any(|x| x % 2 == 0).await);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1811-1814)

#### fn [for\_each](#method.for_each)<F>(self, f: F) -> [ForEachFuture](struct.ForEachFuture.html "struct bevy::tasks::futures_lite::stream::ForEachFuture")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")),

Calls a closure on each item of the stream.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![1, 2, 3]);
s.for_each(|s| println!("{}", s)).await;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1845-1848)

#### fn [try\_for\_each](#method.try_for_each)<F, E>(&mut self, f: F) -> [TryForEachFuture](struct.TryForEachFuture.html "struct bevy::tasks::futures_lite::stream::TryForEachFuture")<'\_, Self, F> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), E>,

Calls a fallible closure on each item of the stream, stopping on first error.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![0, 1, 2, 3]);

let mut v = vec![];
let res = s
    .try_for_each(|n| {
        if n < 2 {
            v.push(n);
            Ok(())
        } else {
            Err("too big")
        }
    })
    .await;

assert_eq!(v, &[0, 1]);
assert_eq!(res, Err("too big"));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1873-1876)

#### fn [zip](#method.zip)<U>(self, other: U) -> [Zip](struct.Zip.html "struct bevy::tasks::futures_lite::stream::Zip")<Self, U>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), U: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

Zips up two streams into a single stream of pairs.

The stream of pairs stops when either of the original two streams is exhausted.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let l = stream::iter(vec![1, 2, 3]);
let r = stream::iter(vec![4, 5, 6, 7]);
let mut s = l.zip(r);

assert_eq!(s.next().await, Some((1, 4)));
assert_eq!(s.next().await, Some((2, 5)));
assert_eq!(s.next().await, Some((3, 6)));
assert_eq!(s.next().await, None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1900-1904)

#### fn [unzip](#method.unzip)<A, B, FromA, FromB>(self) -> [UnzipFuture](struct.UnzipFuture.html "struct bevy::tasks::futures_lite::stream::UnzipFuture")<Self, FromA, FromB> [ⓘ](#)

where FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Collects a stream of pairs into a pair of collections.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::iter(vec![(1, 2), (3, 4)]);
let (left, right): (Vec<_>, Vec<_>) = s.unzip().await;

assert_eq!(left, [1, 3]);
assert_eq!(right, [2, 4]);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1928-1931)

#### fn [or](#method.or)<S>(self, other: S) -> [Or](struct.Or.html "struct bevy::tasks::futures_lite::stream::Or")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Merges with `other` stream, preferring items from `self` whenever both streams are ready.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};
use futures_lite::stream::{once, pending};

assert_eq!(once(1).or(pending()).next().await, Some(1));
assert_eq!(pending().or(once(2)).next().await, Some(2));

// The first future wins.
assert_eq!(once(1).or(once(2)).next().await, Some(1));
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#1956-1959)

#### fn [race](#method.race)<S>(self, other: S) -> [Race](struct.Race.html "struct bevy::tasks::futures_lite::stream::Race")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

Available on **crate features `race` and `std`** only.

Merges with `other` stream, with no preference for either stream when both are ready.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};
use futures_lite::stream::{once, pending};

assert_eq!(once(1).race(pending()).next().await, Some(1));
assert_eq!(pending().race(once(2)).next().await, Some(2));

// One of the two stream is randomly chosen as the winner.
let res = once(1).race(once(2)).next().await;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2015)

#### fn [drain](#method.drain)(&mut self) -> [Drain](struct.Drain.html "struct bevy::tasks::futures_lite::stream::Drain")<'\_, Self>

Yields all immediately available values from a stream.

This is intended to be used as a way of polling a stream without waiting, similar to the [`try_iter`](https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Receiver.html#method.try_iter "method std::sync::mpsc::Receiver::try_iter") function on [`std::sync::mpsc::Receiver`](https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Receiver.html "struct std::sync::mpsc::Receiver"). For instance, running this stream on an [`async_channel::Receiver`](https://docs.rs/async-channel/latest/async_channel/struct.Receiver.html) will return all messages that are currently in the channel, but will not wait for new messages.

This returns a [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") instead of an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") because it still needs access to the polling context in order to poll the underlying stream. Since this stream will never return `Poll::Pending`, wrapping it in [`block_on`](fn.block_on.html "fn bevy::tasks::futures_lite::stream::block_on") will allow it to be effectively used as an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator").

This stream is not necessarily fused. After it returns `None`, it can return `Some(x)` in the future if it is polled again.

##### Examples

```rust
use futures_lite::{future, pin};
use futures_lite::stream::{self, StreamExt};

// A stream that yields two values, returns `Pending`, and then yields one more value.
let pend_once = stream::once_future(async {
    future::yield_now().await;
    3
});
let s = stream::iter(vec![1, 2]).chain(pend_once);
pin!(s);

// This will return the first two values, and then `None` because the stream returns
// `Pending` after that.
let mut iter = stream::block_on(s.drain());
assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), None);

// This will return the last value, because the stream returns `Ready` when polled.
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), None);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2036-2038)

#### fn [boxed](#method.boxed)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Boxes the stream and changes its type to `dyn Stream + Send + 'a`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let a = stream::once(1);
let b = stream::empty();

// Streams of different types can be stored in
// the same collection when they are boxed:
let streams = vec![a.boxed(), b.boxed()];
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2060-2062)

#### fn [boxed\_local](#method.boxed_local)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = Self::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\> + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + 'a,

Available on **crate feature `alloc`** only.

Boxes the stream and changes its type to `dyn Stream + 'a`.

##### Examples

```rust
use futures_lite::stream::{self, StreamExt};

let a = stream::once(1);
let b = stream::empty();

// Streams of different types can be stored in
// the same collection when they are boxed:
let streams = vec![a.boxed_local(), b.boxed_local()];
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2068)

### impl<S> [StreamExt](../trait.StreamExt.html "trait bevy::tasks::futures_lite::StreamExt") for S

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

{"AllFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.AllFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AllFuture\\">AllFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.AllFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AllFuture\\">AllFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>;</div>","AnyFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.AnyFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AnyFuture\\">AnyFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.AnyFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::AnyFuture\\">AnyFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>;</div>","CollectFuture<Self, C>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.CollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CollectFuture\\">CollectFuture</a>&lt;S, C&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, C&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.CollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CollectFuture\\">CollectFuture</a>&lt;S, C&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n C: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = C;</div>","CountFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.CountFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CountFuture\\">CountFuture</a>&lt;S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.CountFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::CountFuture\\">CountFuture</a>&lt;S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>;</div>","FindFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FindFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindFuture\\">FindFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FindFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindFuture\\">FindFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","FindMapFuture<'\_, Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FindMapFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindMapFuture\\">FindMapFuture</a>&lt;'\_, S, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, B, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FindMapFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FindMapFuture\\">FindMapFuture</a>&lt;'\_, S, F&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;B&gt;;</div>","FoldFuture<Self, F, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FoldFuture\\">FoldFuture</a>&lt;S, F, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, F, T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::FoldFuture\\">FoldFuture</a>&lt;S, F, T&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(T, &lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; T,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>","ForEachFuture<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::ForEachFuture\\">ForEachFuture</a>&lt;S, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::ForEachFuture\\">ForEachFuture</a>&lt;S, F&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>),</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>;</div>","LastFuture<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.LastFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::LastFuture\\">LastFuture</a>&lt;S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.LastFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::LastFuture\\">LastFuture</a>&lt;S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","NextFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.NextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NextFuture\\">NextFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.NextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NextFuture\\">NextFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","NthFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.NthFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NthFuture\\">NthFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.NthFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::NthFuture\\">NthFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;;</div>","PartitionFuture<Self, P, B>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.PartitionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PartitionFuture\\">PartitionFuture</a>&lt;S, P, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.PartitionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PartitionFuture\\">PartitionFuture</a>&lt;S, P, B&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,\\n B: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(B, B)</a>;</div>","PositionFuture<'\_, Self, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.PositionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PositionFuture\\">PositionFuture</a>&lt;'\_, S, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, P&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.PositionFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::PositionFuture\\">PositionFuture</a>&lt;'\_, S, P&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>&gt;;</div>","TryCollectFuture<Self, C>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryCollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryCollectFuture\\">TryCollectFuture</a>&lt;S, C&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, E, S, C&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryCollectFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryCollectFuture\\">TryCollectFuture</a>&lt;S, C&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt;,\\n C: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;C, E&gt;;</div>","TryFoldFuture<'\_, Self, F, B>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryFoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryFoldFuture\\">TryFoldFuture</a>&lt;'\_, S, F, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, E, S, F, B&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryFoldFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryFoldFuture\\">TryFoldFuture</a>&lt;'\_, S, F, B&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(B, T) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;B, E&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;B, E&gt;;</div>","TryForEachFuture<'\_, Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryForEachFuture\\">TryForEachFuture</a>&lt;'\_, S, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, F, E&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryForEachFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryForEachFuture\\">TryForEachFuture</a>&lt;'\_, S, F&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, E&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, E&gt;;</div>","TryNextFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryNextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryNextFuture\\">TryNextFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, E, S&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryNextFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::TryNextFuture\\">TryNextFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T, E&gt;&gt; + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;T&gt;, E&gt;;</div>","UnzipFuture<Self, FromA, FromB>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.UnzipFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::UnzipFuture\\">UnzipFuture</a>&lt;S, FromA, FromB&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S, A, B, FromA, FromB&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.UnzipFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::UnzipFuture\\">UnzipFuture</a>&lt;S, FromA, FromB&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(A, B)</a>&gt;,\\n FromA: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;A&gt;,\\n FromB: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\\" title=\\"trait core::default::Default\\">Default</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html\\" title=\\"trait core::iter::traits::collect::Extend\\">Extend</a>&lt;B&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(FromA, FromB)</a>;</div>"}