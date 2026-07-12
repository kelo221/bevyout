[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function iter 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#189)

```rust
pub fn iter<I>(iter: I) -> Iter<<I as IntoIterator>::IntoIter>where
    I: IntoIterator,
```

Creates a stream from an iterator.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::iter(vec![1, 2]);

assert_eq!(s.next().await, Some(1));
assert_eq!(s.next().await, Some(2));
assert_eq!(s.next().await, None);
```