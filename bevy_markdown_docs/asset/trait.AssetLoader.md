[bevy](../index.html)::[asset](index.html)

# Trait AssetLoader 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#32)

```rust
pub trait AssetLoader:
    TypePath
    + Send
    + Sync
    + 'static {
    type Asset: Asset;
    type Settings: Settings + Default + Serialize + for<'a> Deserialize<'a>;
    type Error: Into<BevyError>;

    // Required method
    fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> impl ConditionalSendFuture;

    // Provided method
    fn extensions(&self) -> &[&str] { ... }
}
```

Loads an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") from a given byte [`Reader`](io/trait.Reader.html "trait bevy::asset::io::Reader"). This can accept [`AssetLoader::Settings`](trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings"), which configure how the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") should be loaded.

This trait is generally used in concert with [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader") to load assets from a byte source.

For a complementary version of this trait that can save assets, see [`AssetSaver`](saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver").

## Required Associated Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#34)

#### type [Asset](#associatedtype.Asset): [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset")

The top level [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") loaded by this [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#36)

#### type [Settings](#associatedtype.Settings): [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + for<'a> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'a>

The settings type used by this [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#38)

#### type [Error](#associatedtype.Error): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

The type of [error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") which could be encountered by this loader.

## Required Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#40-45)

#### fn [load](#tymethod.load)( &self, reader: &mut dyn [Reader](io/trait.Reader.html "trait bevy::asset::io::Reader"), settings: &Self::[Settings](trait.AssetLoader.html#associatedtype.Settings "type bevy::asset::AssetLoader::Settings"), load\_context: &mut [LoadContext](struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, ) -> impl [ConditionalSendFuture](../tasks/trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture")

Asynchronously loads [`AssetLoader::Asset`](trait.AssetLoader.html#associatedtype.Asset "associated type bevy::asset::AssetLoader::Asset") (and any other labeled assets) from the bytes provided by [`Reader`](io/trait.Reader.html "trait bevy::asset::io::Reader").

## Provided Methods

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/loader.rs.html#49)

#### fn [extensions](#method.extensions)(&self) -> &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\]

Returns a list of extensions supported by this [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader"), without the preceding dot. Note that users of this [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") may choose to load files with a non-matching extension.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#218)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

The () loader should never be called. This implementation exists to make the meta format nicer to work with.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#219)

#### type [Asset](#associatedtype.Asset) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#220)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#221)

#### type [Error](#associatedtype.Error) = [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#222-227)

#### async fn [load](#tymethod.load)( &self, \_reader: &mut dyn [Reader](io/trait.Reader.html "trait bevy::asset::io::Reader"), \_settings: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader")\>::[Settings](trait.AssetLoader.html#associatedtype.Settings "type bevy::asset::AssetLoader::Settings"), \_load\_context: &mut [LoadContext](struct.LoadContext.html "struct bevy::asset::LoadContext")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader")\>::[Asset](trait.AssetLoader.html#associatedtype.Asset "type bevy::asset::AssetLoader::Asset"), <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader")\>::[Error](trait.AssetLoader.html#associatedtype.Error "type bevy::asset::AssetLoader::Error")\>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#231)

#### fn [extensions](#method.extensions)(&self) -> &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\]

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#747)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [AnimationGraphAssetLoader](../prelude/struct.AnimationGraphAssetLoader.html "struct bevy::prelude::AnimationGraphAssetLoader")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#748)

#### type [Asset](#associatedtype.Asset) = [AnimationGraph](../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#750)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#752)

#### type [Error](#associatedtype.Error) = [AnimationGraphLoadError](../prelude/enum.AnimationGraphLoadError.html "enum bevy::prelude::AnimationGraphLoadError")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#41)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [AudioLoader](../audio/struct.AudioLoader.html "struct bevy::audio::AudioLoader")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#42)

#### type [Asset](#associatedtype.Asset) = [AudioSource](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#43)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#44)

#### type [Error](#associatedtype.Error) = [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#35)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [ExrTextureLoader](../image/struct.ExrTextureLoader.html "struct bevy::image::ExrTextureLoader")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#36)

#### type [Asset](#associatedtype.Asset) = [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#37)

#### type [Settings](#associatedtype.Settings) = [ExrTextureLoaderSettings](../image/struct.ExrTextureLoaderSettings.html "struct bevy::image::ExrTextureLoaderSettings")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/exr_texture_loader.rs.html#38)

#### type [Error](#associatedtype.Error) = [ExrTextureLoaderError](../image/enum.ExrTextureLoaderError.html "enum bevy::image::ExrTextureLoaderError")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#22)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [FontLoader](../text/struct.FontLoader.html "struct bevy::text::FontLoader")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#23)

#### type [Asset](#associatedtype.Asset) = [Font](../prelude/struct.Font.html "struct bevy::prelude::Font")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#24)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_loader.rs.html#25)

#### type [Error](#associatedtype.Error) = [FontLoaderError](../text/enum.FontLoaderError.html "enum bevy::text::FontLoaderError")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#1162)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [GltfLoader](../gltf/struct.GltfLoader.html "struct bevy::gltf::GltfLoader")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#1163)

#### type [Asset](#associatedtype.Asset) = [Gltf](../prelude/struct.Gltf.html "struct bevy::prelude::Gltf")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#1164)

#### type [Settings](#associatedtype.Settings) = [GltfLoaderSettings](../gltf/struct.GltfLoaderSettings.html "struct bevy::gltf::GltfLoaderSettings")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/mod.rs.html#1165)

#### type [Error](#associatedtype.Error) = [GltfError](../gltf/enum.GltfError.html "enum bevy::gltf::GltfError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#33)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [HdrTextureLoader](../image/struct.HdrTextureLoader.html "struct bevy::image::HdrTextureLoader")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#34)

#### type [Asset](#associatedtype.Asset) = [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#35)

#### type [Settings](#associatedtype.Settings) = [HdrTextureLoaderSettings](../image/struct.HdrTextureLoaderSettings.html "struct bevy::image::HdrTextureLoaderSettings")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/hdr_texture_loader.rs.html#36)

#### type [Error](#associatedtype.Error) = [HdrTextureLoaderError](../image/enum.HdrTextureLoaderError.html "enum bevy::image::HdrTextureLoaderError")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#189)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [ImageLoader](../image/struct.ImageLoader.html "struct bevy::image::ImageLoader")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#190)

#### type [Asset](#associatedtype.Asset) = [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#191)

#### type [Settings](#associatedtype.Settings) = [ImageLoaderSettings](../image/struct.ImageLoaderSettings.html "struct bevy::image::ImageLoaderSettings")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_loader.rs.html#192)

#### type [Error](#associatedtype.Error) = [ImageLoaderError](../image/enum.ImageLoaderError.html "enum bevy::image::ImageLoaderError")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#198)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [MeshletMeshLoader](../pbr/experimental/meshlet/struct.MeshletMeshLoader.html "struct bevy::pbr::experimental::meshlet::MeshletMeshLoader")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#199)

#### type [Asset](#associatedtype.Asset) = [MeshletMesh](../pbr/experimental/meshlet/struct.MeshletMesh.html "struct bevy::pbr::experimental::meshlet::MeshletMesh")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#200)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/asset.rs.html#201)

#### type [Error](#associatedtype.Error) = MeshletMeshSaveOrLoadError

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#323)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [ShaderLoader](../shader/struct.ShaderLoader.html "struct bevy::shader::ShaderLoader")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#324)

#### type [Asset](#associatedtype.Asset) = [Shader](../prelude/struct.Shader.html "struct bevy::prelude::Shader")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#325)

#### type [Settings](#associatedtype.Settings) = [ShaderSettings](../shader/struct.ShaderSettings.html "struct bevy::shader::ShaderSettings")

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader.rs.html#326)

#### type [Error](#associatedtype.Error) = [ShaderLoaderError](../shader/enum.ShaderLoaderError.html "enum bevy::shader::ShaderLoaderError")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#50)

### impl [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for [WorldAssetLoader](../world_serialization/struct.WorldAssetLoader.html "struct bevy::world_serialization::WorldAssetLoader")

Available on **crate feature `serialize`** only.

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#51)

#### type [Asset](#associatedtype.Asset) = [DynamicWorld](../prelude/struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#52)

#### type [Settings](#associatedtype.Settings) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#53)

#### type [Error](#associatedtype.Error) = [WorldAssetLoaderError](../world_serialization/enum.WorldAssetLoaderError.html "enum bevy::world_serialization::WorldAssetLoaderError")