[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module reload 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#226)

Available on **crate feature `std`** only.

Wrapper for a `Layer` to allow it to be dynamically reloaded.

This module provides a [`Layer` type](struct.Layer.html "struct bevy::log::tracing_subscriber::reload::Layer") implementing the [`Layer` trait](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") or [`Filter` trait](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") which wraps another type implementing the corresponding trait. This allows the wrapped type to be replaced with another instance of that type at runtime.

This can be used in cases where a subset of `Layer` or `Filter` functionality should be dynamically reconfigured, such as when filtering directives may change at runtime. Note that this layer introduces a (relatively small) amount of overhead, and should thus only be used as needed.

## Examples

Reloading a [global filtering](../layer/index.html#global-filtering "mod bevy::log::tracing_subscriber::layer") layer:

```rust
use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filter = filter::LevelFilter::WARN;
let (filter, reload_handle) = reload::Layer::new(filter);
tracing_subscriber::registry()
  .with(filter)
  .with(fmt::Layer::default())
  .init();
info!("This will be ignored");
reload_handle.modify(|filter| *filter = filter::LevelFilter::INFO);
info!("This will be logged");
```

Reloading a [`Filtered`](../filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") layer:

```rust
use tracing_subscriber::{filter, fmt, reload, prelude::*};
let filtered_layer = fmt::Layer::default().with_filter(filter::LevelFilter::WARN);
let (filtered_layer, reload_handle) = reload::Layer::new(filtered_layer);
tracing_subscriber::registry()
  .with(filtered_layer)
  .init();
info!("This will be ignored");
reload_handle.modify(|layer| *layer.filter_mut() = filter::LevelFilter::INFO);
info!("This will be logged");
```

### Note

The [`Layer`](struct.Layer.html "struct bevy::log::tracing_subscriber::reload::Layer") implementation is unable to implement downcasting functionality, so certain [`Layer`](struct.Layer.html "struct bevy::log::tracing_subscriber::reload::Layer") will fail to downcast if wrapped in a `reload::Layer`.

If you only want to be able to dynamically change the `Filter` on a layer, prefer wrapping that `Filter` in the `reload::Layer`.

## Structs

[Error](struct.Error.html "struct bevy::log::tracing_subscriber::reload::Error")

Indicates that an error occurred when reloading a layer.

[Handle](struct.Handle.html "struct bevy::log::tracing_subscriber::reload::Handle")

Allows reloading the state of an associated [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer").

[Layer](struct.Layer.html "struct bevy::log::tracing_subscriber::reload::Layer")

Wraps a `Layer` or `Filter`, allowing it to be reloaded dynamically at runtime.