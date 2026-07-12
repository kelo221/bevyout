[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function once 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#230)

```rust
pub fn once<T>(t: T) -> Once<T>
```

Creates a stream that yields a single item.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::once(7);

assert_eq!(s.next().await, Some(7));
assert_eq!(s.next().await, None);
```