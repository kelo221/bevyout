//! Pure policy shared by hybrid point-shadow shading tests and the observable
//! lighting demo. This module intentionally has no Bevy dependency so the
//! Cucumber harness can include it verbatim.

/// Combines prepared static visibility with realtime visibility.
///
/// Shadow values are visibility factors (`0` fully occluded, `1` fully lit),
/// so the darker available source wins.
#[allow(dead_code)]
pub(crate) fn hybrid_shadow_visibility(prepared: Option<f32>, realtime: Option<f32>) -> f32 {
    let prepared = prepared.map(sanitize_visibility);
    let realtime = realtime.map(sanitize_visibility);

    match (prepared, realtime) {
        (Some(prepared), Some(realtime)) => prepared.min(realtime),
        (Some(prepared), None) => prepared,
        (None, Some(realtime)) => realtime,
        (None, None) => 1.0,
    }
}

fn sanitize_visibility(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        1.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DemoOrbit {
    pub(crate) center: [f32; 3],
    pub(crate) radius: f32,
    pub(crate) radians_per_second: f32,
}

impl DemoOrbit {
    pub(crate) fn position(self, elapsed_seconds: f32) -> [f32; 3] {
        let angle = elapsed_seconds * self.radians_per_second;
        [
            self.center[0] + angle.cos() * self.radius,
            self.center[1],
            self.center[2] + angle.sin() * self.radius,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_sources_are_fully_lit() {
        assert_eq!(hybrid_shadow_visibility(None, None), 1.0);
    }

    #[test]
    fn non_finite_inputs_cannot_poison_visibility() {
        assert_eq!(hybrid_shadow_visibility(Some(f32::NAN), None), 1.0);
        assert_eq!(hybrid_shadow_visibility(None, Some(f32::NAN)), 1.0);
    }
}
