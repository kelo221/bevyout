[bevy](../../index.html)::[asset](../index.html)

# Module io 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#153)

## Modules

[embedded](embedded/index.html "mod bevy::asset::io::embedded")

[file](file/index.html "mod bevy::asset::io::file")Non-WebAssembly

[memory](memory/index.html "mod bevy::asset::io::memory")

[processor\_gated](processor_gated/index.html "mod bevy::asset::io::processor_gated")

[web](web/index.html "mod bevy::asset::io::web")`http` or `https`

## Structs

[AssetSource](struct.AssetSource.html "struct bevy::asset::io::AssetSource")

A collection of unprocessed and processed [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader"), [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter"), and [`AssetWatcher`](trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher") instances for a specific asset source, identified by an [`AssetSourceId`](enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId").

[AssetSourceBuilder](struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder")

Metadata about an “asset source”, such as how to construct the [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") and [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") for the source, and whether or not the source is processed.

[AssetSourceBuilders](struct.AssetSourceBuilders.html "struct bevy::asset::io::AssetSourceBuilders")

A [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") that hold (repeatable) functions capable of producing new [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") and [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") instances for a given asset source.

[AssetSources](struct.AssetSources.html "struct bevy::asset::io::AssetSources")

A collection of [`AssetSource`](struct.AssetSource.html "struct bevy::asset::io::AssetSource")s.

[MissingAssetSourceError](struct.MissingAssetSourceError.html "struct bevy::asset::io::MissingAssetSourceError")

An error returned when an [`AssetSource`](struct.AssetSource.html "struct bevy::asset::io::AssetSource") does not exist for a given id.

[MissingAssetWriterError](struct.MissingAssetWriterError.html "struct bevy::asset::io::MissingAssetWriterError")

An error returned when an [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") does not exist for a given id.

[MissingProcessedAssetReaderError](struct.MissingProcessedAssetReaderError.html "struct bevy::asset::io::MissingProcessedAssetReaderError")

An error returned when a processed [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") does not exist for a given id.

[MissingProcessedAssetWriterError](struct.MissingProcessedAssetWriterError.html "struct bevy::asset::io::MissingProcessedAssetWriterError")

An error returned when a processed [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") does not exist for a given id.

[ReaderNotSeekableError](struct.ReaderNotSeekableError.html "struct bevy::asset::io::ReaderNotSeekableError")

Error returned by [`Reader::seekable`](trait.Reader.html#tymethod.seekable "method bevy::asset::io::Reader::seekable") when the reader implementation does not support [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") behavior.

[SliceReader](struct.SliceReader.html "struct bevy::asset::io::SliceReader")

An [`AsyncRead`](../../tasks/futures_lite/trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") implementation capable of reading a \[`&[u8]`\].

[StackFuture](struct.StackFuture.html "struct bevy::asset::io::StackFuture")

A wrapper that stores a future in space allocated by the container

[VecReader](struct.VecReader.html "struct bevy::asset::io::VecReader")

An [`AsyncRead`](../../tasks/futures_lite/trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") implementation capable of reading a [`Vec<u8>`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").

## Enums

[AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")

Errors that occur while loading assets.

[AssetSourceEvent](enum.AssetSourceEvent.html "enum bevy::asset::io::AssetSourceEvent")

An “asset source change event” that occurs whenever asset (or asset metadata) is created/added/removed

[AssetSourceId](enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")

A reference to an “asset source”, which maps to an [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") and/or [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter").

[AssetWriterError](enum.AssetWriterError.html "enum bevy::asset::io::AssetWriterError")

Errors that occur while loading assets.

## Constants

[STACK\_FUTURE\_SIZE](constant.STACK_FUTURE_SIZE.html "constant bevy::asset::io::STACK_FUTURE_SIZE")

The maximum size of a future returned from [`Reader::read_to_end`](trait.Reader.html#method.read_to_end "method bevy::asset::io::Reader::read_to_end"). This is large enough to fit ten references.

## Traits

[AssetReader](trait.AssetReader.html "trait bevy::asset::io::AssetReader")

Performs read operations on an asset storage. [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given `path`. This trait is not object safe, if needed use a dyn [`ErasedAssetReader`](trait.ErasedAssetReader.html "trait bevy::asset::io::ErasedAssetReader") instead.

[AssetReaderFuture](trait.AssetReaderFuture.html "trait bevy::asset::io::AssetReaderFuture")

A future that returns a value or an [`AssetReaderError`](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")

[AssetWatcher](trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher")

A handle to an “asset watcher” process, that will listen for and emit [`AssetSourceEvent`](enum.AssetSourceEvent.html "enum bevy::asset::io::AssetSourceEvent") values for as long as [`AssetWatcher`](trait.AssetWatcher.html "trait bevy::asset::io::AssetWatcher") has not been dropped.

[AssetWriter](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter")

Performs write operations on an asset storage. [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") exposes a “virtual filesystem” API, where asset bytes and asset metadata bytes are both stored and accessible for a given `path`. This trait is not object safe, if needed use a dyn [`ErasedAssetWriter`](trait.ErasedAssetWriter.html "trait bevy::asset::io::ErasedAssetWriter") instead.

[AsyncWriteExt](trait.AsyncWriteExt.html "trait bevy::asset::io::AsyncWriteExt")

Extension trait for [`AsyncWrite`](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite").

[ErasedAssetReader](trait.ErasedAssetReader.html "trait bevy::asset::io::ErasedAssetReader")

Equivalent to an [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") but using boxed futures, necessary eg. when using a `dyn AssetReader`, as [`AssetReader`](trait.AssetReader.html "trait bevy::asset::io::AssetReader") isn’t currently object safe.

[ErasedAssetWriter](trait.ErasedAssetWriter.html "trait bevy::asset::io::ErasedAssetWriter")

Equivalent to an [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") but using boxed futures, necessary eg. when using a `dyn AssetWriter`, as [`AssetWriter`](trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") isn’t currently object safe.

[Reader](trait.Reader.html "trait bevy::asset::io::Reader")

A type returned from [`AssetReader::read`](trait.AssetReader.html#tymethod.read "method bevy::asset::io::AssetReader::read"), which is used to read the contents of a file (or virtual file) corresponding to an asset.

[SeekableReader](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader")

A [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") that also has [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") functionality. See [`Reader::seekable`](trait.Reader.html#tymethod.seekable "method bevy::asset::io::Reader::seekable") for details.

## Type Aliases

[PathStream](type.PathStream.html "type bevy::asset::io::PathStream")

[Writer](type.Writer.html "type bevy::asset::io::Writer")