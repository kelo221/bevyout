[bevy](../../index.html)::[tasks](../index.html)::[futures](index.html)

# Function now\_or\_never 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/futures.rs.html#12)

```rust
pub fn now_or_never<F>(future: F) -> Option<<F as Future>::Output>where
    F: Future,
```

Consumes a future, polls it once, and immediately returns the output or returns `None` if it wasn’t ready yet.

This will cancel the future if it’s not ready.