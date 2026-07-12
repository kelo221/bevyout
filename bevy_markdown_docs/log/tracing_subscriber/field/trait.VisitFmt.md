[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[field](index.html)

# Trait VisitFmt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#124)

```rust
pub trait VisitFmt: VisitOutput<Result<(), Error>> {
    // Required method
    fn writer(&mut self) -> &mut dyn Write;
}
```

Extension trait implemented by visitors to indicate that they write to a `fmt::Write` instance, and allow access to that writer.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#126)

#### fn [writer](#tymethod.writer)(&mut self) -> &mut dyn [Write](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write")

Returns the formatter that this visitor writes to.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1349)

### impl [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") for [DefaultVisitor](../fmt/format/struct.DefaultVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultVisitor")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#518)

### impl [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") for [PrettyVisitor](../fmt/format/struct.PrettyVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyVisitor")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1617-1619)

### impl<'a, F> [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") for [FieldFnVisitor](../fmt/format/struct.FieldFnVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFnVisitor")<'a, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [Writer](../fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>, &[Field](../../tracing/field/struct.Field.html "struct bevy::log::tracing::field::Field"), &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/delimited.rs.html#125-128)

### impl<D, V> [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") for [VisitDelimited](delimited/struct.VisitDelimited.html "struct bevy::log::tracing_subscriber::field::delimited::VisitDelimited")<D, V>

where V: [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"), D: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/debug.rs.html#103-105)

### impl<V> [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") for [Alt](debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<V>

where V: [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/display.rs.html#109-111)

### impl<V> [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") for [Messages](display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<V>

where V: [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"),