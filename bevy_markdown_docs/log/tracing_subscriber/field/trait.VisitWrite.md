[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[field](index.html)

# Trait VisitWrite 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#116)

```rust
pub trait VisitWrite: VisitOutput<Result<(), Error>> {
    // Required method
    fn writer(&mut self) -> &mut dyn Write;
}
```

Available on **crate feature `std`** only.

Extension trait implemented by visitors to indicate that they write to an `io::Write` instance, and allow access to that writer.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#118)

#### fn [writer](#tymethod.writer)(&mut self) -> &mut dyn [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write")

Returns the writer that this visitor writes to.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/debug.rs.html#92-94)

### impl<V> [VisitWrite](trait.VisitWrite.html "trait bevy::log::tracing_subscriber::field::VisitWrite") for [Alt](debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<V>

where V: [VisitWrite](trait.VisitWrite.html "trait bevy::log::tracing_subscriber::field::VisitWrite"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/display.rs.html#98-100)

### impl<V> [VisitWrite](trait.VisitWrite.html "trait bevy::log::tracing_subscriber::field::VisitWrite") for [Messages](display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<V>

where V: [VisitWrite](trait.VisitWrite.html "trait bevy::log::tracing_subscriber::field::VisitWrite"),