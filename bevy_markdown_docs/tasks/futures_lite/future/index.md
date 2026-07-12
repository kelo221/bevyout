[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)

# Module future 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/lib.rs.html#57)

Combinators for the [`Future`](../trait.Future.html "trait bevy::tasks::futures_lite::Future") trait.

## Examples

```rust
use futures_lite::future;

for step in 0..3 {
    println!("step {}", step);

    // Give other tasks a chance to run.
    future::yield_now().await;
}
```

## Structs

[CatchUnwind](struct.CatchUnwind.html "struct bevy::tasks::futures_lite::future::CatchUnwind")

Future for the [`FutureExt::catch_unwind()`](../trait.FutureExt.html#method.catch_unwind "method bevy::tasks::futures_lite::FutureExt::catch_unwind") method.

[Fuse](struct.Fuse.html "struct bevy::tasks::futures_lite::future::Fuse")

[`Future`](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for the [`fuse`](fn.fuse.html "fn bevy::tasks::futures_lite::future::fuse") method.

[Or](struct.Or.html "struct bevy::tasks::futures_lite::future::Or")

Future for the [`or()`](fn.or.html "fn bevy::tasks::futures_lite::future::or") function and the [`FutureExt::or()`](../trait.FutureExt.html#method.or "method bevy::tasks::futures_lite::FutureExt::or") method.

[Pending](struct.Pending.html "struct bevy::tasks::futures_lite::future::Pending")

Creates a future which never resolves, representing a computation that never finishes.

[PollFn](struct.PollFn.html "struct bevy::tasks::futures_lite::future::PollFn")

Future for the [`poll_fn()`](fn.poll_fn.html "fn bevy::tasks::futures_lite::future::poll_fn") function.

[PollOnce](struct.PollOnce.html "struct bevy::tasks::futures_lite::future::PollOnce")

Future for the [`poll_once()`](../../fn.poll_once.html "fn bevy::tasks::poll_once") function.

[Race](struct.Race.html "struct bevy::tasks::futures_lite::future::Race")

Future for the [`race()`](fn.race.html "fn bevy::tasks::futures_lite::future::race") function and the [`FutureExt::race()`](../trait.FutureExt.html#method.race "method bevy::tasks::futures_lite::FutureExt::race") method.

[Ready](struct.Ready.html "struct bevy::tasks::futures_lite::future::Ready")

A future that is immediately ready with a value.

[TryZip](struct.TryZip.html "struct bevy::tasks::futures_lite::future::TryZip")

Future for the [`try_zip()`](fn.try_zip.html "fn bevy::tasks::futures_lite::future::try_zip") function.

[YieldNow](struct.YieldNow.html "struct bevy::tasks::futures_lite::future::YieldNow")

Future for the [`yield_now()`](fn.yield_now.html "fn bevy::tasks::futures_lite::future::yield_now") function.

[Zip](struct.Zip.html "struct bevy::tasks::futures_lite::future::Zip")

Future for the [`zip()`](fn.zip.html "fn bevy::tasks::futures_lite::future::zip") function.

## Traits

[Future](trait.Future.html "trait bevy::tasks::futures_lite::future::Future")

A future represents an asynchronous computation, commonly obtained by use of [`async`](../../std/keyword.async.html).

[FutureExt](trait.FutureExt.html "trait bevy::tasks::futures_lite::future::FutureExt")

Extension trait for [`Future`](../trait.Future.html "trait bevy::tasks::futures_lite::Future").

## Functions

[block\_on](fn.block_on.html "fn bevy::tasks::futures_lite::future::block_on")`std`

Blocks the current thread on a future.

[fuse](fn.fuse.html "fn bevy::tasks::futures_lite::future::fuse")

Fuse a future such that `poll` will never again be called once it has completed. This method can be used to turn any `Future` into a `FusedFuture`.

[or](fn.or.html "fn bevy::tasks::futures_lite::future::or")

Returns the result of the future that completes first, preferring `future1` if both are ready.

[pending](fn.pending.html "fn bevy::tasks::futures_lite::future::pending")

Creates a future which never resolves, representing a computation that never finishes.

[poll\_fn](fn.poll_fn.html "fn bevy::tasks::futures_lite::future::poll_fn")

Creates a future from a function returning [`Poll`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll").

[poll\_once](fn.poll_once.html "fn bevy::tasks::futures_lite::future::poll_once")

Polls a future just once and returns an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") with the result.

[race](fn.race.html "fn bevy::tasks::futures_lite::future::race")`race` and `std`

Returns the result of the future that completes first, with no preference if both are ready.

[race\_with\_seed](fn.race_with_seed.html "fn bevy::tasks::futures_lite::future::race_with_seed")`race`

Race two futures but with a predefined random seed.

[ready](fn.ready.html "fn bevy::tasks::futures_lite::future::ready")

Creates a future that is immediately ready with a value.

[try\_zip](fn.try_zip.html "fn bevy::tasks::futures_lite::future::try_zip")

Joins two fallible futures, waiting for both to complete or one of them to error.

[yield\_now](fn.yield_now.html "fn bevy::tasks::futures_lite::future::yield_now")

Wakes the current task and returns [`Poll::Pending`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending") once.

[zip](fn.zip.html "fn bevy::tasks::futures_lite::future::zip")

Joins two futures, waiting for both to complete.

## Type Aliases

[Boxed](type.Boxed.html "type bevy::tasks::futures_lite::future::Boxed")`alloc`

Type alias for `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.

[BoxedLocal](type.BoxedLocal.html "type bevy::tasks::futures_lite::future::BoxedLocal")`alloc`

Type alias for `Pin<Box<dyn Future<Output = T> + 'static>>`.