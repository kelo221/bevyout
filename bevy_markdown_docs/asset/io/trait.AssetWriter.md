[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait AssetWriter 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#330)

```rust
pub trait AssetWriter:
    Send
    + Sync
    + 'static {
    // Required methods
    fn write<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn write_meta<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn remove<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn remove_meta<'a>(&'a self, path: &'a Path) -> impl ConditionalSendFuture;
    fn rename<'a>(
        &'a self,
        old_path: &'a Path,
        new_path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn rename_meta<'a>(
        &'a self,
        old_path: &'a Path,
        new_path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn create_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn remove_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn remove_empty_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture;
    fn remove_assets_in_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl ConditionalSendFuture;

    // Provided methods
    fn write_bytes<'a>(
        &'a self,
        path: &'a Path,
        bytes: &'a [u8],
    ) -> impl ConditionalSendFuture { ... }
    fn write_meta_bytes<'a>(
        &'a self,
        path: &'a Path,
        bytes: &'a [u8],
    ) -> impl ConditionalSendFuture { ... }
}
```

Performs write operations on an asset storage. [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given `path`. This trait is not object safe, if needed use a dyn [`ErasedAssetWriter`](trait.ErasedAssetWriter.html "trait bevy::asset::io::ErasedAssetWriter") instead.

This trait defines asset-agnostic mechanisms to write bytes to a storage system. For the per-asset-type saving/loading logic, see [`AssetSaver`](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") and [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader").

For a complementary version of this trait that can read assets from storage, see [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader").

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#332-335)

#### fn [write](#tymethod.write)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Writes the full asset bytes at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#338-341)

#### fn [write\_meta](#tymethod.write_meta)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Writes the full asset meta bytes at the provided path. This _should not_ include storage specific extensions like `.meta`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#343-346)

#### fn [remove](#tymethod.remove)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Removes the asset stored at the given path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#349-352)

#### fn [remove\_meta](#tymethod.remove_meta)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Removes the asset meta stored at the given path. This _should not_ include storage specific extensions like `.meta`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#354-358)

#### fn [rename](#tymethod.rename)<'a>( &'a self, old\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), new\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Renames the asset at `old_path` to `new_path`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#361-365)

#### fn [rename\_meta](#tymethod.rename_meta)<'a>( &'a self, old\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), new\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Renames the asset meta for the asset at `old_path` to `new_path`. This _should not_ include storage specific extensions like `.meta`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#368-371)

#### fn [create\_directory](#tymethod.create_directory)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Creates a directory at the given path, including all parent directories if they do not already exist.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#373-376)

#### fn [remove\_directory](#tymethod.remove_directory)<'a>(&'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Removes the directory at the given path, including all assets _and_ directories in that directory.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#379-382)

#### fn [remove\_empty\_directory](#tymethod.remove_empty_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Removes the directory at the given path, but only if it is completely empty. This will return an error if the directory is not empty.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#384-387)

#### fn [remove\_assets\_in\_directory](#tymethod.remove_assets_in_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Removes all assets (and directories) in this directory, resulting in an empty directory.

## Provided Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#389-393)

#### fn [write\_bytes](#method.write_bytes)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), bytes: &'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Writes the asset `bytes` to the given `path`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#402-406)

#### fn [write\_meta\_bytes](#method.write_meta_bytes)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), bytes: &'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Writes the asset meta `bytes` to the given `path`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/file/file_asset.rs.html#174)

### impl [AssetWriter](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") for [FileAssetWriter](file/struct.FileAssetWriter.html "struct bevy::asset::io::file::FileAssetWriter")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/memory.rs.html#459)

### impl [AssetWriter](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") for [MemoryAssetWriter](memory/struct.MemoryAssetWriter.html "struct bevy::asset::io::memory::MemoryAssetWriter")