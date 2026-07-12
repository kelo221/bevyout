[bevy](../index.html)::[prelude](index.html)

# Macro info 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#1896)

```rust
macro_rules! info {
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

Constructs an event at the info level.

This functions similarly to the [`event!`](../log/macro.event.html "macro bevy::log::event") macro. See [the top-level documentation](../log/tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
use tracing::info;
use tracing::field;

let addr = Ipv4Addr::new(127, 0, 0, 1);
let conn = Connection { port: 40, speed: 3.20 };

info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
info!(name: "completed", "completed connection to {:?}", addr);
```