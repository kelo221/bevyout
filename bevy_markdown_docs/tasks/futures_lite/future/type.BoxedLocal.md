[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Type Alias BoxedLocal 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#683)

```rust
pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T>>>;
```

Available on **crate feature `alloc`** only.

Type alias for `Pin<Box<dyn Future<Output = T> + 'static>>`.

## Examples

```rust
use futures_lite::future::{self, FutureExt};

// These two lines are equivalent:
let f1: future::BoxedLocal<i32> = async { 1 + 2 }.boxed_local();
let f2: future::BoxedLocal<i32> = Box::pin(async { 1 + 2 });
```

## Aliased Type

```rust
pub struct BoxedLocal<T> { /* private fields */ }
```