[bevy](../../index.html)::[asset](../index.html)

# Module meta 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#154)

## Structs

[AssetMeta](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta")

Asset metadata that informs how an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") should be handled by the asset system.

[AssetMetaMinimal](struct.AssetMetaMinimal.html "struct bevy::asset::meta::AssetMetaMinimal")

This is a minimal counterpart to [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") that exists to speed up (or enable) serialization in cases where the whole [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") isn’t necessary.

[ProcessDependencyInfo](struct.ProcessDependencyInfo.html "struct bevy::asset::meta::ProcessDependencyInfo")

Information about a dependency used to process an asset. This is used to determine whether an asset’s “process dependency” has changed.

[ProcessedInfo](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo")

Info produced by the [`AssetProcessor`](../processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor") for a given processed asset. This is used to determine if an asset source file (or its dependencies) has changed.

[ProcessedInfoMinimal](struct.ProcessedInfoMinimal.html "struct bevy::asset::meta::ProcessedInfoMinimal")

This is a minimal counterpart to [`ProcessedInfo`](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo") that exists to speed up serialization in cases where the whole [`ProcessedInfo`](struct.ProcessedInfo.html "struct bevy::asset::meta::ProcessedInfo") isn’t necessary.

## Enums

[AssetAction](enum.AssetAction.html "enum bevy::asset::meta::AssetAction")

Configures how an asset source file should be handled by the asset system.

[AssetActionMinimal](enum.AssetActionMinimal.html "enum bevy::asset::meta::AssetActionMinimal")

This is a minimal counterpart to [`AssetAction`](enum.AssetAction.html "enum bevy::asset::meta::AssetAction") that exists to speed up (or enable) serialization in cases where the whole [`AssetAction`](enum.AssetAction.html "enum bevy::asset::meta::AssetAction") isn’t necessary.

## Constants

[META\_FORMAT\_VERSION](constant.META_FORMAT_VERSION.html "constant bevy::asset::meta::META_FORMAT_VERSION")

## Traits

[AssetMetaDyn](trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")

A dynamic type-erased counterpart to [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") that enables passing around and interacting with [`AssetMeta`](struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") without knowing its type.

[Settings](trait.Settings.html "trait bevy::asset::meta::Settings")

Settings used by the asset system, such as by [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"), [`Process`](../processor/trait.Process.html "trait bevy::asset::processor::Process"), and [`AssetSaver`](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")

## Type Aliases

[AssetHash](type.AssetHash.html "type bevy::asset::meta::AssetHash")

[MetaTransform](type.MetaTransform.html "type bevy::asset::meta::MetaTransform")