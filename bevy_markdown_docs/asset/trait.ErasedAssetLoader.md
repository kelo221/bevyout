[bevy](../index.html)::[asset](index.html)

# Trait ErasedAssetLoader 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#55)

```rust
pub trait ErasedAssetLoader:
    Send
    + Sync
    + 'static {
    // Required methods
    fn load<'a>(
        &'a self,
        reader: &'a mut dyn Reader,
        settings: &'a (dyn Settings + 'static),
        load_context: LoadContext<'a>,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<ErasedLoadedAsset, BevyError>> + 'a>>;
    fn extensions(&self) -> &[&str];
    fn deserialize_meta(
        &self,
        meta: &[u8],
    ) -> Result<Box<dyn AssetMetaDyn>, DeserializeMetaError>;
    fn default_meta(&self) -> Box<dyn AssetMetaDyn>;
    fn type_path(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    fn asset_type_name(&self) -> &'static str;
    fn asset_type_id(&self) -> TypeId;
}
```

Provides type-erased access to an [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#57-62)

#### fn [load](#tymethod.load)<'a>( &'a self, reader: &'a mut dyn [Reader](io/trait.Reader.html "trait bevy::asset::io::Reader"), settings: &'a (dyn [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings") + 'static), load\_context: [LoadContext](struct.LoadContext.html "struct bevy::asset::LoadContext")<'a>, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ErasedLoadedAsset](struct.ErasedLoadedAsset.html "struct bevy::asset::ErasedLoadedAsset"), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + 'a>>

Asynchronously loads the asset(s) from the bytes provided by [`Reader`](io/trait.Reader.html "trait bevy::asset::io::Reader").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#65)

#### fn [extensions](#tymethod.extensions)(&self) -> &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\]

Returns a list of extensions supported by this asset loader, without the preceding dot.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#67)

#### fn [deserialize\_meta](#tymethod.deserialize_meta)( &self, meta: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](meta/trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>, [DeserializeMetaError](enum.DeserializeMetaError.html "enum bevy::asset::DeserializeMetaError")\>

Deserializes metadata from the input `meta` bytes into the appropriate type (erased as [`Box<dyn AssetMetaDyn>`](../prelude/struct.Box.html "struct bevy::prelude::Box")).

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#69)

#### fn [default\_meta](#tymethod.default_meta)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](meta/trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>

Returns the default meta value for the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") (erased as [`Box<dyn AssetMetaDyn>`](../prelude/struct.Box.html "struct bevy::prelude::Box")).

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#71)

#### fn [type\_path](#tymethod.type_path)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the type path of the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#73)

#### fn [type\_id](#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Returns the [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") of the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#75)

#### fn [asset\_type\_name](#tymethod.asset_type_name)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the type name of the top-level [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") loaded by the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#77)

#### fn [asset\_type\_id](#tymethod.asset_type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Returns the [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") of the top-level [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") loaded by the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#80-82)

### impl<L> [ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader") for L

where L: [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),