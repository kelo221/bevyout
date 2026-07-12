[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Macro span 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#20)

```rust
macro_rules! span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr) => { ... };
    ($lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    ($lvl:expr, $name:expr) => { ... };
}
```

Constructs a new span.

See [the top-level documentation](index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

Creating a new span:

```rust
let span = span!(Level::TRACE, "my span");
let _enter = span.enter();
// do work inside the span...
```