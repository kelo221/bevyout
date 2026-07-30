//! Pure hybrid point-shadow composition policy shared by tests and the shader
//! contract. Visibility factors use `0` for fully occluded and `1` for lit.

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

#[cfg(test)]
#[path = "tests/hybrid_shadow_policy.rs"]
mod tests;
