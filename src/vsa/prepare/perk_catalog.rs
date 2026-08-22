//! Content-set-wide perk catalog (M9 wave 2, #312).
//!
//! Mirrors `gmst_catalog.rs`: a pure std/serde module whose plain input
//! type (core `PerkDefinition`s) is filled by boundary conversion in
//! `orchestrator.rs`, then serialized to the deterministic fingerprint-
//! keyed path `catalogs/<source_fingerprint>/perks.ron`. The manifest
//! carries no pointer to it -- the path is fully deterministic and the
//! viewer's stats plugin (#314) reads it on demand.
//!
//! The definitions themselves are `bevyout_core::perks` types so the pure
//! evaluator (#313) consumes exactly what this catalog persists.

use std::path::{Path, PathBuf};

use anyhow::Result;
use bevyout_core::perks::PerkDefinition;
use serde::{Deserialize, Serialize};

use super::super::paths::fingerprint;

/// Bump whenever this catalog's serialized shape changes, including
/// serde-defaulted fields, per the prepared-asset rule in AGENTS.md.
pub(crate) const PERK_CATALOG_REVISION: &str = "openmw-perks-v2";

/// Plain boundary-conversion inputs; the orchestrator fills these from the
/// parsed plugin chain's `PERK` records with raw CTDA/entry words already
/// resolved through the core perk kernels.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PerkCatalogInputs {
    pub(crate) perks: Vec<PerkDefinition>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PerkCatalogCounters {
    pub(crate) total: usize,
    pub(crate) playable: usize,
    pub(crate) hidden: usize,
    /// Decoded `CTDA` conditions the evaluator cannot run (non-
    /// `GetActorValue` functions, other opers, unmapped AV indices).
    pub(crate) unknown_conditions: usize,
    pub(crate) quest_entries: usize,
    pub(crate) ability_entries: usize,
    pub(crate) entry_point_entries: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedPerkCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    /// Sorted by FormID for deterministic serialization.
    pub(crate) entries: Vec<PerkDefinition>,
    pub(crate) counters: PerkCatalogCounters,
}

impl PreparedPerkCatalog {
    /// Deterministic artifact path relative to the cache root.
    #[must_use]
    pub(crate) fn relative_path(source_fingerprint: &str) -> PathBuf {
        PathBuf::from("catalogs")
            .join(source_fingerprint)
            .join("perks.ron")
    }
}

pub(crate) fn build_perk_catalog(
    inputs: &PerkCatalogInputs,
    source_fingerprint: &str,
) -> PreparedPerkCatalog {
    let mut entries = inputs.perks.clone();
    entries.sort_by_key(|perk| perk.form_id);
    let counters = PerkCatalogCounters {
        total: entries.len(),
        playable: entries.iter().filter(|perk| perk.playable).count(),
        hidden: entries.iter().filter(|perk| perk.hidden).count(),
        unknown_conditions: entries
            .iter()
            .map(|perk| perk.unknown_conditions as usize)
            .sum(),
        quest_entries: entries
            .iter()
            .flat_map(|perk| perk.entries.iter())
            .filter(|entry| matches!(entry, bevyout_core::perks::PerkEntry::Quest { .. }))
            .count(),
        ability_entries: entries
            .iter()
            .flat_map(|perk| perk.entries.iter())
            .filter(|entry| matches!(entry, bevyout_core::perks::PerkEntry::Ability { .. }))
            .count(),
        entry_point_entries: entries
            .iter()
            .flat_map(|perk| perk.entries.iter())
            .filter(|entry| matches!(entry, bevyout_core::perks::PerkEntry::EntryPoint { .. }))
            .count(),
    };
    PreparedPerkCatalog {
        revision: PERK_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        entries,
        counters,
    }
}

/// Writes the deterministic content-set-wide perk catalog artifact
/// (`catalogs/<fingerprint>/perks.ron`), mirroring
/// `gmst_catalog::write_gmst_catalog`.
pub(crate) fn write_perk_catalog(
    cache_dir: &Path,
    catalog: &PreparedPerkCatalog,
) -> Result<(String, String)> {
    let relative = PreparedPerkCatalog::relative_path(&catalog.source_fingerprint);
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())
        .map_err(|error| anyhow::anyhow!("failed to serialize perk catalog: {error}"))?;
    let hash = fingerprint(serialized.as_bytes());
    std::fs::write(&path, &serialized)?;
    Ok((relative.to_string_lossy().replace('\\', "/"), hash))
}

#[cfg(test)]
#[path = "tests/perk_catalog.rs"]
mod tests;
