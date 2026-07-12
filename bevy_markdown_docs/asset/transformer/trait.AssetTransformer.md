[bevy](../../index.html)::[asset](../index.html)::[transformer](index.html)

# Trait AssetTransformer 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#22)

```rust
pub trait AssetTransformer:
    TypePath
    + Send
    + Sync
    + 'static {
    type AssetInput: Asset;
    type AssetOutput: Asset;
    type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>;
    type Error: Into<Box<dyn Error + Send + Sync>>;

    // Required method
    fn transform<'a>(
        &'a self,
        asset: TransformedAsset<Self::AssetInput>,
        settings: &'a Self::Settings,
    ) -> impl ConditionalSendFuture;
}
```

Transforms an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of a given [`AssetTransformer::AssetInput`](trait.AssetTransformer.html#associatedtype.AssetInput "associated type bevy::asset::transformer::AssetTransformer::AssetInput") type to an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of [`AssetTransformer::AssetOutput`](trait.AssetTransformer.html#associatedtype.AssetOutput "associated type bevy::asset::transformer::AssetTransformer::AssetOutput") type.

This trait is commonly used in association with [`LoadTransformAndSave`](../processor/struct.LoadTransformAndSave.html "struct bevy::asset::processor::LoadTransformAndSave") to accomplish common asset pipeline workflows.

## Required Associated Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#24)

#### type [AssetInput](#associatedtype.AssetInput): [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset")

The [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") type which this [`AssetTransformer`](trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer") takes as and input.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#26)

#### type [AssetOutput](#associatedtype.AssetOutput): [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset")

The [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") type which this [`AssetTransformer`](trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer") outputs.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#28)

#### type [Settings](#associatedtype.Settings): [Settings](../meta/trait.Settings.html "trait bevy::asset::meta::Settings") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + for<'a> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'a>

The settings type used by this [`AssetTransformer`](trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#30)

#### type [Error](#associatedtype.Error): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>>

The type of [error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") which could be encountered by this transformer.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#35-39)

#### fn [transform](#tymethod.transform)<'a>( &'a self, asset: [TransformedAsset](struct.TransformedAsset.html "struct bevy::asset::transformer::TransformedAsset")<Self::[AssetInput](trait.AssetTransformer.html#associatedtype.AssetInput "type bevy::asset::transformer::AssetTransformer::AssetInput")\>, settings: &'a Self::[Settings](trait.AssetTransformer.html#associatedtype.Settings "type bevy::asset::transformer::AssetTransformer::Settings"), ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Transforms the given [`TransformedAsset`](struct.TransformedAsset.html "struct bevy::asset::transformer::TransformedAsset") to [`AssetTransformer::AssetOutput`](trait.AssetTransformer.html#associatedtype.AssetOutput "associated type bevy::asset::transformer::AssetTransformer::AssetOutput"). The [`TransformedAsset`](struct.TransformedAsset.html "struct bevy::asset::transformer::TransformedAsset")’s `labeled_assets` can be altered to add new Labeled Sub-Assets The passed in `settings` can influence how the `asset` is transformed

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#423)

### impl<A> [AssetTransformer](trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer") for [IdentityAssetTransformer](struct.IdentityAssetTransformer.html "struct bevy::asset::transformer::IdentityAssetTransformer")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#424)

#### type [AssetInput](#associatedtype.AssetInput) = A

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#425)

#### type [AssetOutput](#associatedtype.AssetOutput) = A

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#426)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/transformer.rs.html#427)

#### type [Error](#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")