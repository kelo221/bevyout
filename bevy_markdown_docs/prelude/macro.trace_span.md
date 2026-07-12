[bevy](../index.html)::[prelude](index.html)

# Macro trace\_span 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#197)

```rust
macro_rules! trace_span {
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

Constructs a span at the trace level.

[Fields](../log/tracing/index.html#recording-fields "mod bevy::log::tracing") and [attributes](../log/tracing/index.html#configuring-attributes "mod bevy::log::tracing") are set using the same syntax as the [`span!`](../log/tracing/macro.span.html "macro bevy::log::tracing::span") macro.

See [the top-level documentation](../log/tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
```

```rust
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```