[bevy](../index.html)

# Crate audio 

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/lib.rs.html#1-120)

Audio support for the game engine Bevy

```rust
fn main() {
   App::new()
        .add_plugins((MinimalPlugins, AssetPlugin::default(), AudioPlugin::default()))
        .add_systems(Startup, play_background_audio)
        .run();
}

fn play_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("background_audio.ogg")),
        PlaybackSettings::LOOP,
    ));
}
```

## Modules

[prelude](prelude/index.html "mod bevy::audio::prelude")

The audio prelude.

## Structs

[AudioLoader](struct.AudioLoader.html "struct bevy::audio::AudioLoader")

Loads files as [`AudioSource`](../prelude/struct.AudioSource.html "struct bevy::prelude::AudioSource") [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets")

[AudioPlayer](struct.AudioPlayer.html "struct bevy::audio::AudioPlayer")

A component for playing a sound.

[AudioPlayerTemplate](struct.AudioPlayerTemplate.html "struct bevy::audio::AudioPlayerTemplate")

[AudioPlugin](struct.AudioPlugin.html "struct bevy::audio::AudioPlugin")

Adds support for audio playback to a Bevy Application

[AudioSink](struct.AudioSink.html "struct bevy::audio::AudioSink")

Used to control audio during playback.

[AudioSource](struct.AudioSource.html "struct bevy::audio::AudioSource")

A source of audio data

[DefaultSpatialScale](struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

The default scale factor applied to the positions of audio sources and listeners for spatial audio. Can be overridden for individual sounds in [`PlaybackSettings`](../prelude/struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings").

[GlobalVolume](struct.GlobalVolume.html "struct bevy::audio::GlobalVolume")

Use this [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") to control the global volume of all audio.

[Pitch](struct.Pitch.html "struct bevy::audio::Pitch")

A source of sine wave sound

[PlaybackSettings](struct.PlaybackSettings.html "struct bevy::audio::PlaybackSettings")

Initial settings to be used when audio starts playing.

[SpatialAudioSink](struct.SpatialAudioSink.html "struct bevy::audio::SpatialAudioSink")

Used to control spatial audio during playback.

[SpatialListener](struct.SpatialListener.html "struct bevy::audio::SpatialListener")

Settings for the listener for spatial audio sources.

[SpatialScale](struct.SpatialScale.html "struct bevy::audio::SpatialScale")

A scale factor applied to the positions of audio sources and listeners for spatial audio.

## Enums

[PlaybackMode](enum.PlaybackMode.html "enum bevy::audio::PlaybackMode")

The way Bevy manages the sound playback.

[SeekError](enum.SeekError.html "enum bevy::audio::SeekError")

Occurs when `try_seek` fails because the underlying decoder has an error or does not support seeking.

[Volume](enum.Volume.html "enum bevy::audio::Volume")

A [`Volume`](enum.Volume.html "enum bevy::audio::Volume") represents an audio source’s volume level.

## Traits

[AddAudioSource](trait.AddAudioSource.html "trait bevy::audio::AddAudioSource")

A trait that allows adding a custom audio source to the object. This is implemented for [`App`](../prelude/struct.App.html "struct bevy::prelude::App") to allow registering custom [`Decodable`](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") types.

[AudioSinkPlayback](trait.AudioSinkPlayback.html "trait bevy::audio::AudioSinkPlayback")

Common interactions with an audio sink.

[CpalSample](trait.CpalSample.html "trait bevy::audio::CpalSample")

A trait for working generically across different **Sample** format types.

[Decodable](trait.Decodable.html "trait bevy::audio::Decodable")

A type implementing this trait can be converted to a [`rodio::Source`](trait.Source.html "trait bevy::audio::Source") type.

[Source](trait.Source.html "trait bevy::audio::Source")

A source of samples.

## Type Aliases

[ChannelCount](type.ChannelCount.html "type bevy::audio::ChannelCount")

Number of channels in a stream. Can never be Zero

[Sample](type.Sample.html "type bevy::audio::Sample")

Represents value of a single sample. Silence corresponds to the value `0.0`. The expected amplitude range is -1.0…1.0. Values below and above this range are clipped in conversion to other sample types. Use conversion traits from [dasp\_sample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/index.html "mod dasp_sample") crate or [crate::conversions::SampleTypeConverter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/conversions/sample/struct.SampleTypeConverter.html "struct rodio::conversions::sample::SampleTypeConverter") to convert between sample types if necessary.

[SampleRate](type.SampleRate.html "type bevy::audio::SampleRate")

Sample rate (a frame rate or samples per second per channel).