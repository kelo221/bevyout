[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[field](index.html)

# Trait MakeVisitor 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#26)

```rust
pub trait MakeVisitor<T> {
    type Visitor: Visit;

    // Required method
    fn make_visitor(&self, target: T) -> Self::Visitor;
}
```

Creates new [visitors](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").

A type implementing `MakeVisitor` represents a composable factory for types implementing the [`Visit` trait](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit"). The `MakeVisitor` trait defines a single function, `make_visitor`, which takes in a `T`\-typed `target` and returns a type implementing `Visit` configured for that target. A target may be a string, output stream, or data structure that the visitor will record data to, configuration variables that determine the visitor’s behavior, or `()` when no input is required to produce a visitor.

## Required Associated Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#28)

#### type [Visitor](#associatedtype.Visitor): [Visit](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit")

The visitor type produced by this `MakeVisitor`.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#31)

#### fn [make\_visitor](#tymethod.make_visitor)(&self, target: T) -> Self::[Visitor](trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor")

Make a new visitor for the provided `target`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1582-1584)

### impl<'a, F> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<[Writer](../fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>> for [FieldFn](../fmt/format/struct.FieldFn.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFn")<F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [Writer](../fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>, &[Field](../../tracing/field/struct.Field.html "struct bevy::log::tracing::field::Field"), &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1586)

#### type [Visitor](#associatedtype.Visitor) = [FieldFnVisitor](../fmt/format/struct.FieldFnVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFnVisitor")<'a, F>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1228)

### impl<'a> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<[Writer](../fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>> for [DefaultFields](../fmt/format/struct.DefaultFields.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultFields")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1229)

#### type [Visitor](#associatedtype.Visitor) = [DefaultVisitor](../fmt/format/struct.DefaultVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultVisitor")<'a>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#387)

### impl<'a> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<[Writer](../fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>> for [PrettyFields](../fmt/format/struct.PrettyFields.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyFields")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#388)

#### type [Visitor](#associatedtype.Visitor) = [PrettyVisitor](../fmt/format/struct.PrettyVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyVisitor")<'a>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/delimited.rs.html#27-31)

### impl<D, V, T> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T> for [Delimited](delimited/struct.Delimited.html "struct bevy::log::tracing_subscriber::field::delimited::Delimited")<D, V>

where D: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), V: [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>, <V as [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>>::[Visitor](trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor"): [VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/delimited.rs.html#33)

#### type [Visitor](#associatedtype.Visitor) = [VisitDelimited](delimited/struct.VisitDelimited.html "struct bevy::log::tracing_subscriber::field::delimited::VisitDelimited")<D, <V as [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>>::[Visitor](trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#193-196)

### impl<T, V, F> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T> for F

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> V, V: [Visit](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#198)

#### type [Visitor](#associatedtype.Visitor) = V

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/debug.rs.html#25-27)

### impl<T, V> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T> for [Alt](debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<V>

where V: [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/debug.rs.html#29)

#### type [Visitor](#associatedtype.Visitor) = [Alt](debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<<V as [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>>::[Visitor](trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/display.rs.html#27-29)

### impl<T, V> [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T> for [Messages](display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<V>

where V: [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/display.rs.html#31)

#### type [Visitor](#associatedtype.Visitor) = [Messages](display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<<V as [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>>::[Visitor](trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor")\>