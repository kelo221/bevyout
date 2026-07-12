[bevy](../../index.html)::[log](../index.html)::[tracing](index.html)

# Trait Value 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#350)

```rust
pub trait Value: Sealed {
    // Required method
    fn record(&self, key: &Field, visitor: &mut dyn Visit);
}
```

A field value of an erased type.

Implementors of `Value` may call the appropriate typed recording methods on the [visitor](field/trait.Visit.html "trait bevy::log::tracing::field::Visit") passed to their `record` method in order to indicate how their data should be recorded.

## Required Methods

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#352)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

Visits this value with the given `Visitor`.

## Trait Implementations

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#680)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for dyn [Value](trait.Value.html "trait bevy::log::tracing::Value")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#681)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#709)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for dyn [Value](trait.Value.html "trait bevy::log::tracing::Value")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#710)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#655)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [Arguments](https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html "struct core::fmt::Arguments")<'\_>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#656)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#579)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#580)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#590)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error")

Available on **crate feature `std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#591)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#601)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")

Available on **crate feature `std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#602)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#623)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

Available on **crate feature `std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#624)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#612)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

Available on **crate feature `std`** only.

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#613)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#571)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#572)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#551-560)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#631-633)

### impl<'a, T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Value](trait.Value.html "trait bevy::log::tracing::Value") + 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#635)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#642-644)

### impl<'a, T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Value](trait.Value.html "trait bevy::log::tracing::Value") + 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#646)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#791)

### impl<T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [Value](trait.Value.html "trait bevy::log::tracing::Value"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#792)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#563)

### impl<T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>

where T: [Value](trait.Value.html "trait bevy::log::tracing::Value"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#564)

#### fn [record](#tymethod.record)(&self, key: &[Field](field/struct.Field.html "struct bevy::log::tracing::field::Field"), visitor: &mut dyn [Visit](field/trait.Visit.html "trait bevy::log::tracing::field::Visit"))

## Implementors

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#784)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [Empty](field/struct.Empty.html "struct bevy::log::tracing::field::Empty")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#674)

### impl [Value](trait.Value.html "trait bevy::log::tracing::Value") for [String](../../prelude/struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#663-665)

### impl<T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [Value](trait.Value.html "trait bevy::log::tracing::Value") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#744-746)

### impl<T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [DebugValue](field/struct.DebugValue.html "struct bevy::log::tracing::field::DebugValue")<T>

where T: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#719-721)

### impl<T> [Value](trait.Value.html "trait bevy::log::tracing::Value") for [DisplayValue](field/struct.DisplayValue.html "struct bevy::log::tracing::field::DisplayValue")<T>

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),