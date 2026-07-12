[bevy](../index.html)::[prelude](index.html)

# Macro warn 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#2176)

```rust
macro_rules! warn {
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

Constructs an event at the warn level.

This functions similarly to the [`event!`](../log/macro.event.html "macro bevy::log::event") macro. See [the top-level documentation](../log/tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
use tracing::warn;

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
```