[bevy](../../../index.html)::[asset](../../index.html)::[io](../index.html)::[embedded](index.html)

# Trait GetAssetServer 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#149)

```rust
pub trait GetAssetServer {
    // Required method
    fn get_asset_server(&self) -> &AssetServer;
}
```

Trait for the [`load_embedded_asset!`](../../macro.load_embedded_asset.html "macro bevy::asset::load_embedded_asset") macro, to access [`AssetServer`](../../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") from arbitrary things.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#150)

#### fn [get\_asset\_server](#tymethod.get_asset_server)(&self) -> &[AssetServer](../../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#153)

### impl [GetAssetServer](trait.GetAssetServer.html "trait bevy::asset::io::embedded::GetAssetServer") for [App](../../../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#165)

### impl [GetAssetServer](trait.GetAssetServer.html "trait bevy::asset::io::embedded::GetAssetServer") for [AssetServer](../../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#159)

### impl [GetAssetServer](trait.GetAssetServer.html "trait bevy::asset::io::embedded::GetAssetServer") for [World](../../../prelude/struct.World.html "struct bevy::prelude::World")