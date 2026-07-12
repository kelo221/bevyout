[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[callsite](index.html)

# Function rebuild\_interest\_cache 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/callsite.rs.html#222)

```rust
pub fn rebuild_interest_cache()
```

Clear and reregister interest on every [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite")

This function is intended for runtime reconfiguration of filters on traces when the filter recalculation is much less frequent than trace events are. The alternative is to have the [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") that supports runtime reconfiguration of filters always return [`Interest::sometimes()`](../subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes") so that [`enabled`](../trait.Subscriber.html#tymethod.enabled "trait bevy::log::tracing::Subscriber") is evaluated for every event.

This function will also re-compute the global maximum level as determined by the [`max_level_hint`](../trait.Subscriber.html#method.max_level_hint "method bevy::log::tracing::Subscriber::max_level_hint") method. If a [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") implementation changes the value returned by its `max_level_hint` implementation at runtime, then it **must** call this function after that value changes, in order for the change to be reflected.

See the [documentation on callsite interest caching](index.html#rebuilding-cached-interest "mod bevy::log::tracing::callsite") for additional information on this function’s usage.