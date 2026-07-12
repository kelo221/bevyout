[bevy](../../index.html)::[asset](../index.html)

# Module saver 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#156)

## Structs

[ErasedSavedAsset](struct.ErasedSavedAsset.html "struct bevy::asset::saver::ErasedSavedAsset")

[SavedAsset](struct.SavedAsset.html "struct bevy::asset::saver::SavedAsset")

An [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") (and any labeled “sub assets”) intended to be saved.

[SavedAssetBuilder](struct.SavedAssetBuilder.html "struct bevy::asset::saver::SavedAssetBuilder")

A builder for creating [`SavedAsset`](struct.SavedAsset.html "struct bevy::asset::saver::SavedAsset") instances (for use with asset saving).

## Enums

[SaveAssetError](enum.SaveAssetError.html "enum bevy::asset::saver::SaveAssetError")

An error occurring when saving an asset.

## Traits

[AssetSaver](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")

Saves an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of a given [`AssetSaver::Asset`](trait.AssetSaver.html#associatedtype.Asset "associated type bevy::asset::saver::AssetSaver::Asset") type. [`AssetSaver::OutputLoader`](trait.AssetSaver.html#associatedtype.OutputLoader "associated type bevy::asset::saver::AssetSaver::OutputLoader") will then be used to load the saved asset in the final deployed application. The saver should produce asset bytes in a format that [`AssetSaver::OutputLoader`](trait.AssetSaver.html#associatedtype.OutputLoader "associated type bevy::asset::saver::AssetSaver::OutputLoader") can read.

[ErasedAssetSaver](trait.ErasedAssetSaver.html "trait bevy::asset::saver::ErasedAssetSaver")

A type-erased dynamic variant of [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") that allows callers to save assets without knowing the actual type of the [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

## Functions

[save\_using\_saver](fn.save_using_saver.html "fn bevy::asset::saver::save_using_saver")

Saves `asset` to `path` using the provided `saver` and `settings`.