[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[time](index.html)

# Trait FormatTime 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#47)

```rust
pub trait FormatTime {
    // Required method
    fn format_time(&self, w: &mut Writer<'_>) -> Result<(), Error>;
}
```

Available on **crate features `fmt` and `std`** only.

A type that can measure and format the current time.

This trait is used by `Format` to include a timestamp with each `Event` when it is logged.

Notable default implementations of this trait are `SystemTime` and `()`. The former prints the current time as reported by `std::time::SystemTime`, and the latter does not print the current time at all. `FormatTime` is also automatically implemented for any function pointer with the appropriate signature.

The full list of provided implementations can be found in [`time`](index.html "mod bevy::log::tracing_subscriber::fmt::time").

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#53)

#### fn [format\_time](#tymethod.format_time)(&self, w: &mut [Writer](../format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Measure and write out the current time.

When `format_time` is called, implementors should get the current time using their desired mechanism, and write it out to the given `fmt::Write`. Implementors must insert a trailing space themselves if they wish to separate the time from subsequent log message text.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#98)

### impl [FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#99)

#### fn [format\_time](#tymethod.format_time)(&self, \_: &mut [Writer](../format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#104)

### impl [FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime") for [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)(&mut [Writer](../format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#105)

#### fn [format\_time](#tymethod.format_time)(&self, w: &mut [Writer](../format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#89-91)

### impl<F> [FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime") for [&F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where F: [FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#93)

#### fn [format\_time](#tymethod.format_time)(&self, w: &mut [Writer](../format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#136)

### impl [FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime") for [SystemTime](struct.SystemTime.html "struct bevy::log::tracing_subscriber::fmt::time::SystemTime")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#146)

### impl [FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime") for [Uptime](struct.Uptime.html "struct bevy::log::tracing_subscriber::fmt::time::Uptime")