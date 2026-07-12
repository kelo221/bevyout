[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[prelude](index.html)

# Trait Future 

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#37)

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

A future represents an asynchronous computation, commonly obtained by use of [`async`](../../std/keyword.async.html).

A future is a value that might not have finished computing yet. This kind of “asynchronous value” makes it possible for a thread to continue doing useful work while it waits for the value to become available.

## The `poll` method

The core method of future, `poll`, _attempts_ to resolve the future into a final value. This method does not block if the value is not ready. Instead, the current task is scheduled to be woken up when it’s possible to make further progress by `poll`ing again. The `context` passed to the `poll` method can provide a [`Waker`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html "struct core::task::wake::Waker"), which is a handle for waking up the current task.

When using a future, you generally won’t call `poll` directly, but instead `.await` the value.

## Required Associated Types

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#41)

#### type [Output](#associatedtype.Output)

The type of value produced on completion.

## Required Methods

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#113)

#### fn [poll](#tymethod.poll)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<Self::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available.

##### Return value

This function returns:

*   [`Poll::Pending`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending") if the future is not ready yet
*   [`Poll::Ready(val)`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready "variant core::task::poll::Poll::Ready") with the result `val` of this future if it finished successfully.

Once a future has finished, clients should not `poll` it again.

When a future is not ready yet, `poll` returns `Poll::Pending` and stores a clone of the [`Waker`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html "struct core::task::wake::Waker") copied from the current [`Context`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context"). This [`Waker`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html "struct core::task::wake::Waker") is then woken once the future can make progress. For example, a future waiting for a socket to become readable would call `.clone()` on the [`Waker`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html "struct core::task::wake::Waker") and store it. When a signal arrives elsewhere indicating that the socket is readable, [`Waker::wake`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html#method.wake "method core::task::wake::Waker::wake") is called and the socket future’s task is awoken. Once a task has been woken up, it should attempt to `poll` the future again, which may or may not produce a final value.

Note that on multiple calls to `poll`, only the [`Waker`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Waker.html "struct core::task::wake::Waker") from the [`Context`](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context") passed to the most recent call should be scheduled to receive a wakeup.

##### Runtime characteristics

Futures alone are _inert_; they must be _actively_ `poll`ed for the underlying computation to make progress, meaning that each time the current task is woken up, it should actively re-`poll` pending futures that it still has an interest in.

Having said that, some Futures may represent a value that is being computed in a different task. In this case, the future’s underlying computation is simply acting as a conduit for a value being computed by that other task, which will proceed independently of the Future. Futures of this kind are typically obtained when spawning a new task into an async runtime.

The `poll` function should not be called repeatedly in a tight loop – instead, it should only be called when the future indicates that it is ready to make progress (by calling `wake()`). If you’re familiar with the `poll(2)` or `select(2)` syscalls on Unix it’s worth noting that futures typically do _not_ suffer the same problems of “all wakeups must poll all events”; they are more like `epoll(4)`.

An implementation of `poll` should strive to return quickly, and should not block. Returning quickly prevents unnecessarily clogging up threads or event loops. If it is known ahead of time that a call to `poll` may end up taking a while, the work should be offloaded to a thread pool (or something similar) to ensure that `poll` can return quickly.

##### Panics

Once a future has completed (returned `Ready` from `poll`), calling its `poll` method again may panic, block forever, or cause other kinds of problems; the `Future` trait places no requirements on the effects of such a call. However, as the `poll` method is not marked `unsafe`, Rust’s usual rules apply: calls must never cause undefined behavior (memory corruption, incorrect use of `unsafe` functions, or the like), regardless of the future’s state.

## Trait Implementations

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#187)

### impl<'a, T> [UnsafeFutureObj](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html "trait futures_task::future_obj::UnsafeFutureObj")<'a, T> for &'a mut (dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"))

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#188)

#### fn [into\_raw](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)(self) -> [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a

Convert an owned instance into a (conceptually owned) fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#192)

#### unsafe fn [drop](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)(\_ptr: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a)

Drops the future represented by the given fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#206)

### impl<'a, T> [UnsafeFutureObj](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html "trait futures_task::future_obj::UnsafeFutureObj")<'a, T> for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&'a mut dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>>

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#207)

#### fn [into\_raw](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)(self) -> [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a

Convert an owned instance into a (conceptually owned) fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#211)

#### unsafe fn [drop](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)(\_ptr: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a)

Drops the future represented by the given fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#232)

### impl<'a, T> [UnsafeFutureObj](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html "trait futures_task::future_obj::UnsafeFutureObj")<'a, T> for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a>

where T: 'a,

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#233)

#### fn [into\_raw](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)(self) -> [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a

Convert an owned instance into a (conceptually owned) fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#237)

#### unsafe fn [drop](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)(ptr: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a)

Drops the future represented by the given fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#242)

### impl<'a, T> [UnsafeFutureObj](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html "trait futures_task::future_obj::UnsafeFutureObj")<'a, T> for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>

where T: 'a,

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#243)

#### fn [into\_raw](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)(self) -> [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a

Convert an owned instance into a (conceptually owned) fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.into_raw)

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#247)

#### unsafe fn [drop](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)(ptr: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) dyn [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T> + 'a)

Drops the future represented by the given fat pointer. [Read more](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/trait.UnsafeFutureObj.html#tymethod.drop)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/semaphore.rs.html#296-301)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AcquireArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/semaphore/struct.AcquireArc.html "struct async_lock::semaphore::AcquireArc")

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/semaphore.rs.html#296-301)

#### type [Output](#associatedtype.Output) = [SemaphoreGuardArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/semaphore/struct.SemaphoreGuardArc.html "struct async_lock::semaphore::SemaphoreGuardArc")

[Source](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/src/tokio/sync/notify.rs.html#1033)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Notified](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/tokio/sync/notify/struct.Notified.html "struct tokio::sync::notify::Notified")<'\_>

[Source](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/src/tokio/sync/notify.rs.html#1034)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/src/hyper/upgrade.rs.html#225)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [OnUpgrade](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/upgrade/struct.OnUpgrade.html "struct hyper::upgrade::OnUpgrade")

[Source](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/src/hyper/upgrade.rs.html#226)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Upgraded](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/upgrade/struct.Upgraded.html "struct hyper::upgrade::Upgraded"), [Error](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/error/struct.Error.html "struct hyper::error::Error")\>

[Source](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/src/tokio/sync/notify.rs.html#1088)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [OwnedNotified](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/tokio/sync/notify/struct.OwnedNotified.html "struct tokio::sync::notify::OwnedNotified")

[Source](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/src/tokio/sync/notify.rs.html#1089)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/sources/timer.rs.html#342)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TimeoutFuture](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/calloop/sources/timer/struct.TimeoutFuture.html "struct calloop::sources::timer::TimeoutFuture")

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/sources/timer.rs.html#343)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#464)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Timer](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Timer.html "struct async_io::Timer")

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#465)

#### type [Output](#associatedtype.Output) = [Instant](../../../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#225)

### impl [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [YieldNow](../future/struct.YieldNow.html "struct bevy::tasks::futures_lite::future::YieldNow")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#226)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1754-1756)

### impl<'a, R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FillBuf](../io/struct.FillBuf.html "struct bevy::tasks::futures_lite::io::FillBuf")<'a, R>

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1758)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#908-910)

### impl<'a, S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for ordered\_stream::adapters::[Next](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.Next.html "struct ordered_stream::adapters::Next")<'a, S>

where S: [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#912)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Data](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Data "type ordered_stream::OrderedStream::Data")\>

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#934-936)

### impl<'a, S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [NextBefore](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/adapters/struct.NextBefore.html "struct ordered_stream::adapters::NextBefore")<'a, S>

where S: [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/src/ordered_stream/adapters.rs.html#938)

#### type [Output](#associatedtype.Output) = [PollResult](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/enum.PollResult.html "enum ordered_stream::PollResult")<<S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Ordering](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Ordering "type ordered_stream::OrderedStream::Ordering"), <S as [OrderedStream](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html "trait ordered_stream::OrderedStream")\>::[Data](https://docs.rs/ordered-stream/0.2.0/x86_64-unknown-linux-gnu/ordered_stream/trait.OrderedStream.html#associatedtype.Data "type ordered_stream::OrderedStream::Data")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#261-263)

### impl<'a, St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Peek](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.Peek.html "struct futures_util::stream::stream::peek::Peek")<'a, St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#265)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#303-305)

### impl<'a, St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [PeekMut](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.PeekMut.html "struct futures_util::stream::stream::peek::PeekMut")<'a, St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#307)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a mut <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/stackfuture/0.3.1/x86_64-unknown-linux-gnu/src/stackfuture/lib.rs.html#280)

### impl<'a, T, const STACK\_SIZE: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [StackFuture](../../../asset/io/struct.StackFuture.html "struct bevy::asset::io::StackFuture")<'a, T, STACK\_SIZE>

[Source](https://docs.rs/stackfuture/0.3.1/x86_64-unknown-linux-gnu/src/stackfuture/lib.rs.html#281)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#1320-1327)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Closed](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Closed.html "struct async_channel::Closed")<'a, T>

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#1320-1327)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/mutex.rs.html#322-327)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Lock](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/mutex/struct.Lock.html "struct async_lock::mutex::Lock")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/mutex.rs.html#322-327)

#### type [Output](#associatedtype.Output) = [MutexGuard](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/mutex/struct.MutexGuard.html "struct async_lock::mutex::MutexGuard")<'a, T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/lock/mutex.rs.html#338)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [MutexLockFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/lock/mutex/struct.MutexLockFuture.html "struct futures_util::lock::mutex::MutexLockFuture")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/lock/mutex.rs.html#339)

#### type [Output](#associatedtype.Output) = [MutexGuard](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/lock/mutex/struct.MutexGuard.html "struct futures_util::lock::mutex::MutexGuard")<'a, T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#16-21)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Read](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.Read.html "struct async_lock::rwlock::futures::Read")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#16-21)

#### type [Output](#associatedtype.Output) = [RwLockReadGuard](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockReadGuard.html "struct async_lock::rwlock::RwLockReadGuard")<'a, T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#71-76)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.ReadArc.html "struct async_lock::rwlock::futures::ReadArc")<'a, T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#71-76)

#### type [Output](#associatedtype.Output) = [RwLockReadGuardArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockReadGuardArc.html "struct async_lock::rwlock::RwLockReadGuardArc")<T>

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#1265-1272)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for async\_channel::[Recv](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Recv.html "struct async_channel::Recv")<'a, T>

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#1265-1272)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RecvError](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.RecvError.html "struct async_channel::RecvError")\>

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1789-1796)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for async\_broadcast::[Recv](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/struct.Recv.html "struct async_broadcast::Recv")<'a, T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1789-1796)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RecvError](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/enum.RecvError.html "enum async_broadcast::RecvError")\>

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#1206-1213)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for async\_channel::[Send](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.Send.html "struct async_channel::Send")<'a, T>

[Source](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/src/async_channel/lib.rs.html#1206-1213)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SendError](https://docs.rs/async-channel/2.5.0/x86_64-unknown-linux-gnu/async_channel/struct.SendError.html "struct async_channel::SendError")<T>>

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1716-1723)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for async\_broadcast::[Send](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/struct.Send.html "struct async_broadcast::Send")<'a, T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/src/async_broadcast/lib.rs.html#1716-1723)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>, [SendError](https://docs.rs/async-broadcast/0.7.2/x86_64-unknown-linux-gnu/async_broadcast/struct.SendError.html "struct async_broadcast::SendError")<T>>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#124-131)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UpgradableRead](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.UpgradableRead.html "struct async_lock::rwlock::futures::UpgradableRead")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#124-131)

#### type [Output](#associatedtype.Output) = [RwLockUpgradableReadGuard](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockUpgradableReadGuard.html "struct async_lock::rwlock::RwLockUpgradableReadGuard")<'a, T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#182-189)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UpgradableReadArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.UpgradableReadArc.html "struct async_lock::rwlock::futures::UpgradableReadArc")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#182-189)

#### type [Output](#associatedtype.Output) = [RwLockUpgradableReadGuardArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockUpgradableReadGuardArc.html "struct async_lock::rwlock::RwLockUpgradableReadGuardArc")<T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#344-349)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Upgrade](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.Upgrade.html "struct async_lock::rwlock::futures::Upgrade")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#344-349)

#### type [Output](#associatedtype.Output) = [RwLockWriteGuard](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockWriteGuard.html "struct async_lock::rwlock::RwLockWriteGuard")<'a, T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#236-241)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Write](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.Write.html "struct async_lock::rwlock::futures::Write")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#236-241)

#### type [Output](#associatedtype.Output) = [RwLockWriteGuard](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockWriteGuard.html "struct async_lock::rwlock::RwLockWriteGuard")<'a, T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#291-296)

### impl<'a, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WriteArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.WriteArc.html "struct async_lock::rwlock::futures::WriteArc")<'a, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#291-296)

#### type [Output](#associatedtype.Output) = [RwLockWriteGuardArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockWriteGuardArc.html "struct async_lock::rwlock::RwLockWriteGuardArc")<T>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/semaphore.rs.html#243-248)

### impl<'a> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Acquire](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/semaphore/struct.Acquire.html "struct async_lock::semaphore::Acquire")<'a>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/semaphore.rs.html#243-248)

#### type [Output](#associatedtype.Output) = [SemaphoreGuard](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/semaphore/struct.SemaphoreGuard.html "struct async_lock::semaphore::SemaphoreGuard")<'a>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/barrier.rs.html#138-143)

### impl<'a> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [BarrierWait](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/barrier/struct.BarrierWait.html "struct async_lock::barrier::BarrierWait")<'a>

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/barrier.rs.html#138-143)

#### type [Output](#associatedtype.Output) = [BarrierWaitResult](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/barrier/struct.BarrierWaitResult.html "struct async_lock::barrier::BarrierWaitResult")

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/io.rs.html#149)

### impl<'s, 'l, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for calloop::io::[Readable](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/calloop/io/struct.Readable.html "struct calloop::io::Readable")<'s, 'l, F>

where F: [AsFd](https://doc.rust-lang.org/nightly/std/os/fd/owned/trait.AsFd.html "trait std::os::fd::owned::AsFd"),

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/io.rs.html#150)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/io.rs.html#169)

### impl<'s, 'l, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for calloop::io::[Writable](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/calloop/io/struct.Writable.html "struct calloop::io::Writable")<'s, 'l, F>

where F: [AsFd](https://doc.rust-lang.org/nightly/std/os/fd/owned/trait.AsFd.html "trait std::os::fd::owned::AsFd"),

[Source](https://docs.rs/calloop/0.13.0/x86_64-unknown-linux-gnu/src/calloop/io.rs.html#170)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/either.rs.html#99-102)

### impl<A, B> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::future::either::[Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<A, B>

where A: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), B: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = <A as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/either.rs.html#104)

#### type [Output](#associatedtype.Output) = <A as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/select.rs.html#93-96)

### impl<A, B> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Select](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/select/struct.Select.html "struct futures_util::future::select::Select")<A, B>

where A: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), B: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/select.rs.html#98)

#### type [Output](#associatedtype.Output) = [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<(<A as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), B), (<B as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), A)>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_select.rs.html#63-66)

### impl<A, B> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TrySelect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_select/struct.TrySelect.html "struct futures_util::future::try_select::TrySelect")<A, B>

where A: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"), B: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_select.rs.html#68)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<(<A as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), B), (<B as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), A)>, [Either](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/either/enum.Either.html "enum futures_util::future::either::Either")<(<A as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error"), B), (<B as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error"), A)>>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#296-299)

### impl<F1, F2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Zip](../future/struct.Zip.html "struct bevy::tasks::futures_lite::future::Zip")<F1, F2>

where F1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), F2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#301)

#### type [Output](#associatedtype.Output) = (<F1 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <F2 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"))

[Source](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/src/allocator_api2/stable/boxed.rs.html#2022-2024)

### impl<F, A> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for allocator\_api2::stable::boxed::[Box](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/boxed/struct.Box.html "struct allocator_api2::stable::boxed::Box")<F, A>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), A: [Allocator](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/alloc/trait.Allocator.html "trait allocator_api2::stable::alloc::Allocator") + 'static,

[Source](https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/src/allocator_api2/stable/boxed.rs.html#2026)

#### type [Output](#associatedtype.Output) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/boxed.rs.html#2409)

### impl<F, A> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for bevy::prelude::[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<F, A>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), A: [Allocator](https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html "trait core::alloc::Allocator"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/boxed.rs.html#2410)

#### type [Output](#associatedtype.Output) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/lazy.rs.html#51-53)

### impl<F, R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Lazy](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/lazy/struct.Lazy.html "struct futures_util::future::lazy::Lazy")<F>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> R,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/lazy.rs.html#55)

#### type [Output](#associatedtype.Output) = R

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#117)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#118)

#### type [Output](#associatedtype.Output) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/src/warnings/warnings.rs.html#135)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AllowFuture](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/warnings/warnings/struct.AllowFuture.html "struct warnings::warnings::AllowFuture")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/warnings/0.2.1/x86_64-unknown-linux-gnu/src/warnings/warnings.rs.html#136)

#### type [Output](#associatedtype.Output) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/panic/unwind_safe.rs.html#294)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AssertUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/struct.AssertUnwindSafe.html "struct core::panic::unwind_safe::AssertUnwindSafe")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://doc.rust-lang.org/nightly/src/core/panic/unwind_safe.rs.html#295)

#### type [Output](#associatedtype.Output) = <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#648)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for bevy::tasks::futures\_lite::future::[CatchUnwind](../future/struct.CatchUnwind.html "struct bevy::tasks::futures_lite::future::CatchUnwind")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

Available on **crate feature `std`** only.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#649)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#28-34)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Flatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Flatten.html "struct futures_util::future::future::Flatten")<F>

where Flatten<F, <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#28-34)

#### type [Output](#associatedtype.Output) = <Flatten<F, <F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/event-listener-strategy/0.5.4/x86_64-unknown-linux-gnu/src/event_listener_strategy/lib.rs.html#381)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FutureWrapper](https://docs.rs/event-listener-strategy/0.5.4/x86_64-unknown-linux-gnu/event_listener_strategy/struct.FutureWrapper.html "struct event_listener_strategy::FutureWrapper")<F>

where F: [EventListenerFuture](https://docs.rs/event-listener-strategy/0.5.4/x86_64-unknown-linux-gnu/event_listener_strategy/trait.EventListenerFuture.html "trait event_listener_strategy::EventListenerFuture") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/event-listener-strategy/0.5.4/x86_64-unknown-linux-gnu/src/event_listener_strategy/lib.rs.html#382)

#### type [Output](#associatedtype.Output) = <F as [EventListenerFuture](https://docs.rs/event-listener-strategy/0.5.4/x86_64-unknown-linux-gnu/event_listener_strategy/trait.EventListenerFuture.html "trait event_listener_strategy::EventListenerFuture")\>::[Output](https://docs.rs/event-listener-strategy/0.5.4/x86_64-unknown-linux-gnu/event_listener_strategy/trait.EventListenerFuture.html#associatedtype.Output "type event_listener_strategy::EventListenerFuture::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join_all.rs.html#131-133)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [JoinAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/join_all/struct.JoinAll.html "struct futures_util::future::join_all::JoinAll")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join_all.rs.html#135)

#### type [Output](#associatedtype.Output) = [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/option.rs.html#40)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [OptionFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/option/struct.OptionFuture.html "struct futures_util::future::option::OptionFuture")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/option.rs.html#41)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<F as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join_all.rs.html#151-153)

### impl<F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryJoinAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_join_all/struct.TryJoinAll.html "struct futures_util::future::try_join_all::TryJoinAll")<F>

where F: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join_all.rs.html#155)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<<F as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")\>, <F as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#61-66)

### impl<Fut1, Fut2, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AndThen](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.AndThen.html "struct futures_util::future::try_future::AndThen")<Fut1, Fut2, F>

where [TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlatten.html "struct futures_util::future::try_future::TryFlatten")<[MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOk.html "struct futures_util::future::try_future::MapOk")<Fut1, F>, Fut2>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#61-66)

#### type [Output](#associatedtype.Output) = <[TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlatten.html "struct futures_util::future::try_future::TryFlatten")<[MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOk.html "struct futures_util::future::try_future::MapOk")<Fut1, F>, Fut2> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#68-73)

### impl<Fut1, Fut2, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [OrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.OrElse.html "struct futures_util::future::try_future::OrElse")<Fut1, Fut2, F>

where TryFlattenErr<[MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapErr.html "struct futures_util::future::try_future::MapErr")<Fut1, F>, Fut2>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#68-73)

#### type [Output](#associatedtype.Output) = <TryFlattenErr<[MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapErr.html "struct futures_util::future::try_future::MapErr")<Fut1, F>, Fut2> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#68-73)

### impl<Fut1, Fut2, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Then](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Then.html "struct futures_util::future::future::Then")<Fut1, Fut2, F>

where Flatten<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut1, F>, Fut2>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#68-73)

#### type [Output](#associatedtype.Output) = <Flatten<[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut1, F>, Fut2> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

### impl<Fut1, Fut2, Fut3, Fut4, Fut5> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Join5](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/join/struct.Join5.html "struct futures_util::future::join::Join5")<Fut1, Fut2, Fut3, Fut4, Fut5>

where Fut1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut3: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut4: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut5: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

#### type [Output](#associatedtype.Output) = (<Fut1 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut2 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut3 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut4 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut5 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"))

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

### impl<Fut1, Fut2, Fut3, Fut4, Fut5> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryJoin5](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_join/struct.TryJoin5.html "struct futures_util::future::try_join::TryJoin5")<Fut1, Fut2, Fut3, Fut4, Fut5>

where Fut1: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"), Fut2: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Fut3: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Fut4: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Fut5: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<(<Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut2 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut3 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut4 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut5 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")), <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

### impl<Fut1, Fut2, Fut3, Fut4> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Join4](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/join/struct.Join4.html "struct futures_util::future::join::Join4")<Fut1, Fut2, Fut3, Fut4>

where Fut1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut3: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut4: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

#### type [Output](#associatedtype.Output) = (<Fut1 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut2 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut3 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut4 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"))

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

### impl<Fut1, Fut2, Fut3, Fut4> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryJoin4](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_join/struct.TryJoin4.html "struct futures_util::future::try_join::TryJoin4")<Fut1, Fut2, Fut3, Fut4>

where Fut1: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"), Fut2: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Fut3: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Fut4: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<(<Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut2 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut3 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut4 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")), <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

### impl<Fut1, Fut2, Fut3> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Join3](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/join/struct.Join3.html "struct futures_util::future::join::Join3")<Fut1, Fut2, Fut3>

where Fut1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut3: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

#### type [Output](#associatedtype.Output) = (<Fut1 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut2 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut3 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"))

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

### impl<Fut1, Fut2, Fut3> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryJoin3](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_join/struct.TryJoin3.html "struct futures_util::future::try_join::TryJoin3")<Fut1, Fut2, Fut3>

where Fut1: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"), Fut2: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>, Fut3: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<(<Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut2 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut3 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")), <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

### impl<Fut1, Fut2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Join](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/join/struct.Join.html "struct futures_util::future::join::Join")<Fut1, Fut2>

where Fut1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), Fut2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/join.rs.html#76-88)

#### type [Output](#associatedtype.Output) = (<Fut1 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), <Fut2 as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"))

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#30-35)

### impl<Fut1, Fut2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryFlatten](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.TryFlatten.html "struct futures_util::future::try_future::TryFlatten")<Fut1, Fut2>

where TryFlatten<Fut1, Fut2>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#30-35)

#### type [Output](#associatedtype.Output) = <TryFlatten<Fut1, Fut2> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

### impl<Fut1, Fut2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryJoin](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_join/struct.TryJoin.html "struct futures_util::future::try_join::TryJoin")<Fut1, Fut2>

where Fut1: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"), Fut2: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Error = <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_join.rs.html#92-104)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<(<Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut2 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok")), <Fut1 as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#75-80)

### impl<Fut, E> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ErrInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.ErrInto.html "struct futures_util::future::try_future::ErrInto")<Fut, E>

where [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapErr.html "struct futures_util::future::try_future::MapErr")<Fut, IntoFn<E>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#75-80)

#### type [Output](#associatedtype.Output) = <[MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapErr.html "struct futures_util::future::try_future::MapErr")<Fut, IntoFn<E>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#82-87)

### impl<Fut, E> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [OkInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.OkInto.html "struct futures_util::future::try_future::OkInto")<Fut, E>

where [MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOk.html "struct futures_util::future::try_future::MapOk")<Fut, IntoFn<E>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#82-87)

#### type [Output](#associatedtype.Output) = <[MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOk.html "struct futures_util::future::try_future::MapOk")<Fut, IntoFn<E>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#120-125)

### impl<Fut, F, G> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [MapOkOrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOkOrElse.html "struct futures_util::future::try_future::MapOkOrElse")<Fut, F, G>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, ChainFn<MapOkFn<F>, ChainFn<MapErrFn<G>, MergeResultFn>>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#120-125)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, ChainFn<MapOkFn<F>, ChainFn<MapErrFn<G>, MergeResultFn>>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#75-80)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Inspect.html "struct futures_util::future::future::Inspect")<Fut, F>

where Map<Fut, InspectFn<F>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#75-80)

#### type [Output](#associatedtype.Output) = <Map<Fut, InspectFn<F>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#96-101)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [InspectErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.InspectErr.html "struct futures_util::future::try_future::InspectErr")<Fut, F>

where [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Inspect.html "struct futures_util::future::future::Inspect")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, InspectErrFn<F>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#96-101)

#### type [Output](#associatedtype.Output) = <[Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Inspect.html "struct futures_util::future::future::Inspect")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, InspectErrFn<F>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#89-94)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [InspectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.InspectOk.html "struct futures_util::future::try_future::InspectOk")<Fut, F>

where [Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Inspect.html "struct futures_util::future::future::Inspect")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, InspectOkFn<F>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#89-94)

#### type [Output](#associatedtype.Output) = <[Inspect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Inspect.html "struct futures_util::future::future::Inspect")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, InspectOkFn<F>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#47-52)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, F>

where Map<Fut, F>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#47-52)

#### type [Output](#associatedtype.Output) = <Map<Fut, F> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#113-118)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [MapErr](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapErr.html "struct futures_util::future::try_future::MapErr")<Fut, F>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, MapErrFn<F>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#113-118)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, MapErrFn<F>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#106-111)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [MapOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.MapOk.html "struct futures_util::future::try_future::MapOk")<Fut, F>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, MapOkFn<F>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#106-111)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, MapOkFn<F>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#127-132)

### impl<Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UnwrapOrElse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/struct.UnwrapOrElse.html "struct futures_util::future::try_future::UnwrapOrElse")<Fut, F>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, UnwrapOrElseFn<F>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#127-132)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<[IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>, UnwrapOrElseFn<F>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#61-66)

### impl<Fut, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [MapInto](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.MapInto.html "struct futures_util::future::future::MapInto")<Fut, T>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, IntoFn<T>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#61-66)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, IntoFn<T>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/abortable.rs.html#163-165)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Abortable](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/abortable/struct.Abortable.html "struct futures_util::abortable::Abortable")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/abortable.rs.html#167)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), [Aborted](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/abortable/struct.Aborted.html "struct futures_util::abortable::Aborted")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/catch_unwind.rs.html#29-31)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::future::future::catch\_unwind::[CatchUnwind](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/catch_unwind/struct.CatchUnwind.html "struct futures_util::future::future::catch_unwind::CatchUnwind")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/catch_unwind.rs.html#33)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#500)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for bevy::tasks::futures\_lite::future::[Fuse](../future/struct.Fuse.html "struct bevy::tasks::futures_lite::future::Fuse")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#501)

#### type [Output](#associatedtype.Output) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/fuse.rs.html#80)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::future::future::fuse::[Fuse](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/fuse/struct.Fuse.html "struct futures_util::future::future::fuse::Fuse")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/fuse.rs.html#81)

#### type [Output](#associatedtype.Output) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/into_future.rs.html#29)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [IntoFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_future/into_future/struct.IntoFuture.html "struct futures_util::future::try_future::into_future::IntoFuture")<Fut>

where Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/into_future.rs.html#30)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/maybe_done.rs.html#89)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [MaybeDone](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/maybe_done/enum.MaybeDone.html "enum futures_util::future::maybe_done::MaybeDone")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/maybe_done.rs.html#90)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#82-87)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [NeverError](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.NeverError.html "struct futures_util::future::future::NeverError")<Fut>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, OkFn<[Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")\>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#82-87)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, OkFn<[Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")\>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/select_all.rs.html#51)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [SelectAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/select_all/struct.SelectAll.html "struct futures_util::future::select_all::SelectAll")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/select_all.rs.html#52)

#### type [Output](#associatedtype.Output) = (<Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Fut>)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/select_ok.rs.html#45)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [SelectOk](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/select_ok/struct.SelectOk.html "struct futures_util::future::select_ok::SelectOk")<Fut>

where Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/select_ok.rs.html#46)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<(<Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok "type futures_core::future::TryFuture::Ok"), [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<Fut>), <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/shared.rs.html#261-264)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Shared](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/shared/struct.Shared.html "struct futures_util::future::future::shared::Shared")<Fut>

where Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/shared.rs.html#266)

#### type [Output](#associatedtype.Output) = <Fut as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_maybe_done.rs.html#73)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryMaybeDone](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/try_maybe_done/enum.TryMaybeDone.html "enum futures_util::future::try_maybe_done::TryMaybeDone")<Fut>

where Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/try_maybe_done.rs.html#74)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <Fut as [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error "type futures_core::future::TryFuture::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#89-94)

### impl<Fut> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UnitError](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.UnitError.html "struct futures_util::future::future::UnitError")<Fut>

where [Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, OkFn<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#89-94)

#### type [Output](#associatedtype.Output) = <[Map](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/future/struct.Map.html "struct futures_util::future::future::Map")<Fut, OkFn<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>> as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/src/hyper/server/conn/http1.rs.html#206-212)

### impl<I, B, S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Connection](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/server/conn/http1/struct.Connection.html "struct hyper::server::conn::http1::Connection")<I, S>

where S: [HttpService](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/service/http/trait.HttpService.html "trait hyper::service::http::HttpService")<[Incoming](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/body/incoming/struct.Incoming.html "struct hyper::body::incoming::Incoming"), ResBody = B>, <S as [HttpService](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/service/http/trait.HttpService.html "trait hyper::service::http::HttpService")<[Incoming](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/body/incoming/struct.Incoming.html "struct hyper::body::incoming::Incoming")\>>::[Error](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/service/http/trait.HttpService.html#associatedtype.Error "type hyper::service::http::HttpService::Error"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>, I: [Read](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/rt/io/trait.Read.html "trait hyper::rt::io::Read") + [Write](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/rt/io/trait.Write.html "trait hyper::rt::io::Write") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), B: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") + 'static, <B as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>,

[Source](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/src/hyper/server/conn/http1.rs.html#214)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/error/struct.Error.html "struct hyper::error::Error")\>

[Source](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/src/hyper/server/conn/http1.rs.html#540-546)

### impl<I, B, S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UpgradeableConnection](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/server/conn/http1/struct.UpgradeableConnection.html "struct hyper::server::conn::http1::UpgradeableConnection")<I, S>

where S: [HttpService](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/service/http/trait.HttpService.html "trait hyper::service::http::HttpService")<[Incoming](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/body/incoming/struct.Incoming.html "struct hyper::body::incoming::Incoming"), ResBody = B>, <S as [HttpService](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/service/http/trait.HttpService.html "trait hyper::service::http::HttpService")<[Incoming](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/body/incoming/struct.Incoming.html "struct hyper::body::incoming::Incoming")\>>::[Error](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/service/http/trait.HttpService.html#associatedtype.Error "type hyper::service::http::HttpService::Error"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>, I: [Read](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/rt/io/trait.Read.html "trait hyper::rt::io::Read") + [Write](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/rt/io/trait.Write.html "trait hyper::rt::io::Write") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, B: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") + 'static, <B as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error"): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>,

[Source](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/src/hyper/server/conn/http1.rs.html#548)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://docs.rs/hyper/1.10.1/x86_64-unknown-linux-gnu/hyper/error/struct.Error.html "struct hyper::error::Error")\>

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/lib.rs.html#1407-1410)

### impl<L, R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for either::[Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<L, R>

where L: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"), R: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = <L as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")\>,

`Either<L, R>` is a future if both `L` and `R` are futures.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/lib.rs.html#1412)

#### type [Output](#associatedtype.Output) = <L as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#126-128)

### impl<P> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>

where P: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#130)

#### type [Output](#associatedtype.Output) = <<P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2366)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadExactFuture](../io/struct.ReadExactFuture.html "struct bevy::tasks::futures_lite::io::ReadExactFuture")<'\_, R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2367)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2208)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadFuture](../io/struct.ReadFuture.html "struct bevy::tasks::futures_lite::io::ReadFuture")<'\_, R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2209)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1848)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadLineFuture](../io/struct.ReadLineFuture.html "struct bevy::tasks::futures_lite::io::ReadLineFuture")<'\_, R>

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1849)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2247)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadToEndFuture](../io/struct.ReadToEndFuture.html "struct bevy::tasks::futures_lite::io::ReadToEndFuture")<'\_, R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2248)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2272)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadToStringFuture](../io/struct.ReadToStringFuture.html "struct bevy::tasks::futures_lite::io::ReadToStringFuture")<'\_, R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2273)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1793)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadUntilFuture](../io/struct.ReadUntilFuture.html "struct bevy::tasks::futures_lite::io::ReadUntilFuture")<'\_, R>

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1794)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2227)

### impl<R> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadVectoredFuture](../io/struct.ReadVectoredFuture.html "struct bevy::tasks::futures_lite::io::ReadVectoredFuture")<'\_, R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2228)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3436-3440)

### impl<S, A, B, FromA, FromB> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UnzipFuture](../stream/struct.UnzipFuture.html "struct bevy::tasks::futures_lite::stream::UnzipFuture")<S, FromA, FromB>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3442)

#### type [Output](#associatedtype.Output) = [(FromA, FromB)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3219-3222)

### impl<S, B, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FindMapFuture](../stream/struct.FindMapFuture.html "struct bevy::tasks::futures_lite::stream::FindMapFuture")<'\_, S, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3224)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<B>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2171-2174)

### impl<S, C> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [CollectFuture](../stream/struct.CollectFuture.html "struct bevy::tasks::futures_lite::stream::CollectFuture")<S, C>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2176)

#### type [Output](#associatedtype.Output) = C

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3375-3378)

### impl<S, F, E> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryForEachFuture](../stream/struct.TryForEachFuture.html "struct bevy::tasks::futures_lite::stream::TryForEachFuture")<'\_, S, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), E>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3380)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), E>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2268-2271)

### impl<S, F, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FoldFuture](../stream/struct.FoldFuture.html "struct bevy::tasks::futures_lite::stream::FoldFuture")<S, F, T>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, <S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> T,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2273)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3347-3350)

### impl<S, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ForEachFuture](../stream/struct.ForEachFuture.html "struct bevy::tasks::futures_lite::stream::ForEachFuture")<S, F>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3352)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2230-2234)

### impl<S, P, B> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [PartitionFuture](../stream/struct.PartitionFuture.html "struct bevy::tasks::futures_lite::stream::PartitionFuture")<S, P, B>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), B: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2236)

#### type [Output](#associatedtype.Output) = [(B, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3284-3287)

### impl<S, P> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AllFuture](../stream/struct.AllFuture.html "struct bevy::tasks::futures_lite::stream::AllFuture")<'\_, S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3289)

#### type [Output](#associatedtype.Output) = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3315-3318)

### impl<S, P> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AnyFuture](../stream/struct.AnyFuture.html "struct bevy::tasks::futures_lite::stream::AnyFuture")<'\_, S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3320)

#### type [Output](#associatedtype.Output) = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3191-3194)

### impl<S, P> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FindFuture](../stream/struct.FindFuture.html "struct bevy::tasks::futures_lite::stream::FindFuture")<'\_, S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3196)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3251-3254)

### impl<S, P> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [PositionFuture](../stream/struct.PositionFuture.html "struct bevy::tasks::futures_lite::stream::PositionFuture")<'\_, S, P>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3256)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2147)

### impl<S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [CountFuture](../stream/struct.CountFuture.html "struct bevy::tasks::futures_lite::stream::CountFuture")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2148)

#### type [Output](#associatedtype.Output) = [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3167)

### impl<S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [LastFuture](../stream/struct.LastFuture.html "struct bevy::tasks::futures_lite::stream::LastFuture")<S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3168)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2107)

### impl<S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [NextFuture](../stream/struct.NextFuture.html "struct bevy::tasks::futures_lite::stream::NextFuture")<'\_, S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2108)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3137-3139)

### impl<S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [NthFuture](../stream/struct.NthFuture.html "struct bevy::tasks::futures_lite::stream::NthFuture")<'\_, S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#3141)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<S as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2765)

### impl<S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [SeekFuture](../io/struct.SeekFuture.html "struct bevy::tasks::futures_lite::io::SeekFuture")<'\_, S>

where S: [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2766)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/unzip.rs.html#43-47)

### impl<St, A, B, FromA, FromB> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Unzip](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/unzip/struct.Unzip.html "struct futures_util::stream::stream::unzip::Unzip")<St, FromA, FromB>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, FromA: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<A>, FromB: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<B>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/unzip.rs.html#49)

#### type [Output](#associatedtype.Output) = [(FromA, FromB)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/collect.rs.html#40-43)

### impl<St, C> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::stream::stream::collect::[Collect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/collect/struct.Collect.html "struct futures_util::stream::stream::collect::Collect")<St, C>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/collect.rs.html#45)

#### type [Output](#associatedtype.Output) = C

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_collect.rs.html#36-39)

### impl<St, C> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryCollect](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_collect/struct.TryCollect.html "struct futures_util::stream::try_stream::try_collect::TryCollect")<St, C>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_collect.rs.html#41)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<C, <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#351-354)

### impl<St, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [NextIf](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.NextIf.html "struct futures_util::stream::stream::peek::NextIf")<'\_, St, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: for<'a> FnOnce1<&'a <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#356)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/all.rs.html#58-62)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [All](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/all/struct.All.html "struct futures_util::stream::stream::all::All")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/all.rs.html#64)

#### type [Output](#associatedtype.Output) = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/any.rs.html#58-62)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Any](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/any/struct.Any.html "struct futures_util::stream::stream::any::Any")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/any.rs.html#64)

#### type [Output](#associatedtype.Output) = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/for_each.rs.html#56-60)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ForEach](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/for_each/struct.ForEach.html "struct futures_util::stream::stream::for_each::ForEach")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/for_each.rs.html#62)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/for_each_concurrent.rs.html#65-69)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ForEachConcurrent](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/for_each_concurrent/struct.ForEachConcurrent.html "struct futures_util::stream::stream::for_each_concurrent::ForEachConcurrent")<St, Fut, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/for_each_concurrent.rs.html#71)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_all.rs.html#58-62)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryAll](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_all/struct.TryAll.html "struct futures_util::stream::try_stream::try_all::TryAll")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_all.rs.html#64)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_any.rs.html#58-62)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryAny](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_any/struct.TryAny.html "struct futures_util::stream::try_stream::try_any::TryAny")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_any.rs.html#64)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_for_each.rs.html#45-49)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryForEach](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_for_each/struct.TryForEach.html "struct futures_util::stream::try_stream::try_for_each::TryForEach")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_for_each.rs.html#51)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_for_each_concurrent.rs.html#66-70)

### impl<St, Fut, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryForEachConcurrent](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_for_each_concurrent/struct.TryForEachConcurrent.html "struct futures_util::stream::try_stream::try_for_each_concurrent::TryForEachConcurrent")<St, Fut, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_for_each_concurrent.rs.html#72)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/fold.rs.html#59-63)

### impl<St, Fut, T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Fold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/fold/struct.Fold.html "struct futures_util::stream::stream::fold::Fold")<St, Fut, T, F>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")) -> Fut, Fut: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/fold.rs.html#65)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_fold.rs.html#59-63)

### impl<St, Fut, T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryFold](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_fold/struct.TryFold.html "struct futures_util::stream::try_stream::try_fold::TryFold")<St, Fut, T, F>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(T, <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")) -> Fut, Fut: [TryFuture](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html "trait futures_core::future::TryFuture")<Ok = T, Error = <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_fold.rs.html#65)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#413-417)

### impl<St, T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [NextIfEq](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/peek/struct.NextIfEq.html "struct futures_util::stream::stream::peek::NextIfEq")<'\_, St, T>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/peek.rs.html#419)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/concat.rs.html#29-32)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Concat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/concat/struct.Concat.html "struct futures_util::stream::stream::concat::Concat")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"): [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\> + [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/concat.rs.html#34)

#### type [Output](#associatedtype.Output) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/count.rs.html#40)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Count](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/count/struct.Count.html "struct futures_util::stream::stream::count::Count")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/count.rs.html#41)

#### type [Output](#associatedtype.Output) = [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/next.rs.html#28)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::stream::stream::next::[Next](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/next/struct.Next.html "struct futures_util::stream::stream::next::Next")<'\_, St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/next.rs.html#29)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1171)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_channel::mpsc::[Recv](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.Recv.html "struct futures_channel::mpsc::Recv")<'\_, St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/mpsc/mod.rs.html#1172)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item"), [RecvError](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/mpsc/struct.RecvError.html "struct futures_channel::mpsc::RecvError")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/select_next_some.rs.html#28)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [SelectNextSome](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/select_next_some/struct.SelectNextSome.html "struct futures_util::stream::stream::select_next_some::SelectNextSome")<'\_, St>

where St: [FusedStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.FusedStream.html "trait futures_core::stream::FusedStream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/select_next_some.rs.html#29)

#### type [Output](#associatedtype.Output) = <St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/into_future.rs.html#79)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [StreamFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/stream/into_future/struct.StreamFuture.html "struct futures_util::stream::stream::into_future::StreamFuture")<St>

where St: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/stream/into_future.rs.html#80)

#### type [Output](#associatedtype.Output) = ([Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")\>::[Item](../trait.Stream.html#associatedtype.Item "type bevy::tasks::futures_lite::Stream::Item")\>, St)

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_concat.rs.html#29-32)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryConcat](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_concat/struct.TryConcat.html "struct futures_util::stream::try_stream::try_concat::TryConcat")<St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"): [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")\> + [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_concat.rs.html#34)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok"), <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_next.rs.html#28)

### impl<St> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryNext](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/stream/try_stream/try_next/struct.TryNext.html "struct futures_util::stream::try_stream::try_next::TryNext")<'\_, St>

where St: [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/stream/try_stream/try_next.rs.html#29)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Ok](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Ok "type futures_core::stream::TryStream::Ok")\>, <St as [TryStream](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html "trait futures_core::stream::TryStream")\>::[Error](https://docs.rs/futures-core/0.3.32/x86_64-unknown-linux-gnu/futures_core/stream/trait.TryStream.html#associatedtype.Error "type futures_core::stream::TryStream::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#365-368)

### impl<T1, T2, E, F1, F2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryZip](../future/struct.TryZip.html "struct bevy::tasks::futures_lite::future::TryZip")<F1, T1, F2, T2>

where F1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T1, E>>, F2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T2, E>>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#370)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[(T1, T2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html), E>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2200-2203)

### impl<T, E, S, C> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryCollectFuture](../stream/struct.TryCollectFuture.html "struct bevy::tasks::futures_lite::stream::TryCollectFuture")<S, C>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>, C: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2205)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<C, E>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2301-2304)

### impl<T, E, S, F, B> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryFoldFuture](../stream/struct.TryFoldFuture.html "struct bevy::tasks::futures_lite::stream::TryFoldFuture")<'\_, S, F, B>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(B, T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<B, E>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2306)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<B, E>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2124-2126)

### impl<T, E, S> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [TryNextFuture](../stream/struct.TryNextFuture.html "struct bevy::tasks::futures_lite::stream::TryNextFuture")<'\_, S>

where S: [Stream](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2128)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>, E>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#441-444)

### impl<T, F1, F2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Or](../future/struct.Or.html "struct bevy::tasks::futures_lite::future::Or")<F1, F2>

where F1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>, F2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#446)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#607-610)

### impl<T, F1, F2> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Race](../future/struct.Race.html "struct bevy::tasks::futures_lite::future::Race")<F1, F2>

where F1: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>, F2: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>,

Available on **crate feature `race`** only.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#612)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/always_ready.rs.html#32)

### impl<T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [AlwaysReady](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/always_ready/struct.AlwaysReady.html "struct futures_util::future::always_ready::AlwaysReady")<T, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")() -> T,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/always_ready.rs.html#33)

#### type [Output](#associatedtype.Output) = T

1.64.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/poll_fn.rs.html#143-145)

### impl<T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for core::future::poll\_fn::[PollFn](https://doc.rust-lang.org/nightly/core/future/poll_fn/struct.PollFn.html "struct core::future::poll_fn::PollFn")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/future/poll_fn.rs.html#147)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#189-191)

### impl<T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for bevy::tasks::futures\_lite::future::[PollFn](../future/struct.PollFn.html "struct bevy::tasks::futures_lite::future::PollFn")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<T>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#193)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_fn.rs.html#49-51)

### impl<T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::future::poll\_fn::[PollFn](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/poll_fn/struct.PollFn.html "struct futures_util::future::poll_fn::PollFn")<F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<T>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_fn.rs.html#53)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_immediate.rs.html#19-21)

### impl<T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [PollImmediate](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/poll_immediate/struct.PollImmediate.html "struct futures_util::future::poll_immediate::PollImmediate")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>,

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/poll_immediate.rs.html#23)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#138-140)

### impl<T, F> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [PollOnce](../future/struct.PollOnce.html "struct bevy::tasks::futures_lite::future::PollOnce")<F>

where F: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = T>,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#142)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

[Source](https://docs.rs/async-task/4.7.1/x86_64-unknown-linux-gnu/src/async_task/task.rs.html#551)

### impl<T, M> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FallibleTask](https://docs.rs/async-task/4.7.1/x86_64-unknown-linux-gnu/async_task/task/struct.FallibleTask.html "struct async_task::task::FallibleTask")<T, M>

[Source](https://docs.rs/async-task/4.7.1/x86_64-unknown-linux-gnu/src/async_task/task.rs.html#552)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

[Source](https://docs.rs/async-task/4.7.1/x86_64-unknown-linux-gnu/src/async_task/task.rs.html#447)

### impl<T, M> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Task](../../struct.Task.html "struct bevy::tasks::Task")<T, M>

[Source](https://docs.rs/async-task/4.7.1/x86_64-unknown-linux-gnu/src/async_task/task.rs.html#448)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/oneshot.rs.html#408)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Cancellation](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/oneshot/struct.Cancellation.html "struct futures_channel::oneshot::Cancellation")<'\_, T>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/oneshot.rs.html#409)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/combinators/collect.rs.html#26)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for http\_body\_util::combinators::collect::[Collect](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/collect/struct.Collect.html "struct http_body_util::combinators::collect::Collect")<T>

where T: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/combinators/collect.rs.html#27)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Collected](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/collected/struct.Collected.html "struct http_body_util::collected::Collected")<<T as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data")\>, <T as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>

[Source](https://docs.rs/event-listener/5.4.1/x86_64-unknown-linux-gnu/src/event_listener/lib.rs.html#949)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [EventListener](https://docs.rs/event-listener/5.4.1/x86_64-unknown-linux-gnu/event_listener/struct.EventListener.html "struct event_listener::EventListener")<T>

[Source](https://docs.rs/event-listener/5.4.1/x86_64-unknown-linux-gnu/src/event_listener/lib.rs.html#949)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/combinators/frame.rs.html#12)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Frame](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/http_body_util/combinators/frame/struct.Frame.html "struct http_body_util::combinators::frame::Frame")<'\_, T>

where T: [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/http-body-util/0.1.3/x86_64-unknown-linux-gnu/src/http_body_util/combinators/frame.rs.html#13)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Frame](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/frame/struct.Frame.html "struct http_body::frame::Frame")<<T as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Data](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Data "type http_body::Body::Data")\>, <T as [Body](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html "trait http_body::Body")\>::[Error](https://docs.rs/http-body/1.0.1/x86_64-unknown-linux-gnu/http_body/trait.Body.html#associatedtype.Error "type http_body::Body::Error")\>>

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#122)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FutureObj](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/struct.FutureObj.html "struct futures_task::future_obj::FutureObj")<'\_, T>

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#123)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#315)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<T>

where T: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#316)

#### type [Output](#associatedtype.Output) = <T as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#79)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [LocalFutureObj](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/futures_task/future_obj/struct.LocalFutureObj.html "struct futures_task::future_obj::LocalFutureObj")<'\_, T>

[Source](https://docs.rs/futures-task/0.3.32/x86_64-unknown-linux-gnu/src/futures_task/future_obj.rs.html#80)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/mutex.rs.html#380-385)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [LockArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/mutex/struct.LockArc.html "struct async_lock::mutex::LockArc")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/mutex.rs.html#380-385)

#### type [Output](#associatedtype.Output) = [MutexGuardArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/mutex/struct.MutexGuardArc.html "struct async_lock::mutex::MutexGuardArc")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/lock/mutex.rs.html#226)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [OwnedMutexLockFuture](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/lock/mutex/struct.OwnedMutexLockFuture.html "struct futures_util::lock::mutex::OwnedMutexLockFuture")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/lock/mutex.rs.html#227)

#### type [Output](#associatedtype.Output) = [OwnedMutexGuard](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/lock/mutex/struct.OwnedMutexGuard.html "struct futures_util::lock::mutex::OwnedMutexGuard")<T>

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/pending.rs.html#38)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for bevy::tasks::futures\_lite::future::[Pending](../future/struct.Pending.html "struct bevy::tasks::futures_lite::future::Pending")<T>

[Source](https://doc.rust-lang.org/nightly/src/core/future/pending.rs.html#39)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/pending.rs.html#41)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::future::pending::[Pending](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/pending/struct.Pending.html "struct futures_util::future::pending::Pending")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/pending.rs.html#42)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#520)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for async\_io::reactor::[Readable](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/reactor/struct.Readable.html "struct async_io::reactor::Readable")<'\_, T>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#521)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#541)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [ReadableOwned](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/reactor/struct.ReadableOwned.html "struct async_io::reactor::ReadableOwned")<T>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#542)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/ready.rs.html#18)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for bevy::tasks::futures\_lite::future::[Ready](../future/struct.Ready.html "struct bevy::tasks::futures_lite::future::Ready")<T>

[Source](https://doc.rust-lang.org/nightly/src/core/future/ready.rs.html#19)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/ready.rs.html#27)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_util::future::ready::[Ready](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/futures_util/future/ready/struct.Ready.html "struct futures_util::future::ready::Ready")<T>

[Source](https://docs.rs/futures-util/0.3.32/x86_64-unknown-linux-gnu/src/futures_util/future/ready.rs.html#28)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/oneshot.rs.html#455)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for futures\_channel::oneshot::[Receiver](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/oneshot/struct.Receiver.html "struct futures_channel::oneshot::Receiver")<T>

[Source](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/src/futures_channel/oneshot.rs.html#456)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Canceled](https://docs.rs/futures-channel/0.3.32/x86_64-unknown-linux-gnu/futures_channel/oneshot/struct.Canceled.html "struct futures_channel::oneshot::Canceled")\>

[Source](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/src/tokio/sync/oneshot.rs.html#1268)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for tokio::sync::oneshot::[Receiver](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/tokio/sync/oneshot/struct.Receiver.html "struct tokio::sync::oneshot::Receiver")<T>

[Source](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/src/tokio/sync/oneshot.rs.html#1269)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RecvError](https://docs.rs/tokio/1.52.3/x86_64-unknown-linux-gnu/tokio/sync/oneshot/error/struct.RecvError.html "struct tokio::sync::oneshot::error::RecvError")\>

[Source](https://doc.rust-lang.org/nightly/src/core/sync/sync_view.rs.html#288-290)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [SyncView](https://doc.rust-lang.org/nightly/core/sync/sync_view/struct.SyncView.html "struct core::sync::sync_view::SyncView")<T>

where T: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/sync/sync_view.rs.html#292)

#### type [Output](#associatedtype.Output) = <T as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#399-404)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [UpgradeArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/futures/struct.UpgradeArc.html "struct async_lock::rwlock::futures::UpgradeArc")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/src/async_lock/rwlock/futures.rs.html#399-404)

#### type [Output](#associatedtype.Output) = [RwLockWriteGuardArc](https://docs.rs/async-lock/3.4.2/x86_64-unknown-linux-gnu/async_lock/rwlock/struct.RwLockWriteGuardArc.html "struct async_lock::rwlock::RwLockWriteGuardArc")<T>

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#380)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<T>

where T: [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future"),

Available on **crate feature `std`** only.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#381)

#### type [Output](#associatedtype.Output) = <T as [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future")\>::[Output](../trait.Future.html#associatedtype.Output "type bevy::tasks::futures_lite::Future::Output")

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#562)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for async\_io::reactor::[Writable](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/reactor/struct.Writable.html "struct async_io::reactor::Writable")<'\_, T>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#563)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#583)

### impl<T> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WritableOwned](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/reactor/struct.WritableOwned.html "struct async_io::reactor::WritableOwned")<T>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/reactor.rs.html#584)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2998)

### impl<W> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [CloseFuture](../io/struct.CloseFuture.html "struct bevy::tasks::futures_lite::io::CloseFuture")<'\_, W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2999)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2981)

### impl<W> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [FlushFuture](../io/struct.FlushFuture.html "struct bevy::tasks::futures_lite::io::FlushFuture")<'\_, W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2982)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2952)

### impl<W> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WriteAllFuture](../io/struct.WriteAllFuture.html "struct bevy::tasks::futures_lite::io::WriteAllFuture")<'\_, W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2953)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2914)

### impl<W> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WriteFuture](../io/struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'\_, W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2915)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2933)

### impl<W> [Future](../trait.Future.html "trait bevy::tasks::futures_lite::Future") for [WriteVectoredFuture](../io/struct.WriteVectoredFuture.html "struct bevy::tasks::futures_lite::io::WriteVectoredFuture")<'\_, W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2934)

#### type [Output](#associatedtype.Output) = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>