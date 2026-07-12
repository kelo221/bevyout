[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function repeat\_with 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#401-403)

```rust
pub fn repeat_with<T, F>(repeater: F) -> RepeatWith<F>where
    F: FnMut() -> T,
```

Creates an infinite stream from a closure that generates items.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::repeat_with(|| 7);

assert_eq!(s.next().await, Some(7));
assert_eq!(s.next().await, Some(7));
```