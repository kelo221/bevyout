[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Macro error 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#2452)

```rust
macro_rules! error {
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

Constructs an event at the error level.

This functions similarly to the [`event!`](../macro.event.html "macro bevy::log::event") macro. See [the top-level documentation](index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
use tracing::error;

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
```