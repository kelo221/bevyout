//! Content-set-wide effect catalog (M9 wave 3, #316).
//!
//! Mirrors `perk_catalog.rs`: a pure std/serde module whose plain input
//! type (core `IngestibleDefinition`s and `EffectDefinition`s) is filled
//! by boundary conversion in `orchestrator.rs`, then serialized to the
//! deterministic fingerprint-keyed path
//! `catalogs/<source_fingerprint>/effects.ron`. Like perks.ron, the
//! manifest carries no pointer to it -- the path is fully deterministic
//! and the viewer's chem runtime (#318) reads it on demand.
//!
//! The definitions themselves are `bevyout_core::effects` types so the
//! pure kernels (#317) consume exactly what this catalog persists.

use std::path::{Path, PathBuf};

use anyhow::Result;
use bevyout_core::effects::{EffectDefinition, IngestibleDefinition};
use serde::{Deserialize, Serialize};

use super::super::paths::fingerprint;

/// Bump whenever this catalog's serialized shape changes, including
/// serde-defaulted fields, per the prepared-asset rule in AGENTS.md.
pub(crate) const EFFECT_CATALOG_REVISION: &str = "openmw-effects-v1";

/// Plain boundary-conversion inputs; the orchestrator fills these from the
/// parsed plugin chain's `ALCH`/`MGEF` records with the associated actor
/// values already resolved through the core effect kernels.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct EffectCatalogInputs {
    pub(crate) ingestibles: Vec<IngestibleDefinition>,
    pub(crate) effects: Vec<EffectDefinition>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct EffectCatalogCounters {
    /// Decoded `ALCH` ingestibles.
    pub(crate) ingestibles: usize,
    /// Decoded `MGEF` base effects.
    pub(crate) effects: usize,
    /// Ingestibles with a non-zero addiction chance and a withdrawal
    /// effect (the engine's two requirements for an addictive chem).
    pub(crate) addictive: usize,
    /// Effect items whose MGEF FormID resolved to no decoded record (the
    /// engine-builtin UMON monitor `0x0000014F` on real data) plus items
    /// with an unmapped associated actor value: cataloged, not applied.
    pub(crate) unresolved_effects: usize,
    /// Effect items gated by a `CTDA` condition: stored, never run (the
    /// wave-2 conservative-contract pattern).
    pub(crate) conditioned_effects: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedEffectCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    /// Sorted by FormID for deterministic serialization.
    pub(crate) ingestibles: Vec<IngestibleDefinition>,
    /// Sorted by FormID for deterministic serialization.
    pub(crate) effects: Vec<EffectDefinition>,
    pub(crate) counters: EffectCatalogCounters,
}

impl PreparedEffectCatalog {
    /// Deterministic artifact path relative to the cache root.
    #[must_use]
    pub(crate) fn relative_path(source_fingerprint: &str) -> PathBuf {
        PathBuf::from("catalogs")
            .join(source_fingerprint)
            .join("effects.ron")
    }
}

pub(crate) fn build_effect_catalog(
    inputs: &EffectCatalogInputs,
    source_fingerprint: &str,
) -> PreparedEffectCatalog {
    let mut ingestibles = inputs.ingestibles.clone();
    ingestibles.sort_by_key(|ingestible| ingestible.form_id);
    let mut effects = inputs.effects.clone();
    effects.sort_by_key(|effect| effect.form_id);
    let counters = EffectCatalogCounters {
        ingestibles: ingestibles.len(),
        effects: effects.len(),
        addictive: ingestibles.iter().filter(|i| i.addictive()).count(),
        unresolved_effects: ingestibles
            .iter()
            .flat_map(|ingestible| ingestible.effects.iter())
            .filter(|effect| effect.actor_value.is_none())
            .count(),
        conditioned_effects: ingestibles
            .iter()
            .flat_map(|ingestible| ingestible.effects.iter())
            .filter(|effect| effect.conditioned)
            .count(),
    };
    PreparedEffectCatalog {
        revision: EFFECT_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        ingestibles,
        effects,
        counters,
    }
}

/// Writes the deterministic content-set-wide effect catalog artifact
/// (`catalogs/<fingerprint>/effects.ron`), mirroring
/// `perk_catalog::write_perk_catalog`.
pub(crate) fn write_effect_catalog(
    cache_dir: &Path,
    catalog: &PreparedEffectCatalog,
) -> Result<(String, String)> {
    let relative = PreparedEffectCatalog::relative_path(&catalog.source_fingerprint);
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())
        .map_err(|error| anyhow::anyhow!("failed to serialize effect catalog: {error}"))?;
    let hash = fingerprint(serialized.as_bytes());
    std::fs::write(&path, &serialized)?;
    Ok((relative.to_string_lossy().replace('\\', "/"), hash))
}

#[cfg(test)]
#[path = "tests/effect_catalog.rs"]
mod tests;
