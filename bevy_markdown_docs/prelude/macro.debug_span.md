[bevy](../index.html)::[prelude](index.html)

# Macro debug\_span 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#278)

```rust
macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```

Constructs a span at the debug level.

[Fields](../log/tracing/index.html#recording-fields "mod bevy::log::tracing") and [attributes](../log/tracing/index.html#configuring-attributes "mod bevy::log::tracing") are set using the same syntax as the [`span!`](../log/tracing/macro.span.html "macro bevy::log::tracing::span") macro.

See [the top-level documentation](../log/tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
```

```rust
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```