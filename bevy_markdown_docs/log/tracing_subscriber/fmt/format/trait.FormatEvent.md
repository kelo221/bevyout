[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[format](index.html)

# Trait FormatEvent 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#198)

```rust
pub trait FormatEvent<S, N>where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,{
    // Required method
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        writer: Writer<'_>,
        event: &Event<'_>,
    ) -> Result<(), Error>;
}
```

Available on **crate features `fmt` and `std`** only.

A type that can format a tracing [`Event`](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event") to a [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

[`FormatEvent`](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") is primarily used in the context of [`fmt::Subscriber`](../../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber") or [`fmt::Layer`](../struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer"). Each time an event is dispatched to [`fmt::Subscriber`](../../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber") or [`fmt::Layer`](../struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer"), the subscriber or layer forwards it to its associated [`FormatEvent`](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") to emit a log message.

This trait is already implemented for function pointers with the same signature as `format_event`.

## Arguments

The following arguments are passed to [`FormatEvent::format_event`](../trait.FormatEvent.html#tymethod.format_event "method bevy::log::tracing_subscriber::fmt::FormatEvent::format_event"):

*   A [`FmtContext`](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext"). This is an extension of the [`layer::Context`](../../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context") type, which can be used for accessing stored information such as the current span context an event occurred in.
    
    In addition, [`FmtContext`](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext") exposes access to the [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") implementation that the subscriber was configured to use via the [`FmtContext::field_format`](../struct.FmtContext.html#method.field_format "method bevy::log::tracing_subscriber::fmt::FmtContext::field_format") method. This can be used when the [`FormatEvent`](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") implementation needs to format the event’s fields.
    
    For convenience, [`FmtContext`](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext") also implements [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields"), forwarding to the configured [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") type.
    
*   A [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer") to which the formatted representation of the event is written. This type implements the [`std::fmt::Write`](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write") trait, and therefore can be used with the [`std::write!`](https://doc.rust-lang.org/nightly/core/macro.write.html "macro core::write") and [`std::writeln!`](https://doc.rust-lang.org/nightly/core/macro.writeln.html "macro core::writeln") macros, as well as calling [`std::fmt::Write`](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html "trait core::fmt::Write") methods directly.
    
    The [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer") type also implements additional methods that provide information about how the event should be formatted. The [`Writer::has_ansi_escapes`](struct.Writer.html#method.has_ansi_escapes "method bevy::log::tracing_subscriber::fmt::format::Writer::has_ansi_escapes") method indicates whether [ANSI terminal escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code) are supported by the underlying I/O writer that the event will be written to. If this returns `true`, the formatter is permitted to use ANSI escape codes to add colors and other text formatting to its output. If it returns `false`, the event will be written to an output that does not support ANSI escape codes (such as a log file), and they should not be emitted.
    
    Crates like [`nu_ansi_term`](https://crates.io/crates/nu_ansi_term) and [`owo-colors`](https://crates.io/crates/owo-colors) can be used to add ANSI escape codes to formatted output.
    
*   The actual [`Event`](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event") to be formatted.
    

## Examples

This example re-implements a simplified version of this crate’s [default formatter](struct.Full.html "struct bevy::log::tracing_subscriber::fmt::format::Full"):

```rust
use std::fmt;
use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

let _subscriber = tracing_subscriber::fmt()
    .event_format(MyFormatter)
    .init();

let _span = tracing::info_span!("my_span", answer = 42).entered();
tracing::info!(question = "life, the universe, and everything", "hello world");
```

This formatter will print events like this:

```
DEBUG yak_shaving::shaver: some-span{field-on-span=foo}: started shaving yak
```

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#204-209)

#### fn [format\_event](#tymethod.format_event)( &self, ctx: &[FmtContext](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext")<'\_, S, N>, writer: [Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>, event: &[Event](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Write a log message for [`Event`](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event") in [`FmtContext`](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext") to the given [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#212-216)

### impl<S, N> [FormatEvent](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")<S, N> for [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)(&[FmtContext](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext")<'\_, S, N>, [Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>, &[Event](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

where S: [Subscriber](../../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'a> [LookupSpan](../../registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>, N: for<'a> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'a> + 'static,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#218-223)

#### fn [format\_event](#tymethod.format_event)( &self, ctx: &[FmtContext](../struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext")<'\_, S, N>, writer: [Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")<'\_>, event: &[Event](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/pretty.rs.html#169-173)

### impl<C, N, T> [FormatEvent](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")<C, N> for [Format](struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format")<[Pretty](struct.Pretty.html "struct bevy::log::tracing_subscriber::fmt::format::Pretty"), T>

where C: [Subscriber](../../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'a> [LookupSpan](../../registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>, N: for<'a> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'a> + 'static, T: [FormatTime](../time/trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#1050-1054)

### impl<S, N, T> [FormatEvent](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")<S, N> for [Format](struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format")<[Compact](struct.Compact.html "struct bevy::log::tracing_subscriber::fmt::format::Compact"), T>

where S: [Subscriber](../../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'a> [LookupSpan](../../registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>, N: for<'a> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'a> + 'static, T: [FormatTime](../time/trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/format/mod.rs.html#922-926)

### impl<S, N, T> [FormatEvent](../trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")<S, N> for [Format](struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format")<[Full](struct.Full.html "struct bevy::log::tracing_subscriber::fmt::format::Full"), T>

where S: [Subscriber](../../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'a> [LookupSpan](../../registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>, N: for<'a> [FormatFields](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'a> + 'static, T: [FormatTime](../time/trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime"),