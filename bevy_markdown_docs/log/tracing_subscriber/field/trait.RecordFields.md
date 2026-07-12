[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[field](index.html)

# Trait RecordFields 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#88)

```rust
pub trait RecordFields: Sealed<RecordFieldsMarker> {
    // Required method
    fn record(&self, visitor: &mut dyn Visit);
}
```

Extension trait implemented by types which can be recorded by a [visitor](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").

This allows writing code that is generic over `tracing_core`’s [`span::Attributes`](../../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes"), [`span::Record`](../../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record"), and [`Event`](../../tracing/struct.Event.html "struct bevy::log::tracing::Event") types. These types all provide inherent `record` methods that allow a visitor to record their fields, but there is no common trait representing this.

With `RecordFields`, we can write code like this:

```rust
use tracing_core::field::Visit;
use tracing_subscriber::field::RecordFields;

struct MyVisitor {
    // ...
}
impl Visit for MyVisitor {
    // ...
}

fn record_with_my_visitor<R>(r: R)
where
    R: RecordFields,
{
    let mut visitor = MyVisitor::new();
    r.record(&mut visitor);
}
```

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#90)

#### fn [record](#tymethod.record)(&self, visitor: &mut dyn [Visit](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

Record all the fields in `self` with the provided `visitor`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#182-184)

### impl<F> [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") for [&F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where F: [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#186)

#### fn [record](#tymethod.record)(&self, visitor: &mut dyn [Visit](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#168)

### impl [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") for [Attributes](../../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#161)

### impl [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") for [Event](../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/mod.rs.html#175)

### impl [RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") for [Record](../../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>