//! Pure glTF extension policy for the Rust static-scene composer.

use serde_json::Value;

const SUPPORTED_REQUIRED_EXTENSIONS: &[&str] = &[
    "KHR_materials_emissive_strength",
    "KHR_materials_specular",
    "KHR_materials_volume",
    "KHR_materials_unlit",
];

pub(crate) fn unsupported_required_extensions<'a>(
    required: impl IntoIterator<Item = &'a str>,
) -> Vec<String> {
    let mut unsupported = required
        .into_iter()
        .filter(|extension| !SUPPORTED_REQUIRED_EXTENSIONS.contains(extension))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    unsupported.sort();
    unsupported.dedup();
    unsupported
}

pub(crate) fn material_extensions_used(materials: &[Value]) -> Vec<String> {
    let mut extensions = materials
        .iter()
        .filter_map(|material| material.get("extensions").and_then(Value::as_object))
        .flat_map(|extensions| extensions.keys().cloned())
        .collect::<Vec<_>>();
    extensions.sort();
    extensions.dedup();
    extensions
}

#[cfg(test)]
#[path = "tests/gltf_extension_policy.rs"]
mod tests;
