[bevy](../index.html)::[log](index.html)

# Macro event 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#615)

```rust
macro_rules! event {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, ?$($k:ident).+) => { ... };
    ($lvl:expr, %$($k:ident).+) => { ... };
    ($lvl:expr, $($k:ident).+) => { ... };
    ( $lvl:expr, $($arg:tt)+ ) => { ... };
}
```

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields. Optionally, a format string and arguments may follow the fields; this will be used to construct an implicit field named “message”.

See [the top-level documentation](tracing/index.html#using-the-macros "mod bevy::log::tracing") for details on the syntax accepted by this macro.

## Examples

```rust
use tracing::{event, Level};

let data = (42, "forty-two");
let private_data = "private";
let error = "a bad error";

event!(Level::ERROR, %error, "Received error");
event!(
    target: "app_events",
    Level::WARN,
    private_data,
    ?data,
    "App warning: {}",
    error
);
event!(name: "answer", Level::INFO, the_answer = data.0);
event!(Level::INFO, the_answer = data.0);
```