[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function poll\_fn 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#315-317)

```rust
pub fn poll_fn<T, F>(f: F) -> PollFn<F>where
    F: FnMut(&mut Context<'_>) -> Poll<Option<T>>,
```

Creates a stream from a function returning [`Poll`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll").

## Examples

```rust
use futures_lite::stream::{self, StreamExt};
use std::task::{Context, Poll};

fn f(_: &mut Context<'_>) -> Poll<Option<i32>> {
    Poll::Ready(Some(7))
}

assert_eq!(stream::poll_fn(f).next().await, Some(7));
```