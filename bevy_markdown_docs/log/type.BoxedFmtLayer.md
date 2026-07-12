[bevy](../index.html)::[log](index.html)

# Type Alias BoxedFmtLayer 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/lib.rs.html#267)

```rust
pub type BoxedFmtLayer = Box<dyn Layer<Layered<ErrorLayer<Layered<EnvFilter, Layered<Option<Box<dyn Layer<Registry> + Send + Sync>>, Registry>>>, Layered<EnvFilter, Layered<Option<Box<dyn Layer<Registry> + Send + Sync>>, Registry>>>> + Send + Sync>;
```

A boxed [`Layer`](tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") that can be used with [`LogPlugin::fmt_layer`](struct.LogPlugin.html#structfield.fmt_layer "field bevy::log::LogPlugin::fmt_layer").

## Aliased Type

```rust
pub struct BoxedFmtLayer(/* private fields */);
```