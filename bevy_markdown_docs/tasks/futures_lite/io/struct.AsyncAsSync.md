[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Struct AsyncAsSync 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#350)

```rust
pub struct AsyncAsSync<'r, 'ctx, T> {
    pub context: &'r mut Context<'ctx>,
    pub inner: T,
}
```

Available on **crate feature `std`** only.

A wrapper around a type that implements `AsyncRead` or `AsyncWrite` that converts `Pending` polls to `WouldBlock` errors.

This wrapper can be used as a compatibility layer between `AsyncRead` and `Read`, for types that take `Read` as a parameter.

## Examples

```rust
use std::io::Read;
use std::task::{Poll, Context};

fn poll_for_io(cx: &mut Context<'_>) -> Poll<usize> {
    // Assume we have a library that's built around `Read` and `Write` traits.
    use cooltls::Session;

    // We want to use it with our writer that implements `AsyncWrite`.
    let writer = Stream::new();

    // First, we wrap our `Writer` with `AsyncAsSync` to convert `Pending` polls to `WouldBlock`.
    use futures_lite::io::AsyncAsSync;
    let writer = AsyncAsSync::new(cx, writer);

    // Now, we can use it with `cooltls`.
    let mut session = Session::new(writer);

    // Match on the result of `read()` and translate it to poll.
    match session.read(&mut [0; 1024]) {
        Ok(n) => Poll::Ready(n),
        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => Poll::Pending,
        Err(err) => panic!("unexpected error: {}", err),
    }
}

// Usually, poll-based functions are best wrapped using `poll_fn`.
use futures_lite::future::poll_fn;
poll_fn(|cx| poll_for_io(cx)).await;
```

## Fields

`context: &'r mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'ctx>`

The context we are using to poll the future.

`inner: T`

The actual reader/writer we are wrapping.

## Implementations

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#358)

### impl<'r, 'ctx, T> [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#375)

#### pub fn [new](#method.new)(context: &'r mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'ctx>, inner: T) -> [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T> [ⓘ](#)

Wraps an I/O handle implementing [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") or [`AsyncWrite`](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") traits.

##### Examples

```rust
use futures_lite::io::AsyncAsSync;
use std::task::Context;
use waker_fn::waker_fn;

let reader: &[u8] = b"hello";
let waker = waker_fn(|| {});
let mut context = Context::from_waker(&waker);

let async_reader = AsyncAsSync::new(&mut context, reader);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#396-398)

#### pub fn [close](#method.close)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Attempt to shutdown the I/O handle.

##### Examples

```rust
use futures_lite::io::AsyncAsSync;
use std::task::Context;
use waker_fn::waker_fn;

let reader: Vec<u8> = b"hello".to_vec();
let waker = waker_fn(|| {});
let mut context = Context::from_waker(&waker);

let mut async_reader = AsyncAsSync::new(&mut context, reader);
async_reader.close().unwrap();
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#421-426)

#### pub fn [poll\_with](#method.poll_with)<R>( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, &mut [Context](https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html "struct core::task::wake::Context")<'\_>) -> [Poll](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<R, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<R, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Poll this `AsyncAsSync` for some function.

##### Examples

```rust
use futures_lite::io::{AsyncAsSync, AsyncRead};
use std::task::Context;
use waker_fn::waker_fn;

let reader: &[u8] = b"hello";
let waker = waker_fn(|| {});
let mut context = Context::from_waker(&waker);

let mut async_reader = AsyncAsSync::new(&mut context, reader);
let r = async_reader.poll_with(|io, cx| io.poll_read(cx, &mut [0; 1024]));
assert_eq!(r.unwrap(), 5);
```

## Trait Implementations

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#478)

### impl<T> [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<T> for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#480)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#471)

### impl<T> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<T> for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#473)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#485)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#487)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#492)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#494)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#349)

### impl<'r, 'ctx, T> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#349)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#435)

### impl<T> [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

where T: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#437)

#### fn [read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read)(&mut self, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Pull some bytes from this source into the specified buffer, returning how many bytes were read. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#tymethod.read)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#442)

#### fn [read\_vectored](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored)(&mut self, bufs: &mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'\_>\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Like `read`, except that it reads into a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_vectored)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#822)

#### fn [is\_read\_vectored](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Read`er has an efficient `read_vectored` implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.is_read_vectored)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#919)

#### fn [read\_to\_end](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)(&mut self, buf: &mut [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads all bytes until EOF in this source, placing them into `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_end)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#975)

#### fn [read\_to\_string](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)(&mut self, buf: &mut [String](../../../prelude/struct.String.html "struct bevy::prelude::String")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads all bytes until EOF in this source, appending them to `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_to_string)

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1028)

#### fn [read\_exact](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact)(&mut self, buf: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads the exact number of bytes required to fill `buf`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_exact)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1041)

#### fn [read\_buf](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf)(&mut self, buf: [BorrowedCursor](https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html "struct core::io::borrowed_buf::BorrowedCursor")<'\_, [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

🔬This is a nightly-only experimental API. (`read_buf`)

Pull some bytes from this source into the specified buffer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1064)

#### fn [read\_buf\_exact](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact)( &mut self, cursor: [BorrowedCursor](https://doc.rust-lang.org/nightly/core/io/borrowed_buf/struct.BorrowedCursor.html "struct core::io::borrowed_buf::BorrowedCursor")<'\_, [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

🔬This is a nightly-only experimental API. (`read_buf`)

Reads the exact number of bytes required to fill `cursor`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_buf_exact)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1103-1105)

#### fn [by\_ref](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref)(&mut self) -> &mut Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a “by reference” adapter for this instance of `Read`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.by_ref)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1146-1148)

#### fn [bytes](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes)(self) -> [Bytes](https://doc.rust-lang.org/nightly/std/io/struct.Bytes.html "struct std::io::Bytes")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Transforms this `Read` instance to an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over its bytes. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.bytes)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1184-1186)

#### fn [chain](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain)<R>(self, next: R) -> [Chain](https://doc.rust-lang.org/nightly/core/io/util/struct.Chain.html "struct core::io::util::Chain")<Self, R>

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an adapter which will chain this stream with another. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.chain)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1223-1225)

#### fn [take](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take)(self, limit: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Take](https://doc.rust-lang.org/nightly/core/io/util/struct.Take.html "struct core::io::util::Take")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an adapter which will read at most `limit` bytes from it. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.take)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1258-1260)

#### fn [read\_array](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_array)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`read_array`)

Read and return a fixed array of bytes from this source. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_array)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1296-1298)

#### fn [read\_le](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_le)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [FromEndianBytes](https://doc.rust-lang.org/nightly/std/io/trait.FromEndianBytes.html "trait std::io::FromEndianBytes"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`read_le`)

Read and return a type (e.g. an integer) in little-endian order. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_le)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1329-1331)

#### fn [read\_be](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_be)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [FromEndianBytes](https://doc.rust-lang.org/nightly/std/io/trait.FromEndianBytes.html "trait std::io::FromEndianBytes"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

🔬This is a nightly-only experimental API. (`read_le`)

Read and return a type (e.g. an integer) in big-endian order. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Read.html#method.read_be)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#464)

### impl<T> [Seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

where T: [AsyncSeek](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#466)

#### fn [seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek)(&mut self, pos: [SeekFrom](enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Seek to an offset, in bytes, in a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#tymethod.seek)

1.55.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1837)

#### fn [rewind](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Rewind to the beginning of a stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.rewind)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1877)

#### fn [stream\_len](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

🔬This is a nightly-only experimental API. (`seek_stream_len`)

Returns the length of this stream (in bytes). [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_len)

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1905)

#### fn [stream\_position](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Returns the current seek position from the start of the stream. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.stream_position)

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1933)

#### fn [seek\_relative](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative)(&mut self, offset: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Seeks relative to the current position. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html#method.seek_relative)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#447)

### impl<T> [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'\_, '\_, T>

where T: [AsyncWrite](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#449)

#### fn [write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)(&mut self, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#454)

#### fn [write\_vectored](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)(&mut self, bufs: &\[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write "method std::io::Write::write"), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#459)

#### fn [flush](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1546)

#### fn [is\_write\_vectored](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored "method std::io::Write::write_vectored") implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1608)

#### fn [write\_all](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)(&mut self, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

[Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1670)

#### fn [write\_all\_vectored](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)(&mut self, bufs: &mut \[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'\_>\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

🔬This is a nightly-only experimental API. (`write_all_vectored`)

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1723)

#### fn [write\_fmt](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)(&mut self, args: [Arguments](https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html "struct core::fmt::Arguments")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1753-1755)

#### fn [by\_ref](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)(&mut self) -> &mut Self

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates a “by reference” adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

## Auto Trait Implementations

### impl<'r, 'ctx, T> ![Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

### impl<'r, 'ctx, T> ![Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

### impl<'r, 'ctx, T> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

### impl<'r, 'ctx, T> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

where T: [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze"),

### impl<'r, 'ctx, T> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

where T: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<'r, 'ctx, T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

where T: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<'r, 'ctx, T> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")<'r, 'ctx, T>

where T: [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin"),

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#43)

### impl<R> [ReadBytes](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html "trait endi::io::ReadBytes") for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#44)

#### fn [read\_u8](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_u8)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `u8`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#45)

#### fn [read\_u16](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_u16)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `u16`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#46)

#### fn [read\_u32](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_u32)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `u32`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#47)

#### fn [read\_u64](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_u64)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `u64`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#48)

#### fn [read\_u128](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_u128)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `u128`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#50)

#### fn [read\_i8](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_i8)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `i8`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#51)

#### fn [read\_i16](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_i16)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `i16`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#52)

#### fn [read\_i32](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_i32)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `i32`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#53)

#### fn [read\_i64](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_i64)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `i64`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#54)

#### fn [read\_i128](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_i128)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `i128`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#56)

#### fn [read\_f32](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_f32)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `f32`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#57)

#### fn [read\_f64](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.ReadBytes.html#tymethod.read_f64)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read a `f64`.

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1049)

### impl<R> [ReadBytesExt](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html "trait byteorder::io::ReadBytesExt") for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#55)

#### fn [read\_u8](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u8)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads an unsigned 8 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u8)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#85)

#### fn [read\_i8](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i8)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads a signed 8 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i8)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#112)

#### fn [read\_u16](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u16)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned 16 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u16)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#139)

#### fn [read\_i16](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i16)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed 16 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i16)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#165)

#### fn [read\_u24](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u24)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned 24 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u24)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#191)

#### fn [read\_i24](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i24)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed 24 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i24)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#217)

#### fn [read\_u32](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u32)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned 32 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u32)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#243)

#### fn [read\_i32](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i32)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed 32 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i32)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#269)

#### fn [read\_u48](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u48)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned 48 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u48)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#295)

#### fn [read\_i48](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i48)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed 48 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i48)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#321)

#### fn [read\_u64](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u64)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned 64 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u64)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#347)

#### fn [read\_i64](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i64)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed 64 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i64)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#376)

#### fn [read\_u128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u128)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned 128 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u128)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#402)

#### fn [read\_i128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i128)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed 128 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i128)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#427)

#### fn [read\_uint](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_uint)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned n-bytes integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_uint)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#452)

#### fn [read\_int](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_int)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed n-bytes integer from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_int)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#460)

#### fn [read\_uint128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_uint128)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads an unsigned n-bytes integer from the underlying reader.

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#468)

#### fn [read\_int128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_int128)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a signed n-bytes integer from the underlying reader.

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#499)

#### fn [read\_f32](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f32)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a IEEE754 single-precision (4 bytes) floating point number from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f32)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#530)

#### fn [read\_f64](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f64)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a IEEE754 double-precision (8 bytes) floating point number from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f64)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#562)

#### fn [read\_u16\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u16_into)<T>(&mut self, dst: &mut \[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of unsigned 16 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u16_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#597)

#### fn [read\_u32\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u32_into)<T>(&mut self, dst: &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of unsigned 32 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u32_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#635)

#### fn [read\_u64\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u64_into)<T>(&mut self, dst: &mut \[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of unsigned 64 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u64_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#673-676)

#### fn [read\_u128\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u128_into)<T>(&mut self, dst: &mut \[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of unsigned 128 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_u128_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#716)

#### fn [read\_i8\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i8_into)(&mut self, dst: &mut \[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads a sequence of signed 8 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i8_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#747)

#### fn [read\_i16\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i16_into)<T>(&mut self, dst: &mut \[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of signed 16 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i16_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#782)

#### fn [read\_i32\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i32_into)<T>(&mut self, dst: &mut \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of signed 32 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i32_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#820)

#### fn [read\_i64\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i64_into)<T>(&mut self, dst: &mut \[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of signed 64 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i64_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#858-861)

#### fn [read\_i128\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i128_into)<T>(&mut self, dst: &mut \[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of signed 128 bit integers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_i128_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#902)

#### fn [read\_f32\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f32_into)<T>(&mut self, dst: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of IEEE754 single-precision (4 bytes) floating point numbers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f32_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#948-951)

#### fn [read\_f32\_into\_unchecked](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f32_into_unchecked)<T>(&mut self, dst: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

👎Deprecated since 1.2.0:

please use `read_f32_into` instead

**DEPRECATED**. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f32_into_unchecked)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#987)

#### fn [read\_f64\_into](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f64_into)<T>(&mut self, dst: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Reads a sequence of IEEE754 double-precision (8 bytes) floating point numbers from the underlying reader. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f64_into)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1039-1042)

#### fn [read\_f64\_into\_unchecked](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f64_into_unchecked)<T>(&mut self, dst: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

👎Deprecated since 1.2.0:

please use `read_f64_into` instead

**DEPRECATED**. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.ReadBytesExt.html#method.read_f64_into_unchecked)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#536)

### impl<R> [ReadBytesExt](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html "trait byteorder_lite::io::ReadBytesExt") for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#52)

#### fn [read\_u8](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u8)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads an unsigned 8 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u8)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#82)

#### fn [read\_i8](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i8)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Reads a signed 8 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i8)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#109)

#### fn [read\_u16](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u16)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned 16 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u16)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#136)

#### fn [read\_i16](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i16)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed 16 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i16)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#162)

#### fn [read\_u24](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u24)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned 24 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u24)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#188)

#### fn [read\_i24](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i24)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed 24 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i24)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#214)

#### fn [read\_u32](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u32)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned 32 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u32)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#240)

#### fn [read\_i32](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i32)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed 32 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i32)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#266)

#### fn [read\_u48](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u48)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned 48 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u48)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#292)

#### fn [read\_i48](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i48)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed 48 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i48)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#318)

#### fn [read\_u64](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u64)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned 64 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u64)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#344)

#### fn [read\_i64](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i64)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed 64 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i64)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#373)

#### fn [read\_u128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u128)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned 128 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_u128)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#399)

#### fn [read\_i128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i128)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed 128 bit integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_i128)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#424)

#### fn [read\_uint](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_uint)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned n-bytes integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_uint)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#449)

#### fn [read\_int](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_int)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed n-bytes integer from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_int)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#457)

#### fn [read\_uint128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_uint128)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads an unsigned n-bytes integer from the underlying reader.

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#465)

#### fn [read\_int128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_int128)<T>(&mut self, nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a signed n-bytes integer from the underlying reader.

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#496)

#### fn [read\_f32](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_f32)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a IEEE754 single-precision (4 bytes) floating point number from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_f32)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#527)

#### fn [read\_f64](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_f64)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Reads a IEEE754 double-precision (8 bytes) floating point number from the underlying reader. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.ReadBytesExt.html#method.read_f64)

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<\[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\]> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut \[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut \[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)(&mut self, value: &mut [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<R> [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\> for R

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_little\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_little_endian_into)( &mut self, value: &mut [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [read\_from\_big\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#tymethod.read_from_big_endian_into)(&mut self, value: &mut [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#317)

#### fn [read\_from\_native\_endian\_into](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian_into)(&mut self, value: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#324)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_little_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#332)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_big_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#340)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html#method.read_from_native_endian)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Read the byte value of the inferred type

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#98)

### impl<W> [WriteBytes](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html "trait endi::io::WriteBytes") for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#99)

#### fn [write\_u8](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_u8)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `u8`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#100)

#### fn [write\_u16](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_u16)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `u16`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#101)

#### fn [write\_u32](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_u32)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `u32`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#102)

#### fn [write\_u64](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_u64)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `u64`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#103)

#### fn [write\_u128](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_u128)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `u128`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#105)

#### fn [write\_i8](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_i8)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `i8`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#106)

#### fn [write\_i16](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_i16)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `i16`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#107)

#### fn [write\_i32](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_i32)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `i32`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#108)

#### fn [write\_i64](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_i64)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `i64`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#109)

#### fn [write\_i128](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_i128)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `i128`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#111)

#### fn [write\_f32](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_f32)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `f32`.

[Source](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/src/endi/io.rs.html#112)

#### fn [write\_f64](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/io/trait.WriteBytes.html#tymethod.write_f64)(&mut self, endian: [Endian](https://docs.rs/endi/1.1.1/x86_64-unknown-linux-gnu/endi/endian/enum.Endian.html "enum endi::endian::Endian"), n: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write a `f64`.

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1579)

### impl<W> [WriteBytesExt](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html "trait byteorder::io::WriteBytesExt") for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1098)

#### fn [write\_u8](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u8)(&mut self, n: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Writes an unsigned 8 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u8)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1126)

#### fn [write\_i8](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i8)(&mut self, n: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Writes a signed 8 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i8)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1151)

#### fn [write\_u16](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u16)<T>(&mut self, n: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned 16 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u16)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1178)

#### fn [write\_i16](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i16)<T>(&mut self, n: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed 16 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i16)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1205)

#### fn [write\_u24](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u24)<T>(&mut self, n: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned 24 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u24)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1232)

#### fn [write\_i24](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i24)<T>(&mut self, n: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed 24 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i24)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1259)

#### fn [write\_u32](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u32)<T>(&mut self, n: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned 32 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u32)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1286)

#### fn [write\_i32](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i32)<T>(&mut self, n: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed 32 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i32)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1313)

#### fn [write\_u48](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u48)<T>(&mut self, n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned 48 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u48)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1340)

#### fn [write\_i48](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i48)<T>(&mut self, n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed 48 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i48)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1367)

#### fn [write\_u64](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u64)<T>(&mut self, n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned 64 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u64)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1394)

#### fn [write\_i64](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i64)<T>(&mut self, n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed 64 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i64)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1402)

#### fn [write\_u128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_u128)<T>(&mut self, n: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned 128 bit integer to the underlying writer.

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1410)

#### fn [write\_i128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_i128)<T>(&mut self, n: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed 128 bit integer to the underlying writer.

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1442-1446)

#### fn [write\_uint](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_uint)<T>(&mut self, n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_uint)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1478-1482)

#### fn [write\_int](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_int)<T>(&mut self, n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_int)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1493-1497)

#### fn [write\_uint128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_uint128)<T>(&mut self, n: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes an unsigned n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_uint128)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1508-1512)

#### fn [write\_int128](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_int128)<T>(&mut self, n: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a signed n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_int128)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1541)

#### fn [write\_f32](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_f32)<T>(&mut self, n: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a IEEE754 single-precision (4 bytes) floating point number to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_f32)

[Source](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/src/byteorder/io.rs.html#1570)

#### fn [write\_f64](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_f64)<T>(&mut self, n: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/trait.ByteOrder.html "trait byteorder::ByteOrder"),

Writes a IEEE754 double-precision (8 bytes) floating point number to the underlying writer. [Read more](https://docs.rs/byteorder/1.5.0/x86_64-unknown-linux-gnu/byteorder/io/trait.WriteBytesExt.html#method.write_f64)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#1066)

### impl<W> [WriteBytesExt](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html "trait byteorder_lite::io::WriteBytesExt") for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#585)

#### fn [write\_u8](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u8)(&mut self, n: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Writes an unsigned 8 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u8)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#613)

#### fn [write\_i8](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i8)(&mut self, n: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Writes a signed 8 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i8)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#638)

#### fn [write\_u16](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u16)<T>(&mut self, n: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned 16 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u16)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#665)

#### fn [write\_i16](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i16)<T>(&mut self, n: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed 16 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i16)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#692)

#### fn [write\_u24](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u24)<T>(&mut self, n: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned 24 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u24)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#719)

#### fn [write\_i24](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i24)<T>(&mut self, n: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed 24 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i24)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#746)

#### fn [write\_u32](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u32)<T>(&mut self, n: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned 32 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u32)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#773)

#### fn [write\_i32](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i32)<T>(&mut self, n: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed 32 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i32)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#800)

#### fn [write\_u48](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u48)<T>(&mut self, n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned 48 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u48)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#827)

#### fn [write\_i48](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i48)<T>(&mut self, n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed 48 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i48)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#854)

#### fn [write\_u64](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u64)<T>(&mut self, n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned 64 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u64)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#881)

#### fn [write\_i64](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i64)<T>(&mut self, n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed 64 bit integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i64)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#889)

#### fn [write\_u128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_u128)<T>(&mut self, n: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned 128 bit integer to the underlying writer.

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#897)

#### fn [write\_i128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_i128)<T>(&mut self, n: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed 128 bit integer to the underlying writer.

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#929-933)

#### fn [write\_uint](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_uint)<T>(&mut self, n: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_uint)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#965-969)

#### fn [write\_int](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_int)<T>(&mut self, n: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_int)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#980-984)

#### fn [write\_uint128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_uint128)<T>(&mut self, n: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes an unsigned n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_uint128)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#995-999)

#### fn [write\_int128](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_int128)<T>(&mut self, n: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html), nbytes: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a signed n-bytes integer to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_int128)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#1028)

#### fn [write\_f32](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_f32)<T>(&mut self, n: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a IEEE754 single-precision (4 bytes) floating point number to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_f32)

[Source](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/src/byteorder_lite/io.rs.html#1057)

#### fn [write\_f64](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_f64)<T>(&mut self, n: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

where T: [ByteOrder](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/trait.ByteOrder.html "trait byteorder_lite::ByteOrder"),

Writes a IEEE754 double-precision (8 bytes) floating point number to the underlying writer. [Read more](https://docs.rs/byteorder-lite/0.1.0/x86_64-unknown-linux-gnu/byteorder_lite/io/trait.WriteBytesExt.html#method.write_f64)

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<\[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\]> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &\[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#478-483)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &\[[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

### impl<W> [WriteEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html "trait lebe::io::WriteEndian")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\> for W

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_little_endian)(&mut self, value: &[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to little endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#422-427)

#### fn [write\_as\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#tymethod.write_as_big_endian)(&mut self, value: &[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, converting it to big endianness

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#296)

#### fn [write\_as\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.WriteEndian.html#method.write_as_native_endian)(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Write the byte value of the specified reference, not converting it

[Source](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/src/zune_core/bytestream/writer/std_writer.rs.html#7)

### impl<T> [ZByteWriterTrait](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html "trait zune_core::bytestream::traits::ZByteWriterTrait") for T

where T: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/src/zune_core/bytestream/writer/std_writer.rs.html#8)

#### fn [write\_bytes](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.write_bytes)(&mut self, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [ZByteIoError](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/reader/enum.ZByteIoError.html "enum zune_core::bytestream::reader::ZByteIoError")\>

Write some bytes into the sink returning number of bytes written or an error if something bad happened [Read more](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.write_bytes)

[Source](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/src/zune_core/bytestream/writer/std_writer.rs.html#12)

#### fn [write\_all\_bytes](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.write_all_bytes)(&mut self, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ZByteIoError](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/reader/enum.ZByteIoError.html "enum zune_core::bytestream::reader::ZByteIoError")\>

Write all bytes to the buffer or return an error if something occurred [Read more](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.write_all_bytes)

[Source](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/src/zune_core/bytestream/writer/std_writer.rs.html#16)

#### fn [write\_const\_bytes](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.write_const_bytes)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, buf: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ZByteIoError](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/reader/enum.ZByteIoError.html "enum zune_core::bytestream::reader::ZByteIoError")\>

Write a fixed number of bytes and error out if we can’t write the bytes [Read more](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.write_const_bytes)

[Source](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/src/zune_core/bytestream/writer/std_writer.rs.html#19)

#### fn [flush\_bytes](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.flush_bytes)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ZByteIoError](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/reader/enum.ZByteIoError.html "enum zune_core::bytestream::reader::ZByteIoError")\>

Ensure bytes are written to the sink. [Read more](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.flush_bytes)

[Source](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/src/zune_core/bytestream/writer/std_writer.rs.html#22)

#### fn [reserve\_capacity](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.reserve_capacity)(&mut self, \_: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ZByteIoError](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/reader/enum.ZByteIoError.html "enum zune_core::bytestream::reader::ZByteIoError")\>

A hint to tell the implementation how big of a size we expect the image to be An implementation like in memory `Vec` can use this to reserve additional memory to prevent reallocation when encoding [Read more](https://docs.rs/zune-core/0.5.1/x86_64-unknown-linux-gnu/zune_core/bytestream/traits/trait.ZByteWriterTrait.html#tymethod.reserve_capacity)

{"AsyncAsSync<'r, 'ctx, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.AsyncAsSync.html\\" title=\\"struct bevy::tasks::futures\_lite::io::AsyncAsSync\\">AsyncAsSync</a>&lt;'\_, '\_, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"struct.AsyncAsSync.html\\" title=\\"struct bevy::tasks::futures\_lite::io::AsyncAsSync\\">AsyncAsSync</a>&lt;'\_, '\_, T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.AsyncRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncRead\\">AsyncRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a>,</div></div><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"struct.AsyncAsSync.html\\" title=\\"struct bevy::tasks::futures\_lite::io::AsyncAsSync\\">AsyncAsSync</a>&lt;'\_, '\_, T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.AsyncWrite.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncWrite\\">AsyncWrite</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a>,</div></div>","Bytes<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/std/io/struct.Bytes.html\\" title=\\"struct std::io::Bytes\\">Bytes</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/std/io/struct.Bytes.html\\" title=\\"struct std::io::Bytes\\">Bytes</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}