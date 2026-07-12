[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Type Alias Boxed 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#669)

```rust
pub type Boxed<T> = Pin<Box<dyn Future<Output = T> + Send>>;
```

Available on **crate feature `alloc`** only.

Type alias for `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.

## Examples

```rust
use futures_lite::future::{self, FutureExt};

// These two lines are equivalent:
let f1: future::Boxed<i32> = async { 1 + 2 }.boxed();
let f2: future::Boxed<i32> = Box::pin(async { 1 + 2 });
```

## Aliased Type

```rust
pub struct Boxed<T> { /* private fields */ }
```