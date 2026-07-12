[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[prelude](index.html)

# Trait AsyncWrite 

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#113)

```rust
pub trait AsyncWrite {
    // Required methods
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>>;
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Error>>;
    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Error>>;

    // Provided method
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<Result<usize, Error>> { ... }
}
```

Write bytes asynchronously.

This trait is analogous to the `std::io::Write` trait, but integrates with the asynchronous task system. In particular, the `poll_write` method, unlike `Write::write`, will automatically queue the current task for wakeup and return if the writer cannot take more data, rather than blocking the calling thread.

## Required Methods

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#132-136)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to write bytes from `buf` into the object.

On success, returns `Poll::Ready(Ok(num_bytes_written))`.

If the object is not ready for writing, the method returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object becomes writable or is closed.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

`poll_write` must try to make progress by flushing the underlying object if that is the only way the underlying object can become writable again.

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#193)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to flush the object, ensuring that any buffered data reach their destination.

On success, returns `Poll::Ready(Ok(()))`.

If flushing cannot immediately complete, this method returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object can make progress towards flushing.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

It only makes sense to do anything here if you actually buffer data.

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#210)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to close the object.

On success, returns `Poll::Ready(Ok(()))`.

If closing cannot immediately complete, this function returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object can make progress towards closing.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

## Provided Methods

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#161-165)

#### fn [poll\_write\_vectored](#method.poll_write_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &\[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to write bytes from `bufs` into the object using vectored IO operations.

This method is similar to `poll_write`, but allows data from multiple buffers to be written using a single operation.

On success, returns `Poll::Ready(Ok(num_bytes_written))`.

If the object is not ready for writing, the method returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object becomes writable or is closed.

By default, this method delegates to using `poll_write` on the first nonempty buffer in `bufs`, or an empty one if none exists. Objects which support vectored IO should override this method.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#483)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [ChildStdin](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStdin.html "struct async_process::ChildStdin")

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#484-488)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ChildStdin](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStdin.html "struct async_process::ChildStdin")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#492)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ChildStdin](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStdin.html "struct async_process::ChildStdin")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#496)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ChildStdin](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStdin.html "struct async_process::ChildStdin")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1176)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1177-1181)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1187)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1195)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#1173)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Writer](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Writer.html "struct piper::Writer")

Available on **crate feature `std`** only.

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#1174-1178)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Writer](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Writer.html "struct piper::Writer")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#1182)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Writer](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Writer.html "struct piper::Writer")\>, \_cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#1187)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Writer](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Writer.html "struct piper::Writer")\>, \_cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#413-416)

### impl<P> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>

where P: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#418-422)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#426-430)

#### fn [poll\_write\_vectored](#method.poll_write_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &\[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#434)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#438)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1378-1380)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>

where [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1382-1386)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1396-1400)

#### fn [poll\_write\_vectored](#method.poll_write_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &\[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1410)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1420)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#409)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#410)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#410)

#### fn [poll\_write\_vectored](#method.poll_write_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &\[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#410)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#410)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1334)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>

where T: [IoSafe](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/trait.IoSafe.html "trait async_io::IoSafe") + [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1335-1339)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1349-1353)

#### fn [poll\_write\_vectored](#method.poll_write_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &\[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1363)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1373)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#871)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>

where T: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#872-876)

#### fn [poll\_write](#tymethod.poll_write)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#925)

#### fn [poll\_flush](#tymethod.poll_flush)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#944)

#### fn [poll\_close](#tymethod.poll_close)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1392)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Cursor](../io/struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")<&mut [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1366)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Cursor](../io/struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")<&mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1410)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Cursor](../io/struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")<[Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1532)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Sink](../io/struct.Sink.html "struct bevy::tasks::futures_lite::io::Sink")

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#471)

### impl [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#928)

### impl<R> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [BufReader](../io/struct.BufReader.html "struct bevy::tasks::futures_lite::io::BufReader")<R>

where R: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#242)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [AssertAsync](../io/struct.AssertAsync.html "struct bevy::tasks::futures_lite::io::AssertAsync")<T>

where T: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#405)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#3078)

### impl<T> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [WriteHalf](../io/struct.WriteHalf.html "struct bevy::tasks::futures_lite::io::WriteHalf")<T>

where T: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1147)

### impl<W> [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") for [BufWriter](../io/struct.BufWriter.html "struct bevy::tasks::futures_lite::io::BufWriter")<W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite"),