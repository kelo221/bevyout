[bevy](../../index.html)::[asset](../index.html)

# Module transformer 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#157)

## Structs

[IdentityAssetTransformer](struct.IdentityAssetTransformer.html "struct bevy::asset::transformer::IdentityAssetTransformer")

An identity [`AssetTransformer`](trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer") which infallibly returns the input [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") on transformation.\]

[TransformedAsset](struct.TransformedAsset.html "struct bevy::asset::transformer::TransformedAsset")

An [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") (and any “sub assets”) intended to be transformed

[TransformedSubAsset](struct.TransformedSubAsset.html "struct bevy::asset::transformer::TransformedSubAsset")

A labeled sub-asset of [`TransformedAsset`](struct.TransformedAsset.html "struct bevy::asset::transformer::TransformedAsset")

## Traits

[AssetTransformer](trait.AssetTransformer.html "trait bevy::asset::transformer::AssetTransformer")

Transforms an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of a given [`AssetTransformer::AssetInput`](trait.AssetTransformer.html#associatedtype.AssetInput "associated type bevy::asset::transformer::AssetTransformer::AssetInput") type to an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of [`AssetTransformer::AssetOutput`](trait.AssetTransformer.html#associatedtype.AssetOutput "associated type bevy::asset::transformer::AssetTransformer::AssetOutput") type.