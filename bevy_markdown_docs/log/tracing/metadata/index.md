[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module metadata 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/lib.rs.html#287)

Metadata describing trace data.

## Structs

[Kind](struct.Kind.html "struct bevy::log::tracing::metadata::Kind")

Indicates whether the callsite is a span or event.

[Level](struct.Level.html "struct bevy::log::tracing::metadata::Level")

Describes the level of verbosity of a span or event.

[LevelFilter](struct.LevelFilter.html "struct bevy::log::tracing::metadata::LevelFilter")

A filter comparable to a verbosity [`Level`](../../struct.Level.html "struct bevy::log::Level").

[Metadata](struct.Metadata.html "struct bevy::log::tracing::metadata::Metadata")

Metadata describing a [span](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/index.html "mod tracing_core::span") or [event](../event/index.html "mod bevy::log::tracing::event").

[ParseLevelError](struct.ParseLevelError.html "struct bevy::log::tracing::metadata::ParseLevelError")

Returned if parsing a `Level` fails.

[ParseLevelFilterError](struct.ParseLevelFilterError.html "struct bevy::log::tracing::metadata::ParseLevelFilterError")

Indicates that a string could not be parsed to a valid level.