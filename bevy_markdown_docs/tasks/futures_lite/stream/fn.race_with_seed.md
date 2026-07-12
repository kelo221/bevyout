[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function race\_with\_seed 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2672-2675)

```rust
pub fn race_with_seed<T, S1, S2>(
    stream1: S1,
    stream2: S2,
    seed: u64,
) -> Race<S1, S2>where
    S1: Stream<Item = T>,
    S2: Stream<Item = T>,
```

Available on **crate feature `race`** only.

Races two streams, but with a user-provided seed for randomness.

## Examples

```rust
use futures_lite::stream::{self, once, pending, StreamExt};

// A fixed seed is used for reproducibility.
const SEED: u64 = 123;

assert_eq!(stream::race_with_seed(once(1), pending(), SEED).next().await, Some(1));
assert_eq!(stream::race_with_seed(pending(), once(2), SEED).next().await, Some(2));

// One of the two stream is randomly chosen as the winner.
let res = stream::race_with_seed(once(1), once(2), SEED).next().await;
```