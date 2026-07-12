[bevy](../../index.html)::[asset](../index.html)::[processor](index.html)

# Trait ErasedProcessor 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#220)

```rust
pub trait ErasedProcessor: Send + Sync {
    // Required methods
    fn process<'a>(
        &'a self,
        context: &'a mut ProcessContext<'_>,
        settings: &'a (dyn Settings + 'static),
        writer: &'a mut (dyn AsyncWrite + Send + Unpin + Sync + 'static),
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn AssetMetaDyn>, ProcessError>> + 'a>>;
    fn deserialize_meta(
        &self,
        meta: &[u8],
    ) -> Result<Box<dyn AssetMetaDyn>, DeserializeMetaError>;
    fn type_path(&self) -> &'static str;
    fn short_type_path(&self) -> &'static str;
    fn default_meta(
        &self,
        processor_path_kind: MetaTypePathKind,
    ) -> Box<dyn AssetMetaDyn>;
}
```

A type-erased variant of [`Process`](trait.Process.html "trait bevy::asset::processor::Process") that enables interacting with processor implementations without knowing their type.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#222-227)

#### fn [process](#tymethod.process)<'a>( &'a self, context: &'a mut [ProcessContext](struct.ProcessContext.html "struct bevy::asset::processor::ProcessContext")<'\_>, settings: &'a (dyn [Settings](../meta/trait.Settings.html "trait bevy::asset::meta::Settings") + 'static), writer: &'a mut (dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static), ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](../meta/trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>, [ProcessError](enum.ProcessError.html "enum bevy::asset::processor::ProcessError")\>> + 'a>>

Type-erased variant of [`Process::process`](trait.Process.html#tymethod.process "method bevy::asset::processor::Process::process").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#230)

#### fn [deserialize\_meta](#tymethod.deserialize_meta)( &self, meta: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](../meta/trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>, [DeserializeMetaError](../enum.DeserializeMetaError.html "enum bevy::asset::DeserializeMetaError")\>

Deserialized `meta` as type-erased [`AssetMeta`](../meta/struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta"), operating under the assumption that it matches the meta for the underlying [`Process`](trait.Process.html "trait bevy::asset::processor::Process") impl.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#232)

#### fn [type\_path](#tymethod.type_path)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the type-path of the original [`Process`](trait.Process.html "trait bevy::asset::processor::Process").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#234)

#### fn [short\_type\_path](#tymethod.short_type_path)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the short type path of this processor.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#236)

#### fn [default\_meta](#tymethod.default_meta)( &self, processor\_path\_kind: [MetaTypePathKind](enum.MetaTypePathKind.html "enum bevy::asset::processor::MetaTypePathKind"), ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AssetMetaDyn](../meta/trait.AssetMetaDyn.html "trait bevy::asset::meta::AssetMetaDyn")\>

Returns the default type-erased [`AssetMeta`](../meta/struct.AssetMeta.html "struct bevy::asset::meta::AssetMeta") for the underlying [`Process`](trait.Process.html "trait bevy::asset::processor::Process") impl.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/process.rs.html#247)

### impl<P> [ErasedProcessor](trait.ErasedProcessor.html "trait bevy::asset::processor::ErasedProcessor") for P

where P: [Process](trait.Process.html "trait bevy::asset::processor::Process"),