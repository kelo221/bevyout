[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function race 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2641-2644)

```rust
pub fn race<T, S1, S2>(stream1: S1, stream2: S2) -> Race<S1, S2>where
    S1: Stream<Item = T>,
    S2: Stream<Item = T>,
```

Available on **crate features `race` and `std`** only.

Merges two streams, with no preference for either stream when both are ready.

## Examples

```rust
use futures_lite::stream::{self, once, pending, StreamExt};

assert_eq!(stream::race(once(1), pending()).next().await, Some(1));
assert_eq!(stream::race(pending(), once(2)).next().await, Some(2));

// One of the two stream is randomly chosen as the winner.
let res = stream::race(once(1), once(2)).next().await;
```