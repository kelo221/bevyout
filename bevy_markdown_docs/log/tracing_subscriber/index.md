[bevy](../../index.html)::[log](../index.html)

# Crate tracing\_subscriber 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#1-256)

Utilities for implementing and composing [`tracing`](https://docs.rs/tracing/latest/tracing) subscribers.

[`tracing`](https://docs.rs/tracing/latest/tracing) is a framework for instrumenting Rust programs to collect scoped, structured, and async-aware diagnostics. The [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") trait represents the functionality necessary to collect this trace data. This crate contains tools for composing subscribers out of smaller units of behaviour, and batteries-included implementations of common subscriber functionality.

`tracing-subscriber` is intended for use by both `Subscriber` authors and application authors using `tracing` to instrument their applications.

_Compiler support: [requires `rustc` 1.65+](#supported-rust-versions)_

### `Layer`s and `Filter`s

The most important component of the `tracing-subscriber` API is the [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait, which provides a composable abstraction for building [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s. Like the [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") trait, a [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") defines a particular behavior for collecting trace data. Unlike [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s, which implement a _complete_ strategy for how trace data is collected, [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s provide _modular_ implementations of specific behaviors. Therefore, they can be [composed together](layer/index.html#composing-layers "mod bevy::log::tracing_subscriber::layer") to form a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") which is capable of recording traces in a variety of ways. See the [`layer` module’s documentation](layer/index.html "mod bevy::log::tracing_subscriber::layer") for details on using [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s.

In addition, the [`Filter`](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") trait defines an interface for filtering what spans and events are recorded by a particular layer. This allows different [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s to handle separate subsets of the trace data emitted by a program. See the [documentation on per-layer filtering](layer/index.html#per-layer-filtering "mod bevy::log::tracing_subscriber::layer") for more information on using [`Filter`](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")s.

### Included Subscribers

The following `Subscriber`s are provided for application authors:

*   [`fmt`](fmt/index.html "mod bevy::log::tracing_subscriber::fmt") - Formats and logs tracing data (requires the `fmt` feature flag)

### Feature Flags

*   `std`: Enables APIs that depend on the Rust standard library (enabled by default).
*   `alloc`: Depend on [`liballoc`](https://doc.rust-lang.org/alloc/index.html) (enabled by “std”).
*   `env-filter`: Enables the [`EnvFilter`](struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter") type, which implements filtering similar to the [`env_logger` crate](https://crates.io/crates/env_logger). **Requires “std”**.
*   `fmt`: Enables the [`fmt`](fmt/index.html "mod bevy::log::tracing_subscriber::fmt") module, which provides a subscriber implementation for printing formatted representations of trace events. Enabled by default. **Requires “registry” and “std”**.
*   `ansi`: Enables `fmt` support for ANSI terminal colors. Enabled by default.
*   `registry`: enables the [`registry`](registry/index.html "mod bevy::log::tracing_subscriber::registry") module. Enabled by default. **Requires “std”**.
*   `json`: Enables `fmt` support for JSON output. In JSON output, the ANSI feature does nothing. **Requires “fmt” and “std”**.
*   `local-time`: Enables local time formatting when using the [`time` crate](https://crates.io/crates/time)’s timestamp formatters with the `fmt` subscriber.

#### Optional Dependencies

*   [`tracing-log`](https://crates.io/crates/tracing-log): Enables better formatting for events emitted by `log` macros in the `fmt` subscriber. Enabled by default.
*   [`time`](https://crates.io/crates/time): Enables support for using the [`time` crate](https://crates.io/crates/time) for timestamp formatting in the `fmt` subscriber.
*   [`smallvec`](https://crates.io/crates/smallvec): Causes the `EnvFilter` type to use the `smallvec` crate (rather than `Vec`) as a performance optimization. Enabled by default.
*   [`parking_lot`](https://crates.io/crates/parking_lot): Use the `parking_lot` crate’s `RwLock` implementation rather than the Rust standard library’s implementation.

#### `no_std` Support

In embedded systems and other bare-metal applications, `tracing` can be used without requiring the Rust standard library, although some features are disabled. Although most of the APIs provided by `tracing-subscriber`, such as [`fmt`](fmt/index.html "mod bevy::log::tracing_subscriber::fmt") and [`EnvFilter`](struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter"), require the standard library, some functionality, such as the [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait, can still be used in `no_std` environments.

The dependency on the standard library is controlled by two crate feature flags, “std”, which enables the dependency on [`libstd`](https://doc.rust-lang.org/std/index.html), and “alloc”, which enables the dependency on [`liballoc`](https://doc.rust-lang.org/alloc/index.html) (and is enabled by the “std” feature). These features are enabled by default, but `no_std` users can disable them using:

```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false }
```

Additional APIs are available when [`liballoc`](https://doc.rust-lang.org/alloc/index.html) is available. To enable `liballoc` but not `std`, use:

```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
```

#### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x releases. To enable these features, the `--cfg tracing_unstable` must be passed to `rustc` when compiling.

The following unstable feature flags are currently available:

*   `valuable`: Enables support for serializing values recorded using the [`valuable`](https://crates.io/crates/valuable) crate as structured JSON in the [`format::Json`](crate::fmt::format::Json) formatter.

##### Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS` env variable when running `cargo` commands:

```
RUSTFLAGS="--cfg tracing_unstable" cargo build
```

Alternatively, the following can be added to the `.cargo/config` file in a project to automatically enable the cfg flag for that project:

```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

### Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported version is 1.65. The current Tracing version is not guaranteed to build on Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio project. The current stable Rust compiler and the three most recent minor versions before it will always be supported. For example, if the current stable compiler version is 1.69, the minimum supported version will not be increased past 1.66, three minor versions prior. Increasing the minimum supported compiler version is not considered a semver breaking change as long as doing so complies with this policy.

## Modules

[field](field/index.html "mod bevy::log::tracing_subscriber::field")

Utilities for working with [fields](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/field/index.html "mod tracing_core::field") and [field visitors](../tracing/field/trait.Visit.html "trait bevy::log::tracing::field::Visit").

[filter](filter/index.html "mod bevy::log::tracing_subscriber::filter")

[`Layer`](layer/index.html "mod bevy::log::tracing_subscriber::layer")s that control which spans and events are enabled by the wrapped subscriber.

[fmt](fmt/index.html "mod bevy::log::tracing_subscriber::fmt")`fmt` and `std`

A `Subscriber` for formatting and logging `tracing` data.

[layer](layer/index.html "mod bevy::log::tracing_subscriber::layer")

The [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait, a composable abstraction for building [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s.

[prelude](prelude/index.html "mod bevy::log::tracing_subscriber::prelude")

The `tracing-subscriber` prelude.

[registry](registry/index.html "mod bevy::log::tracing_subscriber::registry")

Storage for span data shared by multiple [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s.

[reload](reload/index.html "mod bevy::log::tracing_subscriber::reload")`std`

Wrapper for a `Layer` to allow it to be dynamically reloaded.

[util](util/index.html "mod bevy::log::tracing_subscriber::util")

Extension traits and other utilities to make working with subscribers more ergonomic.

## Structs

[EnvFilter](struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter")

A [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") which filters spans and events based on a set of filter directives.

[FmtSubscriber](struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber")

A `Subscriber` that logs formatted representations of `tracing` events.

[Registry](struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")`registry`

A shared, reusable store for spans.

## Traits

[Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")

A composable handler for `tracing` events.

## Functions

[fmt](fn.fmt.html "fn bevy::log::tracing_subscriber::fmt")

Returns a new [`SubscriberBuilder`](fmt/struct.SubscriberBuilder.html "struct bevy::log::tracing_subscriber::fmt::SubscriberBuilder") for configuring a [formatting subscriber](struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber").

[registry](fn.registry.html "fn bevy::log::tracing_subscriber::registry")`registry` and `std`

Returns a default [`Registry`](struct.Registry.html "struct bevy::log::tracing_subscriber::Registry").