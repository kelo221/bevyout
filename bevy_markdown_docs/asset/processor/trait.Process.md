[bevy](../../index.html)::[asset](../index.html)::[processor](index.html)

# Trait Process 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#31)

```rust
pub trait Process:
    Sized
    + TypePath
    + Send
    + Sync
    + 'static {
    type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>;
    type OutputLoader: AssetLoader;

    // Required method
    fn process(
        &self,
        context: &mut ProcessContext<'_>,
        settings: &Self::Settings,
        writer: &mut (dyn AsyncWrite + Send + Unpin + Sync + 'static),
    ) -> impl ConditionalSendFuture;
}
```

Asset “processor” logic that reads input asset bytes (stored on [`ProcessContext`](struct.ProcessContext.html "struct bevy::asset::processor::ProcessContext")), processes the value in some way, and then writes the final processed bytes with [`Writer`](../io/type.Writer.html "type bevy::asset::io::Writer"). The resulting bytes must be loadable with the given [`Process::OutputLoader`](trait.Process.html#associatedtype.OutputLoader "associated type bevy::asset::processor::Process::OutputLoader").

This is a “low level”, maximally flexible interface. Most use cases are better served by the [`LoadTransformAndSave`](struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave") implementation of [`Process`](trait.Process.html "trait bevy::asset::processor::Process").

## Required Associated Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#33)

#### type [Settings](#associatedtype.Settings): [Settings](../meta/trait.Settings.html "trait bevy::asset::meta::Settings") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + for<'a> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'a>

The configuration / settings used to process the asset. This will be stored in the [`AssetMeta`](../meta/struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") and is user-configurable per-asset.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#35)

#### type [OutputLoader](#associatedtype.OutputLoader): [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader")

The [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") that will be used to load the final processed asset.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#38-45)

#### fn [process](#tymethod.process)( &self, context: &mut [ProcessContext](struct.ProcessContext.html "struct bevy::asset::processor::ProcessContext")<'\_>, settings: &Self::[Settings](trait.Process.html#associatedtype.Settings "type bevy::asset::processor::Process::Settings"), writer: &mut (dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static), ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Processes the asset stored on `context` in some way using the settings stored on `meta`. The results are written to `writer`. The final written processed asset is loadable using [`Process::OutputLoader`](trait.Process.html#associatedtype.OutputLoader "associated type bevy::asset::processor::Process::OutputLoader"). This load will use the returned [`AssetLoader::Settings`](../trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#195)

### impl [Process](trait.Process.html "trait bevy::asset::processor::Process") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

The () processor should never be called. This implementation exists to make the meta format nicer to work with.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#196)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#197)

#### type [OutputLoader](#associatedtype.OutputLoader) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#199-204)

#### async fn [process](#tymethod.process)( &self, \_context: &mut [ProcessContext](struct.ProcessContext.html "struct bevy::asset::processor::ProcessContext")<'\_>, \_settings: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [Process](trait.Process.html "trait bevy::asset::processor::Process")\>::[Settings](trait.Process.html#associatedtype.Settings "type bevy::asset::processor::Process::Settings"), \_writer: &mut (dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ProcessError](enum.ProcessError.html "enum bevy::asset::processor::ProcessError")\>

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#172-176)

### impl<Loader, Transformer, Saver> [Process](trait.Process.html "trait bevy::asset::processor::Process") for [LoadTransformAndSave](struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave")<Loader, Transformer, Saver>

where Loader: [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader"), Transformer: [AssetTransformer](../transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer")<AssetInput = <Loader as [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader")\>::[Asset](../trait.AssetLoader.html#associatedtype.Asset "type bevy::asset::AssetLoader::Asset")\>, Saver: [AssetSaver](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")<Asset = <Transformer as [AssetTransformer](../transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer")\>::[AssetOutput](../transformer/trait.AssetTransformer.html#associatedtype.AssetOutput "type bevy::asset::transformer::AssetTransformer::AssetOutput")\>,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#178)

#### type [Settings](#associatedtype.Settings) = [LoadTransformAndSaveSettings](struct.LoadTransformAndSaveSettings.html "struct bevy::asset::processor::LoadTransformAndSaveSettings")<<Loader as [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader")\>::[Settings](../trait.AssetLoader.html#associatedtype.Settings "type bevy::asset::AssetLoader::Settings"), <Transformer as [AssetTransformer](../transformer/trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer")\>::[Settings](../transformer/trait.AssetTransformer.html#associatedtype.Settings "type bevy::asset::transformer::AssetTransformer::Settings"), <Saver as [AssetSaver](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")\>::[Settings](../saver/trait.AssetSaver.html#associatedtype.Settings "type bevy::asset::saver::AssetSaver::Settings")\>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#180)

#### type [OutputLoader](#associatedtype.OutputLoader) = <Saver as [AssetSaver](../saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver")\>::[OutputLoader](../saver/trait.AssetSaver.html#associatedtype.OutputLoader "type bevy::asset::saver::AssetSaver::OutputLoader")