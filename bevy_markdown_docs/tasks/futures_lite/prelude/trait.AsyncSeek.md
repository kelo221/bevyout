[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[prelude](index.html)

# Trait AsyncSeek 

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#220)

```rust
pub trait AsyncSeek {
    // Required method
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<Result<u64, Error>>;
}
```

Seek bytes asynchronously.

This trait is analogous to the `std::io::Seek` trait, but integrates with the asynchronous task system. In particular, the `poll_seek` method, unlike `Seek::seek`, will automatically queue the current task for wakeup and return if data is not yet available, rather than blocking the calling thread.

## Required Methods

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#240-244)

#### fn [poll\_seek](#tymethod.poll_seek)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut Self>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, pos: [SeekFrom](../io/enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom"), ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

Attempt to seek to an offset, in bytes, in a stream.

A seek beyond the end of a stream is allowed, but behavior is defined by the implementation.

If the seek operation completed successfully, this method returns the new position from the start of the stream. That position can be used later with [`SeekFrom::Start`](../io/enum.SeekFrom.html#variant.Start "variant bevy::tasks::futures_lite::io::SeekFrom::Start").

##### Errors

Seeking to a negative offset is considered an error.

##### Implementation

This function may not return errors of kind `WouldBlock` or `Interrupted`. Implementations must convert `WouldBlock` into `Poll::Pending` and either internally retry or convert `Interrupted` into another error kind.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1200)

### impl [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")

[Source](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/src/async_fs/lib.rs.html#1201-1205)

#### fn [poll\_seek](#tymethod.poll_seek)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [File](https://docs.rs/async-fs/2.2.0/x86_64-unknown-linux-gnu/async_fs/struct.File.html "struct async_fs::File")\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, pos: [SeekFrom](../io/enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom"), ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#495-498)

### impl<P> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>

where P: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"), <P as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#500-504)

#### fn [poll\_seek](#tymethod.poll_seek)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<P>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, pos: [SeekFrom](../io/enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom"), ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#491)

### impl<T> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#492)

#### fn [poll\_seek](#tymethod.poll_seek)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, pos: [SeekFrom](../io/enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom"), ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#954)

### impl<T> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>

where T: [Seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/src/blocking/lib.rs.html#955-959)

#### fn [poll\_seek](#tymethod.poll_seek)( self: [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<&mut [Unblock](https://docs.rs/blocking/1.6.2/x86_64-unknown-linux-gnu/blocking/struct.Unblock.html "struct blocking::Unblock")<T>>, cx: &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>, pos: [SeekFrom](../io/enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom"), ) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](../io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#628)

### impl [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [VecReader](../../../asset/io/struct.VecReader.html "struct bevy::asset::io::VecReader")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#679)

### impl<'a> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [SliceReader](../../../asset/io/struct.SliceReader.html "struct bevy::asset::io::SliceReader")<'a>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#873)

### impl<R> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [BufReader](../io/struct.BufReader.html "struct bevy::tasks::futures_lite::io::BufReader")<R>

where R: [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#272)

### impl<T> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [AssertAsync](../io/struct.AssertAsync.html "struct bevy::tasks::futures_lite::io::AssertAsync")<T>

where T: [Seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek"),

[Source](https://docs.rs/futures-io/0.3.32/x86_64-unknown-linux-gnu/src/futures_io/lib.rs.html#487)

### impl<T> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1319-1321)

### impl<T> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [Cursor](../io/struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]> + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1174)

### impl<W> [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") for [BufWriter](../io/struct.BufWriter.html "struct bevy::tasks::futures_lite::io::BufWriter")<W>

where W: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek"),