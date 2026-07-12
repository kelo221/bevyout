[bevy](../index.html)::[log](index.html)

# Macro debug\_once 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/once.rs.html#15)

```rust
macro_rules! debug_once {
    ($($arg:tt)+) => { ... };
}
```

Call [`debug!`](../prelude/macro.debug.html "macro bevy::prelude::debug") once per call site.

Useful for logging within systems which are called every frame.