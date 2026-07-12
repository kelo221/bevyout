[bevy](../../index.html)::[asset](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#162)

The asset prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AssetChanged](struct.AssetChanged.html "struct bevy::asset::prelude::AssetChanged")

Filter that selects entities with an `A` for an asset that changed after the system last ran, where `A` is a component that implements [`AsAssetId`](../trait.AsAssetId.html "trait bevy::asset::AsAssetId").

[AssetPlugin](struct.AssetPlugin.html "struct bevy::asset::prelude::AssetPlugin")

Provides “asset” loading and processing functionality. An [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") is a “runtime value” that is loaded from an [`AssetSource`](../io/struct.AssetSource.html "struct bevy::asset::io::AssetSource"), which can be something like a filesystem, a network, etc.

[AssetServer](struct.AssetServer.html "struct bevy::asset::prelude::AssetServer")

Loads and tracks the state of [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") values from a configured [`AssetReader`](../io/trait.AssetReader.html "trait bevy::asset::io::AssetReader"). This can be used to kick off new asset loads and retrieve their current load states.

[Assets](struct.Assets.html "struct bevy::asset::prelude::Assets")

Stores [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") values identified by their [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

## Enums

[AssetEvent](enum.AssetEvent.html "enum bevy::asset::prelude::AssetEvent")

[`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s that occur for a specific loaded [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"), such as “value changed” events and “dependency” events.

[AssetId](enum.AssetId.html "enum bevy::asset::prelude::AssetId")

A unique runtime-only identifier for an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"). This is cheap to [`Copy`](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy")/[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") and is not directly tied to the lifetime of the Asset. This means it _can_ point to an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") that no longer exists.

[AssetMode](enum.AssetMode.html "enum bevy::asset::prelude::AssetMode")

Controls whether or not assets are pre-processed before being loaded.

[Handle](enum.Handle.html "enum bevy::asset::prelude::Handle")

A handle to a specific [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A`. Handles act as abstract “references” to assets, whose data are stored in the [`Assets<A>`](../../prelude/struct.Assets.html "struct bevy::prelude::Assets") resource, avoiding the need to store multiple copies of the same data.

[UntypedHandle](enum.UntypedHandle.html "enum bevy::asset::prelude::UntypedHandle")

An untyped variant of [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle"), which internally stores the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") type information at runtime as a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") instead of encoding it in the compile-time type. This allows handles across [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") types to be stored together and compared.

## Traits

[Asset](trait.Asset.html "trait bevy::asset::prelude::Asset")

Declares that this type is an asset, which can be loaded and managed by the [`AssetServer`](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") and stored in [`Assets`](../../prelude/struct.Assets.html "struct bevy::prelude::Assets") collections.

[AssetApp](trait.AssetApp.html "trait bevy::asset::prelude::AssetApp")

Adds asset-related builder methods to [`App`](../../prelude/struct.App.html "struct bevy::prelude::App").

[DirectAssetAccessExt](trait.DirectAssetAccessExt.html "trait bevy::asset::prelude::DirectAssetAccessExt")

An extension trait for methods for working with assets directly from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Functions

[asset\_value](fn.asset_value.html "fn bevy::asset::prelude::asset_value")

This will create a new [`HandleTemplate`](../enum.HandleTemplate.html "enum bevy::asset::HandleTemplate") for the given `asset` value. This makes it possible to define assets “inline” in templates / scenes that produce a [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle").

## Derive Macros

[Asset](derive.Asset.html "derive bevy::asset::prelude::Asset")

Implement the `Asset` trait.