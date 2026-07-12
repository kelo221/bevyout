[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Macro enabled 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#1214)

```rust
macro_rules! enabled {
    (kind: $kind:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (kind: $kind:expr, target: $target:expr, $lvl:expr ) => { ... };
    (target: $target:expr, $lvl:expr ) => { ... };
    (kind: $kind:expr, target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
    (kind: $kind:expr, $lvl:expr, $($field:tt)*) => { ... };
    (kind: $kind:expr, $lvl:expr) => { ... };
    ($lvl:expr) => { ... };
    ($lvl:expr, $($field:tt)*) => { ... };
}
```

Checks whether a span or event is [enabled](trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled") based on the provided [metadata](struct.Metadata.html "struct bevy::log::tracing::Metadata").

This macro is a specialized tool: it is intended to be used prior to an expensive computation required _just_ for that event, but _cannot_ be done as part of an argument to that event, such as when multiple events are emitted (e.g., iterating over a collection and emitting an event for each item).

## Usage

[Subscribers](trait.Subscriber.html "trait bevy::log::tracing::Subscriber") can make filtering decisions based all the data included in a span or event’s [`Metadata`](struct.Metadata.html "struct bevy::log::tracing::Metadata"). This means that it is possible for `enabled!` to return a _false positive_ (indicating that something would be enabled when it actually would not be) or a _false negative_ (indicating that something would be disabled when it would actually be enabled).

This occurs when a subscriber is using a _more specific_ filter than the metadata provided to the `enabled!` macro. Some situations that can result in false positives or false negatives include:

*   If a subscriber is using a filter which may enable a span or event based on field names, but `enabled!` is invoked without listing field names, `enabled!` may return a false negative if a specific field name would cause the subscriber to enable something that would otherwise be disabled.
*   If a subscriber is using a filter which enables or disables specific events by file path and line number, a particular event may be enabled/disabled even if an `enabled!` invocation with the same level, target, and fields indicated otherwise.
*   The subscriber can choose to enable _only_ spans or _only_ events, which `enabled` will not reflect.

`enabled!()` requires a [level](../struct.Level.html "struct bevy::log::Level") argument, an optional `target:` argument, and an optional set of field names. If the fields are not provided, they are considered to be unknown. `enabled!` attempts to match the syntax of `event!()` as closely as possible, which can be seen in the examples below.

## Examples

If the current subscriber is interested in recording `DEBUG`\-level spans and events in the current file and module path, this will evaluate to true:

```rust
use tracing::{enabled, Level};

if enabled!(Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events in the current file and module path, with the target “my\_crate”, and at the level `DEBUG`, this will evaluate to true:

```rust
if enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events in the current file and module path, with the target “my\_crate”, at the level `DEBUG`, and with a field named “hello”, this will evaluate to true:

```rust
if enabled!(target: "my_crate", Level::DEBUG, hello) {
    // some expensive work...
}
```

## Alternatives

`enabled!` queries subscribers with [`Metadata`](struct.Metadata.html "struct bevy::log::tracing::Metadata") where [`is_event`](struct.Metadata.html#method.is_event "method bevy::log::tracing::Metadata::is_event") and [`is_span`](struct.Metadata.html#method.is_span "method bevy::log::tracing::Metadata::is_span") both return `false`. Alternatively, use [`event_enabled!`](macro.event_enabled.html "macro bevy::log::tracing::event_enabled") or [`span_enabled!`](macro.span_enabled.html "macro bevy::log::tracing::span_enabled") to ensure one of these returns true.