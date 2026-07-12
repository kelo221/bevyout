[bevy](../../index.html)::[asset](../index.html)::[processor](index.html)

# Trait ProcessorTransactionLogFactory 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#29)

```rust
pub trait ProcessorTransactionLogFactory:
    Send
    + Sync
    + 'static {
    // Required methods
    fn read(
        &self,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Vec<LogEntry>, BevyError>> + '_>>;
    fn create_new_log(
        &self,
    ) -> Pin<Box<dyn ConditionalSendFuture<Output = Result<Box<dyn ProcessorTransactionLog>, BevyError>> + '_>>;
}
```

A factory of [`ProcessorTransactionLog`](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog") that handles the state before the log has been started.

This trait also assists in recovering from partial processing by fetching the previous state of the transaction log.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#33)

#### fn [read](#tymethod.read)( &self, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[LogEntry](enum.LogEntry.html "enum bevy::asset::processor::LogEntry")\>, [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + '\_>>

Reads all entries in a previous transaction log if present.

If there is no previous transaction log, this method should return an empty Vec of entries.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#38-40)

#### fn [create\_new\_log](#tymethod.create_new_log)( &self, ) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ProcessorTransactionLog](trait.ProcessorTransactionLog.html "trait bevy::asset::processor::ProcessorTransactionLog")\>, [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>> + '\_>>

Creates a new transaction log to write to.

This should remove any previous entries if they exist.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/log.rs.html#132)

### impl [ProcessorTransactionLogFactory](trait.ProcessorTransactionLogFactory.html "trait bevy::asset::processor::ProcessorTransactionLogFactory") for [FileTransactionLogFactory](struct.FileTransactionLogFactory.html "struct bevy::asset::processor::FileTransactionLogFactory")