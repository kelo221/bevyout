[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Macro span\_enabled 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#1117)

```rust
macro_rules! span_enabled {
    ($($rest:tt)*) => { ... };
}
```

Tests whether a span with the specified level and target would be enabled.

This is similar to [`enabled!`](macro.enabled.html "macro bevy::log::tracing::enabled"), but queries the current subscriber specifically for an event, whereas [`enabled!`](macro.enabled.html "macro bevy::log::tracing::enabled") queries for an event _or_ span.

See the documentation for \[`enabled!]` for more details on using this macro. See also [`span_enabled!`](macro.span_enabled.html "macro bevy::log::tracing::span_enabled").

## Examples

```rust
if span_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if span_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if span_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```