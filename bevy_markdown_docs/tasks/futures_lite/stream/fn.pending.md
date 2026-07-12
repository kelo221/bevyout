[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function pending 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#272)

```rust
pub fn pending<T>() -> Pending<T>
```

Creates a stream that is always pending.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

let mut s = stream::pending::<i32>();
s.next().await;
unreachable!();
```