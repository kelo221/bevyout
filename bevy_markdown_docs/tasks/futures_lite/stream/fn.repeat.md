[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function repeat 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#362)

```rust
pub fn repeat<T>(item: T) -> Repeat<T>where
    T: Clone,
```

Creates an infinite stream that yields the same item repeatedly.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::repeat(7);

assert_eq!(s.next().await, Some(7));
assert_eq!(s.next().await, Some(7));
```