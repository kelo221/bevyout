[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[field](index.html)

# Trait VisitOutput 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#37)

```rust
pub trait VisitOutput<Out>: Visit {
    // Required method
    fn finish(self) -> Out;

    // Provided method
    fn visit<R>(self, fields: &R) -> Out
       where R: RecordFields,
             Self: Sized { ... }
}
```

A [visitor](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit") that produces output once it has visited a set of fields.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#41)

#### fn [finish](#tymethod.finish)(self) -> Out

Completes the visitor, returning any output.

This is called once a full set of fields has been visited.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#45-48)

#### fn [visit](#method.visit)<R>(self, fields: [&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> Out

where R: [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Visit a set of fields, and return the output of finishing the visitor once the fields have been visited.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1343)

### impl [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>> for [DefaultVisitor](../fmt/format/struct.DefaultVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultVisitor")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#511)

### impl [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>> for [PrettyVisitor](../fmt/format/struct.PrettyVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyVisitor")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1608-1610)

### impl<'a, F> [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>> for [FieldFnVisitor](../fmt/format/struct.FieldFnVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFnVisitor")<'a, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [Writer](../fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>, &[Field](../../tracing/field/struct.Field.html "struct bevy::log::tracing::field::Field"), &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/delimited.rs.html#114-117)

### impl<D, V> [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>> for [VisitDelimited](delimited/struct.VisitDelimited.html "struct bevy::log::tracing_subscriber::field::delimited::VisitDelimited")<D, V>

where V: [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"), D: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/debug.rs.html#77-79)

### impl<V, O> [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<O> for [Alt](debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<V>

where V: [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<O>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/display.rs.html#83-85)

### impl<V, O> [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<O> for [Messages](display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<V>

where V: [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<O>,