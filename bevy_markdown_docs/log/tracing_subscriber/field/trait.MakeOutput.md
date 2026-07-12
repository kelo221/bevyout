[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[field](index.html)

# Trait MakeOutput 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#95)

```rust
pub trait MakeOutput<T, Out>: MakeVisitor<T> + Sealed<(T, Out)>where
    Self::Visitor: VisitOutput<Out>,{
    // Provided method
    fn visit_with<F>(&self, target: T, fields: &F) -> Out
       where F: RecordFields { ... }
}
```

Extension trait implemented for all `MakeVisitor` implementations that produce a visitor implementing `VisitOutput`.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#102-104)

#### fn [visit\_with](#method.visit_with)<F>(&self, target: T, fields: [&F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> Out

where F: [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields"),

Visits all fields in `fields` with a new visitor constructed from `target`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#211-214)

### impl<T, Out, M> [MakeOutput](trait.MakeOutput.html "trait bevy::log::tracing_subscriber::field::MakeOutput")<T, Out> for M

where M: [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>, <M as [MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")<T>>::[Visitor](trait.MakeVisitor.html#associatedtype.Visitor "type bevy::log::tracing_subscriber::field::MakeVisitor::Visitor"): [VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")<Out>,