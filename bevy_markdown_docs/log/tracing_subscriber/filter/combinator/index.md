[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[filter](../index.html)

# Module combinator 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#48)

Filter combinators

## Structs

[And](struct.And.html "struct bevy::log::tracing_subscriber::filter::combinator::And")

Combines two [`Filter`](../../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")s so that spans and events are enabled if and only if _both_ filters return `true`.

[Not](struct.Not.html "struct bevy::log::tracing_subscriber::filter::combinator::Not")

Inverts the result of a [`Filter`](../../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter").

[Or](struct.Or.html "struct bevy::log::tracing_subscriber::filter::combinator::Or")

Combines two [`Filter`](../../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")s so that spans and events are enabled if _either_ filter returns `true`.