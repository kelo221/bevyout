[bevy](../index.html)::[audio](index.html)

# Trait AddAudioSource 

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#103)

```rust
pub trait AddAudioSource {
    // Required method
    fn add_audio_source<T>(&mut self) -> &mut Self
       where T: Decodable + Asset,
             f32: FromSample<f32>;
}
```

A trait that allows adding a custom audio source to the object. This is implemented for [`App`](../prelude/struct.App.html "struct bevy::prelude::App") to allow registering custom [`Decodable`](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") types.

## Required Methods

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#110-113)

#### fn [add\_audio\_source](#tymethod.add_audio_source)<T>(&mut self) -> &mut Self

where T: [Decodable](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") + [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Registers an audio source. The type must implement [`Decodable`](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable"), so that it can be converted to a [`rodio::Source`](trait.Source.html "trait bevy::audio::Source") type, and [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), so that it can be registered as an asset. To use this method on [`App`](../prelude/struct.App.html "struct bevy::prelude::App"), the [audio](struct.AudioPlugin.html "struct bevy::audio::AudioPlugin") and [asset](../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin") plugins must be added first.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/lib.rs.html#107)

### impl [AddAudioSource](trait.AddAudioSource.html "trait bevy::audio::AddAudioSource") for [App](../prelude/struct.App.html "struct bevy::prelude::App")