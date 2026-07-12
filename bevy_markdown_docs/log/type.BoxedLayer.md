[bevy](../index.html)::[log](index.html)

# Type Alias BoxedLayer 

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/lib.rs.html#253)

```rust
pub type BoxedLayer = Box<dyn Layer<Registry> + Send + Sync>;
```

A boxed [`Layer`](tracing_subscriber/trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") that can be used with [`LogPlugin::custom_layer`](struct.LogPlugin.html#structfield.custom_layer "field bevy::log::LogPlugin::custom_layer").

## Aliased Type

```rust
pub struct BoxedLayer(/* private fields */);
```