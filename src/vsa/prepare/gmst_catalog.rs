//! Content-set-wide GMST/AVIF catalog (M9 wave 1, #308).
//!
//! Mirrors `package_catalog.rs`: a pure std/serde module whose plain input
//! types are filled by boundary conversion in `orchestrator.rs`, then
//! serialized to the deterministic fingerprint-keyed path
//! `catalogs/<source_fingerprint>/gmst.ron`. Like packages.ron, the manifest
//! carries no pointer to it -- the path is fully deterministic and consumers
//! (the viewer's stats plugin, #310) read it on demand.
//!
//! The settings view (`GmstSettings`) and its typed value enum live in
//! `bevyout_core::stats` so the pure kernels (#309) consume exactly what
//! this catalog persists, with GOTY defaults for absent settings.

use std::path::{Path, PathBuf};

use anyhow::Result;
use bevyout_core::stats::{GmstSettings, GmstValue};
use serde::{Deserialize, Serialize};

use super::super::paths::fingerprint;

/// Bump whenever this catalog's serialized shape changes, including
/// serde-defaulted fields, per the prepared-asset rule in AGENTS.md.
pub(crate) const GMST_CATALOG_REVISION: &str = "openmw-gmst-v1";

/// The GMST setting names the stat kernels consume (`bevyout_core::stats`
/// constants). Used only for the "consumed" counter below.
const KNOWN_SETTING_NAMES: [&str; 11] = [
    bevyout_core::stats::GMST_HEALTH_ENDURANCE_MULT,
    bevyout_core::stats::GMST_HEALTH_LEVEL_MULT,
    bevyout_core::stats::GMST_ACTION_POINTS_BASE,
    bevyout_core::stats::GMST_ACTION_POINTS_MULT,
    bevyout_core::stats::GMST_CARRY_WEIGHT_BASE,
    bevyout_core::stats::GMST_CARRY_WEIGHT_MULT,
    bevyout_core::stats::GMST_MAX_PLAYER_LEVEL,
    bevyout_core::stats::GMST_LEVEL_UP_SKILL_POINTS_BASE,
    bevyout_core::stats::GMST_LEVEL_UP_SKILL_POINTS_INTERVAL,
    bevyout_core::stats::GMST_XP_BASE,
    bevyout_core::stats::GMST_XP_BUMP_BASE,
];

/// Prepared `AVIF` actor-value metadata: FormID, EditorID, display name,
/// and description.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedActorValueInfo {
    pub(crate) form_id: u32,
    pub(crate) editor_id: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}

/// Plain boundary-conversion inputs; the orchestrator fills these from the
/// parsed plugin chain's `GMST`/`AVIF` records.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct GmstCatalogInputs {
    pub(crate) settings_pairs: Vec<(String, GmstValue)>,
    pub(crate) actor_values: Vec<PreparedActorValueInfo>,
    /// Records whose EditorID carried none of the `f`/`i`/`b`/`s` prefixes
    /// or whose `DATA` failed to decode.
    pub(crate) undecoded: usize,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct GmstCatalogCounters {
    pub(crate) total: usize,
    pub(crate) consumed: usize,
    pub(crate) undecoded: usize,
    pub(crate) actor_values: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedGmstCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    pub(crate) settings: GmstSettings,
    pub(crate) actor_values: Vec<PreparedActorValueInfo>,
    pub(crate) counters: GmstCatalogCounters,
}

impl PreparedGmstCatalog {
    /// Deterministic artifact path relative to the cache root.
    #[must_use]
    pub(crate) fn relative_path(source_fingerprint: &str) -> PathBuf {
        PathBuf::from("catalogs")
            .join(source_fingerprint)
            .join("gmst.ron")
    }
}

pub(crate) fn build_gmst_catalog(
    inputs: &GmstCatalogInputs,
    source_fingerprint: &str,
) -> PreparedGmstCatalog {
    let settings = GmstSettings::from_pairs(
        inputs
            .settings_pairs
            .iter()
            .map(|(name, value)| (name.as_str(), value.clone())),
    );
    let consumed = inputs
        .settings_pairs
        .iter()
        .filter(|(name, _)| {
            KNOWN_SETTING_NAMES
                .iter()
                .any(|known| name.eq_ignore_ascii_case(known))
        })
        .count();
    PreparedGmstCatalog {
        revision: GMST_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        settings,
        actor_values: inputs.actor_values.clone(),
        counters: GmstCatalogCounters {
            total: inputs.settings_pairs.len() + inputs.undecoded,
            consumed,
            undecoded: inputs.undecoded,
            actor_values: inputs.actor_values.len(),
        },
    }
}

/// Writes the deterministic content-set-wide GMST catalog artifact
/// (`catalogs/<fingerprint>/gmst.ron`), mirroring
/// `package_catalog::write_package_catalog`.
pub(crate) fn write_gmst_catalog(
    cache_dir: &Path,
    catalog: &PreparedGmstCatalog,
) -> Result<(String, String)> {
    let relative = PreparedGmstCatalog::relative_path(&catalog.source_fingerprint);
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())
        .map_err(|error| anyhow::anyhow!("failed to serialize gmst catalog: {error}"))?;
    let hash = fingerprint(serialized.as_bytes());
    std::fs::write(&path, &serialized)?;
    Ok((relative.to_string_lossy().replace('\\', "/"), hash))
}

#[cfg(test)]
#[path = "tests/gmst_catalog.rs"]
mod tests;
