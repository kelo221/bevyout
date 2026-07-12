[bevy](../../index.html)::[log](../index.html)::[prelude](index.html)

# Macro trace 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#1333)

```rust
macro_rules! trace {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
}
```

Constructs an event at the trace level.

This functions similarly to the [`event!`](../macro.event.html "macro bevy::log::event") macro. See [the top-level documentation](../tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
use tracing::trace;
let pos = Position { x: 3.234, y: -1.223 };
let origin_dist = pos.dist(Position::ORIGIN);

trace!(position = ?pos, ?origin_dist);
trace!(
    target: "app_events",
    position = ?pos,
    "x is {} and y is {}",
    if pos.x >= 0.0 { "positive" } else { "negative" },
    if pos.y >= 0.0 { "positive" } else { "negative" }
);
trace!(name: "completed", position = ?pos);
```