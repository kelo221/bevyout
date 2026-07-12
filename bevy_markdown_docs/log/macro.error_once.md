[bevy](../index.html)::[log](index.html)

# Macro error\_once 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/once.rs.html#45)

```rust
macro_rules! error_once {
    ($($arg:tt)+) => { ... };
}
```

Call [`error!`](../prelude/macro.error.html "macro bevy::prelude::error") once per call site.

Useful for logging within systems which are called every frame.