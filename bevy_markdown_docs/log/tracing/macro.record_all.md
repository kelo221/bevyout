[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Macro record\_all 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/macros.rs.html#152)

```rust
macro_rules! record_all {
    ($span:expr, $($fields:tt)*) => { ... };
}
```

Records multiple values on a span in a single call. As with recording individual values, all fields must be declared when the span is created.

This macro supports two optional sigils:

*   `%` uses the Display implementation.
*   `?` uses the Debug implementation.

For more details, see the [top-level documentation](tracing/#recording-fields).

## Examples

```rust
let span = info_span!("my span", field1 = field::Empty, field2 = field::Empty, field3 = field::Empty).entered();
record_all!(span, field1 = ?"1", field2 = %"2", field3 = 3);
```