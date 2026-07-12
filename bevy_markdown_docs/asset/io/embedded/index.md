[bevy](../../../index.html)::[asset](../../index.html)::[io](../index.html)

# Module embedded 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#10)

## Structs

[EmbeddedAssetRegistry](struct.EmbeddedAssetRegistry.html "struct bevy::asset::io::embedded::EmbeddedAssetRegistry")

A [`Resource`](../../../prelude/trait.Resource.html "trait bevy::prelude::Resource") that manages “rust source files” in a virtual in memory [`Dir`](../memory/struct.Dir.html "struct bevy::asset::io::memory::Dir"), which is intended to be shared with a [`MemoryAssetReader`](../memory/struct.MemoryAssetReader.html "struct bevy::asset::io::memory::MemoryAssetReader"). Generally this should not be interacted with directly. The [`embedded_asset`](../../macro.embedded_asset.html "macro bevy::asset::embedded_asset") will populate this.

[EmbeddedWatcher](struct.EmbeddedWatcher.html "struct bevy::asset::io::embedded::EmbeddedWatcher")

A watcher for assets stored in the `embedded` asset source. Embedded assets are assets whose bytes have been embedded into the Rust binary using the [`embedded_asset`](../../macro.embedded_asset.html "macro bevy::asset::embedded_asset") macro. This watcher will watch for changes to the “source files”, read the contents of changed files from the file system and overwrite the initial static bytes of the file embedded in the binary with the new dynamically loaded bytes.

## Constants

[EMBEDDED](constant.EMBEDDED.html "constant bevy::asset::io::embedded::EMBEDDED")

The name of the `embedded` [`AssetSource`](../struct.AssetSource.html "struct bevy::asset::io::AssetSource"), as stored in the [`AssetSourceBuilders`](../struct.AssetSourceBuilders.html "struct bevy::asset::io::AssetSourceBuilders") resource.

## Traits

[GetAssetServer](trait.GetAssetServer.html "trait bevy::asset::io::embedded::GetAssetServer")

Trait for the [`load_embedded_asset!`](../../macro.load_embedded_asset.html "macro bevy::asset::load_embedded_asset") macro, to access [`AssetServer`](../../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") from arbitrary things.