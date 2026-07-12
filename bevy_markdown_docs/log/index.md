[bevy](../index.html)

# Crate log 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/lib.rs.html#1-435)

This crate provides logging functions and configuration for [Bevy](https://bevy.org) apps, and automatically configures platform specific log handlers (i.e. Wasm or Android).

The macros provided for logging are reexported from [`tracing`](https://docs.rs/tracing), and behave identically to it.

By default, the [`LogPlugin`](struct.LogPlugin.html "struct bevy::log::LogPlugin") from this crate is included in Bevy’s `DefaultPlugins` and the logging macros can be used out of the box, if used.

For more fine-tuned control over logging behavior, set up the [`LogPlugin`](struct.LogPlugin.html "struct bevy::log::LogPlugin") or `DefaultPlugins` during app initialization.

## Modules

[prelude](prelude/index.html "mod bevy::log::prelude")

The log prelude.

[tracing](tracing/index.html "mod bevy::log::tracing")

A scoped, structured logging and diagnostics system.

[tracing\_subscriber](tracing_subscriber/index.html "mod bevy::log::tracing_subscriber")

Utilities for implementing and composing [`tracing`](https://docs.rs/tracing/latest/tracing) subscribers.

## Macros

[debug](macro.debug.html "macro bevy::log::debug")

Constructs an event at the debug level.

[debug\_once](macro.debug_once.html "macro bevy::log::debug_once")

Call [`debug!`](../prelude/macro.debug.html "macro bevy::prelude::debug") once per call site.

[debug\_span](macro.debug_span.html "macro bevy::log::debug_span")

Constructs a span at the debug level.

[error](macro.error.html "macro bevy::log::error")

Constructs an event at the error level.

[error\_once](macro.error_once.html "macro bevy::log::error_once")

Call [`error!`](../prelude/macro.error.html "macro bevy::prelude::error") once per call site.

[error\_span](macro.error_span.html "macro bevy::log::error_span")

Constructs a span at the error level.

[event](macro.event.html "macro bevy::log::event")

Constructs a new `Event`.

[info](macro.info.html "macro bevy::log::info")

Constructs an event at the info level.

[info\_once](macro.info_once.html "macro bevy::log::info_once")

Call [`info!`](../prelude/macro.info.html "macro bevy::prelude::info") once per call site.

[info\_span](macro.info_span.html "macro bevy::log::info_span")

Constructs a span at the info level.

[once](macro.once.html "macro bevy::log::once")

Call some expression only once per call site.

[trace](macro.trace.html "macro bevy::log::trace")

Constructs an event at the trace level.

[trace\_once](macro.trace_once.html "macro bevy::log::trace_once")

Call [`trace!`](../prelude/macro.trace.html "macro bevy::prelude::trace") once per call site.

[trace\_span](macro.trace_span.html "macro bevy::log::trace_span")

Constructs a span at the trace level.

[warn](macro.warn.html "macro bevy::log::warn")

Constructs an event at the warn level.

[warn\_once](macro.warn_once.html "macro bevy::log::warn_once")

Call [`warn!`](../prelude/macro.warn.html "macro bevy::prelude::warn") once per call site.

[warn\_span](macro.warn_span.html "macro bevy::log::warn_span")

Constructs a span at the warn level.

## Structs

[Level](struct.Level.html "struct bevy::log::Level")

Describes the level of verbosity of a span or event.

[LogPlugin](struct.LogPlugin.html "struct bevy::log::LogPlugin")

Adds logging to Apps. This plugin is part of the `DefaultPlugins`. Adding this plugin will setup a collector appropriate to your target platform:

## Constants

[DEFAULT\_FILTER](constant.DEFAULT_FILTER.html "constant bevy::log::DEFAULT_FILTER")

The default [`LogPlugin`](struct.LogPlugin.html "struct bevy::log::LogPlugin") [`EnvFilter`](tracing_subscriber/struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter").

## Type Aliases

[BoxedFmtLayer](type.BoxedFmtLayer.html "type bevy::log::BoxedFmtLayer")

A boxed [`Layer`](tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") that can be used with [`LogPlugin::fmt_layer`](struct.LogPlugin.html#structfield.fmt_layer "field bevy::log::LogPlugin::fmt_layer").

[BoxedLayer](type.BoxedLayer.html "type bevy::log::BoxedLayer")

A boxed [`Layer`](tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") that can be used with [`LogPlugin::custom_layer`](struct.LogPlugin.html#structfield.custom_layer "field bevy::log::LogPlugin::custom_layer").