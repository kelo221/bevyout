[bevy](../../index.html)::[asset](../index.html)::[saver](index.html)

# Trait AssetSaver 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#29)

```rust
pub trait AssetSaver:
    TypePath
    + Send
    + Sync
    + 'static {
    type Asset: Asset;
    type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>;
    type OutputLoader: AssetLoader;
    type Error: Into<BevyError>;

    // Required method
    fn save(
        &self,
        writer: &mut (dyn AsyncWrite + Send + Unpin + Sync + 'static),
        asset: SavedAsset<'_, '_, Self::Asset>,
        settings: &Self::Settings,
        asset_path: AssetPath<'_>,
    ) -> impl ConditionalSendFuture;
}
```

Saves an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") of a given [`AssetSaver::Asset`](trait.AssetSaver.html#associatedtype.Asset "associated type bevy::asset::saver::AssetSaver::Asset") type. [`AssetSaver::OutputLoader`](trait.AssetSaver.html#associatedtype.OutputLoader "associated type bevy::asset::saver::AssetSaver::OutputLoader") will then be used to load the saved asset in the final deployed application. The saver should produce asset bytes in a format that [`AssetSaver::OutputLoader`](trait.AssetSaver.html#associatedtype.OutputLoader "associated type bevy::asset::saver::AssetSaver::OutputLoader") can read.

This trait is generally used in concert with [`AssetWriter`](../io/trait.AssetWriter.html "trait bevy::asset::io::AssetWriter") to write assets as bytes.

For a version of this trait that can load assets, see [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader").

Note: This is currently only leveraged by the [`AssetProcessor`](../processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor"), and does not provide a suitable interface for general purpose asset persistence. See [github issue #11216](https://github.com/bevyengine/bevy/issues/11216).

## Required Associated Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#31)

#### type [Asset](#associatedtype.Asset): [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset")

The top level [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") saved by this [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#33)

#### type [Settings](#associatedtype.Settings): [Settings](../meta/trait.Settings.html "trait bevy::asset::meta::Settings") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + for<'a> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'a>

The settings type used by this [`AssetSaver`](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#35)

#### type [OutputLoader](#associatedtype.OutputLoader): [AssetLoader](../trait.AssetLoader.html "trait bevy::asset::AssetLoader")

The type of [`AssetLoader`](../trait.AssetLoader.html "trait bevy::asset::AssetLoader") used to load this [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#37)

#### type [Error](#associatedtype.Error): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

The type of [error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") which could be encountered by this saver.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/saver.rs.html#41-49)

#### fn [save](#tymethod.save)( &self, writer: &mut (dyn [AsyncWrite](../../tasks/futures_lite/trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static), asset: [SavedAsset](struct.SavedAsset.html "struct bevy::asset::saver::SavedAsset")<'\_, '\_, Self::[Asset](trait.AssetSaver.html#associatedtype.Asset "type bevy::asset::saver::AssetSaver::Asset")\>, settings: &Self::[Settings](trait.AssetSaver.html#associatedtype.Settings "type bevy::asset::saver::AssetSaver::Settings"), asset\_path: [AssetPath](../struct.AssetPath.html "struct bevy::asset::AssetPath")<'\_>, ) -> impl [ConditionalSendFuture](../../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Saves the given runtime [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") by writing it to a byte format using `writer`. The passed in `settings` can influence how the `asset` is saved.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#27)

### impl [AssetSaver](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") for [CompressedImageSaver](../../image/struct.CompressedImageSaver.html "struct bevy::image::CompressedImageSaver")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#28)

#### type [Asset](#associatedtype.Asset) = [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#30)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#31)

#### type [OutputLoader](#associatedtype.OutputLoader) = [ImageLoader](../../image/struct.ImageLoader.html "struct bevy::image::ImageLoader")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/compressed_image_saver.rs.html#32)

#### type [Error](#associatedtype.Error) = [CompressedImageSaverError](../../image/enum.CompressedImageSaverError.html "enum bevy::image::CompressedImageSaverError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#23)

### impl [AssetSaver](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") for [ImageSaver](../../image/struct.ImageSaver.html "struct bevy::image::ImageSaver")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#24)

#### type [Asset](#associatedtype.Asset) = [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#25)

#### type [Error](#associatedtype.Error) = [SaveImageError](../../image/enum.SaveImageError.html "enum bevy::image::SaveImageError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#26)

#### type [OutputLoader](#associatedtype.OutputLoader) = [ImageLoader](../../image/struct.ImageLoader.html "struct bevy::image::ImageLoader")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/saver.rs.html#27)

#### type [Settings](#associatedtype.Settings) = [ImageSaverSettings](../../image/struct.ImageSaverSettings.html "struct bevy::image::ImageSaverSettings")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#151)

### impl [AssetSaver](trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") for [MeshletMeshSaver](../../pbr/experimental/meshlet/struct.MeshletMeshSaver.html "struct bevy::pbr::experimental::meshlet::MeshletMeshSaver")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#152)

#### type [Asset](#associatedtype.Asset) = [MeshletMesh](../../pbr/experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#153)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#154)

#### type [OutputLoader](#associatedtype.OutputLoader) = [MeshletMeshLoader](../../pbr/experimental/meshlet/struct.MeshletMeshLoader.html "struct bevy::pbr::experimental::meshlet::MeshletMeshLoader")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#155)

#### type [Error](#associatedtype.Error) = MeshletMeshSaveOrLoadError