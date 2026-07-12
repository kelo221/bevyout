[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function or 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2588-2591)

```rust
pub fn or<T, S1, S2>(stream1: S1, stream2: S2) -> Or<S1, S2>where
    S1: Stream<Item = T>,
    S2: Stream<Item = T>,
```

Merges two streams, preferring items from `stream1` whenever both streams are ready.

## Examples

```rust
use futures_lite::stream::{self, once, pending, StreamExt};

assert_eq!(stream::or(once(1), pending()).next().await, Some(1));
assert_eq!(stream::or(pending(), once(2)).next().await, Some(2));

// The first stream wins.
assert_eq!(stream::or(once(1), once(2)).next().await, Some(1));
```