[bevy](../../index.html)::[asset](../index.html)::[prelude](index.html)

# Function asset\_value 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#379)

```rust
pub fn asset_value<I, A>(asset: I) -> HandleTemplate<A>where
    I: Into<A>,
    A: Asset,
```

This will create a new [`HandleTemplate`](../enum.HandleTemplate.html "enum bevy::asset::HandleTemplate") for the given `asset` value. This makes it possible to define assets “inline” in templates / scenes that produce a [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle").

This supports [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") to automatically convert values that can become `A`.