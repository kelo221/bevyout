[bevy](../../index.html)::[audio](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/lib.rs.html#42)

The audio prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AudioPlayer](struct.AudioPlayer.html "struct bevy::audio::prelude::AudioPlayer")

A component for playing a sound.

[AudioSink](struct.AudioSink.html "struct bevy::audio::prelude::AudioSink")

Used to control audio during playback.

[AudioSource](struct.AudioSource.html "struct bevy::audio::prelude::AudioSource")

A source of audio data

[GlobalVolume](struct.GlobalVolume.html "struct bevy::audio::prelude::GlobalVolume")

Use this [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") to control the global volume of all audio.

[Pitch](struct.Pitch.html "struct bevy::audio::prelude::Pitch")

A source of sine wave sound

[PlaybackSettings](struct.PlaybackSettings.html "struct bevy::audio::prelude::PlaybackSettings")

Initial settings to be used when audio starts playing.

[SpatialAudioSink](struct.SpatialAudioSink.html "struct bevy::audio::prelude::SpatialAudioSink")

Used to control spatial audio during playback.

[SpatialListener](struct.SpatialListener.html "struct bevy::audio::prelude::SpatialListener")

Settings for the listener for spatial audio sources.

## Traits

[AudioSinkPlayback](trait.AudioSinkPlayback.html "trait bevy::audio::prelude::AudioSinkPlayback")

Common interactions with an audio sink.

[Decodable](trait.Decodable.html "trait bevy::audio::prelude::Decodable")

A type implementing this trait can be converted to a [`rodio::Source`](../trait.Source.html "trait bevy::audio::Source") type.