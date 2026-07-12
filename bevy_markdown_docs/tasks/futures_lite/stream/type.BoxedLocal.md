[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Type Alias BoxedLocal 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#2096)

```rust
pub type BoxedLocal<T> = Pin<Box<dyn Stream<Item = T>>>;
```

Available on **crate feature `alloc`** only.

Type alias for `Pin<Box<dyn Stream<Item = T> + 'static>>`.

## Examples

```rust
use futures_lite::stream::{self, StreamExt};

// These two lines are equivalent:
let s1: stream::BoxedLocal<i32> = stream::once(7).boxed_local();
let s2: stream::BoxedLocal<i32> = Box::pin(stream::once(7));
```

## Aliased Type

```rust
pub struct BoxedLocal<T> { /* private fields */ }
```