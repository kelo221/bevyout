[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait ErasedAssetReader 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#242)

```rust
pub trait ErasedAssetReader:
    Send
    + Sync
    + 'static {
    // Required methods
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn Reader + 'a>, AssetReaderError>> + 'a>>;
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn Reader + 'a>, AssetReaderError>> + 'a>>;
    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn Stream<Item = PathBuf> + Send + Unpin>, AssetReaderError>> + 'a>>;
    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<bool, AssetReaderError>> + 'a>>;
    fn read_meta_bytes<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Vec<u8>, AssetReaderError>> + 'a>>;
}
```

Equivalent to an [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") but using boxed futures, necessary eg. when using a `dyn AssetReader`, as [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") isn’t currently object safe.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#244-247)

#### fn [read](#tymethod.read)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + 'a>, [AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")\>> + 'a>>

Returns a future to load the full file data at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#249-252)

#### fn [read\_meta](#tymethod.read_meta)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + 'a>, [AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")\>> + 'a>>

Returns a future to load the full file data at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#254-257)

#### fn [read\_directory](#tymethod.read_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Stream](../../tasks/futures_lite/trait.Stream.html "trait bevy::tasks::futures_lite::Stream")<Item = [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin")\>, [AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")\>> + 'a>>

Returns an iterator of directory entry names at the provided path.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#259-262)

#### fn [is\_directory](#tymethod.is_directory)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), [AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")\>> + 'a>>

Returns true if the provided path points to a directory.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#265-268)

#### fn [read\_meta\_bytes](#tymethod.read_meta_bytes)<'a>( &'a self, path: &'a [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path"), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, [AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")\>> + 'a>>

Reads asset metadata bytes at the given `path` into a [`Vec<u8>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec"). This is a convenience function that wraps [`ErasedAssetReader::read_meta`](trait.ErasedAssetReader.html#tymethod.read_meta "method bevy::asset::io::ErasedAssetReader::read_meta") by default.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#271)

### impl<T> [ErasedAssetReader](trait.ErasedAssetReader.html "trait bevy::asset::io::ErasedAssetReader") for T

where T: [AssetReader](trait.AssetReader.html "trait bevy::asset::io::AssetReader"),