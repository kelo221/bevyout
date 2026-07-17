#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SpecularAlphaRoughnessDefaults {
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) curve: f32,
}

pub(crate) const SPECULAR_ALPHA_ROUGHNESS_DEFAULTS: SpecularAlphaRoughnessDefaults =
    SpecularAlphaRoughnessDefaults {
        min: 0.089,
        max: 0.9,
        curve: 1.0,
    };

pub(crate) fn specular_alpha_roughness_eligible(
    has_normal_map: bool,
    has_specular_map: bool,
    normal_and_specular_share_image: bool,
    has_authored_roughness_map: bool,
    is_non_opaque: bool,
) -> bool {
    has_normal_map
        && has_specular_map
        && normal_and_specular_share_image
        && !has_authored_roughness_map
        && !is_non_opaque
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eligibility_requires_shared_normal_and_specular_without_authored_roughness() {
        assert!(specular_alpha_roughness_eligible(
            true, true, true, false, false
        ));
        assert!(!specular_alpha_roughness_eligible(
            true, true, false, false, false
        ));
        assert!(!specular_alpha_roughness_eligible(
            true, false, true, false, false
        ));
        assert!(!specular_alpha_roughness_eligible(
            true, true, true, true, false
        ));
        assert!(!specular_alpha_roughness_eligible(
            true, true, true, false, true
        ));
    }

    #[test]
    fn defaults_match_the_balanced_profile() {
        assert_eq!(
            SPECULAR_ALPHA_ROUGHNESS_DEFAULTS,
            SpecularAlphaRoughnessDefaults {
                min: 0.089,
                max: 0.9,
                curve: 1.0,
            }
        );
    }
}
