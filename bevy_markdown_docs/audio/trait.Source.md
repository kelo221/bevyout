[bevy](../index.html)::[audio](index.html)

# Trait Source 

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#166)

```rust
pub trait Source: Iterator<Item = f32> {
    // Required methods
    fn current_span_len(&self) -> Option<usize>;
    fn channels(&self) -> NonZero<u16>;
    fn sample_rate(&self) -> NonZero<u32>;
    fn total_duration(&self) -> Option<Duration>;

    // Provided methods
    fn is_exhausted(&self) -> bool { ... }
    fn buffered(self) -> Buffered<Self> ⓘ
       where Self: Sized { ... }
    fn mix<S>(self, other: S) -> Mix<Self, S> ⓘ
       where Self: Sized,
             S: Source { ... }
    fn repeat_infinite(self) -> Repeat<Self> ⓘ
       where Self: Sized { ... }
    fn take_duration(self, duration: Duration) -> TakeDuration<Self> ⓘ
       where Self: Sized { ... }
    fn delay(self, duration: Duration) -> Delay<Self> ⓘ
       where Self: Sized { ... }
    fn skip_duration(self, duration: Duration) -> SkipDuration<Self> ⓘ
       where Self: Sized { ... }
    fn amplify(self, value: f32) -> Amplify<Self> ⓘ
       where Self: Sized { ... }
    fn amplify_decibel(self, value: f32) -> Amplify<Self> ⓘ
       where Self: Sized { ... }
    fn amplify_normalized(self, value: f32) -> Amplify<Self> ⓘ
       where Self: Sized { ... }
    fn automatic_gain_control(
        self,
        agc_settings: AutomaticGainControlSettings,
    ) -> AutomaticGainControl<Self> ⓘ
       where Self: Sized { ... }
    fn take_crossfade_with<S>(
        self,
        other: S,
        duration: Duration,
    ) -> Mix<TakeDuration<Self>, FadeIn<TakeDuration<S>>> ⓘ
       where S: Source,
             Self: Sized,
             Self::Item: FromSample<<S as Iterator>::Item> { ... }
    fn fade_in(self, duration: Duration) -> FadeIn<Self> ⓘ
       where Self: Sized { ... }
    fn fade_out(self, duration: Duration) -> FadeOut<Self> ⓘ
       where Self: Sized { ... }
    fn limit(self, settings: LimitSettings) -> Limit<Self> ⓘ
       where Self: Sized { ... }
    fn linear_gain_ramp(
        self,
        duration: Duration,
        start_value: f32,
        end_value: f32,
        clamp_end: bool,
    ) -> LinearGainRamp<Self> ⓘ
       where Self: Sized { ... }
    fn periodic_access<F>(
        self,
        period: Duration,
        access: F,
    ) -> PeriodicAccess<Self, F> ⓘ
       where Self: Sized,
             F: FnMut(&mut Self) { ... }
    fn speed(self, ratio: f32) -> Speed<Self> ⓘ
       where Self: Sized { ... }
    fn record(self) -> SamplesBuffer ⓘ
       where Self: Sized { ... }
    fn reverb(
        self,
        duration: Duration,
        amplitude: f32,
    ) -> Mix<Self, Delay<Amplify<Self>>> ⓘ
       where Self: Sized + Clone { ... }
    fn pausable(self, initially_paused: bool) -> Pausable<Self> ⓘ
       where Self: Sized { ... }
    fn stoppable(self) -> Stoppable<Self> ⓘ
       where Self: Sized { ... }
    fn skippable(self) -> Skippable<Self> ⓘ
       where Self: Sized { ... }
    fn track_position(self) -> TrackPosition<Self> ⓘ
       where Self: Sized { ... }
    fn low_pass(self, freq: u32) -> BltFilter<Self> ⓘ
       where Self: Sized + Source<Item = f32> { ... }
    fn high_pass(self, freq: u32) -> BltFilter<Self> ⓘ
       where Self: Sized + Source<Item = f32> { ... }
    fn low_pass_with_q(self, freq: u32, q: f32) -> BltFilter<Self> ⓘ
       where Self: Sized + Source<Item = f32> { ... }
    fn high_pass_with_q(self, freq: u32, q: f32) -> BltFilter<Self> ⓘ
       where Self: Sized + Source<Item = f32> { ... }
    fn distortion(self, gain: f32, threshold: f32) -> Distortion<Self> ⓘ
       where Self: Sized { ... }
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> { ... }
}
```

A source of samples.

## A quick lesson about sounds

### Sampling

A sound is a vibration that propagates through air and reaches your ears. This vibration can be represented as an analog signal.

In order to store this signal in the computer’s memory or on the disk, we perform what is called _sampling_. This consists in choosing an interval of time (for example 20µs) and reading the amplitude of the signal at each interval (for example, if the interval is 20µs we read the amplitude every 20µs). By doing so we obtain a list of numerical values, each value being called a _sample_.

Therefore, a sound can be represented in memory by a frequency and a list of samples. The frequency is expressed in hertz and corresponds to the number of samples that have been read per second. For example if we read one sample every 20µs, the frequency would be 50000 Hz. In reality, common values for the frequency are 44100, 48000 and 96000.

### Channels

But a frequency and a list of values only represent one signal. When you listen to a sound, your left and right ears don’t receive exactly the same signal. In order to handle this, we usually record not one but two different signals: one for the left ear and one for the right ear. We say that such a sound has two _channels_.

Sometimes sounds even have five or six channels, each corresponding to a location around the head of the listener.

The standard in audio manipulation is to _interleave_ the multiple channels. In other words, in a sound with two channels the list of samples contains the first sample of the first channel, then the first sample of the second channel, then the second sample of the first channel, then the second sample of the second channel, and so on. The same applies if you have more than two channels. The rodio library only supports this schema.

Therefore, in order to represent a sound in memory in fact we need three characteristics: the frequency, the number of channels, and the list of samples.

### The `Source` trait

A Rust object that represents a sound should implement the `Source` trait.

The three characteristics that describe a sound are provided through this trait:

*   The number of channels can be retrieved with `channels`.
*   The frequency can be retrieved with `sample_rate`.
*   The list of values can be retrieved by iterating on the source. The `Source` trait requires that the `Iterator` trait be implemented as well. When a `Source` returns None the sound has ended.

## Spans

The samples rate and number of channels of some sound sources can change by itself from time to time.

> **Note**: As a basic example, if you play two audio files one after the other and treat the whole as a single source, then the channels and samples rate of that source may change at the transition between the two files.

However, for optimization purposes rodio supposes that the number of channels and the frequency stay the same for long periods of time and avoids calling `channels()` and `sample_rate` too frequently.

In order to properly handle this situation, the `current_span_len()` method should return the number of samples that remain in the iterator before the samples rate and number of channels can potentially change.

## Required Methods

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#183)

#### fn [current\_span\_len](#tymethod.current_span_len)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the number of samples before the current span ends.

`None` means “infinite” or “until the sound ends”. Sources that return `Some(x)` should return `Some(0)` if and only if when there’s no more data.

After the engine has finished reading the specified number of samples, it will check whether the value of `channels()` and/or `sample_rate()` have changed.

##### Frame Alignment

Span lengths must be multiples of the channel count to ensure spans end on frame boundaries. A “frame” is one sample for each channel. Returning a span length that is not a multiple of `channels()` will cause channel misalignment issues.

Note: This returns the total span size, not the remaining samples. Use `Iterator::size_hint` to determine how many samples remain in the iterator.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#193)

#### fn [channels](#tymethod.channels)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

Returns the number of channels. Channels are always interleaved. Should never be Zero

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#196)

#### fn [sample\_rate](#tymethod.sample_rate)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Returns the rate at which the source should be played. In number of samples per second.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#201)

#### fn [total\_duration](#tymethod.total_duration)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>

Returns the total duration of this source, if known.

`None` indicates at the same time “infinite” or “unknown”.

## Provided Methods

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#187)

#### fn [is\_exhausted](#method.is_exhausted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the source is exhausted (has no more samples available).

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#205-207)

#### fn [buffered](#method.buffered)(self) -> [Buffered](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html "struct rodio::source::buffered::Buffered")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Stores the source in a buffer in addition to returning it. This iterator can be cloned.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#238-241)

#### fn [mix](#method.mix)<S>(self, other: S) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, S> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Source](trait.Source.html "trait bevy::audio::Source"),

Mixes this source with another one.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#251-253)

#### fn [repeat\_infinite](#method.repeat_infinite)(self) -> [Repeat](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html "struct rodio::source::repeat::Repeat")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Repeats this source forever.

Note that this works by storing the data in a buffer, so the amount of memory used is proportional to the size of the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#260-262)

#### fn [take\_duration](#method.take_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a certain duration of this source and then stops.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#272-274)

#### fn [delay](#method.delay)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Delays the sound by a certain duration.

The rate and channels of the silence will use the same format as the first span of the source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#283-285)

#### fn [skip\_duration](#method.skip_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [SkipDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html "struct rodio::source::skip::SkipDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immediately skips a certain duration of this source.

If the specified duration is longer than the source itself, `skip_duration` will skip to the end of the source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#292-294)

#### fn [amplify](#method.amplify)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#301-303)

#### fn [amplify\_decibel](#method.amplify_decibel)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound logarithmically by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#315-317)

#### fn [amplify\_normalized](#method.amplify_normalized)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Normalized amplification in `[0.0, 1.0]` range. This method better matches the perceived loudness of sounds in human hearing and is recommended to use when you want to change volume in `[0.0, 1.0]` range. based on article: [https://www.dr-lex.be/info-stuff/volumecontrols.html](https://www.dr-lex.be/info-stuff/volumecontrols.html)

**note: it clamps values outside this range.**

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#407-412)

#### fn [automatic\_gain\_control](#method.automatic_gain_control)( self, agc\_settings: [AutomaticGainControlSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControlSettings.html "struct rodio::source::agc::AutomaticGainControlSettings"), ) -> [AutomaticGainControl](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html "struct rodio::source::agc::AutomaticGainControl")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies automatic gain control to the sound.

Automatic Gain Control (AGC) adjusts the amplitude of the audio signal to maintain a consistent output level.

##### Parameters

`target_level`: **TL;DR**: Desired output level. 1.0 = original level, > 1.0 amplifies, < 1.0 reduces.

The desired output level, where 1.0 represents the original sound level. Values above 1.0 will amplify the sound, while values below 1.0 will lower it. For example, a target\_level of 1.4 means that at normal sound levels, the AGC will aim to increase the gain by a factor of 1.4, resulting in a minimum 40% amplification. A recommended level is `1.0`, which maintains the original sound level.

`attack_time`: **TL;DR**: Response time for volume increases. Shorter = faster but may cause abrupt changes. **Recommended: `4.0` seconds**.

The time (in seconds) for the AGC to respond to input level increases. Shorter times mean faster response but may cause abrupt changes. Longer times result in smoother transitions but slower reactions to sudden volume changes. Too short can lead to overreaction to peaks, causing unnecessary adjustments. Too long can make the AGC miss important volume changes or react too slowly to sudden loud passages. Very high values might result in excessively loud output or sluggish response, as the AGC’s adjustment speed is limited by the attack time. Balance is key for optimal performance. A recommended attack\_time of `4.0` seconds provides a sweet spot for most applications.

`release_time`: **TL;DR**: Response time for volume decreases. Shorter = faster gain reduction. **Recommended: `0.0` seconds**.

The time (in seconds) for the AGC to respond to input level decreases. This parameter controls how quickly the gain is reduced when the signal level drops. Shorter release times result in faster gain reduction, which can be useful for quick adaptation to quieter passages but may lead to pumping effects. Longer release times provide smoother transitions but may be slower to respond to sudden decreases in volume. However, if the release\_time is too high, the AGC may not be able to lower the gain quickly enough, potentially leading to clipping and distorted sound before it can adjust. Finding the right balance is crucial for maintaining natural-sounding dynamics and preventing distortion. A recommended release\_time of `0.0` seconds works well for general use, allowing the AGC to decrease the gain immediately with no delay, ensuring there is no clipping.

`absolute_max_gain`: **TL;DR**: Maximum allowed gain. Prevents over-amplification. **Recommended: `5.0`**.

The maximum gain that can be applied to the signal. This parameter acts as a safeguard against excessive amplification of quiet signals or background noise. It establishes an upper boundary for the AGC’s signal boost, effectively preventing distortion or overamplification of low-level sounds. This is crucial for maintaining audio quality and preventing unexpected volume spikes. A recommended value for `absolute_max_gain` is `5`, which provides a good balance between amplification capability and protection against distortion in most scenarios.

`automatic_gain_control` example in this project shows a pattern you can use to enable/disable the AGC filter dynamically.

##### Example (Quick start)

```rust
// Apply Automatic Gain Control to the source (AGC is on by default)
use rodio::source::{Source, SineWave, AutomaticGainControlSettings};
use rodio::Player;
use std::time::Duration;
let source = SineWave::new(444.0); // An example.
let (player, output) = Player::new(); // An example.

let agc_source = source.automatic_gain_control(AutomaticGainControlSettings::default());

// Add the AGC-controlled source to the sink
player.append(agc_source);
```

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#431-434)

#### fn [take\_crossfade\_with](#method.take_crossfade_with)<S>( self, other: S, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self>, [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<S>>> [ⓘ](#)

where S: [Source](trait.Source.html "trait bevy::audio::Source"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<<S as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Mixes this sound fading out with another sound fading in for the given duration.

Only the crossfaded portion (beginning of self, beginning of other) is returned.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#441-443)

#### fn [fade\_in](#method.fade_in)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades in the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#450-452)

#### fn [fade\_out](#method.fade_out)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeOut](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html "struct rodio::source::fadeout::FadeOut")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades out the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#504-506)

#### fn [limit](#method.limit)(self, settings: [LimitSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.LimitSettings.html "struct rodio::source::limit::LimitSettings")) -> [Limit](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html "struct rodio::source::limit::Limit")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies limiting to prevent audio peaks from exceeding a threshold.

A limiter reduces the amplitude of audio signals that exceed a specified level, preventing clipping and maintaining consistent output levels. The limiter processes each channel independently for envelope detection but applies gain reduction uniformly across all channels to preserve stereo imaging.

##### Arguments

*   `settings` - [`LimitSettings`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.LimitSettings.html "struct rodio::source::limit::LimitSettings") struct containing:
    *   **threshold** - Level in dB where limiting begins (must be negative)
    *   **knee\_width** - Range in dB over which limiting gradually increases
    *   **attack** - Time to respond to level increases
    *   **release** - Time to recover after level decreases

##### Returns

A [`Limit`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html "struct rodio::source::limit::Limit") source that applies the limiting to the input audio.

##### Examples

###### Basic Usage with Default Settings

```rust
use rodio::source::{SineWave, Source, LimitSettings};
use std::time::Duration;

// Create a loud sine wave and apply default limiting (-1dB threshold)
let source = SineWave::new(440.0).amplify(2.0);
let limited = source.limit(LimitSettings::default());
```

###### Custom Settings with Builder Pattern

```rust
use rodio::source::{SineWave, Source, LimitSettings};
use std::time::Duration;

let source = SineWave::new(440.0).amplify(3.0);
let settings = LimitSettings::default()
    .with_threshold(-6.0)                    // Limit at -6dB
    .with_knee_width(2.0)                    // 2dB soft knee
    .with_attack(Duration::from_millis(3))   // Fast 3ms attack
    .with_release(Duration::from_millis(50)); // 50ms release

let limited = source.limit(settings);
```

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#517-525)

#### fn [linear\_gain\_ramp](#method.linear_gain_ramp)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), start\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), end\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), clamp\_end: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [LinearGainRamp](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/linear_ramp/struct.LinearGainRamp.html "struct rodio::source::linear_ramp::LinearGainRamp")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a linear gain ramp to the sound.

If `clamp_end` is `true`, all samples subsequent to the end of the ramp will be scaled by the `end_value`. If `clamp_end` is `false`, all subsequent samples will not have any scaling applied.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#541-544)

#### fn [periodic\_access](#method.periodic_access)<F>( self, period: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), access: F, ) -> [PeriodicAccess](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html "struct rodio::source::periodic::PeriodicAccess")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut Self),

Calls the `access` closure on `Self` the first time the source is iterated and every time `period` elapses.

Later changes in either `sample_rate()` or `channels_count()` won’t be reflected in the rate of access.

The rate is based on playback speed, so both the following will call `access` when the same samples are reached: `periodic_access(Duration::from_secs(1), ...).speed(2.0)` `speed(2.0).periodic_access(Duration::from_secs(2), ...)`

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#564-566)

#### fn [speed](#method.speed)(self, ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Speed](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html "struct rodio::source::speed::Speed")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Changes the play speed of the sound. Does not adjust the samples, only the playback speed.

##### Note:

1.  **Increasing the speed will increase the pitch by the same factor**

*   If you set the speed to 0.5 this will halve the frequency of the sound lowering its pitch.
*   If you set the speed to 2 the frequency will double raising the pitch of the sound.

2.  **Change in the speed affect the total duration inversely**

*   If you set the speed to 0.5, the total duration will be twice as long.
*   If you set the speed to 2 the total duration will be halve of what it was.

See [`Speed`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html "struct rodio::source::speed::Speed") for details

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#591-593)

#### fn [record](#method.record)(self) -> [SamplesBuffer](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html "struct rodio::buffer::SamplesBuffer") [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the source and returns a SamplesBuffer

Use `take_duration` on infinite sources (like the microphone source) before calling `record` to prevent this from hanging forever.

##### Note

As `SamplesBuffer` only supports a single _samplerate_ and _channel count_ all samples are resampled to the initial samplerate and channel count is.

##### Example

```rust
let wave = SineWave::new(740.0)
    .amplify(0.2)
    .take_duration(Duration::from_secs(3));
let wave = wave.record();
```

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#611-613)

#### fn [reverb](#method.reverb)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), amplitude: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<[Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self>>> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Adds a basic reverb effect.

This function requires the source to implement `Clone`. This can be done by using `buffered()`.

##### Example

[ⓘ](# "This example is not tested")

```rust
use std::time::Duration;

let source = source.buffered().reverb(Duration::from_millis(100), 0.7);
```

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#622-624)

#### fn [pausable](#method.pausable)(self, initially\_paused: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Pausable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html "struct rodio::source::pausable::Pausable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound pausable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#632-634)

#### fn [stoppable](#method.stoppable)(self) -> [Stoppable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html "struct rodio::source::stoppable::Stoppable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound stoppable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#642-644)

#### fn [skippable](#method.skippable)(self) -> [Skippable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html "struct rodio::source::skippable::Skippable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Adds a method [`Skippable::skip`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html#method.skip "method rodio::source::skippable::Skippable::skip") for skipping this source. Skipping makes Source::next() return None. Which in turn makes the Player skip to the next source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#659-661)

#### fn [track\_position](#method.track_position)(self) -> [TrackPosition](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html "struct rodio::source::position::TrackPosition")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Start tracking the elapsed duration since the start of the underlying source.

If a speedup and or delay is applied after this that will not be reflected in the position returned by [`get_pos`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html#method.get_pos "method rodio::source::position::TrackPosition::get_pos").

This can get confusing when using [`get_pos()`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html#method.get_pos "method rodio::source::position::TrackPosition::get_pos") together with [`Source::try_seek()`](trait.Source.html#method.try_seek "method bevy::audio::Source::try_seek") as the latter does take all speedup’s and delay’s into account. It’s recommended therefore to apply track\_position after speedup’s and delay’s.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#669-672)

#### fn [low\_pass](#method.low_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source. **Warning**: Probably buggy.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#679-682)

#### fn [high\_pass](#method.high_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#689-692)

#### fn [low\_pass\_with\_q](#method.low_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#699-702)

#### fn [high\_pass\_with\_q](#method.high_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#709-711)

#### fn [distortion](#method.distortion)(self, gain: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), threshold: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Distortion](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html "struct rodio::source::distortion::Distortion")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a distortion effect to the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#737)

#### fn [try\_seek](#method.try_seek)(&mut self, pos: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SeekError](enum.SeekError.html "enum bevy::audio::SeekError")\>

Attempts to seek to a given position in the current source.

As long as the duration of the source is known, seek is guaranteed to saturate at the end of the source. For example given a source that reports a total duration of 42 seconds calling `try_seek()` with 60 seconds as argument will seek to 42 seconds.

##### Errors

This function will return [`SeekError::NotSupported`](enum.SeekError.html#variant.NotSupported "variant bevy::audio::SeekError::NotSupported") if one of the underlying sources does not support seeking.

It will return an error if an implementation ran into one during the seek.

Seeking beyond the end of a source might return an error if the total duration of the source is not known.

## Trait Implementations

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

#### fn [current\_span\_len](trait.Source.html#tymethod.current_span_len)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the number of samples before the current span ends. [Read more](trait.Source.html#tymethod.current_span_len)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

#### fn [channels](trait.Source.html#tymethod.channels)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

Returns the number of channels. Channels are always interleaved. Should never be Zero

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

#### fn [sample\_rate](trait.Source.html#tymethod.sample_rate)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Returns the rate at which the source should be played. In number of samples per second.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

#### fn [total\_duration](trait.Source.html#tymethod.total_duration)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>

Returns the total duration of this source, if known. [Read more](trait.Source.html#tymethod.total_duration)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

#### fn [try\_seek](trait.Source.html#method.try_seek)(&mut self, pos: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SeekError](enum.SeekError.html "enum bevy::audio::SeekError")\>

Attempts to seek to a given position in the current source. [Read more](trait.Source.html#method.try_seek)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#187)

#### fn [is\_exhausted](trait.Source.html#method.is_exhausted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the source is exhausted (has no more samples available).

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#205-207)

#### fn [buffered](trait.Source.html#method.buffered)(self) -> [Buffered](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html "struct rodio::source::buffered::Buffered")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Stores the source in a buffer in addition to returning it. This iterator can be cloned.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#238-241)

#### fn [mix](trait.Source.html#method.mix)<S>(self, other: S) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, S> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Source](trait.Source.html "trait bevy::audio::Source"),

Mixes this source with another one.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#251-253)

#### fn [repeat\_infinite](trait.Source.html#method.repeat_infinite)(self) -> [Repeat](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html "struct rodio::source::repeat::Repeat")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Repeats this source forever. [Read more](trait.Source.html#method.repeat_infinite)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#260-262)

#### fn [take\_duration](trait.Source.html#method.take_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a certain duration of this source and then stops.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#272-274)

#### fn [delay](trait.Source.html#method.delay)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Delays the sound by a certain duration. [Read more](trait.Source.html#method.delay)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#283-285)

#### fn [skip\_duration](trait.Source.html#method.skip_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [SkipDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html "struct rodio::source::skip::SkipDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immediately skips a certain duration of this source. [Read more](trait.Source.html#method.skip_duration)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#292-294)

#### fn [amplify](trait.Source.html#method.amplify)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#301-303)

#### fn [amplify\_decibel](trait.Source.html#method.amplify_decibel)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound logarithmically by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#315-317)

#### fn [amplify\_normalized](trait.Source.html#method.amplify_normalized)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Normalized amplification in `[0.0, 1.0]` range. This method better matches the perceived loudness of sounds in human hearing and is recommended to use when you want to change volume in `[0.0, 1.0]` range. based on article: [https://www.dr-lex.be/info-stuff/volumecontrols.html](https://www.dr-lex.be/info-stuff/volumecontrols.html) [Read more](trait.Source.html#method.amplify_normalized)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#407-412)

#### fn [automatic\_gain\_control](trait.Source.html#method.automatic_gain_control)( self, agc\_settings: [AutomaticGainControlSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControlSettings.html "struct rodio::source::agc::AutomaticGainControlSettings"), ) -> [AutomaticGainControl](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html "struct rodio::source::agc::AutomaticGainControl")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies automatic gain control to the sound. [Read more](trait.Source.html#method.automatic_gain_control)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#431-434)

#### fn [take\_crossfade\_with](trait.Source.html#method.take_crossfade_with)<S>( self, other: S, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self>, [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<S>>> [ⓘ](#)

where S: [Source](trait.Source.html "trait bevy::audio::Source"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<<S as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Mixes this sound fading out with another sound fading in for the given duration. [Read more](trait.Source.html#method.take_crossfade_with)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#441-443)

#### fn [fade\_in](trait.Source.html#method.fade_in)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades in the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#450-452)

#### fn [fade\_out](trait.Source.html#method.fade_out)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeOut](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html "struct rodio::source::fadeout::FadeOut")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades out the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#504-506)

#### fn [limit](trait.Source.html#method.limit)(self, settings: [LimitSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.LimitSettings.html "struct rodio::source::limit::LimitSettings")) -> [Limit](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html "struct rodio::source::limit::Limit")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies limiting to prevent audio peaks from exceeding a threshold. [Read more](trait.Source.html#method.limit)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#517-525)

#### fn [linear\_gain\_ramp](trait.Source.html#method.linear_gain_ramp)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), start\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), end\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), clamp\_end: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [LinearGainRamp](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/linear_ramp/struct.LinearGainRamp.html "struct rodio::source::linear_ramp::LinearGainRamp")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a linear gain ramp to the sound. [Read more](trait.Source.html#method.linear_gain_ramp)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#541-544)

#### fn [periodic\_access](trait.Source.html#method.periodic_access)<F>( self, period: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), access: F, ) -> [PeriodicAccess](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html "struct rodio::source::periodic::PeriodicAccess")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut Self),

Calls the `access` closure on `Self` the first time the source is iterated and every time `period` elapses. [Read more](trait.Source.html#method.periodic_access)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#564-566)

#### fn [speed](trait.Source.html#method.speed)(self, ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Speed](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html "struct rodio::source::speed::Speed")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Changes the play speed of the sound. Does not adjust the samples, only the playback speed. [Read more](trait.Source.html#method.speed)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#591-593)

#### fn [record](trait.Source.html#method.record)(self) -> [SamplesBuffer](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html "struct rodio::buffer::SamplesBuffer") [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the source and returns a SamplesBuffer [Read more](trait.Source.html#method.record)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#611-613)

#### fn [reverb](trait.Source.html#method.reverb)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), amplitude: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<[Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self>>> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Adds a basic reverb effect. [Read more](trait.Source.html#method.reverb)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#622-624)

#### fn [pausable](trait.Source.html#method.pausable)(self, initially\_paused: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Pausable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html "struct rodio::source::pausable::Pausable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound pausable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#632-634)

#### fn [stoppable](trait.Source.html#method.stoppable)(self) -> [Stoppable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html "struct rodio::source::stoppable::Stoppable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound stoppable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#642-644)

#### fn [skippable](trait.Source.html#method.skippable)(self) -> [Skippable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html "struct rodio::source::skippable::Skippable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Adds a method [`Skippable::skip`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html#method.skip "method rodio::source::skippable::Skippable::skip") for skipping this source. Skipping makes Source::next() return None. Which in turn makes the Player skip to the next source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#659-661)

#### fn [track\_position](trait.Source.html#method.track_position)(self) -> [TrackPosition](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html "struct rodio::source::position::TrackPosition")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Start tracking the elapsed duration since the start of the underlying source. [Read more](trait.Source.html#method.track_position)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#669-672)

#### fn [low\_pass](trait.Source.html#method.low_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source. **Warning**: Probably buggy.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#679-682)

#### fn [high\_pass](trait.Source.html#method.high_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#689-692)

#### fn [low\_pass\_with\_q](trait.Source.html#method.low_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#699-702)

#### fn [high\_pass\_with\_q](trait.Source.html#method.high_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#709-711)

#### fn [distortion](trait.Source.html#method.distortion)(self, gain: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), threshold: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Distortion](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html "struct rodio::source::distortion::Distortion")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a distortion effect to the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

#### fn [current\_span\_len](trait.Source.html#tymethod.current_span_len)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the number of samples before the current span ends. [Read more](trait.Source.html#tymethod.current_span_len)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

#### fn [channels](trait.Source.html#tymethod.channels)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

Returns the number of channels. Channels are always interleaved. Should never be Zero

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

#### fn [sample\_rate](trait.Source.html#tymethod.sample_rate)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Returns the rate at which the source should be played. In number of samples per second.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

#### fn [total\_duration](trait.Source.html#tymethod.total_duration)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>

Returns the total duration of this source, if known. [Read more](trait.Source.html#tymethod.total_duration)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

#### fn [try\_seek](trait.Source.html#method.try_seek)(&mut self, pos: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SeekError](enum.SeekError.html "enum bevy::audio::SeekError")\>

Attempts to seek to a given position in the current source. [Read more](trait.Source.html#method.try_seek)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#187)

#### fn [is\_exhausted](trait.Source.html#method.is_exhausted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the source is exhausted (has no more samples available).

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#205-207)

#### fn [buffered](trait.Source.html#method.buffered)(self) -> [Buffered](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html "struct rodio::source::buffered::Buffered")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Stores the source in a buffer in addition to returning it. This iterator can be cloned.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#238-241)

#### fn [mix](trait.Source.html#method.mix)<S>(self, other: S) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, S> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Source](trait.Source.html "trait bevy::audio::Source"),

Mixes this source with another one.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#251-253)

#### fn [repeat\_infinite](trait.Source.html#method.repeat_infinite)(self) -> [Repeat](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html "struct rodio::source::repeat::Repeat")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Repeats this source forever. [Read more](trait.Source.html#method.repeat_infinite)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#260-262)

#### fn [take\_duration](trait.Source.html#method.take_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a certain duration of this source and then stops.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#272-274)

#### fn [delay](trait.Source.html#method.delay)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Delays the sound by a certain duration. [Read more](trait.Source.html#method.delay)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#283-285)

#### fn [skip\_duration](trait.Source.html#method.skip_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [SkipDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html "struct rodio::source::skip::SkipDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immediately skips a certain duration of this source. [Read more](trait.Source.html#method.skip_duration)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#292-294)

#### fn [amplify](trait.Source.html#method.amplify)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#301-303)

#### fn [amplify\_decibel](trait.Source.html#method.amplify_decibel)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound logarithmically by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#315-317)

#### fn [amplify\_normalized](trait.Source.html#method.amplify_normalized)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Normalized amplification in `[0.0, 1.0]` range. This method better matches the perceived loudness of sounds in human hearing and is recommended to use when you want to change volume in `[0.0, 1.0]` range. based on article: [https://www.dr-lex.be/info-stuff/volumecontrols.html](https://www.dr-lex.be/info-stuff/volumecontrols.html) [Read more](trait.Source.html#method.amplify_normalized)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#407-412)

#### fn [automatic\_gain\_control](trait.Source.html#method.automatic_gain_control)( self, agc\_settings: [AutomaticGainControlSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControlSettings.html "struct rodio::source::agc::AutomaticGainControlSettings"), ) -> [AutomaticGainControl](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html "struct rodio::source::agc::AutomaticGainControl")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies automatic gain control to the sound. [Read more](trait.Source.html#method.automatic_gain_control)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#431-434)

#### fn [take\_crossfade\_with](trait.Source.html#method.take_crossfade_with)<S>( self, other: S, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self>, [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<S>>> [ⓘ](#)

where S: [Source](trait.Source.html "trait bevy::audio::Source"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<<S as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Mixes this sound fading out with another sound fading in for the given duration. [Read more](trait.Source.html#method.take_crossfade_with)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#441-443)

#### fn [fade\_in](trait.Source.html#method.fade_in)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades in the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#450-452)

#### fn [fade\_out](trait.Source.html#method.fade_out)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeOut](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html "struct rodio::source::fadeout::FadeOut")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades out the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#504-506)

#### fn [limit](trait.Source.html#method.limit)(self, settings: [LimitSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.LimitSettings.html "struct rodio::source::limit::LimitSettings")) -> [Limit](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html "struct rodio::source::limit::Limit")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies limiting to prevent audio peaks from exceeding a threshold. [Read more](trait.Source.html#method.limit)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#517-525)

#### fn [linear\_gain\_ramp](trait.Source.html#method.linear_gain_ramp)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), start\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), end\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), clamp\_end: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [LinearGainRamp](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/linear_ramp/struct.LinearGainRamp.html "struct rodio::source::linear_ramp::LinearGainRamp")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a linear gain ramp to the sound. [Read more](trait.Source.html#method.linear_gain_ramp)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#541-544)

#### fn [periodic\_access](trait.Source.html#method.periodic_access)<F>( self, period: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), access: F, ) -> [PeriodicAccess](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html "struct rodio::source::periodic::PeriodicAccess")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut Self),

Calls the `access` closure on `Self` the first time the source is iterated and every time `period` elapses. [Read more](trait.Source.html#method.periodic_access)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#564-566)

#### fn [speed](trait.Source.html#method.speed)(self, ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Speed](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html "struct rodio::source::speed::Speed")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Changes the play speed of the sound. Does not adjust the samples, only the playback speed. [Read more](trait.Source.html#method.speed)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#591-593)

#### fn [record](trait.Source.html#method.record)(self) -> [SamplesBuffer](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html "struct rodio::buffer::SamplesBuffer") [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the source and returns a SamplesBuffer [Read more](trait.Source.html#method.record)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#611-613)

#### fn [reverb](trait.Source.html#method.reverb)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), amplitude: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<[Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self>>> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Adds a basic reverb effect. [Read more](trait.Source.html#method.reverb)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#622-624)

#### fn [pausable](trait.Source.html#method.pausable)(self, initially\_paused: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Pausable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html "struct rodio::source::pausable::Pausable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound pausable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#632-634)

#### fn [stoppable](trait.Source.html#method.stoppable)(self) -> [Stoppable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html "struct rodio::source::stoppable::Stoppable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound stoppable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#642-644)

#### fn [skippable](trait.Source.html#method.skippable)(self) -> [Skippable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html "struct rodio::source::skippable::Skippable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Adds a method [`Skippable::skip`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html#method.skip "method rodio::source::skippable::Skippable::skip") for skipping this source. Skipping makes Source::next() return None. Which in turn makes the Player skip to the next source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#659-661)

#### fn [track\_position](trait.Source.html#method.track_position)(self) -> [TrackPosition](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html "struct rodio::source::position::TrackPosition")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Start tracking the elapsed duration since the start of the underlying source. [Read more](trait.Source.html#method.track_position)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#669-672)

#### fn [low\_pass](trait.Source.html#method.low_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source. **Warning**: Probably buggy.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#679-682)

#### fn [high\_pass](trait.Source.html#method.high_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#689-692)

#### fn [low\_pass\_with\_q](trait.Source.html#method.low_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#699-702)

#### fn [high\_pass\_with\_q](trait.Source.html#method.high_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#709-711)

#### fn [distortion](trait.Source.html#method.distortion)(self, gain: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), threshold: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Distortion](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html "struct rodio::source::distortion::Distortion")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a distortion effect to the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

#### fn [current\_span\_len](trait.Source.html#tymethod.current_span_len)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the number of samples before the current span ends. [Read more](trait.Source.html#tymethod.current_span_len)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

#### fn [channels](trait.Source.html#tymethod.channels)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

Returns the number of channels. Channels are always interleaved. Should never be Zero

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

#### fn [sample\_rate](trait.Source.html#tymethod.sample_rate)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Returns the rate at which the source should be played. In number of samples per second.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

#### fn [total\_duration](trait.Source.html#tymethod.total_duration)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>

Returns the total duration of this source, if known. [Read more](trait.Source.html#tymethod.total_duration)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

#### fn [try\_seek](trait.Source.html#method.try_seek)(&mut self, pos: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SeekError](enum.SeekError.html "enum bevy::audio::SeekError")\>

Attempts to seek to a given position in the current source. [Read more](trait.Source.html#method.try_seek)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#187)

#### fn [is\_exhausted](trait.Source.html#method.is_exhausted)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the source is exhausted (has no more samples available).

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#205-207)

#### fn [buffered](trait.Source.html#method.buffered)(self) -> [Buffered](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html "struct rodio::source::buffered::Buffered")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Stores the source in a buffer in addition to returning it. This iterator can be cloned.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#238-241)

#### fn [mix](trait.Source.html#method.mix)<S>(self, other: S) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, S> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), S: [Source](trait.Source.html "trait bevy::audio::Source"),

Mixes this source with another one.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#251-253)

#### fn [repeat\_infinite](trait.Source.html#method.repeat_infinite)(self) -> [Repeat](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html "struct rodio::source::repeat::Repeat")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Repeats this source forever. [Read more](trait.Source.html#method.repeat_infinite)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#260-262)

#### fn [take\_duration](trait.Source.html#method.take_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Takes a certain duration of this source and then stops.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#272-274)

#### fn [delay](trait.Source.html#method.delay)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Delays the sound by a certain duration. [Read more](trait.Source.html#method.delay)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#283-285)

#### fn [skip\_duration](trait.Source.html#method.skip_duration)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [SkipDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html "struct rodio::source::skip::SkipDuration")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immediately skips a certain duration of this source. [Read more](trait.Source.html#method.skip_duration)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#292-294)

#### fn [amplify](trait.Source.html#method.amplify)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#301-303)

#### fn [amplify\_decibel](trait.Source.html#method.amplify_decibel)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Amplifies the sound logarithmically by the given value.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#315-317)

#### fn [amplify\_normalized](trait.Source.html#method.amplify_normalized)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Normalized amplification in `[0.0, 1.0]` range. This method better matches the perceived loudness of sounds in human hearing and is recommended to use when you want to change volume in `[0.0, 1.0]` range. based on article: [https://www.dr-lex.be/info-stuff/volumecontrols.html](https://www.dr-lex.be/info-stuff/volumecontrols.html) [Read more](trait.Source.html#method.amplify_normalized)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#407-412)

#### fn [automatic\_gain\_control](trait.Source.html#method.automatic_gain_control)( self, agc\_settings: [AutomaticGainControlSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControlSettings.html "struct rodio::source::agc::AutomaticGainControlSettings"), ) -> [AutomaticGainControl](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html "struct rodio::source::agc::AutomaticGainControl")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies automatic gain control to the sound. [Read more](trait.Source.html#method.automatic_gain_control)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#431-434)

#### fn [take\_crossfade\_with](trait.Source.html#method.take_crossfade_with)<S>( self, other: S, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<Self>, [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<[TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<S>>> [ⓘ](#)

where S: [Source](trait.Source.html "trait bevy::audio::Source"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<<S as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>,

Mixes this sound fading out with another sound fading in for the given duration. [Read more](trait.Source.html#method.take_crossfade_with)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#441-443)

#### fn [fade\_in](trait.Source.html#method.fade_in)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades in the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#450-452)

#### fn [fade\_out](trait.Source.html#method.fade_out)(self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [FadeOut](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html "struct rodio::source::fadeout::FadeOut")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fades out the sound.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#504-506)

#### fn [limit](trait.Source.html#method.limit)(self, settings: [LimitSettings](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.LimitSettings.html "struct rodio::source::limit::LimitSettings")) -> [Limit](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html "struct rodio::source::limit::Limit")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies limiting to prevent audio peaks from exceeding a threshold. [Read more](trait.Source.html#method.limit)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#517-525)

#### fn [linear\_gain\_ramp](trait.Source.html#method.linear_gain_ramp)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), start\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), end\_value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), clamp\_end: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [LinearGainRamp](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/linear_ramp/struct.LinearGainRamp.html "struct rodio::source::linear_ramp::LinearGainRamp")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a linear gain ramp to the sound. [Read more](trait.Source.html#method.linear_gain_ramp)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#541-544)

#### fn [periodic\_access](trait.Source.html#method.periodic_access)<F>( self, period: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), access: F, ) -> [PeriodicAccess](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html "struct rodio::source::periodic::PeriodicAccess")<Self, F> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut Self),

Calls the `access` closure on `Self` the first time the source is iterated and every time `period` elapses. [Read more](trait.Source.html#method.periodic_access)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#564-566)

#### fn [speed](trait.Source.html#method.speed)(self, ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Speed](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html "struct rodio::source::speed::Speed")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Changes the play speed of the sound. Does not adjust the samples, only the playback speed. [Read more](trait.Source.html#method.speed)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#591-593)

#### fn [record](trait.Source.html#method.record)(self) -> [SamplesBuffer](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html "struct rodio::buffer::SamplesBuffer") [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Consumes the source and returns a SamplesBuffer [Read more](trait.Source.html#method.record)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#611-613)

#### fn [reverb](trait.Source.html#method.reverb)( self, duration: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration"), amplitude: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<Self, [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<[Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<Self>>> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Adds a basic reverb effect. [Read more](trait.Source.html#method.reverb)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#622-624)

#### fn [pausable](trait.Source.html#method.pausable)(self, initially\_paused: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Pausable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html "struct rodio::source::pausable::Pausable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound pausable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#632-634)

#### fn [stoppable](trait.Source.html#method.stoppable)(self) -> [Stoppable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html "struct rodio::source::stoppable::Stoppable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Makes the sound stoppable.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#642-644)

#### fn [skippable](trait.Source.html#method.skippable)(self) -> [Skippable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html "struct rodio::source::skippable::Skippable")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Adds a method [`Skippable::skip`](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html#method.skip "method rodio::source::skippable::Skippable::skip") for skipping this source. Skipping makes Source::next() return None. Which in turn makes the Player skip to the next source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#659-661)

#### fn [track\_position](trait.Source.html#method.track_position)(self) -> [TrackPosition](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html "struct rodio::source::position::TrackPosition")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Start tracking the elapsed duration since the start of the underlying source. [Read more](trait.Source.html#method.track_position)

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#669-672)

#### fn [low\_pass](trait.Source.html#method.low_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source. **Warning**: Probably buggy.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#679-682)

#### fn [high\_pass](trait.Source.html#method.high_pass)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#689-692)

#### fn [low\_pass\_with\_q](trait.Source.html#method.low_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a low-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#699-702)

#### fn [high\_pass\_with\_q](trait.Source.html#method.high_pass_with_q)(self, freq: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), q: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Applies a high-pass filter to the source while allowing the q (bandwidth) to be changed.

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#709-711)

#### fn [distortion](trait.Source.html#method.distortion)(self, gain: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), threshold: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Distortion](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html "struct rodio::source::distortion::Distortion")<Self> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Applies a distortion effect to the sound.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#832)

### impl<'a, Src> [Source](trait.Source.html "trait bevy::audio::Source") for [&'a mut Src](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where Src: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#832)

#### fn [current\_span\_len](#tymethod.current_span_len)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#832)

#### fn [channels](#tymethod.channels)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#832)

#### fn [sample\_rate](#tymethod.sample_rate)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#832)

#### fn [total\_duration](#tymethod.total_duration)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#832)

#### fn [try\_seek](#method.try_seek)(&mut self, pos: [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [SeekError](enum.SeekError.html "enum bevy::audio::SeekError")\>

## Implementors

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#830)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#828)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mod.rs.html#826)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/chirp.rs.html#87)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Chirp](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/chirp/struct.Chirp.html "struct rodio::source::chirp::Chirp")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/empty.rs.html#37)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Empty](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/empty/struct.Empty.html "struct rodio::source::empty::Empty")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/empty_callback.rs.html#34)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [EmptyCallback](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/empty_callback/struct.EmptyCallback.html "struct rodio::source::empty_callback::EmptyCallback")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/mixer.rs.html#83)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [MixerSource](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/mixer/struct.MixerSource.html "struct rodio::mixer::MixerSource")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/buffer.rs.html#74)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [SamplesBuffer](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html "struct rodio::buffer::SamplesBuffer")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/sawtooth.rs.html#41)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [SawtoothWave](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/sawtooth/struct.SawtoothWave.html "struct rodio::source::sawtooth::SawtoothWave")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/signal_generator.rs.html#138)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [SignalGenerator](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/signal_generator/struct.SignalGenerator.html "struct rodio::source::signal_generator::SignalGenerator")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/sine.rs.html#41)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [SineWave](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/sine/struct.SineWave.html "struct rodio::source::sine::SineWave")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/queue.rs.html#135)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [SourcesQueueOutput](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/queue/struct.SourcesQueueOutput.html "struct rodio::queue::SourcesQueueOutput")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/square.rs.html#41)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [SquareWave](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/square/struct.SquareWave.html "struct rodio::source::square::SquareWave")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/static_buffer.rs.html#74)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [StaticSamplesBuffer](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/static_buffer/struct.StaticSamplesBuffer.html "struct rodio::static_buffer::StaticSamplesBuffer")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/triangle.rs.html#41)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [TriangleWave](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/triangle/struct.TriangleWave.html "struct rodio::source::triangle::TriangleWave")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/zero.rs.html#76)

### impl [Source](trait.Source.html "trait bevy::audio::Source") for [Zero](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/zero/struct.Zero.html "struct rodio::source::zero::Zero")

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/mix.rs.html#77-80)

### impl<I1, I2> [Source](trait.Source.html "trait bevy::audio::Source") for [Mix](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html "struct rodio::source::mix::Mix")<I1, I2>

where I1: [Source](trait.Source.html "trait bevy::audio::Source"), I2: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/periodic.rs.html#94-98)

### impl<I, F> [Source](trait.Source.html "trait bevy::audio::Source") for [PeriodicAccess](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html "struct rodio::source::periodic::PeriodicAccess")<I, F>

where I: [Source](trait.Source.html "trait bevy::audio::Source"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut I](https://doc.rust-lang.org/nightly/std/primitive.reference.html)),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/amplify.rs.html#75-77)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Amplify](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html "struct rodio::source::amplify::Amplify")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/agc.rs.html#520-522)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [AutomaticGainControl](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html "struct rodio::source::agc::AutomaticGainControl")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/blt.rs.html#152-154)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [BltFilter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html "struct rodio::source::blt::BltFilter")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/buffered.rs.html#201-203)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Buffered](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html "struct rodio::source::buffered::Buffered")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/channel_volume.rs.html#97-99)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [ChannelVolume](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/channel_volume/struct.ChannelVolume.html "struct rodio::source::channel_volume::ChannelVolume")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/delay.rs.html#87-89)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Delay](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html "struct rodio::source::delay::Delay")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/distortion.rs.html#82-84)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Distortion](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html "struct rodio::source::distortion::Distortion")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/done.rs.html#69-71)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Done](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/done/struct.Done.html "struct rodio::source::done::Done")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/fadein.rs.html#65-67)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [FadeIn](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html "struct rodio::source::fadein::FadeIn")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/fadeout.rs.html#65-67)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [FadeOut](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html "struct rodio::source::fadeout::FadeOut")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/from_iter.rs.html#74-77)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [FromIter](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/from_iter/struct.FromIter.html "struct rodio::source::from_iter::FromIter")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"), <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/limit.rs.html#575-577)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Limit](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html "struct rodio::source::limit::Limit")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/linear_ramp.rs.html#109-111)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [LinearGainRamp](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/linear_ramp/struct.LinearGainRamp.html "struct rodio::source::linear_ramp::LinearGainRamp")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/pausable.rs.html#105-107)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Pausable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html "struct rodio::source::pausable::Pausable")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/repeat.rs.html#53-55)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Repeat](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html "struct rodio::source::repeat::Repeat")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/skip.rs.html#132-134)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [SkipDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html "struct rodio::source::skip::SkipDuration")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/skippable.rs.html#73-75)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Skippable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html "struct rodio::source::skippable::Skippable")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/spatial.rs.html#91-93)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Spatial](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/spatial/struct.Spatial.html "struct rodio::source::spatial::Spatial")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/speed.rs.html#115-117)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Speed](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html "struct rodio::source::speed::Speed")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/stoppable.rs.html#69-71)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [Stoppable](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html "struct rodio::source::stoppable::Stoppable")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/take.rs.html#131-133)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [TakeDuration](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html "struct rodio::source::take::TakeDuration")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/position.rs.html#121-123)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [TrackPosition](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html "struct rodio::source::position::TrackPosition")<I>

where I: [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/source/uniform.rs.html#98-100)

### impl<I> [Source](trait.Source.html "trait bevy::audio::Source") for [UniformSourceIterator](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/source/uniform/struct.UniformSourceIterator.html "struct rodio::source::uniform::UniformSourceIterator")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") + [Source](trait.Source.html "trait bevy::audio::Source"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/decoder/mod.rs.html#577-579)

### impl<R> [Source](trait.Source.html "trait bevy::audio::Source") for [Decoder](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/decoder/struct.Decoder.html "struct rodio::decoder::Decoder")<R>

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [Seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek"),

[Source](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/src/rodio/decoder/mod.rs.html#698-700)

### impl<R> [Source](trait.Source.html "trait bevy::audio::Source") for [LoopedDecoder](https://docs.rs/rodio/0.22.2/x86_64-unknown-linux-gnu/rodio/decoder/struct.LoopedDecoder.html "struct rodio::decoder::LoopedDecoder")<R>

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [Seek](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek"),

{"Amplify<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html\\" title=\\"struct rodio::source::amplify::Amplify\\">Amplify</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/amplify/struct.Amplify.html\\" title=\\"struct rodio::source::amplify::Amplify\\">Amplify</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","AutomaticGainControl<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html\\" title=\\"struct rodio::source::agc::AutomaticGainControl\\">AutomaticGainControl</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/agc/struct.AutomaticGainControl.html\\" title=\\"struct rodio::source::agc::AutomaticGainControl\\">AutomaticGainControl</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","BltFilter<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html\\" title=\\"struct rodio::source::blt::BltFilter\\">BltFilter</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/blt/struct.BltFilter.html\\" title=\\"struct rodio::source::blt::BltFilter\\">BltFilter</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>&lt;Item = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\\">f32</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\\">f32</a>;</div>","Buffered<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html\\" title=\\"struct rodio::source::buffered::Buffered\\">Buffered</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/buffered/struct.Buffered.html\\" title=\\"struct rodio::source::buffered::Buffered\\">Buffered</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Delay<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html\\" title=\\"struct rodio::source::delay::Delay\\">Delay</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/delay/struct.Delay.html\\" title=\\"struct rodio::source::delay::Delay\\">Delay</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Distortion<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html\\" title=\\"struct rodio::source::distortion::Distortion\\">Distortion</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/distortion/struct.Distortion.html\\" title=\\"struct rodio::source::distortion::Distortion\\">Distortion</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","FadeIn<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html\\" title=\\"struct rodio::source::fadein::FadeIn\\">FadeIn</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/fadein/struct.FadeIn.html\\" title=\\"struct rodio::source::fadein::FadeIn\\">FadeIn</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","FadeOut<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html\\" title=\\"struct rodio::source::fadeout::FadeOut\\">FadeOut</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/fadeout/struct.FadeOut.html\\" title=\\"struct rodio::source::fadeout::FadeOut\\">FadeOut</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Limit<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html\\" title=\\"struct rodio::source::limit::Limit\\">Limit</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/limit/struct.Limit.html\\" title=\\"struct rodio::source::limit::Limit\\">Limit</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","LinearGainRamp<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/linear\_ramp/struct.LinearGainRamp.html\\" title=\\"struct rodio::source::linear\_ramp::LinearGainRamp\\">LinearGainRamp</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/linear\_ramp/struct.LinearGainRamp.html\\" title=\\"struct rodio::source::linear\_ramp::LinearGainRamp\\">LinearGainRamp</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Mix<Self, Delay<Amplify<Self>>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html\\" title=\\"struct rodio::source::mix::Mix\\">Mix</a>&lt;I1, I2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I1, I2&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html\\" title=\\"struct rodio::source::mix::Mix\\">Mix</a>&lt;I1, I2&gt;<div class=\\"where\\">where\\n I1: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,\\n I2: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I1 as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Mix<Self, S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html\\" title=\\"struct rodio::source::mix::Mix\\">Mix</a>&lt;I1, I2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I1, I2&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html\\" title=\\"struct rodio::source::mix::Mix\\">Mix</a>&lt;I1, I2&gt;<div class=\\"where\\">where\\n I1: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,\\n I2: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I1 as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Mix<TakeDuration<Self>, FadeIn<TakeDuration<S>>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html\\" title=\\"struct rodio::source::mix::Mix\\">Mix</a>&lt;I1, I2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I1, I2&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/mix/struct.Mix.html\\" title=\\"struct rodio::source::mix::Mix\\">Mix</a>&lt;I1, I2&gt;<div class=\\"where\\">where\\n I1: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,\\n I2: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I1 as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Pausable<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html\\" title=\\"struct rodio::source::pausable::Pausable\\">Pausable</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/pausable/struct.Pausable.html\\" title=\\"struct rodio::source::pausable::Pausable\\">Pausable</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","PeriodicAccess<Self, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html\\" title=\\"struct rodio::source::periodic::PeriodicAccess\\">PeriodicAccess</a>&lt;I, F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, F&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/periodic/struct.PeriodicAccess.html\\" title=\\"struct rodio::source::periodic::PeriodicAccess\\">PeriodicAccess</a>&lt;I, F&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut I</a>),</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Repeat<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html\\" title=\\"struct rodio::source::repeat::Repeat\\">Repeat</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/repeat/struct.Repeat.html\\" title=\\"struct rodio::source::repeat::Repeat\\">Repeat</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","SamplesBuffer":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html\\" title=\\"struct rodio::buffer::SamplesBuffer\\">SamplesBuffer</a></code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/buffer/struct.SamplesBuffer.html\\" title=\\"struct rodio::buffer::SamplesBuffer\\">SamplesBuffer</a></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\\">f32</a>;</div>","SkipDuration<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html\\" title=\\"struct rodio::source::skip::SkipDuration\\">SkipDuration</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/skip/struct.SkipDuration.html\\" title=\\"struct rodio::source::skip::SkipDuration\\">SkipDuration</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Skippable<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html\\" title=\\"struct rodio::source::skippable::Skippable\\">Skippable</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/skippable/struct.Skippable.html\\" title=\\"struct rodio::source::skippable::Skippable\\">Skippable</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Speed<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html\\" title=\\"struct rodio::source::speed::Speed\\">Speed</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/speed/struct.Speed.html\\" title=\\"struct rodio::source::speed::Speed\\">Speed</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","Stoppable<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html\\" title=\\"struct rodio::source::stoppable::Stoppable\\">Stoppable</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/stoppable/struct.Stoppable.html\\" title=\\"struct rodio::source::stoppable::Stoppable\\">Stoppable</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","TakeDuration<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html\\" title=\\"struct rodio::source::take::TakeDuration\\">TakeDuration</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/take/struct.TakeDuration.html\\" title=\\"struct rodio::source::take::TakeDuration\\">TakeDuration</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","TrackPosition<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html\\" title=\\"struct rodio::source::position::TrackPosition\\">TrackPosition</a>&lt;I&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rodio/0.22.2/x86\_64-unknown-linux-gnu/rodio/source/position/struct.TrackPosition.html\\" title=\\"struct rodio::source::position::TrackPosition\\">TrackPosition</a>&lt;I&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"trait.Source.html\\" title=\\"trait bevy::audio::Source\\">Source</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>"}