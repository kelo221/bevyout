[bevy](../../index.html)::[tasks](../index.html)

# Crate futures\_lite 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/lib.rs.html#1-147)

Futures, streams, and async I/O combinators.

This crate is a subset of [futures](https://docs.rs/futures) that compiles an order of magnitude faster, fixes minor warts in its API, fills in some obvious gaps, and removes almost all unsafe code from it.

In short, this crate aims to be more enjoyable than [futures](https://docs.rs/futures) but still fully compatible with it.

The API for this crate is intentionally constrained. Please consult the [features list](https://github.com/smol-rs/futures-lite/blob/master/FEATURES.md) for APIs that are occluded from this crate.

## Examples

```rust
use futures_lite::future;

fn main() {
    future::block_on(async {
        println!("Hello world!");
    })
}
```

## Modules

[future](future/index.html "mod bevy::tasks::futures_lite::future")

Combinators for the [`Future`](trait.Future.html "trait bevy::tasks::futures_lite::Future") trait.

[io](io/index.html "mod bevy::tasks::futures_lite::io")`std`

Tools and combinators for I/O.

[prelude](prelude/index.html "mod bevy::tasks::futures_lite::prelude")

Traits [`Future`](trait.Future.html "trait bevy::tasks::futures_lite::Future"), [`Stream`](trait.Stream.html "trait bevy::tasks::futures_lite::Stream"), [`AsyncRead`](trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"), [`AsyncWrite`](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite"), [`AsyncBufRead`](trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead"), [`AsyncSeek`](trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek"), and their extensions.

[stream](stream/index.html "mod bevy::tasks::futures_lite::stream")

Combinators for the [`Stream`](trait.Stream.html "trait bevy::tasks::futures_lite::Stream") trait.

## Macros

[pin](macro.pin.html "macro bevy::tasks::futures_lite::pin")

Pins a variable of type `T` on the stack and rebinds it as `Pin<&mut T>`.

[ready](macro.ready.html "macro bevy::tasks::futures_lite::ready")

Unwraps `Poll<T>` or returns [`Pending`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending").

## Traits

[AsyncBufRead](trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead")

Read bytes asynchronously.

[AsyncBufReadExt](trait.AsyncBufReadExt.html "trait bevy::tasks::futures_lite::AsyncBufReadExt")

Extension trait for [`AsyncBufRead`](trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead").

[AsyncRead](trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead")

Read bytes asynchronously.

[AsyncReadExt](trait.AsyncReadExt.html "trait bevy::tasks::futures_lite::AsyncReadExt")

Extension trait for [`AsyncRead`](trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead").

[AsyncSeek](trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek")

Seek bytes asynchronously.

[AsyncSeekExt](trait.AsyncSeekExt.html "trait bevy::tasks::futures_lite::AsyncSeekExt")

Extension trait for [`AsyncSeek`](trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek").

[AsyncWrite](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite")

Write bytes asynchronously.

[AsyncWriteExt](trait.AsyncWriteExt.html "trait bevy::tasks::futures_lite::AsyncWriteExt")

Extension trait for [`AsyncWrite`](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite").

[Future](trait.Future.html "trait bevy::tasks::futures_lite::Future")

A future represents an asynchronous computation, commonly obtained by use of [`async`](../../std/keyword.async.html).

[FutureExt](trait.FutureExt.html "trait bevy::tasks::futures_lite::FutureExt")

Extension trait for [`Future`](trait.Future.html "trait bevy::tasks::futures_lite::Future").

[Stream](trait.Stream.html "trait bevy::tasks::futures_lite::Stream")

A stream of values produced asynchronously.

[StreamExt](trait.StreamExt.html "trait bevy::tasks::futures_lite::StreamExt")

Extension trait for [`Stream`](trait.Stream.html "trait bevy::tasks::futures_lite::Stream").