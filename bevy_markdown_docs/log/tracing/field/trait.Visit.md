[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[field](index.html)

# Trait Visit 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#275)

```rust
pub trait Visit {
    // Required method
    fn record_debug(&mut self, field: &Field, value: &dyn Debug);

    // Provided methods
    fn record_f64(&mut self, field: &Field, value: f64) { ... }
    fn record_i64(&mut self, field: &Field, value: i64) { ... }
    fn record_u64(&mut self, field: &Field, value: u64) { ... }
    fn record_i128(&mut self, field: &Field, value: i128) { ... }
    fn record_u128(&mut self, field: &Field, value: u128) { ... }
    fn record_bool(&mut self, field: &Field, value: bool) { ... }
    fn record_str(&mut self, field: &Field, value: &str) { ... }
    fn record_bytes(&mut self, field: &Field, value: &[u8]) { ... }
    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) { ... }
}
```

Visits typed values.

An instance of `Visit` (“a visitor”) represents the logic necessary to record field values of various types. When an implementor of [`Value`](../trait.Value.html "trait bevy::log::tracing::Value") is [recorded](../trait.Value.html#tymethod.record "method bevy::log::tracing::Value::record"), it calls the appropriate method on the provided visitor to indicate the type that value should be recorded as.

When a [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") implementation [records an `Event`](../trait.Subscriber.html#tymethod.event "method bevy::log::tracing::Subscriber::event") or a [set of `Value`s added to a `Span`](../trait.Subscriber.html#tymethod.record "method bevy::log::tracing::Subscriber::record"), it can pass an `&mut Visit` to the `record` method on the provided [`ValueSet`](struct.ValueSet.html "struct bevy::log::tracing::field::ValueSet") or [`Event`](../struct.Event.html "struct bevy::log::tracing::Event"). This visitor will then be used to record all the field-value pairs present on that `Event` or `ValueSet`.

## Examples

A simple visitor that writes to a string might be implemented like so:

```rust
use std::fmt::{self, Write};
use tracing::field::{Value, Visit, Field};
pub struct StringVisitor<'a> {
    string: &'a mut String,
}

impl<'a> Visit for StringVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        write!(self.string, "{} = {:?}; ", field.name(), value).unwrap();
    }
}
```

This visitor will format each recorded value using `fmt::Debug`, and append the field name and formatted value to the provided string, regardless of the type of the recorded value. When all the values have been recorded, the `StringVisitor` may be dropped, allowing the string to be printed or stored in some other data structure.

The `Visit` trait provides default implementations for `record_i64`, `record_u64`, `record_bool`, `record_str`, and `record_error`, which simply forward the recorded value to `record_debug`. Thus, `record_debug` is the only method which a `Visit` implementation _must_ implement. However, visitors may override the default implementations of these functions in order to implement type-specific behavior.

Additionally, when a visitor receives a value of a type it does not care about, it is free to ignore those values completely. For example, a visitor which only records numeric data might look like this:

```rust
pub struct SumVisitor {
    sum: i64,
}

impl Visit for SumVisitor {
    fn record_i64(&mut self, _field: &Field, value: i64) {
       self.sum += value;
    }

    fn record_u64(&mut self, _field: &Field, value: u64) {
        self.sum += value as i64;
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn fmt::Debug) {
        // Do nothing
    }
}
```

This visitor (which is probably not particularly useful) keeps a running sum of all the numeric values it records, and ignores all other values. A more practical example of recording typed values is presented in `examples/counters.rs`, which demonstrates a very simple metrics system implemented using `tracing`.

**Note**: The `record_error` trait method is only
available when the Rust standard library is present, as it requires the
`std::error::Error` trait.

## Required Methods

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#340)

#### fn [record\_debug](#tymethod.record_debug)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"))

Visit a value implementing `fmt::Debug`.

## Provided Methods

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#286)

#### fn [record\_f64](#method.record_f64)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Visit a double-precision floating point value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#291)

#### fn [record\_i64](#method.record_i64)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Visit a signed 64-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#296)

#### fn [record\_u64](#method.record_u64)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Visit an unsigned 64-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#301)

#### fn [record\_i128](#method.record_i128)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html))

Visit a signed 128-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#306)

#### fn [record\_u128](#method.record_u128)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html))

Visit an unsigned 128-bit integer value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#311)

#### fn [record\_bool](#method.record_bool)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Visit a boolean value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#316)

#### fn [record\_str](#method.record_str)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Visit a string value.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#321)

#### fn [record\_bytes](#method.record_bytes)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Visit a byte slice.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#335)

#### fn [record\_error](#method.record_error)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: &(dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + 'static))

Available on **crate feature `std`** only.

Records a type implementing `Error`.

**Note**: This is only enabled when the Rust standard library is
present.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#425)

### impl [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [DebugMap](https://doc.rust-lang.org/nightly/core/fmt/builders/struct.DebugMap.html "struct core::fmt::builders::DebugMap")<'\_, '\_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#426)

#### fn [record\_debug](#tymethod.record_debug)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#419)

### impl [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [DebugStruct](https://doc.rust-lang.org/nightly/core/fmt/builders/struct.DebugStruct.html "struct core::fmt::builders::DebugStruct")<'\_, '\_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#420)

#### fn [record\_debug](#tymethod.record_debug)(&mut self, field: &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), value: &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"))

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1263)

### impl [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [DefaultVisitor](../../tracing_subscriber/fmt/format/struct.DefaultVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultVisitor")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#440)

### impl [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [PrettyVisitor](../../tracing_subscriber/fmt/format/struct.PrettyVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyVisitor")<'\_>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1597-1599)

### impl<'a, F> [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [FieldFnVisitor](../../tracing_subscriber/fmt/format/struct.FieldFnVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFnVisitor")<'a, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [Writer](../../tracing_subscriber/fmt/format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'a>, &[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/delimited.rs.html#83-86)

### impl<D, V> [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [VisitDelimited](../../tracing_subscriber/field/delimited/struct.VisitDelimited.html "struct bevy::log::tracing_subscriber::field::delimited::VisitDelimited")<D, V>

where V: [VisitFmt](../../tracing_subscriber/field/trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt"), D: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#431-433)

### impl<F> [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for F

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&[Field](struct.Field.html "struct bevy::log::tracing::field::Field"), &dyn [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug")),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/debug.rs.html#37-39)

### impl<V> [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [Alt](../../tracing_subscriber/field/debug/struct.Alt.html "struct bevy::log::tracing_subscriber::field::debug::Alt")<V>

where V: [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/field/display.rs.html#39-41)

### impl<V> [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit") for [Messages](../../tracing_subscriber/field/display/struct.Messages.html "struct bevy::log::tracing_subscriber::field::display::Messages")<V>

where V: [Visit](trait.Visit.html "trait bevy::log::tracing::field::Visit"),