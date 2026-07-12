[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function block\_on 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#53)

```rust
pub fn block_on<T>(future: impl Future<Output = T>) -> T
```

Available on **crate feature `std`** only.

Blocks the current thread on a future.

## Examples

```rust
use futures_lite::future;

let val = future::block_on(async {
    1 + 2
});

assert_eq!(val, 3);
```