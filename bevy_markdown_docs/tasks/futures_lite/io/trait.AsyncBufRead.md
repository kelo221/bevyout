[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Trait AsyncBufRead 

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#254)

```rust
pub trait AsyncBufRead: AsyncRead {
    // Required methods
    fn poll_fill_buf(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<&[u8], Error>>;
    fn consume(self: Pin<&mut Self>, amt: usize);
}
```

Available on **crate feature `std`** only.

Read bytes asynchronously.

This trait is analogous to the `std::io::BufRead` trait, but integrates with the asynchronous task system. In particular, the `poll_fill_buf` method, unlike `BufRead::fill_buf`, will automatically queue the current task for wakeup and return if data is not yet available, rather than blocking the calling thread.

## Required Methods

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#283)

#### fn [poll\_fill\_buf](#tymethod.poll_fill_buf)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.

On success, returns `Poll::Ready(Ok(buf))`.

If no data is available for reading, the method returns `Poll::Pending` and arranges for the current task (via `cx.waker().wake_by_ref()`) to receive a notification when the object becomes readable or is closed.

This function is a lower-level call. It needs to be paired with the [`consume`](../trait.AsyncBufRead.html#tymethod.consume "method bevy::tasks::futures_lite::AsyncBufRead::consume") method to function properly. When calling this method, none of the contents will be “read” in the sense that later calling [`poll_read`](../trait.AsyncRead.html#tymethod.poll_read "method bevy::tasks::futures_lite::AsyncRead::poll_read") may return the same contents. As such, [`consume`](../trait.AsyncBufRead.html#tymethod.consume "method bevy::tasks::futures_lite::AsyncBufRead::consume") must be called with the number of bytes that are consumed from this buffer to ensure that the bytes are never returned twice.

An empty buffer returned indicates that the stream has reached EOF.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#300)

#### fn [consume](#tymethod.consume)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, amt: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Tells this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to [`poll_read`](../trait.AsyncRead.html#tymethod.poll_read "method bevy::tasks::futures_lite::AsyncRead::poll_read").

This function is a lower-level call. It needs to be paired with the [`poll_fill_buf`](../trait.AsyncBufRead.html#tymethod.poll_fill_buf "method bevy::tasks::futures_lite::AsyncBufRead::poll_fill_buf") method to function properly. This function does not perform any I/O, it simply informs this object that some amount of its buffer, returned from [`poll_fill_buf`](../trait.AsyncBufRead.html#tymethod.poll_fill_buf "method bevy::tasks::futures_lite::AsyncBufRead::poll_fill_buf"), has been consumed and should no longer be returned. As such, this function may do odd things if [`poll_fill_buf`](../trait.AsyncBufRead.html#tymethod.poll_fill_buf "method bevy::tasks::futures_lite::AsyncBufRead::poll_fill_buf") isn’t called before calling it.

The `amt` must be `<=` the number of bytes in the buffer returned by [`poll_fill_buf`](../trait.AsyncBufRead.html#tymethod.poll_fill_buf "method bevy::tasks::futures_lite::AsyncBufRead::poll_fill_buf").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#555)

### impl [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#556)

#### fn [poll\_fill\_buf](#tymethod.poll_fill_buf)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>, \_: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#556)

#### fn [consume](#tymethod.consume)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>, amt: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#772)

### impl [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Reader](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Reader.html "struct piper::Reader")

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#773)

#### fn [poll\_fill\_buf](#tymethod.poll_fill_buf)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Reader](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Reader.html "struct piper::Reader")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/src/piper/lib.rs.html#780)

#### fn [consume](#tymethod.consume)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Reader](https://docs.rs/piper/0.2.5/x86_64-unknown-linux-gnu/piper/struct.Reader.html "struct piper::Reader")\>, amt: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#529-532)

### impl<P> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>

where P: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#534)

#### fn [poll\_fill\_buf](#tymethod.poll_fill_buf)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#538)

#### fn [consume](#tymethod.consume)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, amt: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#525)

### impl<T> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#526)

#### fn [poll\_fill\_buf](#tymethod.poll_fill_buf)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#526)

#### fn [consume](#tymethod.consume)(self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, amt: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1465)

### impl [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Empty](struct.Empty.html "struct bevy::tasks::futures_lite::io::Empty")

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2698)

### impl<R1, R2> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Chain](struct.Chain.html "struct bevy::tasks::futures_lite::io::Chain")<R1, R2>

where R1: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"), R2: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#839)

### impl<R> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [BufReader](struct.BufReader.html "struct bevy::tasks::futures_lite::io::BufReader")<R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2518)

### impl<R> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Take](struct.Take.html "struct bevy::tasks::futures_lite::io::Take")<R>

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#521)

### impl<T> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1353-1355)

### impl<T> [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") for [Cursor](struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),