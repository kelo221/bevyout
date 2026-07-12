[bevy](../../../index.html)::[asset](../../index.html)::[io](../index.html)

# Module file 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#12)

Available on **non-WebAssembly** only.

## Structs

[FileAssetReader](struct.FileAssetReader.html "struct bevy::asset::io::file::FileAssetReader")

I/O implementation for the local filesystem.

[FileAssetWriter](struct.FileAssetWriter.html "struct bevy::asset::io::file::FileAssetWriter")

A writer for the local filesystem.

[FileWatcher](struct.FileWatcher.html "struct bevy::asset::io::file::FileWatcher")

An [`AssetWatcher`](../trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher") that watches the filesystem for changes to asset files in a given root folder and emits [`AssetSourceEvent`](../enum.AssetSourceEvent.html "enum bevy::asset::io::AssetSourceEvent") for each relevant change.