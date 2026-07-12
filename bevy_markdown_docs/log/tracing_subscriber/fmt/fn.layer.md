[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[fmt](index.html)

# Function layer 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#337)

```rust
pub fn layer<S>() -> Layer<S>
```

Available on **crate features `fmt` and `std`** only.

Returns a new [formatting layer](struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer") that can be [composed](../layer/index.html "mod bevy::log::tracing_subscriber::layer") with other layers to construct a [`Subscriber`](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber").

This is a shorthand for the equivalent [`Layer::default()`](struct.Layer.html#method.default "associated function bevy::log::tracing_subscriber::fmt::Layer::default") function.

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/app/log\_layers.rs ([line 32](../../../../src/log_layers/log_layers.rs.html#32))

```rust
29fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
30    // You can provide multiple layers like this, since Vec<Layer> is also a layer:
31    Some(Box::new(vec![
32        bevy::log::tracing_subscriber::fmt::layer()
33            .with_file(true)
34            .boxed(),
35        CustomLayer.boxed(),
36    ]))
37}
```