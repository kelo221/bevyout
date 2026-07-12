[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[prelude](index.html)

# Trait AsyncRead 

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#47)

```rust
pub trait AsyncRead {
    // Required method
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>>;

    // Provided method
    fn poll_read_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
    ) -> Poll<Result<usize, Error>> { ... }
}
```

Read bytes asynchronously.

This trait is analogous to the `std::io::Read` trait, but integrates with the asynchronous task system. In particular, the `poll_read` method, unlike `Read::read`, will automatically queue the current task for wakeup and return if data is not yet available, rather than blocking the calling thread.

## Required Methods

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#63-67)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to read from the `AsyncRead` into `buf`.

On success, returns `Poll::Ready(Ok(num_bytes_read))`.

If no data is available for reading, the method returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object becomes readable or is closed.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

## Provided Methods

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#91-95)

#### fn [poll\_read\_vectored](#method.poll_read_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to read from the `AsyncRead` into `bufs` using vectored IO operations.

This method is similar to `poll_read`, but allows data to be read into multiple buffers using a single operation.

On success, returns `Poll::Ready(Ok(num_bytes_read))`.

If no data is available for reading, the method returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object becomes readable or is closed. By default, this method delegates to using `poll_read` on the first nonempty buffer in `bufs`, or an empty one if none exists. Objects which support vectored IO should override this method.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#373)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#374)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#374)

#### fn [poll\_read\_vectored](#method.poll_read_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#648)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [ChildStderr](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStderr.html "struct async_process::ChildStderr")

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#649-653)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ChildStderr](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStderr.html "struct async_process::ChildStderr")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#575)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [ChildStdout](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStdout.html "struct async_process::ChildStdout")

[Source](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/src/async_process/lib.rs.html#576-580)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [ChildStdout](https://docs.rs/async-process/2.5.0/x86_64-unknown-linux-gnu/async_process/struct.ChildStdout.html "struct async_process::ChildStdout")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1153)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1154-1158)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#761)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Reader](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Reader.html "struct piper::Reader")

Available on **crate feature `std`** only.

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#762-766)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Reader](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Reader.html "struct piper::Reader")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#331-334)

### impl<P> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>

where P: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#336-340)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#344-348)

#### fn [poll\_read\_vectored](#method.poll_read_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1301-1303)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>

where [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1305-1309)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1319-1323)

#### fn [poll\_read\_vectored](#method.poll_read_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &[Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#327)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#328)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#328)

#### fn [poll\_read\_vectored](#method.poll_read_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1269)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>

where T: [IoSafe](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/trait.IoSafe.html "trait async_io::IoSafe") + [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1270-1274)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/lib.rs.html#1284-1288)

#### fn [poll\_read\_vectored](#method.poll_read_vectored)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Async](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/async_io/struct.Async.html "struct async_io::Async")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#803)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>

where T: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#804-808)

#### fn [poll\_read](#tymethod.poll_read)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1458)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Empty](../io/struct.Empty.html "struct bevy::tasks::futures_lite::io::Empty")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1500)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Repeat](../io/struct.Repeat.html "struct bevy::tasks::futures_lite::io::Repeat")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/processor_gated.rs.html#131)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [TransactionLockedReader](../../../asset/io/processor_gated/struct.TransactionLockedReader.html "struct bevy::asset::io::processor_gated::TransactionLockedReader")<'\_>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#616)

### impl [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [VecReader](../../../asset/io/struct.VecReader.html "struct bevy::asset::io::VecReader")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#669)

### impl<'a> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [SliceReader](../../../asset/io/struct.SliceReader.html "struct bevy::asset::io::SliceReader")<'a>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2662)

### impl<R1, R2> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Chain](../io/struct.Chain.html "struct bevy::tasks::futures_lite::io::Chain")<R1, R2>

where R1: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"), R2: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#801)

### impl<R> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [BufReader](../io/struct.BufReader.html "struct bevy::tasks::futures_lite::io::BufReader")<R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2571)

### impl<R> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Bytes](../io/struct.Bytes.html "struct bevy::tasks::futures_lite::io::Bytes")<R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2485)

### impl<R> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Take](../io/struct.Take.html "struct bevy::tasks::futures_lite::io::Take")<R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#222)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [AssertAsync](../io/struct.AssertAsync.html "struct bevy::tasks::futures_lite::io::AssertAsync")<T>

where T: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#323)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1332-1334)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [Cursor](../io/struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#3058)

### impl<T> [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") for [ReadHalf](../io/struct.ReadHalf.html "struct bevy::tasks::futures_lite::io::ReadHalf")<T>

where T: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),