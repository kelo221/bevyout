[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait AssetReaderFuture 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#169-170)

```rust
pub trait AssetReaderFuture: ConditionalSendFuture<Output = Result<Self::Value, AssetReaderError>> {
    type Value;
}
```

A future that returns a value or an [`AssetReaderError`](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")

## Required Associated Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#172)

#### type [Value](#associatedtype.Value)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#175-177)

### impl<F, T> [AssetReaderFuture](trait.AssetReaderFuture.html "trait bevy::asset::io::AssetReaderFuture") for F

where F: [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [AssetReaderError](enum.AssetReaderError.html "enum bevy::asset::io::AssetReaderError")\>>,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#179)

#### type [Value](#associatedtype.Value) = T