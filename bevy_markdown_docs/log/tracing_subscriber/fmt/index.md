[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module fmt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#232)

Available on **crate features `fmt` and `std`** only.

A `Subscriber` for formatting and logging `tracing` data.

## Overview

[`tracing`](https://crates.io/crates/tracing) is a framework for instrumenting Rust programs with context-aware, structured, event-based diagnostic information. This crate provides an implementation of the [`Subscriber`](https://docs.rs/tracing/latest/tracing/trait.Subscriber.html) trait that records `tracing`’s `Event`s and `Span`s by formatting them as text and logging them to stdout.

## Usage

First, add this to your `Cargo.toml` file:

```toml
[dependencies]
tracing-subscriber = "0.3"
```

_Compiler support: [requires `rustc` 1.65+](../index.html#supported-rust-versions "mod bevy::log::tracing_subscriber")_

Add the following to your executable to initialize the default subscriber:

```rust
use tracing_subscriber;

tracing_subscriber::fmt::init();
```

### Filtering Events with Environment Variables

The default subscriber installed by `init` enables you to filter events at runtime using environment variables (using the [`EnvFilter`](../struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter")).

The filter syntax is a superset of the [`env_logger`](https://docs.rs/env_logger/) syntax.

For example:

*   Setting `RUST_LOG=debug` enables all `Span`s and `Event`s set to the log level `DEBUG` or higher
*   Setting `RUST_LOG=my_crate=trace` enables `Span`s and `Event`s in `my_crate` at all log levels

**Note**: This should **not** be called by libraries. Libraries should use [`tracing`](https://crates.io/crates/tracing) to publish `tracing` `Event`s.

## Configuration

You can configure a subscriber instead of using the defaults with the following functions:

#### Subscriber

The [`FmtSubscriber`](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber") formats and records `tracing` events as line-oriented logs. You can create one by calling:

```rust
let subscriber = tracing_subscriber::fmt()
    // ... add configuration
    .finish();
```

You can find the configuration methods for [`FmtSubscriber`](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber") in [`SubscriberBuilder`](struct.SubscriberBuilder.html "struct bevy::log::tracing_subscriber::fmt::SubscriberBuilder").

### Formatters

The output format used by the layer and subscriber in this module is represented by implementing the [`FormatEvent`](trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") trait, and can be customized. This module provides a number of formatter implementations:

*   [`format::Full`](format/struct.Full.html "struct bevy::log::tracing_subscriber::fmt::format::Full"): The default formatter. This emits human-readable, single-line logs for each event that occurs, with the current span context displayed before the formatted representation of the event. See [here](format/struct.Full.html#example-output "struct bevy::log::tracing_subscriber::fmt::format::Full") for sample output.
    
*   [`format::Compact`](format/struct.Compact.html "struct bevy::log::tracing_subscriber::fmt::format::Compact"): A variant of the default formatter, optimized for short line lengths. Fields from the current span context are appended to the fields of the formatted event. See [here](format/struct.Compact.html#example-output "struct bevy::log::tracing_subscriber::fmt::format::Compact") for sample output.
    
*   [`format::Pretty`](format/struct.Pretty.html "struct bevy::log::tracing_subscriber::fmt::format::Pretty"): Emits excessively pretty, multi-line logs, optimized for human readability. This is primarily intended to be used in local development and debugging, or for command-line applications, where automated analysis and compact storage of logs is less of a priority than readability and visual appeal. See [here](format/struct.Pretty.html#example-output "struct bevy::log::tracing_subscriber::fmt::format::Pretty") for sample output.
    
*   \[`format::Json`\]: Outputs newline-delimited JSON logs. This is intended for production use with systems where structured logs are consumed as JSON by analysis and viewing tools. The JSON output is not optimized for human readability. See [here](format::Json#example-output) for sample output.
    

#### Customizing Formatters

The formatting of log lines for spans and events is controlled by two traits, [`FormatEvent`](trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") and [`FormatFields`](trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields"). The [`FormatEvent`](trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") trait determines the overall formatting of the log line, such as what information from the event’s metadata and span context is included and in what order. The [`FormatFields`](trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields") trait determines how fields — both the event’s fields and fields on spans — are formatted.

The [`fmt::format`](format/index.html "mod bevy::log::tracing_subscriber::fmt::format") module provides several types which implement these traits, many of which expose additional configuration options to customize their output. The [`format::Format`](format/struct.Format.html "struct bevy::log::tracing_subscriber::fmt::format::Format") type implements common configuration used by all the formatters provided in this crate, and can be used as a builder to set specific formatting settings. For example:

```rust
use tracing_subscriber::fmt;

// Configure a custom event formatter
let format = fmt::format()
   .with_level(false) // don't include levels in formatted output
   .with_target(false) // don't include targets
   .with_thread_ids(true) // include the thread ID of the current thread
   .with_thread_names(true) // include the name of the current thread
   .compact(); // use the `Compact` formatting style.

// Create a `fmt` subscriber that uses our custom event format, and set it
// as the default.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

However, if a specific output format is needed, other crates can also implement [`FormatEvent`](trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent") and [`FormatFields`](trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields"). See those traits’ documentation for details on how to implement them.

### Filters

If you want to filter the `tracing` `Events` based on environment variables, you can use the [`EnvFilter`](../struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter") as follows:

```rust
use tracing_subscriber::EnvFilter;

let filter = EnvFilter::from_default_env();
```

As mentioned above, the [`EnvFilter`](../struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter") allows `Span`s and `Event`s to be filtered at runtime by setting the `RUST_LOG` environment variable.

You can find the other available [`filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter")s in the documentation.

#### Using Your Subscriber

Finally, once you have configured your `Subscriber`, you need to configure your executable to use it.

A subscriber can be installed globally using:

```rust
use tracing;
use tracing_subscriber::FmtSubscriber;

let subscriber = FmtSubscriber::new();

tracing::subscriber::set_global_default(subscriber)
    .map_err(|_err| eprintln!("Unable to set global default subscriber"));
// Note this will only fail if you try to set the global default
// subscriber multiple times
```

#### Composing Layers

Composing an [`EnvFilter`](../struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter") `Layer` and a [format `Layer`](struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer"):

```rust
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;

let fmt_layer = fmt::layer()
    .with_target(false);
let filter_layer = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();

tracing_subscriber::registry()
    .with(filter_layer)
    .with(fmt_layer)
    .init();
```

## Modules

[format](format/index.html "mod bevy::log::tracing_subscriber::fmt::format")

Formatters for logging [`tracing`](../../tracing/index.html "mod bevy::log::tracing") events.

[time](time/index.html "mod bevy::log::tracing_subscriber::fmt::time")

Formatters for event timestamps.

[writer](writer/index.html "mod bevy::log::tracing_subscriber::fmt::writer")

Abstractions for creating [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") instances.

## Structs

[FmtContext](struct.FmtContext.html "struct bevy::log::tracing_subscriber::fmt::FmtContext")

Provides the current span context to a formatter.

[FormattedFields](struct.FormattedFields.html "struct bevy::log::tracing_subscriber::fmt::FormattedFields")

A formatted representation of a span’s fields stored in its [extensions](../registry/struct.Extensions.html "struct bevy::log::tracing_subscriber::registry::Extensions").

[Layer](struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")

A [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") that logs formatted representations of `tracing` events.

[Subscriber](struct.Subscriber.html "struct bevy::log::tracing_subscriber::fmt::Subscriber")

A `Subscriber` that logs formatted representations of `tracing` events.

[SubscriberBuilder](struct.SubscriberBuilder.html "struct bevy::log::tracing_subscriber::fmt::SubscriberBuilder")

Configures and constructs `Subscriber`s.

[TestWriter](struct.TestWriter.html "struct bevy::log::tracing_subscriber::fmt::TestWriter")

A writer intended to support [`libtest`’s output capturing](https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output) for use in unit tests.

## Traits

[FormatEvent](trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")

A type that can format a tracing [`Event`](../../tracing/struct.Event.html "struct bevy::log::tracing::Event") to a [`Writer`](format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

[FormatFields](trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")

A type that can format a [set of fields](../field/trait.RecordFields.html "trait bevy::log::tracing_subscriber::field::RecordFields") to a [`Writer`](format/struct.Writer.html "struct bevy::log::tracing_subscriber::fmt::format::Writer").

[MakeWriter](trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")

A type that can create [`io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") instances.

## Functions

[fmt](fn.fmt.html "fn bevy::log::tracing_subscriber::fmt::fmt")

Returns a new [`SubscriberBuilder`](struct.SubscriberBuilder.html "struct bevy::log::tracing_subscriber::fmt::SubscriberBuilder") for configuring a [formatting subscriber](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber").

[format](fn.format.html "fn bevy::log::tracing_subscriber::fmt::format")

Returns the default configuration for an event formatter.

[init](fn.init.html "fn bevy::log::tracing_subscriber::fmt::init")

Install a global tracing subscriber that listens for events and filters based on the value of the [`RUST_LOG` environment variable](../struct.EnvFilter.html#associatedconstant.DEFAULT_ENV "associated constant bevy::log::tracing_subscriber::EnvFilter::DEFAULT_ENV").

[layer](fn.layer.html "fn bevy::log::tracing_subscriber::fmt::layer")

Returns a new [formatting layer](struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer") that can be [composed](../layer/index.html "mod bevy::log::tracing_subscriber::layer") with other layers to construct a [`Subscriber`](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber").

[time](fn.time.html "fn bevy::log::tracing_subscriber::fmt::time")

Returns a new `SystemTime` timestamp provider.

[try\_init](fn.try_init.html "fn bevy::log::tracing_subscriber::fmt::try_init")

Install a global tracing subscriber that listens for events and filters based on the value of the [`RUST_LOG` environment variable](../struct.EnvFilter.html#associatedconstant.DEFAULT_ENV "associated constant bevy::log::tracing_subscriber::EnvFilter::DEFAULT_ENV"), if one is not already set.

## Type Aliases

[Formatter](type.Formatter.html "type bevy::log::tracing_subscriber::fmt::Formatter")

A `Subscriber` that logs formatted representations of `tracing` events. This type only logs formatted events; it does not perform any filtering.