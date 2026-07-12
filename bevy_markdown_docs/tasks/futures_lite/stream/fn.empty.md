[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function empty 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#147)

```rust
pub fn empty<T>() -> Empty<T>
```

Creates an empty stream.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::empty::<i32>();
assert_eq!(s.next().await, None);
```