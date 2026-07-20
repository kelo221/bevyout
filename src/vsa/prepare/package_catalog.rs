//! Prepared AI package catalog (issue #175, M4 wave 11 lane C).
//!
//! Stages the FO3 `PACK` records already decoded by
//! `openmw_esm4::actor_support::parse_package` into a revisioned,
//! runtime-consumable catalog. `actor_catalog.rs` only ever kept a
//! `HashSet<u32>` of referenced package FormIDs for link validation --
//! nothing carried the decoded type/flags/schedule/location/target/
//! conditions through to a runtime-readable artifact until this module.
//!
//! This module is deliberately std/serde-only (no `openmw_esm4`/Bevy
//! imports), mirroring `actor_catalog.rs`'s own module doc comment, so it
//! can be pulled into `tests/features.rs` verbatim via `#[path]`. Boundary
//! conversion from `openmw_esm4::actor_support::PackageRecord` into the
//! plain `PackageInput` below happens in `orchestrator.rs`.
//!
//! Per-actor priority ordering already lives on
//! `ActorBlueprint::package_form_ids` (`actor_catalog.rs`'s
//! `resolve_group!("ai_packages", ai_packages)` walk over `TPLT` chains,
//! unchanged by this module -- see its own test coverage for proof the
//! authored order survives template inheritance without sorting). This
//! catalog supplies the full decoded data those FormIDs point at, content-
//! set-wide like `items.rs`/`recipes.rs` (packages are plugin-wide content,
//! not tied to one cell's placements, so -- unlike the per-cell actor
//! catalog -- this catalog is written once per content-set fingerprint and
//! shared across every cell prepared from it).
//!
//! Conditions (`CTDA`) are carried through as opaque byte payloads only --
//! no evaluator exists yet (#115's runtime layer owns package selection/
//! execution; the full GECK function registry stays with #15).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::super::paths::fingerprint;

/// Bump whenever this catalog's shape changes, even when new fields are
/// serde-defaulted, per the `ITEM_CATALOG_REVISION`/`ACTOR_CATALOG_REVISION`
/// precedent (AGENTS.md's prepared-asset rule).
pub(crate) const PACKAGE_CATALOG_REVISION: &str = "openmw-packages-v1";

/// Highest FO3-documented `PKDT.type` value (fopdoc's Fallout3 PACK page:
/// 0 Find .. 16 Use Weapon, including the page's own undocumented-but-
/// assigned 11). A `package_type` above this is flagged as an unsupported
/// package type rather than silently accepted.
pub(crate) const MAX_KNOWN_PACKAGE_TYPE: u8 = 16;

// ---------------------------------------------------------------------
// Plain input types (boundary conversion happens in orchestrator.rs)
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PackageScheduleInput {
    pub(crate) month: i8,
    pub(crate) day_of_week: i8,
    pub(crate) date: u8,
    pub(crate) time: i8,
    pub(crate) duration: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PackageLocationInput {
    pub(crate) location_type: u32,
    /// `None` for the "Object Type" sentinel (`location_type == 5`) or a
    /// zero raw value, matching `actor_support::parse_package`'s own gate.
    pub(crate) form_id: Option<u32>,
    pub(crate) raw_value: u32,
    pub(crate) radius: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PackageTargetInput {
    pub(crate) target_type: i32,
    /// `None` for the "Object Type" sentinel (`target_type == 2`) or a zero
    /// raw value, matching `actor_support::parse_package`'s own gate.
    pub(crate) form_id: Option<u32>,
    pub(crate) raw_value: u32,
    pub(crate) count_or_distance: i32,
}

/// Plain input mirroring `openmw_esm4::actor_support::PackageRecord`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PackageInput {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) general_flags: u32,
    pub(crate) package_type: u8,
    pub(crate) location: Option<PackageLocationInput>,
    pub(crate) schedule: Option<PackageScheduleInput>,
    pub(crate) target: Option<PackageTargetInput>,
    /// Raw `CTDA` payloads, data only (see module doc comment).
    pub(crate) conditions: Vec<Vec<u8>>,
    /// `PackageRecord::ignored_subrecords` -- subrecords this decode pass
    /// does not understand, diagnosed rather than silently dropped.
    pub(crate) unsupported_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PackageCatalogInputs {
    /// Every decoded `PACK` record in the load order, keyed by FormID.
    pub(crate) packages: HashMap<u32, PackageInput>,
    /// Every FormID this prepare pass decoded (base records and placed
    /// references), used only to diagnose an unresolved `PLDT`/`PTDT`
    /// FormID. Scoped to this decode session exactly like
    /// `ActorCatalogInputs::known_bases` -- a location/target pointing at
    /// content this session legitimately never decoded (a different cell's
    /// reference, for instance) is still flagged, matching the same known
    /// limitation the race/class/faction/package link diagnostics already
    /// accept in `actor_catalog.rs`.
    pub(crate) known_form_ids: HashSet<u32>,
}

// ---------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct PreparedPackageEntry {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) general_flags: u32,
    pub(crate) package_type: u8,
    pub(crate) location: Option<PackageLocationInput>,
    pub(crate) schedule: Option<PackageScheduleInput>,
    pub(crate) target: Option<PackageTargetInput>,
    pub(crate) conditions: Vec<Vec<u8>>,
    /// Stable per-package diagnostics: unsupported package type, unsupported
    /// subrecord(s), unresolved location/target FormID. Sorted and
    /// deduplicated, mirroring `ActorBlueprint::diagnostics`.
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PackageCatalogCounters {
    pub(crate) total: usize,
    pub(crate) unsupported_type: usize,
    pub(crate) unsupported_subrecord: usize,
    pub(crate) unresolved_location: usize,
    pub(crate) unresolved_target: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedPackageCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    /// Sorted by `form_id`.
    pub(crate) packages: Vec<PreparedPackageEntry>,
    pub(crate) counters: PackageCatalogCounters,
}

// ---------------------------------------------------------------------
// Catalog construction
// ---------------------------------------------------------------------

/// Stages every decoded package in `inputs` into a `PreparedPackageEntry`,
/// sorted by FormID for deterministic output.
pub(crate) fn build_package_catalog(
    inputs: &PackageCatalogInputs,
    source_fingerprint: &str,
) -> PreparedPackageCatalog {
    let mut ids: Vec<u32> = inputs.packages.keys().copied().collect();
    ids.sort_unstable();
    let mut counters = PackageCatalogCounters::default();
    let packages = ids
        .into_iter()
        .map(|form_id| {
            let input = &inputs.packages[&form_id];
            let mut diagnostics = Vec::new();
            if input.package_type > MAX_KNOWN_PACKAGE_TYPE {
                counters.unsupported_type += 1;
                diagnostics.push(format!(
                    "package {form_id:08x} has unsupported package type {}",
                    input.package_type
                ));
            }
            if !input.unsupported_subrecords.is_empty() {
                counters.unsupported_subrecord += 1;
                diagnostics.push(format!(
                    "package {form_id:08x} has unsupported subrecord(s): {}",
                    input.unsupported_subrecords.join(",")
                ));
            }
            if let Some(location) = input.location
                && let Some(target) = location.form_id
                && !inputs.known_form_ids.contains(&target)
            {
                counters.unresolved_location += 1;
                diagnostics.push(format!(
                    "package {form_id:08x} location references unresolved FormID {target:08x}"
                ));
            }
            if let Some(target_spec) = input.target
                && let Some(target) = target_spec.form_id
                && !inputs.known_form_ids.contains(&target)
            {
                counters.unresolved_target += 1;
                diagnostics.push(format!(
                    "package {form_id:08x} target references unresolved FormID {target:08x}"
                ));
            }
            diagnostics.sort();
            diagnostics.dedup();
            counters.total += 1;
            PreparedPackageEntry {
                form_id,
                editor_id: input.editor_id.clone(),
                general_flags: input.general_flags,
                package_type: input.package_type,
                location: input.location,
                schedule: input.schedule,
                target: input.target,
                conditions: input.conditions.clone(),
                diagnostics,
            }
        })
        .collect();
    PreparedPackageCatalog {
        revision: PACKAGE_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        packages,
        counters,
    }
}

// ---------------------------------------------------------------------
// Artifact I/O
// ---------------------------------------------------------------------

/// Writes the deterministic content-set-wide package catalog artifact
/// (`catalogs/<fingerprint>/packages.ron`), mirroring
/// `items::write_item_catalog` -- packages are plugin-wide content like
/// items/recipes, not per-cell like `actor_catalog::write_actor_catalog`'s
/// `actors.ron`.
pub(crate) fn write_package_catalog(
    cache_dir: &Path,
    catalog: &PreparedPackageCatalog,
) -> Result<(String, String)> {
    let relative = PathBuf::from("catalogs")
        .join(&catalog.source_fingerprint)
        .join("packages.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())
        .map_err(|error| anyhow::anyhow!("failed to serialize package catalog: {error}"))?;
    let hash = fingerprint(serialized.as_bytes());
    std::fs::write(&path, &serialized)?;
    Ok((relative.to_string_lossy().replace('\\', "/"), hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(form_id: u32) -> PackageInput {
        PackageInput {
            form_id,
            ..PackageInput::default()
        }
    }

    #[test]
    fn revision_is_pinned() {
        assert_eq!(PACKAGE_CATALOG_REVISION, "openmw-packages-v1");
    }

    #[test]
    fn built_catalogs_carry_the_pinned_revision() {
        let catalog = build_package_catalog(&PackageCatalogInputs::default(), "fp");
        assert_eq!(catalog.revision, PACKAGE_CATALOG_REVISION);
        assert_eq!(catalog.source_fingerprint, "fp");
    }

    #[test]
    fn catalog_is_sorted_by_form_id() {
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x30, package(0x30)), (0x10, package(0x10))]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(
            catalog
                .packages
                .iter()
                .map(|p| p.form_id)
                .collect::<Vec<_>>(),
            vec![0x10, 0x30]
        );
        assert_eq!(catalog.counters.total, 2);
    }

    #[test]
    fn unsupported_package_type_is_diagnosed() {
        let mut input = package(0x10);
        input.package_type = 200;
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unsupported_type, 1);
        assert!(
            catalog.packages[0]
                .diagnostics
                .iter()
                .any(|message| message.contains("unsupported package type 200"))
        );
    }

    #[test]
    fn known_package_types_are_not_flagged() {
        for package_type in 0..=MAX_KNOWN_PACKAGE_TYPE {
            let mut input = package(0x10);
            input.package_type = package_type;
            let inputs = PackageCatalogInputs {
                packages: HashMap::from([(0x10, input)]),
                ..PackageCatalogInputs::default()
            };
            let catalog = build_package_catalog(&inputs, "fp");
            assert_eq!(catalog.counters.unsupported_type, 0, "type {package_type}");
        }
    }

    #[test]
    fn unsupported_subrecord_is_diagnosed() {
        let mut input = package(0x10);
        input.unsupported_subrecords = vec!["XNAM".into()];
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unsupported_subrecord, 1);
        assert!(
            catalog.packages[0]
                .diagnostics
                .iter()
                .any(|message| message.contains("unsupported subrecord(s): XNAM"))
        );
    }

    #[test]
    fn unresolved_location_formid_is_diagnosed() {
        let mut input = package(0x10);
        input.location = Some(PackageLocationInput {
            location_type: 0,
            form_id: Some(0xDEAD),
            raw_value: 0xDEAD,
            radius: 0,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unresolved_location, 1);
        assert!(
            catalog.packages[0]
                .diagnostics
                .iter()
                .any(|message| message.contains("location references unresolved FormID 0000dead"))
        );
    }

    #[test]
    fn unresolved_target_formid_is_diagnosed() {
        let mut input = package(0x10);
        input.target = Some(PackageTargetInput {
            target_type: 0,
            form_id: Some(0xBEEF),
            raw_value: 0xBEEF,
            count_or_distance: 1,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unresolved_target, 1);
        assert!(
            catalog.packages[0]
                .diagnostics
                .iter()
                .any(|message| message.contains("target references unresolved FormID 0000beef"))
        );
    }

    #[test]
    fn resolved_location_and_target_are_not_flagged() {
        let mut input = package(0x10);
        input.location = Some(PackageLocationInput {
            location_type: 0,
            form_id: Some(0x20),
            raw_value: 0x20,
            radius: 0,
        });
        input.target = Some(PackageTargetInput {
            target_type: 0,
            form_id: Some(0x20),
            raw_value: 0x20,
            count_or_distance: 1,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            known_form_ids: HashSet::from([0x20]),
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unresolved_location, 0);
        assert_eq!(catalog.counters.unresolved_target, 0);
        assert!(catalog.packages[0].diagnostics.is_empty());
    }

    /// The "Object Type" sentinel (`location_type == 5` / `target_type ==
    /// 2`) always decodes to `form_id: None` upstream in
    /// `actor_support::parse_package` -- proves the catalog never invents an
    /// unresolved-link diagnostic for a `None` form_id regardless of cause.
    #[test]
    fn a_none_form_id_is_never_flagged_as_unresolved() {
        let mut input = package(0x10);
        input.location = Some(PackageLocationInput {
            location_type: 5,
            form_id: None,
            raw_value: 3,
            radius: 0,
        });
        input.target = Some(PackageTargetInput {
            target_type: 2,
            form_id: None,
            raw_value: 3,
            count_or_distance: 1,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unresolved_location, 0);
        assert_eq!(catalog.counters.unresolved_target, 0);
    }

    #[test]
    fn serialization_is_deterministic_across_runs() {
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x30, package(0x30)), (0x10, package(0x10))]),
            ..PackageCatalogInputs::default()
        };
        let a = build_package_catalog(&inputs, "fp");
        let b = build_package_catalog(&inputs, "fp");
        let ron_a = ron::ser::to_string_pretty(&a, ron::ser::PrettyConfig::default()).unwrap();
        let ron_b = ron::ser::to_string_pretty(&b, ron::ser::PrettyConfig::default()).unwrap();
        assert_eq!(ron_a, ron_b);
    }

    #[test]
    fn writes_content_set_wide_artifact_under_catalogs() {
        let cache_dir = std::env::temp_dir().join(format!(
            "bevyout-package-catalog-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
        ));
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, package(0x10))]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "shared-fp");
        let (relative, hash) = write_package_catalog(&cache_dir, &catalog).unwrap();
        assert_eq!(relative, "catalogs/shared-fp/packages.ron");
        let bytes = std::fs::read(cache_dir.join(&relative)).unwrap();
        assert_eq!(fingerprint(&bytes), hash);
        std::fs::remove_dir_all(&cache_dir).unwrap();
    }
}
