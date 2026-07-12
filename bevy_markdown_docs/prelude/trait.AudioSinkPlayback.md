[bevy](../index.html)::[prelude](index.html)

# Trait AudioSinkPlayback 

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#10)

```rust
pub trait AudioSinkPlayback {
    // Required methods
    fn volume(&self) -> Volume;
    fn set_volume(&mut self, volume: Volume);
    fn speed(&self) -> f32;
    fn set_speed(&self, speed: f32);
    fn play(&self);
    fn position(&self) -> Duration;
    fn try_seek(&self, pos: Duration) -> Result<(), SeekError>;
    fn pause(&self);
    fn is_paused(&self) -> bool;
    fn stop(&self);
    fn empty(&self) -> bool;
    fn is_muted(&self) -> bool;
    fn mute(&mut self);
    fn unmute(&mut self);

    // Provided methods
    fn toggle_playback(&self) { ... }
    fn toggle_mute(&mut self) { ... }
}
```

Common interactions with an audio sink.

## Required Methods

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#17)

#### fn [volume](#tymethod.volume)(&self) -> [Volume](../audio/enum.Volume.html "enum bevy::audio::Volume")

Gets the volume of the sound as a [`Volume`](../audio/enum.Volume.html "enum bevy::audio::Volume").

If the sink is muted, this returns the managed volume rather than the sink’s actual volume. This allows you to use the returned volume as if the sink were not muted, because a muted sink has a physical volume of 0.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#26)

#### fn [set\_volume](#tymethod.set_volume)(&mut self, volume: [Volume](../audio/enum.Volume.html "enum bevy::audio::Volume"))

Changes the volume of the sound to the given [`Volume`](../audio/enum.Volume.html "enum bevy::audio::Volume").

If the sink is muted, changing the volume won’t unmute it, i.e. the sink’s volume will remain “off” / “muted”. However, the sink will remember the volume change and it will be used when [`unmute`](trait.AudioSinkPlayback.html#tymethod.unmute "method bevy::prelude::AudioSinkPlayback::unmute") is called. This allows you to control the volume even when the sink is muted.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#32)

#### fn [speed](#tymethod.speed)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Gets the speed of the sound.

The value `1.0` is the “normal” speed (unfiltered input). Any value other than `1.0` will change the play speed of the sound.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#38)

#### fn [set\_speed](#tymethod.set_speed)(&self, speed: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Changes the speed of the sound.

The value `1.0` is the “normal” speed (unfiltered input). Any value other than `1.0` will change the play speed of the sound.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#43)

#### fn [play](#tymethod.play)(&self)

Resumes playback of a paused sink.

No effect if not paused.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#51)

#### fn [position](#tymethod.position)(&self) -> [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

Returns the position of the sound that’s being played.

This takes into account any speedup or delay applied.

Example: if you [`set_speed(2.0)`](trait.AudioSinkPlayback.html#tymethod.set_speed "method bevy::prelude::AudioSinkPlayback::set_speed") and [`position()`](trait.AudioSinkPlayback.html#tymethod.position "method bevy::prelude::AudioSinkPlayback::position") returns _5s_, then the position in the recording is _10s_ from its start.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#71)

#### fn [try\_seek](#tymethod.try_seek)(&self, pos: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SeekError](../audio/enum.SeekError.html "enum bevy::audio::SeekError")\>

Attempts to seek to a given position in the current source.

This blocks between 0 and ~5 milliseconds.

As long as the duration of the source is known, seek is guaranteed to saturate at the end of the source. For example given a source that reports a total duration of 42 seconds calling `try_seek()` with 60 seconds as argument will seek to 42 seconds.

##### Errors

This function will return [`SeekError::NotSupported`](../audio/enum.SeekError.html#variant.NotSupported "variant bevy::audio::SeekError::NotSupported") if one of the underlying sources does not support seeking.

It will return an error if an implementation ran into one during the seek.

When seeking beyond the end of a source, this function might return an error if the duration of the source is not known.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#77)

#### fn [pause](#tymethod.pause)(&self)

Pauses playback of this sink.

No effect if already paused. A paused sink can be resumed with [`play`](trait.AudioSinkPlayback.html#tymethod.play "method bevy::prelude::AudioSinkPlayback::play").

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#94)

#### fn [is\_paused](#tymethod.is_paused)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the sink is paused.

Sinks can be paused and resumed using [`pause`](trait.AudioSinkPlayback.html#tymethod.pause "method bevy::prelude::AudioSinkPlayback::pause") and [`play`](trait.AudioSinkPlayback.html#tymethod.play "method bevy::prelude::AudioSinkPlayback::play").

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#99)

#### fn [stop](#tymethod.stop)(&self)

Stops the sink.

It won’t be possible to restart it afterwards.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#102)

#### fn [empty](#tymethod.empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if this sink has no more sounds to play.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#105)

#### fn [is\_muted](#tymethod.is_muted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the sink is muted.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#111)

#### fn [mute](#tymethod.mute)(&mut self)

Mutes the sink.

Muting a sink sets the volume to 0. Use [`unmute`](trait.AudioSinkPlayback.html#tymethod.unmute "method bevy::prelude::AudioSinkPlayback::unmute") to unmute the sink and restore the original volume.

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#116)

#### fn [unmute](#tymethod.unmute)(&mut self)

Unmutes the sink.

Restores the volume to the value it was before it was muted.

## Provided Methods

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#83)

#### fn [toggle\_playback](#method.toggle_playback)(&self)

Toggles playback of the sink.

If the sink is paused, toggling playback resumes it. If the sink is playing, toggling playback pauses it.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/audio/audio\_control.rs ([line 81](../../src/audio_control/audio_control.rs.html#81))

```rust
72fn pause(
73    keyboard_input: Res<ButtonInput<KeyCode>>,
74    music_controller: Query<&AudioSink, With<MyMusic>>,
75) {
76    let Ok(sink) = music_controller.single() else {
77        return;
78    };
79
80    if keyboard_input.just_pressed(KeyCode::Space) {
81        sink.toggle_playback();
82    }
83}
```

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#119)

#### fn [toggle\_mute](#method.toggle_mute)(&mut self)

Toggles whether the sink is muted or not.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/audio/spatial\_audio\_3d.rs ([line 136](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#136))

```rust
133fn mute(keyboard_input: Res<ButtonInput<KeyCode>>, mut sinks: Query<&mut SpatialAudioSink>) {
134    if keyboard_input.just_pressed(KeyCode::KeyM) {
135        for mut sink in sinks.iter_mut() {
136            sink.toggle_mute();
137        }
138    }
139}
```

Hide additional examples

examples/audio/audio\_control.rs ([line 94](../../src/audio_control/audio_control.rs.html#94))

```rust
85fn mute(
86    keyboard_input: Res<ButtonInput<KeyCode>>,
87    mut music_controller: Query<&mut AudioSink, With<MyMusic>>,
88) {
89    let Ok(mut sink) = music_controller.single_mut() else {
90        return;
91    };
92
93    if keyboard_input.just_pressed(KeyCode::KeyM) {
94        sink.toggle_mute();
95    }
96}
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#166)

### impl [AudioSinkPlayback](trait.AudioSinkPlayback.html "trait bevy::prelude::AudioSinkPlayback") for [AudioSink](struct.AudioSink.html "struct bevy::prelude::AudioSink")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/sinks.rs.html#270)

### impl [AudioSinkPlayback](trait.AudioSinkPlayback.html "trait bevy::prelude::AudioSinkPlayback") for [SpatialAudioSink](struct.SpatialAudioSink.html "struct bevy::prelude::SpatialAudioSink")