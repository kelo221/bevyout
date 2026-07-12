[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait AssetReader 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#190)

```rust
pub trait AssetReader:
    Send
    + Sync
    + 'static {
    // Required methods
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl AssetReaderFuture + Reader + 'a;
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl AssetReaderFuture + Reader + 'a;
    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn is_directory<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;

    // Provided method
    fn read_meta_bytes<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture { ... }
}
```

Performs read operations on an asset storage. [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given `path`. This trait is not object safe, if needed use a dyn [`ErasedAssetReader`](trait.ErasedAssetReader.html "trait bevy::asset::io::ErasedAssetReader") instead.

This trait defines asset-agnostic mechanisms to read bytes from a storage system. For the per-asset-type saving/loading logic, see [`AssetSaver`](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") and [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader").

For a complementary version of this trait that can write assets to storage, see [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#212)

#### fn [read](#tymethod.read)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [AssetReaderFuture](trait.AssetReaderFuture.html "trait bevy::asset::io::AssetReaderFuture") + [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + 'a

Returns a future to load the full file data at the provided path.

##### Note for implementors

The preferred style for implementing this method is an `async fn` returning an opaque type.

```rust
impl AssetReader for MyReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<impl Reader + 'a, AssetReaderError> {
        // ...
    }
}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#214)

#### fn [read\_meta](#tymethod.read_meta)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> impl [AssetReaderFuture](trait.AssetReaderFuture.html "trait bevy::asset::io::AssetReaderFuture") + [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + 'a

Returns a future to load the full file data at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#216-219)

#### fn [read\_directory](#tymethod.read_directory)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Returns an iterator of directory entry names at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#221-224)

#### fn [is\_directory](#tymethod.is_directory)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Returns true if the provided path points to a directory.

## Provided Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#227-230)

#### fn [read\_meta\_bytes](#method.read_meta_bytes)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Reads asset metadata bytes at the given `path` into a [`Vec<u8>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec"). This is a convenience function that wraps [`AssetReader::read_meta`](trait.AssetReader.html#tymethod.read_meta "method bevy::asset::io::AssetReader::read_meta") by default.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/file/file_asset.rs.html#74)

### impl [AssetReader](trait.AssetReader.html "trait bevy::asset::io::AssetReader") for [FileAssetReader](file/struct.FileAssetReader.html "struct bevy::asset::io::file::FileAssetReader")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/memory.rs.html#374)

### impl [AssetReader](trait.AssetReader.html "trait bevy::asset::io::AssetReader") for [MemoryAssetReader](memory/struct.MemoryAssetReader.html "struct bevy::asset::io::memory::MemoryAssetReader")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/web.rs.html#193)

### impl [AssetReader](trait.AssetReader.html "trait bevy::asset::io::AssetReader") for [WebAssetReader](web/enum.WebAssetReader.html "enum bevy::asset::io::web::WebAssetReader")