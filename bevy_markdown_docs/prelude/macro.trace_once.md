[bevy](../index.html)::[prelude](index.html)

# Macro trace\_once 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/once.rs.html#5)

```rust
macro_rules! trace_once {
    ($($arg:tt)+) => { ... };
}
```

Call [`trace!`](macro.trace.html "macro bevy::prelude::trace") once per call site.

Useful for logging within systems which are called every frame.