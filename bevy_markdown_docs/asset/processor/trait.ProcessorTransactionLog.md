[bevy](../../index.html)::[asset](../index.html)::[processor](index.html)

# Trait ProcessorTransactionLog 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#48)

```rust
pub trait ProcessorTransactionLog:
    Send
    + Sync
    + 'static {
    // Required methods
    fn begin_processing<'a>(
        &'a mut self,
        asset: &'a AssetPath<'_>,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), BevyError>> + 'a>>;
    fn end_processing<'a>(
        &'a mut self,
        asset: &'a AssetPath<'_>,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), BevyError>> + 'a>>;
    fn unrecoverable(
        &mut self,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<(), BevyError>> + '_>>;
}
```

A “write ahead” logger that helps ensure asset importing is transactional.

Prior to processing an asset, we write to the log to indicate it has started. After processing an asset, we write to the log to indicate it has finished. On startup, the log can be read through [`ProcessorTransactionLogFactory`](trait.ProcessorTransactionLogFactory.html "trait bevy::asset::processor::ProcessorTransactionLogFactory") to determine if any transactions were incomplete.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#54-57)

#### fn [begin\_processing](#tymethod.begin_processing)<'a>( &'a mut self, asset: &'a [AssetPath](../struct.AssetPath.html "struct bevy::asset::AssetPath")<'\_>, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + 'a>>

Logs the start of an asset being processed.

If this is not followed at some point in the log by a closing [`ProcessorTransactionLog::end_processing`](trait.ProcessorTransactionLog.html#tymethod.end_processing "method bevy::asset::processor::ProcessorTransactionLog::end_processing"), in the next run of the processor the asset processing will be considered “incomplete” and it will be reprocessed.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#61-64)

#### fn [end\_processing](#tymethod.end_processing)<'a>( &'a mut self, asset: &'a [AssetPath](../struct.AssetPath.html "struct bevy::asset::AssetPath")<'\_>, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + 'a>>

Logs the end of an asset being successfully processed. See [`ProcessorTransactionLog::begin_processing`](trait.ProcessorTransactionLog.html#tymethod.begin_processing "method bevy::asset::processor::ProcessorTransactionLog::begin_processing").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#71)

#### fn [unrecoverable](#tymethod.unrecoverable)( &mut self, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + '\_>>

Logs an unrecoverable error.

On the next run of the processor, all assets will be regenerated. This should only be used as a last resort. Every call to this should be considered with scrutiny and ideally replaced with something more granular.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors