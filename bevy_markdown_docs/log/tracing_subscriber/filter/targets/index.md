[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[filter](../index.html)

# Module targets 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/mod.rs.html#35)

Available on **crate features `alloc` or `std`** only.

A [filter](../../layer/index.html#filtering-with-layers "mod bevy::log::tracing_subscriber::layer") that enables or disables spans and events based on their [target](../../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target") and [level](../../../struct.Level.html "struct bevy::log::Level").

See [`Targets`](../struct.Targets.html "struct bevy::log::tracing_subscriber::filter::Targets") for details.

## Structs

[IntoIter](struct.IntoIter.html "struct bevy::log::tracing_subscriber::filter::targets::IntoIter")

An owning iterator over the [target](../../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target")\-[level](../../../struct.Level.html "struct bevy::log::Level") pairs of a `Targets` filter.

[Iter](struct.Iter.html "struct bevy::log::tracing_subscriber::filter::targets::Iter")

A borrowing iterator over the [target](../../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target")\-[level](../../../struct.Level.html "struct bevy::log::Level") pairs of a `Targets` filter.

[Targets](struct.Targets.html "struct bevy::log::tracing_subscriber::filter::targets::Targets")

A filter that enables or disables spans and events based on their [target](../../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target") and [level](../../../struct.Level.html "struct bevy::log::Level").