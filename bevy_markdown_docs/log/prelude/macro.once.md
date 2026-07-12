[bevy](../../index.html)::[log](../index.html)::[prelude](index.html)

# Macro once 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/once.rs.html#28)

```rust
macro_rules! once {
    ($expression:expr) => { ... };
}
```

Call some expression only once per call site.