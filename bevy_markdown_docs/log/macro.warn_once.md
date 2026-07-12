[bevy](../index.html)::[log](index.html)

# Macro warn\_once 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/once.rs.html#35)

```rust
macro_rules! warn_once {
    ($($arg:tt)+) => { ... };
}
```

Call [`warn!`](../prelude/macro.warn.html "macro bevy::prelude::warn") once per call site.

Useful for logging within systems which are called every frame.