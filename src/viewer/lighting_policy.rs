//! Pure policy for prepared point-shadow softness.

pub(crate) const DEFAULT_SOURCE_RADIUS_METERS: f32 = 0.05;
pub(crate) const MAX_SOURCE_RADIUS_METERS: f32 = 0.25;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PreparedPointShadowFilter {
    Hard,
    Pcss,
}

pub(crate) fn validate_source_radius(value: f32) -> bool {
    value.is_finite() && (0.0..=MAX_SOURCE_RADIUS_METERS).contains(&value)
}

pub(crate) fn filter_for_source_radius(value: f32) -> PreparedPointShadowFilter {
    if value == 0.0 {
        PreparedPointShadowFilter::Hard
    } else {
        PreparedPointShadowFilter::Pcss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_radius_selects_pcss() {
        assert_eq!(DEFAULT_SOURCE_RADIUS_METERS, 0.05);
        assert_eq!(
            filter_for_source_radius(DEFAULT_SOURCE_RADIUS_METERS),
            PreparedPointShadowFilter::Pcss
        );
    }

    #[test]
    fn zero_radius_selects_hardware_filtering() {
        assert_eq!(
            filter_for_source_radius(0.0),
            PreparedPointShadowFilter::Hard
        );
    }

    #[test]
    fn range_accepts_endpoints_and_rejects_invalid_values() {
        for value in [0.0, 0.25] {
            assert!(validate_source_radius(value));
        }
        for value in [-0.001, 0.251, f32::NAN, f32::INFINITY] {
            assert!(!validate_source_radius(value));
        }
    }
}
