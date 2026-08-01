//! Content-addressed preparation for Fallout 3 ImageSpace modifiers.

use anyhow::Result;
use bevyout_core::image_space::{
    IMAGE_SPACE_MODIFIER_CATALOG_REVISION, ImageSpaceModifier, PreparedImageSpaceModifierCatalog,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::super::paths::fingerprint;

pub(crate) fn build_image_space_modifier_catalog(
    modifiers: &HashMap<u32, ImageSpaceModifier>,
    source_fingerprint: &str,
) -> PreparedImageSpaceModifierCatalog {
    let mut modifiers = modifiers.values().cloned().collect::<Vec<_>>();
    modifiers.sort_by_key(|modifier| modifier.form_id);
    PreparedImageSpaceModifierCatalog {
        revision: IMAGE_SPACE_MODIFIER_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        modifiers,
    }
}

pub(crate) fn write_image_space_modifier_catalog(
    cache_dir: &Path,
    catalog: &PreparedImageSpaceModifierCatalog,
) -> Result<(String, String)> {
    let relative = PathBuf::from("catalogs")
        .join(&catalog.source_fingerprint)
        .join("image_space_modifiers.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())?;
    let hash = fingerprint(serialized.as_bytes());
    fs::write(&path, serialized)?;
    Ok((relative.to_string_lossy().replace('\\', "/"), hash))
}

#[cfg(test)]
#[path = "tests/imad.rs"]
mod tests;
