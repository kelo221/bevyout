[bevy](../index.html)::[audio](index.html)

# Trait Decodable 

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#83)

```rust
pub trait Decodable:
    Send
    + Sync
    + 'static {
    type Decoder: Source<Item = f32> + Send + Iterator;

    // Required method
    fn decoder(&self) -> Self::Decoder;
}
```

A type implementing this trait can be converted to a [`rodio::Source`](trait.Source.html "trait bevy::audio::Source") type.

It must be [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") and [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") in order to be registered. Types that implement this trait usually contain raw sound data that can be converted into an iterator of samples. This trait is implemented for [`AudioSource`](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource"). Check the example [`decodable`](https://github.com/bevyengine/bevy/blob/latest/examples/audio/decodable.rs) for how to implement this trait on a custom type.

## Required Associated Types

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#87)

#### type [Decoder](#associatedtype.Decoder): [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")

The type of the iterator of the audio samples, which iterates over samples of type [`rodio::Sample`](type.Sample.html "type bevy::audio::Sample"). Must be a [`rodio::Source`](trait.Source.html "trait bevy::audio::Source") so that it can provide information on the audio it is iterating over.

## Required Methods

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#90)

#### fn [decoder](#tymethod.decoder)(&self) -> Self::[Decoder](../prelude/trait.Decodable.html#associatedtype.Decoder "type bevy::prelude::Decodable::Decoder")

Build and return a [`Self::Decoder`](../prelude/trait.Decodable.html#associatedtype.Decoder "associated type bevy::prelude::Decodable::Decoder") of the implementing type

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#93)

### impl [Decodable](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") for [AudioSource](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio_source.rs.html#94)

#### type [Decoder](#associatedtype.Decoder) = [Decoder](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/decoder/struct.Decoder.html "struct rodio::decoder::Decoder")<[Cursor](https://doc.rust-lang.org/nightly/core/io/cursor/struct.Cursor.html "struct core::io::cursor::Cursor")<[AudioSource](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource")\>>

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/pitch.rs.html#28)

### impl [Decodable](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") for [Pitch](../prelude/struct.Pitch.html "struct bevy::prelude::Pitch")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/pitch.rs.html#29)

#### type [Decoder](#associatedtype.Decoder) = [TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<[SineWave](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/sine/struct.SineWave.html "struct rodio::source::sine::SineWave")\>