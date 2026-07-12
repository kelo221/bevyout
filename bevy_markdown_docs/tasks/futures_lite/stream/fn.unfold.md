[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function unfold 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#454-457)

```rust
pub fn unfold<T, F, Fut, Item>(seed: T, f: F) -> Unfold<T, F, Fut>where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = Option<(Item, T)>>,
```

Creates a stream from a seed value and an async closure operating on it.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::unfold(0, |mut n| async move {
    if n < 2 {
        let m = n + 1;
        Some((n, m))
    } else {
        None
    }
});

let v: Vec<i32> = s.collect().await;
assert_eq!(v, [0, 1]);
```