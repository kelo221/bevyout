use super::types::{LightEffect, LightEffectState};

const TAU: f32 = std::f32::consts::PI * 2.0;

pub(crate) fn intensity_multiplier(state: LightEffectState, elapsed_seconds: f32) -> f32 {
    let time = if elapsed_seconds.is_finite() {
        elapsed_seconds.max(0.0)
    } else {
        0.0
    };
    let modifier = finite_clamp01(state.pulse_modifier, 0.25);
    let speed = finite_nonnegative(state.pulse_speed);
    let offset = if state.pulse_offset.is_finite() {
        state.pulse_offset
    } else {
        0.0
    };
    let value = match state.effect {
        LightEffect::Steady => 1.0,
        LightEffect::Pulse => lerp(
            modifier,
            1.0,
            (1.0 + ((offset + time * speed) * TAU).sin()) * 0.5,
        ),
        LightEffect::Random => {
            let step = fixed_step(time + offset, state.timestep_seconds);
            lerp(modifier, 1.0, hash01(state.random_seed, step, 0x31))
        }
        LightEffect::Strobe => strobe_value(state, time),
        LightEffect::Flicker => {
            let step = fixed_step(time + offset, state.timestep_seconds);
            let random = hash01(state.random_seed, step, 0x41);
            if random < 0.5 {
                0.0
            } else {
                lerp(modifier, 1.0, hash01(state.random_seed, step, 0x42))
            }
        }
        LightEffect::FluorescentStarter => fluorescent_starter(state, time),
        LightEffect::FluorescentClicker => fluorescent_clicker(state, time),
        LightEffect::FluorescentRandom => fluorescent_random(state, time),
        LightEffect::Candle => candle(state, time),
        LightEffect::Pulsar => {
            let phase = ((offset + time) * TAU * speed).sin().max(0.0);
            lerp(modifier, 1.0, phase.powi(5))
        }
        LightEffect::Fire => fire(state, time),
        LightEffect::Generator => {
            let generator_time = offset + time * speed;
            let flicker = (-1.0
                + value_noise(generator_time, generator_time * 0.1, state.random_seed) * 2.5)
                .clamp(-1.0, 0.0)
                * (1.0 - modifier);
            1.0 + flicker
        }
        LightEffect::Lightning => lightning(state, time),
        LightEffect::Cloudy => cloudy(state, time),
        LightEffect::Overcast => overcast(state, time),
    };
    if value.is_finite() {
        value.max(0.0)
    } else {
        1.0
    }
}

fn strobe_value(state: LightEffectState, time: f32) -> f32 {
    if !state.frequency_hz.is_finite() || state.frequency_hz <= 0.0 {
        return 1.0;
    }
    let period = 1.0 / state.frequency_hz;
    let duty = finite_clamp01(state.duty_cycle, 0.5);
    if (time + state.pulse_offset).rem_euclid(period) < period * duty {
        1.0
    } else {
        finite_clamp01(state.pulse_modifier, 0.25)
    }
}

fn fluorescent_starter(state: LightEffectState, time: f32) -> f32 {
    let sequence = (state.pulse_offset + time).rem_euclid(3.3);
    if sequence < 0.5 {
        (0.25 + (sequence * std::f32::consts::PI * 50.0).sin() * 0.125)
            * (1.0 - finite_clamp01(state.pulse_modifier, 0.25))
    } else if sequence > 2.95 {
        lerp(1.0, 0.0, (sequence - 3.0) * 20.0)
    } else {
        1.0
    }
}

fn fluorescent_clicker(state: LightEffectState, time: f32) -> f32 {
    let sequence = (state.pulse_offset + time).rem_euclid(6.0);
    if sequence < 0.3 {
        0.1 + (sequence * std::f32::consts::PI * 20.0).sin() * 0.05
    } else if sequence < 1.5 {
        if (sequence.rem_euclid(0.2)) < 0.05 {
            1.0
        } else {
            0.0
        }
    } else if sequence < 4.5 {
        1.0
    } else {
        lerp(1.0, 0.0, (sequence - 4.5) / 0.0625)
    }
}

fn fluorescent_random(state: LightEffectState, time: f32) -> f32 {
    let step = fixed_step(time + state.pulse_offset, state.timestep_seconds);
    match hash_u32(state.random_seed ^ step as u32) % 3 {
        0 => finite_clamp01(state.pulse_modifier, 0.25),
        1 => 0.25 + (time * std::f32::consts::PI * 50.0).sin() * 0.125,
        _ => 1.0,
    }
}

fn candle(state: LightEffectState, time: f32) -> f32 {
    let modifier = finite_clamp01(state.pulse_modifier, 0.25);
    let flicker_time = state.pulse_offset + time;
    let base = value_noise(flicker_time * 0.5, 0.0, state.random_seed);
    let random_dip = value_noise(flicker_time * 2.0, 10.0, state.random_seed);
    let dip = if random_dip > 0.8 {
        (random_dip - 0.8) * 2.0
    } else {
        0.0
    };
    let mut target = lerp(modifier, 1.0, base - dip);
    if value_noise(flicker_time * 5.0, 20.0, state.random_seed) > 0.7 {
        target *= modifier;
    }
    target.clamp(modifier, 1.0)
}

fn fire(state: LightEffectState, time: f32) -> f32 {
    let modifier = finite_clamp01(state.pulse_modifier, 0.25);
    let flicker_time = state.pulse_offset + time;
    let base = value_noise(flicker_time * 0.5, 0.0, state.random_seed);
    let chaotic = value_noise(flicker_time * 3.0, 10.0, state.random_seed);
    let mut target = lerp(modifier, 1.0, base * 0.6 + chaotic * 0.4);
    if value_noise(flicker_time * 5.0, 20.0, state.random_seed) > 0.7 {
        target = 1.0;
    }
    target.clamp(modifier, 1.0)
}

fn lightning(state: LightEffectState, time: f32) -> f32 {
    let modifier = finite_clamp01(state.pulse_modifier, 0.25);
    let lightning_time = time + state.pulse_offset;
    let slow_wave = 0.5 + (lightning_time * TAU).sin() * 0.5;
    let mut intensity = modifier;
    for (index, unique) in [0.2, 4.5, 7.8].into_iter().enumerate() {
        if index >= usize::from(state.lightning_layers.clamp(1, 3)) {
            break;
        }
        let layer_time = lightning_time + unique;
        let layer_wave = 0.5 + ((layer_time + unique) * TAU).sin() * 0.5;
        let remainder = (1.0 - modifier) * layer_wave;
        if value_noise(layer_time * 5.0, unique, state.random_seed) > 0.74 {
            intensity =
                modifier + remainder + (layer_time * std::f32::consts::PI * 40.0).sin() * remainder;
        }
    }
    // Retain the source's slow dimming bias while keeping the result bounded.
    lerp(intensity, modifier, (1.0 - slow_wave).clamp(0.0, 1.0) * 0.1)
}

fn cloudy(state: LightEffectState, time: f32) -> f32 {
    let generator_time = state.pulse_offset + time * finite_nonnegative(state.pulse_speed);
    let mut noise = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    for _ in 0..2 {
        let t = generator_time * frequency;
        let perlin = value_noise(t * 0.05, t * 0.005, state.random_seed);
        noise += ((perlin - 0.5) * 4.0).clamp(0.0, 1.0) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    let raw = noise / 1.5;
    let inverse = 1.0 - raw;
    let stretched = inverse * inverse * (3.0 - 2.0 * inverse);
    lerp(finite_clamp01(state.pulse_modifier, 0.25), 1.0, stretched)
}

fn overcast(state: LightEffectState, time: f32) -> f32 {
    let generator_time = state.pulse_offset + time * finite_nonnegative(state.pulse_speed);
    let mut noise = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    for _ in 0..3 {
        let t = generator_time * frequency;
        noise += value_noise(t * 0.1, t * 0.01, state.random_seed) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    let raw = noise / 1.75;
    let stretched = raw * raw * (3.0 - 2.0 * raw);
    lerp(finite_clamp01(state.pulse_modifier, 0.25), 1.0, stretched)
}

fn fixed_step(time: f32, timestep: f32) -> i32 {
    if timestep.is_finite() && timestep > 0.0 {
        (time.max(0.0) / timestep).floor() as i32
    } else {
        time.max(0.0).floor() as i32
    }
}

fn finite_nonnegative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn finite_clamp01(value: f32, fallback: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        fallback
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

fn hash01(seed: u32, step: i32, salt: u32) -> f32 {
    let mixed = hash_u32(seed ^ (step as u32).wrapping_mul(0x9e37_79b9) ^ salt);
    mixed as f32 / u32::MAX as f32
}

fn hash_u32(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
}

fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    if !x.is_finite() || !y.is_finite() {
        return 0.5;
    }
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let tx = smoothstep(x - x0 as f32);
    let ty = smoothstep(y - y0 as f32);
    let n00 = hash01_2d(x0, y0, seed);
    let n10 = hash01_2d(x0 + 1, y0, seed);
    let n01 = hash01_2d(x0, y0 + 1, seed);
    let n11 = hash01_2d(x0 + 1, y0 + 1, seed);
    lerp(lerp(n00, n10, tx), lerp(n01, n11, tx), ty)
}

fn hash01_2d(x: i32, y: i32, seed: u32) -> f32 {
    let mixed = hash_u32(
        seed ^ (x as u32).wrapping_mul(0x85eb_ca6b) ^ (y as u32).wrapping_mul(0xc2b2_ae35),
    );
    mixed as f32 / u32::MAX as f32
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_intensity_effect_is_finite_at_multiple_times() {
        for effect in LightEffect::ALL {
            let state = LightEffectState::default().with_effect(effect);
            for time in [0.0, 0.1, 0.5, 1.0, 4.25] {
                assert!(
                    state.intensity_multiplier(time).is_finite(),
                    "{effect:?} at {time}"
                );
            }
        }
    }

    #[test]
    fn pulse_reaches_both_bounds_when_not_modified() {
        let state = LightEffectState {
            effect: LightEffect::Pulse,
            pulse_modifier: 0.0,
            ..Default::default()
        };
        assert!((state.intensity_multiplier(0.25) - 1.0).abs() < 1e-5);
        assert!(state.intensity_multiplier(0.75) < 1e-5);
    }

    #[test]
    fn noise_is_deterministic_for_a_seed() {
        assert_eq!(value_noise(1.25, 2.5, 7), value_noise(1.25, 2.5, 7));
        assert_ne!(value_noise(1.25, 2.5, 7), value_noise(1.25, 2.5, 8));
    }
}
