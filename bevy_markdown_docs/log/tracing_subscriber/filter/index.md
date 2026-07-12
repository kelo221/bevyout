[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module filter 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#217)

[`Layer`](../layer/index.html "mod bevy::log::tracing_subscriber::layer")s that control which spans and events are enabled by the wrapped subscriber.

This module contains a number of types that provide implementations of various strategies for filtering which spans and events are enabled. For details on filtering spans and events using [`Layer`](../layer/index.html "mod bevy::log::tracing_subscriber::layer")s, see the [`layer` module’s documentation](../layer/index.html#filtering-with-layers "mod bevy::log::tracing_subscriber::layer").

## Modules

[combinator](combinator/index.html "mod bevy::log::tracing_subscriber::filter::combinator")

Filter combinators

[targets](targets/index.html "mod bevy::log::tracing_subscriber::filter::targets")`alloc` or `std`

A [filter](../layer/index.html#filtering-with-layers "mod bevy::log::tracing_subscriber::layer") that enables or disables spans and events based on their [target](../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target") and [level](../../struct.Level.html "struct bevy::log::Level").

## Structs

[BadFieldName](struct.BadFieldName.html "struct bevy::log::tracing_subscriber::filter::BadFieldName")

Indicates that a field name specified in a filter directive was invalid.

[Builder](struct.Builder.html "struct bevy::log::tracing_subscriber::filter::Builder")

A [builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html) for constructing new [`EnvFilter`](../struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter")s.

[Directive](struct.Directive.html "struct bevy::log::tracing_subscriber::filter::Directive")

A single filtering directive.

[DynFilterFn](struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn")

A filter implemented by a closure or function pointer that determines whether a given span or event is enabled _dynamically_, potentially based on the current [span context](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context").

[EnvFilter](struct.EnvFilter.html "struct bevy::log::tracing_subscriber::filter::EnvFilter")

A [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") which filters spans and events based on a set of filter directives.

[FilterFn](struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn")

A filter implemented by a closure or function pointer that determines whether a given span or event is enabled, based on its [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata").

[FilterId](struct.FilterId.html "struct bevy::log::tracing_subscriber::filter::FilterId")`registry`

Uniquely identifies an individual [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") instance in the context of a [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

[Filtered](struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered")

A [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") that wraps an inner [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") and adds a [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") which controls what spans and events are enabled for that layer.

[FromEnvError](struct.FromEnvError.html "struct bevy::log::tracing_subscriber::filter::FromEnvError")

Indicates that an error occurred while parsing a `EnvFilter` from an environment variable.

[LevelFilter](struct.LevelFilter.html "struct bevy::log::tracing_subscriber::filter::LevelFilter")

A filter comparable to a verbosity [`Level`](../../struct.Level.html "struct bevy::log::Level").

[LevelParseError](struct.LevelParseError.html "struct bevy::log::tracing_subscriber::filter::LevelParseError")

Indicates that a string could not be parsed to a valid level.

[ParseError](struct.ParseError.html "struct bevy::log::tracing_subscriber::filter::ParseError")

Indicates that a string could not be parsed as a filtering directive.

[Targets](struct.Targets.html "struct bevy::log::tracing_subscriber::filter::Targets")

A filter that enables or disables spans and events based on their [target](../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target") and [level](../../struct.Level.html "struct bevy::log::Level").

## Traits

[FilterExt](trait.FilterExt.html "trait bevy::log::tracing_subscriber::filter::FilterExt")

Extension trait adding [combinators](combinator/index.html "mod bevy::log::tracing_subscriber::filter::combinator") for combining [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter").

## Functions

[dynamic\_filter\_fn](fn.dynamic_filter_fn.html "fn bevy::log::tracing_subscriber::filter::dynamic_filter_fn")

Constructs a [`DynFilterFn`](struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn") from a function or closure that returns `true` if a span or event should be enabled within a particular [span context](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context").

[filter\_fn](fn.filter_fn.html "fn bevy::log::tracing_subscriber::filter::filter_fn")

Constructs a [`FilterFn`](struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn"), from a function or closure that returns `true` if a span or event should be enabled, based on its [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata").