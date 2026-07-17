use serde::{Deserialize, Serialize};

use super::{
    config::DynamicLightConfig,
    fixed_timestep::FixedTimestep,
    types::DynamicLightEffect,
    unity_math::{clamp01, lerp, move_towards, perlin_noise, unity_clamp},
    unity_random::UnityRandom,
};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct LightEffectRuntime {
    pub(crate) initialized: bool,
    pub(crate) fixed_timestep: FixedTimestep,
    pub(crate) intensity: f32,
    pub(crate) strobe_active: bool,
    pub(crate) fluorescent_random_state: i32,
    pub(crate) fluorescent_random_time: f32,
    pub(crate) elapsed_seconds: f32,
}

impl Default for LightEffectRuntime {
    fn default() -> Self {
        Self {
            initialized: false,
            fixed_timestep: FixedTimestep::default(),
            intensity: 0.0,
            strobe_active: false,
            fluorescent_random_state: 0,
            fluorescent_random_time: 0.0,
            elapsed_seconds: 0.0,
        }
    }
}

/// Advances one manager update in the same order as the frozen upstream
/// DynamicLightManager.UpdateLightEffects implementation.
pub(crate) fn advance_effect(
    config: &DynamicLightConfig,
    runtime: &mut LightEffectRuntime,
    random: &mut UnityRandom,
    delta_seconds: f32,
) {
    runtime.fixed_timestep.time_per_step = config.effect_parameters.timestep_seconds;
    runtime.fixed_timestep.update(delta_seconds);

    let time = runtime.elapsed_seconds;
    let parameters = config.effect_parameters;
    let modifier = parameters.pulse_modifier;

    match config.effect {
        DynamicLightEffect::Steady => runtime.intensity = 1.0,
        DynamicLightEffect::Candle => {
            let flicker_time = parameters.pulse_offset + time;
            let base_flicker = perlin_noise(flicker_time * 0.5, 0.0);
            let random_dip = perlin_noise(flicker_time * 2.0, 10.0);
            let mut target = lerp(
                modifier,
                1.0,
                base_flicker
                    - if random_dip > 0.8 {
                        (random_dip - 0.8) * 2.0
                    } else {
                        0.0
                    },
            );
            if perlin_noise(flicker_time * 5.0, 20.0) > 0.7 {
                target *= modifier;
            }
            let moved = move_towards(runtime.intensity, target, delta_seconds);
            runtime.intensity = unity_clamp(modifier, moved, 1.0);
        }
        DynamicLightEffect::Fire => {
            let flicker_time = parameters.pulse_offset + time;
            let base_flicker = perlin_noise(flicker_time * 0.5, 0.0);
            let chaotic_flicker = perlin_noise(flicker_time * 3.0, 10.0);
            let mut target = lerp(modifier, 1.0, base_flicker * 0.6 + chaotic_flicker * 0.4);
            if perlin_noise(flicker_time * 5.0, 20.0) > 0.7 {
                target = 1.0;
            }
            let moved = move_towards(runtime.intensity, target, delta_seconds * 3.0);
            runtime.intensity = unity_clamp(modifier, moved, 1.0);
        }
        DynamicLightEffect::Generator => {
            let generator_time = parameters.pulse_offset + time * parameters.pulse_speed;
            let flicker = unity_clamp(
                -1.0 + perlin_noise(generator_time, generator_time * 0.1) * 2.5,
                -1.0,
                0.0,
            ) * (1.0 - modifier);
            runtime.intensity = 1.0 + flicker;
        }
        DynamicLightEffect::Lightning => {
            let lightning_time = time + parameters.pulse_offset;
            let slow_wave = 0.5 + (lightning_time * core::f32::consts::TAU).sin() * 0.5;
            runtime.intensity = move_towards(
                runtime.intensity,
                modifier,
                delta_seconds * 10.0 * slow_wave,
            );
            let layers = parameters.pulse_speed.abs().round_ties_even() as i32;
            lightning_layer(runtime, modifier, lightning_time, 0.2);
            if layers >= 2 {
                lightning_layer(runtime, modifier, lightning_time + 121.81, 4.5);
            }
            if layers >= 3 {
                lightning_layer(runtime, modifier, lightning_time + 281.24, 7.8);
            }
        }
        DynamicLightEffect::Pulsar => {
            let pulsar_time = parameters.pulse_offset + time;
            let phase = (pulsar_time * core::f32::consts::TAU * parameters.pulse_speed)
                .sin()
                .max(0.0);
            let target = lerp(modifier, 1.0, phase.powf(5.0));
            runtime.intensity = move_towards(runtime.intensity, target, delta_seconds * 2.0);
        }
        DynamicLightEffect::Pulse => {
            let phase =
                (parameters.pulse_offset + time * parameters.pulse_speed) * core::f32::consts::TAU;
            runtime.intensity = lerp(modifier, 1.0, (1.0 + phase.sin()) * 0.5);
        }
        DynamicLightEffect::FluorescentStarter => {
            let sequence = (parameters.pulse_offset + time) % 3.3;
            runtime.intensity = if sequence < 0.5 {
                (0.25 + (sequence * core::f32::consts::PI * 50.0).sin() * 0.125) * (1.0 - modifier)
            } else if sequence > 2.95 {
                lerp(1.0, 0.0, (sequence - 3.0) * 20.0)
            } else {
                1.0
            };
        }
        DynamicLightEffect::FluorescentClicker => {
            let sequence = (parameters.pulse_offset + time) % 6.0;
            runtime.intensity = if sequence < 0.3 {
                0.1 + (sequence * core::f32::consts::PI * 20.0).sin() * 0.05
            } else if sequence < 1.5 {
                if sequence % 0.2 < 0.05 { 1.0 } else { 0.0 }
            } else if sequence < 4.5 {
                1.0
            } else {
                lerp(1.0, 0.0, (sequence - 4.5) / 0.0625)
            };
        }
        DynamicLightEffect::FluorescentRandom => {
            if time > runtime.fluorescent_random_time {
                runtime.fluorescent_random_state = random.range_i32(0, 3);
                runtime.fluorescent_random_time = time + random.value();
            }
            match runtime.fluorescent_random_state {
                0 => {
                    runtime.intensity =
                        move_towards(runtime.intensity, modifier, delta_seconds * 10.0);
                }
                1 => {
                    runtime.intensity = (0.25
                        + (time * core::f32::consts::PI * 50.0).sin() * 0.125)
                        * (1.0 - modifier);
                }
                2 => {
                    runtime.intensity = move_towards(runtime.intensity, 1.0, delta_seconds * 20.0);
                }
                _ => {}
            }
        }
        DynamicLightEffect::Overcast => {
            let generator_time = parameters.pulse_offset + time * parameters.pulse_speed;
            let mut noise = 0.0;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;
            for _ in 0..3 {
                let t = generator_time * frequency;
                noise += clamp01(perlin_noise(t * 0.1, t * 0.01)) * amplitude;
                frequency *= 2.0;
                amplitude *= 0.5;
            }
            let raw = noise / 1.75;
            let stretched = raw * raw * (3.0 - 2.0 * raw);
            runtime.intensity = lerp(modifier, 1.0, stretched);
        }
        DynamicLightEffect::Cloudy => {
            let generator_time = parameters.pulse_offset + time * parameters.pulse_speed;
            let mut noise = 0.0;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;
            for _ in 0..2 {
                let t = generator_time * frequency;
                let adjusted = clamp01((perlin_noise(t * 0.05, t * 0.005) - 0.5) * 4.0);
                noise += adjusted * amplitude;
                frequency *= 2.0;
                amplitude *= 0.5;
            }
            let inverse = 1.0 - noise / 1.5;
            let stretched = inverse * inverse * (3.0 - 2.0 * inverse);
            runtime.intensity = lerp(modifier, 1.0, stretched);
        }
        DynamicLightEffect::Random | DynamicLightEffect::Strobe | DynamicLightEffect::Flicker => {}
    }

    if runtime.fixed_timestep.pending_steps > 0 || !runtime.initialized {
        runtime.initialized = true;
        match config.effect {
            DynamicLightEffect::Flicker => {
                let first = random.value();
                runtime.intensity = if first < 0.5 {
                    0.0
                } else {
                    lerp(modifier, 1.0, random.value())
                };
            }
            DynamicLightEffect::Random => {
                runtime.intensity = lerp(modifier, 1.0, random.value());
            }
            DynamicLightEffect::Strobe => {
                runtime.strobe_active = !runtime.strobe_active;
                runtime.intensity = if runtime.strobe_active { 1.0 } else { modifier };
            }
            _ => {}
        }
    }

    runtime.elapsed_seconds += delta_seconds;
}

fn lightning_layer(
    runtime: &mut LightEffectRuntime,
    modifier: f32,
    lightning_time: f32,
    unique: f32,
) {
    let slow_wave = 0.5 + ((lightning_time + unique) * core::f32::consts::TAU).sin() * 0.5;
    let remainder = (1.0 - modifier) * slow_wave;
    if perlin_noise(lightning_time * 5.0, unique) > 0.74 {
        runtime.intensity = modifier
            + remainder
            + (lightning_time * core::f32::consts::PI * 40.0).sin() * remainder;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EffectsFixture {
        traces: Vec<EffectTrace>,
    }

    #[derive(Deserialize)]
    struct EffectTrace {
        discriminant: usize,
        seed: i32,
        schedule: String,
        samples: Vec<GoldenSample>,
    }

    #[derive(Deserialize)]
    struct GoldenSample {
        frame: usize,
        delta: f32,
        intensity: f32,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct MultiLightFixture {
        seed: i32,
        effect_discriminants: Vec<usize>,
        frames: Vec<MultiLightFrame>,
    }

    #[derive(Deserialize)]
    struct MultiLightFrame {
        frame: usize,
        intensities: Vec<f32>,
    }

    #[test]
    fn strobe_toggles_once_per_source_fixed_update() {
        let config = DynamicLightConfig {
            effect: DynamicLightEffect::Strobe,
            ..Default::default()
        };
        let mut runtime = LightEffectRuntime::default();
        let mut random = UnityRandom::from_seed(12345);
        advance_effect(&config, &mut runtime, &mut random, 1.0 / 60.0);
        assert_eq!(runtime.intensity, 1.0);
        advance_effect(&config, &mut runtime, &mut random, 1.0 / 60.0);
        assert_eq!(runtime.intensity, 0.25);
    }

    #[test]
    fn random_first_samples_match_unity_fixture() {
        let config = DynamicLightConfig {
            effect: DynamicLightEffect::Random,
            ..Default::default()
        };
        let mut runtime = LightEffectRuntime::default();
        let mut random = UnityRandom::from_seed(12345);
        let expected = [0.690_138_04, 0.924_670_46, 0.924_670_46, 0.649_387_3];
        for expected in expected {
            advance_effect(&config, &mut runtime, &mut random, 1.0 / 60.0);
            assert!((runtime.intensity - expected).abs() <= 1.0e-6);
        }
    }

    #[test]
    fn every_effect_matches_unity_6000_3_golden_checkpoints() {
        let fixture: EffectsFixture =
            serde_json::from_str(include_str!("../tests/golden/unity_effects_v1.json")).unwrap();

        for trace in fixture.traces {
            let config = DynamicLightConfig {
                effect: DynamicLightEffect::ALL[trace.discriminant],
                ..Default::default()
            };
            let mut runtime = LightEffectRuntime::default();
            let mut random = UnityRandom::from_seed(trace.seed);
            let last_frame = trace.samples.last().unwrap().frame;
            let mut samples = trace.samples.into_iter().peekable();
            for frame in 0..=last_frame {
                let checkpoint_delta = samples
                    .peek()
                    .filter(|sample| sample.frame == frame)
                    .map(|sample| sample.delta);
                let delta = checkpoint_delta.unwrap_or_else(|| match trace.schedule.as_str() {
                    "30hz" => 1.0 / 30.0,
                    "60hz" => 1.0 / 60.0,
                    "120hz" => 1.0 / 120.0,
                    "jitter" => [1.0 / 53.0, 1.0 / 71.0, 1.0 / 44.0, 1.0 / 97.0][frame % 4],
                    schedule => panic!("unexpected Unity schedule {schedule}"),
                });
                advance_effect(&config, &mut runtime, &mut random, delta);
                if samples.peek().is_some_and(|sample| sample.frame == frame) {
                    let sample = samples.next().unwrap();
                    assert!(
                        (runtime.intensity - sample.intensity).abs() <= 2.0e-5,
                        "effect {:?} seed {} schedule {} frame {frame}: Rust {} != Unity {}",
                        config.effect,
                        trace.seed,
                        trace.schedule,
                        runtime.intensity,
                        sample.intensity,
                    );
                }
            }
        }
    }

    #[test]
    fn stable_multilight_order_matches_unitys_single_random_stream() {
        let fixture: MultiLightFixture = serde_json::from_str(include_str!(
            "../tests/golden/unity_multilight_random_v1.json"
        ))
        .unwrap();
        let configs = fixture
            .effect_discriminants
            .into_iter()
            .map(|discriminant| DynamicLightConfig {
                effect: DynamicLightEffect::ALL[discriminant],
                ..Default::default()
            })
            .collect::<Vec<_>>();
        let mut runtimes = vec![LightEffectRuntime::default(); configs.len()];
        let mut random = UnityRandom::from_seed(fixture.seed);
        let last_frame = fixture.frames.last().unwrap().frame;
        let mut frames = fixture.frames.into_iter().peekable();

        for frame in 0..=last_frame {
            for (config, runtime) in configs.iter().zip(&mut runtimes) {
                advance_effect(config, runtime, &mut random, 1.0 / 60.0);
            }
            if frames.peek().is_some_and(|sample| sample.frame == frame) {
                let sample = frames.next().unwrap();
                for (index, (runtime, expected)) in
                    runtimes.iter().zip(sample.intensities).enumerate()
                {
                    assert!(
                        (runtime.intensity - expected).abs() <= 2.0e-5,
                        "frame {frame} light {index}: Rust {} != Unity {expected}",
                        runtime.intensity,
                    );
                }
            }
        }
    }

    #[test]
    fn randomized_effects_repeat_per_seed_and_diverge_between_seeds() {
        let config = DynamicLightConfig {
            effect: DynamicLightEffect::Random,
            ..Default::default()
        };
        let mut runtime_a = LightEffectRuntime::default();
        let mut runtime_b = LightEffectRuntime::default();
        let mut runtime_c = LightEffectRuntime::default();
        let mut random_a = UnityRandom::from_seed(17);
        let mut random_b = UnityRandom::from_seed(17);
        let mut random_c = UnityRandom::from_seed(18);
        let mut diverged = false;
        for _ in 0..32 {
            advance_effect(&config, &mut runtime_a, &mut random_a, 1.0 / 30.0);
            advance_effect(&config, &mut runtime_b, &mut random_b, 1.0 / 30.0);
            advance_effect(&config, &mut runtime_c, &mut random_c, 1.0 / 30.0);
            assert_eq!(runtime_a.intensity, runtime_b.intensity);
            diverged |= runtime_a.intensity != runtime_c.intensity;
        }
        assert!(diverged);
    }
}
