//! Image-space metadata preparation.

use super::*;

pub(crate) fn info_image_space(
    diagnostics: &mut Vec<Diagnostic>,
    image_space: &crate::vsa::manifest::ImageSpaceInfo,
) {
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: format!(
            "resolved ImageSpace {:08x} ({}) eye_adapt_speed={:.3} target_lum={:.3}",
            image_space.form_id,
            image_space.editor_id.as_deref().unwrap_or("<unnamed>"),
            image_space.eye_adapt_speed,
            image_space.hdr_target_lum,
        ),
    });
}
