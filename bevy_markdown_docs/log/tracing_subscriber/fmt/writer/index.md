[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)

# Module writer 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#204)

Available on **crate features `fmt` and `std`** only.

Abstractions for creating [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") instances.

## Structs

[BoxMakeWriter](struct.BoxMakeWriter.html "struct bevy::log::tracing_subscriber::fmt::writer::BoxMakeWriter")

A writer that erases the specific [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") and [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") types being used.

[MutexGuardWriter](struct.MutexGuardWriter.html "struct bevy::log::tracing_subscriber::fmt::writer::MutexGuardWriter")

A type implementing [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") for a [`MutexGuard`](../../../../platform/sync/struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard") where the type inside the [`Mutex`](../../../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex") implements [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write").

[OrElse](struct.OrElse.html "struct bevy::log::tracing_subscriber::fmt::writer::OrElse")

Combines a [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that returns an [`OptionalWriter`](type.OptionalWriter.html "type bevy::log::tracing_subscriber::fmt::writer::OptionalWriter") with another [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter"), so that the second [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") is used when the first [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") returns [`OptionalWriter::none`](enum.EitherWriter.html#method.none "associated function bevy::log::tracing_subscriber::fmt::writer::EitherWriter::none").

[Tee](struct.Tee.html "struct bevy::log::tracing_subscriber::fmt::writer::Tee")

Combines two types implementing [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") (or [`std::io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write")) to produce a writer that writes to both [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")’s returned writers.

[TestWriter](struct.TestWriter.html "struct bevy::log::tracing_subscriber::fmt::writer::TestWriter")

A writer intended to support [`libtest`’s output capturing](https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output) for use in unit tests.

[WithFilter](struct.WithFilter.html "struct bevy::log::tracing_subscriber::fmt::writer::WithFilter")

A [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") combinator that wraps a [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") with a predicate for span and event [`Metadata`](../../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"), so that the [`MakeWriter::make_writer_for`](../trait.MakeWriter.html#method.make_writer_for "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer_for") method returns [`OptionalWriter::some`](enum.EitherWriter.html#method.some "associated function bevy::log::tracing_subscriber::fmt::writer::EitherWriter::some") when the predicate returns `true`, and [`OptionalWriter::none`](enum.EitherWriter.html#method.none "associated function bevy::log::tracing_subscriber::fmt::writer::EitherWriter::none") when the predicate returns `false`.

[WithMaxLevel](struct.WithMaxLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMaxLevel")

A [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") combinator that only returns an enabled [writer](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") for spans and events with metadata at or below a specified verbosity [`Level`](../../../struct.Level.html "struct bevy::log::Level").

[WithMinLevel](struct.WithMinLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMinLevel")

A [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") combinator that only returns an enabled [writer](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") for spans and events with metadata at or above a specified verbosity [`Level`](../../../struct.Level.html "struct bevy::log::Level").

## Enums

[EitherWriter](enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")

A [writer](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") that is one of two types implementing [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write").

## Traits

[MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::writer::MakeWriter")

A type that can create [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") instances.

[MakeWriterExt](trait.MakeWriterExt.html "trait bevy::log::tracing_subscriber::fmt::writer::MakeWriterExt")

Extension trait adding combinators for working with types implementing [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter").

## Type Aliases

[OptionalWriter](type.OptionalWriter.html "type bevy::log::tracing_subscriber::fmt::writer::OptionalWriter")

A [writer](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") which may or may not be enabled.