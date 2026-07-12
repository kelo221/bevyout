[bevy](../index.html)::[asset](index.html)

# Trait DirectAssetAccessExt 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/direct_access_ext.rs.html#9)

```rust
pub trait DirectAssetAccessExt {
    // Required methods
    fn add_asset<A>(&mut self, asset: impl Into<A>) -> Handle<A>
       where A: Asset;
    fn load_asset<'a, A>(&self, path: impl Into<AssetPath<'a>>) -> Handle<A>
       where A: Asset;
    fn load_builder(&self) -> LoadBuilder<'_>;
    fn load_asset_with_settings<'a, A, S>(
        &self,
        path: impl Into<AssetPath<'a>>,
        settings: impl Fn(&mut S) + Send + Sync + 'static,
    ) -> Handle<A>
       where A: Asset,
             S: Settings;
}
```

An extension trait for methods for working with assets directly from a [`World`](../prelude/struct.World.html "struct bevy::prelude::World").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/direct_access_ext.rs.html#11)

#### fn [add\_asset](#tymethod.add_asset)<A>(&mut self, asset: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<A>) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Insert an asset similarly to [`Assets::add`](../prelude/struct.Assets.html#method.add "method bevy::prelude::Assets::add").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/direct_access_ext.rs.html#14)

#### fn [load\_asset](#tymethod.load_asset)<'a, A>(&self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Load an asset similarly to [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/direct_access_ext.rs.html#17)

#### fn [load\_builder](#tymethod.load_builder)(&self) -> [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'\_>

Creates a new [`LoadBuilder`](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder") similar to [`AssetServer::load_builder`](../prelude/struct.AssetServer.html#method.load_builder "method bevy::prelude::AssetServer::load_builder").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/direct_access_ext.rs.html#21-25)

#### fn [load\_asset\_with\_settings](#tymethod.load_asset_with_settings)<'a, A, S>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, settings: impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), S: [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings"),

👎Deprecated:

Use `world.load_builder().with_settings(settings).load(path)`

Load an asset with settings, similarly to [`AssetServer::load_with_settings`](../prelude/struct.AssetServer.html#method.load_with_settings "method bevy::prelude::AssetServer::load_with_settings").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/direct_access_ext.rs.html#28)

### impl [DirectAssetAccessExt](../prelude/trait.DirectAssetAccessExt.html "trait bevy::prelude::DirectAssetAccessExt") for [World](../prelude/struct.World.html "struct bevy::prelude::World")