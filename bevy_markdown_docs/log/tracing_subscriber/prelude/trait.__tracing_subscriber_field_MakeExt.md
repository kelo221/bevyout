[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[prelude](index.html)

# Trait \_\_tracing\_subscriber\_field\_MakeExt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#130)

```rust
pub trait __tracing_subscriber_field_MakeExt<T>:
    Sized
    + MakeVisitor<T>
    + Sealed<MakeExtMarker<T>> {
    // Provided methods
    fn debug_alt(self) -> Alt<Self> { ... }
    fn display_messages(self) -> Messages<Self> { ... }
    fn delimited<D>(self, delimiter: D) -> Delimited<D, Self>
       where D: AsRef<str> + Clone,
             Self::Visitor: VisitFmt { ... }
}
```

Extension trait providing `MakeVisitor` combinators.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#137)

#### fn [debug\_alt](#method.debug_alt)(self) -> [Alt](../field/debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<Self>

Wraps `self` so that any `fmt::Debug` fields are recorded using the alternate formatter (`{:#?}`).

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#143)

#### fn [display\_messages](#method.display_messages)(self) -> [Messages](../field/display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<Self>

Wraps `self` so that any string fields named “message” are recorded using `fmt::Display`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#149-152)

#### fn [delimited](#method.delimited)<D>(self, delimiter: D) -> [Delimited](../field/delimited/struct.Delimited.html "struct bevy::log::tracing_subscriber::field::delimited::Delimited")<D, Self>

where D: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), Self::[Visitor](../field/trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor"): [VisitFmt](../field/trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"),

Wraps `self` so that when fields are formatted to a writer, they are separated by the provided `delimiter`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#220-223)

### impl<T, M> [MakeExt](../field/trait.MakeExt.html "trait bevy::log::tracing_subscriber::field::MakeExt")<T> for M

where M: [MakeVisitor](../field/trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T> + Sealed<MakeExtMarker<T>>,