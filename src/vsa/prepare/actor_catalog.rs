//! Pure actor-catalog resolution (issue #103, M4 wave 1 task C).
//!
//! Resolves every `NPC_`/`CREA` base reachable from a prepared cell's
//! `ACHR`/`ACRE` references into an [`ActorBlueprint`], applying `ACBS`
//! template flags field-by-field through `TPLT` chains. This module is
//! deliberately std/serde-only (plus the already-approved `anyhow`/`ron`
//! helper crates other pure `vsa::prepare` seams use, e.g. `jobs.rs`) with
//! no `openmw_esm4`/Bevy imports, so it can be pulled into `tests/features.rs`
//! verbatim via `#[path]` the same way `selectors.rs`/`fingerprints.rs` are --
//! see those modules' doc comments and `AGENTS.md`'s testing section for the
//! pattern.
//!
//! Boundary conversion from the parser's `ActorData`/`RaceRecord`/
//! `ClassRecord`/`FactionRecord`/`PackageRecord`/`ReferenceRecord` types into
//! the plain input types below happens in `orchestrator.rs`, mirroring how
//! `cell_map::CellMap::build` is fed by `catalog.rs` rather than importing
//! the ESM4 reader itself.
//!
//! The one exception to "plain input types" is `inventory`, which reuses
//! `vsa::manifest::PreparedInventoryEntry` directly (via the same
//! `super::super::manifest` path `fingerprints.rs` uses for
//! `vsa::assets::NIF_CONVERTER_REVISION`) rather than inventing a parallel
//! item shape, per this task's brief: initial inventory reuses the existing
//! prepared item-catalog/inventory-entry contract from #95/#98.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::super::manifest::PreparedInventoryEntry;
use super::super::paths::fingerprint;

/// Bump whenever the catalog shape changes, even when new fields are
/// serde-defaulted, per the `ITEM_CATALOG_REVISION`/`RECIPE_CATALOG_REVISION`
/// precedent (a stale cached `actors.ron` would otherwise deserialize
/// silently with defaulted fields).
pub(crate) const ACTOR_CATALOG_REVISION: &str = "openmw-actors-v2-appearance-seeded-levels";

/// Maximum number of concrete `NPC_`/`CREA` nodes in one `TPLT` chain,
/// including the starting actor itself (`build_chain` checks `nodes.len()`
/// before pushing the next node, so the maximum number of `TPLT` hops
/// followed is `MAX_TEMPLATE_CHAIN_DEPTH - 1`); a chain that would grow
/// past this is diagnosed as too deep. Bounds pathological (but non-cyclic)
/// content without an unbounded walk; ordinary FO3 template chains are one
/// or two hops.
const MAX_TEMPLATE_CHAIN_DEPTH: usize = 32;

// ---------------------------------------------------------------------
// Plain input types (boundary conversion happens in orchestrator.rs)
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ActorRecordKind {
    #[default]
    Npc,
    Creature,
}

impl ActorRecordKind {
    pub(crate) fn as_record_signature(self) -> &'static str {
        match self {
            Self::Npc => "NPC_",
            Self::Creature => "CREA",
        }
    }
}

/// Which of the 10 documented `ACBS.template_flags` bits (FO3 fopdoc's
/// `ACBS` page) this actor's own record sets, pre-computed by the
/// orchestrator from `ActorBaseConfig::uses_template_flag` so this module
/// never needs the bitmask constants themselves. `unsupported_bits` carries
/// any bits outside the documented 10 (diagnosed, never silently dropped).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ActorTemplateUsage {
    pub(crate) traits: bool,
    pub(crate) stats: bool,
    pub(crate) factions: bool,
    pub(crate) actor_effect_list: bool,
    pub(crate) ai_data: bool,
    pub(crate) ai_packages: bool,
    pub(crate) model_animation: bool,
    pub(crate) base_data: bool,
    pub(crate) inventory: bool,
    pub(crate) script: bool,
    pub(crate) unsupported_bits: u16,
}

impl ActorTemplateUsage {
    fn any(self) -> bool {
        self.traits
            || self.stats
            || self.factions
            || self.actor_effect_list
            || self.ai_data
            || self.ai_packages
            || self.model_animation
            || self.base_data
            || self.inventory
            || self.script
    }
}

/// `USE_TRAITS` group: race/sex-relevant traits, height/weight, hair/eyes/
/// head parts, and voice. `female` is decoded from `ACBS.flags` bit `0x1`
/// (the standard "Female" bit shared by `ACBS` across Bethesda's ESM/ESM4
/// games, confirmed against the FO3 fopdoc `ACBS` page) and travels with
/// this group rather than `base_data`, since the task brief bundles
/// "race/sex-relevant traits" under `USE_TRAITS`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorTraits {
    pub(crate) race_form_id: Option<u32>,
    pub(crate) female: bool,
    pub(crate) height: Option<f32>,
    pub(crate) weight: Option<f32>,
    pub(crate) hair_form_id: Option<u32>,
    pub(crate) eyes_form_id: Option<u32>,
    pub(crate) head_part_form_ids: Vec<u32>,
    pub(crate) voice_form_id: Option<u32>,
    /// Whether any `FGGS`/`FGGA`/`FGTS` FaceGen bytes were present on the
    /// source record. The bytes themselves are opaque per task A/B and are
    /// not carried into the catalog; only their presence is tracked.
    pub(crate) facegen_present: bool,
}

/// `USE_STATS` group: level, S.P.E.C.I.A.L., skills, karma, speed, fatigue,
/// barter gold. `disposition_base` travels with this group too (it is
/// another numeric `ACBS` field with no group of its own in the task brief).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct ActorStats {
    pub(crate) level_or_mult: i16,
    pub(crate) calc_min_level: u16,
    pub(crate) calc_max_level: u16,
    pub(crate) speed_multiplier: u16,
    pub(crate) karma: f32,
    pub(crate) disposition_base: i16,
    pub(crate) fatigue: u16,
    pub(crate) barter_gold: u16,
    /// `NPC_.DATA` base health, or `CREA.DATA` health widened to `i32`.
    pub(crate) health: Option<i32>,
    /// Strength, Perception, Endurance, Charisma, Intelligence, Agility,
    /// Luck -- shared shape between `NPC_.DATA` and `CREA.DATA`.
    pub(crate) special: Option<[u8; 7]>,
    /// `NPC_.DNAM` skill values (offsets are not carried into the catalog).
    pub(crate) npc_skill_values: Option<[u8; 14]>,
    pub(crate) creature_type: Option<u8>,
    pub(crate) combat_skill: Option<u8>,
    pub(crate) magic_skill: Option<u8>,
    pub(crate) stealth_skill: Option<u8>,
    pub(crate) creature_damage: Option<i16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActorFactionInput {
    pub(crate) faction_form_id: u32,
    pub(crate) rank: i8,
}

/// `USE_MODEL_ANIMATION` group: the `NPC_`/`CREA` base's own `MODL` model
/// plus `CREA`-only `NIFZ`/`KFFZ` candidate lists.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorModelAnimation {
    pub(crate) model_path: Option<String>,
    pub(crate) creature_model_list: Vec<String>,
    pub(crate) creature_animation_files: Vec<String>,
}

/// `USE_BASE_DATA` group: display name plus the raw `ACBS.flags` (essential/
/// respawn/etc. bits are not individually decoded -- only the `female` bit
/// is, and it lives on `ActorTraits` -- so the whole mask is retained for
/// future consumers).
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorBaseData {
    pub(crate) name: Option<String>,
    pub(crate) acbs_flags: u32,
}

/// `AIDT`, mirrored from `openmw_esm4::actors::ActorAiData` without
/// importing it (see the module doc comment).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ActorAiDataInput {
    pub(crate) aggression: u8,
    pub(crate) confidence: u8,
    pub(crate) energy_level: u8,
    pub(crate) responsibility: u8,
    pub(crate) mood: u8,
    pub(crate) services: u32,
    pub(crate) teaches: i8,
    pub(crate) max_training_level: u8,
    pub(crate) assistance: i8,
    pub(crate) aggro_radius_behavior: u8,
    pub(crate) aggro_radius: i32,
}

/// One `NPC_`/`CREA` base record's actor-relevant fields, grouped to match
/// the 10 `ACBS.template_flags` bits one-for-one so `resolve_group_source`
/// can pick a group's value from whichever chain node ultimately supplies
/// it, generically.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorRecordInput {
    pub(crate) form_id: u32,
    pub(crate) kind: ActorRecordKind,
    pub(crate) editor_id: Option<String>,
    /// `TPLT`, shared by every record kind (not actor-specific).
    pub(crate) base_template_form_id: Option<u32>,
    pub(crate) template_usage: ActorTemplateUsage,
    pub(crate) traits: ActorTraits,
    pub(crate) stats: ActorStats,
    pub(crate) factions: Vec<ActorFactionInput>,
    /// `EITM`: `NPC_`'s unarmed attack effect / innate actor effect list.
    pub(crate) actor_effect_form_id: Option<u32>,
    pub(crate) ai_data: Option<ActorAiDataInput>,
    pub(crate) package_form_ids: Vec<u32>,
    pub(crate) model_animation: ActorModelAnimation,
    pub(crate) base_data: ActorBaseData,
    pub(crate) inventory: Vec<PreparedInventoryEntry>,
    pub(crate) script_form_id: Option<u32>,
    /// Not template-gated per the task brief's 10 named groups: carried
    /// straight from the record, never resolved through `TPLT`.
    pub(crate) class_form_id: Option<u32>,
    pub(crate) combat_style_form_id: Option<u32>,
    pub(crate) death_item_form_id: Option<u32>,
}

/// A resolved `LVLN`/`LVLC` leveled-list base reachable from a `TPLT` chain
/// (or directly as an `ACRE`/`ACHR` base -- see `ActorCatalogEntry::Skipped`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct LeveledInput {
    pub(crate) form_id: u32,
    /// Every `LVLO` entry's base FormID, in source order (may repeat).
    pub(crate) entries: Vec<u32>,
}

/// One `FACT` rank: number plus per-sex titles.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct FactionRankInput {
    pub(crate) rank_number: i32,
    pub(crate) male_title: Option<String>,
    pub(crate) female_title: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct FactionInput {
    pub(crate) form_id: u32,
    pub(crate) ranks: Vec<FactionRankInput>,
}

/// One `ACHR`/`ACRE` placement's identity, transform, and enable state,
/// already resolved (`initially_enabled` folds in the XESP enable-parent
/// chain and `RECORD_DISABLED`, per `openmw_esm4::enable::resolve_initially_enabled`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ActorPlacementInput {
    pub(crate) reference_form_id: u32,
    pub(crate) base_form_id: u32,
    pub(crate) kind: ActorRecordKind,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: f32,
    pub(crate) initially_enabled: bool,
}

impl Default for ActorPlacementInput {
    fn default() -> Self {
        Self {
            reference_form_id: 0,
            base_form_id: 0,
            kind: ActorRecordKind::default(),
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            initially_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ActorCatalogInputs {
    pub(crate) actors: HashMap<u32, ActorRecordInput>,
    pub(crate) leveled: HashMap<u32, LeveledInput>,
    pub(crate) races: HashSet<u32>,
    pub(crate) classes: HashSet<u32>,
    pub(crate) factions: HashMap<u32, FactionInput>,
    pub(crate) packages: HashSet<u32>,
    /// FormIDs of every decoded base record, used to check hair/eyes/
    /// head-part links. `HAIR`/`EYES`/`HDPT` are not yet decoded record
    /// kinds (see `openmw_esm4::records::is_supported_base`), so those
    /// links are honestly diagnosed unresolved today; this set is forward
    /// compatible with a future task that adds them.
    pub(crate) known_bases: HashSet<u32>,
    pub(crate) placements: Vec<ActorPlacementInput>,
}

// ---------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub(crate) struct ActorFactionMembership {
    pub(crate) faction_form_id: u32,
    pub(crate) rank: i8,
    pub(crate) title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct ActorBlueprint {
    pub(crate) base_form_id: u32,
    /// Original placement base. For a direct actor this equals
    /// `base_form_id`; for an LVLN/LVLC placement it preserves the list while
    /// `base_form_id` names the deterministic concrete candidate.
    #[serde(default)]
    pub(crate) source_base_form_id: u32,
    #[serde(default)]
    pub(crate) resolved_base_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) selection_seed: Option<u64>,
    pub(crate) reference_form_id: u32,
    pub(crate) record_kind: String,
    pub(crate) editor_id: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) model_path: Option<String>,
    pub(crate) creature_model_list: Vec<String>,
    pub(crate) animation_candidates: Vec<String>,
    pub(crate) race_form_id: Option<u32>,
    pub(crate) female: bool,
    pub(crate) height: Option<f32>,
    pub(crate) weight: Option<f32>,
    pub(crate) hair_form_id: Option<u32>,
    pub(crate) eyes_form_id: Option<u32>,
    pub(crate) head_part_form_ids: Vec<u32>,
    pub(crate) voice_form_id: Option<u32>,
    pub(crate) level_or_mult: i16,
    pub(crate) calc_min_level: u16,
    pub(crate) calc_max_level: u16,
    pub(crate) speed_multiplier: u16,
    pub(crate) karma: f32,
    pub(crate) disposition_base: i16,
    pub(crate) fatigue: u16,
    pub(crate) barter_gold: u16,
    pub(crate) health: Option<i32>,
    pub(crate) special: Option<[u8; 7]>,
    pub(crate) npc_skill_values: Option<[u8; 14]>,
    pub(crate) creature_type: Option<u8>,
    pub(crate) combat_skill: Option<u8>,
    pub(crate) magic_skill: Option<u8>,
    pub(crate) stealth_skill: Option<u8>,
    pub(crate) creature_damage: Option<i16>,
    pub(crate) factions: Vec<ActorFactionMembership>,
    pub(crate) actor_effect_form_id: Option<u32>,
    pub(crate) ai_data: Option<ActorAiDataInput>,
    /// `PKID` order, in source file order.
    pub(crate) package_form_ids: Vec<u32>,
    pub(crate) class_form_id: Option<u32>,
    pub(crate) combat_style_form_id: Option<u32>,
    pub(crate) death_item_form_id: Option<u32>,
    pub(crate) script_form_id: Option<u32>,
    pub(crate) inventory: Vec<PreparedInventoryEntry>,
    /// `true` when this actor's `TPLT` chain passed through an `LVLN`/
    /// `LVLC` leveled list rather than resolving to a single concrete
    /// actor; `template_candidates` then carries every leveled entry's
    /// FormID, sorted and deduplicated (F103.C, "record the resolved
    /// candidate set deterministically").
    pub(crate) is_leveled_template: bool,
    pub(crate) template_candidates: Vec<u32>,
    pub(crate) base_template_form_id: Option<u32>,
    /// `true` when at least one of the 10 field groups above was pulled
    /// from a template rather than this actor's own record.
    pub(crate) inherited: bool,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: f32,
    pub(crate) initially_enabled: bool,
    /// Stable diagnostic strings: template cycles, missing/unresolved
    /// template targets, unresolved race/class/faction/package/hair/eyes/
    /// head-part links, and unsupported template flag bits. Sorted and
    /// deduplicated.
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum ActorCatalogEntry {
    Prepared(Box<ActorBlueprint>),
    /// The placement's base FormID does not resolve to any decoded record
    /// at all (missing base record).
    Unresolved {
        base_form_id: u32,
        reference_form_id: u32,
        reason: String,
    },
    /// The placement's base FormID resolves to a record whose kind does not
    /// match the reference kind (`ACHR` pointing at a `CREA` base, etc.).
    Unsupported {
        base_form_id: u32,
        reference_form_id: u32,
        reason: String,
    },
    /// The placement's base FormID resolves to an `LVLN`/`LVLC` leveled
    /// list rather than a concrete actor; direct leveled placements are
    /// not resolved into blueprints (unlike leveled targets reached mid
    /// `TPLT` chain, which populate `template_candidates` instead).
    Skipped {
        base_form_id: u32,
        reference_form_id: u32,
        reason: String,
    },
}

fn entry_sort_key(entry: &ActorCatalogEntry) -> (u32, u32) {
    match entry {
        ActorCatalogEntry::Prepared(blueprint) => {
            (blueprint.base_form_id, blueprint.reference_form_id)
        }
        ActorCatalogEntry::Unresolved {
            base_form_id,
            reference_form_id,
            ..
        }
        | ActorCatalogEntry::Unsupported {
            base_form_id,
            reference_form_id,
            ..
        }
        | ActorCatalogEntry::Skipped {
            base_form_id,
            reference_form_id,
            ..
        } => (*base_form_id, *reference_form_id),
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ActorCatalogCounters {
    pub(crate) prepared: usize,
    pub(crate) inherited: usize,
    pub(crate) unresolved: usize,
    pub(crate) unsupported: usize,
    pub(crate) skipped: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedActorCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    pub(crate) entries: Vec<ActorCatalogEntry>,
    pub(crate) counters: ActorCatalogCounters,
}

// ---------------------------------------------------------------------
// Template chain resolution
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum ChainEnd {
    /// The last concrete node in the chain has no `TPLT` of its own.
    NoTemplate,
    /// The last concrete node's `TPLT` targets an `LVLN`/`LVLC` leveled
    /// list. `candidates` is every `LVLO` entry's FormID, sorted+deduped.
    Leveled {
        target_form_id: u32,
        candidates: Vec<u32>,
    },
    /// The last concrete node's `TPLT` targets a FormID absent from both
    /// the actor and leveled-list input sets.
    Missing { target_form_id: u32 },
    /// The last concrete node's `TPLT` targets a FormID already visited
    /// earlier in the chain.
    Cycle { target_form_id: u32 },
    /// The chain exceeded `MAX_TEMPLATE_CHAIN_DEPTH` concrete hops.
    TooDeep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TemplateChain {
    /// Concrete `NPC_`/`CREA` node FormIDs in walk order; `nodes[0]` is
    /// always the starting actor itself.
    nodes: Vec<u32>,
    end: ChainEnd,
}

impl TemplateChain {
    /// A one-node chain that never resolves anything from a template.
    /// Used whenever `ActorTemplateUsage::any()` is `false`, so an unused
    /// (but dangling or cyclic) `TPLT` on a record that opts out of every
    /// inheritance group produces no diagnostics and no leveled-candidate
    /// noise -- nothing in this actor's blueprint actually depends on it.
    fn trivial(start: u32) -> Self {
        Self {
            nodes: vec![start],
            end: ChainEnd::NoTemplate,
        }
    }
}

/// Follows `base_template_form_id` from `start` through concrete `NPC_`/
/// `CREA` nodes until the chain ends: no further `TPLT`, a leveled-list
/// target, a missing target, a cycle, or the depth guard. The walk does not
/// consult `ActorTemplateUsage` at all -- a node's `TPLT` link is a single
/// fact about that record, independent of which field groups any walker
/// cares about; `resolve_group_source` below decides how far along this
/// shared chain a given group actually travels.
fn build_chain(
    start: u32,
    actors: &HashMap<u32, ActorRecordInput>,
    leveled: &HashMap<u32, LeveledInput>,
) -> TemplateChain {
    let mut nodes = vec![start];
    let mut visited = HashSet::from([start]);
    loop {
        let current_id = *nodes.last().expect("chain always has at least one node");
        let Some(current) = actors.get(&current_id) else {
            return TemplateChain {
                nodes,
                end: ChainEnd::NoTemplate,
            };
        };
        let Some(target) = current.base_template_form_id else {
            return TemplateChain {
                nodes,
                end: ChainEnd::NoTemplate,
            };
        };
        if nodes.len() >= MAX_TEMPLATE_CHAIN_DEPTH {
            return TemplateChain {
                nodes,
                end: ChainEnd::TooDeep,
            };
        }
        if visited.contains(&target) {
            return TemplateChain {
                nodes,
                end: ChainEnd::Cycle {
                    target_form_id: target,
                },
            };
        }
        if let Some(list) = leveled.get(&target) {
            let mut candidates = list.entries.clone();
            candidates.sort_unstable();
            candidates.dedup();
            return TemplateChain {
                nodes,
                end: ChainEnd::Leveled {
                    target_form_id: target,
                    candidates,
                },
            };
        }
        if actors.contains_key(&target) {
            visited.insert(target);
            nodes.push(target);
            continue;
        }
        return TemplateChain {
            nodes,
            end: ChainEnd::Missing {
                target_form_id: target,
            },
        };
    }
}

/// Walks `chain` for one field group: starting at `chain.nodes[0]`, keeps
/// moving to the next concrete node as long as the current node's own
/// `ActorTemplateUsage` (via `uses_group`) says "my value for this group
/// comes from my template". Returns the FormID whose own group data should
/// be used, whether that FormID differs from the start (`inherited`), and a
/// stable diagnostic when the group asked to go further than the chain
/// could actually resolve.
fn resolve_group_source(
    chain: &TemplateChain,
    actors: &HashMap<u32, ActorRecordInput>,
    uses_group: impl Fn(&ActorTemplateUsage) -> bool,
) -> (u32, bool, Option<String>) {
    let mut index = 0usize;
    loop {
        let node_id = chain.nodes[index];
        let node = &actors[&node_id];
        if !uses_group(&node.template_usage) {
            return (node_id, index != 0, None);
        }
        if index + 1 < chain.nodes.len() {
            index += 1;
            continue;
        }
        let message = match &chain.end {
            ChainEnd::NoTemplate => Some("template flags set but no TPLT subrecord".to_string()),
            ChainEnd::Leveled {
                target_form_id,
                candidates,
            } => Some(format!(
                "template chain resolves to leveled list {target_form_id:08x} ({} candidate(s)); using own data",
                candidates.len()
            )),
            ChainEnd::Missing { target_form_id } => {
                Some(format!("unresolved template target {target_form_id:08x}"))
            }
            ChainEnd::Cycle { target_form_id } => {
                Some(format!("template cycle at {target_form_id:08x}"))
            }
            ChainEnd::TooDeep => Some("template chain exceeds maximum depth".to_string()),
        };
        return (node_id, index != 0, message);
    }
}

// ---------------------------------------------------------------------
// Catalog construction
// ---------------------------------------------------------------------

/// Resolves every actor placement in `inputs` into an `ActorBlueprint` (or a
/// diagnosed `Unresolved`/`Unsupported`/`Skipped` entry), in deterministic
/// `(base_form_id, reference_form_id)` order.
pub(crate) fn build_actor_catalog(
    inputs: &ActorCatalogInputs,
    source_fingerprint: &str,
) -> PreparedActorCatalog {
    let mut entries = Vec::with_capacity(inputs.placements.len());
    let mut counters = ActorCatalogCounters::default();
    for placement in &inputs.placements {
        let entry = match inputs.actors.get(&placement.base_form_id) {
            Some(actor) if actor.kind == placement.kind => {
                let blueprint = build_blueprint(actor, placement, inputs);
                counters.prepared += 1;
                if blueprint.inherited {
                    counters.inherited += 1;
                }
                ActorCatalogEntry::Prepared(Box::new(blueprint))
            }
            Some(actor) => {
                counters.unsupported += 1;
                ActorCatalogEntry::Unsupported {
                    base_form_id: placement.base_form_id,
                    reference_form_id: placement.reference_form_id,
                    reason: format!(
                        "reference {:08x} expects {:?} but base {:08x} is {:?}",
                        placement.reference_form_id,
                        placement.kind,
                        placement.base_form_id,
                        actor.kind
                    ),
                }
            }
            None if inputs.leveled.contains_key(&placement.base_form_id) => {
                let list = &inputs.leveled[&placement.base_form_id];
                let mut candidates = list
                    .entries
                    .iter()
                    .filter_map(|candidate| inputs.actors.get(candidate))
                    .filter(|actor| actor.kind == placement.kind)
                    .map(|actor| actor.form_id)
                    .collect::<Vec<_>>();
                candidates.sort_unstable();
                candidates.dedup();
                if candidates.is_empty() {
                    counters.skipped += 1;
                    ActorCatalogEntry::Skipped {
                        base_form_id: placement.base_form_id,
                        reference_form_id: placement.reference_form_id,
                        reason: format!(
                            "leveled list {:08x} has no concrete {:?} candidate",
                            placement.base_form_id, placement.kind
                        ),
                    }
                } else {
                    let selection_seed = stable_selection_seed(
                        source_fingerprint,
                        placement.reference_form_id,
                        placement.base_form_id,
                    );
                    let selected = candidates[(selection_seed as usize) % candidates.len()];
                    let actor = &inputs.actors[&selected];
                    let mut blueprint = build_blueprint(actor, placement, inputs);
                    blueprint.base_form_id = selected;
                    blueprint.source_base_form_id = placement.base_form_id;
                    blueprint.resolved_base_form_id = Some(selected);
                    blueprint.selection_seed = Some(selection_seed);
                    blueprint.is_leveled_template = true;
                    blueprint.template_candidates = candidates;
                    blueprint.diagnostics.push(format!(
                        "leveled list {:08x} selected candidate {:08x} with seed {selection_seed:016x}",
                        placement.base_form_id, selected
                    ));
                    blueprint.diagnostics.sort();
                    blueprint.diagnostics.dedup();
                    counters.prepared += 1;
                    ActorCatalogEntry::Prepared(Box::new(blueprint))
                }
            }
            None => {
                counters.unresolved += 1;
                ActorCatalogEntry::Unresolved {
                    base_form_id: placement.base_form_id,
                    reference_form_id: placement.reference_form_id,
                    reason: format!("missing base record {:08x}", placement.base_form_id),
                }
            }
        };
        entries.push(entry);
    }
    entries.sort_by_key(entry_sort_key);
    PreparedActorCatalog {
        revision: ACTOR_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        entries,
        counters,
    }
}

fn stable_selection_seed(
    source_fingerprint: &str,
    reference_form_id: u32,
    list_form_id: u32,
) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in format!("{source_fingerprint}:{reference_form_id:08x}:{list_form_id:08x}").bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn build_blueprint(
    actor: &ActorRecordInput,
    placement: &ActorPlacementInput,
    inputs: &ActorCatalogInputs,
) -> ActorBlueprint {
    let mut diagnostics = Vec::new();
    if actor.template_usage.unsupported_bits != 0 {
        diagnostics.push(format!(
            "unsupported template flag bits {:#06x}",
            actor.template_usage.unsupported_bits
        ));
    }

    let chain = if actor.template_usage.any() {
        build_chain(actor.form_id, &inputs.actors, &inputs.leveled)
    } else {
        TemplateChain::trivial(actor.form_id)
    };

    let (is_leveled_template, template_candidates) = match &chain.end {
        ChainEnd::Leveled { candidates, .. } => (true, candidates.clone()),
        _ => (false, Vec::new()),
    };

    let mut inherited = false;
    macro_rules! resolve_group {
        ($name:literal, $field:ident) => {{
            let (source, group_inherited, diag) =
                resolve_group_source(&chain, &inputs.actors, |usage| usage.$field);
            inherited |= group_inherited;
            if let Some(message) = diag {
                diagnostics.push(format!("{}: {}", $name, message));
            }
            source
        }};
    }

    let traits = inputs.actors[&resolve_group!("traits", traits)]
        .traits
        .clone();
    let stats = inputs.actors[&resolve_group!("stats", stats)].stats;
    let factions_raw = inputs.actors[&resolve_group!("factions", factions)]
        .factions
        .clone();
    let actor_effect_form_id =
        inputs.actors[&resolve_group!("actor_effect_list", actor_effect_list)].actor_effect_form_id;
    let ai_data = inputs.actors[&resolve_group!("ai_data", ai_data)].ai_data;
    let package_form_ids = inputs.actors[&resolve_group!("ai_packages", ai_packages)]
        .package_form_ids
        .clone();
    let model_animation = inputs.actors[&resolve_group!("model_animation", model_animation)]
        .model_animation
        .clone();
    let base_data = inputs.actors[&resolve_group!("base_data", base_data)]
        .base_data
        .clone();
    let inventory = inputs.actors[&resolve_group!("inventory", inventory)]
        .inventory
        .clone();
    let script_form_id = inputs.actors[&resolve_group!("script", script)].script_form_id;

    let factions = factions_raw
        .into_iter()
        .map(|membership| {
            let resolved = inputs.factions.get(&membership.faction_form_id);
            if resolved.is_none() {
                diagnostics.push(format!(
                    "unresolved faction link {:08x}",
                    membership.faction_form_id
                ));
            }
            let title = resolved.and_then(|faction| {
                faction
                    .ranks
                    .iter()
                    .find(|rank| rank.rank_number == i32::from(membership.rank))
                    .and_then(|rank| {
                        if traits.female {
                            rank.female_title
                                .clone()
                                .or_else(|| rank.male_title.clone())
                        } else {
                            rank.male_title
                                .clone()
                                .or_else(|| rank.female_title.clone())
                        }
                    })
            });
            ActorFactionMembership {
                faction_form_id: membership.faction_form_id,
                rank: membership.rank,
                title,
            }
        })
        .collect::<Vec<_>>();

    if let Some(race) = traits.race_form_id
        && !inputs.races.contains(&race)
    {
        diagnostics.push(format!("unresolved race link {race:08x}"));
    }
    if let Some(class) = actor.class_form_id
        && !inputs.classes.contains(&class)
    {
        diagnostics.push(format!("unresolved class link {class:08x}"));
    }
    if let Some(hair) = traits.hair_form_id
        && !inputs.known_bases.contains(&hair)
    {
        diagnostics.push(format!("unresolved hair link {hair:08x}"));
    }
    if let Some(eyes) = traits.eyes_form_id
        && !inputs.known_bases.contains(&eyes)
    {
        diagnostics.push(format!("unresolved eyes link {eyes:08x}"));
    }
    for head_part in &traits.head_part_form_ids {
        if !inputs.known_bases.contains(head_part) {
            diagnostics.push(format!("unresolved head part link {head_part:08x}"));
        }
    }
    for package in &package_form_ids {
        if !inputs.packages.contains(package) {
            diagnostics.push(format!("unresolved package link {package:08x}"));
        }
    }

    diagnostics.sort();
    diagnostics.dedup();

    ActorBlueprint {
        base_form_id: actor.form_id,
        source_base_form_id: placement.base_form_id,
        resolved_base_form_id: None,
        selection_seed: None,
        reference_form_id: placement.reference_form_id,
        record_kind: actor.kind.as_record_signature().to_string(),
        editor_id: actor.editor_id.clone(),
        display_name: base_data.name.clone(),
        model_path: model_animation.model_path.clone(),
        creature_model_list: model_animation.creature_model_list.clone(),
        animation_candidates: model_animation.creature_animation_files.clone(),
        race_form_id: traits.race_form_id,
        female: traits.female,
        height: traits.height,
        weight: traits.weight,
        hair_form_id: traits.hair_form_id,
        eyes_form_id: traits.eyes_form_id,
        head_part_form_ids: traits.head_part_form_ids.clone(),
        voice_form_id: traits.voice_form_id,
        level_or_mult: stats.level_or_mult,
        calc_min_level: stats.calc_min_level,
        calc_max_level: stats.calc_max_level,
        speed_multiplier: stats.speed_multiplier,
        karma: stats.karma,
        disposition_base: stats.disposition_base,
        fatigue: stats.fatigue,
        barter_gold: stats.barter_gold,
        health: stats.health,
        special: stats.special,
        npc_skill_values: stats.npc_skill_values,
        creature_type: stats.creature_type,
        combat_skill: stats.combat_skill,
        magic_skill: stats.magic_skill,
        stealth_skill: stats.stealth_skill,
        creature_damage: stats.creature_damage,
        factions,
        actor_effect_form_id,
        ai_data,
        package_form_ids,
        class_form_id: actor.class_form_id,
        combat_style_form_id: actor.combat_style_form_id,
        death_item_form_id: actor.death_item_form_id,
        script_form_id,
        inventory,
        is_leveled_template,
        template_candidates,
        base_template_form_id: actor.base_template_form_id,
        inherited,
        translation: placement.translation,
        rotation_xyzw: placement.rotation_xyzw,
        scale: placement.scale,
        initially_enabled: placement.initially_enabled,
        diagnostics,
    }
}

// ---------------------------------------------------------------------
// Artifact I/O
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActorCatalogArtifact {
    pub(crate) relative_path: String,
    pub(crate) hash: String,
    pub(crate) reused: bool,
}

/// Writes the deterministic actor-catalog artifact into the cell's own
/// scene directory (`scenes/<cell>/actors.ron`, next to `scene.ron`).
///
/// Unlike the item/recipe catalogs -- which are content-set-wide, identical
/// for every cell, and therefore keyed by the source fingerprint under
/// `catalogs/` -- the actor catalog embeds this cell's ACHR/ACRE placements,
/// so it must be stored per cell: a shared fingerprint-keyed file would be
/// overwritten by each prepared cell in turn, leaving every manifest
/// pointing at whichever cell was prepared last. The returned hash covers
/// exactly this cell's serialized catalog. A byte-identical existing file
/// is left untouched and reported as reused, mirroring
/// `write_recipe_catalog`.
pub(crate) fn write_actor_catalog(
    cache_dir: &Path,
    cell_form_id: u32,
    catalog: &PreparedActorCatalog,
) -> Result<ActorCatalogArtifact> {
    let relative = PathBuf::from("scenes")
        .join(format!("{cell_form_id:08x}"))
        .join("actors.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let serialized = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default())
        .map_err(|error| anyhow::anyhow!("failed to serialize actor catalog: {error}"))?;
    let hash = fingerprint(serialized.as_bytes());
    let reused = std::fs::read(&path)
        .map(|existing| existing == serialized.as_bytes())
        .unwrap_or(false);
    if !reused {
        std::fs::write(&path, serialized)?;
    }
    Ok(ActorCatalogArtifact {
        relative_path: relative.to_string_lossy().replace('\\', "/"),
        hash,
        reused,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn npc(form_id: u32) -> ActorRecordInput {
        ActorRecordInput {
            form_id,
            kind: ActorRecordKind::Npc,
            ..ActorRecordInput::default()
        }
    }

    fn placement(
        reference_form_id: u32,
        base_form_id: u32,
        kind: ActorRecordKind,
    ) -> ActorPlacementInput {
        ActorPlacementInput {
            reference_form_id,
            base_form_id,
            kind,
            ..ActorPlacementInput::default()
        }
    }

    #[test]
    fn revision_is_pinned() {
        assert_eq!(
            ACTOR_CATALOG_REVISION,
            "openmw-actors-v2-appearance-seeded-levels"
        );
    }

    #[test]
    fn inherits_a_flagged_group_from_its_template() {
        let mut template = npc(0x10);
        template.traits.race_form_id = Some(0xAA);
        let mut source = npc(0x20);
        source.base_template_form_id = Some(0x10);
        source.template_usage.traits = true;
        source.traits.race_form_id = Some(0xBB); // must be ignored once inherited

        let mut actors = HashMap::new();
        actors.insert(0x10, template);
        actors.insert(0x20, source);
        let inputs = ActorCatalogInputs {
            actors,
            placements: vec![placement(1, 0x20, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert_eq!(blueprint.race_form_id, Some(0xAA));
        assert!(blueprint.inherited);
        assert_eq!(catalog.counters.inherited, 1);
        assert_eq!(catalog.counters.prepared, 1);
    }

    #[test]
    fn does_not_inherit_an_unflagged_group() {
        let mut template = npc(0x10);
        template.traits.race_form_id = Some(0xAA);
        let mut source = npc(0x20);
        source.base_template_form_id = Some(0x10);
        source.template_usage.stats = true; // traits NOT flagged
        source.traits.race_form_id = Some(0xBB);

        let mut actors = HashMap::new();
        actors.insert(0x10, template);
        actors.insert(0x20, source);
        let inputs = ActorCatalogInputs {
            actors,
            placements: vec![placement(1, 0x20, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert_eq!(blueprint.race_form_id, Some(0xBB));
    }

    #[test]
    fn diagnoses_a_template_cycle() {
        let mut a = npc(0x10);
        a.base_template_form_id = Some(0x20);
        a.template_usage.traits = true;
        let mut b = npc(0x20);
        b.base_template_form_id = Some(0x10);
        b.template_usage.traits = true;

        let mut actors = HashMap::new();
        actors.insert(0x10, a);
        actors.insert(0x20, b);
        let inputs = ActorCatalogInputs {
            actors,
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert!(
            blueprint
                .diagnostics
                .iter()
                .any(|message| message.contains("template cycle"))
        );
    }

    #[test]
    fn diagnoses_a_missing_template_target() {
        let mut a = npc(0x10);
        a.base_template_form_id = Some(0xDEAD);
        a.template_usage.stats = true;

        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x10, a)]),
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert!(
            blueprint
                .diagnostics
                .iter()
                .any(|message| message.contains("unresolved template target 0000dead"))
        );
    }

    #[test]
    fn records_leveled_template_candidates_deterministically() {
        let mut a = npc(0x10);
        a.base_template_form_id = Some(0x99);
        a.template_usage.traits = true;

        let leveled = LeveledInput {
            form_id: 0x99,
            entries: vec![0x30, 0x10, 0x20, 0x10],
        };

        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x10, a)]),
            leveled: HashMap::from([(0x99, leveled)]),
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert!(blueprint.is_leveled_template);
        assert_eq!(blueprint.template_candidates, vec![0x10, 0x20, 0x30]);
    }

    #[test]
    fn an_unused_dangling_template_produces_no_leveled_marker() {
        let mut a = npc(0x10);
        a.base_template_form_id = Some(0xDEAD); // dangling, but no flags use it

        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x10, a)]),
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert!(!blueprint.is_leveled_template);
        assert!(blueprint.diagnostics.is_empty());
    }

    #[test]
    fn unresolved_race_class_faction_package_links_are_diagnosed() {
        let mut a = npc(0x10);
        a.traits.race_form_id = Some(0x01);
        a.class_form_id = Some(0x02);
        a.factions.push(ActorFactionInput {
            faction_form_id: 0x03,
            rank: 0,
        });
        a.package_form_ids.push(0x04);

        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x10, a)]),
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        for expected in [
            "unresolved race link 00000001",
            "unresolved class link 00000002",
            "unresolved faction link 00000003",
            "unresolved package link 00000004",
        ] {
            assert!(
                blueprint.diagnostics.iter().any(|m| m == expected),
                "missing {expected:?} in {:?}",
                blueprint.diagnostics
            );
        }
    }

    #[test]
    fn faction_rank_titles_are_resolved_by_sex() {
        let mut a = npc(0x10);
        a.traits.female = true;
        a.factions.push(ActorFactionInput {
            faction_form_id: 0x50,
            rank: 2,
        });
        let faction = FactionInput {
            form_id: 0x50,
            ranks: vec![FactionRankInput {
                rank_number: 2,
                male_title: Some("Paladin".into()),
                female_title: Some("Paladine".into()),
            }],
        };

        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x10, a)]),
            factions: HashMap::from([(0x50, faction)]),
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };

        let catalog = build_actor_catalog(&inputs, "fp");
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        assert_eq!(blueprint.factions[0].title.as_deref(), Some("Paladine"));
    }

    #[test]
    fn missing_base_record_is_unresolved_not_silent() {
        let inputs = ActorCatalogInputs {
            placements: vec![placement(1, 0xFFEE, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };
        let catalog = build_actor_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unresolved, 1);
        assert!(matches!(
            &catalog.entries[0],
            ActorCatalogEntry::Unresolved { .. }
        ));
    }

    #[test]
    fn direct_leveled_base_is_skipped_not_silently_defaulted() {
        let leveled = LeveledInput {
            form_id: 0x77,
            entries: vec![0x10, 0x20],
        };
        let inputs = ActorCatalogInputs {
            leveled: HashMap::from([(0x77, leveled)]),
            placements: vec![placement(1, 0x77, ActorRecordKind::Creature)],
            ..ActorCatalogInputs::default()
        };
        let catalog = build_actor_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.skipped, 1);
        assert!(matches!(
            &catalog.entries[0],
            ActorCatalogEntry::Skipped { .. }
        ));
    }

    #[test]
    fn kind_mismatch_is_unsupported() {
        let creature = ActorRecordInput {
            form_id: 0x10,
            kind: ActorRecordKind::Creature,
            ..ActorRecordInput::default()
        };
        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x10, creature)]),
            // ACHR (Npc) pointing at a CREA base.
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };
        let catalog = build_actor_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unsupported, 1);
        assert!(matches!(
            &catalog.entries[0],
            ActorCatalogEntry::Unsupported { .. }
        ));
    }

    #[test]
    fn entries_are_sorted_by_base_then_reference_form_id() {
        let mut actors = HashMap::new();
        actors.insert(0x20, npc(0x20));
        actors.insert(0x10, npc(0x10));
        let inputs = ActorCatalogInputs {
            actors,
            placements: vec![
                placement(2, 0x20, ActorRecordKind::Npc),
                placement(1, 0x10, ActorRecordKind::Npc),
                placement(1, 0x20, ActorRecordKind::Npc),
            ],
            ..ActorCatalogInputs::default()
        };
        let catalog = build_actor_catalog(&inputs, "fp");
        let keys: Vec<(u32, u32)> = catalog.entries.iter().map(entry_sort_key).collect();
        assert_eq!(keys, vec![(0x10, 1), (0x20, 1), (0x20, 2)]);
    }

    #[test]
    fn serialization_is_deterministic_across_runs() {
        let mut actors = HashMap::new();
        actors.insert(0x20, npc(0x20));
        actors.insert(0x10, npc(0x10));
        let inputs = ActorCatalogInputs {
            actors,
            placements: vec![
                placement(2, 0x20, ActorRecordKind::Npc),
                placement(1, 0x10, ActorRecordKind::Npc),
            ],
            ..ActorCatalogInputs::default()
        };
        let a = build_actor_catalog(&inputs, "fp");
        let b = build_actor_catalog(&inputs, "fp");
        let ron_a = ron::ser::to_string_pretty(&a, ron::ser::PrettyConfig::default()).unwrap();
        let ron_b = ron::ser::to_string_pretty(&b, ron::ser::PrettyConfig::default()).unwrap();
        assert_eq!(ron_a, ron_b);
    }

    /// Regression test for the M4 wave 1 acceptance bug: the actor catalog
    /// was originally keyed by the content-set source fingerprint like the
    /// item/recipe catalogs, so preparing several cells from the same
    /// content set overwrote one shared `catalogs/<fp>/actors.ron` and every
    /// manifest ended up pointing at the last-prepared cell's actors. Two
    /// different cells from the same content set must get distinct per-cell
    /// paths, distinct hashes for distinct contents, and both files must
    /// survive preparing the other cell.
    #[test]
    fn two_cells_from_the_same_content_set_get_distinct_catalog_artifacts() {
        let cache_dir = std::env::temp_dir().join(format!(
            "bevyout-actor-catalog-per-cell-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
        ));

        // Same content set (same fingerprint), different cells: different
        // ACHR placements reference different bases.
        let mut actors = HashMap::new();
        actors.insert(0x10, npc(0x10));
        actors.insert(0x20, npc(0x20));
        let cell_a_inputs = ActorCatalogInputs {
            actors: actors.clone(),
            placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };
        let cell_b_inputs = ActorCatalogInputs {
            actors,
            placements: vec![placement(2, 0x20, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };
        let catalog_a = build_actor_catalog(&cell_a_inputs, "shared-fp");
        let catalog_b = build_actor_catalog(&cell_b_inputs, "shared-fp");

        let artifact_a = write_actor_catalog(&cache_dir, 0x0000_3a35, &catalog_a).unwrap();
        let artifact_b = write_actor_catalog(&cache_dir, 0x0002_4d6b, &catalog_b).unwrap();

        assert_ne!(artifact_a.relative_path, artifact_b.relative_path);
        assert_eq!(artifact_a.relative_path, "scenes/00003a35/actors.ron");
        assert_eq!(artifact_b.relative_path, "scenes/00024d6b/actors.ron");
        assert_ne!(artifact_a.hash, artifact_b.hash);

        // Writing cell B must not clobber cell A's artifact: A's file still
        // exists, still matches A's recorded hash, and still round-trips to
        // A's own catalog (not B's).
        let bytes_a = std::fs::read(cache_dir.join(&artifact_a.relative_path)).unwrap();
        assert_eq!(fingerprint(&bytes_a), artifact_a.hash);
        let decoded_a: PreparedActorCatalog =
            ron::de::from_str(std::str::from_utf8(&bytes_a).unwrap()).unwrap();
        assert_eq!(decoded_a, catalog_a);

        let bytes_b = std::fs::read(cache_dir.join(&artifact_b.relative_path)).unwrap();
        assert_eq!(fingerprint(&bytes_b), artifact_b.hash);

        // Re-preparing the same cell with identical content reuses the file.
        let artifact_a_again = write_actor_catalog(&cache_dir, 0x0000_3a35, &catalog_a).unwrap();
        assert!(artifact_a_again.reused);
        assert_eq!(artifact_a_again.hash, artifact_a.hash);

        std::fs::remove_dir_all(&cache_dir).unwrap();
    }
}
