[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[callsite](index.html)

# Function register 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/callsite.rs.html#236)

```rust
pub fn register(callsite: &'static dyn Callsite)
```

Register a new [`Callsite`](../trait.Callsite.html "trait bevy::log::tracing::Callsite") with the global registry.

This should be called once per callsite after the callsite has been constructed.

See the [documentation on callsite registration](index.html#registering-callsites "mod bevy::log::tracing::callsite") for details on the global callsite registry.