[bevy](../../index.html)::[asset](../index.html)

# Module processor 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#155)

Asset processing in Bevy is a framework for automatically transforming artist-authored assets into the format that best suits the needs of your particular game.

You can think of the asset processing system as a “build system” for assets. When an artist adds a new asset to the project or an asset is changed (assuming asset hot reloading is enabled), the asset processing system will automatically perform the specified processing steps on the asset. This can include things like creating lightmaps for baked lighting, compressing a `.wav` file to an `.ogg`, or generating mipmaps for a texture.

Its core values are:

1.  Automatic: new and changed assets should be ready to use in-game without requiring any manual conversion or cleanup steps.
2.  Configurable: every game has its own needs, and a high level of transparency and control is required.
3.  Lossless: the original asset should always be preserved, ensuring artists can make changes later.
4.  Deterministic: performing the same processing steps on the same asset should (generally) produce the exact same result. In cases where this doesn’t make sense (steps that involve a degree of randomness or uncertainty), the results across runs should be “acceptably similar”, as they will be generated once for a given set of inputs and cached.

Taken together, this means that the original asset plus the processing steps should be enough to regenerate the final asset. While it may be possible to manually edit the final asset, this should be discouraged. Final post-processed assets should generally not be version-controlled, except to save developer time when recomputing heavy asset processing steps.

## Usage

Asset processing can be enabled or disabled in [`AssetPlugin`](../../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin") by setting the [`AssetMode`](../../prelude/enum.AssetMode.html "enum bevy::prelude::AssetMode").  
Enable Bevy’s `file_watcher` feature to automatically watch for changes to assets and reprocess them.

To register a new asset processor, use [`AssetProcessor::register_processor`](struct.AssetProcessor.html#method.register_processor "method bevy::asset::processor::AssetProcessor::register_processor"). To set the default asset processor for a given extension, use [`AssetProcessor::set_default_processor`](struct.AssetProcessor.html#method.set_default_processor "method bevy::asset::processor::AssetProcessor::set_default_processor"). In most cases, these methods will be called directly on [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") using the [`AssetApp`](../../prelude/trait.AssetApp.html "trait bevy::prelude::AssetApp") extension trait.

If a default asset processor is set, assets with a matching extension will be processed using that processor before loading.

For an end-to-end example, check out the examples in the [`examples/asset/processing`](https://github.com/bevyengine/bevy/tree/latest/examples/asset/processing) directory of the Bevy repository.

## Defining asset processors

Bevy provides two different ways to define new asset processors:

*   [`LoadTransformAndSave`](struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave") + [`AssetTransformer`](../transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer"): a high-level API for loading, transforming, and saving assets.
*   [`Process`](trait.Process.html "trait bevy::asset::processor::Process"): a flexible low-level API for processing assets in arbitrary ways.

In most cases, [`LoadTransformAndSave`](struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave") should be sufficient.

## Structs

[AssetProcessor](struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor")

A “background” asset processor that reads asset values from a source [`AssetSource`](../io/struct.AssetSource.html "struct bevy::asset::io::AssetSource") (which corresponds to an [`AssetReader`](../io/trait.AssetReader.html "trait bevy::asset::io::AssetReader") / [`AssetWriter`](../io/trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") pair), processes them in some way, and writes them to a destination [`AssetSource`](../io/struct.AssetSource.html "struct bevy::asset::io::AssetSource").

[AssetProcessorData](struct.AssetProcessorData.html "struct bevy::asset::processor::AssetProcessorData")

Internal data stored inside an [`AssetProcessor`](struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor").

[FileTransactionLogFactory](struct.FileTransactionLogFactory.html "struct bevy::asset::processor::FileTransactionLogFactory")

A transaction log factory that uses a file as its storage.

[LoadTransformAndSave](struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave")

A flexible [`Process`](trait.Process.html "trait bevy::asset::processor::Process") implementation that loads the source [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") using the `L` [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"), then transforms the `L` asset into an `S` [`AssetSaver`](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") asset using the `T` [`AssetTransformer`](../transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer"), and lastly saves the asset using the `S` [`AssetSaver`](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

[LoadTransformAndSaveSettings](struct.LoadTransformAndSaveSettings.html "struct bevy::asset::processor::LoadTransformAndSaveSettings")

Settings for the [`LoadTransformAndSave`](struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave") [`Process::Settings`](trait.Process.html#associatedtype.Settings "associated type bevy::asset::processor::Process::Settings") implementation.

[ProcessContext](struct.ProcessContext.html "struct bevy::asset::processor::ProcessContext")

Provides scoped data access to the [`AssetProcessor`](struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor"). This must only expose processor data that is represented in the asset’s hash.

[ProcessorAssetInfos](struct.ProcessorAssetInfos.html "struct bevy::asset::processor::ProcessorAssetInfos")

The “current” in memory view of the asset space. This is “eventually consistent”. It does not directly represent the state of assets in storage, but rather a valid historical view that will gradually become more consistent as events are processed.

## Enums

[GetProcessorError](enum.GetProcessorError.html "enum bevy::asset::processor::GetProcessorError")

An error when retrieving an asset processor.

[InitializeError](enum.InitializeError.html "enum bevy::asset::processor::InitializeError")

An error that occurs when initializing the [`AssetProcessor`](struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor").

[LogEntry](enum.LogEntry.html "enum bevy::asset::processor::LogEntry")

An in-memory representation of a single [`ProcessorTransactionLog`](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog") entry.

[LogEntryError](enum.LogEntryError.html "enum bevy::asset::processor::LogEntryError")

An error that occurs when validating individual [`ProcessorTransactionLog`](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog") entries.

[MetaTypePathKind](enum.MetaTypePathKind.html "enum bevy::asset::processor::MetaTypePathKind")

Specifies which kind of path to use to specify a type.

[ProcessError](enum.ProcessError.html "enum bevy::asset::processor::ProcessError")

An error that is encountered during [`Process::process`](trait.Process.html#tymethod.process "method bevy::asset::processor::Process::process").

[ProcessResult](enum.ProcessResult.html "enum bevy::asset::processor::ProcessResult")

The (successful) result of processing an asset

[ProcessStatus](enum.ProcessStatus.html "enum bevy::asset::processor::ProcessStatus")

The final status of processing an asset

[ProcessorState](enum.ProcessorState.html "enum bevy::asset::processor::ProcessorState")

The current state of the [`AssetProcessor`](struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor").

[ReadLogError](enum.ReadLogError.html "enum bevy::asset::processor::ReadLogError")

An error that occurs when reading from the [`ProcessorTransactionLog`](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog") fails.

[SetTransactionLogFactoryError](enum.SetTransactionLogFactoryError.html "enum bevy::asset::processor::SetTransactionLogFactoryError")

An error when attempting to set the transaction log factory.

[ValidateLogError](enum.ValidateLogError.html "enum bevy::asset::processor::ValidateLogError")

An error that occurs when validating the [`ProcessorTransactionLog`](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog") fails.

## Traits

[ErasedProcessor](trait.ErasedProcessor.html "trait bevy::asset::processor::ErasedProcessor")

A type-erased variant of [`Process`](trait.Process.html "trait bevy::asset::processor::Process") that enables interacting with processor implementations without knowing their type.

[Process](trait.Process.html "trait bevy::asset::processor::Process")

Asset “processor” logic that reads input asset bytes (stored on [`ProcessContext`](struct.ProcessContext.html "struct bevy::asset::processor::ProcessContext")), processes the value in some way, and then writes the final processed bytes with [`Writer`](../io/type.Writer.html "type bevy::asset::io::Writer"). The resulting bytes must be loadable with the given [`Process::OutputLoader`](trait.Process.html#associatedtype.OutputLoader "associated type bevy::asset::processor::Process::OutputLoader").

[ProcessorTransactionLog](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog")

A “write ahead” logger that helps ensure asset importing is transactional.

[ProcessorTransactionLogFactory](trait.ProcessorTransactionLogFactory.html "trait bevy::asset::processor::ProcessorTransactionLogFactory")

A factory of [`ProcessorTransactionLog`](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog") that handles the state before the log has been started.