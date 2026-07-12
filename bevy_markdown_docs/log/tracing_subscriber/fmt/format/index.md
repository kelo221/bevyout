[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)

# Module format 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#200)

Available on **crate features `fmt` and `std`** only.

Formatters for logging [`tracing`](../../../tracing/index.html "mod bevy::log::tracing") events.

This module provides several formatter implementations, as well as utilities for implementing custom formatters.

## Formatters

This module provides a number of formatter implementations:

*   [`Full`](struct.Full.html "struct bevy::log::tracing_subscriber::fmt::format::Full"): The default formatter. This emits human-readable, single-line logs for each event that occurs, with the current span context displayed before the formatted representation of the event. See [here](struct.Full.html#example-output "struct bevy::log::tracing_subscriber::fmt::format::Full") for sample output.
    
*   [`Compact`](struct.Compact.html "struct bevy::log::tracing_subscriber::fmt::format::Compact"): A variant of the default formatter, optimized for short line lengths. Fields from the current span context are appended to the fields of the formatted event, and span names are not shown; the verbosity level is abbreviated to a single character. See [here](struct.Compact.html#example-output "struct bevy::log::tracing_subscriber::fmt::format::Compact") for sample output.
    
*   [`Pretty`](struct.Pretty.html "struct bevy::log::tracing_subscriber::fmt::format::Pretty"): Emits excessively pretty, multi-line logs, optimized for human readability. This is primarily intended to be used in local development and debugging, or for command-line applications, where automated analysis and compact storage of logs is less of a priority than readability and visual appeal. See [here](struct.Pretty.html#example-output "struct bevy::log::tracing_subscriber::fmt::format::Pretty") for sample output.
    
*   \[`Json`\]: Outputs newline-delimited JSON logs. This is intended for production use with systems where structured logs are consumed as JSON by analysis and viewing tools. The JSON output is not optimized for human readability. See [here](Json#example-output) for sample output.
    

## Structs

[Compact](struct.Compact.html "struct bevy::log::tracing_subscriber::fmt::format::Compact")

Marker for [`Format`](struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format") that indicates that the compact log format should be used.

[DefaultFields](struct.DefaultFields.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultFields")

The default [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") implementation.

[DefaultVisitor](struct.DefaultVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultVisitor")

The [visitor](../../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit") produced by [`DefaultFields`](struct.DefaultFields.html "struct bevy::log::tracing_subscriber::fmt::format::DefaultFields")’s [`MakeVisitor`](../../field/trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor") implementation.

[FieldFn](struct.FieldFn.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFn")

A [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") implementation that formats fields by calling a function or closure.

[FieldFnVisitor](struct.FieldFnVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFnVisitor")

The [visitor](../../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit") produced by [`FieldFn`](struct.FieldFn.html "struct bevy::log::tracing_subscriber::fmt::format::FieldFn")’s [`MakeVisitor`](../../field/trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor") implementation.

[FmtSpan](struct.FmtSpan.html "struct bevy::log::tracing_subscriber::fmt::format::FmtSpan")

Configures what points in the span lifecycle are logged as events.

[Format](struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format")

A pre-configured event formatter.

[Full](struct.Full.html "struct bevy::log::tracing_subscriber::fmt::format::Full")

Marker for [`Format`](struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format") that indicates that the default log format should be used.

[Pretty](struct.Pretty.html "struct bevy::log::tracing_subscriber::fmt::format::Pretty")

An excessively pretty, human-readable event formatter.

[PrettyFields](struct.PrettyFields.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyFields")

An excessively pretty, human-readable [`MakeVisitor`](../../field/trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor") implementation.

[PrettyVisitor](struct.PrettyVisitor.html "struct bevy::log::tracing_subscriber::fmt::format::PrettyVisitor")

The [visitor](../../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit") produced by [`Pretty`](struct.Pretty.html "struct bevy::log::tracing_subscriber::fmt::format::Pretty")’s [`MakeVisitor`](../../field/trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor") implementation.

[Writer](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer")

A writer to which formatted representations of spans and events are written.

## Traits

[FormatEvent](trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::format::FormatEvent")

A type that can format a tracing [`Event`](../../../tracing/struct.Event.html "struct bevy::log::tracing::Event") to a [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

[FormatFields](trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::format::FormatFields")

A type that can format a [set of fields](../../field/trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") to a [`Writer`](struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

## Functions

[debug\_fn](fn.debug_fn.html "fn bevy::log::tracing_subscriber::fmt::format::debug_fn")

Returns a [`FormatFields`](../trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") implementation that formats fields using the provided function or closure.

[format](fn.format.html "fn bevy::log::tracing_subscriber::fmt::format::format")

Returns the default configuration for an event formatter.