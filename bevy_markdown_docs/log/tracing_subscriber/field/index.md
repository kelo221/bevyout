[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module field 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#216)

Utilities for working with [fields](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/field/index.html "mod tracing_core::field") and [field visitors](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").

## Modules

[debug](debug/index.html "mod bevy::log::tracing_subscriber::field::debug")

`MakeVisitor` wrappers for working with `fmt::Debug` fields.

[delimited](delimited/index.html "mod bevy::log::tracing_subscriber::field::delimited")

A `MakeVisitor` wrapper that separates formatted fields with a delimiter.

[display](display/index.html "mod bevy::log::tracing_subscriber::field::display")

`MakeVisitor` wrappers for working with `fmt::Display` fields.

## Traits

[MakeExt](trait.MakeExt.html "trait bevy::log::tracing_subscriber::field::MakeExt")

Extension trait providing `MakeVisitor` combinators.

[MakeOutput](trait.MakeOutput.html "trait bevy::log::tracing_subscriber::field::MakeOutput")

Extension trait implemented for all `MakeVisitor` implementations that produce a visitor implementing `VisitOutput`.

[MakeVisitor](trait.MakeVisitor.html "trait bevy::log::tracing_subscriber::field::MakeVisitor")

Creates new [visitors](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").

[RecordFields](trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields")

Extension trait implemented by types which can be recorded by a [visitor](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").

[Visit](trait.Visit.html "trait bevy::log::tracing_subscriber::field::Visit")

Visits typed values.

[VisitFmt](trait.VisitFmt.html "trait bevy::log::tracing_subscriber::field::VisitFmt")

Extension trait implemented by visitors to indicate that they write to a `fmt::Write` instance, and allow access to that writer.

[VisitOutput](trait.VisitOutput.html "trait bevy::log::tracing_subscriber::field::VisitOutput")

A [visitor](../../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit") that produces output once it has visited a set of fields.

[VisitWrite](trait.VisitWrite.html "trait bevy::log::tracing_subscriber::field::VisitWrite")`std`

Extension trait implemented by visitors to indicate that they write to an `io::Write` instance, and allow access to that writer.