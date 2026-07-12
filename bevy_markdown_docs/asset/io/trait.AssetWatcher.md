[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait AssetWatcher 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#597)

```rust
pub trait AssetWatcher:
    Send
    + Sync
    + 'static { }
```

A handle to an “asset watcher” process, that will listen for and emit [`AssetSourceEvent`](enum.AssetSourceEvent.html "enum bevy::asset::io::AssetSourceEvent") values for as long as [`AssetWatcher`](trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher") has not been dropped.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/embedded_watcher.rs.html#47)

### impl [AssetWatcher](trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher") for [EmbeddedWatcher](embedded/struct.EmbeddedWatcher.html "struct bevy::asset::io::embedded::EmbeddedWatcher")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/file/file_watcher.rs.html#52)

### impl [AssetWatcher](trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher") for [FileWatcher](file/struct.FileWatcher.html "struct bevy::asset::io::file::FileWatcher")