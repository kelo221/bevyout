[bevy](../../../index.html)::[asset](../../index.html)::[io](../index.html)

# Module memory 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#13)

## Structs

[Data](struct.Data.html "struct bevy::asset::io::memory::Data")

Asset data stored in a [`Dir`](struct.Dir.html "struct bevy::asset::io::memory::Dir").

[Dir](struct.Dir.html "struct bevy::asset::io::memory::Dir")

A clone-able (internally Arc-ed) / thread-safe “in memory” filesystem. This is built for [`MemoryAssetReader`](struct.MemoryAssetReader.html "struct bevy::asset::io::memory::MemoryAssetReader") and is primarily intended for unit tests.

[DirStream](struct.DirStream.html "struct bevy::asset::io::memory::DirStream")

[MemoryAssetReader](struct.MemoryAssetReader.html "struct bevy::asset::io::memory::MemoryAssetReader")

In-memory [`AssetReader`](../trait.AssetReader.html "trait bevy::asset::io::AssetReader") implementation. This is primarily intended for unit tests.

[MemoryAssetWriter](struct.MemoryAssetWriter.html "struct bevy::asset::io::memory::MemoryAssetWriter")

In-memory [`AssetWriter`](../trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") implementation.

## Enums

[Value](enum.Value.html "enum bevy::asset::io::memory::Value")

Stores either an allocated vec of bytes or a static array of bytes.