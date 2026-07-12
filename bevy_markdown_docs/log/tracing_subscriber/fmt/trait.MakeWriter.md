[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[fmt](index.html)

# Trait MakeWriter 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#98)

```rust
pub trait MakeWriter<'a> {
    type Writer: Write;

    // Required method
    fn make_writer(&'a self) -> Self::Writer;

    // Provided method
    fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer { ... }
}
```

Available on **crate features `fmt` and `std`** only.

A type that can create [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") instances.

`MakeWriter` is used by [`fmt::Layer`](struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer") or [`fmt::Subscriber`](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber") to print formatted text representations of [`Event`](../../tracing/struct.Event.html "struct bevy::log::tracing::Event")s.

This trait is already implemented for function pointers and immutably-borrowing closures that return an instance of [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"), such as [`io::stdout`](https://doc.rust-lang.org/nightly/std/io/stdio/fn.stdout.html "fn std::io::stdio::stdout") and [`io::stderr`](https://doc.rust-lang.org/nightly/std/io/stdio/fn.stderr.html "fn std::io::stdio::stderr"). Additionally, it is implemented for [`std::sync::Mutex`](../../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex") when the type inside the mutex implements [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write").

## Examples

The simplest usage is to pass in a named function that returns a writer. For example, to log all events to stderr, we could write:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .finish();
```

Any function that returns a writer can be used:

```rust
fn make_my_great_writer() -> impl std::io::Write {
    // ...
}

let subscriber = tracing_subscriber::fmt()
    .with_writer(make_my_great_writer)
    .finish();
```

A closure can be used to introduce arbitrary logic into how the writer is created. Consider the (admittedly rather silly) example of sending every 5th event to stderr, and all other events to stdout:

```rust
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

let n = AtomicUsize::new(0);
let subscriber = tracing_subscriber::fmt()
    .with_writer(move || -> Box<dyn io::Write> {
        if n.fetch_add(1, Relaxed) % 5 == 0 {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
       }
    })
    .finish();
```

A single instance of a type implementing [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") may be used as a `MakeWriter` by wrapping it in a [`Mutex`](../../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex"). For example, we could write to a file like so:

```rust
use std::{fs::File, sync::Mutex};

let log_file = File::create("my_cool_trace.log")?;
let subscriber = tracing_subscriber::fmt()
    .with_writer(Mutex::new(log_file))
    .finish();
```

## Required Associated Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#103)

#### type [Writer](#associatedtype.Writer): [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write")

The concrete [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") implementation returned by [`make_writer`](trait.MakeWriter.html#tymethod.make_writer "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer").

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#118)

#### fn [make\_writer](#tymethod.make_writer)(&'a self) -> Self::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")

Returns an instance of [`Writer`](trait.MakeWriter.html#associatedtype.Writer "associated type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer").

##### Implementer notes

[`fmt::Layer`](struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer") or [`fmt::Subscriber`](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber") will call this method each time an event is recorded. Ensure any state that must be saved across writes is not lost when the [`Writer`](trait.MakeWriter.html#associatedtype.Writer "associated type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer") instance is dropped. If creating a [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") instance is expensive, be sure to cache it when implementing [`MakeWriter`](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") to improve performance.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#208)

#### fn [make\_writer\_for](#method.make_writer_for)(&'a self, meta: &[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> Self::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")

Returns a [`Writer`](trait.MakeWriter.html#associatedtype.Writer "associated type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer") for writing data from the span or event described by the provided [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata").

By default, this calls [`self.make_writer()`](trait.MakeWriter.html#tymethod.make_writer "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer"), ignoring the provided metadata, but implementations can override this to provide metadata-specific behaviors.

This method allows `MakeWriter` implementations to implement different behaviors based on the span or event being written. The `MakeWriter` type might return different writers based on the provided metadata, or might write some values to the writer before or after providing it to the caller.

For example, we might want to write data from spans and events at the [`ERROR`](../../struct.Level.html#associatedconstant.ERROR "associated constant bevy::log::Level::ERROR") and [`WARN`](../../struct.Level.html#associatedconstant.WARN "associated constant bevy::log::Level::WARN") levels to `stderr`, and data from spans or events at lower levels to stdout:

```rust
use std::io::{self, Stdout, Stderr, StdoutLock, StderrLock};
use tracing_subscriber::fmt::writer::MakeWriter;
use tracing_core::{Metadata, Level};

pub struct MyMakeWriter {
    stdout: Stdout,
    stderr: Stderr,
}

/// A lock on either stdout or stderr, depending on the verbosity level
/// of the event being written.
pub enum StdioLock<'a> {
    Stdout(StdoutLock<'a>),
    Stderr(StderrLock<'a>),
}

impl<'a> io::Write for StdioLock<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            StdioLock::Stdout(lock) => lock.write(buf),
            StdioLock::Stderr(lock) => lock.write(buf),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        // ...
    }

    fn flush(&mut self) -> io::Result<()> {
        // ...
    }
}

impl<'a> MakeWriter<'a> for MyMakeWriter {
    type Writer = StdioLock<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        // We must have an implementation of `make_writer` that makes
        // a "default" writer without any configuring metadata. Let's
        // just return stdout in that case.
        StdioLock::Stdout(self.stdout.lock())
    }

    fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
        // Here's where we can implement our special behavior. We'll
        // check if the metadata's verbosity level is WARN or ERROR,
        // and return stderr in that case.
        if meta.level() <= &Level::WARN {
            return StdioLock::Stderr(self.stderr.lock());
        }

        // Otherwise, we'll return stdout.
        StdioLock::Stdout(self.stdout.lock())
    }
}
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#704)

### impl<'a> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [File](https://doc.rust-lang.org/nightly/std/fs/struct.File.html "struct std::fs::File")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#705)

#### type [Writer](#associatedtype.Writer) = &'a [File](https://doc.rust-lang.org/nightly/std/fs/struct.File.html "struct std::fs::File")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#706)

#### fn [make\_writer](#tymethod.make_writer)(&'a self) -> <[File](https://doc.rust-lang.org/nightly/std/fs/struct.File.html "struct std::fs::File") as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1129-1133)

### impl<'a, A, B, W> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [OrElse](writer/struct.OrElse.html "struct bevy::log::tracing_subscriber::fmt::writer::OrElse")<A, B>

where A: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a, Writer = [EitherWriter](writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<W, [Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink")\>>, B: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>, W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1135)

#### type [Writer](#associatedtype.Writer) = [EitherWriter](writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<W, <B as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1051-1054)

### impl<'a, A, B> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [Tee](writer/struct.Tee.html "struct bevy::log::tracing_subscriber::fmt::writer::Tee")<A, B>

where A: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>, B: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1056)

#### type [Writer](#associatedtype.Writer) = [Tee](writer/struct.Tee.html "struct bevy::log::tracing_subscriber::fmt::writer::Tee")<<A as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer"), <B as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#682-685)

### impl<'a, F, W> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for F

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")() -> W, W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#687)

#### type [Writer](#associatedtype.Writer) = W

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1014-1017)

### impl<'a, M, F> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [WithFilter](writer/struct.WithFilter.html "struct bevy::log::tracing_subscriber::fmt::writer::WithFilter")<M, F>

where M: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>, F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1019)

#### type [Writer](#associatedtype.Writer) = [EitherWriter](writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<<M as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer"), [Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#944)

### impl<'a, M> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [WithMaxLevel](writer/struct.WithMaxLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMaxLevel")<M>

where M: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#945)

#### type [Writer](#associatedtype.Writer) = [EitherWriter](writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<<M as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer"), [Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#977)

### impl<'a, M> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [WithMinLevel](writer/struct.WithMinLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMinLevel")<M>

where M: [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#978)

#### type [Writer](#associatedtype.Writer) = [EitherWriter](writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<<M as [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>>::[Writer](trait.MakeWriter.html#associatedtype.Writer "type bevy::log::tracing_subscriber::fmt::MakeWriter::Writer"), [Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#694-696)

### impl<'a, W> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<W>

where [&'a W](https://doc.rust-lang.org/nightly/std/primitive.reference.html): [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + 'a,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#698)

#### type [Writer](#associatedtype.Writer) = [&'a W](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#808-810)

### impl<'a, W> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [Mutex](../../../platform/sync/struct.Mutex.html "struct bevy::platform::sync::Mutex")<W>

where W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + 'a,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#812)

#### type [Writer](#associatedtype.Writer) = [MutexGuardWriter](writer/struct.MutexGuardWriter.html "struct bevy::log::tracing_subscriber::fmt::writer::MutexGuardWriter")<'a, W>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#773)

### impl<'a> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [BoxMakeWriter](writer/struct.BoxMakeWriter.html "struct bevy::log::tracing_subscriber::fmt::writer::BoxMakeWriter")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#774)

#### type [Writer](#associatedtype.Writer) = [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") + 'a>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#741)

### impl<'a> [MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a> for [TestWriter](struct.TestWriter.html "struct bevy::log::tracing_subscriber::fmt::TestWriter")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#742)

#### type [Writer](#associatedtype.Writer) = [TestWriter](struct.TestWriter.html "struct bevy::log::tracing_subscriber::fmt::TestWriter")