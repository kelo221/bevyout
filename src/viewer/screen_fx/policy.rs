//! Engine-independent ImageSpace/IMAD screen-feedback policy.
//!
//! This module intentionally contains no Bevy types.  The viewer adapter owns
//! camera components and UI entities; this policy owns the reproducible part of
//! screen feedback: integer-time sampling, ordering, composition, lifecycle,
//! and user settings.

use std::collections::BTreeMap;

const EPSILON: f32 = 0.000_1;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenFxSource {
    #[default]
    Developer,
    WeaponHit,
    Gameplay,
    Script,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenFxClearReason {
    Death,
    SaveLoad,
    CameraMode,
    CellTransition,
    #[default]
    Teardown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenFxCurveOperation {
    #[default]
    Additive,
    Multiplier,
    Set,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenFxProperty {
    #[default]
    Brightness,
    Saturation,
    Contrast,
    Blur,
    DoubleVision,
    MotionBlur,
    RadialBlur,
    RadialCenterX,
    RadialCenterY,
    RadialRampUp,
    RadialStart,
    RadialRampDown,
    RadialDownStart,
    Fade,
    DepthOfField,
    DepthOfFieldDistance,
    DepthOfFieldRange,
    Blood,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenFxValues {
    /// Multiplier; `1.0` is neutral.
    pub brightness: f32,
    /// Multiplier; `1.0` is neutral.
    pub saturation: f32,
    /// Multiplier; `1.0` is neutral.
    pub contrast: f32,
    /// Premultiplied overlay tint in authored RGBA order.
    pub tint: [f32; 4],
    /// Additive normalized strengths, all clamped to `[0, 1]`.
    pub blur: f32,
    pub double_vision: f32,
    pub motion_blur: f32,
    pub radial_blur: f32,
    pub radial_center: [f32; 2],
    pub radial_ramp_up: f32,
    pub radial_start: f32,
    pub radial_ramp_down: f32,
    pub radial_down_start: f32,
    pub fade: f32,
    pub blood: f32,
    pub depth_of_field: f32,
    pub depth_of_field_distance: f32,
    pub depth_of_field_range: f32,
}

impl ScreenFxValues {
    pub const fn neutral() -> Self {
        Self {
            brightness: 1.0,
            saturation: 1.0,
            contrast: 1.0,
            tint: [0.0; 4],
            blur: 0.0,
            double_vision: 0.0,
            motion_blur: 0.0,
            radial_blur: 0.0,
            radial_center: [0.5, 0.5],
            radial_ramp_up: 0.0,
            radial_start: 0.0,
            radial_ramp_down: 0.0,
            radial_down_start: 0.0,
            fade: 0.0,
            blood: 0.0,
            depth_of_field: 0.0,
            depth_of_field_distance: 0.0,
            depth_of_field_range: 0.0,
        }
    }

    fn finite_or_neutral(self) -> Self {
        Self {
            brightness: finite_or(self.brightness, 1.0),
            saturation: finite_or(self.saturation, 1.0),
            contrast: finite_or(self.contrast, 1.0),
            tint: self.tint.map(|value| finite_or(value, 0.0)),
            blur: finite_or(self.blur, 0.0),
            double_vision: finite_or(self.double_vision, 0.0),
            motion_blur: finite_or(self.motion_blur, 0.0),
            radial_blur: finite_or(self.radial_blur, 0.0),
            radial_center: self.radial_center.map(|value| finite_or(value, 0.5)),
            radial_ramp_up: finite_or(self.radial_ramp_up, 0.0),
            radial_start: finite_or(self.radial_start, 0.0),
            radial_ramp_down: finite_or(self.radial_ramp_down, 0.0),
            radial_down_start: finite_or(self.radial_down_start, 0.0),
            fade: finite_or(self.fade, 0.0),
            blood: finite_or(self.blood, 0.0),
            depth_of_field: finite_or(self.depth_of_field, 0.0),
            depth_of_field_distance: finite_or(self.depth_of_field_distance, 0.0),
            depth_of_field_range: finite_or(self.depth_of_field_range, 0.0),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenFxKeyframe {
    pub time_ms: u32,
    pub value: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenFxColorKeyframe {
    pub time_ms: u32,
    pub rgba: [f32; 4],
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenFxCurve {
    pub property: ScreenFxProperty,
    pub operation: ScreenFxCurveOperation,
    pub keyframes: Vec<ScreenFxKeyframe>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenFxDefinition {
    pub modifier_form_id: u32,
    pub duration_ms: u32,
    pub static_values: ScreenFxValues,
    pub curves: Vec<ScreenFxCurve>,
    pub color_keyframes: Vec<ScreenFxColorKeyframe>,
    pub fade_color_keyframes: Vec<ScreenFxColorKeyframe>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScreenFxStart {
    pub source: ScreenFxSource,
    pub modifier_form_id: u32,
    pub priority: i32,
    pub start_ms: u64,
    pub intensity: f32,
    pub definition: ScreenFxDefinition,
}

impl ScreenFxStart {
    pub fn new(
        source: ScreenFxSource,
        modifier_form_id: u32,
        priority: i32,
        start_ms: u64,
        definition: ScreenFxDefinition,
    ) -> Self {
        Self {
            source,
            modifier_form_id,
            priority,
            start_ms,
            intensity: 1.0,
            definition,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScreenFxRequest {
    Start(ScreenFxStart),
    Stop { modifier_form_id: u32 },
    Clear { reason: ScreenFxClearReason },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScreenFxSettings {
    pub overall_intensity: f32,
    pub screen_blood: f32,
    pub flashes: f32,
    pub motion_and_distortion: f32,
}

impl Default for ScreenFxSettings {
    fn default() -> Self {
        Self {
            overall_intensity: 1.0,
            screen_blood: 1.0,
            flashes: 1.0,
            motion_and_distortion: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ActiveModifier {
    source: ScreenFxSource,
    priority: i32,
    start_ms: u64,
    intensity: f32,
    sequence: u64,
    definition: ScreenFxDefinition,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScreenFxPolicy {
    base: ScreenFxValues,
    active: BTreeMap<u32, ActiveModifier>,
    now_ms: u64,
    next_sequence: u64,
    settings: ScreenFxSettings,
}

impl Default for ScreenFxPolicy {
    fn default() -> Self {
        Self::new(ScreenFxValues::neutral())
    }
}

impl ScreenFxPolicy {
    pub fn new(base: ScreenFxValues) -> Self {
        Self {
            base: base.finite_or_neutral(),
            active: BTreeMap::new(),
            now_ms: 0,
            next_sequence: 0,
            settings: ScreenFxSettings::default(),
        }
    }

    pub fn base(&self) -> ScreenFxValues {
        self.base
    }

    pub fn set_base(&mut self, base: ScreenFxValues) {
        self.base = base.finite_or_neutral();
    }

    pub fn settings(&self) -> ScreenFxSettings {
        self.settings
    }

    pub fn set_settings(&mut self, settings: ScreenFxSettings) {
        self.settings = ScreenFxSettings {
            overall_intensity: clamp01(settings.overall_intensity),
            screen_blood: clamp01(settings.screen_blood),
            flashes: clamp01(settings.flashes),
            motion_and_distortion: clamp01(settings.motion_and_distortion),
        };
    }

    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }

    pub fn active_len(&self) -> usize {
        self.active.len()
    }

    pub fn active_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.active.keys().copied()
    }

    pub fn apply(&mut self, request: ScreenFxRequest) {
        match request {
            ScreenFxRequest::Start(start) => self.start(start),
            ScreenFxRequest::Stop { modifier_form_id } => {
                self.active.remove(&modifier_form_id);
            }
            ScreenFxRequest::Clear { .. } => self.active.clear(),
        }
    }

    pub fn start(&mut self, mut start: ScreenFxStart) {
        let id = start.modifier_form_id;
        start.definition.modifier_form_id = id;
        let definition = sanitize_definition(start.definition);
        let intensity = if start.intensity.is_finite() {
            start.intensity.max(0.0)
        } else {
            0.0
        };
        if let Some(existing) = self.active.get_mut(&id) {
            // A repeated start for the same identity is an idempotent update:
            // it replaces authored data without creating another sequence slot.
            existing.source = start.source;
            existing.priority = start.priority;
            existing.start_ms = start.start_ms;
            existing.intensity = intensity;
            existing.definition = definition;
            return;
        }
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        self.active.insert(
            id,
            ActiveModifier {
                source: start.source,
                priority: start.priority,
                start_ms: start.start_ms,
                intensity,
                sequence,
                definition,
            },
        );
    }

    pub fn advance_to(&mut self, now_ms: u64) {
        self.now_ms = now_ms;
        self.active.retain(|_, modifier| {
            modifier.definition.duration_ms == 0
                || now_ms.saturating_sub(modifier.start_ms)
                    < u64::from(modifier.definition.duration_ms)
        });
    }

    pub fn sample(&self, definition: &ScreenFxDefinition, elapsed_ms: u64) -> ScreenFxValues {
        let elapsed_ms = elapsed_ms.min(u64::from(definition.duration_ms.max(1))) as u32;
        let mut values = definition.static_values.finite_or_neutral();
        for curve in &definition.curves {
            let value = sample_curve(curve, elapsed_ms);
            apply_scalar(&mut values, curve.property, curve.operation, value, 1.0);
        }
        if let Some(color) = sample_color_curve(&definition.color_keyframes, elapsed_ms) {
            values.tint = color;
        }
        if let Some(color) = sample_color_curve(&definition.fade_color_keyframes, elapsed_ms) {
            values.fade = color[3];
        }
        values.finite_or_neutral()
    }

    pub fn snapshot(&self) -> ScreenFxValues {
        let mut output = self.base;
        let mut modifiers = self.active.values().collect::<Vec<_>>();
        modifiers.sort_by_key(|modifier| {
            (
                modifier.priority,
                modifier.definition.modifier_form_id,
                modifier.sequence,
            )
        });
        for modifier in modifiers {
            let elapsed_ms = self.now_ms.saturating_sub(modifier.start_ms);
            let values = self.sample(&modifier.definition, elapsed_ms);
            let intensity = modifier.intensity * self.settings.overall_intensity;
            let flashes = if modifier.source == ScreenFxSource::WeaponHit {
                self.settings.flashes
            } else {
                1.0
            };
            let distortion = if modifier.source == ScreenFxSource::WeaponHit {
                self.settings.motion_and_distortion
            } else {
                1.0
            };
            apply_values(
                &mut output,
                values,
                intensity,
                self.settings.screen_blood,
                flashes,
                distortion,
            );
        }
        output.finite_or_neutral()
    }
}

fn sanitize_definition(mut definition: ScreenFxDefinition) -> ScreenFxDefinition {
    definition.static_values = definition.static_values.finite_or_neutral();
    for curve in &mut definition.curves {
        curve.keyframes.retain(|keyframe| {
            keyframe.value.is_finite() && keyframe.time_ms <= definition.duration_ms
        });
        curve.keyframes.sort_by_key(|keyframe| keyframe.time_ms);
        curve.keyframes.dedup_by_key(|keyframe| keyframe.time_ms);
    }
    for keyframes in [
        &mut definition.color_keyframes,
        &mut definition.fade_color_keyframes,
    ] {
        keyframes.retain(|keyframe| {
            keyframe.time_ms <= definition.duration_ms
                && keyframe.rgba.iter().all(|value| value.is_finite())
        });
        keyframes.sort_by_key(|keyframe| keyframe.time_ms);
        keyframes.dedup_by_key(|keyframe| keyframe.time_ms);
    }
    definition
}

fn sample_curve(curve: &ScreenFxCurve, elapsed_ms: u32) -> f32 {
    let Some(first) = curve.keyframes.first() else {
        return 0.0;
    };
    if elapsed_ms <= first.time_ms {
        return first.value;
    }
    for pair in curve.keyframes.windows(2) {
        let [left, right] = pair else { unreachable!() };
        if elapsed_ms <= right.time_ms {
            let span = right.time_ms.saturating_sub(left.time_ms);
            if span == 0 {
                return right.value;
            }
            let fraction = (elapsed_ms - left.time_ms) as f32 / span as f32;
            return left.value + (right.value - left.value) * fraction;
        }
    }
    curve
        .keyframes
        .last()
        .map_or(0.0, |keyframe| keyframe.value)
}

fn sample_color_curve(keyframes: &[ScreenFxColorKeyframe], elapsed_ms: u32) -> Option<[f32; 4]> {
    let first = keyframes.first()?;
    if elapsed_ms <= first.time_ms {
        return Some(first.rgba);
    }
    for pair in keyframes.windows(2) {
        let [left, right] = pair else { unreachable!() };
        if elapsed_ms <= right.time_ms {
            let span = right.time_ms.saturating_sub(left.time_ms);
            if span == 0 {
                return Some(right.rgba);
            }
            let fraction = (elapsed_ms - left.time_ms) as f32 / span as f32;
            return Some(std::array::from_fn(|index| {
                left.rgba[index] + (right.rgba[index] - left.rgba[index]) * fraction
            }));
        }
    }
    Some(keyframes.last()?.rgba)
}

fn apply_values(
    output: &mut ScreenFxValues,
    values: ScreenFxValues,
    intensity: f32,
    blood_scale: f32,
    flashes: f32,
    distortion: f32,
) {
    let intensity = intensity.max(0.0);
    output.brightness *= lerp_identity(values.brightness, intensity * flashes);
    output.saturation *= lerp_identity(values.saturation, intensity);
    output.contrast *= lerp_identity(values.contrast, intensity);
    compose_tint(&mut output.tint, values.tint, intensity * flashes);
    output.blur = clamp01(output.blur + values.blur * intensity);
    output.double_vision =
        clamp01(output.double_vision + values.double_vision * intensity * distortion);
    output.motion_blur = clamp01(output.motion_blur + values.motion_blur * intensity * distortion);
    output.radial_blur = clamp01(output.radial_blur + values.radial_blur * intensity * distortion);
    output.radial_center = [
        output.radial_center[0] + (values.radial_center[0] - 0.5) * intensity * distortion,
        output.radial_center[1] + (values.radial_center[1] - 0.5) * intensity * distortion,
    ];
    output.radial_ramp_up =
        clamp01(output.radial_ramp_up + values.radial_ramp_up * intensity * distortion);
    output.radial_start =
        clamp01(output.radial_start + values.radial_start * intensity * distortion);
    output.radial_ramp_down =
        clamp01(output.radial_ramp_down + values.radial_ramp_down * intensity * distortion);
    output.radial_down_start =
        clamp01(output.radial_down_start + values.radial_down_start * intensity * distortion);
    output.fade = clamp01(output.fade + values.fade * intensity * flashes);
    output.blood = clamp01(output.blood + values.blood * intensity * blood_scale);
    output.depth_of_field =
        clamp01(output.depth_of_field + values.depth_of_field * intensity * distortion);
    output.depth_of_field_distance += values.depth_of_field_distance * intensity * distortion;
    output.depth_of_field_range += values.depth_of_field_range * intensity * distortion;
}

fn apply_scalar(
    values: &mut ScreenFxValues,
    property: ScreenFxProperty,
    operation: ScreenFxCurveOperation,
    value: f32,
    intensity: f32,
) {
    let value = finite_or(value, 0.0) * intensity;
    match property {
        ScreenFxProperty::Brightness => apply_multiplier(&mut values.brightness, value, operation),
        ScreenFxProperty::Saturation => apply_multiplier(&mut values.saturation, value, operation),
        ScreenFxProperty::Contrast => apply_multiplier(&mut values.contrast, value, operation),
        ScreenFxProperty::Blur => apply_strength(&mut values.blur, value, operation),
        ScreenFxProperty::DoubleVision => {
            apply_strength(&mut values.double_vision, value, operation)
        }
        ScreenFxProperty::MotionBlur => apply_strength(&mut values.motion_blur, value, operation),
        ScreenFxProperty::RadialBlur => apply_strength(&mut values.radial_blur, value, operation),
        ScreenFxProperty::RadialCenterX => {
            apply_scalar_value(&mut values.radial_center[0], value, operation)
        }
        ScreenFxProperty::RadialCenterY => {
            apply_scalar_value(&mut values.radial_center[1], value, operation)
        }
        ScreenFxProperty::RadialRampUp => {
            apply_strength(&mut values.radial_ramp_up, value, operation)
        }
        ScreenFxProperty::RadialStart => apply_strength(&mut values.radial_start, value, operation),
        ScreenFxProperty::RadialRampDown => {
            apply_strength(&mut values.radial_ramp_down, value, operation)
        }
        ScreenFxProperty::RadialDownStart => {
            apply_strength(&mut values.radial_down_start, value, operation)
        }
        ScreenFxProperty::Fade => apply_strength(&mut values.fade, value, operation),
        ScreenFxProperty::DepthOfField => {
            apply_strength(&mut values.depth_of_field, value, operation)
        }
        ScreenFxProperty::DepthOfFieldDistance => {
            apply_scalar_value(&mut values.depth_of_field_distance, value, operation)
        }
        ScreenFxProperty::DepthOfFieldRange => {
            apply_scalar_value(&mut values.depth_of_field_range, value, operation)
        }
        ScreenFxProperty::Blood => apply_strength(&mut values.blood, value, operation),
    }
}

fn apply_multiplier(target: &mut f32, value: f32, operation: ScreenFxCurveOperation) {
    match operation {
        ScreenFxCurveOperation::Additive => *target += value,
        ScreenFxCurveOperation::Multiplier => *target *= value,
        ScreenFxCurveOperation::Set => *target = value,
    }
}

fn apply_strength(target: &mut f32, value: f32, operation: ScreenFxCurveOperation) {
    match operation {
        ScreenFxCurveOperation::Additive => *target = clamp01(*target + value),
        ScreenFxCurveOperation::Multiplier => *target = clamp01(*target * value),
        ScreenFxCurveOperation::Set => *target = clamp01(value),
    }
}

fn apply_scalar_value(target: &mut f32, value: f32, operation: ScreenFxCurveOperation) {
    match operation {
        ScreenFxCurveOperation::Additive => *target += value,
        ScreenFxCurveOperation::Multiplier => *target *= value,
        ScreenFxCurveOperation::Set => *target = value,
    }
}

fn compose_tint(target: &mut [f32; 4], source: [f32; 4], intensity: f32) {
    let alpha = clamp01(source[3] * intensity);
    if alpha <= EPSILON {
        return;
    }
    for channel in 0..3 {
        target[channel] = target[channel] * (1.0 - alpha) + source[channel] * alpha;
    }
    target[3] = alpha + target[3] * (1.0 - alpha);
}

fn lerp_identity(value: f32, intensity: f32) -> f32 {
    1.0 + (value - 1.0) * intensity.max(0.0)
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

fn clamp01(value: f32) -> f32 {
    finite_or(value, 0.0).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition(id: u32, duration_ms: u32, values: ScreenFxValues) -> ScreenFxDefinition {
        ScreenFxDefinition {
            modifier_form_id: id,
            duration_ms,
            static_values: values,
            curves: Vec::new(),
            color_keyframes: Vec::new(),
            fade_color_keyframes: Vec::new(),
        }
    }

    #[test]
    fn samples_integer_time_and_clamps_endpoints() {
        let definition = ScreenFxDefinition {
            modifier_form_id: 1,
            duration_ms: 100,
            static_values: ScreenFxValues::neutral(),
            curves: vec![ScreenFxCurve {
                property: ScreenFxProperty::Blood,
                operation: ScreenFxCurveOperation::Additive,
                keyframes: vec![
                    ScreenFxKeyframe {
                        time_ms: 0,
                        value: 0.0,
                    },
                    ScreenFxKeyframe {
                        time_ms: 100,
                        value: 1.0,
                    },
                ],
            }],
            color_keyframes: Vec::new(),
            fade_color_keyframes: Vec::new(),
        };
        let policy = ScreenFxPolicy::default();
        assert_eq!(policy.sample(&definition, 0).blood, 0.0);
        assert!((policy.sample(&definition, 50).blood - 0.5).abs() < EPSILON);
        assert_eq!(policy.sample(&definition, 100).blood, 1.0);
        assert_eq!(policy.sample(&definition, 200).blood, 1.0);
    }

    #[test]
    fn starts_are_idempotent_and_replacements_keep_one_active_modifier() {
        let mut policy = ScreenFxPolicy::default();
        let mut start = ScreenFxStart::new(
            ScreenFxSource::Gameplay,
            7,
            1,
            0,
            definition(
                7,
                1000,
                ScreenFxValues {
                    blood: 0.25,
                    ..ScreenFxValues::neutral()
                },
            ),
        );
        policy.start(start.clone());
        policy.start(start.clone());
        assert_eq!(policy.active_len(), 1);
        start.definition.static_values.blood = 0.75;
        policy.start(start);
        assert_eq!(policy.active_len(), 1);
        assert!((policy.snapshot().blood - 0.75).abs() < EPSILON);
    }

    #[test]
    fn order_is_priority_then_form_id_then_sequence() {
        let mut policy = ScreenFxPolicy::default();
        for (id, priority, blood) in [(20, 2, 0.2), (10, 1, 0.1), (30, 2, 0.3)] {
            policy.start(ScreenFxStart::new(
                ScreenFxSource::Gameplay,
                id,
                priority,
                0,
                definition(
                    id,
                    1000,
                    ScreenFxValues {
                        tint: [blood, 0.0, 0.0, blood],
                        ..ScreenFxValues::neutral()
                    },
                ),
            ));
        }
        let snapshot = policy.snapshot();
        // The priority-1 tint is applied first, then FormID 20, then 30.
        assert!(snapshot.tint[0] > 0.0);
        assert!(snapshot.tint[3] > 0.0);
    }

    #[test]
    fn expiry_and_clear_restore_the_base() {
        let base = ScreenFxValues {
            brightness: 0.8,
            ..ScreenFxValues::neutral()
        };
        let mut policy = ScreenFxPolicy::new(base);
        policy.start(ScreenFxStart::new(
            ScreenFxSource::WeaponHit,
            2,
            0,
            0,
            definition(
                2,
                10,
                ScreenFxValues {
                    blood: 1.0,
                    ..ScreenFxValues::neutral()
                },
            ),
        ));
        policy.advance_to(10);
        assert_eq!(policy.active_len(), 0);
        assert_eq!(policy.snapshot(), base);
        policy.apply(ScreenFxRequest::Clear {
            reason: ScreenFxClearReason::CellTransition,
        });
        assert_eq!(policy.snapshot(), base);
    }

    #[test]
    fn settings_can_disable_blood_and_distortion() {
        let mut policy = ScreenFxPolicy::default();
        policy.set_settings(ScreenFxSettings {
            overall_intensity: 1.0,
            screen_blood: 0.0,
            flashes: 1.0,
            motion_and_distortion: 0.0,
        });
        policy.start(ScreenFxStart::new(
            ScreenFxSource::WeaponHit,
            4,
            0,
            0,
            definition(
                4,
                100,
                ScreenFxValues {
                    blood: 1.0,
                    double_vision: 1.0,
                    ..ScreenFxValues::neutral()
                },
            ),
        ));
        let output = policy.snapshot();
        assert_eq!(output.blood, 0.0);
        assert_eq!(output.double_vision, 0.0);
    }
}
