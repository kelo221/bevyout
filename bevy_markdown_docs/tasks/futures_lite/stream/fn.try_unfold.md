[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function try\_unfold 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#543-546)

```rust
pub fn try_unfold<T, E, F, Fut, Item>(init: T, f: F) -> TryUnfold<T, F, Fut>where
    F: FnMut(T) -> Fut,
    Fut: Future<Output = Result<Option<(Item, T)>, E>>,
```

Creates a stream from a seed value and a fallible async closure operating on it.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let s = stream::try_unfold(0, |mut n| async move {
    if n < 2 {
        let m = n + 1;
        Ok(Some((n, m)))
    } else {
        std::io::Result::Ok(None)
    }
});

let v: Vec<i32> = s.try_collect().await?;
assert_eq!(v, [0, 1]);
```