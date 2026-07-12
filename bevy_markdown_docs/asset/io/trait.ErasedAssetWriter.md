[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait ErasedAssetWriter 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#418)

```rust
pub trait ErasedAssetWriter:
    Send
    + Sync
    + 'static {
    // Required methods
    fn write<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn AsyncWrite + Send + Unpin + Sync>, AssetWriterError>> + 'a>>;
    fn write_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn AsyncWrite + Send + Unpin + Sync>, AssetWriterError>> + 'a>>;
    fn remove<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn remove_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn rename<'a>(
        &'a self,
        old_path: &'a Path,
        new_path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn rename_meta<'a>(
        &'a self,
        old_path: &'a Path,
        new_path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn create_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn remove_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn remove_empty_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn remove_assets_in_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn write_bytes<'a>(
        &'a self,
        path: &'a Path,
        bytes: &'a [u8],
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
    fn write_meta_bytes<'a>(
        &'a self,
        path: &'a Path,
        bytes: &'a [u8],
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), AssetWriterError>> + 'a>>;
}
```

Equivalent to an [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") but using boxed futures, necessary eg. when using a `dyn AssetWriter`, as [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") isn’t currently object safe.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#420-423)

#### fn [write](#tymethod.write)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>, [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Writes the full asset bytes at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#426-429)

#### fn [write\_meta](#tymethod.write_meta)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>, [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Writes the full asset meta bytes at the provided path. This _should not_ include storage specific extensions like `.meta`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#431)

#### fn [remove](#tymethod.remove)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Removes the asset stored at the given path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#434)

#### fn [remove\_meta](#tymethod.remove_meta)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Removes the asset meta stored at the given path. This _should not_ include storage specific extensions like `.meta`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#436-440)

#### fn [rename](#tymethod.rename)<'a>( &'a self, old\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), new\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Renames the asset at `old_path` to `new_path`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#443-447)

#### fn [rename\_meta](#tymethod.rename_meta)<'a>( &'a self, old\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), new\_path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Renames the asset meta for the asset at `old_path` to `new_path`. This _should not_ include storage specific extensions like `.meta`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#450-453)

#### fn [create\_directory](#tymethod.create_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Creates a directory at the given path, including all parent directories if they do not already exist.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#455-458)

#### fn [remove\_directory](#tymethod.remove_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Removes the directory at the given path, including all assets _and_ directories in that directory.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#461-464)

#### fn [remove\_empty\_directory](#tymethod.remove_empty_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Removes the directory at the given path, but only if it is completely empty. This will return an error if the directory is not empty.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#466-469)

#### fn [remove\_assets\_in\_directory](#tymethod.remove_assets_in_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Removes all assets (and directories) in this directory, resulting in an empty directory.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#471-475)

#### fn [write\_bytes](#tymethod.write_bytes)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), bytes: &'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Writes the asset `bytes` to the given `path`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#477-481)

#### fn [write\_meta\_bytes](#tymethod.write_meta_bytes)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), bytes: &'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")\>> + 'a>>

Writes the asset meta `bytes` to the given `path`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#484)

### impl<T> [ErasedAssetWriter](trait.ErasedAssetWriter.html "trait bevy::asset::io::ErasedAssetWriter") for T

where T: [AssetWriter](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter"),