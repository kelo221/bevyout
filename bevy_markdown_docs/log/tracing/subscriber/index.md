[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module subscriber 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/lib.rs.html#985)

Collects and records trace data.

## Structs

[DefaultGuard](struct.DefaultGuard.html "struct bevy::log::tracing::subscriber::DefaultGuard")`std`

A guard that resets the current default dispatcher to the prior default dispatcher when dropped.

[Interest](struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

Indicates a [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber")’s interest in a particular callsite.

[NoSubscriber](struct.NoSubscriber.html "struct bevy::log::tracing::subscriber::NoSubscriber")

A no-op [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

[SetGlobalDefaultError](struct.SetGlobalDefaultError.html "struct bevy::log::tracing::subscriber::SetGlobalDefaultError")

Returned if setting the global dispatcher fails.

## Traits

[Subscriber](trait.Subscriber.html "trait bevy::log::tracing::subscriber::Subscriber")

Trait representing the functions required to collect trace data.

## Functions

[set\_default](fn.set_default.html "fn bevy::log::tracing::subscriber::set_default")`std`

Sets the [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") as the default for the current thread for the duration of the lifetime of the returned [`DefaultGuard`](../dispatcher/struct.DefaultGuard.html "struct bevy::log::tracing::dispatcher::DefaultGuard").

[set\_global\_default](fn.set_global_default.html "fn bevy::log::tracing::subscriber::set_global_default")

Sets this subscriber as the global default for the duration of the entire program. Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)

[with\_default](fn.with_default.html "fn bevy::log::tracing::subscriber::with_default")`std`

Sets this [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") as the default for the current thread for the duration of a closure.