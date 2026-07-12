[bevy](../../index.html)::[asset](../index.html)::[saver](index.html)

# Trait ErasedAssetSaver 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#53)

```rust
pub trait ErasedAssetSaver:
    Send
    + Sync
    + 'static {
    // Required methods
    fn save<'a>(
        &'a self,
        writer: &'a mut (dyn AsyncWrite + Send + Unpin + Sync + 'static),
        asset: &'a ErasedLoadedAsset,
        settings: &'a (dyn Settings + 'static),
        asset_path: AssetPath<'a>,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), BevyError>> + 'a>>;
    fn type_name(&self) -> &'static str;
}
```

A type-erased dynamic variant of [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") that allows callers to save assets without knowing the actual type of the [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#56-62)

#### fn [save](#tymethod.save)<'a>( &'a self, writer: &'a mut (dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static), asset: &'a [ErasedLoadedAsset](../struct.ErasedLoadedAsset.html "struct bevy::asset::ErasedLoadedAsset"), settings: &'a (dyn [Settings](../meta/trait.Settings.html "trait bevy::asset::meta::Settings") + 'static), asset\_path: [AssetPath](../struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + 'a>>

Saves the given runtime [`ErasedLoadedAsset`](../struct.ErasedLoadedAsset.html "struct bevy::asset::ErasedLoadedAsset") by writing it to a byte format using `writer`. The passed in `settings` can influence how the `asset` is saved.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#65)

#### fn [type\_name](#tymethod.type_name)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The type name of the [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#68)

### impl<S> [ErasedAssetSaver](trait.ErasedAssetSaver.html "trait bevy::asset::saver::ErasedAssetSaver") for S

where S: [AssetSaver](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver"),