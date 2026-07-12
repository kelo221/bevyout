[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function once\_future 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#630)

```rust
pub fn once_future<F>(future: F) -> OnceFuture<F>where
    F: Future,
```

Creates a stream that invokes the given future as its first item, and then produces no more items.

## Example

```rust
use futures_lite::{stream, prelude::*};

let mut stream = Box::pin(stream::once_future(async { 1 }));
assert_eq!(stream.next().await, Some(1));
assert_eq!(stream.next().await, None);
```