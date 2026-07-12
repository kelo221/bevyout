[bevy](../../index.html)::[tasks](../index.html)::[prelude](index.html)

# Function block\_on 

[Source](https://docs.rs/async-io/2.6.0/x86_64-unknown-linux-gnu/src/async_io/driver.rs.html#114)

```rust
pub fn block_on<T>(future: impl Future<Output = T>) -> T
```

Blocks the current thread on a future, processing I/O events when idle.

## Examples

```rust
use async_io::Timer;
use std::time::Duration;

async_io::block_on(async {
    // This timer will likely be processed by the current
    // thread rather than the fallback "async-io" thread.
    Timer::after(Duration::from_millis(1)).await;
});
```