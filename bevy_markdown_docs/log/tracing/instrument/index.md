[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module instrument 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/lib.rs.html#982)

Attach a span to a `std::future::Future`.

## Structs

[Instrumented](struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")

A [`Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") that has been instrumented with a `tracing` [`Span`](../struct.Span.html "struct bevy::log::tracing::Span").

[WithDispatch](struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")

A [`Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") that has been instrumented with a `tracing` [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

## Traits

[Instrument](trait.Instrument.html "trait bevy::log::tracing::instrument::Instrument")

Attaches spans to a [`std::future::Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future").

[WithSubscriber](trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber")

Extension trait allowing futures to be instrumented with a `tracing` [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber").