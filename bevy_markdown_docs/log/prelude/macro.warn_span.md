[bevy](../../index.html)::[log](../index.html)::[prelude](index.html)

# Macro warn\_span 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#440)

```rust
macro_rules! warn_span {
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

Constructs a span at the warn level.

[Fields](../tracing/index.html#recording-fields "mod bevy::log::tracing") and [attributes](../tracing/index.html#configuring-attributes "mod bevy::log::tracing") are set using the same syntax as the [`span!`](../tracing/macro.span.html "macro bevy::log::tracing::span") macro.

See [the top-level documentation](../tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
```

```rust
use tracing::warn_span;
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
```