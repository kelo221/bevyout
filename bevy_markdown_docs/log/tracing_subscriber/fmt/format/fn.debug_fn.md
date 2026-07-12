[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[format](index.html)

# Function debug\_fn 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#287-289)

```rust
pub fn debug_fn<F>(f: F) -> FieldFn<F>where
    F: Fn(&mut Writer<'_>, &Field, &dyn Debug) -> Result<(), Error> + Clone,
```

Available on **crate features `fmt` and `std`** only.

Returns a [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") implementation that formats fields using the provided function or closure.