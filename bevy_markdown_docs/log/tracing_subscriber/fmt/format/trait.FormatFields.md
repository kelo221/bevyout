[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[format](index.html)

# Trait FormatFields 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#235)

```rust
pub trait FormatFields<'writer> {
    // Required method
    fn format_fields<R>(
        &self,
        writer: Writer<'writer>,
        fields: R,
    ) -> Result<(), Error>
       where R: RecordFields;

    // Provided method
    fn add_fields(
        &self,
        current: &'writer mut FormattedFields<Self>,
        fields: &Record<'_>,
    ) -> Result<(), Error> { ... }
}
```

Available on **crate features `fmt` and `std`** only.

A type that can format a [set of fields](../../field/trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") to a [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

[`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") is primarily used in the context of [`FmtSubscriber`](../../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber"). Each time a span or event with fields is recorded, the subscriber will format those fields with its associated [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") implementation.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#237)

#### fn [format\_fields](#tymethod.format_fields)<R>( &self, writer: [Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'writer>, fields: R, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

where R: [RecordFields](../../field/trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields"),

Format the provided `fields` to the provided [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer"), returning a result.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#244-248)

#### fn [add\_fields](#method.add_fields)( &self, current: &'writer mut [FormattedFields](../struct.FormattedFields.html "struct bevy::log::tracing_subscriber::fmt::FormattedFields")<Self>, fields: &[Record](../../../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Record additional field(s) on an existing span.

By default, this appends a space to the current set of fields if it is non-empty, and then calls `self.format_fields`. If different behavior is required, the default implementation of this method can be overridden.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1183-1186)

### impl<'writer, M> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> for M

where M: [MakeOutput](../../field/trait.MakeOutput.html "trait bevy::log::tracing_subscriber::field::MakeOutput")<[Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'writer>, [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>>, <M as [MakeVisitor](../../field/trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<[Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'writer>>>::[Visitor](../../field/trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor"): [VisitFmt](../../field/trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt") + [VisitOutput](../../field/trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/fmt_layer.rs.html#1098-1101)

### impl<'writer, S, N> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> for [FmtContext](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext")<'\_, S, N>

where S: [Subscriber](../../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'lookup> [LookupSpan](../../registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'lookup>, N: [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> + 'static,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#337)

### impl<'writer> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> for [Pretty](struct.Pretty.html "struct bevy::log::tracing_subscriber::fmt::format::Pretty")