[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function stop\_after\_future 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#696-699)

```rust
pub fn stop_after_future<S, F>(stream: S, future: F) -> StopAfterFuture<S, F>where
    S: Stream,
    F: Future,
```

Take elements from this stream until the provided future resolves.

This function will take elements from the stream until the provided stopping future `fut` resolves. Once the `fut` future becomes ready, this stream combinator will always return that the stream is done.

The stopping future may return any type. Once the stream is stopped the result of the stopping future may be accessed with `StopAfterFuture::take_result()`. The stream may also be resumed with `StopAfterFuture::take_future()`. See the documentation of [`StopAfterFuture`](struct.StopAfterFuture.html "struct bevy::tasks::futures_lite::stream::StopAfterFuture") for more information.

```rust
use futures_lite::stream::{self, StreamExt, stop_after_future};
use futures_lite::future;
use std::task::Poll;

let stream = stream::iter(1..=10);

let mut i = 0;
let stop_fut = future::poll_fn(|_cx| {
    i += 1;
    if i <= 5 {
        Poll::Pending
    } else {
        Poll::Ready(())
    }
});

let stream = stop_after_future(stream, stop_fut);

assert_eq!(vec![1, 2, 3, 4, 5], stream.collect::<Vec<_>>().await);
```