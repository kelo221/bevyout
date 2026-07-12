[bevy](../index.html)::[log](index.html)

# Macro info\_once 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/once.rs.html#25)

```rust
macro_rules! info_once {
    ($($arg:tt)+) => { ... };
}
```

Call [`info!`](../prelude/macro.info.html "macro bevy::prelude::info") once per call site.

Useful for logging within systems which are called every frame.