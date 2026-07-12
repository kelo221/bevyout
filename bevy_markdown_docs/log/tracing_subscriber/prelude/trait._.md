[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[prelude](index.html)

# Trait \_ 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#220)

```rust
pub trait _<'a>: MakeWriter<'a> {
    // Provided methods
    fn with_max_level(self, level: Level) -> WithMaxLevel<Self>
       where Self: Sized { ... }
    fn with_min_level(self, level: Level) -> WithMinLevel<Self>
       where Self: Sized { ... }
    fn with_filter<F>(self, filter: F) -> WithFilter<Self, F>
       where Self: Sized,
             F: Fn(&Metadata<'_>) -> bool { ... }
    fn and<B>(self, other: B) -> Tee<Self, B> ⓘ
       where Self: Sized,
             B: MakeWriter<'a> { ... }
    fn or_else<W, B>(self, other: B) -> OrElse<Self, B>
       where Self: Sized + MakeWriter<'a, Writer = EitherWriter<W, Sink>>,
             B: MakeWriter<'a>,
             W: Write { ... }
}
```

Extension trait adding combinators for working with types implementing [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter").

This is not intended to be implemented directly for user-defined [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")s; instead, it should be imported when the desired methods are used.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#279-281)

#### fn [with\_max\_level](#method.with_max_level)(self, level: [Level](../../struct.Level.html "struct bevy::log::Level")) -> [WithMaxLevel](../fmt/writer/struct.WithMaxLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMaxLevel")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps `self` and returns a [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that will only write output for events at or below the provided verbosity [`Level`](../../struct.Level.html "struct bevy::log::Level"). For instance, `Level::TRACE` is considered to be \_more verbose`than`Level::INFO\`.

Events whose level is more verbose than `level` will be ignored, and no output will be written.

##### Examples

```rust
use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;

// Construct a writer that outputs events to `stderr` only if the span or
// event's level is <= WARN (WARN and ERROR).
let mk_writer = std::io::stderr.with_max_level(Level::WARN);

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

Writing the `ERROR` and `WARN` levels to `stderr`, and everything else to `stdout`:

```rust
let mk_writer = std::io::stderr
    .with_max_level(Level::WARN)
    .or_else(std::io::stdout);

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

Writing the `ERROR` level to `stderr`, the `INFO` and `WARN` levels to `stdout`, and the `INFO` and DEBUG\` levels to a file:

```rust
use std::{sync::Arc, fs::File};
let debug_log = Arc::new(File::create("debug.log")?);

let mk_writer = std::io::stderr
    .with_max_level(Level::ERROR)
    .or_else(std::io::stdout
        .with_max_level(Level::INFO)
        .and(debug_log.with_max_level(Level::DEBUG))
    );

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#321-323)

#### fn [with\_min\_level](#method.with_min_level)(self, level: [Level](../../struct.Level.html "struct bevy::log::Level")) -> [WithMinLevel](../fmt/writer/struct.WithMinLevel.html "struct bevy::log::tracing_subscriber::fmt::writer::WithMinLevel")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps `self` and returns a [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that will only write output for events at or above the provided verbosity [`Level`](../../struct.Level.html "struct bevy::log::Level").

Events whose level is less verbose than `level` will be ignored, and no output will be written.

##### Examples

```rust
use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;

// Construct a writer that outputs events to `stdout` only if the span or
// event's level is >= DEBUG (DEBUG and TRACE).
let mk_writer = std::io::stdout.with_min_level(Level::DEBUG);

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

This can be combined with [`MakeWriterExt::with_max_level`](trait._.html#method.with_max_level "method bevy::log::tracing_subscriber::prelude::_::with_max_level") to write only within a range of levels:

```rust
// Only write the `DEBUG` and `INFO` levels to stdout.
let mk_writer = std::io::stdout
    .with_max_level(Level::DEBUG)
    .with_min_level(Level::INFO)
    // Write the `WARN` and `ERROR` levels to stderr.
    .and(std::io::stderr.with_min_level(Level::WARN));

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#400-403)

#### fn [with\_filter](#method.with_filter)<F>(self, filter: F) -> [WithFilter](../fmt/writer/struct.WithFilter.html "struct bevy::log::tracing_subscriber::fmt::writer::WithFilter")<Self, F>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Wraps `self` with a predicate that takes a span or event’s [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata") and returns a `bool`. The returned [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")’s [`MakeWriter::make_writer_for`](../fmt/trait.MakeWriter.html#method.make_writer_for "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer_for") method will check the predicate to determine if a writer should be produced for a given span or event.

If the predicate returns `false`, the wrapped [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")’s [`make_writer_for`](../fmt/trait.MakeWriter.html#method.make_writer_for "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer_for") will return [`OptionalWriter::none`](../fmt/writer/enum.EitherWriter.html#method.none "associated function bevy::log::tracing_subscriber::fmt::writer::EitherWriter::none"). Otherwise, it calls the wrapped [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")’s [`make_writer_for`](../fmt/trait.MakeWriter.html#method.make_writer_for "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer_for") method, and returns the produced writer.

This can be used to filter an output based on arbitrary [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata") parameters.

##### Examples

Writing events with a specific target to an HTTP access log, and other events to stdout:

```rust
use tracing_subscriber::fmt::writer::MakeWriterExt;
use std::{sync::Arc, fs::File};
let access_log = Arc::new(File::create("access.log")?);

let mk_writer = access_log
    // Only write events with the target "http::access_log" to the
    // access log file.
    .with_filter(|meta| meta.target() == "http::access_log")
    // Write events with all other targets to stdout.
    .or_else(std::io::stdout);

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

Conditionally enabling or disabling a log file:

```rust
use tracing_subscriber::fmt::writer::MakeWriterExt;
use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    fs::File,
};

static DEBUG_LOG_ENABLED: AtomicBool = AtomicBool::new(false);

// Create the debug log file
let debug_file = Arc::new(File::create("debug.log")?)
    // Enable the debug log only if the flag is enabled.
    .with_filter(|_| DEBUG_LOG_ENABLED.load(Ordering::Acquire));

// Always write to stdout
let mk_writer = std::io::stdout
    // Write to the debug file if it's enabled
    .and(debug_file);

tracing_subscriber::fmt().with_writer(mk_writer).init();

// ...

// Later, we can toggle on or off the debug log file.
DEBUG_LOG_ENABLED.store(true, Ordering::Release);
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#455-458)

#### fn [and](#method.and)<B>(self, other: B) -> [Tee](../fmt/writer/struct.Tee.html "struct bevy::log::tracing_subscriber::fmt::writer::Tee")<Self, B> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [MakeWriter](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

Combines `self` with another type implementing [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter"), returning a new [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that produces [writers](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") that write to _both_ outputs.

If writing to either writer returns an error, the returned writer will return that error. However, both writers will still be written to before the error is returned, so it is possible for one writer to fail while the other is written to successfully.

##### Examples

```rust
use tracing_subscriber::fmt::writer::MakeWriterExt;

// Construct a writer that outputs events to `stdout` *and* `stderr`.
let mk_writer = std::io::stdout.and(std::io::stderr);

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

`and` can be used in conjunction with filtering combinators. For example, if we want to write to a number of outputs depending on the level of an event, we could write:

```rust
use tracing::Level;
use std::{sync::Arc, fs::File};
let debug_log = Arc::new(File::create("debug.log")?);

// Write everything to the debug log.
let mk_writer = debug_log
    // Write the `ERROR` and `WARN` levels to stderr.
    .and(std::io::stderr.with_max_level(Level::WARN))
    // Write `INFO` to `stdout`.
    .and(std::io::stdout
        .with_max_level(Level::INFO)
        .with_min_level(Level::INFO)
    );

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#486-490)

#### fn [or\_else](#method.or_else)<W, B>(self, other: B) -> [OrElse](../fmt/writer/struct.OrElse.html "struct bevy::log::tracing_subscriber::fmt::writer::OrElse")<Self, B>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [MakeWriter](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a, Writer = [EitherWriter](../fmt/writer/enum.EitherWriter.html "enum bevy::log::tracing_subscriber::fmt::writer::EitherWriter")<W, [Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink")\>>, B: [MakeWriter](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>, W: [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"),

Combines `self` with another type implementing [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter"), returning a new [`MakeWriter`](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") that calls `other`’s [`make_writer`](../fmt/trait.MakeWriter.html#tymethod.make_writer "method bevy::log::tracing_subscriber::fmt::MakeWriter::make_writer") if `self`’s `make_writer` returns [`OptionalWriter::none`](../fmt/writer/enum.EitherWriter.html#method.none "associated function bevy::log::tracing_subscriber::fmt::writer::EitherWriter::none").

##### Examples

```rust
use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;

// Produces a writer that writes to `stderr` if the level is <= WARN,
// or returns `OptionalWriter::none()` otherwise.
let stderr = std::io::stderr.with_max_level(Level::WARN);

// If the `stderr` `MakeWriter` is disabled by the max level filter,
// write to stdout instead:
let mk_writer = stderr.or_else(std::io::stdout);

tracing_subscriber::fmt().with_writer(mk_writer).init();
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#1221)

### impl<'a, M> [MakeWriterExt](trait._.html "trait bevy::log::tracing_subscriber::prelude::_")<'a> for M

where M: [MakeWriter](../fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'a>,

{"Tee<Self, B>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../fmt/writer/struct.Tee.html\\" title=\\"struct bevy::log::tracing\_subscriber::fmt::writer::Tee\\">Tee</a>&lt;A, B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A, B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../fmt/writer/struct.Tee.html\\" title=\\"struct bevy::log::tracing\_subscriber::fmt::writer::Tee\\">Tee</a>&lt;A, B&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,\\n B: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>"}