[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module field 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/lib.rs.html#980)

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, referred to as _fields_. These fields consist of a mapping from a key (corresponding to a `&str` but represented internally as an array index) to a [`Value`](../trait.Value.html "trait bevy::log::tracing::Value").

## `Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [span](../span/index.html "mod bevy::log::tracing::span")s or [`Event`](../struct.Event.html "struct bevy::log::tracing::Event")s. The set of field keys on a given span or event is defined on its [`Metadata`](../struct.Metadata.html "struct bevy::log::tracing::Metadata"). When a span is created, it provides [`Attributes`](../span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes") to the `Subscriber`’s [`new_span`](../trait.Subscriber.html#tymethod.new_span "method bevy::log::tracing::Subscriber::new_span") method, containing any fields whose values were provided when the span was created; and may call the `Subscriber`’s [`record`](../span/struct.Record.html "struct bevy::log::tracing::span::Record") method with additional [`Record`](../span/struct.Record.html "struct bevy::log::tracing::span::Record")s if values are added for more of its fields. Similarly, the [`Event`](../struct.Event.html "struct bevy::log::tracing::Event") type passed to the subscriber’s [`event`](../struct.Event.html "struct bevy::log::tracing::Event") method will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives (`i64`, `u64`, `f64`, `bool`, and `&str`) or using a `fmt::Display` or `fmt::Debug` implementation. `Subscriber`s are provided these primitive value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be recorded as typed data by calling the [`Value::record`](../trait.Value.html#tymethod.record "method bevy::log::tracing::Value::record") method on these trait objects with a _visitor_ implementing the [`Visit`](trait.Visit.html "trait bevy::log::tracing::field::Visit") trait. This trait represents the behavior used to record values of various types. For example, an implementation of `Visit` might record integers by incrementing counters for their field names rather than printing them.

## Using `valuable`

`tracing`’s [`Value`](../trait.Value.html "trait bevy::log::tracing::Value") trait is intentionally minimalist: it supports only a small number of Rust primitives as typed values, and only permits recording user-defined types with their [`fmt::Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") or [`fmt::Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") implementations. However, there are some cases where it may be useful to record nested values (such as arrays, `Vec`s, or `HashMap`s containing values), or user-defined `struct` and `enum` types without having to format them as unstructured text.

To address `Value`’s limitations, `tracing` offers experimental support for the [`valuable`](https://crates.io/crates/valuable) crate, which provides object-safe inspection of structured values. User-defined types can implement the [`valuable::Valuable`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html) trait, and be recorded as a `tracing` field by calling their [`as_value`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html#tymethod.as_value) method. If the [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") also supports the `valuable` crate, it can then visit those types fields as structured values using `valuable`.

    **Note**: `valuable` support is an
    [unstable feature](../index.html#unstable-features). See
    the documentation on unstable features for details on how to enable it.

For example:

[ⓘ](# "This example is not tested")

```rust
// Derive `Valuable` for our types:
use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Clone, Debug, Valuable)]
struct Address {
    country: String,
    city: String,
    street: String,
}

let user = User {
    name: "Arwen Undomiel".to_string(),
    age: 3000,
    address: Address {
        country: "Middle Earth".to_string(),
        city: "Rivendell".to_string(),
        street: "leafy lane".to_string(),
    },
};

// Recording `user` as a `valuable::Value` will allow the `tracing` subscriber
// to traverse its fields as a nested, typed structure:
tracing::info!(current_user = user.as_value());
```

Alternatively, the \[`valuable()`\] function may be used to convert a type implementing [`Valuable`](https://crates.io/crates/valuable) into a `tracing` field value.

When the `valuable` feature is enabled, the [`Visit`](trait.Visit.html "trait bevy::log::tracing::field::Visit") trait will include an optional [`record_value`](Visit::record_value) method. `Visit` implementations that wish to record `valuable` values can implement this method with custom behavior. If a visitor does not implement `record_value`, the [`valuable::Value`](https://docs.rs/valuable/latest/valuable/enum.Value.html) will be forwarded to the visitor’s [`record_debug`](trait.Visit.html#tymethod.record_debug "method bevy::log::tracing::field::Visit::record_debug") method.

## Structs

[DebugValue](struct.DebugValue.html "struct bevy::log::tracing::field::DebugValue")

A `Value` which serializes as a string using `fmt::Debug`.

[DisplayValue](struct.DisplayValue.html "struct bevy::log::tracing::field::DisplayValue")

A `Value` which serializes using `fmt::Display`.

[Empty](struct.Empty.html "struct bevy::log::tracing::field::Empty")

An empty field.

[Field](struct.Field.html "struct bevy::log::tracing::field::Field")

An opaque key allowing _O_(1) access to a field in a `Span`’s key-value data.

[FieldSet](struct.FieldSet.html "struct bevy::log::tracing::field::FieldSet")

Describes the fields present on a span.

[Iter](struct.Iter.html "struct bevy::log::tracing::field::Iter")

An iterator over a set of fields.

[ValueSet](struct.ValueSet.html "struct bevy::log::tracing::field::ValueSet")

A set of fields and values for a span.

## Traits

[AsField](trait.AsField.html "trait bevy::log::tracing::field::AsField")

Trait implemented to allow a type to be used as a field key.

[Value](trait.Value.html "trait bevy::log::tracing::field::Value")

A field value of an erased type.

[Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit")

Visits typed values.

## Functions

[debug](fn.debug.html "fn bevy::log::tracing::field::debug")

Wraps a type implementing `fmt::Debug` as a `Value` that can be recorded using its `Debug` implementation.

[display](fn.display.html "fn bevy::log::tracing::field::display")

Wraps a type implementing `fmt::Display` as a `Value` that can be recorded using its `Display` implementation.