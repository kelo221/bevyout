//! Executable-spec seed (WP6): runs the three `features/*.feature` files
//! against real `bevyout` code via the `cucumber` crate.
//!
//! `src/lib.rs` keeps `vsa` private, so this integration test cannot
//! `use bevyout::vsa::...`. Rather than widen the library's public surface,
//! the modules under test are pulled in verbatim with `#[path]` so the exact
//! same source compiles into this test binary. Concretely:
//!
//! - `vsa::manifest` is self-contained (only depends on `bevy`/`serde`, both
//!   already real dependencies), so it is included as-is.
//! - `vsa::paths`, `vsa::bsa`, and `vsa::assets` form a small dependency
//!   chain (`assets` -> `bsa`/`manifest`/`paths`, `paths` -> `plugin`) that
//!   is also included as-is *except* for `vsa::plugin`, which in the real
//!   crate is a facade re-exporting the ~2200-line attributed ESM4 reader
//!   (`vsa::openmw_esm4`). Pulling that whole parser in just so `paths.rs`
//!   type-checks a `&ReferenceRecord` parameter on a function this suite
//!   never calls (`placement_transform`) would be all cost, no signal. So
//!   `plugin` below is a small stand-in exposing only the `ReferenceRecord`
//!   fields `paths.rs` itself touches (its own `#[cfg(test)]` module
//!   constructs one). This is scoped narrowly to unblock compilation, not to
//!   re-implement plugin/ESM4 behaviour -- none of that behaviour is
//!   exercised here.
/// Stand-in for `vsa::plugin`'s re-export of `openmw_esm4::ReferenceRecord`,
/// scoped to the handful of fields `paths.rs` reads. See the module-level
/// comment above for why.
mod plugin {
    #[derive(Debug, Clone, Default)]
    #[allow(dead_code)]
    pub(crate) struct ReferenceRecord {
        pub(crate) form_id: u32,
        pub(crate) base_form_id: u32,
        pub(crate) position: [f32; 3],
        pub(crate) rotation: [f32; 3],
        pub(crate) scale: f32,
        pub(crate) flags: u32,
    }
}

// M3/#95 canonical item instances and atomic holder transactions live in the
// normal engine-independent crate boundary shared by runtime and tests.
#[allow(dead_code, unused_imports)]
mod item_transaction {
    pub use bevyout_core::item_transaction::*;
}

// M4/#110 immutable actor definitions and mutable instance state live in the
// same pure core boundary as canonical item transactions.
#[allow(dead_code, unused_imports)]
mod actor_state {
    pub use bevyout_core::actor_state::*;
}

// M9/#309 pure SPECIAL, skill, derived-stat, and leveling kernels live in
// the same engine-independent core boundary.
#[allow(dead_code, unused_imports)]
mod stats {
    pub use bevyout_core::stats::*;
}
// M9 wave 2/#313 pure perk eligibility and active-modifier kernels live in
// the same engine-independent core boundary.
#[allow(dead_code, unused_imports)]
mod perks {
    pub use bevyout_core::perks::*;
}
// M9 wave 3/#317 pure active-effect, radiation, and addiction kernels live
// in the same engine-independent core boundary.
#[allow(dead_code, unused_imports)]
mod effects {
    pub use bevyout_core::effects::*;
}
#[allow(dead_code, unused_imports)]
mod radiation {
    pub use bevyout_core::radiation::*;
}
#[allow(dead_code, unused_imports)]
mod chems {
    pub use bevyout_core::chems::*;
}
// M6 wave 3's actor-residency policy is pure and dependency-light. Include
// the production seam verbatim so the executable feature coverage exercises
// the same canonical identity and handoff decisions as the viewer.
#[path = "../src/viewer/actor_residency.rs"]
#[allow(unused_imports)]
mod actor_residency;
#[allow(dead_code, unused_imports)]
mod pause_menu {
    pub use bevyout_core::pause_menu::*;
}
#[allow(dead_code, unused_imports)]
mod time_of_day {
    pub use bevyout_core::time_of_day::*;
}

#[path = "../src/vsa/prepare/reflection_probe_distribution.rs"]
mod reflection_probe_distribution;

#[path = "../src/vsa/overlay_policy.rs"]
mod overlay_policy;

#[path = "../src/vsa/cache_stats/policy.rs"]
#[allow(dead_code, unused_imports)]
mod cache_stats_policy;

#[path = "../src/vsa/cache_store/policy.rs"]
#[allow(dead_code, unused_imports)]
mod cache_store_policy;

#[path = "../src/vsa/prepare/exterior_scene_policy.rs"]
#[allow(dead_code, unused_imports)]
mod exterior_scene_policy;

// These files are pulled in verbatim and cover far more ground than the three
// pure seams this suite drives (placement math, cell selectors, manifest
// (de)serialization, conversion-profile selection). Everything else in them
// -- BSA archive I/O, Blender job orchestration, GLB validation, and so on --
// is legitimately unused from here, so allow dead_code per included module
// rather than mask it crate-wide.
#[path = "../src/vsa/paths.rs"]
#[allow(dead_code, unused_imports)]
mod paths;

#[allow(dead_code, unused_imports)]
mod manifest {
    pub use bevyout_core::manifest::*;
}

#[path = "../src/vsa/bsa.rs"]
#[allow(dead_code, unused_imports)]
mod bsa;

// `manifest` and `assets` both lean on the physics sidecar module since the
// native-Havok refactor, so it rides along on the same verbatim-include basis.
#[path = "../src/vsa/physics.rs"]
#[allow(dead_code, unused_imports)]
mod physics;

#[path = "../src/vsa/assets/mod.rs"]
#[allow(dead_code, unused_imports)]
mod assets;

// `cell_map` (issue #45) is deliberately dependency-free (serde + std only,
// no Bevy, no openmw_esm4 types in its public surface -- see its module
// doc comment), so unlike the modules above it needs no stand-ins to
// include verbatim.
#[path = "../src/vsa/cell_map.rs"]
#[allow(dead_code, unused_imports)]
mod cell_map;

// M7/#252's generic winning-record collector is std-only. The ESM4 byte
// walker feeds it in production; this suite drives the exact override,
// provenance, and deletion policy without pulling the parser into the
// integration-test crate.
#[path = "../src/vsa/record_stream/winners.rs"]
#[allow(dead_code, unused_imports)]
mod record_winners;

// M7/#253's structural script-record decoder is std/core-only. The production
// catalog adapts shared ESM4 subrecords into the same input contract.
#[path = "../src/vsa/scripts/record.rs"]
#[allow(dead_code, unused_imports)]
mod record;

#[path = "../src/vsa/scripts/attachments.rs"]
#[allow(dead_code, unused_imports)]
mod attachments;

// Standalone Yarn dialogue preparation is dependency-light and is included
// verbatim so the executable specification drives the same parser/catalog
// code as the production prepare path.
#[path = "../src/vsa/dialogue/mod.rs"]
#[allow(dead_code, unused_imports)]
mod dialogue;

#[path = "../src/viewer/dialogue/host.rs"]
#[allow(dead_code, unused_imports)]
mod dialogue_host;

#[path = "../src/vsa/report/schema.rs"]
#[allow(dead_code, unused_imports)]
mod report_schema;

// `world::policy` (issue #51) is likewise dependency-free (std only, no
// Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/world/policy.rs"]
#[allow(dead_code, unused_imports)]
mod policy;

// M6 wave 2's exterior lifecycle and diagnostics seams are included here so
// the executable specification drives the same generation-aware planner and
// process-memory report used by the viewer. The diagnostics adapter expects
// its sibling lifecycle module at the crate root, matching the production
// `viewer::world::exterior` layout.
#[path = "../src/viewer/world/exterior/lifecycle.rs"]
#[allow(dead_code, unused_imports)]
mod lifecycle;

#[path = "../src/viewer/world/exterior/diagnostics.rs"]
#[allow(dead_code, unused_imports)]
mod exterior_diagnostics;

// `world::swap_policy` (issue #52) is likewise dependency-free (std only, no
// Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/world/swap_policy.rs"]
#[allow(dead_code, unused_imports)]
mod swap_policy;

// `world::reveal_policy` (issue #55) is likewise dependency-free (std only,
// no Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/world/reveal_policy.rs"]
#[allow(dead_code, unused_imports)]
mod reveal_policy;

// `animation::policy` (issue #57) is likewise dependency-free (std only, no
// Bevy) -- see its module doc comment -- so it is included verbatim too.
#[path = "../src/viewer/animation/policy.rs"]
#[allow(dead_code, unused_imports)]
mod animation_policy;

// `viewer::actor_animation::policy` (#106) is the Bevy-free gameplay clip
// resolver/state machine consumed by the actor-animation runtime plugin.
#[path = "../src/viewer/actor_animation/policy.rs"]
#[allow(dead_code, unused_imports)]
mod actor_animation_policy;

// Lane D idle authority/selection is pure std/serde-only and is included
// verbatim so the executable feature suite drives the runtime policy seam.
#[path = "../src/viewer/actor_animation/idle_policy.rs"]
#[allow(dead_code, unused_imports)]
mod actor_idle_policy;

#[path = "../src/vsa/bake/policy.rs"]
#[allow(dead_code, unused_imports)]
mod bake_policy;

// `world::persist_policy` (issues #60/#61) is likewise dependency-free (std
// only, no Bevy) -- see its module doc comment -- so it is included verbatim
// too.
#[path = "../src/viewer/world/persist_policy.rs"]
#[allow(dead_code, unused_imports)]
mod persist_policy;

// `world::ownership_policy` (issue #63) is dependency-free too (std only,
// generic over the collider id types), included verbatim the same way.
#[path = "../src/viewer/world/ownership_policy.rs"]
#[allow(dead_code, unused_imports)]
mod ownership_policy;

// M3 wave 1 inventory policy is intentionally std-only so the executable
// feature suite can drive the same stack and quantity decisions as Bevy.
#[path = "../src/viewer/inventory.rs"]
#[allow(dead_code, unused_imports)]
mod inventory_policy;

// `viewer::performance_policy` is the std-only frame selection/statistics
// seam consumed by the BRP performance probe.
#[path = "../src/viewer/performance_policy.rs"]
#[allow(dead_code, unused_imports)]
mod performance_policy;

// `viewer::screen_fx::policy` is the Bevy-free ImageSpace/IMAD lifecycle and
// composition boundary.  The renderer adapter is intentionally not included
// in this executable-spec crate.
#[path = "../src/viewer/screen_fx/policy.rs"]
#[allow(dead_code, unused_imports)]
mod screen_fx_policy;

// Hybrid point-shadow composition is intentionally Bevy-free so the
// executable specification drives the same source-selection policy as the
// runtime shader contract.
#[path = "../src/viewer/hybrid_shadow_policy.rs"]
#[allow(dead_code, unused_imports)]
mod hybrid_shadow_policy;

// The realtime-shadow write gate (issue #267, PERF wave 1) is Bevy-free for
// the same reason: the executable spec drives the same disabled-path policy
// as the runtime system.
#[path = "../src/viewer/realtime_shadow_policy.rs"]
#[allow(dead_code, unused_imports)]
mod realtime_shadow_policy;

// The AO eligibility tracker and glow-card naming policy (issue #270, PERF
// wave 1) are Bevy-free for the same reason as `realtime_shadow_policy`:
// the executable spec drives the same classification policy the runtime
// adapter in `viewer::controls`/`viewer::scene` consumes.
#[path = "../src/viewer/ao_policy.rs"]
#[allow(dead_code, unused_imports)]
mod ao_policy;

#[path = "../src/viewer/glow_card_policy.rs"]
#[allow(dead_code, unused_imports)]
mod glow_card_policy;

// The material-clamp policy (issue #269, PERF wave 1) is Bevy-free for the
// same reason as `realtime_shadow_policy`: the executable spec drives the
// same snapshot/clamp/restore decisions the runtime system applies.
#[path = "../src/viewer/material_clamp_policy.rs"]
#[allow(dead_code, unused_imports)]
mod material_clamp_policy;

// `interaction::container_policy` (issue #75) is dependency-free too (std
// only, no Bevy) -- see its module doc comment -- so it is included
// verbatim here too.
#[path = "../src/viewer/interaction/container_policy.rs"]
#[allow(dead_code, unused_imports)]
mod container_policy;

// `viewer::nav::landmass_graph`/`viewer::nav::door_link` (issue #112, M4
// wave 3) are both dependency-free of `vsa`/Bevy (only `bevy_landmass`/
// `glam`, real non-dev dependencies of the main crate, so already linked
// into this test binary too) -- see `landmass_graph.rs`'s module doc comment
// for why it cannot reuse `vsa::prepare::nav_graph`'s types directly here.
// Flat top-level includes: neither file has a relative `super::` import to
// line up against another `mod` in this tree.
#[path = "../src/viewer/nav/landmass_graph.rs"]
#[allow(dead_code, unused_imports)]
mod landmass_graph;

#[path = "../src/viewer/nav/doors/fsm.rs"]
#[allow(dead_code, unused_imports)]
mod door_link;

// M6 W3-C's exterior actor-residency sequencing is std-only for exactly this
// reason: the runtime ordering (unload before eviction, handoff on a border
// crossing, restore at a new generation) is executable here, while the Bevy
// adapter that applies it is unit tested against a bare `World`.
#[path = "../src/viewer/world/exterior/actor_residency_plan.rs"]
#[allow(dead_code, unused_imports)]
mod exterior_actor_plan;

// `viewer::nav::repath` (issue #113, M4 wave 4) is std-only, same flat
// top-level include rationale as `door_link` above.
#[path = "../src/viewer/nav/repath.rs"]
#[allow(dead_code, unused_imports)]
mod repath;

// `viewer::nav::ledger_policy` (issue #134, M4 wave 4) is std-only, same
// flat top-level include rationale as `door_link`/`repath` above.
#[path = "../src/viewer/nav/ledger_policy.rs"]
#[allow(dead_code, unused_imports)]
mod ledger_policy;

// `viewer::nav::movement_policy` (issue #114, M4 wave 5) is std-only, same
// flat top-level include rationale as `door_link`/`repath`/`ledger_policy`
// above.
#[path = "../src/viewer/nav/movement_policy.rs"]
#[allow(dead_code, unused_imports)]
mod movement_policy;

// `viewer::nav::locomotion` (issue #188) is std-only, same flat top-level
// include rationale as `movement_policy`/`fall_guard` above.
#[path = "../src/viewer/nav/agent/locomotion.rs"]
#[allow(dead_code, unused_imports)]
mod locomotion;

// `viewer::nav::openmw_doors` (issue #185) is std-only, same flat top-level
// include rationale as `door_link`/`repath`/`movement_policy` above.
#[path = "../src/viewer/nav/openmw_doors/mod.rs"]
#[allow(dead_code, unused_imports)]
mod openmw_doors;

#[path = "../src/converter_policy.rs"]
#[allow(dead_code, unused_imports)]
mod converter_policy;

#[path = "../src/vsa/bake/gltf_extension_policy.rs"]
#[allow(dead_code, unused_imports)]
mod gltf_extension_policy;

// `vsa::prepare::selectors` reuses the selector grammar from `vsa::paths`
// via a relative `super::super::paths` import, and `vsa::prepare::batch_cache`
// (issue #47) similarly reuses `vsa::cell_map` via a relative
// `super::super::cell_map` import, so both are nested one module deep here to
// make those paths land on the `mod paths`/`mod cell_map` includes above.
#[path = "."]
mod prepare {
    #[path = "../src/vsa/prepare/selectors.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod selectors;

    #[path = "../src/vsa/prepare/batch_cache.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod batch_cache;

    // `vsa::prepare::jobs` (issue #48) has no relative `super::super::`
    // imports of its own -- it is std/serde/ron only -- but lives here too
    // for consistency with its sibling pure `prepare` seams above.
    #[path = "../src/vsa/prepare/jobs.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod jobs;

    // `vsa::prepare::fingerprints` (issue #49) reuses `vsa::assets` (for
    // `NIF_CONVERTER_REVISION`) via a relative `super::super::assets`
    // import, so it is nested here too -- same pattern as `selectors`/
    // `batch_cache` above -- to land that path on the `mod assets` include
    // near the top of this file. `jobs` depends on it (`use
    // super::fingerprints::...`), so both must live in this same block.
    #[path = "../src/vsa/prepare/fingerprints.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod fingerprints;

    #[path = "../src/vsa/prepare/native_policy.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod native_policy;

    #[path = "../src/vsa/prepare/container_audio_policy.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod container_audio_policy;

    // `vsa::prepare::actor_catalog` (issue #103, M4 wave 1 task C) reuses
    // `vsa::manifest::PreparedInventoryEntry` (for the reused item-catalog
    // inventory-entry contract) and `vsa::paths::fingerprint` via relative
    // `super::super::...` imports, so it is nested here too -- same pattern
    // as `selectors`/`batch_cache`/`fingerprints` above -- to land those
    // paths on the `mod manifest`/`mod paths` includes near the top of this
    // file.
    #[path = "../src/vsa/prepare/actor_catalog.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod actor_catalog;

    // `vsa::prepare::package_catalog` (issue #175, M4 wave 11 lane C) reuses
    // `vsa::paths::fingerprint` via a relative `super::super::paths` import,
    // so it is nested here too -- same pattern as `actor_catalog` above --
    // to land that path on the `mod paths` include near the top of this
    // file.
    #[path = "../src/vsa/prepare/package_catalog.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod package_catalog;

    #[path = "../src/vsa/prepare/actor_appearance.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod actor_appearance;

    #[path = "../src/vsa/prepare/actor_animation_cache.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod actor_animation_cache;

    // `vsa::prepare::nav_graph` (issue #111, M4 wave 2) reuses
    // `vsa::paths::{FO3_SCALE, fingerprint}` via relative `super::super::`
    // imports, so it is nested here too -- same pattern as `actor_catalog`
    // above -- to land those paths on the `mod paths` include near the top
    // of this file.
    #[path = "../src/vsa/prepare/nav_graph.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod nav_graph;
}
use prepare::actor_animation_cache;
use prepare::actor_appearance;
use prepare::actor_catalog;
use prepare::batch_cache;
use prepare::container_audio_policy;
use prepare::fingerprints;
use prepare::jobs;
use prepare::native_policy;
use prepare::nav_graph;
use prepare::package_catalog;
use prepare::selectors;

// `vsa::bake::plan` (issue #62) reuses the resumable job-manifest machinery
// from `vsa::prepare::jobs` (#48) through relative
// `super::super::prepare::...` imports (in the real crate those names are
// re-exported at `prepare`'s module level by `prepare/mod.rs`'s
// `pub(crate) use jobs::*`), plus `vsa::manifest` for `PreparedBake`. It is
// nested two modules deep here, with small stand-in `manifest`/`prepare`
// re-export modules alongside it, so both relative paths land on the
// verbatim includes above rather than needing `prepare/mod.rs` itself.
#[path = "."]
mod vsa_bake {
    pub mod manifest {
        pub(crate) use crate::manifest::PreparedBake;
    }
    pub mod prepare {
        pub(crate) use crate::prepare::jobs::*;
    }
    #[path = "."]
    pub mod bake {
        #[path = "../src/vsa/bake/plan.rs"]
        #[allow(dead_code, unused_imports)]
        pub mod plan;
    }
}
use vsa_bake::bake::plan as bake_plan;

// The attributed ESM4 reader (`vsa::openmw_esm4`, issue #111's NAVM/NAVI
// decode seam) is anyhow/flate2/std-only apart from
// `super::manifest::{CellInfo, ImageSpaceInfo}` and
// `super::paths::CellSelector`, both already included verbatim above. It is
// nested one module deep behind stand-in re-export modules -- the same trick
// `vsa_bake` uses -- so nav_graph.feature can drive the real byte-level
// NAVM/NAVI subrecord decode with synthetic in-memory plugin byte streams.
#[path = "."]
mod vsa_esm {
    pub mod manifest {
        pub(crate) use crate::manifest::{CellInfo, ImageSpaceInfo};
    }
    pub mod paths {
        pub(crate) use crate::paths::CellSelector;
    }
    #[path = "../src/vsa/openmw_esm4/mod.rs"]
    #[allow(dead_code, unused_imports)]
    pub mod openmw_esm4;
}
use vsa_esm::openmw_esm4;

// `viewer::interaction::leveled` (issue #74) is dependency-free (std only,
// no Bevy, no `vsa::manifest` import -- see its module doc comment for why
// it mirrors `PreparedLeveledList`/`PreparedLeveledEntry` with local plain
// types), so unlike the modules above it needs no nesting or stand-ins to
// include verbatim.
#[path = "../src/viewer/interaction/leveled.rs"]
mod leveled;

// Item rules share the same normal core-crate boundary as item transactions.
#[allow(dead_code, unused_imports)]
mod item_rules {
    pub use bevyout_core::items::*;
}

// Drop placement is a Bevy-free candidate policy, so the runtime and the
// executable spec share the same retreat/fallback decision logic.
#[path = "../src/viewer/world_items/drop_policy.rs"]
#[allow(dead_code, unused_imports)]
mod drop_policy;

// Console transcript/history policy is std-only so the executable feature
// suite drives the same bounded clear semantics as the Bevy frontend.
#[path = "../src/console/openmw_ui/mod.rs"]
#[allow(dead_code, unused_imports)]
mod console_openmw_ui;
// `viewer::interaction::item_use` (issue #99) is dependency-free (std only,
// no Bevy) like `item_rules`, so it is included verbatim too.
#[path = "../src/viewer/interaction/item_use.rs"]
#[allow(dead_code, unused_imports)]
mod item_use;
// `viewer::player::equipment` (issue #98) reuses `viewer::inventory::StackKey`
// via a relative `super::super::inventory::StackKey` import (the same
// nesting-depth trick `vsa::prepare::selectors` uses for `vsa::paths`), so it
// is nested two modules deep here behind a `viewer_player` stand-in that
// aliases the existing `inventory_policy` include as `inventory` -- mirroring
// how `vsa_bake` above aliases `manifest`/`prepare` for `bake::plan`.
#[path = "."]
mod viewer_player {
    pub mod inventory {
        pub(crate) use crate::inventory_policy::*;
    }
    #[path = "."]
    pub mod player {
        #[path = "../src/viewer/player/equipment.rs"]
        #[allow(dead_code, unused_imports)]
        pub mod equipment;
    }
}
use viewer_player::player::equipment;
// The serde/std-only recipe validation seam keeps this executable-spec
// fixture independent of Bevy and game data. Parser/catalog integration is
// covered by the recipe unit tests.
#[path = "../src/vsa/recipe.rs"]
#[allow(dead_code, unused_imports)]
mod recipe_policy;

// Pure AI package runtime layer (issues #193/#194/#195). Each is std/serde-only
// (lifecycle also uses the Bevy-free `bevyout_core::actor_state` checkpoint) so
// all three compile verbatim here via `#[path]`.
#[path = "../src/viewer/ai/lifecycle.rs"]
#[allow(unused_imports)]
mod ai_lifecycle;
#[path = "../src/viewer/ai/resolution.rs"]
#[allow(dead_code, unused_imports)]
mod ai_resolution;
#[path = "../src/viewer/ai/selection.rs"]
#[allow(dead_code, unused_imports)]
mod ai_selection;
// Pure package-family dispatch (issues #196/#197): std-only, no Bevy, so it
// compiles verbatim here via `#[path]` like the other AI package modules.
#[path = "../src/viewer/ai/families.rs"]
#[allow(dead_code, unused_imports)]
mod ai_families;
// The autonomous package driver's pure eligibility gate (issue #218):
// std-only, no Bevy, so the same "an alive actor is selected for auto-bind"
// rule the Bevy system consults is what this cucumber scenario exercises.
#[path = "../src/viewer/ai/autonomous_gate.rs"]
#[allow(dead_code, unused_imports)]
mod autonomous_gate;

use assets::AssetConversion;
use bevyout_core::actor;
use bevyout_core::actor_animation;
use bevyout_core::combat::{
    BASIS_POINT_DENOMINATOR, CombatRngDomain, CombatRngState, MAX_JAM_CHANCE_BASIS_POINTS,
    WeaponConditionPolicy,
};
use bevyout_core::disposition;
use bevyout_core::facegen;
use bevyout_core::faction;
use bevyout_core::perception;
use bevyout_core::weapon;
use cucumber::{World as _, given, then, when};
use item_transaction::{
    CombatTransactionOutcome, CombatTransactionReceipt, HolderId, ItemHolderState, ItemInstance,
    ItemInstanceId, ItemLedger, ItemLedgerSnapshot, ItemState, TransactionError, TransactionId,
    TransactionRequest, WeaponReloadRequest,
};
use manifest::{PreparedPlacement, PreparedSceneManifest, PreparedSemantic};
use paths::{CellSelector, normalize_asset_path, parse_cell_selector, placement_transform_parts};
use selectors::{CellSummary, SelectionSpec, resolve_selection};

#[derive(Debug, Default)]
struct FaceGenFeatureState {
    actor: facegen::FaceGenRaw,
    race: Option<facegen::FaceGenRaw>,
    result: Option<Result<facegen::FaceGenResolved, facegen::FaceGenDiagnostic>>,
}

#[derive(Debug, Default, cucumber::World)]
struct BevyoutWorld {
    // -- coordinates.feature --
    position: [f32; 3],
    rotation: [f32; 3],
    scale: f32,
    translation: [f32; 3],
    rotation_xyzw: [f32; 4],
    scale_out: f32,
    recovered_position: [f32; 3],
    selector: Option<CellSelector>,
    normalized_path: Option<String>,

    // -- manifest.feature --
    manifest: Option<PreparedSceneManifest>,

    // -- asset_materials.feature --
    is_static: bool,
    conversion: Option<AssetConversion>,
    authored_emission: Option<[f32; 3]>,
    source_emission_strength: Option<f32>,
    explicit_emission: bool,
    bulb_emission_override: bool,
    glow_emission_override: bool,
    emission_policy: Option<assets::MaterialEmissionPolicy>,
    material_glossiness: Option<Option<f32>>,
    metallic_csv: Option<String>,
    metallic_csv_rejected: bool,
    material_diffuse_texture: Option<String>,
    material_roughness: Option<f32>,
    material_metallic: Option<f32>,
    material_specular_enabled: bool,
    material_normal_texture: Option<String>,
    material_specular_texture: Option<String>,
    directx_normal_texel: Option<[u8; 4]>,
    converted_normal_texel: Option<[u8; 4]>,
    staged_texture_path: Option<String>,
    staged_texture_is_normal: Option<bool>,

    // -- cell_map.feature --
    cell_map_cells: Vec<cell_map::CellMapEntry>,
    cell_map_worldspaces: Vec<cell_map::WorldspaceEntry>,
    cell_map_doors: Vec<cell_map::DoorEdge>,
    cell_map_unresolved: u32,
    cell_map: Option<cell_map::CellMap>,
    cell_map_ron_a: Option<String>,
    cell_map_ron_b: Option<String>,

    // -- prepare_selectors.feature --
    cells: Vec<CellSummary>,
    worldspace_names: Vec<(u32, String)>,
    selection_result: Option<Result<Vec<u32>, String>>,

    // -- batch_session.feature --
    batch_physics_cache: batch_cache::KeyedBatchCache<()>,
    batch_asset_totals: batch_cache::BatchAssetTotals,
    batch_cache_dir: Option<std::path::PathBuf>,
    written_cell_map_path: Option<std::path::PathBuf>,
    written_cell_map: Option<cell_map::CellMap>,

    // -- preload_policy.feature --
    preload_doors: Vec<policy::DoorLink>,
    preload_prepared: std::collections::HashSet<u32>,
    preload_resident: Vec<u32>,
    preload_active_cell: u32,
    preload_budget: usize,
    preload_plan: Option<policy::PreloadPlan>,

    // -- resumable_prepare.feature (issue #48) --
    job_manifest: Option<jobs::JobManifest>,
    job_manifest_path: Option<std::path::PathBuf>,
    job_resume_result: Option<(Vec<u32>, usize)>,

    // -- instant_swap.feature --
    swap_residency: Option<swap_policy::Residency>,
    swap_manifest_exists: bool,
    swap_decision: Option<swap_policy::SwapDecision>,
    swap_fallback_load_ok: bool,
    swap_fallback_outcome: Option<swap_policy::FallbackOutcome>,
    collider_work: Vec<(usize, bool)>,
    collider_phase_partitions: Option<(Vec<usize>, Vec<usize>)>,

    // -- fingerprints.feature (issue #49) --
    fingerprint_current: Option<fingerprints::CellFingerprints>,
    fingerprint_stale_components: Option<Vec<fingerprints::FingerprintComponent>>,
    fingerprint_resume_result: Option<(Vec<u32>, usize, fingerprints::StaleCells)>,

    // -- first_reveal.feature (issue #55) --
    reveal_candidates: Vec<reveal_policy::RevealCandidate>,
    reveal_budget: usize,
    reveal_chunks: Vec<Vec<usize>>,

    // -- door_animation.feature (issue #57) --
    animation_clip_names: Vec<String>,
    animation_selected_clip: Option<Option<String>>,
    animation_open_clip_seconds: Option<f32>,
    animation_open_lead: Option<f32>,

    // -- rust_irradiance.feature --
    bake_volume_scale: [f32; 3],
    bake_probe_spacing: f32,
    bake_resolution: [u32; 3],
    bake_atlas_dimensions: [u32; 3],
    bake_samples: u32,
    bake_primary_rays: usize,

    // -- loading_fallback.feature (issue #59) --
    fallback_state: Option<swap_policy::FallbackState>,
    fallback_lifecycle_outcome: Option<swap_policy::FallbackLifecycleOutcome>,
    overlay_fade_duration: f32,
    overlay_fade_max_alpha: f32,
    overlay_alpha: Option<f32>,

    // -- resumable_bake.feature (issue #62) --
    bake_recorded: std::collections::BTreeMap<u32, Option<manifest::PreparedBake>>,
    bake_validity: std::collections::BTreeMap<u32, bool>,
    bake_valid_result: Option<bool>,
    bake_resume_result: Option<(Vec<u32>, usize, Vec<u32>)>,

    // -- state_persistence.feature (issues #60/#61) --
    persist_placements: Vec<persist_policy::PlacementInfo>,
    persist_deltas: std::collections::HashMap<u32, persist_policy::ReferenceDelta>,
    persist_effective: Option<std::collections::HashMap<u32, bool>>,
    persist_baselines: Vec<persist_policy::BaselinePlacement>,
    persist_snapshots: Vec<persist_policy::RuntimeSnapshot>,
    persist_captured: Option<std::collections::HashMap<u32, persist_policy::ReferenceDelta>>,
    persist_applications: Vec<persist_policy::PlacementApplication>,

    // -- collider_ownership.feature (issue #63) --
    ownership_ledger: ownership_policy::CellColliderLedger<u64, u64>,
    ownership_released: Option<Option<ownership_policy::CellColliders<u64, u64>>>,
    ownership_next_id: u64,

    // -- container_audio.feature --
    container_audio_is_container: bool,
    container_audio_record_open: Option<u32>,
    container_audio_record_close: Option<u32>,
    container_audio_cues: Vec<container_audio_policy::AnimationSoundCue>,
    container_audio_selected: Option<container_audio_policy::SelectedContainerAudio>,
    container_audio_resolved_open: Option<u32>,
    container_audio_resolved_close: Option<u32>,
    container_audio_prepared_open: Option<u32>,
    container_audio_prepared_close: Option<u32>,

    // -- inventory.feature (M3 wave 1, issues #71/#72) --
    player_inventory: inventory_policy::Inventory,
    inventory_weights: std::collections::HashMap<u32, f32>,
    inventory_drop_action: Option<inventory_policy::DropAction>,
    inventory_transfer: Option<inventory_policy::TransferResult>,

    // -- performance_probe.feature --
    performance_samples: Vec<performance_policy::FrameSample>,
    performance_summary: Option<performance_policy::FrameProbeSummary>,

    // -- m6_wave2_exterior.feature --
    m6_exterior_index:
        std::collections::BTreeMap<bevyout_core::manifest::exterior::GridCoordinate, u32>,
    m6_exterior_states: Vec<bevyout_core::manifest::exterior::ExteriorCellState>,
    m6_exterior_input: Option<bevyout_core::manifest::exterior::ExteriorResidencyInput>,
    m6_exterior_plan: Option<bevyout_core::manifest::exterior::ExteriorResidencyPlan>,
    m6_exterior_memory_state: Option<lifecycle::ExteriorStreamState>,
    m6_exterior_memory: Option<exterior_diagnostics::ProcessMemoryDiagnostics>,
    m6_exterior_memory_report: Option<serde_json::Value>,

    // -- leveled_lists.feature (issue #74) --
    leveled_lists: std::collections::BTreeMap<u32, leveled::PreparedLeveledList>,
    leveled_seeds: std::collections::HashMap<String, leveled::LeveledSeed>,
    leveled_last_resolution: Option<Vec<(u32, i32)>>,
    // -- container_transfer.feature (issue #75) --
    container_seed_entries: Vec<container_policy::SeedEntry>,
    container_resolver_lists: std::collections::BTreeMap<u32, Vec<(u32, i32)>>,
    container_resolver_calls: u32,
    container_state: Option<container_policy::ContainerState>,
    container_stacks: Vec<(u32, i32)>,
    player_stacks: Vec<(u32, i32)>,
    transfer_result: Option<Result<i32, container_policy::TransferError>>,
    // -- container_persistence.feature (issue #76) --
    container_baselines: std::collections::HashMap<u32, persist_policy::ContainerBaseline>,
    container_snapshots: std::collections::HashMap<u32, persist_policy::ContainerSnapshot>,
    container_captured: Option<std::collections::HashMap<u32, persist_policy::ContainerDelta>>,
    container_deltas: std::collections::HashMap<u32, persist_policy::ContainerDelta>,
    container_seeded: Option<std::collections::HashMap<u32, persist_policy::ContainerSnapshot>>,
    // -- item_flags.feature (issue #81) --
    flag_record_flags: u32,
    rule_result: Option<Result<(), item_rules::TransferRejection>>,
    carried_stacks: Vec<(i32, bool, f32)>,
    carried_total: Option<f32>,
    take_classification: Option<item_rules::TakeClassification>,

    // -- item_transactions.feature (M3/#95) --
    canonical_ledger: item_transaction::ItemLedger,
    canonical_result:
        Option<Result<item_transaction::TransactionReceipt, item_transaction::TransactionError>>,

    // -- drop_placement.feature (M3/#95) --
    drop_blocked_count: usize,
    drop_all_blocked: bool,
    drop_decision: Option<drop_policy::DropPlacementDecision>,

    // -- item_use.feature (issue #99) --
    item_use_stats: Option<item_use::ItemStats>,
    item_use_quest_item: bool,
    item_use_action: Option<item_use::ItemUseAction>,

    // -- equipment.feature (issue #98) --
    equipment_state: equipment::EquipmentState,
    equip_result: Option<Result<equipment::EquipOutcome, equipment::EquipError>>,

    // -- recipes.feature (issue #117) --
    recipe_under_test: Option<recipe_policy::PreparedRecipe>,
    recipe_available_items: std::collections::BTreeSet<u32>,
    recipe_validation: Option<Result<(), recipe_policy::RecipeValidationError>>,

    // -- hybrid_lighting.feature --
    hybrid_prepared_visibility: Option<f32>,
    hybrid_realtime_visibility: Option<f32>,
    hybrid_combined_visibility: Option<f32>,

    // -- actor_catalog.feature (issue #103, M4 wave 1 task C) --
    actor_catalog_inputs: actor_catalog::ActorCatalogInputs,
    actor_catalog_result: Option<actor_catalog::PreparedActorCatalog>,

    // -- actor_conversion.feature --
    actor_skeleton: String,
    actor_visual_inputs: Vec<String>,
    actor_assembly: Option<assets::ActorAssemblyDescriptor>,
    actor_gear_kinds: Vec<String>,
    retained_actor_gear_kinds: Vec<String>,
    actor_apparel_candidates: Vec<actor_appearance::ApparelCandidate>,
    unavailable_actor_models: std::collections::HashSet<String>,
    actor_outfit: Option<actor_appearance::SpawnOutfit>,

    // -- nav_graph.feature (issue #111, M4 wave 2) --
    nav_cell_form_id: u32,
    nav_navm_form_id: u32,
    /// Pending NAVM subrecord payloads (signature, raw bytes), assembled
    /// into one synthetic plugin byte stream by the When step.
    nav_navm_parts: Vec<(String, Vec<u8>)>,
    nav_navi_first: Option<NaviFixtureEntry>,
    nav_navi_second: Option<NaviFixtureEntry>,
    nav_navi_second_deleted: Option<u32>,
    nav_parsed: Option<openmw_esm4::ParsedPlugin>,
    nav_graph_inputs: nav_graph::NavGraphInputs,
    nav_graph_result: Option<nav_graph::PreparedNavGraph>,

    // -- nav_backend.feature (issue #112, M4 wave 3) --
    nav_backend_meshes: Vec<landmass_graph::MeshInput>,
    nav_backend_build_result: Option<landmass_graph::BuildResult>,
    nav_backend_descriptors: Option<Vec<landmass_graph::DoorLinkDescriptor>>,
    nav_backend_second_descriptors: Option<Vec<landmass_graph::DoorLinkDescriptor>>,
    nav_backend_door_link_state: door_link::DoorLinkState,

    // -- nav_adapter.feature (issue #113, M4 wave 4) --
    /// Raw bytes appended to the first NAVI fixture entry's 16-byte NVMI
    /// header (the island tail under test).
    nav_navi_first_tail: Vec<u8>,
    nav_adapter_repath_observation: repath::RepathObservation,
    nav_adapter_repath_decision: Option<repath::RepathDecision>,
    nav_adapter_door_observation: repath::DoorObservation,
    nav_adapter_second_graph: Option<nav_graph::PreparedNavGraph>,
    nav_adapter_single_sided: Option<Vec<landmass_graph::SingleSidedDoor>>,
    nav_adapter_merge_inputs: Vec<landmass_graph::MergeInput>,
    nav_adapter_merge_links: Option<Vec<landmass_graph::MergeLinkDescriptor>>,

    // -- nav_ledger.feature (issue #134, M4 wave 4) --
    nav_ledger: ledger_policy::Ledger,
    nav_ledger_claim_result: Option<ledger_policy::ClaimResult>,
    nav_ledger_route_door: Option<u32>,
    nav_ledger_eligibility: Option<ledger_policy::SwapEligibility>,

    // -- nav_movement.feature (issue #114, M4 wave 5) --
    nav_movement_grounded_observation: movement_policy::GroundedObservation,
    nav_movement_grounded_decision: Option<bool>,
    nav_movement_velocity_observation: Option<movement_policy::VelocityObservation>,
    nav_movement_collision_outcome: Option<movement_policy::CollisionOutcome>,
    nav_movement_stuck_observation: Option<movement_policy::StuckObservation>,
    nav_movement_stuck_decision: Option<movement_policy::StuckDecision>,

    // -- nav_door_gate.feature (issue #137, M4 wave 5) --
    nav_door_gate_observation: door_link::CrossingObservation,

    // -- nav_movement.feature (issue #114 added scope, M4 wave 5): solve-rate
    // divisor gating `LandmassSystems::Update`.
    nav_solve_step: u64,
    nav_solve_interval: u32,
    nav_solve_decision: Option<bool>,

    // -- nav_movement.feature (issue #114 added scope, M4 wave 5): solve-
    // output interpolation fraction.
    nav_solve_steps_since_solve: u32,
    nav_solve_blend_interval: u32,
    nav_solve_blend_fraction: Option<f32>,

    // -- note_text.feature (issue #123) --
    // Raw concatenated subrecord bytes (built with `nav_subrecord`) for the
    // synthetic NOTE record's body: `openmw_esm4::Subrecord`'s fields are
    // module-private, so this suite drives the real byte-level ESM4 decode
    // the same way `nav_graph.feature`'s steps do rather than constructing
    // decoded types directly.
    note_record_data: Vec<u8>,
    note_base: Option<openmw_esm4::BaseRecord>,
    note_prepared_stats: Option<manifest::PreparedItemStats>,

    // -- real_corpses.feature (issue #120, M4 wave 6) --
    corpse_cell_form_id: u32,
    /// Pending synthetic ACHR fixtures (reference FormID, base FormID,
    /// starts-dead), assembled into one synthetic plugin byte stream by the
    /// When step.
    corpse_achr_entries: Vec<(u32, u32, bool)>,
    /// Pending synthetic `NPC_` base-record fixtures (base FormID,
    /// starts-dead), assembled into the same synthetic plugin byte stream
    /// by the When step (issue #120 rework: the real FO3 starts-dead
    /// signal lives on the base record's header flags, not the ACHR
    /// reference's).
    corpse_npc_bases: Vec<(u32, bool)>,
    corpse_parsed: Option<openmw_esm4::ParsedPlugin>,

    // -- actor_conversion.feature (authored ragdoll sidecar v3) --
    actor_ragdoll_joint: Option<physics::PreparedPhysicsJoint>,
    actor_physics_asset: Option<physics::PreparedPhysicsAsset>,

    // -- native_conversion.feature --
    native_outcomes: Vec<native_policy::NativeJobOutcome>,
    native_summary: Option<native_policy::NativeBatchSummary>,
    native_sorted_indices: Vec<usize>,
    native_asset_count: usize,
    native_requested_workers: Option<usize>,
    native_host_workers: usize,
    native_worker_count: Option<usize>,
    requested_converter: Option<converter_policy::ConverterBackend>,
    resolved_converter: Option<converter_policy::ConverterBackend>,
    required_gltf_extensions: Vec<String>,
    unsupported_gltf_extensions: Vec<String>,

    // -- actor_assembly.feature / actor_fallback.feature (#107, #108) --
    actor_mesh_parts: Vec<actor::AssembledMeshPart>,
    actor_occupied_slots: u32,
    actor_hair_visible: Option<bool>,
    actor_eyes_visible: Option<bool>,
    actor_weapon_candidates: Vec<actor::ActorWeaponCandidate>,
    actor_selected_weapon: Option<actor::AssembledWeapon>,
    actor_scale_kind: actor::ActorKind,
    actor_reference_scale: f32,
    actor_race_scale: Option<f32>,
    actor_base_scale: Option<f32>,
    actor_resolved_scale: Option<f32>,
    actor_fallback_input: actor::ActorAppearanceAvailability,
    actor_fallback_supplied_reasons: Vec<actor::ActorFallbackReason>,
    actor_fallback_decision: Option<actor::ActorFallbackDecision>,
    // -- nav_stuck_progress.feature (issue #157) --
    nav_stuck_progress_desired: [f32; 2],
    nav_stuck_progress_achieved: [f32; 2],
    nav_stuck_progress_delta: Option<f32>,
    /// "u_shaped", "blocked", or "oscillating" -- picks the per-tick
    /// desired/achieved generator `when_route_is_simulated` runs.
    nav_stuck_progress_route_kind: Option<String>,
    nav_stuck_progress_route_ticks: u32,
    nav_stuck_progress_route_speed: f32,
    nav_stuck_progress_start_recovery_tick: Option<u32>,
    nav_stuck_progress_stuck_tick: Option<u32>,
    // -- nav_stuck_progress.feature (issue #157 follow-up: avoidance-pause
    // and repath-mid-route scenarios) --
    nav_stuck_progress_pause_progress_ticks: u32,
    nav_stuck_progress_pause_speed: f32,
    nav_stuck_progress_pause_ticks: u32,
    nav_stuck_progress_repath_blocked_ticks: u32,
    nav_stuck_progress_repath_leg_ticks: u32,
    nav_stuck_progress_repath_leg_speed: f32,

    // -- nav_door_topology.feature (issue #155) --
    nav_door_topology_type_indices: Option<std::collections::BTreeMap<u32, usize>>,
    nav_door_topology_triangle: Option<[[f32; 3]; 3]>,
    nav_door_topology_point: Option<[f32; 3]>,
    // -- actor_animation_catalog.feature / actor_animation_conversion.feature
    // (issue #104, M4 wave 10): append-only shared seam. --
    npc_kffz_payload: Vec<u8>,
    creature_kffz_payload: Vec<u8>,
    npc_kffz_paths: Vec<String>,
    creature_kffz_paths: Vec<String>,
    actor_animation_discovery_inputs: Vec<actor_animation::ActorAnimationDiscoveryInput>,
    actor_animation_assets: Vec<actor_animation::ActorAnimationAsset>,
    actor_animation_catalog: Option<actor_animation::PreparedActorAnimationCatalog>,
    requested_actor_animation_converter: Option<converter_policy::ActorAnimationBackend>,
    resolved_actor_animation_converter: Option<converter_policy::ActorAnimationBackend>,
    actor_animation_pack_cache_state: actor_animation_cache::ActorAnimationPackCacheState,
    actor_animation_pack_cache_decision:
        Option<actor_animation_cache::ActorAnimationPackCacheDecision>,

    // -- nav_fall_guard.feature (issue #164) --
    nav_fall_guard_bounds_min_y: Option<f32>,
    nav_fall_guard_agent_y: Option<f32>,
    nav_fall_guard_kill_z: Option<f32>,
    nav_fall_guard_verdict: Option<fall_guard::FallVerdict>,

    // -- nav_travel_lock.feature (issue #165 real-data acceptance
    // follow-up: `door_link::effective_door_open`) --
    nav_travel_lock_destination: Option<door_link::LinkDestination>,
    nav_travel_lock_physically_open: bool,
    nav_travel_lock_locked: bool,

    // -- nav_collision_clearance.feature (issue #153, M4 wave 10) --
    nav_clearance_mesh: nav_clearance::NavClearanceMeshInput,
    nav_clearance_collision: Vec<nav_clearance::CollisionTriangle>,
    nav_clearance_result: Option<nav_clearance::NavClearanceResult>,

    // -- nav_portal_quarantine.feature (issue #162, M4 wave 10: per-link
    // merge-portal quarantine) --
    nav_quarantine_candidate_index: Option<usize>,
    nav_quarantine_resolved_kind: Option<usize>,
    nav_quarantine_kind_count: usize,
    nav_quarantine_excluded_kinds: std::collections::BTreeSet<usize>,
    nav_quarantine_permitted: Option<Option<std::collections::BTreeSet<usize>>>,

    // -- ai_packages.feature (issue #175, M4 wave 11 lane C) --
    package_catalog_inputs: package_catalog::PackageCatalogInputs,
    package_catalog_result: Option<package_catalog::PreparedPackageCatalog>,

    // -- actor_animation_gameflow.feature (#106, M4 wave 12) --
    gameplay_actor_weapon_type: Option<u32>,
    gameplay_actor_weapon_prefix: Option<Option<&'static str>>,
    gameplay_actor_kind: actor_animation::PreparedActorAnimationKind,
    gameplay_actor_female: bool,
    gameplay_actor_clips: Vec<actor_animation::PreparedActorAnimationClip>,
    gameplay_actor_requested_state: actor_animation_policy::ActorAnimationState,
    gameplay_actor_selection: Option<actor_animation_policy::ClipSelection>,
    gameplay_actor_next_state: Option<actor_animation_policy::ActorAnimationState>,
    gameplay_actor_cell_active: bool,
    gameplay_actor_visible: bool,
    gameplay_actor_playback_active: Option<bool>,

    // -- nav_derived_doors.feature (issue #177, M4 wave 11) --
    nav_derived_door_blockers: Vec<nav_doors::BlockerVolume>,
    nav_derived_door_meshes: Vec<nav_doors::BlockerMeshInput>,
    nav_derived_door_associations: Option<Vec<nav_doors::DerivedDoorAssociation>>,
    nav_approach_observation: Option<door_link::ApproachObservation>,

    // -- actor_state.feature (issue #110, M4 wave 13) --
    actor_state_definition: actor_state::ActorDefinition,
    actor_state_instance: actor_state::ActorInstanceState,
    actor_state_resolved: Option<actor_state::ResolvedActorValue>,
    actor_state_store: actor_state::ActorStateStore,
    actor_state_serialized: Option<String>,

    // -- nav_locomotion.feature (issue #188) --
    nav_locomotion_state: locomotion::LocomotionState,
    nav_locomotion_changed: bool,
    // -- console_qol.feature (issue #201) --
    console_history: console_openmw_ui::CommandHistory,
    console_transcript: console_openmw_ui::ConsoleTranscript,

    // -- pause_menu.feature --
    pause_menu: pause_menu::PauseMenuState,
    pause_menu_action: Option<Option<pause_menu::PauseMenuAction>>,

    // -- ai_package_selection.feature (issue #193) --
    ai_sel_candidates: Vec<ai_selection::PackageCandidate>,
    ai_sel_hour: f32,
    ai_sel_functions: std::collections::HashMap<u16, f32>,
    ai_sel_report: Option<ai_selection::SelectionReport>,

    // -- ai_package_lifecycle.feature (issue #194) --
    ai_lifecycle: ai_lifecycle::PackageLifecycle,
    ai_lifecycle_checkpoint: Option<bevyout_core::actor_state::ActorPackageCheckpoint>,

    // -- ai_package_resolution.feature (issue #195) --
    ai_res_context: ai_resolution::ResolutionContext,
    ai_res_location: Option<ai_resolution::PackageLocation>,
    ai_res_target: Option<ai_resolution::PackageTarget>,
    ai_res_result:
        Option<Result<ai_resolution::ResolvedPoint, ai_resolution::ResolutionDiagnostic>>,

    // -- faction_hostility.feature (issue #116) --
    hostility_table: faction::FactionRelationTable,
    hostility_observer: disposition::DispositionActor,
    hostility_target: disposition::DispositionTarget,
    hostility_result: Option<disposition::DispositionResult>,

    // -- perception_awareness.feature (issue #116) --
    perception_config: Option<perception::PerceptionConfig>,
    perception_state: perception::AwarenessState,
    perception_candidates: Vec<perception::PerceptionInputs>,
    perception_last_event: Option<perception::AwarenessEvent>,

    // -- nav_door_access.feature (issue #185): key-aware locked doors and
    // the trapped-door barrier, `viewer::nav::openmw_doors::door_openable`.
    nav_door_access_observation: openmw_doors::DoorAccessObservation,
    nav_door_access_result: Option<bool>,

    // -- package_families.feature (issues #196/#197) --
    pf_driver: Option<ai_families::FamilyDriver>,
    pf_markers: Vec<[f32; 3]>,
    pf_step: Option<ai_families::FamilyStep>,

    // -- ai_follow_sandbox.feature (issue #198) --
    // The follow leader's current position (the moving target), the roam
    // centre + radius a sandbox is bounded by, and any door the route is
    // reported blocked on -- all fed into the pure driver via observations.
    fs_leader: Option<[f32; 3]>,
    fs_roam_center: [f32; 3],
    fs_roam_radius: f32,
    fs_blocking_door: Option<u32>,

    // -- ai_package_points.feature (issue #213): editor-location/patrol
    // marker-chain resolution built from the manifest, not just spawned
    // entities.
    pp_chain_result: Vec<ai_resolution::ResolvedPoint>,

    // -- player_weapon.feature (M5 wave 1, issues #235-#238) --
    weapon_state: Option<weapon::WeaponState>,
    weapon_last_fire: Option<weapon::FireDecision>,
    weapon_ammo_consumed: u32,
    weapon_actor_definition: actor_state::ActorDefinition,
    weapon_actor_instance: actor_state::ActorInstanceState,
    weapon_damage_outcome: Option<weapon::DamageOutcome>,

    // -- reflection_probes.feature: deterministic, capped room allocation --
    reflection_probe_region_areas: Vec<f32>,
    reflection_probe_counts: Vec<usize>,

    // -- m5_wave2_ammunition.feature (issues #244-#247) --
    ammo_magazine: bevyout_core::combat::ammo::MagazineState,
    ammo_capacity: u16,
    ammo_reserve: u32,
    ammo_reload: Option<bevyout_core::combat::ammo::ReloadDecision>,
    ammo_fire: Option<Result<(), bevyout_core::combat::ammo::FireBlockReason>>,

    // -- day_night_lighting.feature --
    day_night_interior: bool,
    day_night_behave_like_exterior: bool,
    day_night_preview: bool,
    day_night_dynamic: bool,
    game_hour: f32,
    game_timescale: f32,
    weather_keyframes: time_of_day::ColorKeyframes,
    sampled_weather_color: [f32; 4],
    preview_weather_candidates: Vec<(u32, Option<String>)>,
    selected_preview_weather: Option<u32>,

    // -- script_inventory.feature (M7 wave 1, issue #252) --
    script_record_versions: Vec<(u32, String, Option<String>)>,
    script_record_winners: Option<record_winners::WinningRecords<String>>,
    script_structural_inputs: Vec<(String, Vec<u8>, usize)>,
    script_decoded_record: Option<record::DecodedScriptRecord>,
    script_owner_signature: String,
    script_owner_plugin_index: u8,
    script_extracted_owner: Option<attachments::ExtractedOwnerScripts>,
    script_inventory_report: Option<report_schema::CompatibilityReport>,
    script_inventory_renders: Vec<String>,
    // -- autonomous_actors.feature (issues #218/#224) --
    autonomous_life_state_alive: bool,
    autonomous_already_nav_bound: bool,
    autonomous_already_has_controller: bool,
    autonomous_eligible: Option<bool>,
    nav_locomotion_changes_after_warmup: u32,

    // -- dialogue.feature (standalone Yarn dialogue waves) --
    dialogue_sources: Vec<dialogue::DialogueSource>,
    dialogue_catalog: Option<dialogue::PreparedDialogueCatalog>,
    dialogue_catalog_again: Option<dialogue::PreparedDialogueCatalog>,
    dialogue_bundle: Option<bevyout_core::dialogue::PreparedDialogueBundleRef>,
    dialogue_voice_index: Option<bevyout_core::dialogue::PreparedDialogueVoiceIndex>,
    dialogue_voice_fingerprints: Option<(String, String)>,
    dialogue_host_command: Option<dialogue_host::HostCommand>,
    dialogue_snapshot: bevyout_core::dialogue::DialogueSnapshot,
    dialogue_policy: bevyout_core::dialogue::DialoguePresentationPolicy,
    dialogue_inventory: Option<dialogue::FalloutDialogueInventory>,
    dialogue_generated: Option<(
        dialogue::DialogueSource,
        Vec<dialogue::DialogueSourceMapping>,
    )>,
    dialogue_coverage: Option<bevyout_core::dialogue::DialogueCoverageReport>,
    dialogue_repair_guidance: Option<String>,
    dialogue_metadata_valid: Option<bool>,

    // -- image_space.feature (#96) --
    screen_fx: screen_fx_policy::ScreenFxPolicy,
    screen_fx_definition: Option<screen_fx_policy::ScreenFxDefinition>,
    screen_fx_sample: Option<screen_fx_policy::ScreenFxValues>,

    // -- m5_wave3_condition.feature (#262, #263) --
    combat_ledger: Option<ItemLedger>,
    combat_rng: CombatRngState,
    combat_policy: WeaponConditionPolicy,
    combat_receipt: Option<Result<CombatTransactionReceipt, TransactionError>>,
    combat_snapshot_before: Option<ItemLedgerSnapshot>,
    combat_rng_before: Option<CombatRngState>,

    // -- realtime_shadow_policy.feature (issue #267, PERF wave 1) --
    realtime_shadow_settings_changed: bool,
    realtime_shadow_selection_active: bool,
    realtime_shadow_writes_needed: Option<bool>,

    // -- scene_classification.feature (issue #270, PERF wave 1) --
    scene_ao_tracker: ao_policy::AoEligibilityTracker<u32, u32>,
    glow_name_check: Option<bool>,

    // -- material_clamp_policy.feature (issue #269, PERF wave 1) --
    clamp_settings: material_clamp_policy::ClampSettings,
    clamp_store: material_clamp_policy::ClampStore<u32>,
    clamp_materials: std::collections::HashMap<u32, material_clamp_policy::MaterialFactors>,
    clamp_full_pass_ran: Option<bool>,

    // -- m6_wave3_policy.feature --
    m6_actor_identity: Option<actor_residency::ActorIdentity>,
    m6_actor_owners: Vec<actor_residency::ActorResidencyOwner>,
    m6_actor_decision: Option<actor_residency::ActorResidencyDecision>,
    m6_nav_residents: Vec<landmass_graph::ResidentNavCell>,
    m6_nav_portals: Vec<landmass_graph::ResidentPortal>,
    m6_nav_topology: landmass_graph::ResidentNavTopology,

    // -- M4 Craterside NPC animation repair, Lane B package seams --
    package_point_actor_packages: Vec<Vec<u32>>,
    package_point_packages: std::collections::HashMap<u32, package_catalog::PackageInput>,
    package_point_references: Vec<package_catalog::PackagePointReference>,
    package_point_result: Vec<package_catalog::PackagePointReference>,
    package_pkdt_fixtures: Vec<(u32, usize, u8, u16, u32)>,
    package_pkdt_collections:
        std::collections::HashMap<u32, package_catalog::PreparedPackageIdleCollection>,

    // -- actor_animation_catalog.feature / actor_animation_conversion.feature
    // (M4 Craterside Lane C): append-only authored IDLE seam. --
    authored_idle_definitions: Vec<actor_animation::PreparedActorIdleDefinition>,
    authored_idle_order: Option<actor_animation::PreparedActorIdleOrder>,
    authored_idle_diagnostics: Vec<String>,
    authored_idle_hash: Option<String>,
    authored_idle_conversion_set_count: usize,
    authored_idle_conversion_clip_count: usize,
    authored_idle_revision: Option<String>,

    // -- actor_animation_gameflow.feature Lane D idle runtime/policy seam --
    idle_lifecycle: actor_idle_policy::IdleLifecycleFacts,
    idle_package: Option<actor_idle_policy::IdlePackageContext>,
    idle_global_definitions: Vec<actor_animation::PreparedActorIdleDefinition>,
    idle_selected: Option<actor_idle_policy::IdleSelection>,
    idle_decision: Option<actor_idle_policy::IdleDecision>,
    idle_authority: actor_idle_policy::IdleAuthority,
    idle_same_epoch_selection: Option<(
        actor_idle_policy::IdleDecision,
        actor_idle_policy::IdleDecision,
    )>,
    idle_loop_count: Option<u8>,

    // -- actor_fallback.feature (M4 static NPC FaceGen reconstruction) --
    facegen_feature: FaceGenFeatureState,

    // -- asset_materials.feature legacy world shading --
    legacy_chan_master_strength: f32,
    legacy_glossiness_exponent: Option<f32>,
    legacy_micro_roughness: Option<f32>,
    legacy_chan_weight: Option<f32>,

    // -- fallout_overlays.feature --
    fallout_overlay_editor_id: Option<String>,
    fallout_overlay_model: Option<String>,
    fallout_overlay_kind: Option<overlay_policy::FalloutOverlayKind>,

    // -- cache_stats.feature (prepared-cache compression Wave 0) --
    cache_stat_files: Vec<cache_stats_policy::CacheFileFacts>,
    cache_stat_summary: Option<cache_stats_policy::CacheStorageSummary>,

    // -- cache_object_store.feature (prepared-cache compression Wave 1) --
    prepared_source_path: String,
    normalized_prepared_source_path: Option<Result<String, String>>,
    prepared_recipe_ids: Vec<String>,

    // -- day_night_lighting.feature (shared exterior weather catalog) --
    shared_weather_environment: bevyout_core::manifest::exterior::PreparedExteriorEnvironment,
    shared_weather_catalog: Vec<bevyout_core::manifest::exterior::PreparedWeatherCatalogEntry>,
    shared_weather_resolved: Option<bevyout_core::manifest::exterior::PreparedWeatherProfile>,

    // -- exterior_scene_storage.feature (prepared-cache compression) --
    exterior_storage_worldspace: u32,
    exterior_storage_cell: u32,
    exterior_storage_plan: Option<exterior_scene_policy::ExteriorSceneStoragePlan>,

    // -- m6_wave3_runtime.feature (M6 W3-C exterior actor residency) --
    w3c_cells: Vec<exterior_actor_plan::PlannedCell>,
    w3c_live: Vec<exterior_actor_plan::PlannedLiveActor>,
    w3c_plan: Vec<exterior_actor_plan::PlannedRequest>,
    w3c_max_owners: usize,

    // -- rpg_stats.feature (M9 W1 #309 pure stat kernels) --
    rpg_sheet: stats::CharacterSheet,
    rpg_settings: stats::GmstSettings,
    rpg_last_award: Option<stats::AwardXpOutcome>,
    rpg_clamped_resistance_bps: u32,

    // -- rpg_perks.feature (M9 W2 #313 pure perk kernels) --
    rpg_perk_defs: std::collections::BTreeMap<u32, perks::PerkDefinition>,
    rpg_perk_progression: perks::PerkProgression,
    rpg_perk_modifiers: perks::PerkModifiers,
    rpg_perk_eligibility: Option<(u32, perks::PerkEligibility)>,

    // -- rpg_effects.feature (M9 W3 #317 pure effect/radiation/addiction
    //    kernels) --
    rpg_effect_ledger: effects::ActiveEffectsLedger,
    rpg_radiation_pool: radiation::RadiationPool,
    rpg_last_dose: Option<radiation::RadiationDoseOutcome>,
    rads_removed: u16,
    rpg_expired_effects: Vec<effects::ActiveEffect>,
    rpg_projected_special: std::collections::BTreeMap<actor_state::SpecialAttribute, u8>,
    rpg_projected_derived: stats::DerivedAttributes,
    rpg_perk_progression_effects: perks::PerkProgression,
    rpg_addictions: chems::Addictions,
    rpg_rng: chems::RpgRngState,
    rpg_addiction_roll: Option<(bool, u32)>,

    // -- rpg_limbs.feature (M9 W4 body parts, cripple, medical) --
    rpg_limbs: bevyout_core::combat::LimbState,
    rpg_mapped_part: Option<bevyout_core::combat::BodyPartId>,
    rpg_last_limb_impact: Option<bevyout_core::combat::LimbImpactOutcome>,
    rpg_last_restoration: Option<bevyout_core::combat::RestorationOutcome>,
    rpg_last_impact_outcome: Option<weapon::ImpactOutcome>,
    rpg_actor_definition: actor_state::ActorDefinition,
    rpg_actor_state: actor_state::ActorInstanceState,

    // -- rpg_repair_barter.feature (M9 W5 repair, craft, barter) --
    rpg_repair_skill: u8,
    rpg_repair_max_condition: u32,
    rpg_repair_result:
        Option<Result<bevyout_core::repair::RepairReceipt, bevyout_core::repair::RepairError>>,
    rpg_craft_recipes: std::collections::BTreeMap<u32, bevyout_core::crafting::RecipeDefinition>,
    rpg_craft_result:
        Option<Result<bevyout_core::crafting::CraftReceipt, bevyout_core::crafting::CraftError>>,
    rpg_next_item_id_before: Option<item_transaction::ItemInstanceId>,
    rpg_barter_catalog_value: u64,
    rpg_barter_skill: u8,
    rpg_barter_quote: Option<bevyout_core::barter::BarterQuote>,
    rpg_barter_commit:
        Option<Result<item_transaction::TransactionReceipt, bevyout_core::barter::BarterError>>,
    rpg_restock_state: bevyout_core::barter::MerchantRestockState,
    rpg_restock_outcome: Option<bevyout_core::barter::RestockOutcome>,
}

fn synthetic_dialogue_source() -> dialogue::DialogueSource {
    dialogue::DialogueSource {
        relative_path: "authored/guard.yarn".into(),
        kind: dialogue::DialogueSourceKind::Authored,
        content: "title: Start\n---\nGuard: Welcome.\n-> Continue -> Checkpoint\n===\n\ntitle: Checkpoint\n---\nGuard: The gate opens.\n<<bo_run_action open_gate>>\n===\n".into(),
    }
}

fn synthetic_wav_bytes(sample_rate: u32, samples: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let cursor = std::io::Cursor::new(&mut bytes);
        let mut writer = hound::WavWriter::new(
            cursor,
            hound::WavSpec {
                channels: 1,
                sample_rate,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            },
        )
        .expect("synthetic WAV writer");
        for _ in 0..samples {
            writer.write_sample(0i16).expect("synthetic WAV sample");
        }
        writer.finalize().expect("synthetic WAV finalize");
    }
    bytes
}

#[given("a synthetic Yarn dialogue source")]
async fn given_synthetic_dialogue_source(world: &mut BevyoutWorld) {
    world.dialogue_sources = vec![synthetic_dialogue_source()];
}

#[when("the dialogue source is parsed")]
async fn when_dialogue_source_is_parsed(world: &mut BevyoutWorld) {
    world.dialogue_catalog = Some(dialogue::prepare_catalog(world.dialogue_sources.clone()));
}

#[then("the dialogue source has a Start node")]
async fn then_dialogue_source_has_start_node(world: &mut BevyoutWorld) {
    let catalog = world.dialogue_catalog.as_ref().expect("dialogue catalog");
    assert!(
        catalog
            .conversation(&bevyout_core::dialogue::DialogueKey::new("Start"))
            .is_some_and(|conversation| conversation.nodes.contains_key("Start"))
    );
}

#[when("dialogue preparation is run twice")]
async fn when_dialogue_preparation_is_run_twice(world: &mut BevyoutWorld) {
    let first = dialogue::prepare_catalog(world.dialogue_sources.clone());
    let second = dialogue::prepare_catalog(world.dialogue_sources.clone());
    world.dialogue_catalog = Some(first);
    world.dialogue_catalog_again = Some(second);
    let root =
        std::env::temp_dir().join(format!("bevyout-dialogue-feature-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let output = dialogue::prepare_dialogue_bundle(&root, world.dialogue_sources.clone())
        .expect("synthetic dialogue bundle prepares");
    world.dialogue_bundle = output.bundle;
    assert!(root.join("dialogue/authored/guard.yarn").is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[then("the dialogue preparation fingerprints match")]
async fn then_dialogue_preparation_fingerprints_match(world: &mut BevyoutWorld) {
    assert_eq!(
        world.dialogue_catalog.as_ref().unwrap().bundle_hash(),
        world.dialogue_catalog_again.as_ref().unwrap().bundle_hash()
    );
}

#[when("the dialogue catalog is prepared for runtime")]
async fn when_dialogue_catalog_is_prepared_for_runtime(world: &mut BevyoutWorld) {
    world.dialogue_catalog = Some(dialogue::prepare_catalog(world.dialogue_sources.clone()));
}

#[then("the catalog exposes one line and one choice")]
async fn then_dialogue_catalog_exposes_line_and_choice(world: &mut BevyoutWorld) {
    let catalog = world.dialogue_catalog.as_ref().unwrap();
    let start = catalog
        .conversation(&bevyout_core::dialogue::DialogueKey::new("Start"))
        .unwrap()
        .nodes
        .get("Start")
        .unwrap();
    assert_eq!(start.lines.len(), 1);
    assert_eq!(start.options.len(), 1);
}

#[given(regex = r##"^a synthetic dialogue host command \"([^\"]*)\"$"##)]
async fn given_synthetic_dialogue_host_command(world: &mut BevyoutWorld, command: String) {
    let bridge = dialogue_host::YarnHostBridge::default();
    world.dialogue_host_command = Some(
        bridge
            .command_from_text(&command, &command)
            .expect("synthetic host command is supported"),
    );
}

#[when("the dialogue host command is normalized")]
async fn when_dialogue_host_command_is_normalized(_world: &mut BevyoutWorld) {}

#[then(regex = r##"^the dialogue host command key is \"([^\"]*)\"$"##)]
async fn then_dialogue_host_command_key_is(world: &mut BevyoutWorld, expected: String) {
    let Some(dialogue_host::HostCommand::RunAction { key, .. }) =
        world.dialogue_host_command.as_ref()
    else {
        panic!("expected a normalized RunAction command");
    };
    assert_eq!(key, &expected);
}

#[given("a dialogue snapshot with persistent and session variables")]
async fn given_dialogue_snapshot_with_variables(world: &mut BevyoutWorld) {
    use bevyout_core::dialogue::{DialogueSnapshot, NarrativeValue, NarrativeVariableState};
    let mut variables = NarrativeVariableState::default();
    variables.set("$global_reputation", NarrativeValue::Number(7));
    variables.set("$session_met_guard", NarrativeValue::Bool(true));
    world.dialogue_snapshot = DialogueSnapshot::boundary("fixture", variables);
}

#[when("the dialogue snapshot reaches a boundary")]
async fn when_dialogue_snapshot_reaches_boundary(world: &mut BevyoutWorld) {
    world.dialogue_snapshot.variables.clear_session_boundary();
}

#[then("only the persistent dialogue variable remains")]
async fn then_only_persistent_dialogue_variable_remains(world: &mut BevyoutWorld) {
    assert_eq!(world.dialogue_snapshot.variables.persistent.len(), 1);
    assert!(world.dialogue_snapshot.variables.session.is_empty());
    assert!(world.dialogue_snapshot.variables.temporary.is_empty());
}

#[when("the authored NPC dialogue key is resolved")]
async fn when_authored_npc_dialogue_key_is_resolved(world: &mut BevyoutWorld) {
    world.dialogue_catalog = Some(dialogue::prepare_catalog(world.dialogue_sources.clone()));
}

#[then("the authored NPC conversation is available")]
async fn then_authored_npc_conversation_is_available(world: &mut BevyoutWorld) {
    assert!(
        world
            .dialogue_catalog
            .as_ref()
            .unwrap()
            .conversation(&bevyout_core::dialogue::DialogueKey::new("Start"))
            .is_some()
    );
}

#[given("a synthetic Fallout dialogue inventory")]
async fn given_synthetic_fallout_dialogue_inventory(world: &mut BevyoutWorld) {
    world.dialogue_inventory = Some(dialogue::inventory_fallout_dialogue(&[
        dialogue::FalloutDialogueRecord {
            plugin: "Fixture.esm".into(),
            form_id: 0x100,
            signature: "DIAL".into(),
            topic_key: "Greeting".into(),
            ..Default::default()
        },
        dialogue::FalloutDialogueRecord {
            plugin: "Fixture.esm".into(),
            form_id: 0x101,
            signature: "INFO".into(),
            speaker_form_id: Some(0x200),
            text: Some("Hello".into()),
            topic_key: "Greeting".into(),
            links: vec!["Greeting".into()],
            ..Default::default()
        },
        dialogue::FalloutDialogueRecord {
            plugin: "Fixture.esm".into(),
            form_id: 0x102,
            signature: "XXXX".into(),
            topic_key: "Greeting".into(),
            ..Default::default()
        },
    ]));
}

#[when("the Fallout dialogue inventory is rendered")]
async fn when_fallout_dialogue_inventory_is_rendered(_world: &mut BevyoutWorld) {}

#[then("the inventory reports one topic and one unsupported record")]
async fn then_inventory_reports_topic_and_unsupported(world: &mut BevyoutWorld) {
    let report = world.dialogue_inventory.as_ref().unwrap();
    assert_eq!(report.dial_total, 1);
    assert_eq!(report.info_total, 1);
    assert_eq!(report.unsupported.len(), 1);
    assert!(report.topic_links.contains_key("Greeting"));
}

fn synthetic_fallout_records() -> Vec<dialogue::FalloutDialogueRecord> {
    vec![
        dialogue::FalloutDialogueRecord {
            plugin: "Fixture.esm".into(),
            form_id: 0x200,
            signature: "INFO".into(),
            speaker_form_id: Some(0x300),
            text: Some("[choice] Open the gate".into()),
            topic_key: "Gate".into(),
            actions: vec!["open_gate".into()],
            ..Default::default()
        },
        dialogue::FalloutDialogueRecord {
            plugin: "Fixture.esm".into(),
            form_id: 0x201,
            signature: "INFO".into(),
            speaker_form_id: Some(0x300),
            text: Some("The gate opens.".into()),
            topic_key: "Gate".into(),
            ..Default::default()
        },
    ]
}

#[given("a synthetic Fallout conversation")]
async fn given_synthetic_fallout_conversation(_world: &mut BevyoutWorld) {
    let _ = synthetic_fallout_records();
}

#[when("the Fallout conversation is generated twice")]
async fn when_fallout_conversation_generated_twice(world: &mut BevyoutWorld) {
    let records = synthetic_fallout_records();
    let first = dialogue::generate_fallout_conversation("Gate", &records).unwrap();
    let second = dialogue::generate_fallout_conversation("Gate", &records).unwrap();
    assert_eq!(first, second);
    world.dialogue_generated = Some(first);
}

#[then("the generated dialogue bytes and mappings match")]
async fn then_generated_dialogue_bytes_and_mappings_match(world: &mut BevyoutWorld) {
    let (source, mappings) = world.dialogue_generated.as_ref().unwrap();
    assert!(source.content.contains("bo_run_action"));
    assert_eq!(mappings.len(), 2);
}

#[given(regex = r##"^an explicit dialogue checkpoint at node \"([^\"]*)\"$"##)]
async fn given_explicit_dialogue_checkpoint(world: &mut BevyoutWorld, node: String) {
    use bevyout_core::dialogue::{
        ActiveDialogueCheckpoint, DialogueActionKey, DialogueKey, DialogueSnapshot,
    };
    world.dialogue_snapshot = DialogueSnapshot {
        schema_version: bevyout_core::dialogue::DIALOGUE_SNAPSHOT_SCHEMA_VERSION,
        bundle_hash: "fixture".into(),
        variables: Default::default(),
        active: Some(ActiveDialogueCheckpoint {
            dialogue: DialogueKey::new("Start"),
            node,
            speaker: None,
            listener: None,
            completed_actions: vec![DialogueActionKey::new("bo_run_action open_gate")],
        }),
    };
}

#[when("the checkpoint snapshot is inspected")]
async fn when_checkpoint_snapshot_is_inspected(_world: &mut BevyoutWorld) {}

#[then("the checkpoint contains no Yarn VM state")]
async fn then_checkpoint_contains_no_yarn_vm_state(world: &mut BevyoutWorld) {
    let checkpoint = world.dialogue_snapshot.active.as_ref().unwrap();
    assert_eq!(checkpoint.dialogue.as_str(), "Start");
    assert_eq!(checkpoint.node, "Checkpoint");
    assert_eq!(world.dialogue_snapshot.schema_version, 1);
}

#[given(regex = r##"^a dialogue presentation policy for language \"([^\"]*)\"$"##)]
async fn given_dialogue_presentation_policy(world: &mut BevyoutWorld, language: String) {
    world.dialogue_policy = bevyout_core::dialogue::DialoguePresentationPolicy {
        language,
        subtitles_enabled: true,
        typewriter_enabled: true,
        skip_requires_second_press: true,
        accessible_choice_numbers: true,
    };
}

#[when("dialogue coverage is calculated for one line")]
async fn when_dialogue_coverage_is_calculated(world: &mut BevyoutWorld) {
    let key = bevyout_core::dialogue::DialogueLineKey::new("Start:0");
    world.dialogue_coverage = Some(bevyout_core::dialogue::DialogueCoverageReport {
        total_lines: 1,
        localized_lines: 0,
        voiced_lines: 0,
        missing_localization: vec![key.clone()],
        missing_voice: vec![key],
        unsupported_records: 0,
    });
}

#[then("the dialogue timing and coverage are deterministic")]
async fn then_dialogue_timing_and_coverage_are_deterministic(world: &mut BevyoutWorld) {
    assert_eq!(
        world.dialogue_policy.reveal_duration_seconds("Hello", None),
        0.125
    );
    assert_eq!(
        world
            .dialogue_policy
            .auto_advance_duration_seconds("Hello", None),
        0.5
    );
    assert_eq!(
        world
            .dialogue_policy
            .auto_advance_duration_seconds("Hello", Some(1250)),
        1.25
    );
    let coverage = world.dialogue_coverage.as_ref().unwrap();
    assert_eq!(coverage.total_lines, 1);
    assert_eq!(coverage.missing_localization.len(), 1);
    assert_eq!(coverage.missing_voice.len(), 1);
}

#[given("a synthetic WAV voice manifest for the Start line")]
async fn given_synthetic_wav_voice_manifest(world: &mut BevyoutWorld) {
    world.dialogue_sources = vec![synthetic_dialogue_source()];
    world.dialogue_voice_index = None;
    world.dialogue_voice_fingerprints = None;
}

#[when("dialogue voice preparation is run twice")]
async fn when_dialogue_voice_preparation_is_run_twice(world: &mut BevyoutWorld) {
    let input = || dialogue::DialogueVoiceInput {
        manifest_path: "dialogue/voice/fixture.ron".into(),
        cell_form_id: None,
        entries: vec![dialogue::DialogueVoiceInputEntry {
            line_key: bevyout_core::dialogue::DialogueLineKey::new("Start:0"),
            source_path: "dialogue/voice/fixture.wav".into(),
            bytes: synthetic_wav_bytes(16_000, 8_000),
            source_fingerprint: None,
            source_origin: None,
            speaker_form_id: None,
            voice_type_form_id: None,
        }],
    };
    let roots = [
        std::env::temp_dir().join(format!(
            "bevyout-dialogue-voice-feature-a-{}",
            std::process::id()
        )),
        std::env::temp_dir().join(format!(
            "bevyout-dialogue-voice-feature-b-{}",
            std::process::id()
        )),
    ];
    let mut fingerprints = Vec::new();
    for root in &roots {
        let _ = std::fs::remove_dir_all(root);
        let output = dialogue::prepare_dialogue_bundle_with_voice(
            root,
            world.dialogue_sources.clone(),
            Some(input()),
        )
        .expect("synthetic voice bundle prepares");
        let bundle = output.bundle.expect("voice bundle reference");
        let path = root.join("dialogue/voice_index.ron");
        let index: bevyout_core::dialogue::PreparedDialogueVoiceIndex =
            ron::de::from_bytes(&std::fs::read(path).expect("voice index bytes"))
                .expect("voice index RON");
        world.dialogue_voice_index = Some(index);
        fingerprints.push(bundle.content_fingerprint);
        let _ = std::fs::remove_dir_all(root);
    }
    world.dialogue_voice_fingerprints = Some((fingerprints[0].clone(), fingerprints[1].clone()));
}

#[then("the prepared voice index contains one half-second entry")]
async fn then_prepared_voice_index_contains_half_second_entry(world: &mut BevyoutWorld) {
    let index = world.dialogue_voice_index.as_ref().expect("voice index");
    assert_eq!(index.entries.len(), 1);
    assert_eq!(index.entries[0].duration_millis, 500);
    let fingerprints = world
        .dialogue_voice_fingerprints
        .as_ref()
        .expect("voice fingerprints");
    assert_eq!(fingerprints.0, fingerprints.1);
}

fn find_placement<'a>(
    manifest: &'a PreparedSceneManifest,
    editor_id: &str,
) -> &'a PreparedPlacement {
    manifest
        .placements
        .iter()
        .find(|placement| placement.editor_id.as_deref() == Some(editor_id))
        .unwrap_or_else(|| panic!("no placement with editor id {editor_id:?} in fixture"))
}

// ---------------------------------------------------------------------
// coordinates.feature
// ---------------------------------------------------------------------

#[given(regex = r"^a Fallout position of (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) world units$")]
async fn given_position(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.position = [x, y, z];
}

#[given(regex = r"^a Fallout rotation of (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) radians$")]
async fn given_rotation(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.rotation = [x, y, z];
}

#[given(regex = r"^a Fallout scale of (-?[\d.]+)$")]
async fn given_scale(world: &mut BevyoutWorld, scale: f32) {
    world.scale = scale;
}

#[when("it is converted to a Bevy placement transform")]
async fn when_converted(world: &mut BevyoutWorld) {
    let scale = if world.scale == 0.0 { 1.0 } else { world.scale };
    let (translation, rotation_xyzw, scale_out) =
        placement_transform_parts(world.position, world.rotation, scale);
    world.translation = translation;
    world.rotation_xyzw = rotation_xyzw;
    world.scale_out = scale_out;
}

#[when("the Bevy translation is converted back to Fallout world units")]
async fn when_converted_back(world: &mut BevyoutWorld) {
    // `placement_transform_parts` maps Fallout (x, y, z) to Bevy
    // (x, z, -y) * FO3_SCALE; invert that mapping and the scale to recover
    // the original Fallout world units.
    let [bx, by, bz] = world.translation;
    let scale = paths::FO3_SCALE;
    world.recovered_position = [bx / scale, -bz / scale, by / scale];
}

#[then(regex = r"^the recovered position matches the original Fallout position$")]
async fn then_recovered_matches_original(world: &mut BevyoutWorld) {
    for (recovered, original) in world.recovered_position.iter().zip(world.position.iter()) {
        assert!(
            (recovered - original).abs() < 0.01,
            "recovered {:?} != original {:?}",
            world.recovered_position,
            world.position
        );
    }
}

#[then(regex = r"^the Bevy translation is (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) metres$")]
async fn then_translation_is(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    let expected = [x, y, z];
    for (actual, expected) in world.translation.iter().zip(expected.iter()) {
        assert!(
            (actual - expected).abs() < 1e-4,
            "translation {:?} != expected {:?}",
            world.translation,
            expected
        );
    }
}

#[then("the Bevy rotation is the identity quaternion")]
async fn then_rotation_is_identity(world: &mut BevyoutWorld) {
    let expected = [0.0, 0.0, 0.0, 1.0];
    for (actual, expected) in world.rotation_xyzw.iter().zip(expected.iter()) {
        assert!((actual - expected).abs() < 1e-4);
    }
}

#[then(regex = r"^the Bevy scale is (-?[\d.]+)$")]
async fn then_scale_is(world: &mut BevyoutWorld, scale: f32) {
    assert!((world.scale_out - scale).abs() < 1e-4);
}

#[given(regex = r#"^the cell selector text "([^"]*)"$"#)]
async fn given_cell_selector_text(world: &mut BevyoutWorld, text: String) {
    world.selector = Some(parse_cell_selector(&text).expect("selector should parse"));
}

#[then(regex = r"^it parses as the hexadecimal FormID 0x([0-9a-fA-F]+)$")]
async fn then_parses_as_form_id(world: &mut BevyoutWorld, hex: String) {
    let expected = u32::from_str_radix(&hex, 16).unwrap();
    assert_eq!(world.selector, Some(CellSelector::FormId(expected)));
}

#[then(regex = r#"^it parses as the editor ID "([^"]*)"$"#)]
async fn then_parses_as_editor_id(world: &mut BevyoutWorld, editor_id: String) {
    assert_eq!(world.selector, Some(CellSelector::EditorId(editor_id)));
}

#[given(regex = r#"^the raw asset path "(.*)"$"#)]
async fn given_raw_asset_path(world: &mut BevyoutWorld, raw: String) {
    world.normalized_path = Some(normalize_asset_path(&raw));
}

#[then(regex = r#"^the normalized asset path is "([^"]*)"$"#)]
async fn then_normalized_asset_path_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.normalized_path.as_deref(), Some(expected.as_str()));
}

// ---------------------------------------------------------------------
// manifest.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^the golden manifest fixture "([^"]*)"$"#)]
async fn given_golden_manifest_fixture(world: &mut BevyoutWorld, path: String) {
    let root = env!("CARGO_MANIFEST_DIR");
    let text = std::fs::read_to_string(std::path::Path::new(root).join(&path))
        .unwrap_or_else(|error| panic!("reading golden fixture {path:?}: {error}"));
    let manifest: PreparedSceneManifest = ron::de::from_str(&text).unwrap_or_else(|error| {
        panic!(
            "golden fixture {path:?} no longer parses as PreparedSceneManifest \
             (schema drift): {error}"
        )
    });
    world.manifest = Some(manifest);
}

#[then("it parses as a PreparedSceneManifest")]
async fn then_it_parses(world: &mut BevyoutWorld) {
    assert!(world.manifest.is_some());
}

#[then(regex = r"^the schema version is (\d+)$")]
async fn then_schema_version_is(world: &mut BevyoutWorld, version: u32) {
    assert_eq!(world.manifest.as_ref().unwrap().schema_version, version);
}

#[then(regex = r#"^the placement "([^"]*)" is a Container$"#)]
async fn then_placement_is_container(world: &mut BevyoutWorld, editor_id: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert_eq!(placement.semantic, PreparedSemantic::Container);
}

#[then(regex = r#"^the placement "([^"]*)" has (\d+) inventory entr(?:y|ies)$"#)]
async fn then_placement_inventory_count(world: &mut BevyoutWorld, editor_id: String, count: usize) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert_eq!(placement.inventory.len(), count);
}

#[then(regex = r#"^the placement "([^"]*)" inventory includes "([^"]*)"$"#)]
async fn then_placement_inventory_includes(
    world: &mut BevyoutWorld,
    editor_id: String,
    item_editor_id: String,
) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert!(
        placement
            .inventory
            .iter()
            .any(|entry| entry.editor_id.as_deref() == Some(item_editor_id.as_str())),
        "{editor_id} inventory does not include {item_editor_id}"
    );
}

#[then(regex = r#"^the placement "([^"]*)" has an owner faction rank of (-?\d+)$"#)]
async fn then_placement_owner_faction_rank(world: &mut BevyoutWorld, editor_id: String, rank: i32) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert_eq!(placement.owner_faction_rank, Some(rank));
}

#[then(regex = r#"^the placement "([^"]*)" has an enable parent that pops in$"#)]
async fn then_placement_enable_parent_pops_in(world: &mut BevyoutWorld, editor_id: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    let enable_parent = placement
        .enable_parent
        .as_ref()
        .unwrap_or_else(|| panic!("{editor_id} has no enable_parent"));
    assert!(enable_parent.pop_in);
}

#[then(regex = r#"^the placement "([^"]*)" is a Door$"#)]
async fn then_placement_is_door(world: &mut BevyoutWorld, editor_id: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    assert!(matches!(placement.semantic, PreparedSemantic::Door(_)));
}

#[then(regex = r#"^the placement "([^"]*)" has a lock level of (\d+)$"#)]
async fn then_placement_lock_level(world: &mut BevyoutWorld, editor_id: String, level: i8) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    let PreparedSemantic::Door(door) = &placement.semantic else {
        panic!("{editor_id} is not a Door");
    };
    assert_eq!(door.lock_level, Some(level));
}

#[then(regex = r#"^the placement "([^"]*)" destination cell is 0x([0-9a-fA-F]+)$"#)]
async fn then_placement_destination_cell(world: &mut BevyoutWorld, editor_id: String, hex: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let placement = find_placement(manifest, &editor_id);
    let PreparedSemantic::Door(door) = &placement.semantic else {
        panic!("{editor_id} is not a Door");
    };
    let expected = u32::from_str_radix(&hex, 16).unwrap();
    let destination = door
        .destination
        .as_ref()
        .unwrap_or_else(|| panic!("{editor_id} door has no destination"));
    assert_eq!(destination.cell_form_id, expected);
}

#[then(regex = r"^the cell acoustic environment type is (\d+)$")]
async fn then_cell_acoustic_environment_type(world: &mut BevyoutWorld, expected: u32) {
    let manifest = world.manifest.as_ref().unwrap();
    assert_eq!(
        manifest.cell_audio.acoustic_environment_type,
        Some(expected)
    );
}

#[then(regex = r#"^the footstep set "([^"]*)" has a land clip$"#)]
async fn then_footstep_set_has_land_clip(world: &mut BevyoutWorld, surface: String) {
    let manifest = world.manifest.as_ref().unwrap();
    let set = manifest
        .footstep_sets
        .iter()
        .find(|set| set.surface == surface)
        .unwrap_or_else(|| panic!("no footstep set for surface {surface:?}"));
    assert!(!set.land.is_empty());
}

#[then(regex = r"^there is (\d+) hard landing clips?$")]
async fn then_hard_landing_clip_count(world: &mut BevyoutWorld, count: usize) {
    let manifest = world.manifest.as_ref().unwrap();
    assert_eq!(manifest.hard_landing_clips.len(), count);
}

#[then(regex = r#"^there is a retained NAVM payload with signature "([^"]*)"$"#)]
async fn then_navm_payload_signature(world: &mut BevyoutWorld, signature: String) {
    let manifest = world.manifest.as_ref().unwrap();
    assert!(
        manifest
            .navmeshes
            .iter()
            .flat_map(|navmesh| &navmesh.chunks)
            .any(|chunk| chunk.signature == signature),
        "no NAVM chunk with signature {signature:?}"
    );
}

// ---------------------------------------------------------------------
// asset_materials.feature
// ---------------------------------------------------------------------

#[given("an asset is static")]
async fn given_asset_is_static(world: &mut BevyoutWorld) {
    world.is_static = true;
}

#[given("an asset is dynamic")]
async fn given_asset_is_dynamic(world: &mut BevyoutWorld) {
    world.is_static = false;
}

#[when("its conversion profile is selected")]
async fn when_conversion_profile_selected(world: &mut BevyoutWorld) {
    world.conversion = Some(assets::asset_conversion(world.is_static));
}

#[then(regex = r"^the conversion is (QuickAo|Preserve)$")]
async fn then_conversion_is(world: &mut BevyoutWorld, expected: String) {
    let expected = match expected.as_str() {
        "QuickAo" => AssetConversion::QuickAo,
        "Preserve" => AssetConversion::Preserve,
        other => panic!("unknown conversion {other:?}"),
    };
    assert_eq!(world.conversion, Some(expected));
}

#[then(regex = r#"^the profile tag is "([^"]*)"$"#)]
async fn then_profile_tag_is(world: &mut BevyoutWorld, expected: String) {
    let conversion = world
        .conversion
        .expect("conversion profile not selected yet");
    assert_eq!(conversion.profile_tag(), expected);
}

#[given(
    regex = r#"^an imported material has NIFTools emissive color \(([\d.]+), ([\d.]+), ([\d.]+)\)$"#
)]
async fn given_niftools_emissive_color(
    world: &mut BevyoutWorld,
    red: String,
    green: String,
    blue: String,
) {
    world.authored_emission = Some([
        red.parse().unwrap(),
        green.parse().unwrap(),
        blue.parse().unwrap(),
    ]);
}

#[given(regex = r"^the source emission multiplier is ([\d.]+)$")]
async fn given_source_emission_multiplier(world: &mut BevyoutWorld, strength: String) {
    world.source_emission_strength = Some(strength.parse().unwrap());
}

#[given("an explicit emission is present")]
async fn given_explicit_emission_present(world: &mut BevyoutWorld) {
    world.explicit_emission = true;
}

#[given("an emissive bulb override is present")]
async fn given_emissive_bulb_override_present(world: &mut BevyoutWorld) {
    world.bulb_emission_override = true;
}

#[given("a glow texture override is present")]
async fn given_glow_texture_override_present(world: &mut BevyoutWorld) {
    world.glow_emission_override = true;
}

#[when("its material emission policy is evaluated")]
async fn when_material_emission_policy_evaluated(world: &mut BevyoutWorld) {
    let color = world
        .authored_emission
        .expect("authored emission color was not provided");
    world.emission_policy = Some(assets::material_emission_policy(
        color,
        world.source_emission_strength.unwrap_or(1.0),
        world.explicit_emission,
        world.bulb_emission_override,
        world.glow_emission_override,
    ));
}

#[then(regex = r#"^the exported emission color is \(([\d.]+), ([\d.]+), ([\d.]+)\)$"#)]
async fn then_exported_emission_color(
    world: &mut BevyoutWorld,
    red: String,
    green: String,
    blue: String,
) {
    let expected = [
        red.parse::<f32>().unwrap(),
        green.parse::<f32>().unwrap(),
        blue.parse::<f32>().unwrap(),
    ];
    match world.emission_policy {
        Some(assets::MaterialEmissionPolicy::Authored(emission)) => {
            assert_eq!(emission.color, expected);
        }
        other => panic!("expected authored emission, got {other:?}"),
    }
}

#[then(regex = r"^the exported emission strength is ([\d.]+)$")]
async fn then_exported_emission_strength(world: &mut BevyoutWorld, expected: String) {
    let expected = expected.parse::<f32>().unwrap();
    match world.emission_policy {
        Some(assets::MaterialEmissionPolicy::Authored(emission)) => {
            assert_eq!(emission.strength, expected);
        }
        other => panic!("expected authored emission, got {other:?}"),
    }
}

#[then("the exported material has no emission")]
async fn then_exported_material_has_no_emission(world: &mut BevyoutWorld) {
    assert_eq!(
        world.emission_policy,
        Some(assets::MaterialEmissionPolicy::None)
    );
}

#[then(regex = r"^the selected emission source is (Authored|Explicit|Bulb|Glow|None)$")]
async fn then_selected_emission_source(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .emission_policy
        .expect("emission policy was not evaluated");
    let matches = match expected.as_str() {
        "Authored" => matches!(actual, assets::MaterialEmissionPolicy::Authored(_)),
        "Explicit" => actual == assets::MaterialEmissionPolicy::Explicit,
        "Bulb" => actual == assets::MaterialEmissionPolicy::Bulb,
        "Glow" => actual == assets::MaterialEmissionPolicy::Glow,
        "None" => actual == assets::MaterialEmissionPolicy::None,
        other => panic!("unknown emission source {other}"),
    };
    assert!(matches, "expected {expected}, got {actual:?}");
}

#[given(regex = r"^a NIF material glossiness exponent ([\d.]+)$")]
async fn given_material_glossiness_exponent(world: &mut BevyoutWorld, value: String) {
    world.material_glossiness = Some(Some(value.parse().unwrap()));
}

#[given(regex = r#"^a NIF material glossiness value "([^"]+)"$"#)]
async fn given_material_glossiness_value(world: &mut BevyoutWorld, value: String) {
    world.material_glossiness = Some(match value.as_str() {
        "missing" => None,
        "negative" => Some(-1.0),
        "nan" => Some(f32::NAN),
        "infinite" => Some(f32::INFINITY),
        other => panic!("unknown glossiness fixture {other}"),
    });
}

#[given(regex = r#"(?s)^metallic material CSV "(.*)"$"#)]
async fn given_metallic_material_csv(world: &mut BevyoutWorld, csv: String) {
    world.metallic_csv = Some(csv.replace("\\n", "\n"));
}

#[given(regex = r#"^a material diffuse texture "([^"]+)"$"#)]
async fn given_material_diffuse_texture(world: &mut BevyoutWorld, path: String) {
    world.material_diffuse_texture = Some(path);
}

#[given("a Fallout material has specular enabled")]
async fn given_fallout_material_has_specular_enabled(world: &mut BevyoutWorld) {
    world.material_specular_enabled = true;
}

#[given("a Fallout material has specular disabled")]
async fn given_fallout_material_has_specular_disabled(world: &mut BevyoutWorld) {
    world.material_specular_enabled = false;
}

#[given(regex = r#"^its normal texture is "([^"]+)"$"#)]
async fn given_material_normal_texture(world: &mut BevyoutWorld, path: String) {
    world.material_normal_texture = Some(path);
}

#[when("the metallic material CSV is parsed")]
async fn when_metallic_material_csv_is_parsed(world: &mut BevyoutWorld) {
    world.metallic_csv_rejected =
        assets::MetallicMaterialTable::parse(world.metallic_csv.as_deref().unwrap()).is_err();
}

#[when("its PBR material policy is evaluated")]
async fn when_pbr_material_policy_is_evaluated(world: &mut BevyoutWorld) {
    world.material_roughness = Some(assets::perceptual_roughness_from_glossiness(
        world.material_glossiness.flatten(),
    ));
    if let Some(csv) = &world.metallic_csv {
        let table = assets::MetallicMaterialTable::parse(csv).unwrap();
        world.material_metallic =
            Some(table.metallic_factor(world.material_diffuse_texture.as_deref()));
    }
    world.material_specular_texture = assets::fallout_specular_texture_path(
        world.material_specular_enabled,
        world.material_normal_texture.as_deref(),
    );
}

#[then(regex = r"^its perceptual roughness is approximately ([\d.]+)$")]
async fn then_perceptual_roughness_is(world: &mut BevyoutWorld, expected: String) {
    let expected = expected.parse::<f32>().unwrap();
    let actual = world.material_roughness.unwrap();
    assert!(
        (actual - expected).abs() < 0.000_01,
        "{actual} != {expected}"
    );
}

#[then(regex = r"^its metallic factor is ([01])$")]
async fn then_metallic_factor_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.material_metallic, Some(expected.parse().unwrap()));
}

#[then("the metallic material CSV is rejected")]
async fn then_metallic_material_csv_is_rejected(world: &mut BevyoutWorld) {
    assert!(world.metallic_csv_rejected);
}

#[then(regex = r#"^its specular texture is "([^"]+)"$"#)]
async fn then_material_specular_texture_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.material_specular_texture.as_deref(),
        Some(expected.as_str())
    );
}

#[then("it has no specular texture")]
async fn then_material_has_no_specular_texture(world: &mut BevyoutWorld) {
    assert_eq!(world.material_specular_texture, None);
}

#[given(regex = r"^a DirectX normal texel \((\d+), (\d+), (\d+), (\d+)\)$")]
async fn given_directx_normal_texel(
    world: &mut BevyoutWorld,
    red: String,
    green: String,
    blue: String,
    alpha: String,
) {
    world.directx_normal_texel = Some([
        red.parse().unwrap(),
        green.parse().unwrap(),
        blue.parse().unwrap(),
        alpha.parse().unwrap(),
    ]);
}

#[when("its normal convention is converted for Bevy")]
async fn when_normal_convention_is_converted(world: &mut BevyoutWorld) {
    let mut texel = world
        .directx_normal_texel
        .expect("DirectX normal texel was not provided");
    assets::flip_directx_normal_y_texel(&mut texel);
    world.converted_normal_texel = Some(texel);
}

#[then(regex = r"^the converted normal texel is \((\d+), (\d+), (\d+), (\d+)\)$")]
async fn then_converted_normal_texel_is(
    world: &mut BevyoutWorld,
    red: String,
    green: String,
    blue: String,
    alpha: String,
) {
    assert_eq!(
        world.converted_normal_texel,
        Some([
            red.parse().unwrap(),
            green.parse().unwrap(),
            blue.parse().unwrap(),
            alpha.parse().unwrap(),
        ])
    );
}

#[given(regex = r#"^the staged texture path "([^"]+)"$"#)]
async fn given_staged_texture_path(world: &mut BevyoutWorld, path: String) {
    world.staged_texture_path = Some(path);
}

#[when("its Blender texture role is classified")]
async fn when_blender_texture_role_is_classified(world: &mut BevyoutWorld) {
    let path = world
        .staged_texture_path
        .as_deref()
        .expect("staged texture path was not provided");
    world.staged_texture_is_normal = Some(assets::is_blender_normal_texture_path(path));
}

#[then(regex = r"^it (is|is not) converted as a normal map$")]
async fn then_normal_map_conversion_classification(
    world: &mut BevyoutWorld,
    classification: String,
) {
    assert_eq!(world.staged_texture_is_normal, Some(classification == "is"));
}

// ---------------------------------------------------------------------
// cell_map.feature
// ---------------------------------------------------------------------

fn parse_hex(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16)
        .unwrap_or_else(|error| panic!("invalid hex FormID {hex:?}: {error}"))
}

fn parse_item_instance_id(hex: &str) -> ItemInstanceId {
    let digits = hex.strip_prefix("0x").unwrap_or(hex);
    ItemInstanceId(
        u64::from_str_radix(digits, 16)
            .unwrap_or_else(|error| panic!("invalid item instance id {hex:?}: {error}")),
    )
}

fn assert_hotkey_slot(slot: usize) {
    assert!(slot < 8, "hotkey slot {slot} must be in the range 0..7");
}

#[given(regex = r#"^a cell map with an interior cell "([^"]*)" 0x([0-9a-fA-F]+) and no grid$"#)]
async fn given_interior_cell(world: &mut BevyoutWorld, editor_id: String, hex: String) {
    world.cell_map_cells.push(cell_map::CellMapEntry {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        interior: true,
        worldspace_form_id: None,
        grid: None,
    });
}

#[given(
    regex = r#"^the cell map has an exterior cell "([^"]*)" 0x([0-9a-fA-F]+) with grid (-?\d+), (-?\d+) in worldspace 0x([0-9a-fA-F]+)$"#
)]
async fn given_exterior_cell(
    world: &mut BevyoutWorld,
    editor_id: String,
    hex: String,
    x: i32,
    y: i32,
    worldspace_hex: String,
) {
    let worldspace_form_id = parse_hex(&worldspace_hex);
    world.cell_map_cells.push(cell_map::CellMapEntry {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        interior: false,
        worldspace_form_id: Some(worldspace_form_id),
        grid: Some((x, y)),
    });
    if !world
        .cell_map_worldspaces
        .iter()
        .any(|worldspace| worldspace.form_id == worldspace_form_id)
    {
        world.cell_map_worldspaces.push(cell_map::WorldspaceEntry {
            form_id: worldspace_form_id,
            editor_id: None,
            name: None,
        });
    }
}

#[given(
    regex = r"^a door edge from cell 0x([0-9a-fA-F]+) door 0x([0-9a-fA-F]+) to cell 0x([0-9a-fA-F]+) door 0x([0-9a-fA-F]+)$"
)]
async fn given_door_edge(
    world: &mut BevyoutWorld,
    source_cell: String,
    source_door: String,
    destination_cell: String,
    destination_door: String,
) {
    world.cell_map_doors.push(cell_map::DoorEdge {
        source_cell_form_id: parse_hex(&source_cell),
        door_reference_form_id: parse_hex(&source_door),
        destination_cell_form_id: parse_hex(&destination_cell),
        destination_door_reference_form_id: parse_hex(&destination_door),
        position: [1.0, 2.0, 3.0],
        rotation: [0.0, 0.0, 0.0],
    });
}

#[given(regex = r"^(\d+) unresolved door teleports?$")]
async fn given_unresolved_teleports(world: &mut BevyoutWorld, count: u32) {
    world.cell_map_unresolved += count;
}

fn build_cell_map(world: &BevyoutWorld) -> cell_map::CellMap {
    cell_map::CellMap::build(
        "fingerprint".into(),
        world.cell_map_worldspaces.clone(),
        world.cell_map_cells.clone(),
        world.cell_map_doors.clone(),
        world.cell_map_unresolved,
    )
}

#[when("the cell map is built")]
async fn when_cell_map_built(world: &mut BevyoutWorld) {
    world.cell_map = Some(build_cell_map(world));
}

#[when("the cell map is built twice from the same input")]
async fn when_cell_map_built_twice(world: &mut BevyoutWorld) {
    world.cell_map_ron_a = Some(build_cell_map(world).to_ron().unwrap());
    world.cell_map_ron_b = Some(build_cell_map(world).to_ron().unwrap());
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) has no worldspace$")]
async fn then_cell_has_no_worldspace(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let map = world.cell_map.as_ref().expect("cell map not built yet");
    let cell = map
        .cells
        .iter()
        .find(|cell| cell.form_id == form_id)
        .unwrap_or_else(|| panic!("no cell {form_id:08x} in built map"));
    assert_eq!(cell.worldspace_form_id, None);
}

#[then(
    regex = r"^cell 0x([0-9a-fA-F]+) is in worldspace 0x([0-9a-fA-F]+) with grid (-?\d+), (-?\d+)$"
)]
async fn then_cell_in_worldspace(
    world: &mut BevyoutWorld,
    hex: String,
    worldspace_hex: String,
    x: i32,
    y: i32,
) {
    let form_id = parse_hex(&hex);
    let worldspace_form_id = parse_hex(&worldspace_hex);
    let map = world.cell_map.as_ref().expect("cell map not built yet");
    let cell = map
        .cells
        .iter()
        .find(|cell| cell.form_id == form_id)
        .unwrap_or_else(|| panic!("no cell {form_id:08x} in built map"));
    assert_eq!(cell.worldspace_form_id, Some(worldspace_form_id));
    assert_eq!(cell.grid, Some((x, y)));
}

#[then(regex = r"^there (?:is|are) (\d+) door edges?$")]
async fn then_door_edge_count(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(
        world
            .cell_map
            .as_ref()
            .expect("cell map not built yet")
            .doors
            .len(),
        count
    );
}

#[then(regex = r"^there (?:is|are) (\d+) unresolved doors?$")]
async fn then_unresolved_door_count(world: &mut BevyoutWorld, count: u32) {
    assert_eq!(
        world
            .cell_map
            .as_ref()
            .expect("cell map not built yet")
            .unresolved_door_count,
        count
    );
}

#[then("both RON outputs are byte-identical")]
async fn then_ron_outputs_are_byte_identical(world: &mut BevyoutWorld) {
    assert_eq!(world.cell_map_ron_a, world.cell_map_ron_b);
}

// ---------------------------------------------------------------------
// prepare_selectors.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an interior cell$"#)]
async fn given_selector_interior_cell(world: &mut BevyoutWorld, hex: String, editor_id: String) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: true,
        worldspace_form_id: None,
        grid: None,
    });
}

#[given(regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an exterior cell$"#)]
async fn given_selector_exterior_cell(world: &mut BevyoutWorld, hex: String, editor_id: String) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: false,
        worldspace_form_id: None,
        grid: None,
    });
}

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an exterior cell in worldspace 0x([0-9a-fA-F]+)$"#
)]
async fn given_exterior_cell_in_worldspace(
    world: &mut BevyoutWorld,
    hex: String,
    editor_id: String,
    worldspace_hex: String,
) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: false,
        worldspace_form_id: Some(parse_hex(&worldspace_hex)),
        grid: None,
    });
}

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) "([^"]*)" is an exterior cell in worldspace 0x([0-9a-fA-F]+) at grid \((-?\d+),(-?\d+)\)$"#
)]
async fn given_exterior_cell_in_worldspace_at_grid(
    world: &mut BevyoutWorld,
    hex: String,
    editor_id: String,
    worldspace_hex: String,
    grid_x: i32,
    grid_y: i32,
) {
    world.cells.push(CellSummary {
        form_id: parse_hex(&hex),
        editor_id: Some(editor_id),
        name: None,
        interior: false,
        worldspace_form_id: Some(parse_hex(&worldspace_hex)),
        grid: Some((grid_x, grid_y)),
    });
}

#[given(regex = r#"^worldspace 0x([0-9a-fA-F]+) is named "([^"]*)"$"#)]
async fn given_worldspace_named(world: &mut BevyoutWorld, hex: String, name: String) {
    world.worldspace_names.push((parse_hex(&hex), name));
}

#[when("cells are selected with --all-interiors")]
async fn when_selected_all_interiors(world: &mut BevyoutWorld) {
    let spec = SelectionSpec {
        all_interiors: true,
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[when("cells are selected with --all-exteriors")]
async fn when_selected_all_exteriors(world: &mut BevyoutWorld) {
    let spec = SelectionSpec {
        all_exteriors: true,
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[when(regex = r#"^cells are selected with selectors "([^"]*)"$"#)]
async fn when_selected_explicit(world: &mut BevyoutWorld, list: String) {
    let explicit = list
        .split(',')
        .map(|entry| entry.trim().to_string())
        .collect();
    let spec = SelectionSpec {
        explicit,
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[when(regex = r#"^cells are selected with worldspace "([^"]*)"$"#)]
async fn when_selected_worldspace(world: &mut BevyoutWorld, name: String) {
    let spec = SelectionSpec {
        worldspace: Some(name),
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[when(regex = r#"^cells are selected with anchor "([^"]*)" and --exterior-radius (\d+)$"#)]
async fn when_selected_exterior_radius(world: &mut BevyoutWorld, anchor: String, radius: u32) {
    let spec = SelectionSpec {
        exterior_radius: Some(radius),
        explicit: vec![anchor],
        ..Default::default()
    };
    world.selection_result = Some(
        resolve_selection(&world.cells, &world.worldspace_names, &spec)
            .map_err(|error| error.to_string()),
    );
}

#[then(regex = r#"^the resolved cell selection is "([^"]*)"$"#)]
async fn then_resolved_selection_is(world: &mut BevyoutWorld, list: String) {
    let expected: Vec<u32> = list
        .split(',')
        .map(|entry| parse_hex(entry.trim()))
        .collect();
    let resolved = world
        .selection_result
        .take()
        .expect("a selection was not resolved")
        .expect("selection resolution failed");
    assert_eq!(resolved, expected);
}

#[then(regex = r#"^the cell selection fails naming worldspace "([^"]*)"$"#)]
async fn then_selection_fails_naming_worldspace(world: &mut BevyoutWorld, name: String) {
    let error = world
        .selection_result
        .take()
        .expect("a selection was not attempted")
        .expect_err("expected selection resolution to fail");
    assert!(
        error.contains(&name),
        "error {error:?} does not mention worldspace {name:?}"
    );
}

// ---------------------------------------------------------------------
// batch_session.feature
// ---------------------------------------------------------------------

#[given("a fresh batch cache")]
async fn given_fresh_batch_cache(world: &mut BevyoutWorld) {
    world.batch_physics_cache = batch_cache::KeyedBatchCache::default();
    world.batch_asset_totals = batch_cache::BatchAssetTotals::default();
}

#[when(regex = r#"^cell "[^"]*" reads physics key "([^"]*)"$"#)]
async fn when_cell_reads_physics_key(world: &mut BevyoutWorld, key: String) {
    world
        .batch_physics_cache
        .get_or_insert_with(&key, || Ok(()))
        .expect("synthetic build never fails");
}

#[when(
    regex = r#"^cell "[^"]*" reports asset cache reused (\d+), built (\d+), invalid (\d+), explicit (\d+)$"#
)]
async fn when_cell_reports_asset_cache(
    world: &mut BevyoutWorld,
    reused: usize,
    built: usize,
    invalid: usize,
    explicit: usize,
) {
    world
        .batch_asset_totals
        .add(reused, built, invalid, explicit);
}

#[then(regex = r"^physics reads is (\d+)$")]
async fn then_physics_reads_is(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.batch_physics_cache.accesses(), count);
}

#[then(regex = r"^physics hits is (\d+)$")]
async fn then_physics_hits_is(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.batch_physics_cache.hits, count);
}

#[then(regex = r#"^the batch cache summary line is "([^"]*)"$"#)]
async fn then_batch_cache_summary_line_is(world: &mut BevyoutWorld, expected: String) {
    let line = batch_cache::batch_cache_summary_line(
        world.batch_asset_totals,
        world.batch_physics_cache.accesses(),
        world.batch_physics_cache.hits,
    );
    assert_eq!(line, expected);
}

#[when("the cell map is written to the batch cache dir")]
async fn when_cell_map_written_to_batch_cache_dir(world: &mut BevyoutWorld) {
    let map = build_cell_map(world);
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "bevyout-cucumber-batch-cache-{}-{unique}",
        std::process::id()
    ));
    let path = batch_cache::write_cell_map(&dir, &map).expect("writing the cell map must succeed");
    world.batch_cache_dir = Some(dir);
    world.written_cell_map_path = Some(path);
    // Reuses `cell_map.feature`'s own `Then there are N door edges` step,
    // which reads from `world.cell_map`.
    world.cell_map = Some(map.clone());
    world.written_cell_map = Some(map);
}

#[then(regex = r#"^the written cell map file exists at "([^"]*)" under the batch cache dir$"#)]
async fn then_written_cell_map_file_exists(world: &mut BevyoutWorld, file_name: String) {
    let path = world
        .written_cell_map_path
        .as_ref()
        .expect("cell map was not written yet");
    let dir = world
        .batch_cache_dir
        .as_ref()
        .expect("cell map was not written yet");
    assert_eq!(path, &dir.join(&file_name));
    assert!(path.is_file(), "{path:?} was not written");
    std::fs::remove_dir_all(dir).ok();
}

#[then(regex = r"^the written cell map has (\d+) cells$")]
async fn then_written_cell_map_has_cells(world: &mut BevyoutWorld, count: usize) {
    let map = world
        .written_cell_map
        .as_ref()
        .expect("cell map was not written yet");
    assert_eq!(map.cells.len(), count);
}

// ---------------------------------------------------------------------
// preload_policy.feature (issue #51)
// ---------------------------------------------------------------------

#[given(regex = r"^a door edge from cell 0x([0-9a-fA-F]+) to cell 0x([0-9a-fA-F]+)$")]
async fn given_preload_door_edge(world: &mut BevyoutWorld, source: String, destination: String) {
    world.preload_doors.push(policy::DoorLink {
        source_cell_form_id: parse_hex(&source),
        destination_cell_form_id: parse_hex(&destination),
    });
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is prepared$")]
async fn given_preload_cell_prepared(world: &mut BevyoutWorld, hex: String) {
    world.preload_prepared.insert(parse_hex(&hex));
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is resident$")]
async fn given_preload_cell_resident(world: &mut BevyoutWorld, hex: String) {
    world.preload_resident.push(parse_hex(&hex));
}

#[given(regex = r"^the active cell is 0x([0-9a-fA-F]+)$")]
async fn given_preload_active_cell(world: &mut BevyoutWorld, hex: String) {
    world.preload_active_cell = parse_hex(&hex);
}

#[given(regex = r"^the resident cell limit is (\d+)$")]
async fn given_preload_resident_cell_limit(world: &mut BevyoutWorld, budget: usize) {
    world.preload_budget = budget;
}

#[when("the preload plan is computed")]
async fn when_preload_plan_computed(world: &mut BevyoutWorld) {
    let graph = policy::CellGraph::build(&world.preload_doors);
    world.preload_plan = Some(graph.plan(
        world.preload_active_cell,
        &world.preload_resident,
        &world.preload_prepared,
        world.preload_budget,
    ));
}

#[then(regex = r"^the plan loads cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_loads_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.load.contains(&form_id),
        "expected plan to load {form_id:08x}, load = {:?}",
        plan.load
    );
}

#[then(regex = r"^the plan does not load cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_does_not_load_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        !plan.load.contains(&form_id),
        "expected plan not to load {form_id:08x}, load = {:?}",
        plan.load
    );
}

#[then(regex = r"^the plan evicts cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_evicts_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.evict.contains(&form_id),
        "expected plan to evict {form_id:08x}, evict = {:?}",
        plan.evict
    );
}

#[then(regex = r"^the plan does not evict cell 0x([0-9a-fA-F]+)$")]
async fn then_plan_does_not_evict_cell(world: &mut BevyoutWorld, hex: String) {
    let form_id = parse_hex(&hex);
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        !plan.evict.contains(&form_id),
        "expected plan not to evict {form_id:08x}, evict = {:?}",
        plan.evict
    );
}

#[then("the plan loads nothing")]
async fn then_plan_loads_nothing(world: &mut BevyoutWorld) {
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.load.is_empty(),
        "expected no loads, got {:?}",
        plan.load
    );
}

#[then("the plan evicts nothing")]
async fn then_plan_evicts_nothing(world: &mut BevyoutWorld) {
    let plan = world
        .preload_plan
        .as_ref()
        .expect("preload plan not computed yet");
    assert!(
        plan.evict.is_empty(),
        "expected no evictions, got {:?}",
        plan.evict
    );
}

// ---------------------------------------------------------------------
// resumable_prepare.feature (issue #48)
// ---------------------------------------------------------------------

fn job_manifest_temp_path() -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir()
        .join(format!(
            "bevyout-cucumber-jobs-{}-{unique}",
            std::process::id()
        ))
        .join("prepare_jobs.ron")
}

fn parse_hex_list(list: &str) -> Vec<u32> {
    list.split(',')
        .map(|entry| parse_hex(entry.trim()))
        .collect()
}

#[given(regex = r#"^a fresh job manifest with fingerprint "([^"]*)"$"#)]
async fn given_fresh_job_manifest(world: &mut BevyoutWorld, fingerprint: String) {
    world.job_manifest = Some(jobs::JobManifest::new(fingerprint));
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is marked done$")]
async fn given_job_cell_done(world: &mut BevyoutWorld, hex: String) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .set_status(parse_hex(&hex), jobs::JobStatus::Done);
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) is marked pending$")]
async fn given_job_cell_pending(world: &mut BevyoutWorld, hex: String) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .set_status(parse_hex(&hex), jobs::JobStatus::Pending);
}

#[given(regex = r#"^cell 0x([0-9a-fA-F]+) is marked failed with reason "([^"]*)"$"#)]
async fn given_job_cell_failed(world: &mut BevyoutWorld, hex: String, reason: String) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .set_status(parse_hex(&hex), jobs::JobStatus::Failed(reason));
}

#[given("the manifest is written and reloaded")]
async fn given_job_manifest_written_and_reloaded(world: &mut BevyoutWorld) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let path = job_manifest_temp_path();
    manifest
        .write_atomic(&path)
        .expect("writing the job manifest must succeed");
    let reloaded = jobs::JobManifest::load_or_new(&path, &manifest.content_fingerprint)
        .expect("reloading the job manifest must succeed");
    world.job_manifest_path = Some(path);
    world.job_manifest = Some(reloaded);
}

#[given("the manifest is written to disk")]
async fn given_job_manifest_written_to_disk(world: &mut BevyoutWorld) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let path = job_manifest_temp_path();
    manifest
        .write_atomic(&path)
        .expect("writing the job manifest must succeed");
    world.job_manifest_path = Some(path);
}

#[when(regex = r#"^the manifest is reloaded with fingerprint "([^"]*)"$"#)]
async fn when_job_manifest_reloaded_with_fingerprint(
    world: &mut BevyoutWorld,
    fingerprint: String,
) {
    let path = world
        .job_manifest_path
        .as_ref()
        .expect("job manifest was not written yet");
    world.job_manifest = Some(
        jobs::JobManifest::load_or_new(path, &fingerprint)
            .expect("reloading the job manifest must succeed"),
    );
}

#[when(regex = r#"^cells "([^"]*)" are resumed without force$"#)]
async fn when_job_cells_resumed_without_force(world: &mut BevyoutWorld, list: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.job_resume_result = Some(jobs::filter_resume(manifest, &selection, false));
}

#[when(regex = r#"^cells "([^"]*)" are resumed with force$"#)]
async fn when_job_cells_resumed_with_force(world: &mut BevyoutWorld, list: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.job_resume_result = Some(jobs::filter_resume(manifest, &selection, true));
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) has status done$")]
async fn then_job_cell_status_done(world: &mut BevyoutWorld, hex: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert_eq!(
        manifest.status(parse_hex(&hex)),
        Some(&jobs::JobStatus::Done)
    );
}

#[then(regex = r#"^cell 0x([0-9a-fA-F]+) has status failed with reason "([^"]*)"$"#)]
async fn then_job_cell_status_failed(world: &mut BevyoutWorld, hex: String, reason: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert_eq!(
        manifest.status(parse_hex(&hex)),
        Some(&jobs::JobStatus::Failed(reason))
    );
}

#[then(regex = r#"^the cells to run are "([^"]*)"$"#)]
async fn then_job_cells_to_run_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (to_run, _) = world
        .job_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(to_run, &expected);
}

#[then(regex = r"^(\d+) cell\(s\) were skipped$")]
async fn then_job_cells_skipped(world: &mut BevyoutWorld, count: usize) {
    let (_, skipped) = world
        .job_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(*skipped, count);
}

#[then(regex = r#"^the failed cells are "([^"]*)"$"#)]
async fn then_job_failed_cells_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert_eq!(manifest.failed_form_ids(), expected);
}

#[then("the manifest has no recorded cells")]
async fn then_job_manifest_has_no_recorded_cells(world: &mut BevyoutWorld) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    assert!(manifest.jobs.is_empty());
}

// ---------------------------------------------------------------------
// instant_swap.feature (issue #52)
// ---------------------------------------------------------------------

#[given(regex = r"^the destination cell residency is (Ready|Loading|Absent)$")]
async fn given_swap_residency(world: &mut BevyoutWorld, residency: String) {
    world.swap_residency = Some(match residency.as_str() {
        "Ready" => swap_policy::Residency::Ready,
        "Loading" => swap_policy::Residency::Loading,
        _ => swap_policy::Residency::Absent,
    });
}

#[given("the destination manifest exists on disk")]
async fn given_swap_manifest_exists(world: &mut BevyoutWorld) {
    world.swap_manifest_exists = true;
}

#[given("the destination manifest does not exist on disk")]
async fn given_swap_manifest_does_not_exist(world: &mut BevyoutWorld) {
    world.swap_manifest_exists = false;
}

#[when("the swap decision is computed")]
async fn when_swap_decision_computed(world: &mut BevyoutWorld) {
    let residency = world
        .swap_residency
        .expect("destination cell residency not given");
    world.swap_decision = Some(swap_policy::swap_decision(
        world.swap_manifest_exists,
        residency,
    ));
}

#[then(regex = r"^the swap decision is (Instant|Fallback)$")]
async fn then_swap_decision_is(world: &mut BevyoutWorld, expected: String) {
    let decision = world.swap_decision.expect("swap decision not computed yet");
    let expected = match expected.as_str() {
        "Instant" => swap_policy::SwapDecision::Instant,
        _ => swap_policy::SwapDecision::Fallback,
    };
    assert_eq!(decision, expected);
}

#[given("a fallback load that succeeds")]
async fn given_fallback_load_succeeds(world: &mut BevyoutWorld) {
    world.swap_fallback_load_ok = true;
}

#[given("a fallback load that fails")]
async fn given_fallback_load_fails(world: &mut BevyoutWorld) {
    world.swap_fallback_load_ok = false;
}

#[when("the fallback outcome is computed")]
async fn when_fallback_outcome_computed(world: &mut BevyoutWorld) {
    world.swap_fallback_outcome = Some(swap_policy::fallback_outcome(world.swap_fallback_load_ok));
}

#[then(regex = r"^the fallback outcome is (Proceed|ReturnToSource)$")]
async fn then_fallback_outcome_is(world: &mut BevyoutWorld, expected: String) {
    let outcome = world
        .swap_fallback_outcome
        .expect("fallback outcome not computed yet");
    let expected = match expected.as_str() {
        "Proceed" => swap_policy::FallbackOutcome::Proceed,
        _ => swap_policy::FallbackOutcome::ReturnToSource,
    };
    assert_eq!(outcome, expected);
}

// ---------------------------------------------------------------------
// physics_readiness.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^collider placements have kinds "([^"]*)"$"#)]
async fn given_collider_placements_have_kinds(world: &mut BevyoutWorld, kinds: String) {
    world.collider_work = kinds
        .split(',')
        .enumerate()
        .map(|(index, kind)| {
            let dynamic = match kind.trim() {
                "dynamic" => true,
                "static" | "keyframed" => false,
                other => panic!("unknown collider kind {other:?}"),
            };
            (index, dynamic)
        })
        .collect();
}

#[when("collider placement work is partitioned")]
async fn when_collider_work_partitioned(world: &mut BevyoutWorld) {
    world.collider_phase_partitions = Some(swap_policy::partition_collider_indices(
        world.collider_work.iter().copied(),
    ));
}

fn parse_index_list(list: &str) -> Vec<usize> {
    list.split(',')
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.trim().parse().expect("valid collider index"))
        .collect()
}

#[then(regex = r#"^static collider indices are "([^"]*)"$"#)]
async fn then_static_collider_indices_are(world: &mut BevyoutWorld, expected: String) {
    let (static_indices, _) = world
        .collider_phase_partitions
        .as_ref()
        .expect("collider work was not partitioned");
    assert_eq!(*static_indices, parse_index_list(&expected));
}

#[then(regex = r#"^dynamic collider indices are "([^"]*)"$"#)]
async fn then_dynamic_collider_indices_are(world: &mut BevyoutWorld, expected: String) {
    let (_, dynamic_indices) = world
        .collider_phase_partitions
        .as_ref()
        .expect("collider work was not partitioned");
    assert_eq!(*dynamic_indices, parse_index_list(&expected));
}

#[then("static collision is required before dynamic bodies")]
async fn then_static_collision_is_required_before_dynamic_bodies(world: &mut BevyoutWorld) {
    let (static_indices, dynamic_indices) = world
        .collider_phase_partitions
        .as_ref()
        .expect("collider work was not partitioned");
    assert!(!static_indices.is_empty());
    assert!(!dynamic_indices.is_empty());
    assert_eq!(
        swap_policy::next_collider_build_phase(false, true, true),
        swap_policy::ColliderBuildPhase::Dynamic
    );
    assert_ne!(
        swap_policy::next_collider_build_phase(false, true, false),
        swap_policy::ColliderBuildPhase::Ready
    );
}

// ---------------------------------------------------------------------
// fingerprints.feature (issue #49) -- appended section, do not interleave
// with steps above; new steps for this issue belong below this marker.
// ---------------------------------------------------------------------

fn parse_fingerprints(
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) -> fingerprints::CellFingerprints {
    fingerprints::CellFingerprints {
        plugin_content_set: plugin,
        converter,
        physics,
        prepare_pipeline,
    }
}

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) has recorded fingerprints plugin "([^"]*)" converter "([^"]*)" physics "([^"]*)" prepare_pipeline "([^"]*)"$"#
)]
async fn given_cell_recorded_fingerprints(
    world: &mut BevyoutWorld,
    hex: String,
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) {
    world
        .job_manifest
        .as_mut()
        .expect("job manifest not created yet")
        .record_fingerprints(
            parse_hex(&hex),
            parse_fingerprints(plugin, converter, physics, prepare_pipeline),
        );
}

#[when(
    regex = r#"^cell 0x([0-9a-fA-F]+) is checked against current fingerprints plugin "([^"]*)" converter "([^"]*)" physics "([^"]*)" prepare_pipeline "([^"]*)"$"#
)]
async fn when_cell_checked_against_current(
    world: &mut BevyoutWorld,
    hex: String,
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) {
    let current = parse_fingerprints(plugin, converter, physics, prepare_pipeline);
    let recorded = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet")
        .fingerprints_for(parse_hex(&hex))
        .cloned();
    world.fingerprint_stale_components =
        Some(fingerprints::stale_components(recorded.as_ref(), &current));
    world.fingerprint_current = Some(current);
}

#[then("the cell is valid")]
async fn then_cell_is_valid(world: &mut BevyoutWorld) {
    let stale = world
        .fingerprint_stale_components
        .as_ref()
        .expect("cell was not checked yet");
    assert!(
        stale.is_empty(),
        "expected no stale components, got {stale:?}"
    );
}

#[then(regex = r#"^the cell is stale in component "([^"]*)"$"#)]
async fn then_cell_is_stale_in_component(world: &mut BevyoutWorld, component: String) {
    let stale = world
        .fingerprint_stale_components
        .as_ref()
        .expect("cell was not checked yet");
    let labels: Vec<&str> = stale.iter().map(|c| c.label()).collect();
    assert!(
        labels.contains(&component.as_str()),
        "expected {component:?} among stale components, got {labels:?}"
    );
}

#[when(
    regex = r#"^cells "([^"]*)" are resumed without force against current fingerprints plugin "([^"]*)" converter "([^"]*)" physics "([^"]*)" prepare_pipeline "([^"]*)"$"#
)]
async fn when_cells_resumed_checked(
    world: &mut BevyoutWorld,
    list: String,
    plugin: String,
    converter: String,
    physics: String,
    prepare_pipeline: String,
) {
    let current = parse_fingerprints(plugin, converter, physics, prepare_pipeline);
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.fingerprint_resume_result = Some(jobs::filter_resume_checked(
        manifest, &selection, false, &current,
    ));
}

#[then(regex = r#"^the checked cells to run are "([^"]*)"$"#)]
async fn then_checked_cells_to_run_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (to_run, _, _) = world
        .fingerprint_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(to_run, &expected);
}

#[then(regex = r"^(\d+) cell\(s\) were checked as skipped$")]
async fn then_checked_cells_skipped(world: &mut BevyoutWorld, count: usize) {
    let (_, skipped, _) = world
        .fingerprint_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(*skipped, count);
}

#[then(regex = r"^(\d+) cell\(s\) were stale$")]
async fn then_checked_cells_stale(world: &mut BevyoutWorld, count: usize) {
    let (_, _, stale) = world
        .fingerprint_resume_result
        .as_ref()
        .expect("cells were not resumed yet");
    assert_eq!(stale.len(), count);
}

// ---------------------------------------------------------------------
// first_reveal.feature (issue #55)
// ---------------------------------------------------------------------

#[given(regex = r"^(\d+) reveal candidates evenly spaced from the arrival point$")]
async fn given_reveal_candidates_evenly_spaced(world: &mut BevyoutWorld, count: usize) {
    for i in 0..count {
        world
            .reveal_candidates
            .push(reveal_policy::RevealCandidate {
                index: world.reveal_candidates.len(),
                position: [i as f32, 0.0, 0.0],
            });
    }
}

#[given(regex = r"^a reveal candidate at distance (\d+) from the arrival point$")]
async fn given_reveal_candidate_at_distance(world: &mut BevyoutWorld, distance: f32) {
    world
        .reveal_candidates
        .push(reveal_policy::RevealCandidate {
            index: world.reveal_candidates.len(),
            position: [distance, 0.0, 0.0],
        });
}

#[given(regex = r"^the reveal budget is (\d+)$")]
async fn given_reveal_budget(world: &mut BevyoutWorld, budget: usize) {
    world.reveal_budget = budget;
}

#[when("the reveal chunks are planned")]
async fn when_reveal_chunks_planned(world: &mut BevyoutWorld) {
    world.reveal_chunks = reveal_policy::plan_reveal_chunks(
        &world.reveal_candidates,
        [0.0, 0.0, 0.0],
        world.reveal_budget,
    );
}

#[then(regex = r"^there (?:is|are) (\d+) reveal chunks?$")]
async fn then_reveal_chunk_count_is(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(
        world.reveal_chunks.len(),
        expected,
        "expected {expected} reveal chunks, got {:?}",
        world.reveal_chunks
    );
}

#[then("every reveal candidate appears in exactly one chunk")]
async fn then_every_reveal_candidate_appears_once(world: &mut BevyoutWorld) {
    let mut seen: Vec<usize> = world
        .reveal_chunks
        .iter()
        .flat_map(|chunk| chunk.iter().copied())
        .collect();
    seen.sort_unstable();
    let expected: Vec<usize> = (0..world.reveal_candidates.len()).collect();
    assert_eq!(
        seen, expected,
        "reveal chunks did not partition every candidate exactly once"
    );
}

#[then(regex = r"^the first reveal chunk contains the candidate at distance (\d+)$")]
async fn then_first_chunk_contains_candidate_at_distance(world: &mut BevyoutWorld, distance: f32) {
    let candidate = world
        .reveal_candidates
        .iter()
        .find(|candidate| candidate.position[0] == distance)
        .expect("no reveal candidate at that distance");
    let first_chunk = world
        .reveal_chunks
        .first()
        .expect("reveal chunks not planned yet");
    assert!(
        first_chunk.contains(&candidate.index),
        "expected first chunk {first_chunk:?} to contain candidate index {}",
        candidate.index
    );
}

// ---------------------------------------------------------------------
// door_animation.feature (issue #57)
// ---------------------------------------------------------------------

#[given(regex = r#"^a placement with clips "([^"]*)"$"#)]
async fn given_animation_clips(world: &mut BevyoutWorld, clips: String) {
    world.animation_clip_names = clips
        .split(',')
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect();
}

#[given("a placement with no clips")]
async fn given_animation_no_clips(world: &mut BevyoutWorld) {
    world.animation_clip_names = Vec::new();
}

#[when("the placement is opened")]
async fn when_animation_opened(world: &mut BevyoutWorld) {
    world.animation_selected_clip = Some(animation_policy::select_clip(
        animation_policy::ClipTransition::Opening,
        &world.animation_clip_names,
    ));
}

#[when("the placement is closed")]
async fn when_animation_closed(world: &mut BevyoutWorld) {
    world.animation_selected_clip = Some(animation_policy::select_clip(
        animation_policy::ClipTransition::Closing,
        &world.animation_clip_names,
    ));
}

#[then(regex = r#"^the selected clip is "([^"]*)"$"#)]
async fn then_animation_selected_clip_is(world: &mut BevyoutWorld, expected: String) {
    let selected = world
        .animation_selected_clip
        .clone()
        .expect("clip selection not computed yet");
    assert_eq!(selected, Some(expected));
}

#[then("no clip is selected")]
async fn then_animation_no_clip_selected(world: &mut BevyoutWorld) {
    let selected = world
        .animation_selected_clip
        .clone()
        .expect("clip selection not computed yet");
    assert_eq!(selected, None);
}

#[given(regex = r"^a travel door with an Open clip lasting ([\d.]+) seconds$")]
async fn given_animation_open_clip_seconds(world: &mut BevyoutWorld, seconds: f32) {
    world.animation_open_clip_seconds = Some(seconds);
}

#[given("a travel door with no Open clip")]
async fn given_animation_no_open_clip(world: &mut BevyoutWorld) {
    world.animation_open_clip_seconds = None;
}

#[when("the open lead is computed")]
async fn when_animation_open_lead_computed(world: &mut BevyoutWorld) {
    world.animation_open_lead = Some(animation_policy::open_lead_seconds(
        world.animation_open_clip_seconds,
        animation_policy::OPEN_LEAD_CAP_SECONDS,
    ));
}

#[then(regex = r"^the open lead is ([\d.]+) seconds$")]
async fn then_animation_open_lead_is(world: &mut BevyoutWorld, expected: f32) {
    let lead = world
        .animation_open_lead
        .expect("open lead not computed yet");
    assert!(
        (lead - expected).abs() < 1e-4,
        "open lead {lead} != expected {expected}"
    );
}

// ---------------------------------------------------------------------
// rust_irradiance.feature
// ---------------------------------------------------------------------

#[given(regex = r"^a Rust bake volume scale of ([\d.]+), ([\d.]+), ([\d.]+) metres$")]
async fn given_bake_volume_scale(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.bake_volume_scale = [x, y, z];
}

#[given(regex = r"^irradiance probe spacing is ([\d.]+) metres$")]
async fn given_bake_probe_spacing(world: &mut BevyoutWorld, spacing: f32) {
    world.bake_probe_spacing = spacing;
}

#[given(regex = r"^a probe resolution of (\d+), (\d+), (\d+)$")]
async fn given_bake_resolution(world: &mut BevyoutWorld, x: u32, y: u32, z: u32) {
    world.bake_resolution = [x, y, z];
}

#[given(regex = r"^irradiance sample count is (\d+)$")]
async fn given_bake_sample_count(world: &mut BevyoutWorld, samples: u32) {
    world.bake_samples = samples;
}

#[when("the Rust irradiance layout is planned")]
async fn when_bake_layout_planned(world: &mut BevyoutWorld) {
    world.bake_resolution =
        bake_policy::volume_resolution(world.bake_volume_scale, world.bake_probe_spacing);
}

#[when("the Rust irradiance atlas is planned")]
async fn when_bake_atlas_planned(world: &mut BevyoutWorld) {
    world.bake_atlas_dimensions = bake_policy::atlas_dimensions(world.bake_resolution);
}

#[when("the Rust irradiance ray count is planned")]
async fn when_bake_ray_count_planned(world: &mut BevyoutWorld) {
    world.bake_primary_rays =
        bake_policy::primary_ray_count(world.bake_resolution, world.bake_samples);
}

#[then(regex = r"^the probe resolution is (\d+), (\d+), (\d+)$")]
async fn then_bake_resolution_is(world: &mut BevyoutWorld, x: u32, y: u32, z: u32) {
    assert_eq!(world.bake_resolution, [x, y, z]);
}

#[then(regex = r"^the atlas dimensions are (\d+), (\d+), (\d+)$")]
async fn then_bake_atlas_dimensions_are(world: &mut BevyoutWorld, x: u32, y: u32, z: u32) {
    assert_eq!(world.bake_atlas_dimensions, [x, y, z]);
}

#[then(regex = r"^the primary ray count is (\d+)$")]
async fn then_bake_primary_ray_count_is(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(world.bake_primary_rays, expected);
}

// loading_fallback.feature (issue #59)
// ---------------------------------------------------------------------

#[given("no fallback is in flight")]
async fn given_no_fallback_in_flight(world: &mut BevyoutWorld) {
    world.fallback_state = Some(swap_policy::FallbackState::Idle);
}

#[given("a fallback is in flight")]
async fn given_fallback_in_flight(world: &mut BevyoutWorld) {
    world.fallback_state = Some(swap_policy::FallbackState::InFlight);
}

#[when("the destination becomes ready")]
async fn when_destination_becomes_ready(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::DestinationReady,
    ));
}

#[when("the fallback parse fails")]
async fn when_fallback_parse_fails(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::ParseFailed,
    ));
}

#[when("the player cancels the fallback")]
async fn when_player_cancels_fallback(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::PlayerCancelled,
    ));
}

#[when("a superseding travel request arrives")]
async fn when_superseding_travel_request_arrives(world: &mut BevyoutWorld) {
    world.fallback_lifecycle_outcome = Some(swap_policy::fallback_lifecycle_outcome(
        world.fallback_state.expect("fallback state not given"),
        swap_policy::FallbackEvent::SupersedingRequest,
    ));
}

#[then(
    regex = r"^the fallback lifecycle outcome is (Ignore|Proceed|ReturnToSource|Cancel|Supersede)$"
)]
async fn then_fallback_lifecycle_outcome_is(world: &mut BevyoutWorld, expected: String) {
    let outcome = world
        .fallback_lifecycle_outcome
        .expect("fallback lifecycle outcome not computed yet");
    let expected = match expected.as_str() {
        "Ignore" => swap_policy::FallbackLifecycleOutcome::Ignore,
        "Proceed" => swap_policy::FallbackLifecycleOutcome::Proceed,
        "ReturnToSource" => swap_policy::FallbackLifecycleOutcome::ReturnToSource,
        "Cancel" => swap_policy::FallbackLifecycleOutcome::Cancel,
        _ => swap_policy::FallbackLifecycleOutcome::Supersede,
    };
    assert_eq!(outcome, expected);
}

#[given(regex = r"^an overlay fade duration of ([\d.]+) seconds and max alpha ([\d.]+)$")]
async fn given_overlay_fade_duration(world: &mut BevyoutWorld, duration: f32, max_alpha: f32) {
    world.overlay_fade_duration = duration;
    world.overlay_fade_max_alpha = max_alpha;
}

#[when(regex = r"^the overlay has been fading in for ([\d.]+) seconds$")]
async fn when_overlay_fading_in(world: &mut BevyoutWorld, elapsed: f32) {
    world.overlay_alpha = Some(swap_policy::fade_in_alpha(
        elapsed,
        world.overlay_fade_duration,
        world.overlay_fade_max_alpha,
    ));
}

#[when(regex = r"^the overlay has been fading out for ([\d.]+) seconds$")]
async fn when_overlay_fading_out(world: &mut BevyoutWorld, elapsed: f32) {
    world.overlay_alpha = Some(swap_policy::fade_out_alpha(
        elapsed,
        world.overlay_fade_duration,
        world.overlay_fade_max_alpha,
    ));
}

#[then(regex = r"^the overlay alpha is ([\d.]+)$")]
async fn then_overlay_alpha_is(world: &mut BevyoutWorld, expected: f32) {
    let alpha = world.overlay_alpha.expect("overlay alpha not computed yet");
    assert!(
        (alpha - expected).abs() < 1e-4,
        "alpha {alpha} != expected {expected}"
    );
}

#[then(regex = r"^the overlay alpha matches fading in for ([\d.]+) seconds$")]
async fn then_overlay_alpha_matches_fading_in(world: &mut BevyoutWorld, elapsed: f32) {
    let alpha = world.overlay_alpha.expect("overlay alpha not computed yet");
    let expected = swap_policy::fade_in_alpha(
        elapsed,
        world.overlay_fade_duration,
        world.overlay_fade_max_alpha,
    );
    assert!(
        (alpha - expected).abs() < 1e-4,
        "alpha {alpha} != expected {expected}"
    );
}

// ---------------------------------------------------------------------
// resumable_bake.feature (issue #62) -- appended section, do not interleave
// ---------------------------------------------------------------------

/// The synthetic "current toolchain" every bake-validity check in this
/// section compares against; scenarios vary the *checked* revision and job
/// fingerprint instead.
const BAKE_CURRENT_REVISION: &str = "bake-current";
const BAKE_CURRENT_JOB_FINGERPRINT: &str = "job-current";

#[given(
    regex = r#"^cell 0x([0-9a-fA-F]+) has a recorded bake with revision "([^"]*)" and job fingerprint "([^"]*)"$"#
)]
async fn given_bake_recorded(
    world: &mut BevyoutWorld,
    hex: String,
    revision: String,
    job_fingerprint: String,
) {
    world.bake_recorded.insert(
        parse_hex(&hex),
        Some(manifest::PreparedBake {
            bake_revision: Some(revision),
            source_fingerprint: job_fingerprint,
            scene_path: "scenes/00000001/baked/scene.glb".into(),
            lightmaps: Vec::new(),
            lightmap_variance_pages: Vec::new(),
            lightmap_bindings: Vec::new(),
            bake_settings: Default::default(),
            irradiance_volume: None,
        }),
    );
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+) has no recorded bake$")]
async fn given_bake_not_recorded(world: &mut BevyoutWorld, hex: String) {
    world.bake_recorded.insert(parse_hex(&hex), None);
}

#[given(regex = r"^cell 0x([0-9a-fA-F]+)'s recorded bake is currently (valid|stale)$")]
async fn given_bake_validity(world: &mut BevyoutWorld, hex: String, validity: String) {
    let form_id = parse_hex(&hex);
    // Drive the validity through the real `bake_is_valid` check on a
    // recorded `PreparedBake` rather than asserting the boolean directly: a
    // "valid" cell's recorded bake matches the current toolchain, a "stale"
    // one carries an outdated bake revision.
    let recorded_revision = if validity == "valid" {
        BAKE_CURRENT_REVISION
    } else {
        "bake-outdated"
    };
    let recorded = manifest::PreparedBake {
        bake_revision: Some(recorded_revision.into()),
        source_fingerprint: BAKE_CURRENT_JOB_FINGERPRINT.into(),
        scene_path: "scenes/00000001/baked/scene.glb".into(),
        lightmaps: Vec::new(),
        lightmap_variance_pages: Vec::new(),
        lightmap_bindings: Vec::new(),
        bake_settings: Default::default(),
        irradiance_volume: None,
    };
    let valid = bake_plan::bake_is_valid(
        Some(&recorded),
        BAKE_CURRENT_REVISION,
        BAKE_CURRENT_JOB_FINGERPRINT,
    );
    world.bake_recorded.insert(form_id, Some(recorded));
    world.bake_validity.insert(form_id, valid);
}

#[when(
    regex = r#"^cell 0x([0-9a-fA-F]+)'s bake is checked against revision "([^"]*)" and job fingerprint "([^"]*)"$"#
)]
async fn when_bake_checked(
    world: &mut BevyoutWorld,
    hex: String,
    revision: String,
    job_fingerprint: String,
) {
    let recorded = world
        .bake_recorded
        .get(&parse_hex(&hex))
        .expect("no recorded bake state for that cell");
    world.bake_valid_result = Some(bake_plan::bake_is_valid(
        recorded.as_ref(),
        &revision,
        &job_fingerprint,
    ));
}

#[then(regex = r"^the recorded bake is (valid|stale)$")]
async fn then_bake_validity_is(world: &mut BevyoutWorld, expected: String) {
    let valid = world
        .bake_valid_result
        .expect("bake validity not checked yet");
    assert_eq!(valid, expected == "valid");
}

#[when(regex = r#"^cells "([^"]*)" are bake-resumed (without|with) force$"#)]
async fn when_bake_cells_resumed(world: &mut BevyoutWorld, list: String, force: String) {
    let manifest = world
        .job_manifest
        .as_ref()
        .expect("job manifest not created yet");
    let selection = parse_hex_list(&list);
    world.bake_resume_result = Some(bake_plan::filter_bake_resume(
        manifest,
        &selection,
        force == "with",
        &world.bake_validity,
    ));
}

#[then(regex = r#"^the bake cells to run are "([^"]*)"$"#)]
async fn then_bake_cells_to_run_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (to_run, _, _) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert_eq!(to_run, &expected);
}

#[then(regex = r"^(\d+) cell\(s\) were skipped as validly baked$")]
async fn then_bake_cells_skipped(world: &mut BevyoutWorld, count: usize) {
    let (_, skipped, _) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert_eq!(*skipped, count);
}

#[then(regex = r#"^the stale bake cells are "([^"]*)"$"#)]
async fn then_stale_bake_cells_are(world: &mut BevyoutWorld, list: String) {
    let expected = parse_hex_list(&list);
    let (_, _, stale) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert_eq!(stale, &expected);
}

#[then("no bake cells were stale")]
async fn then_no_bake_cells_stale(world: &mut BevyoutWorld) {
    let (_, _, stale) = world
        .bake_resume_result
        .as_ref()
        .expect("cells were not bake-resumed yet");
    assert!(stale.is_empty(), "unexpected stale bake cells: {stale:?}");
}

#[then(
    regex = r#"^the bake batch summary line for (\d+) baked, (\d+) skipped, and (\d+) failed is "([^"]*)"$"#
)]
async fn then_bake_batch_summary_line_is(
    world: &mut BevyoutWorld,
    baked: usize,
    skipped: usize,
    failed: usize,
    expected: String,
) {
    let _ = world;
    assert_eq!(
        bake_plan::bake_batch_summary_line(baked, skipped, failed),
        expected
    );
}

#[then(regex = r#"^the stale bake line for cell 0x([0-9a-fA-F]+) is "([^"]*)"$"#)]
async fn then_stale_bake_line_is(world: &mut BevyoutWorld, hex: String, expected: String) {
    let _ = world;
    assert_eq!(bake_plan::stale_bake_line(parse_hex(&hex)), expected);
}

// ---------------------------------------------------------------------
// state_persistence.feature (issues #60/#61)
// ---------------------------------------------------------------------

fn persist_transform(translation: [f32; 3]) -> persist_policy::TransformDelta {
    persist_policy::TransformDelta {
        translation,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: [1.0, 1.0, 1.0],
    }
}

#[given(regex = r"^a placement 0x([0-9a-fA-F]+) that is initially (enabled|disabled)$")]
async fn given_persist_placement(world: &mut BevyoutWorld, hex: String, state: String) {
    world
        .persist_placements
        .push(persist_policy::PlacementInfo {
            reference_form_id: parse_hex(&hex),
            initially_enabled: state == "enabled",
            enable_parent: None,
        });
}

#[given(regex = r"^a placement 0x([0-9a-fA-F]+) enable-parented to 0x([0-9a-fA-F]+)$")]
async fn given_persist_enable_parented(world: &mut BevyoutWorld, hex: String, parent: String) {
    world
        .persist_placements
        .push(persist_policy::PlacementInfo {
            reference_form_id: parse_hex(&hex),
            initially_enabled: true,
            enable_parent: Some(persist_policy::EnableParentLink {
                reference_form_id: parse_hex(&parent),
                inverted: false,
            }),
        });
}

#[given(
    regex = r"^a placement 0x([0-9a-fA-F]+) enable-parented to 0x([0-9a-fA-F]+) with the inverted flag$"
)]
async fn given_persist_enable_parented_inverted(
    world: &mut BevyoutWorld,
    hex: String,
    parent: String,
) {
    world
        .persist_placements
        .push(persist_policy::PlacementInfo {
            reference_form_id: parse_hex(&hex),
            initially_enabled: true,
            enable_parent: Some(persist_policy::EnableParentLink {
                reference_form_id: parse_hex(&parent),
                inverted: true,
            }),
        });
}

#[given(regex = r"^a save delta disabling reference 0x([0-9a-fA-F]+)$")]
async fn given_persist_delta_disables(world: &mut BevyoutWorld, hex: String) {
    world.persist_deltas.insert(
        parse_hex(&hex),
        persist_policy::ReferenceDelta {
            enabled: Some(false),
            ..Default::default()
        },
    );
}

#[given(regex = r"^a save delta deleting reference 0x([0-9a-fA-F]+)$")]
async fn given_persist_delta_deletes(world: &mut BevyoutWorld, hex: String) {
    world.persist_deltas.insert(
        parse_hex(&hex),
        persist_policy::ReferenceDelta {
            deleted: true,
            ..Default::default()
        },
    );
}

#[given(
    regex = r"^a save delta pointing reference 0x([0-9a-fA-F]+) at enable root 0x([0-9a-fA-F]+)$"
)]
async fn given_persist_delta_enable_root(world: &mut BevyoutWorld, hex: String, root: String) {
    world.persist_deltas.insert(
        parse_hex(&hex),
        persist_policy::ReferenceDelta {
            enable_root_form_id: Some(parse_hex(&root)),
            ..Default::default()
        },
    );
}

#[when("effective enabled state is resolved")]
async fn when_persist_effective_resolved(world: &mut BevyoutWorld) {
    world.persist_effective = Some(persist_policy::resolve_effective_enabled(
        &world.persist_placements,
        &world.persist_deltas,
    ));
}

#[then(regex = r"^placement 0x([0-9a-fA-F]+) resolves (enabled|disabled)$")]
async fn then_persist_resolves(world: &mut BevyoutWorld, hex: String, state: String) {
    let effective = world
        .persist_effective
        .as_ref()
        .expect("effective enabled state not resolved yet");
    let enabled = effective
        .get(&parse_hex(&hex))
        .copied()
        .expect("placement not resolved");
    assert_eq!(enabled, state == "enabled");
}

#[given(
    regex = r"^a baseline placement 0x([0-9a-fA-F]+) at \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
async fn given_persist_baseline(world: &mut BevyoutWorld, hex: String, x: f32, y: f32, z: f32) {
    world
        .persist_baselines
        .push(persist_policy::BaselinePlacement {
            reference_form_id: parse_hex(&hex),
            transform: persist_transform([x, y, z]),
        });
}

#[given(
    regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) at \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\] with linear velocity \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
#[allow(clippy::too_many_arguments)]
async fn given_persist_snapshot_moving(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
    vx: f32,
    vy: f32,
    vz: f32,
) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: true,
            transform: Some(persist_transform([x, y, z])),
            activated: None,
            body: Some(persist_policy::BodyDelta {
                linear_velocity: [vx, vy, vz],
                angular_velocity: [0.0, 0.0, 0.0],
                sleeping: false,
            }),
        });
}

#[given(
    regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) at \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\] at rest$"
)]
async fn given_persist_snapshot_at_rest(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: true,
            transform: Some(persist_transform([x, y, z])),
            activated: None,
            body: Some(persist_policy::BodyDelta::default()),
        });
}

#[given(regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) that is open$")]
async fn given_persist_snapshot_open(world: &mut BevyoutWorld, hex: String) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: true,
            transform: None,
            activated: Some(true),
            body: None,
        });
}

#[given(regex = r"^a runtime snapshot of 0x([0-9a-fA-F]+) that is no longer present$")]
async fn given_persist_snapshot_absent(world: &mut BevyoutWorld, hex: String) {
    world
        .persist_snapshots
        .push(persist_policy::RuntimeSnapshot {
            reference_form_id: parse_hex(&hex),
            present: false,
            transform: None,
            activated: None,
            body: None,
        });
}

#[when("the cell state is captured")]
async fn when_persist_captured(world: &mut BevyoutWorld) {
    world.persist_captured = Some(persist_policy::diff_capture(
        &world.persist_baselines,
        &world.persist_snapshots,
    ));
}

fn persist_captured_delta(world: &BevyoutWorld, form_id: u32) -> persist_policy::ReferenceDelta {
    world
        .persist_captured
        .as_ref()
        .expect("cell state not captured yet")
        .get(&form_id)
        .copied()
        .expect("no delta captured for reference")
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) has a transform$")]
async fn then_persist_captured_has_transform(world: &mut BevyoutWorld, hex: String) {
    assert!(
        persist_captured_delta(world, parse_hex(&hex))
            .transform
            .is_some()
    );
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) has a body state$")]
async fn then_persist_captured_has_body(world: &mut BevyoutWorld, hex: String) {
    assert!(
        persist_captured_delta(world, parse_hex(&hex))
            .body
            .is_some()
    );
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) is activated$")]
async fn then_persist_captured_activated(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        persist_captured_delta(world, parse_hex(&hex)).activated,
        Some(true)
    );
}

#[then(regex = r"^the captured delta for 0x([0-9a-fA-F]+) is deleted$")]
async fn then_persist_captured_deleted(world: &mut BevyoutWorld, hex: String) {
    assert!(persist_captured_delta(world, parse_hex(&hex)).deleted);
}

#[then(regex = r"^no delta is captured for 0x([0-9a-fA-F]+)$")]
async fn then_persist_no_delta(world: &mut BevyoutWorld, hex: String) {
    let captured = world
        .persist_captured
        .as_ref()
        .expect("cell state not captured yet");
    assert!(!captured.contains_key(&parse_hex(&hex)));
}

#[when("the captured state is applied")]
async fn when_persist_captured_applied(world: &mut BevyoutWorld) {
    let placements: Vec<persist_policy::PlacementInfo> = world
        .persist_baselines
        .iter()
        .map(|baseline| persist_policy::PlacementInfo {
            reference_form_id: baseline.reference_form_id,
            initially_enabled: true,
            enable_parent: None,
        })
        .collect();
    let deltas = world
        .persist_captured
        .clone()
        .expect("cell state not captured yet");
    world.persist_applications = persist_policy::plan_apply(&placements, &deltas);
}

#[when("the save state is applied")]
async fn when_persist_save_state_applied(world: &mut BevyoutWorld) {
    world.persist_applications =
        persist_policy::plan_apply(&world.persist_placements, &world.persist_deltas);
}

fn persist_application(
    world: &BevyoutWorld,
    form_id: u32,
) -> &persist_policy::PlacementApplication {
    world
        .persist_applications
        .iter()
        .find(|application| application.reference_form_id == form_id)
        .expect("save state not applied yet, or placement unknown")
}

#[then(regex = r"^the applied placement 0x([0-9a-fA-F]+) is visible$")]
async fn then_persist_applied_visible(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        persist_application(world, parse_hex(&hex)).visibility,
        persist_policy::VisibilityDecision::Visible
    );
}

#[then(regex = r"^the applied placement 0x([0-9a-fA-F]+) is hidden$")]
async fn then_persist_applied_hidden(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        persist_application(world, parse_hex(&hex)).visibility,
        persist_policy::VisibilityDecision::Hidden
    );
}

#[then(
    regex = r"^the applied placement 0x([0-9a-fA-F]+) has translation \[(-?[\d.]+), (-?[\d.]+), (-?[\d.]+)\]$"
)]
async fn then_persist_applied_translation(
    world: &mut BevyoutWorld,
    hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let transform = persist_application(world, parse_hex(&hex))
        .transform
        .expect("applied placement has no transform delta");
    assert_eq!(transform.translation, [x, y, z]);
}

// ---------------------------------------------------------------------
// collider_ownership.feature (issue #63) -- appended section, do not
// interleave; new steps for this issue belong below this marker.
// ---------------------------------------------------------------------

fn ownership_record(
    world: &mut BevyoutWorld,
    cell: u32,
    statics: usize,
    keyframed: usize,
    dynamics: usize,
) {
    for _ in 0..statics {
        world.ownership_next_id += 1;
        let id = world.ownership_next_id;
        world.ownership_ledger.record_static_shape(cell, id);
    }
    for _ in 0..keyframed {
        world.ownership_next_id += 1;
        let id = world.ownership_next_id;
        world.ownership_ledger.record_keyframed_body(cell, id);
    }
    for _ in 0..dynamics {
        world.ownership_next_id += 1;
        let id = world.ownership_next_id;
        world.ownership_ledger.record_dynamic_body(cell, id);
    }
}

#[given(
    regex = r"^cell 0x([0-9a-fA-F]+) recorded (\d+) static shapes?, (\d+) keyframed bod(?:y|ies), and (\d+) dynamic bod(?:y|ies)$"
)]
async fn given_cell_recorded_colliders(
    world: &mut BevyoutWorld,
    hex: String,
    statics: usize,
    keyframed: usize,
    dynamics: usize,
) {
    ownership_record(world, parse_hex(&hex), statics, keyframed, dynamics);
}

#[when(regex = r"^cell 0x([0-9a-fA-F]+) records (\d+) more static shapes?$")]
async fn when_cell_records_more(world: &mut BevyoutWorld, hex: String, statics: usize) {
    ownership_record(world, parse_hex(&hex), statics, 0, 0);
}

#[when(regex = r"^cell 0x([0-9a-fA-F]+) is released$")]
async fn when_cell_released(world: &mut BevyoutWorld, hex: String) {
    world.ownership_released = Some(world.ownership_ledger.release(parse_hex(&hex)));
}

#[then(regex = r"^the released set has (\d+) static shapes? and (\d+) bod(?:y|ies)$")]
async fn then_released_set_counts(world: &mut BevyoutWorld, shapes: usize, bodies: usize) {
    let released = world
        .ownership_released
        .as_ref()
        .expect("no release performed yet")
        .as_ref()
        .expect("release returned nothing");
    assert_eq!(released.shape_count(), shapes);
    assert_eq!(released.body_count(), bodies);
}

#[then("nothing is released")]
async fn then_nothing_released(world: &mut BevyoutWorld) {
    assert!(
        world
            .ownership_released
            .as_ref()
            .expect("no release performed yet")
            .is_none()
    );
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) is no longer tracked$")]
async fn then_cell_untracked(world: &mut BevyoutWorld, hex: String) {
    assert!(!world.ownership_ledger.is_tracked(parse_hex(&hex)));
}

#[then(regex = r"^cell 0x([0-9a-fA-F]+) is still tracked$")]
async fn then_cell_still_tracked(world: &mut BevyoutWorld, hex: String) {
    assert!(world.ownership_ledger.is_tracked(parse_hex(&hex)));
}

// ---------------------------------------------------------------------
// container_audio.feature -- appended section, do not interleave.
// ---------------------------------------------------------------------

#[given("an animated container with no record open or close sound")]
async fn given_container_without_record_audio(world: &mut BevyoutWorld) {
    world.container_audio_is_container = true;
    world.container_audio_record_open = None;
    world.container_audio_record_close = None;
}

#[given(
    regex = r"^an animated container with record open sound 0x([0-9a-fA-F]+) and record close sound 0x([0-9a-fA-F]+)$"
)]
async fn given_container_with_record_audio(world: &mut BevyoutWorld, open: String, close: String) {
    world.container_audio_is_container = true;
    world.container_audio_record_open = Some(parse_hex(&open));
    world.container_audio_record_close = Some(parse_hex(&close));
}

#[given("a non-container placement with no record open or close sound")]
async fn given_non_container_without_record_audio(world: &mut BevyoutWorld) {
    world.container_audio_is_container = false;
    world.container_audio_record_open = None;
    world.container_audio_record_close = None;
}

#[given(regex = r#"^its animation sound cues are "([^"]*)"$"#)]
async fn given_animation_sound_cues(world: &mut BevyoutWorld, encoded: String) {
    world.container_audio_cues = encoded
        .split(',')
        .map(|entry| {
            let (sequence_time, editor_id) = entry.split_once('=').expect("cue editor id");
            let (sequence, time) = sequence_time.split_once('@').expect("cue time");
            container_audio_policy::AnimationSoundCue {
                sequence: sequence.into(),
                time: time.parse().expect("numeric cue time"),
                editor_id: editor_id.into(),
            }
        })
        .collect();
}

#[given(
    regex = r"^its resolved animation sound ids are open 0x([0-9a-fA-F]+) and close 0x([0-9a-fA-F]+)$"
)]
async fn given_resolved_animation_sound_ids(world: &mut BevyoutWorld, open: String, close: String) {
    world.container_audio_resolved_open = Some(parse_hex(&open));
    world.container_audio_resolved_close = Some(parse_hex(&close));
}

#[when("authored container animation audio is resolved")]
async fn when_authored_container_audio_resolved(world: &mut BevyoutWorld) {
    world.container_audio_selected = Some(container_audio_policy::select_container_audio(
        &world.container_audio_cues,
    ));
}

#[when("resolved container sound ids are applied")]
async fn when_resolved_container_audio_applied(world: &mut BevyoutWorld) {
    let (open, close) = container_audio_policy::apply_container_audio_fallback(
        world.container_audio_is_container,
        world.container_audio_record_open,
        world.container_audio_record_close,
        world.container_audio_resolved_open,
        world.container_audio_resolved_close,
    );
    world.container_audio_prepared_open = open;
    world.container_audio_prepared_close = close;
}

#[then(regex = r#"^the selected open sound is "([^"]*)"$"#)]
async fn then_selected_open_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .container_audio_selected
            .as_ref()
            .expect("container audio not selected")
            .open_editor_id
            .as_deref(),
        Some(expected.as_str())
    );
}

#[then(regex = r#"^the selected close sound is "([^"]*)"$"#)]
async fn then_selected_close_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .container_audio_selected
            .as_ref()
            .expect("container audio not selected")
            .close_editor_id
            .as_deref(),
        Some(expected.as_str())
    );
}

#[then(regex = r"^the prepared open sound id is 0x([0-9a-fA-F]+)$")]
async fn then_prepared_open_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.container_audio_prepared_open,
        Some(parse_hex(&expected))
    );
}

#[then(regex = r"^the prepared close sound id is 0x([0-9a-fA-F]+)$")]
async fn then_prepared_close_sound(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.container_audio_prepared_close,
        Some(parse_hex(&expected))
    );
}

#[then("the prepared open sound id is absent")]
async fn then_prepared_open_sound_absent(world: &mut BevyoutWorld) {
    assert_eq!(world.container_audio_prepared_open, None);
}

#[then("the prepared close sound id is absent")]
async fn then_prepared_close_sound_absent(world: &mut BevyoutWorld) {
    assert_eq!(world.container_audio_prepared_close, None);
}

// ---------------------------------------------------------------------
// inventory.feature (M3 wave 1, issues #71/#72) -- appended section.
// ---------------------------------------------------------------------

#[given("an empty player inventory")]
async fn given_empty_player_inventory(world: &mut BevyoutWorld) {
    world.player_inventory = inventory_policy::Inventory::default();
}

#[given(regex = r"^an inventory stack of (\d+) items$")]
async fn given_inventory_stack(world: &mut BevyoutWorld, count: i32) {
    world.player_inventory =
        inventory_policy::Inventory::from_stacks([inventory_policy::InventoryStack {
            base_form_id: 0x10,
            count,
            condition: None,
        }]);
}

#[when(regex = r"^(\d+) items? 0x([0-9a-fA-F]+) at condition (\d+) (?:is|are) added$")]
async fn when_condition_items_added(
    world: &mut BevyoutWorld,
    count: i32,
    form_id: String,
    condition: u32,
) {
    world
        .player_inventory
        .add(inventory_policy::InventoryStack {
            base_form_id: parse_hex(&form_id),
            count,
            condition: Some(condition),
        });
}

#[when(regex = r"^(\d+) items? 0x([0-9a-fA-F]+) without condition (?:is|are) added$")]
async fn when_conditionless_items_added(world: &mut BevyoutWorld, count: i32, form_id: String) {
    world
        .player_inventory
        .add(inventory_policy::InventoryStack {
            base_form_id: parse_hex(&form_id),
            count,
            condition: None,
        });
}

#[when(regex = r"^item 0x([0-9a-fA-F]+) weighs ([\d.]+)$")]
async fn when_item_weight(world: &mut BevyoutWorld, form_id: String, weight: f32) {
    world.inventory_weights.insert(parse_hex(&form_id), weight);
}

#[when("its right-click drop policy is evaluated")]
async fn when_drop_policy_evaluated(world: &mut BevyoutWorld) {
    let count = world.player_inventory.stacks()[0].count;
    world.inventory_drop_action = inventory_policy::drop_action(count);
}

#[when(regex = r"^removing (\d+) items is attempted$")]
async fn when_removing_items_attempted(world: &mut BevyoutWorld, count: i32) {
    world.inventory_transfer = Some(world.player_inventory.remove(
        inventory_policy::StackKey {
            base_form_id: 0x10,
            condition: None,
        },
        count,
    ));
}

#[then(regex = r"^the inventory has (\d+) stacks? and (\d+) total items$")]
async fn then_inventory_shape(world: &mut BevyoutWorld, stacks: usize, total: i32) {
    assert_eq!(world.player_inventory.stacks().len(), stacks);
    assert_eq!(world.player_inventory.count(0x10), total);
}

#[then(regex = r"^carried weight is ([\d.]+)$")]
async fn then_carried_weight(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .player_inventory
        .total_weight(|form_id| world.inventory_weights.get(&form_id).copied());
    assert!((actual - expected).abs() < f32::EPSILON);
}

#[then("one item is selected for dropping")]
async fn then_drop_one(world: &mut BevyoutWorld) {
    assert_eq!(
        world.inventory_drop_action,
        Some(inventory_policy::DropAction::DropOne)
    );
}

#[then(regex = r"^a quantity from (\d+) through (\d+) is requested with default (\d+)$")]
async fn then_quantity_requested(world: &mut BevyoutWorld, min: i32, max: i32, default: i32) {
    assert_eq!(
        world.inventory_drop_action,
        Some(inventory_policy::DropAction::ChooseQuantity { min, max, default })
    );
}

#[then(regex = r"^the transfer is rejected and (\d+) items remain$")]
async fn then_transfer_rejected(world: &mut BevyoutWorld, remaining: i32) {
    assert_eq!(
        world.inventory_transfer,
        Some(inventory_policy::TransferResult::InsufficientItems)
    );
    assert_eq!(world.player_inventory.count(0x10), remaining);
}

// performance_probe.feature -- appended section, do not interleave;
// new steps for this feature belong below this marker.
// ---------------------------------------------------------------------

#[given(regex = r#"^frame-time samples "([^"]+)"$"#)]
async fn given_frame_time_samples(world: &mut BevyoutWorld, encoded: String) {
    world.performance_samples = encoded
        .split(',')
        .map(|pair| {
            let (sample, frame_time_ms) = pair
                .split_once(':')
                .expect("frame sample must use <sample>:<milliseconds>");
            performance_policy::FrameSample {
                sample: sample.parse().expect("sample id must be an integer"),
                frame_time_ms: frame_time_ms.parse().expect("frame time must be a number"),
            }
        })
        .collect();
}

#[when(
    regex = r"^frames after sample (\d+) are summarized with latest limit (\d+) and budget ([\d.]+) ms$"
)]
async fn when_frames_after_marker_are_summarized(
    world: &mut BevyoutWorld,
    marker: u64,
    limit: usize,
    budget_ms: f64,
) {
    world.performance_summary = Some(performance_policy::summarize_frame_window(
        &world.performance_samples,
        Some(marker),
        limit,
        budget_ms,
    ));
}

#[when(regex = r"^all frames are summarized with latest limit (\d+) and budget ([\d.]+) ms$")]
async fn when_all_frames_are_summarized(world: &mut BevyoutWorld, limit: usize, budget_ms: f64) {
    world.performance_summary = Some(performance_policy::summarize_frame_window(
        &world.performance_samples,
        None,
        limit,
        budget_ms,
    ));
}

fn performance_summary(world: &BevyoutWorld) -> &performance_policy::FrameProbeSummary {
    world
        .performance_summary
        .as_ref()
        .expect("frame samples have not been summarized")
}

#[then(regex = r"^the frame probe covers samples (\d+) through (\d+)$")]
async fn then_frame_probe_sample_range(world: &mut BevyoutWorld, first: u64, last: u64) {
    let summary = performance_summary(world);
    assert_eq!(summary.first_sample, Some(first));
    assert_eq!(summary.last_sample, Some(last));
}

#[then(regex = r"^the frame probe has (\d+) samples$")]
async fn then_frame_probe_sample_count(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(performance_summary(world).sample_count, count);
}

#[then(regex = r"^the frame probe p95 and max are ([\d.]+) and ([\d.]+) ms$")]
async fn then_frame_probe_p95_and_max(world: &mut BevyoutWorld, p95: f64, max: f64) {
    let summary = performance_summary(world);
    assert_eq!(summary.p95_ms, Some(p95));
    assert_eq!(summary.max_ms, Some(max));
}

#[then(regex = r"^(\d+) frames? exceed(?:s)? the probe budget$")]
async fn then_frames_exceed_budget(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(performance_summary(world).over_budget_count, count);
}

// ---------------------------------------------------------------------
// leveled_lists.feature (issue #74)
// ---------------------------------------------------------------------

/// Parses "level:base_form_id:count" entries, comma-separated; an empty
/// string yields no entries.
fn parse_leveled_entries(text: &str) -> Vec<leveled::PreparedLeveledEntry> {
    text.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let fields: Vec<&str> = part.split(':').collect();
            let [level, base_form_id, count] = fields.as_slice() else {
                panic!("leveled entry must be \"level:base_form_id:count\", got {part:?}");
            };
            leveled::PreparedLeveledEntry {
                level: level.parse().expect("entry level must be a u16"),
                base_form_id: base_form_id.parse().expect("entry form id must be a u32"),
                count: count.parse().expect("entry count must be an i32"),
            }
        })
        .collect()
}

/// Parses "base_form_id x count" stacks, comma-separated; an empty string
/// yields no stacks.
fn parse_resolved_stacks(text: &str) -> Vec<(u32, i32)> {
    text.split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let (form_id, count) = part
                .split_once('x')
                .expect("resolved stack must be \"base_form_id x count\"");
            (
                form_id.trim().parse().expect("stack form id must be a u32"),
                count.trim().parse().expect("stack count must be an i32"),
            )
        })
        .collect()
}

fn parse_leveled_flags(token: &str) -> u8 {
    match token {
        "use-all" => leveled::LEVELED_USE_ALL,
        "calculate-for-each-item" => leveled::LEVELED_CALCULATE_FOR_EACH_ITEM,
        numeric => numeric
            .parse()
            .expect("flags must be numeric or a known name"),
    }
}

#[given(
    regex = r#"^leveled list (\d+) with chance-none (\d+) and flags (\S+) and entries "([^"]*)"$"#
)]
async fn given_leveled_list(
    world: &mut BevyoutWorld,
    form_id: u32,
    chance_none: u8,
    flags: String,
    entries: String,
) {
    world.leveled_lists.insert(
        form_id,
        leveled::PreparedLeveledList {
            chance_none,
            flags: parse_leveled_flags(&flags),
            entries: parse_leveled_entries(&entries),
        },
    );
}

#[given(
    regex = r#"^a leveled seed from playthrough (\d+), cell (\d+), reference (\d+) named "([^"]+)"$"#
)]
async fn given_leveled_seed(
    world: &mut BevyoutWorld,
    playthrough_seed: u64,
    cell_form_id: u32,
    reference_form_id: u32,
    name: String,
) {
    world.leveled_seeds.insert(
        name,
        leveled::LeveledSeed::derive(playthrough_seed, cell_form_id, reference_form_id),
    );
}

#[when(regex = r#"^list (\d+) is resolved for player level (\d+) using seed "([^"]+)"$"#)]
async fn when_leveled_list_resolved(
    world: &mut BevyoutWorld,
    list_form_id: u32,
    player_level: u16,
    seed_name: String,
) {
    let seed = *world
        .leveled_seeds
        .get(&seed_name)
        .unwrap_or_else(|| panic!("no leveled seed named {seed_name:?}"));
    world.leveled_last_resolution = Some(leveled::resolve_leveled(
        list_form_id,
        &world.leveled_lists,
        seed,
        player_level,
    ));
}

#[then("the resolved stacks are empty")]
async fn then_resolved_stacks_are_empty(world: &mut BevyoutWorld) {
    assert!(
        world
            .leveled_last_resolution
            .as_ref()
            .expect("no leveled resolution computed yet")
            .is_empty()
    );
}

#[then(regex = r#"^the resolved stacks are "([^"]*)"$"#)]
async fn then_resolved_stacks_are(world: &mut BevyoutWorld, expected: String) {
    let resolved = world
        .leveled_last_resolution
        .as_ref()
        .expect("no leveled resolution computed yet");
    assert_eq!(*resolved, parse_resolved_stacks(&expected));
}

#[then(regex = r#"^seeds "([^"]+)" and "([^"]+)" are identical$"#)]
async fn then_leveled_seeds_are_identical(world: &mut BevyoutWorld, a: String, b: String) {
    assert_eq!(world.leveled_seeds[&a], world.leveled_seeds[&b]);
}

#[then(regex = r#"^seeds "([^"]+)" and "([^"]+)" are different$"#)]
async fn then_leveled_seeds_are_different(world: &mut BevyoutWorld, a: String, b: String) {
    assert_ne!(world.leveled_seeds[&a], world.leveled_seeds[&b]);
}

// ---------------------------------------------------------------------
// container_transfer.feature (issue #75)
// ---------------------------------------------------------------------

fn parse_container_form_id(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).expect("hex form id")
}

#[given(regex = r"^a container inventory entry 0x([0-9a-fA-F]+) x(-?\d+) leveled (yes|no)$")]
async fn given_container_inventory_entry(
    world: &mut BevyoutWorld,
    form_id: String,
    count: i32,
    leveled: String,
) {
    world
        .container_seed_entries
        .push(container_policy::SeedEntry {
            base_form_id: parse_container_form_id(&form_id),
            count,
            leveled: leveled == "yes",
        });
}

#[given(
    regex = r"^the leveled resolver for list 0x([0-9a-fA-F]+) returns 0x([0-9a-fA-F]+) x(-?\d+)$"
)]
async fn given_leveled_resolver_returns(
    world: &mut BevyoutWorld,
    list_form_id: String,
    item_form_id: String,
    count: i32,
) {
    world
        .container_resolver_lists
        .entry(parse_container_form_id(&list_form_id))
        .or_default()
        .push((parse_container_form_id(&item_form_id), count));
}

/// Shared by both "opened for the first time" and "reopened" steps: runs
/// `container_policy::open_container` against the world's current state,
/// counting resolver calls and syncing `container_stacks` (the single
/// source of truth the transfer `Then` steps below read) from the result.
fn open_world_container(world: &mut BevyoutWorld) {
    let entries = world.container_seed_entries.clone();
    let lists = world.container_resolver_lists.clone();
    let existing = world.container_state.take();
    let mut calls = 0u32;
    let resolved = container_policy::open_container(existing, &entries, |list_form_id| {
        calls += 1;
        lists.get(&list_form_id).cloned().unwrap_or_default()
    });
    world.container_resolver_calls += calls;
    world.container_stacks = resolved.stacks.clone();
    world.container_state = Some(resolved);
}

#[when("the container is opened for the first time")]
async fn when_container_opened_first_time(world: &mut BevyoutWorld) {
    open_world_container(world);
}

#[when("the container is reopened")]
async fn when_container_reopened(world: &mut BevyoutWorld) {
    open_world_container(world);
}

#[then("the container is resolved")]
async fn then_container_is_resolved(world: &mut BevyoutWorld) {
    assert!(
        world
            .container_state
            .as_ref()
            .expect("the container has not been opened")
            .resolved
    );
}

#[then(regex = r"^the resolver was called (\d+) times?$")]
async fn then_resolver_called_n_times(world: &mut BevyoutWorld, count: u32) {
    assert_eq!(world.container_resolver_calls, count);
}

#[given(regex = r"^a container stack of 0x([0-9a-fA-F]+) x(-?\d+)$")]
async fn given_container_stack(world: &mut BevyoutWorld, form_id: String, count: i32) {
    world
        .container_stacks
        .push((parse_container_form_id(&form_id), count));
}

#[given(regex = r"^a player stack of 0x([0-9a-fA-F]+) x(-?\d+)$")]
async fn given_player_stack(world: &mut BevyoutWorld, form_id: String, count: i32) {
    world
        .player_stacks
        .push((parse_container_form_id(&form_id), count));
}

#[when(regex = r"^one 0x([0-9a-fA-F]+) is taken from the container$")]
async fn when_take_one(world: &mut BevyoutWorld, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::take_one(
        &mut world.container_stacks,
        &mut world.player_stacks,
        form_id,
    ));
}

#[when(regex = r"^a stack of (-?\d+) 0x([0-9a-fA-F]+) is taken from the container$")]
async fn when_take_stack(world: &mut BevyoutWorld, count: i32, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::take_stack(
        &mut world.container_stacks,
        &mut world.player_stacks,
        form_id,
        count,
    ));
}

#[when(regex = r"^all 0x([0-9a-fA-F]+) is taken from the container$")]
async fn when_take_all(world: &mut BevyoutWorld, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::take_all(
        &mut world.container_stacks,
        &mut world.player_stacks,
        form_id,
    ));
}

#[when(regex = r"^one 0x([0-9a-fA-F]+) is stored into the container$")]
async fn when_store_one(world: &mut BevyoutWorld, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::store_one(
        &mut world.player_stacks,
        &mut world.container_stacks,
        form_id,
    ));
}

#[when(regex = r"^a stack of (-?\d+) 0x([0-9a-fA-F]+) is stored into the container$")]
async fn when_store_stack(world: &mut BevyoutWorld, count: i32, form_id: String) {
    let form_id = parse_container_form_id(&form_id);
    world.transfer_result = Some(container_policy::store_stack(
        &mut world.player_stacks,
        &mut world.container_stacks,
        form_id,
        count,
    ));
}

#[then(regex = r"^the container stack for 0x([0-9a-fA-F]+) is (-?\d+)$")]
async fn then_container_stack_is(world: &mut BevyoutWorld, form_id: String, count: i32) {
    let form_id = parse_container_form_id(&form_id);
    assert_eq!(
        container_policy::stack_count(&world.container_stacks, form_id),
        count
    );
}

#[then(regex = r"^the player stack for 0x([0-9a-fA-F]+) is (-?\d+)$")]
async fn then_player_stack_is(world: &mut BevyoutWorld, form_id: String, count: i32) {
    let form_id = parse_container_form_id(&form_id);
    assert_eq!(
        container_policy::stack_count(&world.player_stacks, form_id),
        count
    );
}

#[then("the transfer is rejected")]
async fn then_transfer_is_rejected(world: &mut BevyoutWorld) {
    assert!(
        world
            .transfer_result
            .as_ref()
            .expect("no transfer op has run yet")
            .is_err()
    );
}

// ---------------------------------------------------------------------
// container_persistence.feature (issue #76) -- appended section, do not
// interleave; new steps for this issue belong below this marker.
// ---------------------------------------------------------------------

/// Parses `0x<hex> x <count>` (e.g. "0x00000010 x 3") into a single-entry
/// stack list, matching the `Vec<(u32, i32)>` shape `persist_policy`'s
/// container types use.
fn parse_single_stack(hex: &str, count: i32) -> Vec<(u32, i32)> {
    vec![(parse_hex(hex), count)]
}

#[given(regex = r"^a container baseline 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+)$")]
async fn given_container_baseline(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_baselines.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerBaseline {
            stacks: parse_single_stack(&item_hex, count),
        },
    );
}

#[given(regex = r"^a container snapshot 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+)$")]
async fn given_container_snapshot(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_snapshots.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerSnapshot {
            stacks: parse_single_stack(&item_hex, count),
            resolved: false,
        },
    );
}

#[given(
    regex = r"^a resolved container snapshot 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+)$"
)]
async fn given_resolved_container_snapshot(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_snapshots.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerSnapshot {
            stacks: parse_single_stack(&item_hex, count),
            resolved: true,
        },
    );
}

#[when("container state is captured")]
async fn when_container_state_captured(world: &mut BevyoutWorld) {
    world.container_captured = Some(persist_policy::diff_capture_containers(
        &world.container_baselines,
        &world.container_snapshots,
    ));
}

fn captured_container_delta(world: &BevyoutWorld, form_id: u32) -> persist_policy::ContainerDelta {
    world
        .container_captured
        .as_ref()
        .expect("container state not captured yet")
        .get(&form_id)
        .cloned()
        .expect("no container delta captured for reference")
}

#[then(regex = r"^no container delta is captured for 0x([0-9a-fA-F]+)$")]
async fn then_no_container_delta(world: &mut BevyoutWorld, hex: String) {
    let captured = world
        .container_captured
        .as_ref()
        .expect("container state not captured yet");
    assert!(!captured.contains_key(&parse_hex(&hex)));
}

#[then(
    regex = r"^the captured container delta for 0x([0-9a-fA-F]+) has stack 0x([0-9a-fA-F]+) x (-?\d+)$"
)]
async fn then_captured_container_delta_has_stack(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    let delta = captured_container_delta(world, parse_hex(&container_hex));
    assert_eq!(delta.inventory, Some(parse_single_stack(&item_hex, count)));
}

#[then(regex = r"^the captured container delta for 0x([0-9a-fA-F]+) has no resolved marker$")]
async fn then_captured_container_delta_has_no_resolved_marker(
    world: &mut BevyoutWorld,
    hex: String,
) {
    let delta = captured_container_delta(world, parse_hex(&hex));
    assert_eq!(delta.leveled_resolved, None);
}

#[then(regex = r"^the captured container delta for 0x([0-9a-fA-F]+) has no inventory override$")]
async fn then_captured_container_delta_has_no_inventory(world: &mut BevyoutWorld, hex: String) {
    let delta = captured_container_delta(world, parse_hex(&hex));
    assert_eq!(delta.inventory, None);
}

#[then(regex = r"^the captured container delta for 0x([0-9a-fA-F]+) is resolved$")]
async fn then_captured_container_delta_is_resolved(world: &mut BevyoutWorld, hex: String) {
    let delta = captured_container_delta(world, parse_hex(&hex));
    assert_eq!(delta.leveled_resolved, Some(true));
}

#[given(
    regex = r"^a container delta 0x([0-9a-fA-F]+) with stack 0x([0-9a-fA-F]+) x (-?\d+) and resolved$"
)]
async fn given_container_delta(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    world.container_deltas.insert(
        parse_hex(&container_hex),
        persist_policy::ContainerDelta {
            inventory: Some(parse_single_stack(&item_hex, count)),
            leveled_resolved: Some(true),
        },
    );
}

#[given(regex = r"^a resolved-only container delta 0x([0-9a-fA-F]+)$")]
async fn given_resolved_only_container_delta(world: &mut BevyoutWorld, hex: String) {
    world.container_deltas.insert(
        parse_hex(&hex),
        persist_policy::ContainerDelta {
            inventory: None,
            leveled_resolved: Some(true),
        },
    );
}

#[when("the container state is applied")]
async fn when_container_state_applied(world: &mut BevyoutWorld) {
    world.container_seeded = Some(persist_policy::plan_apply_containers(
        &world.container_baselines,
        &world.container_deltas,
    ));
}

#[then(regex = r"^no container is seeded for 0x([0-9a-fA-F]+)$")]
async fn then_no_container_seeded(world: &mut BevyoutWorld, hex: String) {
    let seeded = world
        .container_seeded
        .as_ref()
        .expect("container state not applied yet");
    assert!(!seeded.contains_key(&parse_hex(&hex)));
}

fn seeded_container(world: &BevyoutWorld, form_id: u32) -> persist_policy::ContainerSnapshot {
    world
        .container_seeded
        .as_ref()
        .expect("container state not applied yet")
        .get(&form_id)
        .cloned()
        .expect("no container seeded for reference")
}

#[then(regex = r"^the seeded container 0x([0-9a-fA-F]+) has stack 0x([0-9a-fA-F]+) x (-?\d+)$")]
async fn then_seeded_container_has_stack(
    world: &mut BevyoutWorld,
    container_hex: String,
    item_hex: String,
    count: i32,
) {
    let snapshot = seeded_container(world, parse_hex(&container_hex));
    assert_eq!(snapshot.stacks, parse_single_stack(&item_hex, count));
}

#[then(regex = r"^the seeded container 0x([0-9a-fA-F]+) is resolved$")]
async fn then_seeded_container_is_resolved(world: &mut BevyoutWorld, hex: String) {
    let snapshot = seeded_container(world, parse_hex(&hex));
    assert!(snapshot.resolved);
}

// ---------------------------------------------------------------------
// item_flags.feature (issue #81)
// ---------------------------------------------------------------------

#[given(regex = r"^a base record with header flags 0x([0-9a-fA-F]+)$")]
async fn given_header_flags(world: &mut BevyoutWorld, hex: String) {
    world.flag_record_flags = parse_hex(&hex);
}

#[then("the record is a quest item")]
async fn then_record_is_quest_item(world: &mut BevyoutWorld) {
    assert!(item_rules::is_quest_item(world.flag_record_flags));
}

#[then("the record is not a quest item")]
async fn then_record_is_not_quest_item(world: &mut BevyoutWorld) {
    assert!(!item_rules::is_quest_item(world.flag_record_flags));
}

#[when(regex = r"^dropping item 0x([0-9a-fA-F]+) quest (yes|no) is checked$")]
async fn when_drop_checked(world: &mut BevyoutWorld, hex: String, quest: String) {
    world.rule_result = Some(item_rules::can_drop(parse_hex(&hex), quest == "yes"));
}

#[when(regex = r"^storing item 0x([0-9a-fA-F]+) quest (yes|no) is checked$")]
async fn when_store_checked(world: &mut BevyoutWorld, _hex: String, quest: String) {
    world.rule_result = Some(item_rules::can_store(quest == "yes"));
}

#[then("the transfer is allowed")]
async fn then_transfer_allowed(world: &mut BevyoutWorld) {
    assert_eq!(world.rule_result, Some(Ok(())));
}

#[then("the transfer is rejected as quest item")]
async fn then_transfer_rejected_quest(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rule_result,
        Some(Err(item_rules::TransferRejection::QuestItem))
    );
}

#[then("the transfer is rejected as caps")]
async fn then_transfer_rejected_caps(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rule_result,
        Some(Err(item_rules::TransferRejection::Caps))
    );
}

#[given(regex = r"^a carried stack of (\d+) weighing ([\d.]+) each quest (yes|no)$")]
async fn given_carried_stack(world: &mut BevyoutWorld, count: i32, weight: f32, quest: String) {
    world.carried_stacks.push((count, quest == "yes", weight));
}

#[when("the carried weight is totaled")]
async fn when_carried_weight_totaled(world: &mut BevyoutWorld) {
    world.carried_total = Some(
        world
            .carried_stacks
            .iter()
            .map(|&(count, quest, weight)| {
                count as f32 * item_rules::carried_weight(quest, Some(weight)).unwrap_or(0.0)
            })
            .sum(),
    );
}

#[then(regex = r"^the carried weight is ([\d.]+)$")]
async fn then_carried_weight_excluding_quest(world: &mut BevyoutWorld, expected: f32) {
    let total = world.carried_total.expect("carried weight not totaled yet");
    assert!((total - expected).abs() < 1e-4, "carried weight {total}");
}

#[when("taking a reference with no owner is classified")]
async fn when_take_unowned_classified(world: &mut BevyoutWorld) {
    world.take_classification = Some(item_rules::classify_take(None));
}

#[when(regex = r"^taking a reference owned by 0x([0-9a-fA-F]+) is classified$")]
async fn when_take_owned_classified(world: &mut BevyoutWorld, hex: String) {
    world.take_classification = Some(item_rules::classify_take(Some(parse_hex(&hex))));
}

#[then("the take is not theft")]
async fn then_take_not_theft(world: &mut BevyoutWorld) {
    assert_eq!(
        world.take_classification,
        Some(item_rules::TakeClassification::Take)
    );
}

#[then(regex = r"^the take is theft from 0x([0-9a-fA-F]+)$")]
async fn then_take_theft(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(
        world.take_classification,
        Some(item_rules::TakeClassification::Steal {
            owner_form_id: parse_hex(&hex)
        })
    );
}

#[then(regex = r"^the player caps total is (-?\d+)$")]
async fn then_player_caps_total(world: &mut BevyoutWorld, expected: i32) {
    assert_eq!(
        container_policy::stack_count(&world.player_stacks, item_rules::CAPS_FORM_ID),
        expected
    );
}

// ---------------------------------------------------------------------
// item_transactions.feature (M3/#95)
// ---------------------------------------------------------------------

#[given(
    regex = r"^the canonical player holds item 0x([0-9a-fA-F]+) form 0x([0-9a-fA-F]+) x(\d+) condition (none|\d+)$"
)]
async fn given_canonical_player_item(
    world: &mut BevyoutWorld,
    item_hex: String,
    form_hex: String,
    count: u32,
    condition: String,
) {
    world.canonical_ledger = ItemLedger::new();
    let item = ItemInstance::new(
        parse_item_instance_id(&item_hex),
        parse_hex(&form_hex),
        count,
        ItemState {
            condition: (condition != "none").then(|| condition.parse().unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    world
        .canonical_ledger
        .insert_holder(
            HolderId::Player,
            ItemHolderState {
                items: vec![item],
                ..Default::default()
            },
        )
        .unwrap();
}

#[given(regex = r"^the canonical holder 0x([0-9a-fA-F]+) is empty$")]
async fn given_empty_canonical_holder(world: &mut BevyoutWorld, reference_hex: String) {
    world
        .canonical_ledger
        .insert_holder(
            HolderId::FixtureContainer {
                reference_form_id: parse_hex(&reference_hex),
            },
            ItemHolderState::default(),
        )
        .unwrap();
}

#[given(regex = r"^the canonical player hotkey (\d+) is item 0x([0-9a-fA-F]+)$")]
async fn given_canonical_player_hotkey(world: &mut BevyoutWorld, slot: usize, item_hex: String) {
    assert_hotkey_slot(slot);
    world
        .canonical_ledger
        .bind_hotkey(HolderId::Player, slot, parse_item_instance_id(&item_hex))
        .unwrap();
}

#[when(
    regex = r"^transferring (\d+) of item 0x([0-9a-fA-F]+) to canonical holder 0x([0-9a-fA-F]+)$"
)]
async fn when_canonical_transfer(
    world: &mut BevyoutWorld,
    count: u32,
    item_hex: String,
    reference_hex: String,
) {
    world.canonical_result = Some(
        world
            .canonical_ledger
            .execute(TransactionRequest::Transfer {
                source: HolderId::Player,
                destination: HolderId::FixtureContainer {
                    reference_form_id: parse_hex(&reference_hex),
                },
                item_id: parse_item_instance_id(&item_hex),
                count,
            }),
    );
}

#[then(regex = r"^the canonical player item 0x([0-9a-fA-F]+) has count (\d+)$")]
async fn then_canonical_player_count(world: &mut BevyoutWorld, item_hex: String, count: u32) {
    let item_id = parse_item_instance_id(&item_hex);
    let item = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .and_then(|state| state.find(item_id))
        .unwrap_or_else(|| panic!("canonical player item {item_id:?} is missing"));
    assert_eq!(item.count, count);
}

#[then(regex = r"^canonical holder 0x([0-9a-fA-F]+) has item count (\d+)$")]
async fn then_canonical_holder_count(world: &mut BevyoutWorld, reference_hex: String, count: u32) {
    let holder = HolderId::FixtureContainer {
        reference_form_id: parse_hex(&reference_hex),
    };
    let actual = world
        .canonical_ledger
        .holders()
        .get(&holder)
        .and_then(|state| state.items.first())
        .map_or(0, |item| item.count);
    assert_eq!(actual, count);
}

#[then(regex = r"^the transaction moved item id 0x([0-9a-fA-F]+)$")]
async fn then_canonical_moved_id(world: &mut BevyoutWorld, item_hex: String) {
    let receipt = world
        .canonical_result
        .as_ref()
        .expect("canonical transaction was not run")
        .as_ref()
        .expect("canonical transaction failed");
    assert_eq!(receipt.moved[0].0, parse_item_instance_id(&item_hex));
}

#[then("the canonical transaction is rejected")]
async fn then_canonical_rejected(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.canonical_result.as_ref(),
        Some(Err(TransactionError::InsufficientItems))
    ));
}

#[then(regex = r"^canonical holder 0x([0-9a-fA-F]+) is empty$")]
async fn then_canonical_holder_empty(world: &mut BevyoutWorld, reference_hex: String) {
    let holder = HolderId::FixtureContainer {
        reference_form_id: parse_hex(&reference_hex),
    };
    assert!(
        world
            .canonical_ledger
            .holders()
            .get(&holder)
            .is_some_and(|state| state.items.is_empty())
    );
}

#[then(regex = r"^the canonical player hotkey (\d+) is item 0x([0-9a-fA-F]+)$")]
async fn then_canonical_player_hotkey(world: &mut BevyoutWorld, slot: usize, item_hex: String) {
    assert_hotkey_slot(slot);
    assert_eq!(
        world
            .canonical_ledger
            .bindings()
            .get(&HolderId::Player)
            .and_then(|bindings| bindings.hotkeys.get(slot).copied().flatten()),
        Some(parse_item_instance_id(&item_hex))
    );
}

#[then(regex = r"^the canonical player hotkey (\d+) is empty$")]
async fn then_canonical_player_hotkey_empty(world: &mut BevyoutWorld, slot: usize) {
    assert_hotkey_slot(slot);
    assert_eq!(
        world
            .canonical_ledger
            .bindings()
            .get(&HolderId::Player)
            .and_then(|bindings| bindings.hotkeys.get(slot).copied().flatten()),
        None
    );
}

#[then(regex = r"^canonical holder 0x([0-9a-fA-F]+) hotkey (\d+) is empty$")]
async fn then_canonical_holder_hotkey_empty(
    world: &mut BevyoutWorld,
    reference_hex: String,
    slot: usize,
) {
    assert_hotkey_slot(slot);
    let holder = HolderId::FixtureContainer {
        reference_form_id: parse_hex(&reference_hex),
    };
    assert_eq!(
        world
            .canonical_ledger
            .bindings()
            .get(&holder)
            .and_then(|bindings| bindings.hotkeys.get(slot).copied().flatten()),
        None
    );
}

// ---------------------------------------------------------------------
// drop_placement.feature (M3/#95)
// ---------------------------------------------------------------------

#[given("no drop candidates are blocked")]
async fn given_no_drop_candidates_blocked(world: &mut BevyoutWorld) {
    world.drop_blocked_count = 0;
    world.drop_all_blocked = false;
}

#[given(regex = r"^the first (\d+) drop candidates are blocked$")]
async fn given_first_drop_candidates_blocked(world: &mut BevyoutWorld, count: usize) {
    world.drop_blocked_count = count;
    world.drop_all_blocked = false;
}

#[given("every drop candidate is blocked")]
async fn given_every_drop_candidate_blocked(world: &mut BevyoutWorld) {
    world.drop_blocked_count = 0;
    world.drop_all_blocked = true;
}

#[when("the drop placement candidates are evaluated")]
async fn when_drop_placement_candidates_are_evaluated(world: &mut BevyoutWorld) {
    let blocked_count = world.drop_blocked_count;
    let all_blocked = world.drop_all_blocked;
    world.drop_decision = Some(drop_policy::choose_candidate(|distance| {
        if all_blocked {
            return true;
        }
        let candidate_index = ((drop_policy::DROP_DISTANCE_METERS - distance)
            / drop_policy::DROP_RETREAT_STEP_METERS)
            .round() as usize;
        candidate_index < blocked_count
    }));
}

#[then(regex = r"^the drop placement mode is (Camera|Retreat|PlayerFallback)$")]
async fn then_drop_placement_mode(world: &mut BevyoutWorld, mode: String) {
    let expected = match mode.as_str() {
        "Camera" => drop_policy::DropPlacementMode::Camera,
        "Retreat" => drop_policy::DropPlacementMode::Retreat,
        "PlayerFallback" => drop_policy::DropPlacementMode::PlayerFallback,
        other => panic!("unknown drop placement mode {other}"),
    };
    let decision = world
        .drop_decision
        .as_ref()
        .expect("drop placement was not evaluated");
    assert_eq!(decision.mode, expected);
}

#[then(regex = r"^the selected drop distance is ([\d.]+) metres$")]
async fn then_selected_drop_distance(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .drop_decision
        .as_ref()
        .and_then(|decision| decision.distance)
        .expect("drop placement did not select a camera distance");
    assert!(
        (actual - expected).abs() < 0.001,
        "expected {expected}, got {actual}"
    );
}

#[then("the drop distance is the player fallback")]
async fn then_drop_distance_is_player_fallback(world: &mut BevyoutWorld) {
    let decision = world
        .drop_decision
        .as_ref()
        .expect("drop placement was not evaluated");
    assert_eq!(
        decision.mode,
        drop_policy::DropPlacementMode::PlayerFallback
    );
    assert_eq!(decision.distance, None);
}

// item_use.feature (issue #99) -- appended section, do not interleave;
// new steps for later issues belong below this marker.
// ---------------------------------------------------------------------

#[given(regex = r"^an item with stats (Aid|Key|Misc) quest (yes|no)$")]
async fn given_item_stats_flat(world: &mut BevyoutWorld, stats: String, quest: String) {
    world.item_use_stats = Some(match stats.as_str() {
        "Aid" => item_use::ItemStats::Aid,
        "Key" => item_use::ItemStats::Key,
        "Misc" => item_use::ItemStats::Misc,
        other => panic!("unexpected item stats {other}"),
    });
    world.item_use_quest_item = quest == "yes";
}

#[given(regex = r"^an item with stats (Book|Note) text (yes|no) quest (yes|no)$")]
async fn given_item_stats_with_text(
    world: &mut BevyoutWorld,
    stats: String,
    has_text: String,
    quest: String,
) {
    let has_text = has_text == "yes";
    world.item_use_stats = Some(match stats.as_str() {
        "Book" => item_use::ItemStats::Book { has_text },
        "Note" => item_use::ItemStats::Note { has_text },
        other => panic!("unexpected item stats {other}"),
    });
    world.item_use_quest_item = quest == "yes";
}

#[then(regex = r"^the item use action is (Use|Read|Inert)$")]
async fn then_item_use_action_is(world: &mut BevyoutWorld, expected: String) {
    let stats = world
        .item_use_stats
        .expect("item stats not given for this scenario");
    world.item_use_action = Some(item_use::classify(stats, world.item_use_quest_item));
    let expected = match expected.as_str() {
        "Use" => item_use::ItemUseAction::Use,
        "Read" => item_use::ItemUseAction::Read,
        "Inert" => item_use::ItemUseAction::Inert,
        other => panic!("unexpected item use action {other}"),
    };
    assert_eq!(world.item_use_action, Some(expected));
}

#[when(regex = r"^item 0x([0-9a-fA-F]+) with stats (Aid|Key) quest (yes|no) is used$")]
async fn when_item_is_used(
    world: &mut BevyoutWorld,
    form_id: String,
    stats: String,
    quest: String,
) {
    let stats = match stats.as_str() {
        "Aid" => item_use::ItemStats::Aid,
        "Key" => item_use::ItemStats::Key,
        other => panic!("unexpected item stats {other}"),
    };
    if item_use::classify(stats, quest == "yes") == item_use::ItemUseAction::Use {
        world.player_inventory.remove(
            inventory_policy::StackKey {
                base_form_id: parse_hex(&form_id),
                condition: None,
            },
            1,
        );
    }
}

// equipment.feature (issue #98)
// ---------------------------------------------------------------------

fn equipment_stack_key(hex: &str, condition: &str) -> viewer_player::inventory::StackKey {
    viewer_player::inventory::StackKey {
        base_form_id: parse_hex(hex),
        condition: if condition == "none" {
            None
        } else {
            Some(condition.parse().expect("condition must be a whole number"))
        },
    }
}

#[given(regex = r"^apparel 0x([0-9a-fA-F]+) condition (\S+) mask 0x([0-9a-fA-F]+) is equipped$")]
async fn given_apparel_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    mask: String,
) {
    when_apparel_equipped(world, hex, condition, mask).await;
}

#[when(regex = r"^apparel 0x([0-9a-fA-F]+) condition (\S+) mask 0x([0-9a-fA-F]+) is equipped$")]
async fn when_apparel_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    mask: String,
) {
    let key = equipment_stack_key(&hex, &condition);
    let biped_slot_mask = parse_hex(&mask);
    world.equip_result = Some(
        world
            .equipment_state
            .equip(key, equipment::EquipKind::Apparel { biped_slot_mask }),
    );
}

#[given(
    regex = r"^weapon 0x([0-9a-fA-F]+) condition (\S+) requiring ammo 0x([0-9a-fA-F]+) is equipped$"
)]
async fn given_weapon_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    ammo_hex: String,
) {
    when_weapon_equipped(world, hex, condition, ammo_hex).await;
}

#[when(
    regex = r"^weapon 0x([0-9a-fA-F]+) condition (\S+) requiring ammo 0x([0-9a-fA-F]+) is equipped$"
)]
async fn when_weapon_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
    ammo_hex: String,
) {
    let key = equipment_stack_key(&hex, &condition);
    let ammo_form_id = Some(parse_hex(&ammo_hex));
    world.equip_result = Some(
        world
            .equipment_state
            .equip(key, equipment::EquipKind::Weapon { ammo_form_id }),
    );
}

#[given(regex = r"^ammo 0x([0-9a-fA-F]+) condition (\S+) is equipped$")]
async fn given_ammo_equipped(world: &mut BevyoutWorld, hex: String, condition: String) {
    when_ammo_equipped(world, hex, condition).await;
}

#[when(regex = r"^ammo 0x([0-9a-fA-F]+) condition (\S+) is equipped$")]
async fn when_ammo_equipped(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    world.equip_result = Some(world.equipment_state.equip(key, equipment::EquipKind::Ammo));
}

#[then(regex = r"^(?:apparel|weapon|ammo) 0x([0-9a-fA-F]+) condition (\S+) is equipped$")]
async fn then_stack_is_equipped(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(world.equipment_state.is_equipped(key));
}

#[then(regex = r"^(?:apparel|weapon|ammo) 0x([0-9a-fA-F]+) condition (\S+) is evicted$")]
async fn then_stack_is_evicted(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(!world.equipment_state.is_equipped(key));
    let outcome = world
        .equip_result
        .clone()
        .expect("no equip has been performed yet")
        .expect("the last equip did not succeed");
    assert!(
        outcome.evicted.contains(&key),
        "expected {key:?} in evicted {:?}",
        outcome.evicted
    );
}

#[then("the equip attempt is rejected as not equippable")]
async fn then_rejected_not_equippable(world: &mut BevyoutWorld) {
    assert_eq!(
        world.equip_result,
        Some(Err(equipment::EquipError::NotEquippable))
    );
}

#[then("the equip attempt is rejected as incompatible ammo")]
async fn then_rejected_incompatible_ammo(world: &mut BevyoutWorld) {
    assert_eq!(
        world.equip_result,
        Some(Err(equipment::EquipError::IncompatibleAmmo))
    );
}

#[then("the equip attempt is rejected with no weapon equipped")]
async fn then_rejected_no_weapon_equipped(world: &mut BevyoutWorld) {
    assert_eq!(
        world.equip_result,
        Some(Err(equipment::EquipError::NoWeaponEquipped))
    );
}

#[then(regex = r"^dropping apparel 0x([0-9a-fA-F]+) condition (\S+) is refused while equipped$")]
async fn then_drop_refused_while_equipped(
    world: &mut BevyoutWorld,
    hex: String,
    condition: String,
) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(world.equipment_state.is_equipped(key));
}

#[then(regex = r"^dropping apparel 0x([0-9a-fA-F]+) condition (\S+) is allowed$")]
async fn then_drop_allowed(world: &mut BevyoutWorld, hex: String, condition: String) {
    let key = equipment_stack_key(&hex, &condition);
    assert!(!world.equipment_state.is_equipped(key));
}

// recipes.feature (issue #117) -- appended section, do not interleave.
// ---------------------------------------------------------------------

#[given(
    regex = r"^a recipe with ingredient 0x([0-9a-fA-F]+) quantity (-?\d+) and output 0x([0-9a-fA-F]+) quantity (-?\d+)$"
)]
async fn given_recipe(
    world: &mut BevyoutWorld,
    ingredient_form_id: String,
    ingredient_quantity: i32,
    output_form_id: String,
    output_quantity: i32,
) {
    world.recipe_under_test = Some(recipe_policy::PreparedRecipe {
        form_id: 0x100,
        ingredients: vec![recipe_policy::PreparedRecipeItem {
            item_form_id: parse_hex(&ingredient_form_id),
            quantity: ingredient_quantity,
            order: 0,
        }],
        outputs: vec![recipe_policy::PreparedRecipeItem {
            item_form_id: parse_hex(&output_form_id),
            quantity: output_quantity,
            order: 0,
        }],
        ..Default::default()
    });
}

#[given(regex = r#"^recipe items "([^"]*)" are available$"#)]
async fn given_recipe_items_available(world: &mut BevyoutWorld, items: String) {
    world.recipe_available_items = items
        .split(',')
        .map(|item| parse_hex(item.trim()))
        .collect();
}

#[given(regex = r"^the recipe also has ingredient 0x([0-9a-fA-F]+) quantity (-?\d+)$")]
async fn given_duplicate_recipe_ingredient(
    world: &mut BevyoutWorld,
    item_form_id: String,
    quantity: i32,
) {
    world
        .recipe_under_test
        .as_mut()
        .expect("recipe must be created first")
        .ingredients
        .push(recipe_policy::PreparedRecipeItem {
            item_form_id: parse_hex(&item_form_id),
            quantity,
            order: 1,
        });
}

#[when("the recipe is validated")]
async fn when_recipe_validated(world: &mut BevyoutWorld) {
    let recipe = world
        .recipe_under_test
        .as_ref()
        .expect("recipe must be created first");
    world.recipe_validation = Some(recipe_policy::validate_recipe(
        recipe,
        &world.recipe_available_items,
    ));
}

#[then("recipe validation rejects a non-positive quantity")]
async fn then_recipe_rejects_non_positive_quantity(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.recipe_validation.as_ref(),
        Some(Err(
            recipe_policy::RecipeValidationError::NonPositiveQuantity { .. }
        ))
    ));
}

#[then("recipe validation rejects duplicate ingredients")]
async fn then_recipe_rejects_duplicate_ingredients(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.recipe_validation.as_ref(),
        Some(Err(
            recipe_policy::RecipeValidationError::DuplicateIngredient { .. }
        ))
    ));
}

#[then(regex = r"^the recipe ingredient quantity remains (-?\d+)$")]
async fn then_recipe_quantity_unchanged(world: &mut BevyoutWorld, expected: i32) {
    assert_eq!(
        world
            .recipe_under_test
            .as_ref()
            .expect("recipe must be created first")
            .ingredients[0]
            .quantity,
        expected
    );
}

// ---------------------------------------------------------------------
// hybrid_lighting.feature -- appended section, do not interleave.
// ---------------------------------------------------------------------

#[given(regex = r"^prepared point-shadow visibility is ([\d.]+)$")]
async fn given_hybrid_prepared_visibility(world: &mut BevyoutWorld, visibility: f32) {
    world.hybrid_prepared_visibility = Some(visibility);
}

#[given(regex = r"^realtime point-shadow visibility is ([\d.]+)$")]
async fn given_hybrid_realtime_visibility(world: &mut BevyoutWorld, visibility: f32) {
    world.hybrid_realtime_visibility = Some(visibility);
}

#[when("hybrid point-shadow visibility is combined")]
async fn when_hybrid_visibility_is_combined(world: &mut BevyoutWorld) {
    world.hybrid_combined_visibility = Some(hybrid_shadow_policy::hybrid_shadow_visibility(
        world.hybrid_prepared_visibility,
        world.hybrid_realtime_visibility,
    ));
}

#[then(regex = r"^combined point-shadow visibility is ([\d.]+)$")]
async fn then_hybrid_visibility(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .hybrid_combined_visibility
        .expect("hybrid visibility was not evaluated");
    assert!((actual - expected).abs() < 1e-6, "{actual} != {expected}");
}

// actor_catalog.feature (issue #103, M4 wave 1 task C) -- appended section,
// do not interleave.
// ---------------------------------------------------------------------

fn actor_catalog_actor_mut(
    world: &mut BevyoutWorld,
    form_id: u32,
) -> &mut actor_catalog::ActorRecordInput {
    world
        .actor_catalog_inputs
        .actors
        .get_mut(&form_id)
        .unwrap_or_else(|| panic!("actor {form_id:08x} was not created first"))
}

#[given(regex = r"^an NPC_ actor 0x([0-9a-fA-F]+) with race 0x([0-9a-fA-F]+)$")]
async fn given_npc_actor(world: &mut BevyoutWorld, hex: String, race_hex: String) {
    let form_id = parse_hex(&hex);
    let mut actor = actor_catalog::ActorRecordInput {
        form_id,
        kind: actor_catalog::ActorRecordKind::Npc,
        ..actor_catalog::ActorRecordInput::default()
    };
    actor.traits.race_form_id = Some(parse_hex(&race_hex));
    world.actor_catalog_inputs.actors.insert(form_id, actor);
}

#[given(regex = r"^actor 0x([0-9a-fA-F]+) has template 0x([0-9a-fA-F]+) using ([a-z_,]+)$")]
async fn given_actor_template(
    world: &mut BevyoutWorld,
    hex: String,
    template_hex: String,
    groups: String,
) {
    let form_id = parse_hex(&hex);
    let template = parse_hex(&template_hex);
    let actor = actor_catalog_actor_mut(world, form_id);
    actor.base_template_form_id = Some(template);
    for group in groups.split(',') {
        match group.trim() {
            "traits" => actor.template_usage.traits = true,
            "stats" => actor.template_usage.stats = true,
            "factions" => actor.template_usage.factions = true,
            "actor_effect_list" => actor.template_usage.actor_effect_list = true,
            "ai_data" => actor.template_usage.ai_data = true,
            "ai_packages" => actor.template_usage.ai_packages = true,
            "model_animation" => actor.template_usage.model_animation = true,
            "base_data" => actor.template_usage.base_data = true,
            "inventory" => actor.template_usage.inventory = true,
            "script" => actor.template_usage.script = true,
            other => panic!("unknown template group {other:?}"),
        }
    }
}

#[given(regex = r"^actor 0x([0-9a-fA-F]+) is female$")]
async fn given_actor_is_female(world: &mut BevyoutWorld, hex: String) {
    actor_catalog_actor_mut(world, parse_hex(&hex))
        .traits
        .female = true;
}

#[given(regex = r"^actor 0x([0-9a-fA-F]+) has inventory item 0x([0-9a-fA-F]+) x(-?\d+)$")]
async fn given_actor_inventory_item(
    world: &mut BevyoutWorld,
    actor_hex: String,
    item_hex: String,
    count: i32,
) {
    actor_catalog_actor_mut(world, parse_hex(&actor_hex))
        .inventory
        .push(manifest::PreparedInventoryEntry {
            base_form_id: parse_hex(&item_hex),
            count,
            record_kind: "ARMO".into(),
            editor_id: None,
            display_name: None,
            leveled: false,
        });
}

#[given(regex = r"^actor 0x([0-9a-fA-F]+) class is 0x([0-9a-fA-F]+)$")]
async fn given_actor_class(world: &mut BevyoutWorld, hex: String, class_hex: String) {
    let form_id = parse_hex(&hex);
    let class_form_id = parse_hex(&class_hex);
    actor_catalog_actor_mut(world, form_id).class_form_id = Some(class_form_id);
}

#[given(regex = r"^actor 0x([0-9a-fA-F]+) has faction 0x([0-9a-fA-F]+) rank (-?\d+)$")]
async fn given_actor_faction(world: &mut BevyoutWorld, hex: String, faction_hex: String, rank: i8) {
    let form_id = parse_hex(&hex);
    let faction_form_id = parse_hex(&faction_hex);
    actor_catalog_actor_mut(world, form_id)
        .factions
        .push(actor_catalog::ActorFactionInput {
            faction_form_id,
            rank,
        });
}

#[given(regex = r"^actor 0x([0-9a-fA-F]+) has package 0x([0-9a-fA-F]+)$")]
async fn given_actor_package(world: &mut BevyoutWorld, hex: String, package_hex: String) {
    let form_id = parse_hex(&hex);
    let package_form_id = parse_hex(&package_hex);
    actor_catalog_actor_mut(world, form_id)
        .package_form_ids
        .push(package_form_id);
}

#[given(regex = r#"^a leveled list 0x([0-9a-fA-F]+) with entries "([^"]*)"$"#)]
async fn given_actor_catalog_leveled_list(world: &mut BevyoutWorld, hex: String, entries: String) {
    let form_id = parse_hex(&hex);
    world.actor_catalog_inputs.leveled.insert(
        form_id,
        actor_catalog::LeveledInput {
            form_id,
            entries: parse_hex_list(&entries),
        },
    );
}

#[given(
    regex = r#"^faction 0x([0-9a-fA-F]+) is known with rank (-?\d+) male title "([^"]*)" female title "([^"]*)"$"#
)]
async fn given_faction_known(
    world: &mut BevyoutWorld,
    hex: String,
    rank: i32,
    male_title: String,
    female_title: String,
) {
    let form_id = parse_hex(&hex);
    world.actor_catalog_inputs.factions.insert(
        form_id,
        actor_catalog::FactionInput {
            form_id,
            ranks: vec![actor_catalog::FactionRankInput {
                rank_number: rank,
                male_title: Some(male_title),
                female_title: Some(female_title),
            }],
            ..Default::default()
        },
    );
}

#[given(regex = r"^a placement 0x([0-9a-fA-F]+) of base 0x([0-9a-fA-F]+) as (Npc|Creature)$")]
async fn given_placement(
    world: &mut BevyoutWorld,
    reference_hex: String,
    base_hex: String,
    kind: String,
) {
    let kind = match kind.as_str() {
        "Npc" => actor_catalog::ActorRecordKind::Npc,
        "Creature" => actor_catalog::ActorRecordKind::Creature,
        other => panic!("unknown actor kind {other:?}"),
    };
    world
        .actor_catalog_inputs
        .placements
        .push(actor_catalog::ActorPlacementInput {
            reference_form_id: parse_hex(&reference_hex),
            base_form_id: parse_hex(&base_hex),
            kind,
            ..actor_catalog::ActorPlacementInput::default()
        });
}

#[when("the actor catalog is built")]
async fn when_actor_catalog_built(world: &mut BevyoutWorld) {
    world.actor_catalog_result = Some(actor_catalog::build_actor_catalog(
        &world.actor_catalog_inputs,
        "fixture-fingerprint",
    ));
}

fn actor_catalog_blueprint<'a>(
    world: &'a BevyoutWorld,
    reference_hex: &str,
) -> &'a actor_catalog::ActorBlueprint {
    let reference_form_id = parse_hex(reference_hex);
    world
        .actor_catalog_result
        .as_ref()
        .expect("actor catalog must be built first")
        .entries
        .iter()
        .find_map(|entry| match entry {
            actor_catalog::ActorCatalogEntry::Prepared(blueprint)
                if blueprint.reference_form_id == reference_form_id =>
            {
                Some(blueprint.as_ref())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no prepared blueprint for reference {reference_hex}"))
}

#[then(regex = r"^blueprint for reference 0x([0-9a-fA-F]+) has race 0x([0-9a-fA-F]+)$")]
async fn then_blueprint_race(world: &mut BevyoutWorld, reference_hex: String, race_hex: String) {
    let blueprint = actor_catalog_blueprint(world, &reference_hex);
    assert_eq!(blueprint.race_form_id, Some(parse_hex(&race_hex)));
}

#[then(regex = r"^blueprint for reference 0x([0-9a-fA-F]+) is inherited$")]
async fn then_blueprint_inherited(world: &mut BevyoutWorld, reference_hex: String) {
    assert!(actor_catalog_blueprint(world, &reference_hex).inherited);
}

#[then(regex = r"^blueprint for reference 0x([0-9a-fA-F]+) is not inherited$")]
async fn then_blueprint_not_inherited(world: &mut BevyoutWorld, reference_hex: String) {
    assert!(!actor_catalog_blueprint(world, &reference_hex).inherited);
}

#[then(regex = r#"^blueprint for reference 0x([0-9a-fA-F]+) has diagnostic "([^"]*)"$"#)]
async fn then_blueprint_diagnostic(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    let blueprint = actor_catalog_blueprint(world, &reference_hex);
    assert!(
        blueprint.diagnostics.contains(&expected),
        "expected diagnostic {expected:?} in {:?}",
        blueprint.diagnostics
    );
}

#[then(regex = r#"^blueprint for reference 0x([0-9a-fA-F]+) has diagnostic containing "([^"]*)"$"#)]
async fn then_blueprint_diagnostic_containing(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    let blueprint = actor_catalog_blueprint(world, &reference_hex);
    assert!(
        blueprint
            .diagnostics
            .iter()
            .any(|message| message.contains(expected.as_str())),
        "expected a diagnostic containing {expected:?} in {:?}",
        blueprint.diagnostics
    );
}

#[then(
    regex = r#"^blueprint for reference 0x([0-9a-fA-F]+) is a leveled template with candidates "([^"]*)"$"#
)]
async fn then_blueprint_leveled_template(
    world: &mut BevyoutWorld,
    reference_hex: String,
    candidates: String,
) {
    let blueprint = actor_catalog_blueprint(world, &reference_hex);
    assert!(blueprint.is_leveled_template);
    assert_eq!(blueprint.template_candidates, parse_hex_list(&candidates));
}

#[then(regex = r"^blueprint for reference 0x([0-9a-fA-F]+) resolves base 0x([0-9a-fA-F]+)$")]
async fn then_blueprint_resolved_base(
    world: &mut BevyoutWorld,
    reference_hex: String,
    base_hex: String,
) {
    assert_eq!(
        actor_catalog_blueprint(world, &reference_hex).resolved_base_form_id,
        Some(parse_hex(&base_hex))
    );
}

#[then(regex = r"^blueprint for reference 0x([0-9a-fA-F]+) is female$")]
async fn then_blueprint_is_female(world: &mut BevyoutWorld, reference_hex: String) {
    assert!(actor_catalog_blueprint(world, &reference_hex).female);
}

#[then(
    regex = r"^blueprint for reference 0x([0-9a-fA-F]+) has inventory item 0x([0-9a-fA-F]+) x(-?\d+)$"
)]
async fn then_blueprint_inventory_item(
    world: &mut BevyoutWorld,
    reference_hex: String,
    item_hex: String,
    count: i32,
) {
    let item_form_id = parse_hex(&item_hex);
    assert!(
        actor_catalog_blueprint(world, &reference_hex)
            .inventory
            .iter()
            .any(|entry| entry.base_form_id == item_form_id && entry.count == count)
    );
}

#[then(
    regex = r#"^blueprint for reference 0x([0-9a-fA-F]+) has faction 0x([0-9a-fA-F]+) title "([^"]*)"$"#
)]
async fn then_blueprint_faction_title(
    world: &mut BevyoutWorld,
    reference_hex: String,
    faction_hex: String,
    title: String,
) {
    let blueprint = actor_catalog_blueprint(world, &reference_hex);
    let faction_form_id = parse_hex(&faction_hex);
    let membership = blueprint
        .factions
        .iter()
        .find(|membership| membership.faction_form_id == faction_form_id)
        .unwrap_or_else(|| panic!("no faction membership {faction_form_id:08x}"));
    assert_eq!(membership.title.as_deref(), Some(title.as_str()));
}

fn assert_actor_catalog_entry_kind(world: &BevyoutWorld, reference_hex: &str, expected_kind: &str) {
    let reference_form_id = parse_hex(reference_hex);
    let catalog = world
        .actor_catalog_result
        .as_ref()
        .expect("actor catalog must be built first");
    let found = catalog
        .entries
        .iter()
        .any(|entry| match (entry, expected_kind) {
            (
                actor_catalog::ActorCatalogEntry::Unresolved {
                    reference_form_id: id,
                    ..
                },
                "Unresolved",
            ) => *id == reference_form_id,
            (
                actor_catalog::ActorCatalogEntry::Skipped {
                    reference_form_id: id,
                    ..
                },
                "Skipped",
            ) => *id == reference_form_id,
            (
                actor_catalog::ActorCatalogEntry::Unsupported {
                    reference_form_id: id,
                    ..
                },
                "Unsupported",
            ) => *id == reference_form_id,
            _ => false,
        });
    assert!(
        found,
        "no {expected_kind} entry for reference {reference_hex}"
    );
}

#[then(regex = r"^entry for reference 0x([0-9a-fA-F]+) is unresolved$")]
async fn then_entry_unresolved(world: &mut BevyoutWorld, reference_hex: String) {
    assert_actor_catalog_entry_kind(world, &reference_hex, "Unresolved");
}

#[then(regex = r"^entry for reference 0x([0-9a-fA-F]+) is skipped$")]
async fn then_entry_skipped(world: &mut BevyoutWorld, reference_hex: String) {
    assert_actor_catalog_entry_kind(world, &reference_hex, "Skipped");
}

#[then(
    regex = r"^the actor catalog counts prepared (\d+) inherited (\d+) unresolved (\d+) unsupported (\d+) skipped (\d+)$"
)]
async fn then_actor_catalog_counts(
    world: &mut BevyoutWorld,
    prepared: usize,
    inherited: usize,
    unresolved: usize,
    unsupported: usize,
    skipped: usize,
) {
    let counters = &world
        .actor_catalog_result
        .as_ref()
        .expect("actor catalog must be built first")
        .counters;
    assert_eq!(counters.prepared, prepared);
    assert_eq!(counters.inherited, inherited);
    assert_eq!(counters.unresolved, unresolved);
    assert_eq!(counters.unsupported, unsupported);
    assert_eq!(counters.skipped, skipped);
}

#[then(regex = r#"^the actor catalog entries are ordered "([^"]*)"$"#)]
async fn then_actor_catalog_ordered(world: &mut BevyoutWorld, expected: String) {
    let expected_keys = expected
        .split(',')
        .map(|pair| {
            let (base, reference) = pair
                .trim()
                .split_once('/')
                .expect("expected a base/reference pair");
            (parse_hex(base), parse_hex(reference))
        })
        .collect::<Vec<(u32, u32)>>();
    let catalog = world
        .actor_catalog_result
        .as_ref()
        .expect("actor catalog must be built first");
    let actual_keys = catalog
        .entries
        .iter()
        .map(|entry| match entry {
            actor_catalog::ActorCatalogEntry::Prepared(blueprint) => {
                (blueprint.base_form_id, blueprint.reference_form_id)
            }
            actor_catalog::ActorCatalogEntry::Unresolved {
                base_form_id,
                reference_form_id,
                ..
            }
            | actor_catalog::ActorCatalogEntry::Unsupported {
                base_form_id,
                reference_form_id,
                ..
            }
            | actor_catalog::ActorCatalogEntry::Skipped {
                base_form_id,
                reference_form_id,
                ..
            } => (*base_form_id, *reference_form_id),
        })
        .collect::<Vec<(u32, u32)>>();
    assert_eq!(actual_keys, expected_keys);
}

#[then("serializing the actor catalog twice yields identical RON")]
async fn then_actor_catalog_ron_deterministic(world: &mut BevyoutWorld) {
    let catalog = world
        .actor_catalog_result
        .as_ref()
        .expect("actor catalog must be built first");
    let a = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default()).unwrap();
    let b = ron::ser::to_string_pretty(catalog, ron::ser::PrettyConfig::default()).unwrap();
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------
// actor_conversion.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^actor skeleton \"([^\"]*)\"$"#)]
async fn given_actor_skeleton(world: &mut BevyoutWorld, skeleton: String) {
    world.actor_skeleton = skeleton;
}

#[given(regex = r#"^actor visual inputs \"([^\"]*)\"$"#)]
async fn given_actor_visual_inputs(world: &mut BevyoutWorld, inputs: String) {
    world.actor_visual_inputs = inputs
        .split(',')
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect();
}

#[when("the actor conversion inputs are canonicalized")]
async fn when_actor_conversion_inputs_are_canonicalized(world: &mut BevyoutWorld) {
    world.actor_assembly = assets::canonical_actor_assembly(
        (!world.actor_skeleton.is_empty()).then(|| world.actor_skeleton.clone()),
        world.actor_visual_inputs.clone(),
    );
}

#[then(regex = r#"^the actor reference skeleton is \"([^\"]*)\"$"#)]
async fn then_actor_reference_skeleton_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .actor_assembly
            .as_ref()
            .map(|value| value.skeleton.as_str()),
        Some(expected.as_str())
    );
}

#[then(regex = r#"^the actor visual inputs are \"([^\"]*)\"$"#)]
async fn then_actor_visual_inputs_are(world: &mut BevyoutWorld, expected: String) {
    let expected = expected.split(',').map(str::to_owned).collect::<Vec<_>>();
    assert_eq!(
        world
            .actor_assembly
            .as_ref()
            .map(|value| value.visual_inputs.as_slice()),
        Some(expected.as_slice())
    );
}

#[then(regex = r#"^the actor converter profile is \"([^\"]*)\"$"#)]
async fn then_actor_converter_profile_is(_world: &mut BevyoutWorld, expected: String) {
    assert_eq!(assets::NATIVE_ACTOR_CONVERTER_REVISION, expected);
}

#[given(regex = r#"^actor gear record kinds \"([^\"]*)\"$"#)]
async fn given_actor_gear_record_kinds(world: &mut BevyoutWorld, kinds: String) {
    world.actor_gear_kinds = kinds.split(',').map(str::to_owned).collect();
}

#[when("actor visual gear is selected")]
async fn when_actor_visual_gear_is_selected(world: &mut BevyoutWorld) {
    world.retained_actor_gear_kinds = world
        .actor_gear_kinds
        .iter()
        .filter(|kind| assets::actor_visual_gear_kind(kind))
        .cloned()
        .collect();
}

#[then(regex = r#"^the retained actor gear record kinds are \"([^\"]*)\"$"#)]
async fn then_retained_actor_gear_record_kinds_are(world: &mut BevyoutWorld, kinds: String) {
    let expected = kinds.split(',').map(str::to_owned).collect::<Vec<_>>();
    assert_eq!(world.retained_actor_gear_kinds, expected);
}

#[given(
    regex = r#"^apparel 0x([0-9a-fA-F]+) has male worn \"([^\"]*)\" female worn \"([^\"]*)\" male world \"([^\"]*)\" female world \"([^\"]*)\" mask 0x([0-9a-fA-F]+) rating ([0-9.]+) max condition (\d+) current condition (\S+) value (-?\d+)$"#
)]
#[allow(clippy::too_many_arguments)]
async fn given_actor_apparel(
    world: &mut BevyoutWorld,
    form_id: String,
    male_worn: String,
    female_worn: String,
    male_world: String,
    female_world: String,
    mask: String,
    rating: f32,
    max_condition: u32,
    current_condition: String,
    value: i32,
) {
    world
        .actor_apparel_candidates
        .push(actor_appearance::ApparelCandidate {
            form_id: parse_hex(&form_id),
            male_worn: (!male_worn.is_empty()).then_some(male_worn),
            female_worn: (!female_worn.is_empty()).then_some(female_worn),
            male_world: (!male_world.is_empty()).then_some(male_world),
            female_world: (!female_world.is_empty()).then_some(female_world),
            biped_slot_mask: u32::from_str_radix(&mask, 16).unwrap(),
            base_armor_rating: rating,
            max_condition: Some(max_condition),
            current_condition: (current_condition != "full")
                .then(|| current_condition.parse().unwrap()),
            value,
        });
}

#[given(regex = r#"^worn model \"([^\"]*)\" is unavailable$"#)]
async fn given_worn_model_unavailable(world: &mut BevyoutWorld, model: String) {
    world.unavailable_actor_models.insert(model);
}

#[when(regex = r"^spawn apparel is selected for a (male|female) actor$")]
async fn when_spawn_apparel_is_selected(world: &mut BevyoutWorld, sex: String) {
    world.actor_outfit = Some(actor_appearance::select_spawn_outfit(
        &world.actor_apparel_candidates,
        sex == "female",
        |model| !world.unavailable_actor_models.contains(model),
    ));
}

#[then(regex = r#"^worn apparel models are \"([^\"]*)\"$"#)]
async fn then_worn_apparel_models_are(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .actor_outfit
        .as_ref()
        .expect("spawn outfit must be selected")
        .worn
        .iter()
        .map(|item| item.model_path.as_str())
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[then(regex = r"^occupied actor biped slots are 0x([0-9a-fA-F]+)$")]
async fn then_occupied_actor_biped_slots_are(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .actor_outfit
            .as_ref()
            .expect("spawn outfit must be selected")
            .occupied_slots,
        u32::from_str_radix(&expected, 16).unwrap()
    );
}

#[then(regex = r"^race body part (\d+) is (visible|hidden) (?:under|by) the outfit$")]
async fn then_race_body_part_visibility(world: &mut BevyoutWorld, index: u32, expected: String) {
    let visible = actor_appearance::race_body_part_visible(
        index,
        world
            .actor_outfit
            .as_ref()
            .expect("spawn outfit must be selected")
            .occupied_slots,
    );
    assert_eq!(visible, expected == "visible");
}

#[then(regex = r"^actor partition flags 0x([0-9a-fA-F]+) are (visible|hidden)$")]
async fn then_actor_partition_visibility(
    _world: &mut BevyoutWorld,
    flags: String,
    expected: String,
) {
    assert_eq!(
        actor_appearance::partition_is_editor_visible(u16::from_str_radix(&flags, 16).unwrap()),
        expected == "visible"
    );
}

// ---------------------------------------------------------------------
// nav_graph.feature (issue #111, M4 wave 2) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

/// One synthetic `NAVI` `NVMI` fixture entry (a plugin either carries one
/// of these or a deleted-override record).
#[derive(Debug, Clone, Copy)]
struct NaviFixtureEntry {
    form_id: u32,
    navmesh_form_id: u32,
    location_form_id: u32,
    grid_x: i16,
    grid_y: i16,
}

fn nav_subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u16).to_le_bytes());
    result.extend_from_slice(data);
    result
}

fn nav_record(signature: &[u8; 4], flags: u32, form_id: u32, data: &[u8]) -> Vec<u8> {
    let mut result = signature.to_vec();
    result.extend_from_slice(&(data.len() as u32).to_le_bytes());
    result.extend_from_slice(&flags.to_le_bytes());
    result.extend_from_slice(&form_id.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(data);
    result
}

fn nav_group(label: u32, group_type: i32, children: &[u8]) -> Vec<u8> {
    let mut result = b"GRUP".to_vec();
    result.extend_from_slice(&((children.len() + 24) as u32).to_le_bytes());
    result.extend_from_slice(&label.to_le_bytes());
    result.extend_from_slice(&group_type.to_le_bytes());
    result.extend_from_slice(&[0; 8]);
    result.extend_from_slice(children);
    result
}

fn nav_tes4(masters: &[&str]) -> Vec<u8> {
    let mut data = Vec::new();
    for master in masters {
        data.extend(nav_subrecord(b"MAST", format!("{master}\0").as_bytes()));
        data.extend(nav_subrecord(b"DATA", &[0; 8]));
    }
    nav_record(b"TES4", 0, 0, &data)
}

fn nav_navi_payload(entry: NaviFixtureEntry) -> Vec<u8> {
    let mut nvmi = vec![0_u8; 4]; // leading undocumented "Unknown" field
    nvmi.extend_from_slice(&entry.navmesh_form_id.to_le_bytes());
    nvmi.extend_from_slice(&entry.location_form_id.to_le_bytes());
    nvmi.extend_from_slice(&entry.grid_x.to_le_bytes());
    nvmi.extend_from_slice(&entry.grid_y.to_le_bytes());
    [
        nav_subrecord(b"NVER", &12_u32.to_le_bytes()),
        nav_subrecord(b"NVMI", &nvmi),
    ]
    .concat()
}

fn nav_navm_part_mut<'a>(world: &'a mut BevyoutWorld, signature: &str) -> &'a mut Vec<u8> {
    if !world
        .nav_navm_parts
        .iter()
        .any(|(existing, _)| existing == signature)
    {
        world
            .nav_navm_parts
            .push((signature.to_string(), Vec::new()));
    }
    &mut world
        .nav_navm_parts
        .iter_mut()
        .find(|(existing, _)| existing == signature)
        .expect("part was just inserted")
        .1
}

fn nav_parse_i16_triple(list: &str) -> [i16; 3] {
    let values = list
        .split(',')
        .map(|value| {
            value
                .trim()
                .parse::<i16>()
                .unwrap_or_else(|error| panic!("invalid i16 {value:?}: {error}"))
        })
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 3, "expected exactly three values in {list:?}");
    [values[0], values[1], values[2]]
}

fn nav_parsed_navmesh<'a>(
    world: &'a BevyoutWorld,
    form_hex: &str,
) -> &'a openmw_esm4::NavMeshRecord {
    let form_id = parse_hex(form_hex);
    world
        .nav_parsed
        .as_ref()
        .expect("the content set must be parsed first")
        .navmeshes
        .iter()
        .find(|navmesh| navmesh.form_id == form_id)
        .unwrap_or_else(|| panic!("no parsed NAVM {form_hex}"))
}

fn nav_graph_mesh_input_mut<'a>(
    world: &'a mut BevyoutWorld,
    form_hex: &str,
) -> &'a mut nav_graph::NavGraphMeshInput {
    let form_id = parse_hex(form_hex);
    world
        .nav_graph_inputs
        .meshes
        .iter_mut()
        .find(|mesh| mesh.form_id == form_id)
        .unwrap_or_else(|| panic!("nav graph mesh {form_hex} was not created first"))
}

fn nav_graph_result_mesh<'a>(
    world: &'a BevyoutWorld,
    form_hex: &str,
) -> &'a nav_graph::PreparedNavMesh {
    let form_id = parse_hex(form_hex);
    world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .meshes
        .iter()
        .find(|mesh| mesh.form_id == form_id)
        .unwrap_or_else(|| panic!("no prepared nav mesh {form_hex}"))
}

#[given(regex = r"^a plugin cell 0x([0-9a-fA-F]+)$")]
async fn given_nav_plugin_cell(world: &mut BevyoutWorld, hex: String) {
    world.nav_cell_form_id = parse_hex(&hex);
}

#[given(regex = r"^a NAVM 0x([0-9a-fA-F]+) in the cell with version (\d+)$")]
async fn given_navm_in_cell(world: &mut BevyoutWorld, hex: String, version: u32) {
    world.nav_navm_form_id = parse_hex(&hex);
    nav_navm_part_mut(world, "NVER").extend_from_slice(&version.to_le_bytes());
}

#[given(
    regex = r"^the NAVM declares counts vertices (\d+) triangles (\d+) external (\d+) cover (\d+) doors (\d+)$"
)]
async fn given_navm_counts(
    world: &mut BevyoutWorld,
    vertices: u32,
    triangles: u32,
    external: u32,
    cover: u32,
    doors: u32,
) {
    let cell = world.nav_cell_form_id;
    let data = nav_navm_part_mut(world, "DATA");
    data.extend_from_slice(&cell.to_le_bytes());
    for count in [vertices, triangles, external, cover, doors] {
        data.extend_from_slice(&count.to_le_bytes());
    }
}

#[given(regex = r#"^the NAVM has vertices "([^"]*)"$"#)]
async fn given_navm_vertices(world: &mut BevyoutWorld, list: String) {
    let mut payload = Vec::new();
    for vertex in list.split(';') {
        for value in vertex.split(',') {
            let value = value
                .trim()
                .parse::<f32>()
                .unwrap_or_else(|error| panic!("invalid f32 {value:?}: {error}"));
            payload.extend_from_slice(&value.to_le_bytes());
        }
    }
    nav_navm_part_mut(world, "NVVX").extend_from_slice(&payload);
}

#[given(
    regex = r"^the NAVM has triangle ([-\d,\s]+) with edges ([-\d,\s]+) and flags 0x([0-9a-fA-F]+)$"
)]
async fn given_navm_triangle(
    world: &mut BevyoutWorld,
    vertices: String,
    edges: String,
    flags_hex: String,
) {
    let mut payload = Vec::new();
    for value in nav_parse_i16_triple(&vertices) {
        payload.extend_from_slice(&value.to_le_bytes());
    }
    for value in nav_parse_i16_triple(&edges) {
        payload.extend_from_slice(&value.to_le_bytes());
    }
    payload.extend_from_slice(&parse_hex(&flags_hex).to_le_bytes());
    nav_navm_part_mut(world, "NVTR").extend_from_slice(&payload);
}

#[given(regex = r#"^the NAVM has cover ids "([^"]*)"$"#)]
async fn given_navm_cover(world: &mut BevyoutWorld, list: String) {
    let mut payload = Vec::new();
    for value in list.split(',') {
        let value = value
            .trim()
            .parse::<i16>()
            .unwrap_or_else(|error| panic!("invalid i16 {value:?}: {error}"));
        payload.extend_from_slice(&value.to_le_bytes());
    }
    nav_navm_part_mut(world, "NVCA").extend_from_slice(&payload);
}

#[given(regex = r"^the NAVM has door 0x([0-9a-fA-F]+) at triangle (\d+)$")]
async fn given_navm_door(world: &mut BevyoutWorld, door_hex: String, triangle: u16) {
    let door = parse_hex(&door_hex);
    let payload = [
        door.to_le_bytes().as_slice(),
        triangle.to_le_bytes().as_slice(),
        &[0, 0], // unused
    ]
    .concat();
    nav_navm_part_mut(world, "NVDP").extend_from_slice(&payload);
}

#[given(regex = r"^the NAVM has a grid with divisor (\d+)$")]
async fn given_navm_grid(world: &mut BevyoutWorld, divisor: u32) {
    let mut payload = divisor.to_le_bytes().to_vec();
    for value in [140.0_f32, 140.0, -70.0, -70.0, 0.0, 70.0, 70.0, 0.0] {
        payload.extend_from_slice(&value.to_le_bytes());
    }
    nav_navm_part_mut(world, "NVGD").extend_from_slice(&payload);
}

#[given(regex = r"^the NAVM has an external connection to 0x([0-9a-fA-F]+) at triangle (\d+)$")]
async fn given_navm_external(world: &mut BevyoutWorld, target_hex: String, triangle: u16) {
    let target = parse_hex(&target_hex);
    let payload = [
        [0_u8; 4].as_slice(), // leading undocumented "Unknown" field
        target.to_le_bytes().as_slice(),
        triangle.to_le_bytes().as_slice(),
    ]
    .concat();
    nav_navm_part_mut(world, "NVEX").extend_from_slice(&payload);
}

#[given(regex = r"^the NAVM NVTR payload is truncated by (\d+) bytes$")]
async fn given_navm_truncated(world: &mut BevyoutWorld, bytes: usize) {
    let part = nav_navm_part_mut(world, "NVTR");
    let new_len = part.len().checked_sub(bytes).expect("NVTR too short");
    part.truncate(new_len);
}

#[given(
    regex = r"^the plugin has a NAVI 0x([0-9a-fA-F]+) entry linking NAVM 0x([0-9a-fA-F]+) to location 0x([0-9a-fA-F]+) grid (-?\d+),(-?\d+)$"
)]
async fn given_navi_first(
    world: &mut BevyoutWorld,
    form_hex: String,
    navm_hex: String,
    location_hex: String,
    grid_x: i16,
    grid_y: i16,
) {
    world.nav_navi_first = Some(NaviFixtureEntry {
        form_id: parse_hex(&form_hex),
        navmesh_form_id: parse_hex(&navm_hex),
        location_form_id: parse_hex(&location_hex),
        grid_x,
        grid_y,
    });
}

#[given(
    regex = r"^a second plugin has a NAVI 0x([0-9a-fA-F]+) entry linking NAVM 0x([0-9a-fA-F]+) to location 0x([0-9a-fA-F]+) grid (-?\d+),(-?\d+)$"
)]
async fn given_navi_second(
    world: &mut BevyoutWorld,
    form_hex: String,
    navm_hex: String,
    location_hex: String,
    grid_x: i16,
    grid_y: i16,
) {
    world.nav_navi_second = Some(NaviFixtureEntry {
        form_id: parse_hex(&form_hex),
        navmesh_form_id: parse_hex(&navm_hex),
        location_form_id: parse_hex(&location_hex),
        grid_x,
        grid_y,
    });
}

#[given(regex = r"^a second plugin deletes the NAVI 0x([0-9a-fA-F]+) record$")]
async fn given_navi_second_deleted(world: &mut BevyoutWorld, form_hex: String) {
    world.nav_navi_second_deleted = Some(parse_hex(&form_hex));
}

#[when(regex = r"^the content set is parsed for cell 0x([0-9a-fA-F]+)$")]
async fn when_nav_content_set_parsed(world: &mut BevyoutWorld, cell_hex: String) {
    let cell = parse_hex(&cell_hex);
    let mut first = nav_tes4(&[]);
    first.extend(nav_record(
        b"CELL",
        0,
        world.nav_cell_form_id,
        &[
            nav_subrecord(b"EDID", b"NavCell\0"),
            nav_subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    if world.nav_navm_form_id != 0 {
        let payload = world
            .nav_navm_parts
            .iter()
            .map(|(signature, data)| {
                let signature: [u8; 4] = signature
                    .as_bytes()
                    .try_into()
                    .expect("subrecord signatures are 4 bytes");
                nav_subrecord(&signature, data)
            })
            .collect::<Vec<_>>()
            .concat();
        first.extend(nav_group(
            world.nav_cell_form_id,
            6,
            &nav_record(b"NAVM", 0, world.nav_navm_form_id, &payload),
        ));
    }
    if let Some(entry) = world.nav_navi_first {
        // Issue #113: the adapter feature appends island-tail bytes to the
        // first entry's NVMI subrecord; empty for the #111 scenarios.
        first.extend(nav_record(
            b"NAVI",
            0,
            entry.form_id,
            &nav_navi_payload_with_tail(entry, &world.nav_navi_first_tail),
        ));
    }

    let mut second = None;
    if let Some(entry) = world.nav_navi_second {
        let mut bytes = nav_tes4(&["Fallout3.esm"]);
        bytes.extend(nav_record(
            b"NAVI",
            0,
            entry.form_id,
            &nav_navi_payload(entry),
        ));
        second = Some(bytes);
    } else if let Some(form_id) = world.nav_navi_second_deleted {
        let mut bytes = nav_tes4(&["Fallout3.esm"]);
        // 0x20 is the ESM4 record-header "deleted" flag
        // (openmw_esm4::RECORD_DELETED).
        bytes.extend(nav_record(b"NAVI", 0x20, form_id, &[]));
        second = Some(bytes);
    }

    let mut sources = vec![openmw_esm4::PluginSource {
        name: "Fallout3.esm",
        bytes: &first,
    }];
    if let Some(bytes) = second.as_ref() {
        sources.push(openmw_esm4::PluginSource {
            name: "Update.esp",
            bytes,
        });
    }
    let parsed = openmw_esm4::parse_content_set(&sources, &CellSelector::FormId(cell))
        .expect("synthetic content set must parse");
    world.nav_parsed = Some(parsed);
}

#[then(
    regex = r"^the parsed navmesh 0x([0-9a-fA-F]+) has version (\d+) and owner cell 0x([0-9a-fA-F]+)$"
)]
async fn then_parsed_navmesh_version_owner(
    world: &mut BevyoutWorld,
    form_hex: String,
    version: u32,
    cell_hex: String,
) {
    let navmesh = nav_parsed_navmesh(world, &form_hex);
    assert_eq!(navmesh.version, Some(version));
    assert_eq!(navmesh.cell_form_id, Some(parse_hex(&cell_hex)));
}

#[then(regex = r"^the parsed navmesh 0x([0-9a-fA-F]+) has (\d+) vertices and (\d+) triangles$")]
async fn then_parsed_navmesh_counts(
    world: &mut BevyoutWorld,
    form_hex: String,
    vertices: usize,
    triangles: usize,
) {
    let navmesh = nav_parsed_navmesh(world, &form_hex);
    assert_eq!(navmesh.vertices.len(), vertices);
    assert_eq!(navmesh.triangles.len(), triangles);
}

#[then(regex = r"^parsed navmesh 0x([0-9a-fA-F]+) triangle (\d+) has flags 0x([0-9a-fA-F]+)$")]
async fn then_parsed_triangle_flags(
    world: &mut BevyoutWorld,
    form_hex: String,
    index: usize,
    flags_hex: String,
) {
    let navmesh = nav_parsed_navmesh(world, &form_hex);
    assert_eq!(navmesh.triangles[index].flags, parse_hex(&flags_hex));
}

#[then(regex = r#"^the parsed navmesh 0x([0-9a-fA-F]+) has cover ids "([^"]*)"$"#)]
async fn then_parsed_cover(world: &mut BevyoutWorld, form_hex: String, list: String) {
    let expected = list
        .split(',')
        .map(|value| value.trim().parse::<i16>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        nav_parsed_navmesh(world, &form_hex).cover_triangle_ids,
        expected
    );
}

#[then(
    regex = r"^the parsed navmesh 0x([0-9a-fA-F]+) has door 0x([0-9a-fA-F]+) at triangle (\d+)$"
)]
async fn then_parsed_door(
    world: &mut BevyoutWorld,
    form_hex: String,
    door_hex: String,
    triangle: u16,
) {
    let navmesh = nav_parsed_navmesh(world, &form_hex);
    let door = parse_hex(&door_hex);
    assert!(
        navmesh
            .doors
            .iter()
            .any(|entry| entry.door_reference_form_id == Some(door) && entry.triangle == triangle),
        "no door {door_hex} at triangle {triangle} in {:?}",
        navmesh.doors
    );
}

#[then(regex = r"^the parsed navmesh 0x([0-9a-fA-F]+) has grid divisor (\d+)$")]
async fn then_parsed_grid(world: &mut BevyoutWorld, form_hex: String, divisor: u32) {
    let navmesh = nav_parsed_navmesh(world, &form_hex);
    assert_eq!(
        navmesh.grid.as_ref().expect("grid must decode").divisor,
        divisor
    );
}

#[then(
    regex = r"^the parsed navmesh 0x([0-9a-fA-F]+) has an external connection to 0x([0-9a-fA-F]+) at triangle (\d+)$"
)]
async fn then_parsed_external(
    world: &mut BevyoutWorld,
    form_hex: String,
    target_hex: String,
    triangle: u16,
) {
    let navmesh = nav_parsed_navmesh(world, &form_hex);
    let target = parse_hex(&target_hex);
    assert!(
        navmesh
            .external_connections
            .iter()
            .any(|entry| entry.target_navmesh_form_id == Some(target)
                && entry.triangle == triangle),
        "no external connection to {target_hex} at triangle {triangle} in {:?}",
        navmesh.external_connections
    );
}

#[then("the content set has no NAVM diagnostics")]
async fn then_no_navm_diagnostics(world: &mut BevyoutWorld) {
    let parsed = world
        .nav_parsed
        .as_ref()
        .expect("the content set must be parsed first");
    let navm_diagnostics = parsed
        .diagnostics
        .iter()
        .filter(|message| message.contains("NAVM"))
        .collect::<Vec<_>>();
    assert!(
        navm_diagnostics.is_empty(),
        "unexpected NAVM diagnostics: {navm_diagnostics:?}"
    );
}

#[then(regex = r#"^the content set diagnostics include "([^"]*)"$"#)]
async fn then_content_set_diagnostics_include(world: &mut BevyoutWorld, expected: String) {
    let parsed = world
        .nav_parsed
        .as_ref()
        .expect("the content set must be parsed first");
    assert!(
        parsed.diagnostics.contains(&expected),
        "expected diagnostic {expected:?} in {:?}",
        parsed.diagnostics
    );
}

#[then(
    regex = r"^the navigation singleton links NAVM 0x([0-9a-fA-F]+) to location 0x([0-9a-fA-F]+) grid (-?\d+),(-?\d+)$"
)]
async fn then_navigation_singleton(
    world: &mut BevyoutWorld,
    navm_hex: String,
    location_hex: String,
    grid_x: i16,
    grid_y: i16,
) {
    let navigation = world
        .nav_parsed
        .as_ref()
        .expect("the content set must be parsed first")
        .navigation
        .as_ref()
        .expect("a navigation singleton must be captured");
    assert_eq!(navigation.entries.len(), 1, "{:?}", navigation.entries);
    let entry = &navigation.entries[0];
    assert_eq!(entry.navmesh_form_id, Some(parse_hex(&navm_hex)));
    assert_eq!(entry.location_form_id, Some(parse_hex(&location_hex)));
    assert_eq!(entry.grid_x, grid_x);
    assert_eq!(entry.grid_y, grid_y);
}

#[then("there is no navigation singleton")]
async fn then_no_navigation_singleton(world: &mut BevyoutWorld) {
    let parsed = world
        .nav_parsed
        .as_ref()
        .expect("the content set must be parsed first");
    assert!(parsed.navigation.is_none(), "{:?}", parsed.navigation);
}

#[given(regex = r"^a nav graph mesh 0x([0-9a-fA-F]+) for cell 0x([0-9a-fA-F]+)$")]
async fn given_nav_graph_mesh(world: &mut BevyoutWorld, form_hex: String, cell_hex: String) {
    world
        .nav_graph_inputs
        .meshes
        .push(nav_graph::NavGraphMeshInput {
            form_id: parse_hex(&form_hex),
            cell_form_id: Some(parse_hex(&cell_hex)),
            ..nav_graph::NavGraphMeshInput::default()
        });
}

#[given(regex = r"^mesh 0x([0-9a-fA-F]+) has source vertex (-?[\d.]+), (-?[\d.]+), (-?[\d.]+)$")]
async fn given_nav_graph_vertex(
    world: &mut BevyoutWorld,
    form_hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    nav_graph_mesh_input_mut(world, &form_hex)
        .vertices
        .push(nav_graph::NavGraphVertexInput { source: [x, y, z] });
}

#[given(regex = r"^mesh 0x([0-9a-fA-F]+) has triangle ([-\d,\s]+) with edges ([-\d,\s]+)$")]
async fn given_nav_graph_triangle(
    world: &mut BevyoutWorld,
    form_hex: String,
    vertices: String,
    edges: String,
) {
    let vertex_indices = nav_parse_i16_triple(&vertices).map(i32::from);
    let edge_neighbors = nav_parse_i16_triple(&edges).map(i32::from);
    nav_graph_mesh_input_mut(world, &form_hex)
        .triangles
        .push(nav_graph::NavGraphTriangleInput {
            vertex_indices,
            edge_neighbors,
            flags: 0,
        });
}

#[given(regex = r"^mesh 0x([0-9a-fA-F]+) has a door 0x([0-9a-fA-F]+) at triangle (\d+)$")]
async fn given_nav_graph_door(
    world: &mut BevyoutWorld,
    form_hex: String,
    door_hex: String,
    triangle: u32,
) {
    let door_reference_form_id = Some(parse_hex(&door_hex));
    nav_graph_mesh_input_mut(world, &form_hex)
        .doors
        .push(nav_graph::NavGraphDoorInput {
            door_reference_form_id,
            triangle_index: triangle,
        });
}

#[given(
    regex = r"^mesh 0x([0-9a-fA-F]+) has an external connection to 0x([0-9a-fA-F]+) at triangle (\d+)$"
)]
async fn given_nav_graph_external(
    world: &mut BevyoutWorld,
    form_hex: String,
    target_hex: String,
    triangle: u32,
) {
    let target_navmesh_form_id = Some(parse_hex(&target_hex));
    nav_graph_mesh_input_mut(world, &form_hex)
        .external_connections
        .push(nav_graph::NavGraphExternalInput {
            target_navmesh_form_id,
            triangle_index: triangle,
        });
}

#[given(
    regex = r"^mesh 0x([0-9a-fA-F]+) has an external connection with no target at triangle (\d+)$"
)]
async fn given_nav_graph_external_no_target(
    world: &mut BevyoutWorld,
    form_hex: String,
    triangle: u32,
) {
    nav_graph_mesh_input_mut(world, &form_hex)
        .external_connections
        .push(nav_graph::NavGraphExternalInput {
            target_navmesh_form_id: None,
            triangle_index: triangle,
        });
}

#[given(
    regex = r"^a nav graph NAVI entry links mesh 0x([0-9a-fA-F]+) to location 0x([0-9a-fA-F]+) grid (-?\d+),(-?\d+)$"
)]
async fn given_nav_graph_navi_entry(
    world: &mut BevyoutWorld,
    form_hex: String,
    location_hex: String,
    grid_x: i16,
    grid_y: i16,
) {
    world
        .nav_graph_inputs
        .navi_entries
        .push(nav_graph::NavGraphNaviEntryInput {
            navmesh_form_id: Some(parse_hex(&form_hex)),
            location_form_id: Some(parse_hex(&location_hex)),
            grid_x,
            grid_y,
        });
}

#[when(regex = r"^the nav graph is built for cell 0x([0-9a-fA-F]+)$")]
async fn when_nav_graph_built(world: &mut BevyoutWorld, cell_hex: String) {
    world.nav_graph_inputs.cell_form_id = parse_hex(&cell_hex);
    world.nav_graph_result = Some(nav_graph::build_nav_graph(&world.nav_graph_inputs));
}

#[then(
    regex = r"^mesh 0x([0-9a-fA-F]+) vertex (\d+) is (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) in Bevy metres$"
)]
async fn then_nav_graph_vertex_bevy(
    world: &mut BevyoutWorld,
    form_hex: String,
    index: usize,
    x: f32,
    y: f32,
    z: f32,
) {
    let vertex = nav_graph_result_mesh(world, &form_hex).vertices[index];
    for (actual, expected) in vertex.iter().zip([x, y, z]) {
        assert!(
            (actual - expected).abs() < 1e-5,
            "expected vertex {index} to be [{x}, {y}, {z}], got {vertex:?}"
        );
    }
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) polygon (\d+) edge (\d+) neighbours polygon (\d+)$")]
async fn then_nav_graph_adjacency(
    world: &mut BevyoutWorld,
    form_hex: String,
    polygon: usize,
    edge: usize,
    neighbor: u32,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    assert_eq!(mesh.polygons[polygon].adjacency[edge], Some(neighbor));
}

#[then("the nav graph has no diagnostics")]
async fn then_nav_graph_no_diagnostics(world: &mut BevyoutWorld) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    assert!(graph.diagnostics.is_empty(), "{:?}", graph.diagnostics);
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) has door 0x([0-9a-fA-F]+) at polygon (\d+)$")]
async fn then_nav_graph_door(
    world: &mut BevyoutWorld,
    form_hex: String,
    door_hex: String,
    polygon: u32,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    let door = parse_hex(&door_hex);
    assert!(
        mesh.doors
            .iter()
            .any(|entry| entry.door_reference_form_id == Some(door)
                && entry.triangle_index == polygon),
        "no door {door_hex} at polygon {polygon} in {:?}",
        mesh.doors
    );
}

#[then(
    regex = r"^mesh 0x([0-9a-fA-F]+) has an external connection to 0x([0-9a-fA-F]+) at polygon (\d+)$"
)]
async fn then_nav_graph_external(
    world: &mut BevyoutWorld,
    form_hex: String,
    target_hex: String,
    polygon: u32,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    let target = parse_hex(&target_hex);
    assert!(
        mesh.external_connections
            .iter()
            .any(|entry| entry.target_navmesh_form_id == Some(target)
                && entry.triangle_index == polygon),
        "no external connection to {target_hex} at polygon {polygon} in {:?}",
        mesh.external_connections
    );
}

#[then(
    regex = r"^the nav graph counters are meshes (\d+) polygons (\d+) vertices (\d+) doors (\d+) external (\d+)$"
)]
async fn then_nav_graph_counters(
    world: &mut BevyoutWorld,
    meshes: usize,
    polygons: usize,
    vertices: usize,
    doors: usize,
    external: usize,
) {
    let counters = &world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .counters;
    assert_eq!(counters.meshes, meshes);
    assert_eq!(counters.polygons, polygons);
    assert_eq!(counters.vertices, vertices);
    assert_eq!(counters.doors, doors);
    assert_eq!(counters.external_connections, external);
}

#[then(regex = r#"^the nav graph has an? "(warning|error)" diagnostic containing "([^"]*)"$"#)]
async fn then_nav_graph_diagnostic(world: &mut BevyoutWorld, severity: String, expected: String) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    assert!(
        graph
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == severity
                && diagnostic.message.contains(expected.as_str())),
        "expected a {severity} diagnostic containing {expected:?} in {:?}",
        graph.diagnostics
    );
}

#[then(regex = r#"^the nav graph meshes are ordered "([^"]*)"$"#)]
async fn then_nav_graph_ordered(world: &mut BevyoutWorld, expected: String) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    let actual = graph
        .meshes
        .iter()
        .map(|mesh| mesh.form_id)
        .collect::<Vec<_>>();
    assert_eq!(actual, parse_hex_list(&expected));
}

#[then("serializing the nav graph twice yields identical RON")]
async fn then_nav_graph_ron_deterministic(world: &mut BevyoutWorld) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    let a = ron::ser::to_string_pretty(graph, ron::ser::PrettyConfig::default()).unwrap();
    let b = ron::ser::to_string_pretty(graph, ron::ser::PrettyConfig::default()).unwrap();
    assert_eq!(a, b);
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) grid is (-?\d+),(-?\d+)$")]
async fn then_nav_graph_grid(world: &mut BevyoutWorld, form_hex: String, grid_x: i16, grid_y: i16) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    assert_eq!(
        mesh.grid,
        Some(nav_graph::PreparedNavGrid {
            x: grid_x,
            y: grid_y
        })
    );
}

// ---------------------------------------------------------------------
// nav_backend.feature (issue #112, M4 wave 3) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

fn nav_backend_mesh_mut<'a>(
    world: &'a mut BevyoutWorld,
    form_hex: &str,
) -> &'a mut landmass_graph::MeshInput {
    let form_id = parse_hex(form_hex);
    if !world
        .nav_backend_meshes
        .iter()
        .any(|mesh| mesh.form_id == form_id)
    {
        world.nav_backend_meshes.push(landmass_graph::MeshInput {
            form_id,
            ..Default::default()
        });
    }
    world
        .nav_backend_meshes
        .iter_mut()
        .find(|mesh| mesh.form_id == form_id)
        .expect("mesh was just inserted")
}

fn parse_bevy_landmass_agent_state(name: &str) -> bevy_landmass::AgentState {
    use bevy_landmass::AgentState::*;
    match name {
        "Idle" => Idle,
        "Moving" => Moving,
        "ReachedTarget" => ReachedTarget,
        "ReachedAnimationLink" => ReachedAnimationLink,
        "UsingAnimationLink" => UsingAnimationLink,
        "AgentNotOnNavMesh" => AgentNotOnNavMesh,
        "TargetNotOnNavMesh" => TargetNotOnNavMesh,
        "NoPath" => NoPath,
        "Paused" => Paused,
        other => panic!("unknown landmass agent state {other:?}"),
    }
}

#[given(regex = r"^a landmass mesh 0x([0-9a-fA-F]+)$")]
async fn given_landmass_mesh(world: &mut BevyoutWorld, form_hex: String) {
    nav_backend_mesh_mut(world, &form_hex);
}

#[given(
    regex = r"^landmass mesh 0x([0-9a-fA-F]+) has vertex (\d+) at (-?[\d.]+), (-?[\d.]+), (-?[\d.]+)$"
)]
async fn given_landmass_vertex(
    world: &mut BevyoutWorld,
    form_hex: String,
    index: usize,
    x: f32,
    y: f32,
    z: f32,
) {
    let mesh = nav_backend_mesh_mut(world, &form_hex);
    assert_eq!(
        mesh.vertices.len(),
        index,
        "vertices must be given in order starting at 0"
    );
    mesh.vertices.push([x, y, z]);
}

#[given(
    regex = r"^landmass mesh 0x([0-9a-fA-F]+) has polygon (\d+) with vertices (\d+),(\d+),(\d+)$"
)]
async fn given_landmass_polygon(
    world: &mut BevyoutWorld,
    form_hex: String,
    index: u32,
    a: u32,
    b: u32,
    c: u32,
) {
    let mesh = nav_backend_mesh_mut(world, &form_hex);
    mesh.polygons.push(landmass_graph::PolygonInput {
        index,
        vertex_indices: [a, b, c],
        is_water: false,
        is_preferred_pathing: false,
    });
}

#[given(regex = r"^landmass mesh 0x([0-9a-fA-F]+) polygon (\d+) is water$")]
async fn given_landmass_polygon_water(world: &mut BevyoutWorld, form_hex: String, index: u32) {
    let mesh = nav_backend_mesh_mut(world, &form_hex);
    let polygon = mesh
        .polygons
        .iter_mut()
        .find(|polygon| polygon.index == index)
        .expect("polygon must be given first");
    polygon.is_water = true;
}

#[given(regex = r"^landmass mesh 0x([0-9a-fA-F]+) has a door 0x([0-9a-fA-F]+) at polygon (\d+)$")]
async fn given_landmass_door(
    world: &mut BevyoutWorld,
    form_hex: String,
    door_hex: String,
    triangle_index: u32,
) {
    let door_form_id = parse_hex(&door_hex);
    let mesh = nav_backend_mesh_mut(world, &form_hex);
    mesh.doors.push(landmass_graph::DoorInput {
        triangle_index,
        door_reference_form_id: Some(door_form_id),
    });
}

#[when("the mesh is converted to a landmass navigation mesh")]
async fn when_landmass_mesh_converted(world: &mut BevyoutWorld) {
    let mesh = world
        .nav_backend_meshes
        .first()
        .expect("a landmass mesh must be given first")
        .clone();
    world.nav_backend_build_result = Some(landmass_graph::build_navigation_mesh(
        &mesh,
        &[],
        &std::collections::BTreeMap::new(),
        &std::collections::BTreeMap::new(),
    ));
}

#[then("the landmass conversion produces a navigation mesh")]
async fn then_landmass_conversion_produces_mesh(world: &mut BevyoutWorld) {
    let result = world
        .nav_backend_build_result
        .as_ref()
        .expect("conversion must run first");
    assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
}

#[then("the landmass conversion produces no navigation mesh")]
async fn then_landmass_conversion_produces_no_mesh(world: &mut BevyoutWorld) {
    let result = world
        .nav_backend_build_result
        .as_ref()
        .expect("conversion must run first");
    assert!(result.nav_mesh.is_none());
}

#[then("the landmass conversion has no diagnostics")]
async fn then_landmass_conversion_no_diagnostics(world: &mut BevyoutWorld) {
    let result = world
        .nav_backend_build_result
        .as_ref()
        .expect("conversion must run first");
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
}

#[then(
    regex = r#"^the landmass conversion has an? "(warning|error)" diagnostic containing "([^"]*)"$"#
)]
async fn then_landmass_conversion_diagnostic(
    world: &mut BevyoutWorld,
    severity: String,
    needle: String,
) {
    let result = world
        .nav_backend_build_result
        .as_ref()
        .expect("conversion must run first");
    let expected_severity = match severity.as_str() {
        "warning" => landmass_graph::Severity::Warning,
        "error" => landmass_graph::Severity::Error,
        other => panic!("unknown severity {other:?}"),
    };
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.severity == expected_severity && d.message.contains(&needle)),
        "{:?}",
        result.diagnostics
    );
}

#[when("the door-link descriptors are extracted")]
async fn when_door_link_descriptors_extracted(world: &mut BevyoutWorld) {
    world.nav_backend_descriptors = Some(landmass_graph::door_link_descriptors(
        &world.nav_backend_meshes,
    ));
}

#[when("the door-link descriptors are extracted again")]
async fn when_door_link_descriptors_extracted_again(world: &mut BevyoutWorld) {
    world.nav_backend_second_descriptors = Some(landmass_graph::door_link_descriptors(
        &world.nav_backend_meshes,
    ));
}

#[then(regex = r"^there (?:is|are) (\d+) door-link descriptors?$")]
async fn then_door_link_descriptor_count(world: &mut BevyoutWorld, count: usize) {
    let descriptors = world
        .nav_backend_descriptors
        .as_ref()
        .expect("descriptors must be extracted first");
    assert_eq!(descriptors.len(), count);
}

#[then(
    regex = r"^door-link descriptor (\d+) links door 0x([0-9a-fA-F]+) between mesh 0x([0-9a-fA-F]+) and mesh 0x([0-9a-fA-F]+)$"
)]
async fn then_door_link_descriptor_links(
    world: &mut BevyoutWorld,
    index: usize,
    door_hex: String,
    mesh_a_hex: String,
    mesh_b_hex: String,
) {
    let descriptors = world
        .nav_backend_descriptors
        .as_ref()
        .expect("descriptors must be extracted first");
    let descriptor = &descriptors[index];
    assert_eq!(descriptor.door_form_id, parse_hex(&door_hex));
    assert_eq!(descriptor.side_a.mesh_form_id, parse_hex(&mesh_a_hex));
    assert_eq!(descriptor.side_b.mesh_form_id, parse_hex(&mesh_b_hex));
}

#[then("both door-link descriptor extractions are identical")]
async fn then_door_link_descriptor_extractions_identical(world: &mut BevyoutWorld) {
    let first = world
        .nav_backend_descriptors
        .as_ref()
        .expect("first extraction must run first");
    let second = world
        .nav_backend_second_descriptors
        .as_ref()
        .expect("second extraction must run first");
    assert_eq!(first, second);
}

#[then(regex = r#"^landmass agent state "([A-Za-z]+)" maps to nav agent status "([a-z]+)"$"#)]
async fn then_landmass_agent_state_maps(
    _world: &mut BevyoutWorld,
    state_name: String,
    status_name: String,
) {
    let state = parse_bevy_landmass_agent_state(&state_name);
    let status = landmass_graph::map_agent_state(state);
    assert_eq!(status.as_str(), status_name);
}

#[given("a fresh door-link state")]
async fn given_fresh_door_link_state(world: &mut BevyoutWorld) {
    world.nav_backend_door_link_state = door_link::DoorLinkState::Idle;
}

#[when(regex = r"^the door-link reaches door 0x([0-9a-fA-F]+)$")]
async fn when_door_link_reaches(world: &mut BevyoutWorld, door_hex: String) {
    let door_form_id = parse_hex(&door_hex);
    world.nav_backend_door_link_state = door_link::transition(
        world.nav_backend_door_link_state,
        door_link::DoorLinkEvent::LinkReached {
            door_form_id,
            destination: door_link::LinkDestination::IntraCell,
        },
    );
}

#[when(regex = r"^the door-link ticks with the door (open|closed)$")]
async fn when_door_link_ticks(world: &mut BevyoutWorld, door_open: String) {
    world.nav_backend_door_link_state = door_link::transition(
        world.nav_backend_door_link_state,
        door_link::DoorLinkEvent::Tick {
            door_open: door_open == "open",
        },
    );
}

#[when(regex = r"^the door-link ticks (\d+) times with the door closed$")]
async fn when_door_link_ticks_n(world: &mut BevyoutWorld, count: u32) {
    for _ in 0..count {
        world.nav_backend_door_link_state = door_link::transition(
            world.nav_backend_door_link_state,
            door_link::DoorLinkEvent::Tick { door_open: false },
        );
    }
}

#[when("the door-link traversal completes")]
async fn when_door_link_traversal_completes(world: &mut BevyoutWorld) {
    world.nav_backend_door_link_state = door_link::transition(
        world.nav_backend_door_link_state,
        door_link::DoorLinkEvent::TraversalComplete,
    );
}

#[then(regex = r"^the door-link state is paused for door 0x([0-9a-fA-F]+)$")]
async fn then_door_link_paused(world: &mut BevyoutWorld, door_hex: String) {
    let door_form_id = parse_hex(&door_hex);
    match world.nav_backend_door_link_state {
        door_link::DoorLinkState::Paused {
            door_form_id: actual,
            ..
        } => assert_eq!(actual, door_form_id),
        other => panic!("expected Paused, got {other:?}"),
    }
}

#[then(regex = r"^the door-link state is traversing door 0x([0-9a-fA-F]+)$")]
async fn then_door_link_traversing(world: &mut BevyoutWorld, door_hex: String) {
    let door_form_id = parse_hex(&door_hex);
    match world.nav_backend_door_link_state {
        door_link::DoorLinkState::Traversing {
            door_form_id: actual,
            ..
        } => assert_eq!(actual, door_form_id),
        other => panic!("expected Traversing, got {other:?}"),
    }
}

#[then("the door-link state is idle")]
async fn then_door_link_idle(world: &mut BevyoutWorld) {
    assert_eq!(
        world.nav_backend_door_link_state,
        door_link::DoorLinkState::Idle
    );
}

#[then(regex = r"^the door-link state is failed for door 0x([0-9a-fA-F]+)$")]
async fn then_door_link_failed(world: &mut BevyoutWorld, door_hex: String) {
    let door_form_id = parse_hex(&door_hex);
    assert_eq!(
        world.nav_backend_door_link_state,
        door_link::DoorLinkState::Failed { door_form_id }
    );
}

// ---------------------------------------------------------------------
// nav_adapter.feature (issue #113, M4 wave 4) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

/// Like `nav_navi_payload`, with `tail` bytes appended inside the NVMI
/// subrecord after the fixed 16-byte header (the island tail under test).
fn nav_navi_payload_with_tail(entry: NaviFixtureEntry, tail: &[u8]) -> Vec<u8> {
    let mut nvmi = vec![0_u8; 4]; // leading undocumented "Unknown" field
    nvmi.extend_from_slice(&entry.navmesh_form_id.to_le_bytes());
    nvmi.extend_from_slice(&entry.location_form_id.to_le_bytes());
    nvmi.extend_from_slice(&entry.grid_x.to_le_bytes());
    nvmi.extend_from_slice(&entry.grid_y.to_le_bytes());
    nvmi.extend_from_slice(tail);
    [
        nav_subrecord(b"NVER", &12_u32.to_le_bytes()),
        nav_subrecord(b"NVMI", &nvmi),
    ]
    .concat()
}

fn nav_adapter_parse_f32_triple(list: &str) -> [f32; 3] {
    let values = list
        .split(',')
        .map(|value| {
            value
                .trim()
                .parse::<f32>()
                .unwrap_or_else(|error| panic!("invalid f32 {value:?}: {error}"))
        })
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 3, "expected exactly three values in {list:?}");
    [values[0], values[1], values[2]]
}

fn nav_adapter_push_f32_3(bytes: &mut Vec<u8>, value: [f32; 3]) {
    for component in value {
        bytes.extend_from_slice(&component.to_le_bytes());
    }
}

fn nav_adapter_navigation_entry<'a>(
    world: &'a BevyoutWorld,
    navm_hex: &str,
) -> &'a openmw_esm4::NaviInfoEntry {
    let form_id = parse_hex(navm_hex);
    world
        .nav_parsed
        .as_ref()
        .expect("the content set must be parsed first")
        .navigation
        .as_ref()
        .expect("a navigation singleton must be captured")
        .entries
        .iter()
        .find(|entry| entry.navmesh_form_id == Some(form_id))
        .unwrap_or_else(|| panic!("no navigation entry for NAVM {navm_hex}"))
}

#[given(
    regex = r#"^the NAVI entry has an island tail with center ([-\d.,\s]+) bounds ([-\d.,\s]+) to ([-\d.,\s]+) and vertices "([^"]*)" triangle ([\d,\s]+)$"#
)]
async fn given_navi_island_tail(
    world: &mut BevyoutWorld,
    center: String,
    bounds_min: String,
    bounds_max: String,
    vertices: String,
    triangle: String,
) {
    let mut tail = Vec::new();
    nav_adapter_push_f32_3(&mut tail, nav_adapter_parse_f32_triple(&center));
    nav_adapter_push_f32_3(&mut tail, nav_adapter_parse_f32_triple(&bounds_min));
    nav_adapter_push_f32_3(&mut tail, nav_adapter_parse_f32_triple(&bounds_max));
    let vertex_values: Vec<[f32; 3]> = vertices
        .split(';')
        .map(nav_adapter_parse_f32_triple)
        .collect();
    let triangle_indices: Vec<u16> = triangle
        .split(',')
        .map(|value| {
            value
                .trim()
                .parse::<u16>()
                .unwrap_or_else(|error| panic!("invalid u16 {value:?}: {error}"))
        })
        .collect();
    assert_eq!(triangle_indices.len(), 3);
    tail.extend_from_slice(&(vertex_values.len() as u16).to_le_bytes());
    tail.extend_from_slice(&1_u16.to_le_bytes()); // one triangle
    for vertex in vertex_values {
        nav_adapter_push_f32_3(&mut tail, vertex);
    }
    for index in triangle_indices {
        tail.extend_from_slice(&index.to_le_bytes());
    }
    tail.extend_from_slice(&[0, 0, 0, 0]); // trailing field
    world.nav_navi_first_tail = tail;
}

#[given(regex = r"^the NAVI entry has a bare tail with center ([-\d.,\s]+)$")]
async fn given_navi_bare_tail(world: &mut BevyoutWorld, center: String) {
    let mut tail = Vec::new();
    nav_adapter_push_f32_3(&mut tail, nav_adapter_parse_f32_triple(&center));
    tail.extend_from_slice(&[0, 0, 0, 0]); // trailing field
    world.nav_navi_first_tail = tail;
}

#[given(regex = r"^the NAVI entry has a truncated island tail declaring (\d+) vertices$")]
async fn given_navi_truncated_tail(world: &mut BevyoutWorld, declared: u16) {
    let mut tail = Vec::new();
    nav_adapter_push_f32_3(&mut tail, [0.0, 0.0, 0.0]); // center
    nav_adapter_push_f32_3(&mut tail, [0.0, 0.0, 0.0]); // bounds min
    nav_adapter_push_f32_3(&mut tail, [1.0, 1.0, 1.0]); // bounds max
    tail.extend_from_slice(&declared.to_le_bytes());
    tail.extend_from_slice(&0_u16.to_le_bytes());
    nav_adapter_push_f32_3(&mut tail, [2.0, 2.0, 2.0]); // only one vertex present
    world.nav_navi_first_tail = tail;
}

#[then(regex = r"^the navigation entry for NAVM 0x([0-9a-fA-F]+) has center ([-\d.,\s]+)$")]
async fn then_navi_entry_center(world: &mut BevyoutWorld, navm_hex: String, center: String) {
    let entry = nav_adapter_navigation_entry(world, &navm_hex);
    assert_eq!(entry.center, Some(nav_adapter_parse_f32_triple(&center)));
}

#[then(
    regex = r"^the navigation entry for NAVM 0x([0-9a-fA-F]+) has bounds ([-\d.,\s]+) to ([-\d.,\s]+)$"
)]
async fn then_navi_entry_bounds(
    world: &mut BevyoutWorld,
    navm_hex: String,
    bounds_min: String,
    bounds_max: String,
) {
    let entry = nav_adapter_navigation_entry(world, &navm_hex);
    let bounds = entry.bounds.expect("entry must decode bounds");
    assert_eq!(bounds.min, nav_adapter_parse_f32_triple(&bounds_min));
    assert_eq!(bounds.max, nav_adapter_parse_f32_triple(&bounds_max));
}

#[then(
    regex = r"^the navigation entry for NAVM 0x([0-9a-fA-F]+) has an island with (\d+) vertices and (\d+) triangles$"
)]
async fn then_navi_entry_island(
    world: &mut BevyoutWorld,
    navm_hex: String,
    vertices: usize,
    triangles: usize,
) {
    let entry = nav_adapter_navigation_entry(world, &navm_hex);
    let island = entry.island.as_ref().expect("entry must decode an island");
    assert_eq!(island.vertices.len(), vertices);
    assert_eq!(island.triangles.len(), triangles);
}

#[then(regex = r"^the navigation entry for NAVM 0x([0-9a-fA-F]+) has no island$")]
async fn then_navi_entry_no_island(world: &mut BevyoutWorld, navm_hex: String) {
    let entry = nav_adapter_navigation_entry(world, &navm_hex);
    assert!(entry.island.is_none(), "{:?}", entry.island);
    assert!(entry.bounds.is_none(), "{:?}", entry.bounds);
}

#[then(
    regex = r"^the navigation entry for NAVM 0x([0-9a-fA-F]+) retains (no )?undecoded tail bytes$"
)]
async fn then_navi_entry_tail_retention(
    world: &mut BevyoutWorld,
    navm_hex: String,
    no_retention: String,
) {
    let entry = nav_adapter_navigation_entry(world, &navm_hex);
    if no_retention.is_empty() {
        assert!(!entry.tail.is_empty(), "expected retained tail bytes");
    } else {
        assert!(
            entry.tail.is_empty(),
            "unexpected tail bytes {:?}",
            entry.tail
        );
    }
}

#[then(regex = r"^the nav graph has (\d+) cross-mesh merges?$")]
async fn then_nav_graph_merge_count(world: &mut BevyoutWorld, count: usize) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    assert_eq!(graph.mesh_merges.len(), count, "{:?}", graph.mesh_merges);
    assert_eq!(graph.counters.mesh_merges, count);
}

#[then(
    regex = r"^cross-mesh merge (\d+) connects mesh 0x([0-9a-fA-F]+) polygon (\d+) to mesh 0x([0-9a-fA-F]+) polygon (\d+)$"
)]
async fn then_nav_graph_merge_connects(
    world: &mut BevyoutWorld,
    index: usize,
    mesh_a_hex: String,
    triangle_a: u32,
    mesh_b_hex: String,
    triangle_b: u32,
) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    let merge = graph.mesh_merges[index];
    assert_eq!(merge.mesh_a_form_id, parse_hex(&mesh_a_hex));
    assert_eq!(merge.triangle_a, triangle_a);
    assert_eq!(merge.mesh_b_form_id, parse_hex(&mesh_b_hex));
    assert_eq!(merge.triangle_b, triangle_b);
}

#[then("building the nav graph again yields identical cross-mesh merges")]
async fn then_nav_graph_merges_deterministic(world: &mut BevyoutWorld) {
    let second = nav_graph::build_nav_graph(&world.nav_graph_inputs);
    let first = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    assert_eq!(first.mesh_merges, second.mesh_merges);
    world.nav_adapter_second_graph = Some(second);
}

#[given("no repath observations")]
async fn given_no_repath_observations(world: &mut BevyoutWorld) {
    world.nav_adapter_repath_observation = repath::RepathObservation::default();
    world.nav_adapter_repath_decision = None;
}

#[given(regex = r"^the repath observation ([a-z-]+)$")]
async fn given_repath_observation(world: &mut BevyoutWorld, observation: String) {
    let target = &mut world.nav_adapter_repath_observation;
    match observation.as_str() {
        "none" => {}
        "door-became-blocked" => target.door_became_blocked = true,
        "door-became-unblocked" => target.door_became_unblocked = true,
        "target-moved" => target.target_moved_beyond_tolerance = true,
        "agent-off-link" => target.agent_off_link = true,
        "destination-cell-unloaded" => target.destination_cell_unloaded = true,
        other => panic!("unknown repath observation {other:?}"),
    }
}

#[when("the repath decision is made")]
async fn when_repath_decision(world: &mut BevyoutWorld) {
    world.nav_adapter_repath_decision = Some(repath::decide(world.nav_adapter_repath_observation));
}

#[then(regex = r"^the repath decision is ([a-z-]+)$")]
async fn then_repath_decision(world: &mut BevyoutWorld, decision: String) {
    let expected = match decision.as_str() {
        "keep-route" => repath::RepathDecision::KeepRoute,
        "repath" => repath::RepathDecision::Repath,
        "fail" => repath::RepathDecision::Fail,
        other => panic!("unknown repath decision {other:?}"),
    };
    assert_eq!(world.nav_adapter_repath_decision, Some(expected));
}

#[given(regex = r"^a door that is (locked|unlocked) and (open|closed)$")]
async fn given_door_observation(world: &mut BevyoutWorld, locked: String, open: String) {
    world.nav_adapter_door_observation = repath::DoorObservation {
        locked: locked == "locked",
        open: open == "open",
    };
}

#[then(regex = r"^the door is (usable|not usable) for route planning$")]
async fn then_door_usability(world: &mut BevyoutWorld, usability: String) {
    let usable = repath::door_usable(world.nav_adapter_door_observation);
    assert_eq!(usable, usability == "usable");
}

#[when(regex = r"^the door-link reaches travel door 0x([0-9a-fA-F]+) to cell 0x([0-9a-fA-F]+)$")]
async fn when_door_link_reaches_travel(
    world: &mut BevyoutWorld,
    door_hex: String,
    cell_hex: String,
) {
    world.nav_backend_door_link_state = door_link::transition(
        world.nav_backend_door_link_state,
        door_link::DoorLinkEvent::LinkReached {
            door_form_id: parse_hex(&door_hex),
            destination: door_link::LinkDestination::Travel {
                destination_cell_form_id: parse_hex(&cell_hex),
            },
        },
    );
}

#[then(
    regex = r"^the door-link state is travel-reached for door 0x([0-9a-fA-F]+) to cell 0x([0-9a-fA-F]+)$"
)]
async fn then_door_link_travel_reached(
    world: &mut BevyoutWorld,
    door_hex: String,
    cell_hex: String,
) {
    assert_eq!(
        world.nav_backend_door_link_state,
        door_link::DoorLinkState::TravelReached {
            door_form_id: parse_hex(&door_hex),
            destination_cell_form_id: parse_hex(&cell_hex),
        }
    );
}

#[when("the single-sided doors are extracted")]
async fn when_single_sided_doors_extracted(world: &mut BevyoutWorld) {
    world.nav_adapter_single_sided = Some(landmass_graph::single_sided_doors(
        &world.nav_backend_meshes,
    ));
}

#[then(regex = r"^there (?:is|are) (\d+) single-sided doors?$")]
async fn then_single_sided_door_count(world: &mut BevyoutWorld, count: usize) {
    let doors = world
        .nav_adapter_single_sided
        .as_ref()
        .expect("single-sided doors must be extracted first");
    assert_eq!(doors.len(), count, "{doors:?}");
}

#[then(regex = r"^single-sided door (\d+) is door 0x([0-9a-fA-F]+) on mesh 0x([0-9a-fA-F]+)$")]
async fn then_single_sided_door(
    world: &mut BevyoutWorld,
    index: usize,
    door_hex: String,
    mesh_hex: String,
) {
    let doors = world
        .nav_adapter_single_sided
        .as_ref()
        .expect("single-sided doors must be extracted first");
    let door = doors[index];
    assert_eq!(door.door_form_id, parse_hex(&door_hex));
    assert_eq!(door.side.mesh_form_id, parse_hex(&mesh_hex));
}

#[given(
    regex = r"^a prepared merge connects mesh 0x([0-9a-fA-F]+) triangle (\d+) to mesh 0x([0-9a-fA-F]+) triangle (\d+)$"
)]
async fn given_prepared_merge(
    world: &mut BevyoutWorld,
    mesh_a_hex: String,
    triangle_a: u32,
    mesh_b_hex: String,
    triangle_b: u32,
) {
    // Issue #154 widened `MergeInput` with the validated portal interval.
    // Keep this legacy mesh/plumbing step on a valid non-zero interval so it
    // exercises a real runtime link; zero-length intervals are deliberately
    // rejected because landmass cannot build a finite animation-link bound.
    world
        .nav_adapter_merge_inputs
        .push(landmass_graph::MergeInput {
            mesh_a_form_id: parse_hex(&mesh_a_hex),
            triangle_a,
            mesh_b_form_id: parse_hex(&mesh_b_hex),
            triangle_b,
            interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            interval_b: [[3.0, 0.0, 0.0], [4.0, 0.0, 0.0]],
        });
}

#[when("the merge-link descriptors are resolved")]
async fn when_merge_links_resolved(world: &mut BevyoutWorld) {
    world.nav_adapter_merge_links = Some(landmass_graph::merge_link_descriptors(
        &world.nav_backend_meshes,
        &world.nav_adapter_merge_inputs,
    ));
}

#[then(regex = r"^there (?:is|are) (\d+) merge-link descriptors?$")]
async fn then_merge_link_count(world: &mut BevyoutWorld, count: usize) {
    let links = world
        .nav_adapter_merge_links
        .as_ref()
        .expect("merge links must be resolved first");
    assert_eq!(links.len(), count, "{links:?}");
}

#[then(
    regex = r"^merge-link descriptor (\d+) links mesh 0x([0-9a-fA-F]+) polygon (\d+) to mesh 0x([0-9a-fA-F]+) polygon (\d+)$"
)]
async fn then_merge_link_descriptor(
    world: &mut BevyoutWorld,
    index: usize,
    mesh_a_hex: String,
    polygon_a: u32,
    mesh_b_hex: String,
    polygon_b: u32,
) {
    let links = world
        .nav_adapter_merge_links
        .as_ref()
        .expect("merge links must be resolved first");
    let link = links[index];
    assert_eq!(link.side_a.mesh_form_id, parse_hex(&mesh_a_hex));
    assert_eq!(link.side_a.polygon_index, polygon_a);
    assert_eq!(link.side_b.mesh_form_id, parse_hex(&mesh_b_hex));
    assert_eq!(link.side_b.polygon_index, polygon_b);
}

// ---------------------------------------------------------------------
// nav_ledger.feature (issue #134, M4 wave 4) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

fn nav_ledger_parse_f32_triple(x: &str, y: &str, z: &str) -> [f32; 3] {
    let parse = |value: &str| {
        value
            .trim()
            .parse::<f32>()
            .unwrap_or_else(|error| panic!("invalid f32 {value:?}: {error}"))
    };
    [parse(x), parse(y), parse(z)]
}

/// Parses `known doors`'s argument: the literal `none`, or a comma-
/// separated list of `0x`-prefixed hex FormIDs.
fn nav_ledger_parse_known_doors(list: &str) -> std::collections::HashSet<u32> {
    if list.trim() == "none" {
        return std::collections::HashSet::new();
    }
    list.split(',')
        .map(|value| {
            let digits = value
                .trim()
                .strip_prefix("0x")
                .unwrap_or_else(|| panic!("expected a 0x-prefixed hex FormID, got {value:?}"));
            parse_hex(digits)
        })
        .collect()
}

#[given(
    regex = r"^a ledger entry for agent (\d+) in cell 0x([0-9a-fA-F]+) frozen at ([-\d.]+), ([-\d.]+), ([-\d.]+)$"
)]
async fn given_ledger_entry_frozen(
    world: &mut BevyoutWorld,
    agent_id: u32,
    cell_hex: String,
    x: String,
    y: String,
    z: String,
) {
    world.nav_ledger.record(ledger_policy::LedgerEntry {
        agent_id,
        cell_form_id: parse_hex(&cell_hex),
        spawn_kind: ledger_policy::SpawnKind::FrozenPosition {
            position: nav_ledger_parse_f32_triple(&x, &y, &z),
        },
        remaining_target: None,
    });
}

#[given(
    regex = r"^a ledger entry for agent (\d+) in cell 0x([0-9a-fA-F]+) with door marker 0x([0-9a-fA-F]+)$"
)]
async fn given_ledger_entry_door_marker(
    world: &mut BevyoutWorld,
    agent_id: u32,
    cell_hex: String,
    door_hex: String,
) {
    world.nav_ledger.record(ledger_policy::LedgerEntry {
        agent_id,
        cell_form_id: parse_hex(&cell_hex),
        spawn_kind: ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id: parse_hex(&door_hex),
        },
        remaining_target: None,
    });
}

#[when(regex = r"^the ledger is claimed for cell 0x([0-9a-fA-F]+) with known doors (.+)$")]
async fn when_ledger_claimed(world: &mut BevyoutWorld, cell_hex: String, doors: String) {
    let known = nav_ledger_parse_known_doors(&doors);
    let result = world
        .nav_ledger
        .claim_for_activation(parse_hex(&cell_hex), &known);
    world.nav_ledger_claim_result = Some(result);
}

#[then(regex = r"^(\d+) entr(?:y|ies) (?:is|are) restored$")]
async fn then_entries_restored(world: &mut BevyoutWorld, count: usize) {
    let result = world
        .nav_ledger_claim_result
        .as_ref()
        .expect("the ledger must be claimed first");
    assert_eq!(result.restored.len(), count);
}

#[then(regex = r"^(\d+) entr(?:y|ies) (?:is|are) stale$")]
async fn then_entries_stale(world: &mut BevyoutWorld, count: usize) {
    let result = world
        .nav_ledger_claim_result
        .as_ref()
        .expect("the ledger must be claimed first");
    assert_eq!(result.stale.len(), count);
}

#[then(regex = r"^restored entry (\d+) is agent (\d+) frozen at ([-\d.]+), ([-\d.]+), ([-\d.]+)$")]
async fn then_restored_entry_frozen(
    world: &mut BevyoutWorld,
    index: usize,
    agent_id: u32,
    x: String,
    y: String,
    z: String,
) {
    let result = world
        .nav_ledger_claim_result
        .as_ref()
        .expect("the ledger must be claimed first");
    let entry = result.restored[index];
    assert_eq!(entry.agent_id, agent_id);
    assert_eq!(
        entry.spawn_kind,
        ledger_policy::SpawnKind::FrozenPosition {
            position: nav_ledger_parse_f32_triple(&x, &y, &z),
        }
    );
}

#[then(regex = r"^restored entry (\d+) is agent (\d+) with door marker 0x([0-9a-fA-F]+)$")]
async fn then_restored_entry_door_marker(
    world: &mut BevyoutWorld,
    index: usize,
    agent_id: u32,
    door_hex: String,
) {
    let result = world
        .nav_ledger_claim_result
        .as_ref()
        .expect("the ledger must be claimed first");
    let entry = result.restored[index];
    assert_eq!(entry.agent_id, agent_id);
    assert_eq!(
        entry.spawn_kind,
        ledger_policy::SpawnKind::DoorMarker {
            destination_door_form_id: parse_hex(&door_hex),
        }
    );
}

#[then(
    regex = r"^stale entry (\d+) is agent (\d+) cell 0x([0-9a-fA-F]+) missing door 0x([0-9a-fA-F]+)$"
)]
async fn then_stale_entry(
    world: &mut BevyoutWorld,
    index: usize,
    agent_id: u32,
    cell_hex: String,
    door_hex: String,
) {
    let result = world
        .nav_ledger_claim_result
        .as_ref()
        .expect("the ledger must be claimed first");
    let entry = result.stale[index];
    assert_eq!(entry.agent_id, agent_id);
    assert_eq!(entry.cell_form_id, parse_hex(&cell_hex));
    assert_eq!(entry.missing_door_form_id, parse_hex(&door_hex));
}

#[then(regex = r"^the ledger still holds an entry for agent (\d+)$")]
async fn then_ledger_still_holds(world: &mut BevyoutWorld, agent_id: u32) {
    assert!(
        world.nav_ledger.entry_for(agent_id).is_some(),
        "expected agent {agent_id} to remain ledgered"
    );
}

#[given(regex = r"^the agent's active route door is (none|0x[0-9a-fA-F]+)$")]
async fn given_agent_active_route_door(world: &mut BevyoutWorld, door: String) {
    world.nav_ledger_route_door = if door == "none" {
        None
    } else {
        Some(parse_hex(
            door.strip_prefix("0x").expect("checked by regex"),
        ))
    };
}

#[when(regex = r"^the swap eligibility is decided for door 0x([0-9a-fA-F]+)$")]
async fn when_swap_eligibility_decided(world: &mut BevyoutWorld, door_hex: String) {
    let used_door = parse_hex(&door_hex);
    world.nav_ledger_eligibility = Some(ledger_policy::decide_swap_eligibility(
        world.nav_ledger_route_door,
        used_door,
    ));
}

#[then(regex = r"^the swap eligibility is (follow-through|freeze)$")]
async fn then_swap_eligibility(world: &mut BevyoutWorld, expected: String) {
    let expected = match expected.as_str() {
        "follow-through" => ledger_policy::SwapEligibility::FollowThrough,
        "freeze" => ledger_policy::SwapEligibility::Freeze,
        other => panic!("unknown eligibility {other:?}"),
    };
    assert_eq!(world.nav_ledger_eligibility, Some(expected));
}

// ---------------------------------------------------------------------
// nav_movement.feature (issue #114, M4 wave 5) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

#[given(
    regex = r"^a grounded observation with walkable plane (true|false) and stepped up (true|false)$"
)]
async fn given_grounded_observation(
    world: &mut BevyoutWorld,
    has_walkable_plane: String,
    stepped_up: String,
) {
    world.nav_movement_grounded_observation = movement_policy::GroundedObservation {
        has_walkable_plane: has_walkable_plane == "true",
        stepped_up: stepped_up == "true",
    };
}

#[when("the grounded decision is made")]
async fn when_grounded_decision_made(world: &mut BevyoutWorld) {
    world.nav_movement_grounded_decision = Some(movement_policy::decide_grounded(
        world.nav_movement_grounded_observation,
    ));
}

#[then(regex = r"^the agent is (not )?grounded$")]
async fn then_agent_is_grounded(world: &mut BevyoutWorld, negation: String) {
    let expected = negation.is_empty();
    assert_eq!(world.nav_movement_grounded_decision, Some(expected));
}

#[given(
    regex = r"^a velocity observation with desired speed ([\d.]+) and achieved speed ([\d.]+)$"
)]
async fn given_velocity_observation(
    world: &mut BevyoutWorld,
    desired_horizontal_speed: f32,
    achieved_horizontal_speed: f32,
) {
    world.nav_movement_velocity_observation = Some(movement_policy::VelocityObservation {
        desired_horizontal_speed,
        achieved_horizontal_speed,
    });
}

#[when("the collision outcome decision is made")]
async fn when_collision_outcome_decision_made(world: &mut BevyoutWorld) {
    let observation = world
        .nav_movement_velocity_observation
        .expect("a velocity observation must be given first");
    world.nav_movement_collision_outcome =
        Some(movement_policy::decide_collision_outcome(observation));
}

#[then(regex = r"^the collision outcome is (clear|blocked)$")]
async fn then_collision_outcome(world: &mut BevyoutWorld, expected: String) {
    let expected = match expected.as_str() {
        "clear" => movement_policy::CollisionOutcome::Clear,
        "blocked" => movement_policy::CollisionOutcome::Blocked,
        other => panic!("unknown collision outcome {other:?}"),
    };
    assert_eq!(world.nav_movement_collision_outcome, Some(expected));
}

#[given(
    regex = r"^a stuck observation with distance ([\d.]+), best distance ([\d.]+), ticks without progress (\d+), recovery active (true|false)$"
)]
async fn given_stuck_observation(
    world: &mut BevyoutWorld,
    distance_to_target: f32,
    best_distance_so_far: f32,
    ticks_without_progress: u32,
    recovery_active: String,
) {
    world.nav_movement_stuck_observation = Some(movement_policy::StuckObservation {
        distance_to_target,
        best_distance_so_far,
        ticks_without_progress,
        recovery_active: recovery_active == "true",
    });
}

#[when("the stuck decision is made")]
async fn when_stuck_decision_made(world: &mut BevyoutWorld) {
    let observation = world
        .nav_movement_stuck_observation
        .expect("a stuck observation must be given first");
    world.nav_movement_stuck_decision = Some(movement_policy::decide_stuck(observation));
}

#[then(regex = r"^the stuck decision is (progressing|start-recovery|recovery-pending|stuck)$")]
async fn then_stuck_decision(world: &mut BevyoutWorld, expected: String) {
    let expected = match expected.as_str() {
        "progressing" => movement_policy::StuckDecision::Progressing,
        "start-recovery" => movement_policy::StuckDecision::StartRecovery,
        "recovery-pending" => movement_policy::StuckDecision::RecoveryPending,
        "stuck" => movement_policy::StuckDecision::Stuck,
        other => panic!("unknown stuck decision {other:?}"),
    };
    assert_eq!(world.nav_movement_stuck_decision, Some(expected));
}

// ---------------------------------------------------------------------
// nav_door_gate.feature (issue #137, M4 wave 5) -- appended section, do
// not interleave.
// ---------------------------------------------------------------------

#[given(regex = r"^a mid-route door that is (open|closed) and (unlocked|locked)$")]
async fn given_mid_route_door_observation(world: &mut BevyoutWorld, open: String, locked: String) {
    world.nav_door_gate_observation = door_link::CrossingObservation {
        door_open: open == "open",
        door_locked: locked == "locked",
    };
}

#[then(regex = r"^the crossing gate is (pass|wait|blocked)$")]
async fn then_crossing_gate(world: &mut BevyoutWorld, expected: String) {
    let gate = door_link::crossing_gate(world.nav_door_gate_observation);
    let expected = match expected.as_str() {
        "pass" => door_link::CrossingGate::Pass,
        "wait" => door_link::CrossingGate::Wait,
        "blocked" => door_link::CrossingGate::Blocked,
        other => panic!("unknown crossing gate {other:?}"),
    };
    assert_eq!(gate, expected);
}

// ---------------------------------------------------------------------
// nav_movement.feature (issue #114 added scope, M4 wave 5) -- solve-rate
// divisor. Appended section, do not interleave.
// ---------------------------------------------------------------------

#[given(regex = r"^a solve step count of (\d+) and an interval of (\d+)$")]
async fn given_solve_step_and_interval(world: &mut BevyoutWorld, step_count: u64, interval: u32) {
    world.nav_solve_step = step_count;
    world.nav_solve_interval = interval;
}

#[when("the solve decision is made")]
async fn when_solve_decision_made(world: &mut BevyoutWorld) {
    world.nav_solve_decision = Some(movement_policy::should_solve(
        world.nav_solve_step,
        world.nav_solve_interval,
    ));
}

#[then(regex = r"^the solve decision is (solve|skip)$")]
async fn then_solve_decision(world: &mut BevyoutWorld, expected: String) {
    let expected = expected == "solve";
    assert_eq!(world.nav_solve_decision, Some(expected));
}

#[given(regex = r"^(\d+) steps since the last solve and an interval of (\d+)$")]
async fn given_steps_since_solve_and_interval(
    world: &mut BevyoutWorld,
    steps_since_solve: u32,
    interval: u32,
) {
    world.nav_solve_steps_since_solve = steps_since_solve;
    world.nav_solve_blend_interval = interval;
}

#[when("the solve blend fraction is computed")]
async fn when_solve_blend_fraction_computed(world: &mut BevyoutWorld) {
    world.nav_solve_blend_fraction = Some(movement_policy::solve_blend_fraction(
        world.nav_solve_steps_since_solve,
        world.nav_solve_blend_interval,
    ));
}

#[then(regex = r"^the solve blend fraction is ([\d.]+)$")]
async fn then_solve_blend_fraction(world: &mut BevyoutWorld, expected: f32) {
    assert_eq!(world.nav_solve_blend_fraction, Some(expected));
}

// --- #123 note text steps ---
//
// `openmw_esm4::Subrecord`/`FormIdResolver` fields are module-private, so
// (unlike a `#[cfg(test)]` module living inside `openmw_esm4` itself) this
// suite cannot construct them directly. It instead builds a minimal
// synthetic plugin (a `CELL` plus one `NOTE` base record) out of raw ESM4
// bytes -- the same `nav_subrecord`/`nav_record`/`nav_tes4` byte-level
// helpers `nav_graph.feature`'s steps already use -- and drives the real
// `openmw_esm4::parse_content_set` decode path end to end. The final
// scenario mirrors `vsa::prepare::items::prepared_stats`'s `Note` arm
// inline rather than calling it directly: `vsa::prepare::items` pulls in
// the full prepare-module dependency graph (`PreparedPlacement`, physics
// assets, …), which this suite does not otherwise include (see the module
// doc comment at the top of this file for the narrow-inclusion rationale).
// The real `prepared_stats` mapping is covered by
// `vsa::prepare::items::tests::note_text_carries_into_prepared_stats`.

const NOTE_TEXT_CELL_FORM_ID: u32 = 0x0090_0100;
const NOTE_TEXT_NOTE_FORM_ID: u32 = 0x0090_0101;

#[given(regex = r#"^a synthetic NOTE record with DATA type (\d+) and TNAM text "(.*)"$"#)]
async fn given_note_text_type(world: &mut BevyoutWorld, note_type: u8, text: String) {
    let mut tnam = text.into_bytes();
    tnam.push(0);
    world.note_record_data = [
        nav_subrecord(b"DATA", &[note_type]),
        nav_subrecord(b"TNAM", &tnam),
    ]
    .concat();
}

#[given(regex = r"^a synthetic NOTE record with DATA type (\d+) and TNAM formid 0x([0-9A-Fa-f]+)$")]
async fn given_note_formid_type(world: &mut BevyoutWorld, note_type: u8, form_id_hex: String) {
    let form_id = u32::from_str_radix(&form_id_hex, 16).expect("hex form id");
    world.note_record_data = [
        nav_subrecord(b"DATA", &[note_type]),
        nav_subrecord(b"TNAM", &form_id.to_le_bytes()),
    ]
    .concat();
}

#[given(regex = r#"^a synthetic NOTE record with no DATA subrecord and TNAM text "(.*)"$"#)]
async fn given_note_no_data(world: &mut BevyoutWorld, text: String) {
    let mut tnam = text.into_bytes();
    tnam.push(0);
    world.note_record_data = nav_subrecord(b"TNAM", &tnam);
}

#[when("the NOTE record is decoded")]
async fn when_note_decoded(world: &mut BevyoutWorld) {
    let mut plugin = nav_tes4(&[]);
    plugin.extend(nav_record(
        b"CELL",
        0,
        NOTE_TEXT_CELL_FORM_ID,
        &[
            nav_subrecord(b"EDID", b"NoteTextCell\0"),
            nav_subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    plugin.extend(nav_record(
        b"NOTE",
        0,
        NOTE_TEXT_NOTE_FORM_ID,
        &world.note_record_data,
    ));
    let sources = vec![openmw_esm4::PluginSource {
        name: "Fallout3.esm",
        bytes: &plugin,
    }];
    let parsed =
        openmw_esm4::parse_content_set(&sources, &CellSelector::FormId(NOTE_TEXT_CELL_FORM_ID))
            .expect("synthetic NOTE plugin must parse");
    world.note_base = parsed.bases.get(&NOTE_TEXT_NOTE_FORM_ID).cloned();
}

#[then(regex = r#"^the decoded note text is "(.*)"$"#)]
async fn then_note_text_is(world: &mut BevyoutWorld, expected: String) {
    match &world
        .note_base
        .as_ref()
        .expect("NOTE must be decoded")
        .item_stats
    {
        openmw_esm4::OpenMwItemStats::Note { text } => {
            assert_eq!(text.as_deref(), Some(expected.as_str()));
        }
        other => panic!("expected Note item stats, got {other:?}"),
    }
}

#[then("the decoded note text is absent")]
async fn then_note_text_absent(world: &mut BevyoutWorld) {
    match &world
        .note_base
        .as_ref()
        .expect("NOTE must be decoded")
        .item_stats
    {
        openmw_esm4::OpenMwItemStats::Note { text } => assert!(text.is_none()),
        other => panic!("expected Note item stats, got {other:?}"),
    }
}

#[when("the decoded base record is prepared into item catalog stats")]
async fn when_note_prepared_into_catalog_stats(world: &mut BevyoutWorld) {
    let stats = match &world
        .note_base
        .as_ref()
        .expect("NOTE must be decoded")
        .item_stats
    {
        openmw_esm4::OpenMwItemStats::Note { text } => {
            manifest::PreparedItemStats::Note { text: text.clone() }
        }
        other => panic!("expected Note item stats, got {other:?}"),
    };
    world.note_prepared_stats = Some(stats);
}

#[then(regex = r#"^the prepared catalog note text is "(.*)"$"#)]
async fn then_prepared_catalog_note_text_is(world: &mut BevyoutWorld, expected: String) {
    match world
        .note_prepared_stats
        .as_ref()
        .expect("catalog stats must be prepared")
    {
        manifest::PreparedItemStats::Note { text } => {
            assert_eq!(text.as_deref(), Some(expected.as_str()));
        }
        other => panic!("expected Note prepared stats, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// real_corpses.feature (issue #120, M4 wave 6). Appended section, do not
// interleave.
// --- #120 real corpses steps ---
// ---------------------------------------------------------------------

#[given(regex = r"^a real-corpses cell 0x([0-9a-fA-F]+)$")]
async fn given_corpse_cell(world: &mut BevyoutWorld, cell_hex: String) {
    world.corpse_cell_form_id = parse_hex(&cell_hex);
}

#[given(regex = r"^an ACHR reference 0x([0-9a-fA-F]+) of base 0x([0-9a-fA-F]+) that starts dead$")]
async fn given_corpse_achr_dead(world: &mut BevyoutWorld, reference_hex: String, base_hex: String) {
    world
        .corpse_achr_entries
        .push((parse_hex(&reference_hex), parse_hex(&base_hex), true));
}

#[given(regex = r"^a living ACHR reference 0x([0-9a-fA-F]+) of base 0x([0-9a-fA-F]+)$")]
async fn given_corpse_achr_living(
    world: &mut BevyoutWorld,
    reference_hex: String,
    base_hex: String,
) {
    world
        .corpse_achr_entries
        .push((parse_hex(&reference_hex), parse_hex(&base_hex), false));
}

// Issue #120 rework: the real FO3 starts-dead signal lives on the base
// `NPC_` record's own header flags, not the ACHR reference's (see
// features/real_corpses.feature's header comment for the real-data
// survey).
#[given(regex = r"^an NPC_ base 0x([0-9a-fA-F]+) that starts dead$")]
async fn given_corpse_npc_base_dead(world: &mut BevyoutWorld, base_hex: String) {
    world.corpse_npc_bases.push((parse_hex(&base_hex), true));
}

#[given(regex = r"^a living NPC_ base 0x([0-9a-fA-F]+)$")]
async fn given_corpse_npc_base_living(world: &mut BevyoutWorld, base_hex: String) {
    world.corpse_npc_bases.push((parse_hex(&base_hex), false));
}

#[when("the real-corpses content set is parsed")]
async fn when_corpse_content_set_parsed(world: &mut BevyoutWorld) {
    let mut bytes = nav_tes4(&[]);
    bytes.extend(nav_record(
        b"CELL",
        0,
        world.corpse_cell_form_id,
        &[
            nav_subrecord(b"EDID", b"RealCorpsesTestCell\0"),
            nav_subrecord(b"DATA", &[1]),
        ]
        .concat(),
    ));
    // `NPC_` base records are content-set-wide, not cell-scoped (unlike
    // ACHR/ACRE/REFR references), so they are emitted top-level here --
    // same as `fabricated_content`'s STAT/MISC/etc. bases in
    // `src/vsa/prepare/tests/mod.rs`, no GRUP wrapping needed.
    for (base_form_id, starts_dead) in &world.corpse_npc_bases {
        // 0x00080000 is the FO3-specific NPC_ record-header "starts dead"
        // bit (vsa::prepare::placements::NPC_STARTS_DEAD -- see that
        // constant's doc comment for the real-data survey against
        // Fallout3.esm) -- same inline-literal convention
        // `when_nav_content_set_parsed` above uses for `RECORD_DELETED`.
        let flags = if *starts_dead { 0x0008_0000 } else { 0 };
        let edid = nav_subrecord(b"EDID", b"CorpseFixture\0");
        bytes.extend(nav_record(b"NPC_", flags, *base_form_id, &edid));
    }
    let mut children = Vec::new();
    for (reference_form_id, base_form_id, starts_dead) in &world.corpse_achr_entries {
        // ACHR `DATA`: position (3x f32) + rotation (3x f32), 24 bytes --
        // `parse_reference` requires at least that length.
        let mut data = nav_subrecord(b"NAME", &base_form_id.to_le_bytes());
        data.extend(nav_subrecord(b"DATA", &[0_u8; 24]));
        // 0x00000200 is the ESM4 record-header bit OpenMW documents as
        // "starts dead" for ACHR (openmw_esm4's `RECORD_STARTS_DEAD` /
        // `Rec_StartDead`, `components/esm4/common.hpp`) -- kept here as
        // the secondary/harmless path's own round-trip coverage (real FO3
        // data never sets it; see `NPC_STARTS_DEAD`'s doc comment) -- same
        // inline-literal convention `when_nav_content_set_parsed` above
        // uses for `RECORD_DELETED`.
        let flags = if *starts_dead { 0x0000_0200 } else { 0 };
        children.extend(nav_record(b"ACHR", flags, *reference_form_id, &data));
    }
    bytes.extend(nav_group(world.corpse_cell_form_id, 6, &children));

    let sources = [openmw_esm4::PluginSource {
        name: "RealCorpses.esm",
        bytes: &bytes,
    }];
    let parsed =
        openmw_esm4::parse_content_set(&sources, &CellSelector::FormId(world.corpse_cell_form_id))
            .expect("synthetic real-corpses content set must parse");
    world.corpse_parsed = Some(parsed);
}

fn corpse_parsed_reference(world: &BevyoutWorld, form_id: u32) -> &openmw_esm4::ReferenceRecord {
    world
        .corpse_parsed
        .as_ref()
        .expect("the real-corpses content set must be parsed first")
        .references
        .iter()
        .find(|reference| reference.form_id == form_id)
        .unwrap_or_else(|| panic!("reference {form_id:08x} was not parsed"))
}

fn corpse_parsed_base(world: &BevyoutWorld, form_id: u32) -> &openmw_esm4::BaseRecord {
    world
        .corpse_parsed
        .as_ref()
        .expect("the real-corpses content set must be parsed first")
        .bases
        .get(&form_id)
        .unwrap_or_else(|| panic!("NPC_ base {form_id:08x} was not parsed"))
}

#[then(regex = r"^the parsed reference 0x([0-9a-fA-F]+) starts dead$")]
async fn then_parsed_reference_starts_dead(world: &mut BevyoutWorld, form_hex: String) {
    let reference = corpse_parsed_reference(world, parse_hex(&form_hex));
    assert_ne!(
        reference.flags & 0x0000_0200,
        0,
        "reference {form_hex} did not decode the starts-dead flag"
    );
}

#[then(regex = r"^the parsed reference 0x([0-9a-fA-F]+) does not start dead$")]
async fn then_parsed_reference_does_not_start_dead(world: &mut BevyoutWorld, form_hex: String) {
    let reference = corpse_parsed_reference(world, parse_hex(&form_hex));
    assert_eq!(
        reference.flags & 0x0000_0200,
        0,
        "reference {form_hex} unexpectedly decoded the starts-dead flag"
    );
}

#[then(regex = r"^the parsed NPC_ base 0x([0-9a-fA-F]+) starts dead$")]
async fn then_parsed_npc_base_starts_dead(world: &mut BevyoutWorld, form_hex: String) {
    let base = corpse_parsed_base(world, parse_hex(&form_hex));
    assert_ne!(
        base.record_flags & 0x0008_0000,
        0,
        "NPC_ base {form_hex} did not decode the starts-dead flag"
    );
}

#[then(regex = r"^the parsed NPC_ base 0x([0-9a-fA-F]+) does not start dead$")]
async fn then_parsed_npc_base_does_not_start_dead(world: &mut BevyoutWorld, form_hex: String) {
    let base = corpse_parsed_base(world, parse_hex(&form_hex));
    assert_eq!(
        base.record_flags & 0x0008_0000,
        0,
        "NPC_ base {form_hex} unexpectedly decoded the starts-dead flag"
    );
}

fn main() {
    // The cucumber driver future is large (every step's async state plus the
    // BevyoutWorld test state), which exceeds the default 1 MiB main-thread
    // stack on Windows. Run it on a worker thread with an explicit stack.
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            futures::executor::block_on(async {
                BevyoutWorld::cucumber()
                    .fail_on_skipped()
                    .run_and_exit("features")
                    .await;
            });
        })
        .expect("failed to spawn cucumber driver thread")
        .join()
        .expect("cucumber driver thread panicked");
}

// ---------------------------------------------------------------------
// native_conversion.feature -- bounded native worker policy.
// ---------------------------------------------------------------------

#[given("no NIF converter was explicitly requested")]
async fn given_no_converter_requested(world: &mut BevyoutWorld) {
    world.requested_converter = None;
}

#[when("the preparation converter is resolved")]
async fn when_converter_is_resolved(world: &mut BevyoutWorld) {
    world.resolved_converter = Some(converter_policy::resolve_converter_backend(
        world.requested_converter,
    ));
}

#[when(regex = r#"^the \"([^\"]*)\" NIF converter is explicitly requested$"#)]
async fn when_converter_is_explicitly_requested(world: &mut BevyoutWorld, converter: String) {
    world.requested_converter = Some(match converter.as_str() {
        "native" => converter_policy::ConverterBackend::Native,
        other => panic!("unknown converter {other:?}"),
    });
    world.resolved_converter = Some(converter_policy::resolve_converter_backend(
        world.requested_converter,
    ));
}

#[then(regex = r#"^the resolved preparation converter is \"([^\"]*)\"$"#)]
async fn then_resolved_converter_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .resolved_converter
            .expect("resolved converter")
            .as_str(),
        expected
    );
}

#[given(regex = r#"^a native GLB requires extensions \"([^\"]*)\"$"#)]
async fn given_native_glb_required_extensions(world: &mut BevyoutWorld, extensions: String) {
    world.required_gltf_extensions = extensions
        .split(',')
        .map(str::trim)
        .filter(|extension| !extension.is_empty())
        .map(str::to_owned)
        .collect();
}

#[when("the Rust bake validates its required glTF extensions")]
async fn when_rust_bake_validates_extensions(world: &mut BevyoutWorld) {
    world.unsupported_gltf_extensions = gltf_extension_policy::unsupported_required_extensions(
        world.required_gltf_extensions.iter().map(String::as_str),
    );
}

#[then("no required glTF extensions are unsupported")]
async fn then_no_required_extensions_are_unsupported(world: &mut BevyoutWorld) {
    assert!(world.unsupported_gltf_extensions.is_empty());
}

#[given(regex = r#"^native conversion outcomes \"([^\"]*)\"$"#)]
async fn given_native_conversion_outcomes(world: &mut BevyoutWorld, outcomes: String) {
    world.native_outcomes = outcomes
        .split(',')
        .map(|entry| {
            let (index, status) = entry.split_once(':').expect("index:status outcome");
            let status = match status {
                "converted" => native_policy::NativeJobStatus::Converted,
                "failed" => native_policy::NativeJobStatus::Failed,
                "unsupported" => native_policy::NativeJobStatus::Unsupported,
                other => panic!("unknown native status {other:?}"),
            };
            native_policy::NativeJobOutcome {
                index: index.parse().expect("numeric native job index"),
                model: entry.into(),
                status,
                stage: String::new(),
                error: None,
            }
        })
        .collect();
}

#[when("the native conversion batch is summarized")]
async fn when_native_batch_is_summarized(world: &mut BevyoutWorld) {
    world.native_summary = Some(native_policy::summarize_native_jobs(&world.native_outcomes));
    world.native_sorted_indices = native_policy::sorted_native_outcomes(&world.native_outcomes)
        .into_iter()
        .map(|outcome| outcome.index)
        .collect();
}

#[then(regex = r#"^the native conversion summary is \"([^\"]*)\"$"#)]
async fn then_native_summary_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.native_summary.expect("native summary").line(),
        expected
    );
}

#[then(regex = r#"^the native conversion outcome order is \"([^\"]*)\"$"#)]
async fn then_native_outcome_order_is(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .native_sorted_indices
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[given(regex = r"^(\d+) native conversion assets and (\d+) requested workers$")]
async fn given_native_asset_and_worker_count(
    world: &mut BevyoutWorld,
    assets: usize,
    workers: usize,
) {
    world.native_asset_count = assets;
    world.native_requested_workers = Some(workers);
    world.native_host_workers = std::thread::available_parallelism()
        .map(std::num::NonZeroUsize::get)
        .unwrap_or(1);
}

#[given(
    regex = r"^(\d+) native conversion assets, no requested workers, and (\d+) host processors$"
)]
async fn given_native_asset_and_host_worker_count(
    world: &mut BevyoutWorld,
    assets: usize,
    host_workers: usize,
) {
    world.native_asset_count = assets;
    world.native_requested_workers = None;
    world.native_host_workers = host_workers;
}

#[when("the native worker count is resolved")]
async fn when_native_worker_count_is_resolved(world: &mut BevyoutWorld) {
    world.native_worker_count = Some(native_policy::native_worker_count_with_host(
        world.native_requested_workers,
        world.native_asset_count,
        world.native_host_workers.max(1),
    ));
}

#[then(regex = r"^(\d+) native conversion workers are used$")]
async fn then_native_workers_are_used(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(world.native_worker_count, Some(expected));
}

// ---------------------------------------------------------------------
// actor_conversion.feature -- authored ragdoll sidecar v3. Appended
// section; do not interleave.
// ---------------------------------------------------------------------

#[given(
    regex = r"^an authored spherical actor joint with cone ([\d.]+) plane (-?[\d.]+) to (-?[\d.]+) twist (-?[\d.]+) to (-?[\d.]+) strength ([\d.]+)$"
)]
async fn given_authored_spherical_actor_joint(
    world: &mut BevyoutWorld,
    cone: f32,
    plane_lower: f32,
    plane_upper: f32,
    twist_lower: f32,
    twist_upper: f32,
    strength: f32,
) {
    world.actor_ragdoll_joint = Some(physics::PreparedPhysicsJoint {
        kind: "spherical".into(),
        body_a: 0,
        body_b: 1,
        anchor_a: [0.0, 1.0, 0.0],
        anchor_b: [0.0, 1.0, 0.0],
        frame_a_rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        frame_b_rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        cone_limit: Some(cone),
        plane_lower_limit: Some(plane_lower),
        plane_upper_limit: Some(plane_upper),
        twist_lower_limit: Some(twist_lower),
        twist_upper_limit: Some(twist_upper),
        malleable_strength: Some(strength),
        source: physics::PreparedPhysicsJointSource::Authored,
        ..Default::default()
    });
}

#[given("a synthetic fallback actor joint")]
async fn given_synthetic_fallback_actor_joint(world: &mut BevyoutWorld) {
    world.actor_ragdoll_joint = Some(physics::PreparedPhysicsJoint {
        source: physics::PreparedPhysicsJointSource::SyntheticFallback,
        ..Default::default()
    });
}

#[then("the actor physics sidecar schema is 3")]
async fn then_actor_physics_sidecar_schema_is_three(_world: &mut BevyoutWorld) {
    assert_eq!(physics::PHYSICS_ASSET_SCHEMA_VERSION, 3);
}

#[then("the actor joint has complete local frames")]
async fn then_actor_joint_has_complete_local_frames(world: &mut BevyoutWorld) {
    let joint = world
        .actor_ragdoll_joint
        .as_ref()
        .expect("actor ragdoll joint fixture");
    assert_eq!(joint.frame_a_rotation_xyzw, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(joint.frame_b_rotation_xyzw, [0.0, 0.0, 0.0, 1.0]);
}

#[then(
    regex = r"^the actor joint keeps plane (-?[\d.]+) to (-?[\d.]+) separate from twist (-?[\d.]+) to (-?[\d.]+)$"
)]
async fn then_actor_joint_keeps_plane_separate_from_twist(
    world: &mut BevyoutWorld,
    plane_lower: f32,
    plane_upper: f32,
    twist_lower: f32,
    twist_upper: f32,
) {
    let joint = world
        .actor_ragdoll_joint
        .as_ref()
        .expect("actor ragdoll joint fixture");
    assert_eq!(joint.plane_lower_limit, Some(plane_lower));
    assert_eq!(joint.plane_upper_limit, Some(plane_upper));
    assert_eq!(joint.twist_lower_limit, Some(twist_lower));
    assert_eq!(joint.twist_upper_limit, Some(twist_upper));
}

#[then(regex = r#"^the actor joint source is "([^"]*)"$"#)]
async fn then_actor_joint_source_is(world: &mut BevyoutWorld, expected: String) {
    let joint = world
        .actor_ragdoll_joint
        .as_ref()
        .expect("actor ragdoll joint fixture");
    let actual = match joint.source {
        physics::PreparedPhysicsJointSource::Authored => "Authored",
        physics::PreparedPhysicsJointSource::SyntheticFallback => "SyntheticFallback",
    };
    assert_eq!(actual, expected);
}

#[then("Blender ragdoll bodies and constraints use stable NIF source identities")]
async fn then_blender_ragdoll_uses_stable_source_identity(_world: &mut BevyoutWorld) {
    let script = assets::legacy_blender_preview_script();
    assert!(script.contains("bevyout_nif_body_block"));
    assert!(script.contains("body_a_key"));
    assert!(script.contains("body_b_key"));
    assert!(script.contains("resolve_authored_joint_body_groups"));
    assert!(!script.contains("_bevyout_body_group"));
}

#[given("an actor physics sidecar with duplicate body group IDs")]
async fn given_actor_sidecar_with_duplicate_body_ids(world: &mut BevyoutWorld) {
    let body = physics::PreparedPhysicsBody {
        group_id: 7,
        shapes: vec![physics::PreparedPhysicsShape::Sphere {
            center: [0.0; 3],
            radius: 0.25,
        }],
        ..Default::default()
    };
    world.actor_physics_asset = Some(physics::PreparedPhysicsAsset {
        schema_version: physics::PHYSICS_ASSET_SCHEMA_VERSION,
        source: physics::PreparedPhysicsSource::AuthoredHavok,
        bodies: vec![body.clone(), body],
        joints: Vec::new(),
    });
}

#[then("actor physics sidecar validation rejects duplicate body group IDs")]
async fn then_actor_sidecar_rejects_duplicate_body_ids(world: &mut BevyoutWorld) {
    let error = physics::validate_physics_asset(
        world
            .actor_physics_asset
            .as_ref()
            .expect("actor physics sidecar fixture"),
    )
    .unwrap_err();
    assert!(error.to_string().contains("duplicate body group IDs"));
}

#[then("non-ragdoll actor skin weights collapse to their nearest authored body ancestor")]
async fn then_actor_skin_weights_follow_authored_ragdoll(_world: &mut BevyoutWorld) {
    let script = assets::legacy_blender_preview_script();
    assert!(script.contains("actor_ragdoll_weight_target"));
    assert!(script.contains("collapse_actor_ragdoll_weights"));
    assert!(script.contains("target_group.add([vertex.index], weight, 'ADD')"));
    assert!(script.contains("source_group.remove([vertex.index])"));
}

// ---------------------------------------------------------------------
// actor_assembly.feature / actor_fallback.feature (#107, #108) -- appended
// section, do not interleave.
// ---------------------------------------------------------------------

#[given(
    regex = r#"^actor mesh part (Body|Head|Hair|Eyes) index (\d+) form 0x([0-9a-fA-F]+) model \"([^\"]+)\"$"#
)]
async fn given_actor_mesh_part(
    world: &mut BevyoutWorld,
    role: String,
    index: u32,
    form_id: String,
    model_path: String,
) {
    let role = match role.as_str() {
        "Body" => actor::ActorMeshRole::Body(index),
        "Head" => actor::ActorMeshRole::Head(index),
        "Hair" => actor::ActorMeshRole::Hair,
        "Eyes" => actor::ActorMeshRole::Eyes,
        other => panic!("unknown actor mesh role {other}"),
    };
    world.actor_mesh_parts.push(actor::AssembledMeshPart {
        name: format!("{role:?}"),
        source_form_id: Some(parse_hex(&form_id)),
        model_path,
        attachment_point: actor::ActorAttachmentPoint::Head,
        role,
        is_visible: true,
    });
}

#[when("actor mesh parts are canonicalized")]
async fn when_actor_mesh_parts_are_canonicalized(world: &mut BevyoutWorld) {
    actor::canonicalize_mesh_parts(&mut world.actor_mesh_parts);
}

#[then(regex = r#"^actor mesh roles are \"([^\"]*)\"$"#)]
async fn then_actor_mesh_roles_are(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .actor_mesh_parts
        .iter()
        .map(|part| part.role.label())
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[then("every actor mesh part attaches to Head")]
async fn then_every_actor_mesh_part_attaches_to_head(world: &mut BevyoutWorld) {
    assert!(
        world
            .actor_mesh_parts
            .iter()
            .all(|part| part.attachment_point == actor::ActorAttachmentPoint::Head)
    );
}

#[given(regex = r"^occupied actor apparel slots 0x([0-9a-fA-F]+)$")]
async fn given_occupied_actor_apparel_slots(world: &mut BevyoutWorld, slots: String) {
    world.actor_occupied_slots = parse_hex(&slots);
}

#[when("actor optional-part visibility is evaluated")]
async fn when_actor_optional_part_visibility_is_evaluated(world: &mut BevyoutWorld) {
    world.actor_hair_visible = Some(actor::hair_visible(world.actor_occupied_slots));
    world.actor_eyes_visible = Some(actor::eyes_visible(world.actor_occupied_slots));
}

#[then(regex = r"^actor hair is (visible|hidden)$")]
async fn then_actor_hair_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.actor_hair_visible, Some(expected == "visible"));
}

#[then(regex = r"^actor eyes are (visible|hidden)$")]
async fn then_actor_eyes_are(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.actor_eyes_visible, Some(expected == "visible"));
}

#[given(
    regex = r#"^actor weapon 0x([0-9a-fA-F]+) model \"([^\"]+)\" damage (\d+) value (-?\d+) available (yes|no)$"#
)]
async fn given_actor_weapon_candidate(
    world: &mut BevyoutWorld,
    form_id: String,
    model_path: String,
    damage: u16,
    value: i32,
    available: String,
) {
    world
        .actor_weapon_candidates
        .push(actor::ActorWeaponCandidate {
            item_form_id: parse_hex(&form_id),
            model_path: Some(model_path),
            damage,
            value,
            available: available == "yes",
        });
}

#[when("the actor starting weapon is selected")]
async fn when_actor_starting_weapon_is_selected(world: &mut BevyoutWorld) {
    world.actor_selected_weapon = actor::select_starting_weapon(&world.actor_weapon_candidates);
}

#[then(regex = r"^actor weapon 0x([0-9a-fA-F]+) is selected at (RightHand)$")]
async fn then_actor_weapon_is_selected(
    world: &mut BevyoutWorld,
    form_id: String,
    attachment: String,
) {
    let selected = world
        .actor_selected_weapon
        .as_ref()
        .expect("actor starting weapon must be selected");
    assert_eq!(selected.item_form_id, parse_hex(&form_id));
    assert_eq!(attachment, "RightHand");
    assert_eq!(
        selected.attachment_point,
        actor::ActorAttachmentPoint::RightHand
    );
}

#[then("the selected actor weapon model is unavailable")]
async fn then_selected_actor_weapon_model_is_unavailable(world: &mut BevyoutWorld) {
    assert!(
        !world
            .actor_selected_weapon
            .as_ref()
            .expect("actor starting weapon must be selected")
            .model_available
    );
}

fn parse_scale_component(value: &str) -> f32 {
    if value.eq_ignore_ascii_case("nan") {
        f32::NAN
    } else {
        value.parse().expect("scale component must be a float")
    }
}

#[given(regex = r"^humanoid scale reference (\S+) race (\S+) actor (\S+)$")]
async fn given_humanoid_scale(
    world: &mut BevyoutWorld,
    reference: String,
    race: String,
    actor_height: String,
) {
    world.actor_scale_kind = actor::ActorKind::Humanoid;
    world.actor_reference_scale = parse_scale_component(&reference);
    world.actor_race_scale = Some(parse_scale_component(&race));
    world.actor_base_scale = Some(parse_scale_component(&actor_height));
}

#[given(regex = r"^creature scale reference (\S+) base (\S+)$")]
async fn given_creature_scale(world: &mut BevyoutWorld, reference: String, base: String) {
    world.actor_scale_kind = actor::ActorKind::Creature;
    world.actor_reference_scale = parse_scale_component(&reference);
    world.actor_race_scale = None;
    world.actor_base_scale = Some(parse_scale_component(&base));
}

#[when("actor root scale is resolved")]
async fn when_actor_root_scale_is_resolved(world: &mut BevyoutWorld) {
    world.actor_resolved_scale = Some(actor::resolve_actor_root_scale(
        world.actor_scale_kind,
        world.actor_reference_scale,
        world.actor_race_scale,
        world.actor_base_scale,
    ));
}

#[then(regex = r"^actor root scale is ([0-9.]+)$")]
async fn then_actor_root_scale_is(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .actor_resolved_scale
        .expect("actor root scale must be resolved");
    assert!((actual - expected).abs() < 1.0e-5, "{actual} != {expected}");
}

#[given(
    regex = r"^a (Humanoid|Creature) appearance for base 0x([0-9a-fA-F]+) reference 0x([0-9a-fA-F]+)$"
)]
async fn given_actor_fallback_identity(
    world: &mut BevyoutWorld,
    kind: String,
    base_form_id: String,
    reference_form_id: String,
) {
    world.actor_fallback_input = actor::ActorAppearanceAvailability {
        kind: match kind.as_str() {
            "Humanoid" => actor::ActorKind::Humanoid,
            "Creature" => actor::ActorKind::Creature,
            other => panic!("unknown actor kind {other}"),
        },
        base_form_id: parse_hex(&base_form_id),
        reference_form_id: parse_hex(&reference_form_id),
        ..actor::ActorAppearanceAvailability::default()
    };
}

#[given(regex = r"^(exact|race sex|race default|generic) actor assets are available$")]
async fn given_actor_assets_available(world: &mut BevyoutWorld, tier: String) {
    match tier.as_str() {
        "exact" => world.actor_fallback_input.exact_available = true,
        "race sex" => world.actor_fallback_input.race_sex_available = true,
        "race default" => world.actor_fallback_input.race_default_available = true,
        "generic" => world.actor_fallback_input.generic_available = true,
        other => panic!("unknown fallback availability {other}"),
    }
}

#[given("FaceGen is not authored")]
async fn given_facegen_not_authored(world: &mut BevyoutWorld) {
    world.actor_fallback_input.facegen = actor::FaceGenAvailability::NotAuthored;
}

#[given("FaceGen is authored but incompatible")]
async fn given_facegen_authored_incompatible(world: &mut BevyoutWorld) {
    world.actor_fallback_input.facegen = actor::FaceGenAvailability::Incompatible;
}

fn fallback_reason(code: &str) -> actor::ActorFallbackReason {
    match code {
        "missing_facegen" => actor::ActorFallbackReason::MissingFaceGen,
        "missing_equipment" => actor::ActorFallbackReason::MissingEquipmentModel {
            item_form_id: 0,
            path: String::new(),
        },
        "missing_skeleton" => actor::ActorFallbackReason::MissingSkeleton {
            path: "fixture/skeleton.nif".into(),
        },
        "missing_head_model" => actor::ActorFallbackReason::MissingHeadModel {
            path: "fixture/head.nif".into(),
        },
        "incompatible_skin" => actor::ActorFallbackReason::IncompatibleSkin {
            path: "fixture/skin.nif".into(),
        },
        other => panic!("unsupported fallback reason fixture {other}"),
    }
}

#[given(regex = r#"^actor fallback reason \"([^\"]+)\" is supplied$"#)]
async fn given_actor_fallback_reason_supplied(world: &mut BevyoutWorld, code: String) {
    world
        .actor_fallback_supplied_reasons
        .push(fallback_reason(&code));
}

#[when("actor appearance fallback is resolved")]
async fn when_actor_appearance_fallback_is_resolved(world: &mut BevyoutWorld) {
    world.actor_fallback_decision = Some(actor::resolve_actor_fallback(
        &world.actor_fallback_input,
        world.actor_fallback_supplied_reasons.clone(),
    ));
}

#[then(
    regex = r"^actor fallback level is (AuthoredExact|RaceSexSpecific|RaceDefault|GenericProjectBody|ProxyMesh)$"
)]
async fn then_actor_fallback_level_is(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .actor_fallback_decision
        .as_ref()
        .expect("actor fallback must be resolved")
        .level;
    assert_eq!(actual.label(), expected);
}

#[then(regex = r"^actor FaceGen policy is (NotAuthored|Authored|RestPoseFallback)$")]
async fn then_actor_facegen_policy_is(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .actor_fallback_decision
        .as_ref()
        .expect("actor fallback must be resolved")
        .facegen_policy;
    assert_eq!(actual.label(), expected);
}

#[then(regex = r"^actor proxy kind is (None|GenericHumanoid|Bounds)$")]
async fn then_actor_proxy_kind_is(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .actor_fallback_decision
        .as_ref()
        .expect("actor fallback must be resolved")
        .proxy_kind;
    assert_eq!(actual.label(), expected);
}

#[then(regex = r#"^actor fallback reason \"([^\"]+)\" is recorded$"#)]
async fn then_actor_fallback_reason_is_recorded(world: &mut BevyoutWorld, code: String) {
    assert!(
        world
            .actor_fallback_decision
            .as_ref()
            .expect("actor fallback must be resolved")
            .reasons
            .iter()
            .any(|reason| reason.code() == code)
    );
}

#[then(regex = r#"^actor fallback reasons are \"([^\"]*)\"$"#)]
async fn then_actor_fallback_reasons_are(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .actor_fallback_decision
        .as_ref()
        .expect("actor fallback must be resolved")
        .reasons
        .iter()
        .map(|reason| reason.code())
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[then(regex = r"^fallback identity remains base 0x([0-9a-fA-F]+) reference 0x([0-9a-fA-F]+)$")]
async fn then_fallback_identity_remains(
    world: &mut BevyoutWorld,
    base_form_id: String,
    reference_form_id: String,
) {
    let decision = world
        .actor_fallback_decision
        .as_ref()
        .expect("actor fallback must be resolved");
    assert_eq!(decision.base_form_id, parse_hex(&base_form_id));
    assert_eq!(decision.reference_form_id, parse_hex(&reference_form_id));
}
// ---------------------------------------------------------------------
// actor_animation_catalog.feature -- appended section, do not interleave.
// ---------------------------------------------------------------------

fn decode_kffz_fixture(encoded: &str) -> Vec<u8> {
    let encoded = encoded.replace("\\\\0", "\0").replace("\\\\", "\\");
    encoded.into_bytes()
}

#[given(regex = r#"^an NPC KFFZ payload \"([^\"]*)\"$"#)]
async fn given_npc_kffz_payload(world: &mut BevyoutWorld, payload: String) {
    world.npc_kffz_payload = decode_kffz_fixture(&payload);
}

#[given(regex = r#"^a creature KFFZ payload \"([^\"]*)\"$"#)]
async fn given_creature_kffz_payload(world: &mut BevyoutWorld, payload: String) {
    world.creature_kffz_payload = decode_kffz_fixture(&payload);
}

#[when("the actor animation payloads are decoded")]
async fn when_actor_animation_payloads_are_decoded(world: &mut BevyoutWorld) {
    world.npc_kffz_paths = actor_animation::decode_kffz(&world.npc_kffz_payload).paths;
    world.creature_kffz_paths = actor_animation::decode_kffz(&world.creature_kffz_payload).paths;
}

#[then(regex = r#"^the NPC animation paths are \"([^\"]*)\"$"#)]
async fn then_npc_animation_paths(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.npc_kffz_paths.join(","), expected);
}

#[then(regex = r#"^the creature animation paths are \"([^\"]*)\"$"#)]
async fn then_creature_animation_paths(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.creature_kffz_paths.join(","), expected);
}

#[given(
    regex = r#"^actor animation source 0x([0-9a-fA-F]+) uses skeleton \"([^\"]*)\" and clips \"([^\"]*)\"$"#
)]
async fn given_actor_animation_source(
    world: &mut BevyoutWorld,
    form_hex: String,
    skeleton: String,
    clips: String,
) {
    let form_id = parse_hex(&form_hex);
    let actor = actor_catalog::ActorRecordInput {
        form_id,
        kind: actor_catalog::ActorRecordKind::Npc,
        model_animation: actor_catalog::ActorModelAnimation {
            model_path: Some(skeleton),
            animation_files: clips
                .split(',')
                .filter(|clip| !clip.is_empty())
                .map(str::to_owned)
                .collect(),
            ..Default::default()
        },
        ..Default::default()
    };
    world.actor_catalog_inputs.actors.insert(form_id, actor);
    world
        .actor_catalog_inputs
        .placements
        .push(actor_catalog::ActorPlacementInput {
            reference_form_id: form_id,
            base_form_id: form_id,
            kind: actor_catalog::ActorRecordKind::Npc,
            ..Default::default()
        });
}

#[given(
    regex = r"^actor animation source 0x([0-9a-fA-F]+) inherits model animation from 0x([0-9a-fA-F]+)$"
)]
async fn given_actor_animation_source_inherits(
    world: &mut BevyoutWorld,
    actor_hex: String,
    template_hex: String,
) {
    let actor = actor_catalog_actor_mut(world, parse_hex(&actor_hex));
    actor.base_template_form_id = Some(parse_hex(&template_hex));
    actor.template_usage.model_animation = true;
}

#[when("actor animation sources are resolved")]
async fn when_actor_animation_sources_are_resolved(world: &mut BevyoutWorld) {
    world.actor_catalog_result = Some(actor_catalog::build_actor_catalog(
        &world.actor_catalog_inputs,
        "animation-fixture",
    ));
}

#[then(regex = r#"^actor animation source 0x([0-9a-fA-F]+) resolves clips \"([^\"]*)\"$"#)]
async fn then_actor_animation_source_resolves_clips(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    assert_eq!(
        actor_catalog_blueprint(world, &reference_hex)
            .animation_candidates
            .join(","),
        expected
    );
}

#[given(
    regex = r#"^animation actor reference 0x([0-9a-fA-F]+) base 0x([0-9a-fA-F]+) model \"([^\"]*)\" skeleton \"([^\"]*)\" explicit clips \"([^\"]*)\"$"#
)]
async fn given_animation_actor_reference(
    world: &mut BevyoutWorld,
    reference_hex: String,
    base_hex: String,
    model_path: String,
    skeleton_path: String,
    clips: String,
) {
    world
        .actor_animation_discovery_inputs
        .push(actor_animation::ActorAnimationDiscoveryInput {
            reference_form_id: parse_hex(&reference_hex),
            base_form_id: parse_hex(&base_hex),
            kind: if model_path.to_ascii_lowercase().contains("creatures/") {
                actor_animation::PreparedActorAnimationKind::Creature
            } else {
                actor_animation::PreparedActorAnimationKind::Npc
            },
            model_path,
            skeleton_fingerprint: format!("skeleton-{skeleton_path}"),
            skeleton_path,
            explicit_kf_paths: clips
                .split(',')
                .filter(|clip| !clip.is_empty())
                .map(str::to_owned)
                .collect(),
            default_directories: Vec::new(),
        });
}

#[given(regex = r#"^available KF assets \"([^\"]*)\"$"#)]
async fn given_available_kf_assets(world: &mut BevyoutWorld, encoded: String) {
    world.actor_animation_assets = encoded
        .split(',')
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (path, fingerprint_and_state) = entry
                .split_once('@')
                .expect("KF asset fixture must be path@fingerprint");
            let (fingerprint, state) = match fingerprint_and_state.split_once('!') {
                Some((fingerprint, "malformed")) => (
                    fingerprint,
                    actor_animation::ActorAnimationAssetState::Malformed(
                        "synthetic malformed KF".to_owned(),
                    ),
                ),
                Some((fingerprint, "incompatible")) => (
                    fingerprint,
                    actor_animation::ActorAnimationAssetState::Incompatible(
                        "synthetic skeleton mismatch".to_owned(),
                    ),
                ),
                Some((_, other)) => panic!("unknown KF asset fixture state {other}"),
                None => (
                    fingerprint_and_state,
                    actor_animation::ActorAnimationAssetState::Compatible,
                ),
            };
            actor_animation::ActorAnimationAsset {
                path: path.to_owned(),
                fingerprint: fingerprint.to_owned(),
                state,
            }
        })
        .collect();
}

#[when("the prepared actor animation catalog is built")]
async fn when_prepared_actor_animation_catalog_is_built(world: &mut BevyoutWorld) {
    world.actor_animation_catalog = Some(actor_animation::build_actor_animation_catalog(
        "actor-animations-v1",
        "fixture-content",
        &world.actor_animation_discovery_inputs,
        &world.actor_animation_assets,
    ));
}

fn actor_animation_set_for_reference<'a>(
    world: &'a BevyoutWorld,
    reference_hex: &str,
) -> &'a actor_animation::PreparedActorAnimationSet {
    let reference_form_id = parse_hex(reference_hex);
    let catalog = world
        .actor_animation_catalog
        .as_ref()
        .expect("actor animation catalog must be built first");
    let mapping = catalog
        .actor_mappings
        .iter()
        .find(|mapping| mapping.reference_form_id == reference_form_id)
        .expect("actor animation mapping must exist");
    catalog
        .animation_sets
        .iter()
        .find(|set| set.id == mapping.animation_set_id)
        .expect("mapped actor animation set must exist")
}

#[then(regex = r#"^animation set for reference 0x([0-9a-fA-F]+) has source paths \"([^\"]*)\"$"#)]
async fn then_animation_set_source_paths(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    let actual = actor_animation_set_for_reference(world, &reference_hex)
        .clips
        .iter()
        .map(|clip| clip.source_kf_path.as_str())
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[then(regex = r#"^animation set for reference 0x([0-9a-fA-F]+) has clip names \"([^\"]*)\"$"#)]
async fn then_animation_set_clip_names(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    let actual = actor_animation_set_for_reference(world, &reference_hex)
        .clips
        .iter()
        .map(|clip| clip.name.as_str())
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[then(regex = r"^animation set for reference 0x([0-9a-fA-F]+) contains (\d+) ready clip$")]
async fn then_animation_set_ready_count(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: usize,
) {
    assert_eq!(
        actor_animation_set_for_reference(world, &reference_hex)
            .clips
            .iter()
            .filter(|clip| {
                clip.status == actor_animation::PreparedActorAnimationClipStatus::Ready
            })
            .count(),
        expected
    );
}

#[then(
    regex = r#"^animation set for reference 0x([0-9a-fA-F]+) has diagnostic codes \"([^\"]*)\"$"#
)]
async fn then_animation_set_diagnostic_codes(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    let mut actual = actor_animation_set_for_reference(world, &reference_hex)
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.clone())
        .collect::<Vec<_>>();
    actual.sort();
    actual.dedup();
    assert_eq!(actual.join(","), expected);
}

#[then(
    regex = r"^the prepared actor animation catalog has (\d+) actor mappings and (\d+) animation set$"
)]
async fn then_prepared_actor_animation_catalog_counts(
    world: &mut BevyoutWorld,
    mappings: usize,
    sets: usize,
) {
    let catalog = world
        .actor_animation_catalog
        .as_ref()
        .expect("actor animation catalog must be built first");
    assert_eq!(catalog.actor_mappings.len(), mappings);
    assert_eq!(catalog.animation_sets.len(), sets);
}

#[then(regex = r"^references 0x([0-9a-fA-F]+) and 0x([0-9a-fA-F]+) use the same animation set$")]
async fn then_references_use_same_animation_set(
    world: &mut BevyoutWorld,
    left_hex: String,
    right_hex: String,
) {
    let catalog = world
        .actor_animation_catalog
        .as_ref()
        .expect("actor animation catalog must be built first");
    let set_id = |form_id| {
        catalog
            .actor_mappings
            .iter()
            .find(|mapping| mapping.reference_form_id == form_id)
            .map(|mapping| mapping.animation_set_id.as_str())
            .expect("actor animation mapping must exist")
    };
    assert_eq!(set_id(parse_hex(&left_hex)), set_id(parse_hex(&right_hex)));
}

// ---------------------------------------------------------------------
// nav_portals.feature (issue #154, M4 wave 8) -- appended section, do not
// interleave. Reuses `nav_graph.feature`'s "a nav graph mesh"/"has source
// vertex"/"has triangle"/"the nav graph is built" steps (prepare-side
// portal validation, `vsa::prepare::nav_graph::compute_mesh_merges`) and
// `nav_backend.feature`/`nav_adapter.feature`'s "landmass mesh"/"a prepared
// merge connects"/"the merge-link descriptors are resolved" steps (runtime
// interval-to-link conversion, `viewer::nav::landmass_graph::
// merge_link_descriptors`) rather than re-declaring either seam. New steps
// below only cover what #154 actually added: matched-edge identity, a
// portal's recorded vertical drop, and interval-aware merge inputs/cost.
// ---------------------------------------------------------------------

#[then(regex = r"^cross-mesh merge (\d+) has edge_a (\d+),(\d+) and edge_b (\d+),(\d+)$")]
async fn then_nav_portals_merge_edge_identity(
    world: &mut BevyoutWorld,
    index: usize,
    edge_a_start: u32,
    edge_a_end: u32,
    edge_b_start: u32,
    edge_b_end: u32,
) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    let merge = graph.mesh_merges[index];
    assert_eq!(merge.edge_a, [edge_a_start, edge_a_end]);
    assert_eq!(merge.edge_b, [edge_b_start, edge_b_end]);
}

#[then(regex = r"^cross-mesh merge (\d+) has a vertical drop of about ([\d.]+) metres$")]
async fn then_nav_portals_merge_vertical_drop(world: &mut BevyoutWorld, index: usize, drop: f32) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    let merge = graph.mesh_merges[index];
    let actual = (merge.interval_a[0][1] - merge.interval_b[0][1]).abs();
    assert!(
        (actual - drop).abs() < 0.01,
        "expected a vertical drop near {drop} m, got {actual} m ({:?})",
        merge
    );
}

#[given(
    regex = r"^a prepared merge connects mesh 0x([0-9a-fA-F]+) triangle (\d+) to mesh 0x([0-9a-fA-F]+) triangle (\d+) with interval ([-\d.,\s]+) to ([-\d.,\s]+) and interval ([-\d.,\s]+) to ([-\d.,\s]+)$"
)]
#[allow(clippy::too_many_arguments)]
async fn given_prepared_merge_with_interval(
    world: &mut BevyoutWorld,
    mesh_a_hex: String,
    triangle_a: u32,
    mesh_b_hex: String,
    triangle_b: u32,
    interval_a_start: String,
    interval_a_end: String,
    interval_b_start: String,
    interval_b_end: String,
) {
    world
        .nav_adapter_merge_inputs
        .push(landmass_graph::MergeInput {
            mesh_a_form_id: parse_hex(&mesh_a_hex),
            triangle_a,
            mesh_b_form_id: parse_hex(&mesh_b_hex),
            triangle_b,
            interval_a: [
                nav_adapter_parse_f32_triple(&interval_a_start),
                nav_adapter_parse_f32_triple(&interval_a_end),
            ],
            interval_b: [
                nav_adapter_parse_f32_triple(&interval_b_start),
                nav_adapter_parse_f32_triple(&interval_b_end),
            ],
        });
}

#[then(regex = r"^merge-link descriptor (\d+) has a cost of about ([\d.]+)$")]
async fn then_nav_portals_merge_link_cost(world: &mut BevyoutWorld, index: usize, cost: f32) {
    let links = world
        .nav_adapter_merge_links
        .as_ref()
        .expect("merge links must be resolved first");
    let actual = links[index].distance;
    assert!(
        (actual - cost).abs() < 0.01,
        "expected cost near {cost}, got {actual}"
    );
}

// ---------------------------------------------------------------------
// nav_stuck_progress.feature (issue #157) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

#[given(regex = r"^a desired horizontal velocity of ([\-\d.]+), ([\-\d.]+)$")]
async fn given_nav_stuck_progress_desired(world: &mut BevyoutWorld, x: f32, z: f32) {
    world.nav_stuck_progress_desired = [x, z];
}

#[given(regex = r"^an achieved horizontal velocity of ([\-\d.]+), ([\-\d.]+)$")]
async fn given_nav_stuck_progress_achieved(world: &mut BevyoutWorld, x: f32, z: f32) {
    world.nav_stuck_progress_achieved = [x, z];
}

#[when("the route progress delta is computed")]
async fn when_route_progress_delta_computed(world: &mut BevyoutWorld) {
    world.nav_stuck_progress_delta = Some(movement_policy::route_progress_delta(
        world.nav_stuck_progress_desired,
        world.nav_stuck_progress_achieved,
    ));
}

#[then(regex = r"^the route progress delta is ([\-\d.]+)$")]
async fn then_route_progress_delta_is(world: &mut BevyoutWorld, expected: f32) {
    assert_eq!(world.nav_stuck_progress_delta, Some(expected));
}

#[given(
    regex = r"^a U-shaped detour route of (\d+) ticks where the agent always achieves its desired horizontal velocity$"
)]
async fn given_u_shaped_detour_route(world: &mut BevyoutWorld, ticks: u32) {
    world.nav_stuck_progress_route_kind = Some("u_shaped".to_string());
    world.nav_stuck_progress_route_ticks = ticks;
}

#[given(regex = r"^a fully blocked route of (\d+) ticks with desired horizontal speed ([\d.]+)$")]
async fn given_fully_blocked_route(world: &mut BevyoutWorld, ticks: u32, speed: f32) {
    world.nav_stuck_progress_route_kind = Some("blocked".to_string());
    world.nav_stuck_progress_route_ticks = ticks;
    world.nav_stuck_progress_route_speed = speed;
}

#[given(
    regex = r"^an oscillating route of (\d+) ticks where the agent always achieves its desired horizontal velocity$"
)]
async fn given_oscillating_route(world: &mut BevyoutWorld, ticks: u32) {
    world.nav_stuck_progress_route_kind = Some("oscillating".to_string());
    world.nav_stuck_progress_route_ticks = ticks;
}

/// This tick's `(desired, achieved)` horizontal-velocity pair for the named
/// route kind, mirroring exactly what `apply_agent_physics_movement` would
/// sample from a real KCC sweep at tick `tick` (1-indexed) of `total_ticks`.
fn nav_stuck_progress_route_tick(
    kind: &str,
    tick: u32,
    total_ticks: u32,
    blocked_speed: f32,
) -> ([f32; 2], [f32; 2]) {
    match kind {
        "u_shaped" => {
            // Three equal legs: away from the final target, around the
            // wall, then back toward it -- the exact detour shape issue
            // #157 exists to stop false-triggering stuck recovery on. The
            // agent always achieves what it desires (never collision-
            // blocked), so every tick is genuine corridor progress.
            let leg = total_ticks / 3;
            let direction = if tick <= leg {
                [-1.0, 0.0]
            } else if tick <= 2 * leg {
                [0.0, 1.0]
            } else {
                [1.0, 0.0]
            };
            let desired = [direction[0] * 2.0, direction[1] * 2.0];
            (desired, desired)
        }
        "blocked" => {
            // Landmass keeps asking for horizontal motion; the KCC sweep
            // never achieves any of it -- a genuine wedge, not a detour.
            ([blocked_speed, 0.0], [0.0, 0.0])
        }
        "oscillating" => {
            // Known limitation (see `movement_policy.rs`'s module doc
            // comment): desired direction flips every tick and the agent
            // fully achieves each flip, so every tick's achieved motion is
            // trivially parallel to that same tick's desired direction --
            // perpetual "progress" despite the agent effectively orbiting
            // in place. Documented as an accepted trade-off, not desired
            // behaviour.
            let sign = if tick % 2 == 1 { 1.0 } else { -1.0 };
            let desired = [sign * 2.0, 0.0];
            (desired, desired)
        }
        other => panic!("unknown nav stuck-progress route kind {other:?}"),
    }
}

/// The same per-tick stuck-tracking bookkeeping
/// `apply_agent_physics_movement` performs (see that function's
/// `route_progress`/`best_distance`/`ticks_without_progress`/
/// `recovery_active` handling in `src/viewer/nav/agent.rs`), replayed here
/// against the pure `movement_policy` functions directly -- no Bevy/boxddd
/// involved. Shared by every `nav_stuck_progress.feature` route-simulation
/// step so the bookkeeping is written once.
struct NavStuckProgressSim {
    best_distance: f32,
    ticks_without_progress: u32,
    recovery_active: bool,
    route_progress: f32,
    tick: u32,
    start_recovery_tick: Option<u32>,
    stuck_tick: Option<u32>,
}

impl NavStuckProgressSim {
    fn new() -> Self {
        Self {
            best_distance: f32::MAX,
            ticks_without_progress: 0,
            recovery_active: false,
            route_progress: 0.0,
            tick: 0,
            start_recovery_tick: None,
            stuck_tick: None,
        }
    }

    /// Mirrors a real repath / new-target event: `nav/agent.rs`'s
    /// target-change handler resets exactly these three fields (issue
    /// #157 left that handler untouched). `route_progress` itself is never
    /// reset -- `best_distance`'s own reset-to-`f32::MAX` re-baselines
    /// every future comparison to "progress since now" regardless of
    /// `route_progress`'s absolute running total, so a fresh target does
    /// not need a fresh zero there (see `AgentKcc::route_progress`'s doc
    /// comment).
    fn repath(&mut self) {
        self.best_distance = f32::MAX;
        self.ticks_without_progress = 0;
        self.recovery_active = false;
    }

    fn step(&mut self, desired: [f32; 2], achieved: [f32; 2]) {
        const DT: f32 = 1.0 / 64.0;
        self.tick += 1;
        self.route_progress += movement_policy::route_progress_delta(desired, achieved) * DT;
        let distance = -self.route_progress;
        if self.best_distance == f32::MAX {
            self.best_distance = distance;
        }
        let decision = movement_policy::decide_stuck(movement_policy::StuckObservation {
            distance_to_target: distance,
            best_distance_so_far: self.best_distance,
            ticks_without_progress: self.ticks_without_progress,
            recovery_active: self.recovery_active,
        });
        let progressed = distance + movement_policy::STUCK_PROGRESS_EPSILON < self.best_distance;
        if progressed {
            self.best_distance = distance;
            self.ticks_without_progress = 0;
            self.recovery_active = false;
        } else {
            self.ticks_without_progress = self.ticks_without_progress.saturating_add(1);
        }
        match decision {
            movement_policy::StuckDecision::StartRecovery => {
                self.recovery_active = true;
                self.start_recovery_tick.get_or_insert(self.tick);
            }
            movement_policy::StuckDecision::Stuck => {
                self.stuck_tick.get_or_insert(self.tick);
            }
            movement_policy::StuckDecision::Progressing
            | movement_policy::StuckDecision::RecoveryPending => {}
        }
    }
}

/// Returns the first tick (1-indexed) each of `StartRecovery`/`Stuck` was
/// reached along the named route kind, if ever.
#[when("the route is simulated tick by tick")]
async fn when_route_is_simulated(world: &mut BevyoutWorld) {
    let kind = world
        .nav_stuck_progress_route_kind
        .clone()
        .expect("a route must be given first");
    let total_ticks = world.nav_stuck_progress_route_ticks;
    let blocked_speed = world.nav_stuck_progress_route_speed;

    let mut sim = NavStuckProgressSim::new();
    for tick in 1..=total_ticks {
        let (desired, achieved) =
            nav_stuck_progress_route_tick(&kind, tick, total_ticks, blocked_speed);
        sim.step(desired, achieved);
    }

    world.nav_stuck_progress_start_recovery_tick = sim.start_recovery_tick;
    world.nav_stuck_progress_stuck_tick = sim.stuck_tick;
}

#[given(
    regex = r"^an avoidance-paused route with (\d+) ticks of progress at speed ([\d.]+) followed by (\d+) ticks of zero desired velocity$"
)]
async fn given_avoidance_paused_route(
    world: &mut BevyoutWorld,
    progress_ticks: u32,
    speed: f32,
    pause_ticks: u32,
) {
    world.nav_stuck_progress_pause_progress_ticks = progress_ticks;
    world.nav_stuck_progress_pause_speed = speed;
    world.nav_stuck_progress_pause_ticks = pause_ticks;
}

/// Follow-up to the "route is simulated tick by tick" step above (issue
/// #157 follow-up): a two-phase route -- genuine forward progress, then a
/// stretch of zero desired horizontal velocity (landmass legitimately
/// pausing the agent, e.g. queuing at a doorway) -- pinning today's actual
/// behaviour for that pause rather than leaving it implicit.
#[when("the paused route is simulated tick by tick")]
async fn when_paused_route_is_simulated(world: &mut BevyoutWorld) {
    let progress_ticks = world.nav_stuck_progress_pause_progress_ticks;
    let speed = world.nav_stuck_progress_pause_speed;
    let pause_ticks = world.nav_stuck_progress_pause_ticks;

    let mut sim = NavStuckProgressSim::new();
    for _ in 0..progress_ticks {
        sim.step([speed, 0.0], [speed, 0.0]);
    }
    // Zero desired horizontal velocity: `route_progress_delta`'s
    // near-zero-desired-length guard contributes exactly 0.0 every tick
    // here, so corridor progress flatlines for the whole pause exactly
    // like a genuine collision block would (see the "blocked" route kind
    // above) -- today's pinned behaviour, not a claim that this is the
    // *right* behaviour for a legitimate avoidance stall.
    for _ in 0..pause_ticks {
        sim.step([0.0, 0.0], [0.0, 0.0]);
    }

    world.nav_stuck_progress_start_recovery_tick = sim.start_recovery_tick;
    world.nav_stuck_progress_stuck_tick = sim.stuck_tick;
}

#[given(
    regex = r"^a blocked route of (\d+) ticks that repaths onto a new detour leg of (\d+) ticks at speed ([\d.]+)$"
)]
async fn given_repathed_route(
    world: &mut BevyoutWorld,
    blocked_ticks: u32,
    leg_ticks: u32,
    leg_speed: f32,
) {
    world.nav_stuck_progress_repath_blocked_ticks = blocked_ticks;
    world.nav_stuck_progress_repath_leg_ticks = leg_ticks;
    world.nav_stuck_progress_repath_leg_speed = leg_speed;
}

/// Issue #157 follow-up: a genuine block runs right up to (but not past)
/// the recovery threshold, then a repath (`NavStuckProgressSim::repath`,
/// mirroring `nav/agent.rs`'s target-change handler) lands, followed by a
/// new, slower detour leg the agent fully achieves. Pins that the repath's
/// reset means the new leg's own progress is judged on its own terms
/// rather than inheriting the old near-miss window.
#[when("the repathed route is simulated tick by tick")]
async fn when_repathed_route_is_simulated(world: &mut BevyoutWorld) {
    let blocked_ticks = world.nav_stuck_progress_repath_blocked_ticks;
    let leg_ticks = world.nav_stuck_progress_repath_leg_ticks;
    let leg_speed = world.nav_stuck_progress_repath_leg_speed;

    let mut sim = NavStuckProgressSim::new();
    for _ in 0..blocked_ticks {
        sim.step([2.0, 0.0], [0.0, 0.0]);
    }
    sim.repath();
    for _ in 0..leg_ticks {
        sim.step([leg_speed, 0.0], [leg_speed, 0.0]);
    }

    world.nav_stuck_progress_start_recovery_tick = sim.start_recovery_tick;
    world.nav_stuck_progress_stuck_tick = sim.stuck_tick;
}

#[then("no stuck decision along the route ever reaches start-recovery")]
async fn then_no_stuck_recovery_along_route(world: &mut BevyoutWorld) {
    assert_eq!(
        world.nav_stuck_progress_start_recovery_tick, None,
        "a valid detour route must never false-trigger stuck recovery"
    );
}

#[then(regex = r"^the stuck decision first reaches start-recovery at tick (\d+)$")]
async fn then_start_recovery_at_tick(world: &mut BevyoutWorld, tick: u32) {
    assert_eq!(world.nav_stuck_progress_start_recovery_tick, Some(tick));
}

#[then(regex = r"^the stuck decision first reaches stuck at tick (\d+)$")]
async fn then_stuck_at_tick(world: &mut BevyoutWorld, tick: u32) {
    assert_eq!(world.nav_stuck_progress_stuck_tick, Some(tick));
}

// ---------------------------------------------------------------------
// nav_door_topology.feature (issue #155, M4 wave 8) -- appended section,
// do not interleave. Reuses nav_backend.feature's "landmass mesh"/"has a
// door" steps (`world.nav_backend_meshes`) for the door-typing scenarios
// rather than re-declaring mesh-building steps; the point-in-triangle
// scenarios are pure geometry, needing no mesh at all.
// ---------------------------------------------------------------------

#[when("the door type indices are resolved")]
async fn when_door_type_indices_resolved(world: &mut BevyoutWorld) {
    world.nav_door_topology_type_indices =
        Some(landmass_graph::door_type_indices(&world.nav_backend_meshes));
}

#[then(regex = r"^door 0x([0-9a-fA-F]+) has type index (\d+)$")]
async fn then_door_has_type_index(world: &mut BevyoutWorld, door_hex: String, index: usize) {
    let indices = world
        .nav_door_topology_type_indices
        .as_ref()
        .expect("door type indices must be resolved first");
    assert_eq!(indices.get(&parse_hex(&door_hex)), Some(&index));
}

#[then(regex = r"^there is exactly (\d+) resolved door type index$")]
async fn then_resolved_door_type_index_count(world: &mut BevyoutWorld, count: usize) {
    let indices = world
        .nav_door_topology_type_indices
        .as_ref()
        .expect("door type indices must be resolved first");
    assert_eq!(indices.len(), count);
}

#[given(
    regex = r"^a door triangle with vertices ([\-\d.]+), ([\-\d.]+), ([\-\d.]+) and ([\-\d.]+), ([\-\d.]+), ([\-\d.]+) and ([\-\d.]+), ([\-\d.]+), ([\-\d.]+)$"
)]
#[allow(clippy::too_many_arguments)]
async fn given_door_triangle(
    world: &mut BevyoutWorld,
    ax: f32,
    ay: f32,
    az: f32,
    bx: f32,
    by: f32,
    bz: f32,
    cx: f32,
    cy: f32,
    cz: f32,
) {
    world.nav_door_topology_triangle = Some([[ax, ay, az], [bx, by, bz], [cx, cy, cz]]);
}

#[given(regex = r"^a query point at ([\-\d.]+), ([\-\d.]+), ([\-\d.]+)$")]
async fn given_query_point(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.nav_door_topology_point = Some([x, y, z]);
}

/// The vertical-gap tolerance passed to `point_in_door_triangle` in every
/// scenario below: the same value `nav/agent.rs`'s `AGENT_HEIGHT` constant
/// holds (that constant is private to a Bevy-only module this
/// Bevy-engine-free suite does not include -- see `landmass_graph.rs`'s
/// own module doc comment for why -- so this is a small literal
/// duplicate, the same precedent `landmass_graph.rs` itself already sets
/// for `MERGE_PORTAL_STEP_HEIGHT`).
const NAV_DOOR_TOPOLOGY_VERTICAL_GAP: f32 = 1.8;

fn nav_door_topology_triangle_and_point(world: &BevyoutWorld) -> ([[f32; 3]; 3], [f32; 3]) {
    let triangle = world
        .nav_door_topology_triangle
        .expect("a door triangle must be given first");
    let point = world
        .nav_door_topology_point
        .expect("a query point must be given first");
    (triangle, point)
}

#[then("the query point is inside the door triangle")]
async fn then_point_inside_door_triangle(world: &mut BevyoutWorld) {
    let (triangle, point) = nav_door_topology_triangle_and_point(world);
    assert!(landmass_graph::point_in_door_triangle(
        point,
        triangle,
        NAV_DOOR_TOPOLOGY_VERTICAL_GAP
    ));
}

#[then("the query point is outside the door triangle")]
async fn then_point_outside_door_triangle(world: &mut BevyoutWorld) {
    let (triangle, point) = nav_door_topology_triangle_and_point(world);
    assert!(!landmass_graph::point_in_door_triangle(
        point,
        triangle,
        NAV_DOOR_TOPOLOGY_VERTICAL_GAP
    ));
}

#[then(regex = r"^the query point is within ([\d.]+) metres of the door triangle's centroid$")]
async fn then_point_within_centroid_radius(world: &mut BevyoutWorld, radius: f32) {
    let (triangle, point) = nav_door_topology_triangle_and_point(world);
    let centroid = [
        (triangle[0][0] + triangle[1][0] + triangle[2][0]) / 3.0,
        (triangle[0][1] + triangle[1][1] + triangle[2][1]) / 3.0,
        (triangle[0][2] + triangle[1][2] + triangle[2][2]) / 3.0,
    ];
    let dx = point[0] - centroid[0];
    let dz = point[2] - centroid[2];
    let distance = (dx * dx + dz * dz).sqrt();
    assert!(
        distance <= radius,
        "test setup: the query point ({distance} m) must be within the old proximity radius ({radius} m) of the centroid"
    );
}
// ---------------------------------------------------------------------
// actor_animation_conversion.feature (issue #104, M4 wave 10) -- appended
// section, do not interleave.
// ---------------------------------------------------------------------

#[given("no scene converter is requested for actor animation preparation")]
async fn given_no_scene_converter_for_actor_animation(world: &mut BevyoutWorld) {
    world.requested_converter = None;
}

#[given("no actor animation converter is requested")]
async fn given_no_actor_animation_converter(world: &mut BevyoutWorld) {
    world.requested_actor_animation_converter = None;
}

#[given(regex = r#"^the \"([^\"]*)\" actor animation converter is requested$"#)]
async fn given_actor_animation_converter(world: &mut BevyoutWorld, converter: String) {
    world.requested_actor_animation_converter = Some(match converter.as_str() {
        "disabled" => converter_policy::ActorAnimationBackend::Disabled,
        "native" => converter_policy::ActorAnimationBackend::Native,
        other => panic!("unknown actor animation converter {other:?}"),
    });
}

#[given("an actor animation clip pack has an output and report that both validate")]
async fn given_valid_actor_animation_clip_pack(world: &mut BevyoutWorld) {
    world.actor_animation_pack_cache_state.output_present = true;
    world.actor_animation_pack_cache_state.report_present = true;
    world.actor_animation_pack_cache_state.validation_passed = true;
}

#[given("actor animation clip-pack rebuild is not requested")]
async fn given_actor_animation_rebuild_not_requested(world: &mut BevyoutWorld) {
    world.actor_animation_pack_cache_state.rebuild_requested = false;
}

#[given("actor animation clip-pack rebuild is requested")]
async fn given_actor_animation_rebuild_requested(world: &mut BevyoutWorld) {
    world.actor_animation_pack_cache_state.rebuild_requested = true;
}

#[when("the actor animation converter selections are resolved")]
async fn when_actor_animation_converter_selections_resolved(world: &mut BevyoutWorld) {
    world.resolved_converter = Some(converter_policy::resolve_converter_backend(
        world.requested_converter,
    ));
    world.resolved_actor_animation_converter =
        Some(converter_policy::resolve_actor_animation_backend(
            world.requested_actor_animation_converter,
        ));
}

#[when("the actor animation clip-pack cache decision is made")]
async fn when_actor_animation_pack_cache_decision_is_made(world: &mut BevyoutWorld) {
    world.actor_animation_pack_cache_decision =
        Some(actor_animation_cache::actor_animation_pack_cache_decision(
            world.actor_animation_pack_cache_state,
        ));
}

#[then(regex = r#"^the selected scene converter is \"([^\"]*)\"$"#)]
async fn then_selected_scene_converter(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .resolved_converter
            .expect("scene converter must be resolved")
            .as_str(),
        expected
    );
}

#[then(regex = r#"^the selected actor animation converter is \"([^\"]*)\"$"#)]
async fn then_selected_actor_animation_converter(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .resolved_actor_animation_converter
            .expect("actor animation converter must be resolved")
            .as_str(),
        expected
    );
}

#[then("actor animation preparation does not require Blender")]
async fn then_actor_animation_preparation_does_not_require_blender(world: &mut BevyoutWorld) {
    assert!(matches!(
        world
            .resolved_actor_animation_converter
            .expect("actor animation converter must be resolved"),
        converter_policy::ActorAnimationBackend::Disabled
            | converter_policy::ActorAnimationBackend::Native
    ));
}

#[then(regex = r#"^the actor animation clip-pack cache decision is \"([^\"]*)\"$"#)]
async fn then_actor_animation_pack_cache_decision(world: &mut BevyoutWorld, expected: String) {
    let actual = match world
        .actor_animation_pack_cache_decision
        .expect("actor animation cache decision must be made")
    {
        actor_animation_cache::ActorAnimationPackCacheDecision::Reuse => "reuse",
        actor_animation_cache::ActorAnimationPackCacheDecision::Build => "build",
    };
    assert_eq!(actual, expected);
}
// =====================================================================
// nav_fall_guard.feature (issue #164) -- appended step section.
//
// `viewer::nav::fall_guard` is std-only (no bevy/bevy_landmass import),
// same flat `#[path]` include rationale as `movement_policy` et al. above;
// declared here inside this appended section to respect the shared
// merge-seam convention (only World fields and one delimited step section
// appended at the end of this file).
// =====================================================================
#[path = "../src/viewer/nav/agent/fall_guard.rs"]
#[allow(dead_code, unused_imports)]
mod fall_guard;

#[given(regex = r"^a cell whose minimum geometry Y is (-?[\d.]+)$")]
async fn given_cell_minimum_geometry_y(world: &mut BevyoutWorld, min_y: f32) {
    world.nav_fall_guard_bounds_min_y = Some(min_y);
}

#[given(regex = r"^a nav agent at Y (-?[\d.]+)$")]
async fn given_nav_agent_at_y(world: &mut BevyoutWorld, agent_y: f32) {
    world.nav_fall_guard_agent_y = Some(agent_y);
}

#[given("a nav agent resting exactly at the kill plane")]
async fn given_nav_agent_at_kill_plane(world: &mut BevyoutWorld) {
    let min_y = world
        .nav_fall_guard_bounds_min_y
        .expect("a cell minimum geometry Y must be given first");
    world.nav_fall_guard_agent_y = Some(fall_guard::fall_kill_z(min_y));
}

#[given("a nav agent just below the kill plane")]
async fn given_nav_agent_just_below_kill_plane(world: &mut BevyoutWorld) {
    let min_y = world
        .nav_fall_guard_bounds_min_y
        .expect("a cell minimum geometry Y must be given first");
    world.nav_fall_guard_agent_y = Some(fall_guard::fall_kill_z(min_y) - 0.01);
}

#[when("the fall kill plane is computed")]
async fn when_fall_kill_plane_computed(world: &mut BevyoutWorld) {
    let min_y = world
        .nav_fall_guard_bounds_min_y
        .expect("a cell minimum geometry Y must be given first");
    world.nav_fall_guard_kill_z = Some(fall_guard::fall_kill_z(min_y));
}

#[when("the fall guard is evaluated")]
async fn when_fall_guard_evaluated(world: &mut BevyoutWorld) {
    let min_y = world
        .nav_fall_guard_bounds_min_y
        .expect("a cell minimum geometry Y must be given first");
    let agent_y = world
        .nav_fall_guard_agent_y
        .expect("a nav agent Y must be given first");
    world.nav_fall_guard_verdict = Some(fall_guard::evaluate_fall(min_y, agent_y));
}

#[then(regex = r"^the fall kill plane is (-?[\d.]+) metres below the minimum geometry Y$")]
async fn then_fall_kill_plane_below_min(world: &mut BevyoutWorld, margin: f32) {
    let min_y = world
        .nav_fall_guard_bounds_min_y
        .expect("a cell minimum geometry Y must be given first");
    let kill_z = world
        .nav_fall_guard_kill_z
        .expect("the kill plane must be computed first");
    assert!((kill_z - (min_y - margin)).abs() < 1e-4);
}

#[then("the fall guard reports the agent is in bounds")]
async fn then_fall_guard_in_bounds(world: &mut BevyoutWorld) {
    assert_eq!(
        world.nav_fall_guard_verdict,
        Some(fall_guard::FallVerdict::InBounds)
    );
}

#[then("the fall guard reports the agent has fallen out of the world")]
async fn then_fall_guard_fell_out(world: &mut BevyoutWorld) {
    assert_eq!(
        world.nav_fall_guard_verdict,
        Some(fall_guard::FallVerdict::FellOutOfWorld)
    );
}

// ---------------------------------------------------------------------
// nav_authored_semantics.feature (issue #156, M4 wave 9) -- appended
// section, do not interleave. No new `BevyoutWorld` fields: every scenario
// drives the existing `nav_graph_inputs`/`nav_graph_result`/
// `nav_backend_meshes` state via `nav_graph_mesh_input_mut`/
// `nav_graph_result_mesh`/`nav_backend_mesh_mut`, reusing nav_graph.feature's
// mesh/triangle steps and nav_portals.feature's cross-mesh merge steps.
// ---------------------------------------------------------------------

#[given(regex = r"^mesh 0x([0-9a-fA-F]+) triangle (\d+) has flags 0x([0-9a-fA-F]+)$")]
async fn given_nav_graph_triangle_flags(
    world: &mut BevyoutWorld,
    form_hex: String,
    triangle_index: usize,
    flags_hex: String,
) {
    let flags = parse_hex(&flags_hex);
    let mesh = nav_graph_mesh_input_mut(world, &form_hex);
    let triangle = mesh.triangles.get_mut(triangle_index).unwrap_or_else(|| {
        panic!("mesh {form_hex} triangle {triangle_index} was not created first")
    });
    triangle.flags = flags;
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) polygon (\d+) is marked preferred-pathing$")]
async fn then_nav_graph_polygon_preferred_pathing(
    world: &mut BevyoutWorld,
    form_hex: String,
    polygon: usize,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    assert!(
        mesh.polygons[polygon].is_preferred_pathing,
        "{:?}",
        mesh.polygons[polygon]
    );
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) polygon (\d+) is not marked preferred-pathing$")]
async fn then_nav_graph_polygon_not_preferred_pathing(
    world: &mut BevyoutWorld,
    form_hex: String,
    polygon: usize,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    assert!(
        !mesh.polygons[polygon].is_preferred_pathing,
        "{:?}",
        mesh.polygons[polygon]
    );
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) polygon (\d+) edge (\d+) is authored-external$")]
async fn then_nav_graph_edge_authored_external(
    world: &mut BevyoutWorld,
    form_hex: String,
    polygon: usize,
    edge: usize,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    assert!(
        mesh.polygons[polygon].authored_external[edge],
        "{:?}",
        mesh.polygons[polygon]
    );
}

#[then(regex = r"^mesh 0x([0-9a-fA-F]+) polygon (\d+) edge (\d+) is not authored-external$")]
async fn then_nav_graph_edge_not_authored_external(
    world: &mut BevyoutWorld,
    form_hex: String,
    polygon: usize,
    edge: usize,
) {
    let mesh = nav_graph_result_mesh(world, &form_hex);
    assert!(
        !mesh.polygons[polygon].authored_external[edge],
        "{:?}",
        mesh.polygons[polygon]
    );
}

fn nav_graph_merge(world: &BevyoutWorld, index: usize) -> &nav_graph::PreparedNavMeshMerge {
    &world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .mesh_merges[index]
}

#[then(regex = r"^cross-mesh merge (\d+) is authored$")]
async fn then_nav_graph_merge_authored(world: &mut BevyoutWorld, index: usize) {
    let merge = nav_graph_merge(world, index);
    assert!(merge.authored_evidence, "{merge:?}");
}

#[then(regex = r"^cross-mesh merge (\d+) is geometric$")]
async fn then_nav_graph_merge_geometric(world: &mut BevyoutWorld, index: usize) {
    let merge = nav_graph_merge(world, index);
    assert!(!merge.authored_evidence, "{merge:?}");
}

#[then(regex = r"^the nav graph counters report merges authored (\d+) geometric (\d+)$")]
async fn then_nav_graph_counters_merges_authored(
    world: &mut BevyoutWorld,
    authored: usize,
    geometric: usize,
) {
    let counters = &world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .counters;
    assert_eq!(counters.mesh_merges_authored, authored);
    assert_eq!(counters.mesh_merges_geometric, geometric);
}

#[then(regex = r"^the nav graph counters report merge candidates authored (\d+) geometric (\d+)$")]
async fn then_nav_graph_counters_candidates_authored(
    world: &mut BevyoutWorld,
    authored: usize,
    geometric: usize,
) {
    let counters = &world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .counters;
    assert_eq!(counters.merge_candidates_authored, authored);
    assert_eq!(counters.merge_candidates_geometric, geometric);
}

#[then(regex = r"^the nav graph counters report nvex outside-cell (\d+) inside-cell (\d+)$")]
async fn then_nav_graph_counters_nvex(world: &mut BevyoutWorld, outside: usize, inside: usize) {
    let counters = &world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .counters;
    assert_eq!(counters.nvex_targets_outside_cell, outside);
    assert_eq!(counters.nvex_targets_inside_cell, inside);
}

#[then(
    regex = r"^the nav graph counters report nvci subrecords (\d+) entries (\d+) door-matches (\d+) navmesh-matches (\d+)$"
)]
async fn then_nav_graph_counters_nvci(
    world: &mut BevyoutWorld,
    subrecords: usize,
    entries: usize,
    door_matches: usize,
    navmesh_matches: usize,
) {
    let counters = &world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first")
        .counters;
    assert_eq!(counters.nvci_subrecords, subrecords);
    assert_eq!(counters.nvci_entries, entries);
    assert_eq!(counters.nvci_door_matches, door_matches);
    assert_eq!(counters.nvci_navmesh_matches, navmesh_matches);
}

#[then(regex = r#"^the nav graph has an "info" diagnostic containing "([^"]*)"$"#)]
async fn then_nav_graph_info_diagnostic(world: &mut BevyoutWorld, expected: String) {
    let graph = world
        .nav_graph_result
        .as_ref()
        .expect("the nav graph must be built first");
    assert!(
        graph
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == "info"
                && diagnostic.message.contains(expected.as_str())),
        "expected an info diagnostic containing {expected:?} in {:?}",
        graph.diagnostics
    );
}

#[given(regex = r"^a nav graph NAVI correlation with leading NAVM 0x([0-9a-fA-F]+)$")]
async fn given_nav_graph_navi_correlation(world: &mut BevyoutWorld, navm_hex: String) {
    world
        .nav_graph_inputs
        .navi_correlations
        .push(nav_graph::NavGraphNaviCorrelationInput {
            leading_navmesh_form_id: Some(parse_hex(&navm_hex)),
            entries: Vec::new(),
        });
}

#[given(
    regex = r"^that NAVI correlation has an entry linking NAVM 0x([0-9a-fA-F]+) and NAVM 0x([0-9a-fA-F]+) via door 0x([0-9a-fA-F]+)$"
)]
async fn given_nav_graph_navi_correlation_entry(
    world: &mut BevyoutWorld,
    navm_a_hex: String,
    navm_b_hex: String,
    door_hex: String,
) {
    let correlation = world
        .nav_graph_inputs
        .navi_correlations
        .last_mut()
        .expect("a NAVI correlation must be given first");
    correlation
        .entries
        .push(nav_graph::NavGraphNaviCorrelationEntryInput {
            navmesh_form_id: Some(parse_hex(&navm_a_hex)),
            other_navmesh_form_id: Some(parse_hex(&navm_b_hex)),
            door_form_id: Some(parse_hex(&door_hex)),
        });
}

#[given(regex = r"^landmass mesh 0x([0-9a-fA-F]+) polygon (\d+) is preferred-pathing$")]
async fn given_landmass_polygon_preferred_pathing(
    world: &mut BevyoutWorld,
    form_hex: String,
    index: u32,
) {
    let mesh = nav_backend_mesh_mut(world, &form_hex);
    let polygon = mesh
        .polygons
        .iter_mut()
        .find(|polygon| polygon.index == index)
        .expect("polygon must be given first");
    polygon.is_preferred_pathing = true;
}

// ---------------------------------------------------------------------
// nav_travel_lock.feature (issue #165 real-data acceptance follow-up) --
// appended section, do not interleave. `door_link::effective_door_open`
// is the pure decision the travel-arrival `Paused` arm in `nav/agent.rs`
// now consults: a `Travel` destination's hand-off must stay gated on
// lock state even when the door is physically open (left that way by a
// prior successful hand-off through the same door), while an ordinary
// `IntraCell` mid-route crossing keeps passing through an already-open
// door regardless of lock, exactly like `crossing_gate`.
// ---------------------------------------------------------------------

#[given(
    regex = r"^a door-link tick observation that is (open|closed), (locked|unlocked), and bound for (travel|intra-cell)$"
)]
async fn given_travel_lock_tick_observation(
    world: &mut BevyoutWorld,
    physical: String,
    lock: String,
    destination: String,
) {
    world.nav_travel_lock_physically_open = physical == "open";
    world.nav_travel_lock_locked = lock == "locked";
    world.nav_travel_lock_destination = Some(match destination.as_str() {
        "travel" => door_link::LinkDestination::Travel {
            destination_cell_form_id: 0xC0DE,
        },
        _ => door_link::LinkDestination::IntraCell,
    });
}

#[then(regex = r"^the effective door-open decision is (open|closed)$")]
async fn then_travel_lock_effective_open(world: &mut BevyoutWorld, expected: String) {
    let destination = world
        .nav_travel_lock_destination
        .expect("a door-link tick observation must be given first");
    let effective = door_link::effective_door_open(
        destination,
        world.nav_travel_lock_physically_open,
        world.nav_travel_lock_locked,
    );
    assert_eq!(effective, expected == "open");
}

/// Represents leg B's real-data shape: a *prior* hand-off left the door
/// physically open, so every tick observes `physically_open == true`
/// throughout -- only `effective_door_open`'s lock-authoritative rule for
/// `Travel` destinations keeps the agent from completing the crossing.
#[when(regex = r"^the door-link ticks (\d+) times with the door physically open and locked$")]
async fn when_door_link_ticks_n_open_and_locked(world: &mut BevyoutWorld, count: u32) {
    for _ in 0..count {
        let destination = match world.nav_backend_door_link_state {
            door_link::DoorLinkState::Paused { destination, .. } => destination,
            _ => door_link::LinkDestination::IntraCell,
        };
        let door_open = door_link::effective_door_open(destination, true, true);
        world.nav_backend_door_link_state = door_link::transition(
            world.nav_backend_door_link_state,
            door_link::DoorLinkEvent::Tick { door_open },
        );
    }
}

// ---------------------------------------------------------------------
// --- #153 collision-derived navmesh clearance steps (M4 wave 10) ---
// nav_collision_clearance.feature -- appended section, do not interleave.
// `vsa::prepare::nav_clearance` is std-only (no `super::` imports), so it is
// flat top-level included here the same way `door_link`/`repath` are.
// ---------------------------------------------------------------------

#[path = "../src/vsa/prepare/nav_clearance.rs"]
#[allow(dead_code, unused_imports)]
mod nav_clearance;

#[given("a clearance mesh")]
async fn given_clearance_mesh(world: &mut BevyoutWorld) {
    world.nav_clearance_mesh = nav_clearance::NavClearanceMeshInput::default();
    world.nav_clearance_collision = Vec::new();
    world.nav_clearance_result = None;
}

#[given(regex = r"^clearance mesh has vertex (\d+) at (-?[\d.]+), (-?[\d.]+), (-?[\d.]+)$")]
async fn given_clearance_vertex(world: &mut BevyoutWorld, index: usize, x: f32, y: f32, z: f32) {
    assert_eq!(
        world.nav_clearance_mesh.vertices.len(),
        index,
        "vertices must be given in order starting at 0"
    );
    world.nav_clearance_mesh.vertices.push([x, y, z]);
}

#[given(regex = r"^clearance mesh has polygon (\d+) with vertices (\d+),(\d+),(\d+)$")]
async fn given_clearance_polygon(world: &mut BevyoutWorld, index: usize, a: u32, b: u32, c: u32) {
    assert_eq!(
        world.nav_clearance_mesh.polygons.len(),
        index,
        "polygons must be given in order starting at 0"
    );
    world.nav_clearance_mesh.polygons.push([a, b, c]);
}

#[given(regex = r"^clearance mesh has protected edge (\d+),(\d+)$")]
async fn given_clearance_protected_edge(world: &mut BevyoutWorld, a: u32, b: u32) {
    world.nav_clearance_mesh.protected_edges.push((a, b));
}

#[given(
    regex = r"^a collision floor from (-?[\d.]+), (-?[\d.]+) by (-?[\d.]+), (-?[\d.]+) at height (-?[\d.]+)$"
)]
async fn given_collision_floor(
    world: &mut BevyoutWorld,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    y: f32,
) {
    world
        .nav_clearance_collision
        .push(nav_clearance::CollisionTriangle {
            vertices: [[x0, y, z0], [x1, y, z0], [x1, y, z1]],
        });
    world
        .nav_clearance_collision
        .push(nav_clearance::CollisionTriangle {
            vertices: [[x0, y, z0], [x1, y, z1], [x0, y, z1]],
        });
}

#[given(
    regex = r"^a collision wall from (-?[\d.]+), (-?[\d.]+) at z (-?[\d.]+) from (-?[\d.]+) to (-?[\d.]+)$"
)]
async fn given_collision_wall(
    world: &mut BevyoutWorld,
    x0: f32,
    x1: f32,
    z: f32,
    y0: f32,
    y1: f32,
) {
    world
        .nav_clearance_collision
        .push(nav_clearance::CollisionTriangle {
            vertices: [[x0, y0, z], [x1, y0, z], [x1, y1, z]],
        });
    world
        .nav_clearance_collision
        .push(nav_clearance::CollisionTriangle {
            vertices: [[x0, y0, z], [x1, y1, z], [x0, y1, z]],
        });
}

#[when("the clearance pass runs")]
async fn when_clearance_pass_runs(world: &mut BevyoutWorld) {
    world.nav_clearance_result = Some(nav_clearance::validate_and_clear(
        &world.nav_clearance_mesh,
        &world.nav_clearance_collision,
        nav_clearance::NavClearanceParams::default(),
    ));
}

fn clearance_result(world: &BevyoutWorld) -> &nav_clearance::NavClearanceResult {
    world
        .nav_clearance_result
        .as_ref()
        .expect("the clearance pass must run first")
}

#[then(regex = r"^(\d+) polygons? (?:is|are) removed as unsupported$")]
async fn then_clearance_removed(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.removed_unsupported, expected, "{result:?}");
}

#[then(regex = r"^(\d+) polygons? (?:is|are) cut as obstructed$")]
async fn then_clearance_cut(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.cut_obstructed, expected, "{result:?}");
}

#[then(regex = r"^(\d+) polygons? (?:is|are) dropped as unfit$")]
async fn then_clearance_dropped_exact(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.dropped_unfit, expected, "{result:?}");
}

#[then(regex = r"^at least (\d+) polygons? (?:is|are) dropped as unfit$")]
async fn then_clearance_dropped_at_least(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert!(
        result.dropped_unfit >= expected,
        "expected >= {expected} dropped, got {result:?}"
    );
}

#[then(regex = r"^the walkable set forms (\d+) connected component(?:s)?$")]
async fn then_clearance_components(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.component_count, expected, "{result:?}");
}

#[then(regex = r"^the largest connected component has (\d+) polygon(?:s)?$")]
async fn then_clearance_largest_component(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.largest_component, expected, "{result:?}");
}

#[then(regex = r"^clearance polygon (\d+) is walkable$")]
async fn then_clearance_polygon_walkable(world: &mut BevyoutWorld, index: usize) {
    let result = clearance_result(world);
    assert!(result.walkable[index], "polygon {index}: {result:?}");
}

#[then(regex = r"^clearance polygon (\d+) is not walkable$")]
async fn then_clearance_polygon_not_walkable(world: &mut BevyoutWorld, index: usize) {
    let result = clearance_result(world);
    assert!(!result.walkable[index], "polygon {index}: {result:?}");
}

#[then("at least one clearance polygon is walkable")]
async fn then_clearance_any_walkable(world: &mut BevyoutWorld) {
    let result = clearance_result(world);
    assert!(result.walkable.iter().any(|&w| w), "{result:?}");
}

#[then("at least one clearance polygon is not walkable")]
async fn then_clearance_any_not_walkable(world: &mut BevyoutWorld) {
    let result = clearance_result(world);
    assert!(result.walkable.iter().any(|&w| !w), "{result:?}");
}

#[then("every clearance polygon is walkable")]
async fn then_clearance_every_walkable(world: &mut BevyoutWorld) {
    let result = clearance_result(world);
    assert!(result.walkable.iter().all(|&w| w), "{result:?}");
}

#[then(regex = r"^the walkable count is (\d+)$")]
async fn then_clearance_walkable_count(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.walkable_count, expected, "{result:?}");
}

// ---------------------------------------------------------------------
// nav_portal_quarantine.feature (issue #162, M4 wave 10) -- appended
// section, do not interleave. Drives `viewer::nav::landmass_graph`'s pure
// `merge_link_kind`/`permitted_animation_link_kinds` directly -- the
// Bevy-side timeout/quarantine/repath wiring lives in `nav/agent.rs`'s own
// `#[cfg(test)]` unit tests (see this feature's own doc comment for why).
// ---------------------------------------------------------------------

#[given(regex = r"^merge-link candidate index (\d+)$")]
async fn given_merge_link_candidate_index(world: &mut BevyoutWorld, index: usize) {
    world.nav_quarantine_candidate_index = Some(index);
}

#[when("the merge-link kind is resolved")]
async fn when_merge_link_kind_resolved(world: &mut BevyoutWorld) {
    let index = world
        .nav_quarantine_candidate_index
        .expect("a merge-link candidate index must be given first");
    world.nav_quarantine_resolved_kind = Some(landmass_graph::merge_link_kind(index));
}

#[then(regex = r"^the merge-link kind is (\d+)$")]
async fn then_merge_link_kind(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(
        world.nav_quarantine_resolved_kind,
        Some(expected),
        "merge-link kind must resolve deterministically from its candidate index"
    );
}

#[given(regex = r"^(\d+) merge-link kinds exist$")]
async fn given_merge_link_kind_count(world: &mut BevyoutWorld, count: usize) {
    world.nav_quarantine_kind_count = count;
}

#[given(regex = r"^merge-link kind (\d+) is quarantined$")]
async fn given_merge_link_kind_quarantined(world: &mut BevyoutWorld, kind: usize) {
    world.nav_quarantine_excluded_kinds.insert(kind);
}

#[when("the permitted animation link kinds are computed")]
async fn when_permitted_animation_link_kinds_computed(world: &mut BevyoutWorld) {
    world.nav_quarantine_permitted = Some(landmass_graph::permitted_animation_link_kinds(
        &world.nav_quarantine_excluded_kinds,
        world.nav_quarantine_kind_count,
    ));
}

#[then("every animation link kind is permitted")]
async fn then_every_animation_link_kind_permitted(world: &mut BevyoutWorld) {
    let permitted = world
        .nav_quarantine_permitted
        .take()
        .expect("the permitted animation link kinds must be computed first");
    assert_eq!(
        permitted, None,
        "an empty quarantine must signal `PermittedAnimationLinks::All` (`None`), not an explicit full set"
    );
}

#[then(regex = r"^the permitted animation link kinds are (.+)$")]
async fn then_permitted_animation_link_kinds(world: &mut BevyoutWorld, expected: String) {
    let permitted = world
        .nav_quarantine_permitted
        .take()
        .expect("the permitted animation link kinds must be computed first")
        .expect("a non-empty quarantine must produce an explicit allow-list, not `All`");
    let expected: std::collections::BTreeSet<usize> = expected
        .split(',')
        .map(|value| value.trim().parse().expect("valid kind number"))
        .collect();
    assert_eq!(permitted, expected);
}

// --- #171 sub-triangle nav clearance steps (M4 wave 11) ---
// nav_collision_clearance.feature -- appended section, do not interleave.
// `vsa::prepare::nav_clip` is std-only (no `super::` imports) and is included
// here at the crate root so `nav_clearance`'s `use super::nav_clip::..` --
// `vsa::prepare::nav_clip` in the binary -- resolves the same way here.
#[path = "../src/vsa/prepare/nav_clip.rs"]
#[allow(dead_code)]
mod nav_clip;

/// Barycentric containment of `(x, z)` in a clipped result's polygon, on the
/// XZ plane. Local to this section so the steps do not depend on
/// `nav_clearance`'s private helpers.
fn clearance_polygon_contains(
    result: &nav_clearance::NavClearanceResult,
    polygon: usize,
    x: f32,
    z: f32,
) -> bool {
    let tri = result.polygons[polygon];
    let (Some(&a), Some(&b), Some(&c)) = (
        result.vertices.get(tri[0] as usize),
        result.vertices.get(tri[1] as usize),
        result.vertices.get(tri[2] as usize),
    ) else {
        return false;
    };
    let det = (b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2]);
    if det.abs() < 1.0e-9 {
        return false;
    }
    let beta = ((x - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (z - a[2])) / det;
    let gamma = ((b[0] - a[0]) * (z - a[2]) - (x - a[0]) * (b[2] - a[2])) / det;
    let alpha = 1.0 - beta - gamma;
    const EPS: f32 = 1.0e-4;
    alpha >= -EPS && beta >= -EPS && gamma >= -EPS
}

/// The lowest-index walkable polygon of `result` containing `(x, z)`.
fn clearance_walkable_polygon_at(
    result: &nav_clearance::NavClearanceResult,
    x: f32,
    z: f32,
) -> Option<usize> {
    (0..result.polygons.len())
        .find(|&index| result.walkable[index] && clearance_polygon_contains(result, index, x, z))
}

#[then(regex = r"^clearance point (-?[\d.]+), (-?[\d.]+) is walkable$")]
async fn then_clearance_point_walkable(world: &mut BevyoutWorld, x: f32, z: f32) {
    let result = clearance_result(world);
    assert!(
        clearance_walkable_polygon_at(result, x, z).is_some(),
        "({x}, {z}) must be walkable after clearance"
    );
}

#[then(regex = r"^clearance point (-?[\d.]+), (-?[\d.]+) is not walkable$")]
async fn then_clearance_point_not_walkable(world: &mut BevyoutWorld, x: f32, z: f32) {
    let result = clearance_result(world);
    assert!(
        clearance_walkable_polygon_at(result, x, z).is_none(),
        "({x}, {z}) must not be walkable after clearance"
    );
}

/// Whether two points sit in the same walkable connected component, over
/// shared polygon edges -- the routability question the clip has to answer.
fn clearance_same_component(
    result: &nav_clearance::NavClearanceResult,
    from: (f32, f32),
    to: (f32, f32),
) -> bool {
    let (Some(a), Some(b)) = (
        clearance_walkable_polygon_at(result, from.0, from.1),
        clearance_walkable_polygon_at(result, to.0, to.1),
    ) else {
        return false;
    };
    let mut edge_owners: std::collections::BTreeMap<(u32, u32), Vec<usize>> =
        std::collections::BTreeMap::new();
    for (index, tri) in result.polygons.iter().enumerate() {
        if !result.walkable[index] {
            continue;
        }
        for &(p, q) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            let key = if p <= q { (p, q) } else { (q, p) };
            edge_owners.entry(key).or_default().push(index);
        }
    }
    let mut seen = std::collections::BTreeSet::from([a]);
    let mut frontier = vec![a];
    while let Some(current) = frontier.pop() {
        if current == b {
            return true;
        }
        let tri = result.polygons[current];
        for &(p, q) in &[(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
            let key = if p <= q { (p, q) } else { (q, p) };
            for &owner in edge_owners.get(&key).into_iter().flatten() {
                if seen.insert(owner) {
                    frontier.push(owner);
                }
            }
        }
    }
    seen.contains(&b)
}

#[then(
    regex = r"^clearance points (-?[\d.]+), (-?[\d.]+) and (-?[\d.]+), (-?[\d.]+) are connected$"
)]
async fn then_clearance_points_connected(
    world: &mut BevyoutWorld,
    ax: f32,
    az: f32,
    bx: f32,
    bz: f32,
) {
    let result = clearance_result(world);
    assert!(
        clearance_same_component(result, (ax, az), (bx, bz)),
        "({ax}, {az}) and ({bx}, {bz}) must stay in one walkable component"
    );
}

#[then(
    regex = r"^clearance points (-?[\d.]+), (-?[\d.]+) and (-?[\d.]+), (-?[\d.]+) are not connected$"
)]
async fn then_clearance_points_not_connected(
    world: &mut BevyoutWorld,
    ax: f32,
    az: f32,
    bx: f32,
    bz: f32,
) {
    let result = clearance_result(world);
    assert!(
        !clearance_same_component(result, (ax, az), (bx, bz)),
        "({ax}, {az}) and ({bx}, {bz}) must not stay connected"
    );
}

#[then(regex = r"^at least (\d+) polygons? is removed as unsupported$")]
async fn then_clearance_removed_at_least(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert!(result.removed_unsupported >= expected, "{result:?}");
}

#[then(regex = r"^at least (\d+) polygons? is cut as obstructed$")]
async fn then_clearance_cut_at_least(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert!(result.cut_obstructed >= expected, "{result:?}");
}

// ---------------------------------------------------------------------
// ai_packages.feature (issue #175/#176, M4 wave 11 lane C) -- appended
// section, do not interleave. The template-inheritance priority-order
// scenario reuses actor_catalog.feature's own step vocabulary (`an NPC_
// actor ...`, `actor ... has template ... using ...`, `actor ... has
// package ...`, `a placement ...`, `the actor catalog is built`) plus one
// new Then step below; every other scenario drives
// `vsa::prepare::package_catalog` directly.
// ---------------------------------------------------------------------

#[then(regex = r#"^blueprint for reference 0x([0-9a-fA-F]+) has packages "([^"]*)" in order$"#)]
async fn then_blueprint_packages_in_order(
    world: &mut BevyoutWorld,
    reference_hex: String,
    expected: String,
) {
    let blueprint = actor_catalog_blueprint(world, &reference_hex);
    assert_eq!(blueprint.package_form_ids, parse_hex_list(&expected));
}

fn package_catalog_package_mut(
    world: &mut BevyoutWorld,
    form_id: u32,
) -> &mut package_catalog::PackageInput {
    world
        .package_catalog_inputs
        .packages
        .entry(form_id)
        .or_insert_with(|| package_catalog::PackageInput {
            form_id,
            ..package_catalog::PackageInput::default()
        })
}

#[given(regex = r"^a package 0x([0-9a-fA-F]+) with type (\d+)$")]
async fn given_package_with_type(world: &mut BevyoutWorld, hex: String, package_type: u8) {
    let form_id = parse_hex(&hex);
    package_catalog_package_mut(world, form_id).package_type = package_type;
}

#[given(
    regex = r"^package 0x([0-9a-fA-F]+) has schedule month (-?\d+) day (-?\d+) date (\d+) time (-?\d+) duration (-?\d+)$"
)]
async fn given_package_schedule(
    world: &mut BevyoutWorld,
    hex: String,
    month: i8,
    day_of_week: i8,
    date: u8,
    time: i8,
    duration: i32,
) {
    let form_id = parse_hex(&hex);
    package_catalog_package_mut(world, form_id).schedule =
        Some(package_catalog::PackageScheduleInput {
            month,
            day_of_week,
            date,
            time,
            duration,
        });
}

#[given(regex = r#"^package 0x([0-9a-fA-F]+) has unsupported subrecord "([^"]*)"$"#)]
async fn given_package_unsupported_subrecord(
    world: &mut BevyoutWorld,
    hex: String,
    subrecord: String,
) {
    let form_id = parse_hex(&hex);
    package_catalog_package_mut(world, form_id)
        .unsupported_subrecords
        .push(subrecord);
}

#[given(
    regex = r"^package 0x([0-9a-fA-F]+) has location type (\d+) target 0x([0-9a-fA-F]+) radius (-?\d+)$"
)]
async fn given_package_location(
    world: &mut BevyoutWorld,
    hex: String,
    location_type: u32,
    target_hex: String,
    radius: i32,
) {
    let form_id = parse_hex(&hex);
    let target = parse_hex(&target_hex);
    package_catalog_package_mut(world, form_id).location =
        Some(package_catalog::PackageLocationInput {
            location_type,
            form_id: Some(target),
            raw_value: target,
            radius,
        });
}

#[given(
    regex = r"^package 0x([0-9a-fA-F]+) has target type (-?\d+) target 0x([0-9a-fA-F]+) count (-?\d+)$"
)]
async fn given_package_target(
    world: &mut BevyoutWorld,
    hex: String,
    target_type: i32,
    target_hex: String,
    count_or_distance: i32,
) {
    let form_id = parse_hex(&hex);
    let target = parse_hex(&target_hex);
    package_catalog_package_mut(world, form_id).target =
        Some(package_catalog::PackageTargetInput {
            target_type,
            form_id: Some(target),
            raw_value: target,
            count_or_distance,
        });
}

#[given(regex = r#"^known package FormIDs "([^"]*)"$"#)]
async fn given_known_package_form_ids(world: &mut BevyoutWorld, hex_list: String) {
    world
        .package_catalog_inputs
        .known_form_ids
        .extend(parse_hex_list(&hex_list));
}

#[when("the package catalog is built")]
async fn when_package_catalog_built(world: &mut BevyoutWorld) {
    world.package_catalog_result = Some(package_catalog::build_package_catalog(
        &world.package_catalog_inputs,
        "fixture-fingerprint",
    ));
}

fn package_catalog_entry<'a>(
    world: &'a BevyoutWorld,
    hex: &str,
) -> &'a package_catalog::PreparedPackageEntry {
    let form_id = parse_hex(hex);
    world
        .package_catalog_result
        .as_ref()
        .expect("the package catalog must be built first")
        .packages
        .iter()
        .find(|entry| entry.form_id == form_id)
        .unwrap_or_else(|| panic!("no prepared package entry for {hex}"))
}

#[then(regex = r"^the package catalog has (\d+) packages?$")]
async fn then_package_catalog_count(world: &mut BevyoutWorld, expected: usize) {
    let catalog = world
        .package_catalog_result
        .as_ref()
        .expect("the package catalog must be built first");
    assert_eq!(catalog.packages.len(), expected);
}

#[then(regex = r"^package 0x([0-9a-fA-F]+) has no diagnostics$")]
async fn then_package_no_diagnostics(world: &mut BevyoutWorld, hex: String) {
    let entry = package_catalog_entry(world, &hex);
    assert!(
        entry.diagnostics.is_empty(),
        "expected no diagnostics, got {:?}",
        entry.diagnostics
    );
}

#[then(regex = r#"^package 0x([0-9a-fA-F]+) has diagnostic containing "([^"]*)"$"#)]
async fn then_package_diagnostic_containing(
    world: &mut BevyoutWorld,
    hex: String,
    expected: String,
) {
    let entry = package_catalog_entry(world, &hex);
    assert!(
        entry
            .diagnostics
            .iter()
            .any(|message| message.contains(expected.as_str())),
        "expected a diagnostic containing {expected:?} in {:?}",
        entry.diagnostics
    );
}

// ---------------------------------------------------------------------
// m5_wave2_ammunition.feature (issues #244-#247)
// ---------------------------------------------------------------------

#[given(regex = r"^a magazine for ammo ([0-9a-fA-F]{8}) with (\d+) of (\d+) rounds loaded$")]
async fn given_ammo_magazine(
    world: &mut BevyoutWorld,
    ammo_hex: String,
    loaded: u16,
    capacity: u16,
) {
    world.ammo_magazine = bevyout_core::combat::ammo::MagazineState {
        ammo_form_id: Some(parse_hex(&ammo_hex)),
        loaded,
    };
    world.ammo_capacity = capacity;
}

#[given(regex = r"^(\d+) compatible reserve rounds$")]
async fn given_ammo_reserve(world: &mut BevyoutWorld, reserve: u32) {
    world.ammo_reserve = reserve;
}

#[when(regex = r"^a reload with ammo ([0-9a-fA-F]{8}) is planned$")]
async fn when_ammo_reload_planned(world: &mut BevyoutWorld, ammo_hex: String) {
    world.ammo_reload = Some(
        bevyout_core::combat::ammo::plan_reload(
            world.ammo_magazine,
            parse_hex(&ammo_hex),
            world.ammo_capacity,
            world.ammo_reserve,
        )
        .expect("reload should be accepted"),
    );
}

#[when("one loaded round is consumed")]
async fn when_ammo_round_consumed(world: &mut BevyoutWorld) {
    world.ammo_fire = Some(bevyout_core::combat::ammo::consume_round(
        &mut world.ammo_magazine,
    ));
}

#[then(regex = r"^the reload kind is (operational|ammo switch)$")]
async fn then_ammo_reload_kind(world: &mut BevyoutWorld, expected: String) {
    let expected = match expected.as_str() {
        "operational" => bevyout_core::combat::ammo::ReloadKind::Operational,
        "ammo switch" => bevyout_core::combat::ammo::ReloadKind::AmmoSwitch,
        other => panic!("unexpected reload kind {other}"),
    };
    assert_eq!(world.ammo_reload.expect("reload planned").kind, expected);
}

#[then(regex = r"^the reload consumes (\d+) reserve rounds$")]
async fn then_ammo_reload_consumes(world: &mut BevyoutWorld, expected: u16) {
    assert_eq!(
        world.ammo_reload.expect("reload planned").consume_reserve,
        expected
    );
}

#[then(regex = r"^the reload returns (\d+) loaded rounds$")]
async fn then_ammo_reload_returns(world: &mut BevyoutWorld, expected: u16) {
    assert_eq!(
        world.ammo_reload.expect("reload planned").return_loaded,
        expected
    );
}

#[then("fire is blocked because the magazine is empty")]
async fn then_ammo_fire_blocked_empty(world: &mut BevyoutWorld) {
    assert_eq!(
        world.ammo_fire,
        Some(Err(bevyout_core::combat::ammo::FireBlockReason::Empty))
    );
}

#[then(regex = r"^the magazine still contains (\d+) rounds$")]
async fn then_ammo_magazine_loaded(world: &mut BevyoutWorld, expected: u16) {
    assert_eq!(world.ammo_magazine.loaded, expected);
}

#[then(
    regex = r"^the package catalog counts unsupported_type (\d+) unsupported_subrecord (\d+) deferred_subrecord (\d+) unresolved_location (\d+) unresolved_target (\d+) out_of_scope_location (\d+) out_of_scope_target (\d+)$"
)]
#[allow(clippy::too_many_arguments)]
async fn then_package_catalog_counts(
    world: &mut BevyoutWorld,
    unsupported_type: usize,
    unsupported_subrecord: usize,
    deferred_subrecord: usize,
    unresolved_location: usize,
    unresolved_target: usize,
    out_of_scope_location: usize,
    out_of_scope_target: usize,
) {
    let catalog = world
        .package_catalog_result
        .as_ref()
        .expect("the package catalog must be built first");
    assert_eq!(catalog.counters.unsupported_type, unsupported_type);
    assert_eq!(
        catalog.counters.unsupported_subrecord,
        unsupported_subrecord
    );
    assert_eq!(catalog.counters.deferred_subrecord, deferred_subrecord);
    assert_eq!(catalog.counters.unresolved_location, unresolved_location);
    assert_eq!(catalog.counters.unresolved_target, unresolved_target);
    assert_eq!(
        catalog.counters.out_of_scope_location,
        out_of_scope_location
    );
    assert_eq!(catalog.counters.out_of_scope_target, out_of_scope_target);
}

#[then(regex = r"^(\d+) polygons? (?:is|are) rejected as invalid geometry$")]
async fn then_clearance_invalid_geometry(world: &mut BevyoutWorld, expected: usize) {
    let result = clearance_result(world);
    assert_eq!(result.invalid_geometry, expected, "{result:?}");
}

// ---------------------------------------------------------------------
// actor_animation_gameflow.feature (#106, M4 wave 12) -- appended section.
// ---------------------------------------------------------------------

fn parse_gameplay_actor_state(value: &str) -> actor_animation_policy::ActorAnimationState {
    match value {
        "idle" => actor_animation_policy::ActorAnimationState::Idle,
        "walk" => actor_animation_policy::ActorAnimationState::Walk,
        "run" => actor_animation_policy::ActorAnimationState::Run,
        "turn_left" => actor_animation_policy::ActorAnimationState::TurnLeft,
        "turn_right" => actor_animation_policy::ActorAnimationState::TurnRight,
        "equip" => actor_animation_policy::ActorAnimationState::Equip,
        "unequip" => actor_animation_policy::ActorAnimationState::Unequip,
        other => panic!("unknown gameplay actor animation state {other:?}"),
    }
}

#[given(regex = r"^FO3 weapon animation type (\d+)$")]
async fn given_fo3_weapon_animation_type(world: &mut BevyoutWorld, value: u32) {
    world.gameplay_actor_weapon_type = Some(value);
}

#[when("the actor weapon animation prefix is resolved")]
async fn when_actor_weapon_animation_prefix_resolved(world: &mut BevyoutWorld) {
    world.gameplay_actor_weapon_prefix = Some(
        world
            .gameplay_actor_weapon_type
            .and_then(actor_animation_policy::weapon_animation_prefix),
    );
}

#[then(regex = r#"^the actor weapon animation prefix is \"([^\"]+)\"$"#)]
async fn then_actor_weapon_animation_prefix(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.gameplay_actor_weapon_prefix.flatten(),
        Some(expected.as_str())
    );
}

#[then("the actor weapon animation prefix is absent")]
async fn then_actor_weapon_animation_prefix_absent(world: &mut BevyoutWorld) {
    assert_eq!(world.gameplay_actor_weapon_prefix, Some(None));
}

fn parse_gameplay_clips(value: &str) -> Vec<actor_animation::PreparedActorAnimationClip> {
    value
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|entry| {
            let (name, source) = entry
                .split_once('@')
                .expect("clip fixtures use name@source-path");
            actor_animation::PreparedActorAnimationClip {
                name: name.to_owned(),
                source_kf_path: source.to_owned(),
                source_sequence_name: Some(
                    if name.contains("fastforward") {
                        "FastForward"
                    } else if name.contains("forward") {
                        "Forward"
                    } else if name.contains("turnleft") {
                        "TurnLeft"
                    } else if name.contains("turnright") {
                        "TurnRight"
                    } else if name.contains("unequip") {
                        "Unequip"
                    } else if name.contains("equip") {
                        "Equip"
                    } else {
                        "Idle"
                    }
                    .to_owned(),
                ),
                status: actor_animation::PreparedActorAnimationClipStatus::Ready,
                loop_mode: if name.contains("equip") {
                    actor_animation::PreparedActorAnimationLoopMode::Clamp
                } else {
                    actor_animation::PreparedActorAnimationLoopMode::Loop
                },
                ..Default::default()
            }
        })
        .collect()
}

#[given(regex = r#"^a (male|female) humanoid actor animation set with ready clips \"([^\"]*)\"$"#)]
async fn given_humanoid_gameplay_animation_set(
    world: &mut BevyoutWorld,
    sex: String,
    clips: String,
) {
    world.gameplay_actor_kind = actor_animation::PreparedActorAnimationKind::Npc;
    world.gameplay_actor_female = sex == "female";
    world.gameplay_actor_clips = parse_gameplay_clips(&clips);
}

#[given(regex = r#"^a creature actor animation set with ready clips \"([^\"]*)\"$"#)]
async fn given_creature_gameplay_animation_set(world: &mut BevyoutWorld, clips: String) {
    world.gameplay_actor_kind = actor_animation::PreparedActorAnimationKind::Creature;
    world.gameplay_actor_female = false;
    world.gameplay_actor_clips = parse_gameplay_clips(&clips);
}

#[given(regex = r#"^the actor uses weapon animation prefix \"([^\"]+)\"$"#)]
async fn given_actor_weapon_animation_prefix(world: &mut BevyoutWorld, prefix: String) {
    world.gameplay_actor_weapon_prefix = Some(Some(match prefix.as_str() {
        "h2h" => "h2h",
        "1hm" => "1hm",
        "2hm" => "2hm",
        "1hp" => "1hp",
        "2hr" => "2hr",
        "2ha" => "2ha",
        "2hh" => "2hh",
        "2hl" => "2hl",
        "1gt" => "1gt",
        "1lm" => "1lm",
        "1md" => "1md",
        other => panic!("unsupported fixture prefix {other}"),
    }));
}

#[given(regex = r#"^the actor requests animation state \"([^\"]+)\"$"#)]
async fn given_actor_requests_animation_state(world: &mut BevyoutWorld, state: String) {
    world.gameplay_actor_requested_state = parse_gameplay_actor_state(&state);
}

#[when("the gameplay actor clip is resolved")]
async fn when_gameplay_actor_clip_resolved(world: &mut BevyoutWorld) {
    world.gameplay_actor_selection = actor_animation_policy::resolve_clip(
        &world.gameplay_actor_clips,
        actor_animation_policy::ActorAnimationContext {
            kind: world.gameplay_actor_kind,
            female: world.gameplay_actor_female,
            weapon_prefix: world.gameplay_actor_weapon_prefix.flatten(),
        },
        world.gameplay_actor_requested_state,
    );
}

#[then(regex = r#"^the gameplay actor clip is \"([^\"]+)\"$"#)]
async fn then_gameplay_actor_clip(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .gameplay_actor_selection
            .as_ref()
            .map(|selection| selection.clip_name.as_str()),
        Some(expected.as_str())
    );
}

#[then(regex = r#"^the gameplay actor clip source is \"([^\"]+)\"$"#)]
async fn then_gameplay_actor_clip_source(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .gameplay_actor_selection
            .as_ref()
            .map(|selection| selection.source_path.as_str()),
        Some(expected.as_str())
    );
}

#[then(regex = r#"^the gameplay actor clip resolution reports fallback from \"([^\"]+)\"$"#)]
async fn then_gameplay_actor_clip_fallback(world: &mut BevyoutWorld, expected: String) {
    let expected = parse_gameplay_actor_state(&expected);
    assert_eq!(
        world
            .gameplay_actor_selection
            .as_ref()
            .and_then(|selection| selection.fallback_from),
        Some(expected)
    );
}

#[given(regex = r#"^the gameplay actor is playing animation state \"([^\"]+)\"$"#)]
async fn given_gameplay_actor_playing_state(world: &mut BevyoutWorld, state: String) {
    world.gameplay_actor_requested_state = parse_gameplay_actor_state(&state);
}

#[when("the gameplay actor animation finishes")]
async fn when_gameplay_actor_animation_finishes(world: &mut BevyoutWorld) {
    world.gameplay_actor_next_state = Some(actor_animation_policy::state_after_completion(
        world.gameplay_actor_requested_state,
    ));
}

#[then(regex = r#"^the next gameplay actor animation state is \"([^\"]+)\"$"#)]
async fn then_next_gameplay_actor_animation_state(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.gameplay_actor_next_state,
        Some(parse_gameplay_actor_state(&expected))
    );
}

#[given("a gameplay actor belongs to an inactive resident cell")]
async fn given_gameplay_actor_in_inactive_cell(world: &mut BevyoutWorld) {
    world.gameplay_actor_cell_active = false;
}

#[given("a gameplay actor belongs to the active resident cell")]
async fn given_gameplay_actor_in_active_cell(world: &mut BevyoutWorld) {
    world.gameplay_actor_cell_active = true;
}

#[given("the gameplay actor is disabled")]
async fn given_gameplay_actor_disabled(world: &mut BevyoutWorld) {
    world.gameplay_actor_visible = false;
}

#[given("the gameplay actor is visible")]
async fn given_gameplay_actor_visible(world: &mut BevyoutWorld) {
    world.gameplay_actor_visible = true;
}

#[when("gameplay actor activity is resolved")]
async fn when_gameplay_actor_activity_resolved(world: &mut BevyoutWorld) {
    world.gameplay_actor_playback_active = Some(actor_animation_policy::should_advance_playback(
        world.gameplay_actor_cell_active,
        world.gameplay_actor_visible,
    ));
}

#[then("gameplay actor playback is paused")]
async fn then_gameplay_actor_playback_paused(world: &mut BevyoutWorld) {
    assert_eq!(world.gameplay_actor_playback_active, Some(false));
}

#[then("gameplay actor playback advances")]
async fn then_gameplay_actor_playback_advances(world: &mut BevyoutWorld) {
    assert_eq!(world.gameplay_actor_playback_active, Some(true));
}

// ---------------------------------------------------------------------
// nav_derived_doors.feature (issue #177, M4 wave 11) -- appended section,
// do not interleave. `vsa::prepare::nav_doors` is std-only (no `super::`
// imports), so it is flat top-level included here the same way
// `nav_clearance`/`nav_clip` are.
// ---------------------------------------------------------------------

#[path = "../src/vsa/prepare/nav_doors.rs"]
#[allow(dead_code, unused_imports)]
mod nav_doors;

fn derived_door_blocker(world: &mut BevyoutWorld, form_id: u32) -> &mut nav_doors::BlockerVolume {
    world
        .nav_derived_door_blockers
        .iter_mut()
        .find(|blocker| blocker.reference_form_id == form_id)
        .expect("the blocker must be declared first")
}

#[given(
    regex = r"^a blocker 0x([0-9a-fA-F]{8}) with footprint from (-?[\d.]+), (-?[\d.]+) to (-?[\d.]+), (-?[\d.]+) spanning height (-?[\d.]+) to (-?[\d.]+)$"
)]
#[allow(clippy::too_many_arguments)]
async fn given_derived_door_blocker(
    world: &mut BevyoutWorld,
    form_id: String,
    min_x: f32,
    min_z: f32,
    max_x: f32,
    max_z: f32,
    min_y: f32,
    max_y: f32,
) {
    let reference_form_id = u32::from_str_radix(&form_id, 16).expect("hex blocker FormID");
    world
        .nav_derived_door_blockers
        .push(nav_doors::BlockerVolume {
            reference_form_id,
            // Counter-clockwise in (x, z), matching `navmesh::convex_hull_xz`'s
            // winding -- the real footprints come from that hull.
            footprint: vec![
                [min_x, min_z],
                [max_x, min_z],
                [max_x, max_z],
                [min_x, max_z],
            ],
            min_y,
            max_y,
            gated: true,
            // Issue #189 feature 3: the raw collision solid the footprint
            // above is the hull of. The derivation reads the footprint and
            // the invariant check reads these triangles, by different
            // primitives, so the check can disagree with the derivation
            // instead of only ever echoing it.
            collision_triangles: vec![
                [
                    [min_x, min_y, min_z],
                    [max_x, min_y, min_z],
                    [max_x, min_y, max_z],
                ],
                [
                    [min_x, min_y, min_z],
                    [max_x, min_y, max_z],
                    [min_x, min_y, max_z],
                ],
            ],
        });
}

#[given(regex = r"^blocker 0x([0-9a-fA-F]{8}) has no open and close controls$")]
async fn given_derived_door_blocker_ungated(world: &mut BevyoutWorld, form_id: String) {
    let form_id = u32::from_str_radix(&form_id, 16).expect("hex blocker FormID");
    derived_door_blocker(world, form_id).gated = false;
}

#[given(
    regex = r"^nav mesh 0x([0-9a-fA-F]{8}) has walkable polygon (\d+) with vertices (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) and (-?[\d.]+), (-?[\d.]+), (-?[\d.]+) and (-?[\d.]+), (-?[\d.]+), (-?[\d.]+)$"
)]
#[allow(clippy::too_many_arguments)]
async fn given_derived_door_polygon(
    world: &mut BevyoutWorld,
    mesh_form_id: String,
    index: u32,
    ax: f32,
    ay: f32,
    az: f32,
    bx: f32,
    by: f32,
    bz: f32,
    cx: f32,
    cy: f32,
    cz: f32,
) {
    let mesh_form_id = u32::from_str_radix(&mesh_form_id, 16).expect("hex mesh FormID");
    let polygon = nav_doors::BlockerPolygonInput {
        index,
        vertices: [[ax, ay, az], [bx, by, bz], [cx, cy, cz]],
    };
    match world
        .nav_derived_door_meshes
        .iter_mut()
        .find(|mesh| mesh.form_id == mesh_form_id)
    {
        Some(mesh) => mesh.polygons.push(polygon),
        None => world
            .nav_derived_door_meshes
            .push(nav_doors::BlockerMeshInput {
                form_id: mesh_form_id,
                polygons: vec![polygon],
                authored_door_polygons: std::collections::BTreeSet::new(),
            }),
    }
}

#[when("the derived door associations are resolved")]
async fn when_derived_door_associations_resolved(world: &mut BevyoutWorld) {
    world.nav_derived_door_associations = Some(nav_doors::derive_door_associations(
        &world.nav_derived_door_meshes,
        &world.nav_derived_door_blockers,
    ));
}

fn derived_door_associations(world: &BevyoutWorld) -> &[nav_doors::DerivedDoorAssociation] {
    world
        .nav_derived_door_associations
        .as_deref()
        .expect("the derived door associations must be resolved first")
}

#[then(regex = r"^there are exactly (\d+) derived door associations$")]
async fn then_derived_door_association_count(world: &mut BevyoutWorld, expected: usize) {
    let associations = derived_door_associations(world);
    assert_eq!(associations.len(), expected, "{associations:?}");
}

#[then(regex = r"^there are exactly (\d+) blocking door associations$")]
async fn then_derived_door_blocking_count(world: &mut BevyoutWorld, expected: usize) {
    let associations = derived_door_associations(world);
    let blocking = associations
        .iter()
        .filter(|association| association.blocks_when_closed)
        .count();
    assert_eq!(blocking, expected, "{associations:?}");
}

#[then(regex = r"^polygon (\d+) is a (gate|blocking) association for blocker 0x([0-9a-fA-F]{8})$")]
async fn then_derived_door_association_class(
    world: &mut BevyoutWorld,
    index: u32,
    class: String,
    form_id: String,
) {
    let form_id = u32::from_str_radix(&form_id, 16).expect("hex blocker FormID");
    let associations = derived_door_associations(world);
    let association = associations
        .iter()
        .find(|association| {
            association.triangle_index == index && association.door_reference_form_id == form_id
        })
        .unwrap_or_else(|| panic!("no association for polygon {index}: {associations:?}"));
    assert_eq!(
        association.blocks_when_closed,
        class == "blocking",
        "{association:?}"
    );
}

#[then("no walkable polygon is left unreported inside a closed blocker")]
async fn then_no_unreported_interior_polygon(world: &mut BevyoutWorld) {
    let unreported = nav_doors::unreported_interior_polygons(
        &world.nav_derived_door_meshes,
        &world.nav_derived_door_blockers,
        derived_door_associations(world),
    );
    assert!(unreported.is_empty(), "{unreported:?}");
}

#[then(regex = r"^the derived association order is (.+)$")]
async fn then_derived_door_association_order(world: &mut BevyoutWorld, expected: String) {
    let actual: Vec<String> = derived_door_associations(world)
        .iter()
        .map(|association| {
            format!(
                "0x{:08x}/{}",
                association.door_reference_form_id, association.triangle_index
            )
        })
        .collect();
    assert_eq!(actual.join(", "), expected);
}

#[then("resolving the derived door associations again gives the same result")]
async fn then_derived_door_associations_are_stable(world: &mut BevyoutWorld) {
    let again = nav_doors::derive_door_associations(
        &world.nav_derived_door_meshes,
        &world.nav_derived_door_blockers,
    );
    assert_eq!(derived_door_associations(world), again);
}

#[then(regex = r"^polygon (\d+) is reported as (openable|not openable)$")]
async fn then_derived_door_association_openable(
    world: &mut BevyoutWorld,
    index: u32,
    expectation: String,
) {
    let associations = derived_door_associations(world);
    let association = associations
        .iter()
        .find(|association| association.triangle_index == index)
        .unwrap_or_else(|| panic!("no association for polygon {index}: {associations:?}"));
    assert_eq!(
        association.openable,
        expectation == "openable",
        "{association:?}"
    );
}

#[given(regex = r"^an agent (stalled|still moving) ([\d.]+) metres from a door crossing$")]
async fn given_approach_agent(world: &mut BevyoutWorld, progress: String, distance: f32) {
    world.nav_approach_observation = Some(door_link::ApproachObservation {
        distance_to_crossing: distance,
        agent_distance_to_target: 0.0,
        crossing_distance_to_target: 0.0,
        stalled: progress == "stalled",
    });
}

#[given(
    regex = r"^the crossing is ([\d.]+) metres from the target and the agent is ([\d.]+) metres from it$"
)]
async fn given_approach_distances(world: &mut BevyoutWorld, crossing: f32, agent: f32) {
    let observation = world
        .nav_approach_observation
        .as_mut()
        .expect("the agent must be placed first");
    observation.crossing_distance_to_target = crossing;
    observation.agent_distance_to_target = agent;
}

#[then(regex = r"^the approach gate (fires|does not fire)$")]
async fn then_approach_gate(world: &mut BevyoutWorld, outcome: String) {
    let observation = world
        .nav_approach_observation
        .expect("the agent must be placed first");
    assert_eq!(
        door_link::approach_gate(observation),
        outcome == "fires",
        "{observation:?}"
    );
}

// ---------------------------------------------------------------------
// actor_state.feature (#110, M4 wave 13) -- appended shared seam.
// ---------------------------------------------------------------------

fn actor_value(label: &str) -> actor_state::ActorValue {
    actor_state::ActorValue::parse(label).unwrap_or_else(|| panic!("unknown actor value {label:?}"))
}

fn actor_state_by_reference_mut(
    store: &mut actor_state::ActorStateStore,
    reference_form_id: u32,
) -> &mut actor_state::ActorInstanceState {
    store
        .cells
        .values_mut()
        .find_map(|actors| actors.get_mut(&reference_form_id))
        .unwrap_or_else(|| panic!("actor reference {reference_form_id:08x} was not seeded"))
}

fn actor_state_by_reference(
    store: &actor_state::ActorStateStore,
    reference_form_id: u32,
) -> &actor_state::ActorInstanceState {
    store
        .cells
        .values()
        .find_map(|actors| actors.get(&reference_form_id))
        .unwrap_or_else(|| panic!("actor reference {reference_form_id:08x} was not seeded"))
}

#[given(
    regex = r"^an actor definition with template health (-?[\d.]+) and base health (-?[\d.]+)$"
)]
async fn given_actor_template_and_base_health(world: &mut BevyoutWorld, template: f32, base: f32) {
    world.actor_state_definition.base_form_id = 1;
    world.actor_state_definition.reference_form_id = 2;
    world
        .actor_state_definition
        .template_values
        .insert(actor_state::ActorValue::Health, template);
    world
        .actor_state_definition
        .base_values
        .insert(actor_state::ActorValue::Health, base);
    world.actor_state_instance =
        actor_state::ActorInstanceState::new(2, actor_state::ActorLifeState::Alive);
}

#[given(regex = r"^an actor definition with template fatigue (-?[\d.]+) and no base fatigue$")]
async fn given_actor_template_fatigue(world: &mut BevyoutWorld, template: f32) {
    world.actor_state_definition.base_form_id = 1;
    world.actor_state_definition.reference_form_id = 2;
    world
        .actor_state_definition
        .template_values
        .insert(actor_state::ActorValue::Fatigue, template);
    world.actor_state_instance =
        actor_state::ActorInstanceState::new(2, actor_state::ActorLifeState::Alive);
}

#[given(
    regex = r"^the actor has race health modifier (-?[\d.]+), class health modifier (-?[\d.]+), and faction health modifier (-?[\d.]+)$"
)]
async fn given_actor_health_modifiers(
    world: &mut BevyoutWorld,
    race: f32,
    class: f32,
    faction: f32,
) {
    world
        .actor_state_definition
        .race_modifiers
        .insert(actor_state::ActorValue::Health, race);
    world
        .actor_state_definition
        .class_modifiers
        .insert(actor_state::ActorValue::Health, class);
    world
        .actor_state_definition
        .faction_modifiers
        .insert(actor_state::ActorValue::Health, faction);
}

#[given(regex = r"^the actor instance has runtime (health|fatigue) mutation (-?[\d.]+)$")]
async fn given_actor_value_mutation(world: &mut BevyoutWorld, value: String, mutation: f32) {
    world
        .actor_state_instance
        .set_value_mutation(actor_value(&value), mutation)
        .unwrap();
}

#[when(regex = r"^the actor (health|fatigue) is resolved$")]
async fn when_actor_value_resolved(world: &mut BevyoutWorld, value: String) {
    world.actor_state_resolved = Some(
        world
            .actor_state_definition
            .resolve_value(&world.actor_state_instance, actor_value(&value)),
    );
}

#[then(regex = r"^the effective actor (?:health|fatigue) is (-?[\d.]+)$")]
async fn then_effective_actor_value(world: &mut BevyoutWorld, expected: f32) {
    assert_eq!(
        world
            .actor_state_resolved
            .expect("actor value must be resolved")
            .effective,
        expected
    );
}

#[then("the persisted actor state contains no derived value snapshot")]
async fn then_actor_state_has_no_derived_snapshot(world: &mut BevyoutWorld) {
    let serialized = ron::ser::to_string(&world.actor_state_instance).unwrap();
    assert!(!serialized.contains("effective"), "{serialized}");
}

#[given(
    regex = r"^actor reference 0x([0-9a-fA-F]+) belongs to faction 0x([0-9a-fA-F]+) at rank (-?\d+)$"
)]
async fn given_runtime_actor_faction(
    world: &mut BevyoutWorld,
    reference: String,
    faction: String,
    rank: i8,
) {
    world.actor_state_definition.base_form_id = 1;
    world.actor_state_definition.reference_form_id = parse_hex(&reference);
    world
        .actor_state_definition
        .factions
        .push(actor_state::ActorFactionMembership {
            faction_form_id: parse_hex(&faction),
            rank,
            title: None,
        });
}

#[then(regex = r"^actor reference 0x([0-9a-fA-F]+) has faction 0x([0-9a-fA-F]+) at rank (-?\d+)$")]
async fn then_actor_faction(
    world: &mut BevyoutWorld,
    reference: String,
    faction: String,
    rank: i8,
) {
    assert_eq!(
        world.actor_state_definition.reference_form_id,
        parse_hex(&reference)
    );
    assert!(
        world
            .actor_state_definition
            .factions
            .iter()
            .any(|membership| {
                membership.faction_form_id == parse_hex(&faction) && membership.rank == rank
            })
    );
}

#[then("the actor definition contains no hostility decision")]
async fn then_actor_definition_has_no_hostility(world: &mut BevyoutWorld) {
    let serialized = ron::ser::to_string(&world.actor_state_definition).unwrap();
    assert!(!serialized.contains("hostil"), "{serialized}");
}

#[given("an empty actor state store")]
async fn given_empty_actor_state_store(world: &mut BevyoutWorld) {
    world.actor_state_store = actor_state::ActorStateStore::default();
}

#[when(
    regex = r"^actor reference 0x([0-9a-fA-F]+) in cell 0x([0-9a-fA-F]+) is seeded alive(?: again)?$"
)]
async fn when_actor_seeded_alive(world: &mut BevyoutWorld, reference: String, cell: String) {
    world
        .actor_state_store
        .seed(
            parse_hex(&cell),
            parse_hex(&reference),
            actor_state::ActorLifeState::Alive,
        )
        .unwrap();
}

#[when(regex = r"^actor reference 0x([0-9a-fA-F]+) receives runtime health mutation (-?[\d.]+)$")]
async fn when_stored_actor_mutated(world: &mut BevyoutWorld, reference: String, mutation: f32) {
    actor_state_by_reference_mut(&mut world.actor_state_store, parse_hex(&reference))
        .set_value_mutation(actor_state::ActorValue::Health, mutation)
        .unwrap();
}

#[then(regex = r"^actor reference 0x([0-9a-fA-F]+) has runtime health mutation (-?[\d.]+)$")]
async fn then_stored_actor_mutation(world: &mut BevyoutWorld, reference: String, expected: f32) {
    assert_eq!(
        actor_state_by_reference(&world.actor_state_store, parse_hex(&reference)).value_mutations
            [&actor_state::ActorValue::Health],
        expected
    );
}

#[then("exactly one actor instance is stored")]
async fn then_one_actor_instance_stored(world: &mut BevyoutWorld) {
    assert_eq!(world.actor_state_store.len(), 1);
}

#[given(regex = r"^actor reference 0x([0-9a-fA-F]+) in cell 0x([0-9a-fA-F]+) is dead$")]
async fn given_dead_actor(world: &mut BevyoutWorld, reference: String, cell: String) {
    world
        .actor_state_store
        .seed(
            parse_hex(&cell),
            parse_hex(&reference),
            actor_state::ActorLifeState::Dead,
        )
        .unwrap();
}

#[given(
    regex = r"^actor reference 0x([0-9a-fA-F]+) is running package 0x([0-9a-fA-F]+) procedure (\d+) for ([\d.]+) seconds$"
)]
async fn given_actor_package_checkpoint(
    world: &mut BevyoutWorld,
    reference: String,
    package: String,
    procedure_index: u32,
    elapsed_seconds: f32,
) {
    actor_state_by_reference_mut(&mut world.actor_state_store, parse_hex(&reference)).package =
        Some(actor_state::ActorPackageCheckpoint {
            package_form_id: parse_hex(&package),
            procedure_index,
            elapsed_seconds,
        });
}

#[when("the actor state store is serialized and restored")]
async fn when_actor_state_store_round_trips(world: &mut BevyoutWorld) {
    let serialized = ron::ser::to_string(&world.actor_state_store).unwrap();
    world.actor_state_store = ron::de::from_str(&serialized).unwrap();
    world.actor_state_serialized = Some(serialized);
}

#[then(regex = r"^actor reference 0x([0-9a-fA-F]+) remains dead$")]
async fn then_actor_remains_dead(world: &mut BevyoutWorld, reference: String) {
    assert_eq!(
        actor_state_by_reference(&world.actor_state_store, parse_hex(&reference)).life_state,
        actor_state::ActorLifeState::Dead
    );
}

#[then(
    regex = r"^actor reference 0x([0-9a-fA-F]+) retains package 0x([0-9a-fA-F]+) procedure (\d+) at ([\d.]+) seconds$"
)]
async fn then_actor_retains_package(
    world: &mut BevyoutWorld,
    reference: String,
    package: String,
    procedure_index: u32,
    elapsed_seconds: f32,
) {
    assert_eq!(
        actor_state_by_reference(&world.actor_state_store, parse_hex(&reference)).package,
        Some(actor_state::ActorPackageCheckpoint {
            package_form_id: parse_hex(&package),
            procedure_index,
            elapsed_seconds,
        })
    );
}

#[given(
    regex = r"^canonical actor reference 0x([0-9a-fA-F]+) owns item instance (\d+) with (\d+) of base item 0x([0-9a-fA-F]+)$"
)]
async fn given_canonical_actor_item(
    world: &mut BevyoutWorld,
    reference: String,
    instance: u64,
    count: u32,
    base: String,
) {
    let item = item_transaction::ItemInstance::new(
        item_transaction::ItemInstanceId(instance),
        parse_hex(&base),
        count,
        item_transaction::ItemState::default(),
    )
    .unwrap();
    world
        .canonical_ledger
        .insert_holder(
            item_transaction::HolderId::Actor {
                reference_form_id: parse_hex(&reference),
            },
            item_transaction::ItemHolderState {
                items: vec![item],
                ..Default::default()
            },
        )
        .unwrap();
}

#[when(regex = r"^canonical actor reference 0x([0-9a-fA-F]+) is projected twice$")]
async fn when_canonical_actor_projected_twice(world: &mut BevyoutWorld, reference: String) {
    let holder = item_transaction::HolderId::Actor {
        reference_form_id: parse_hex(&reference),
    };
    for _ in 0..2 {
        if !world.canonical_ledger.holders().contains_key(&holder) {
            world
                .canonical_ledger
                .insert_holder(holder, item_transaction::ItemHolderState::default())
                .unwrap();
        }
    }
}

#[then(
    regex = r"^canonical actor reference 0x([0-9a-fA-F]+) still owns (\d+) items in one instance$"
)]
async fn then_canonical_actor_items_unchanged(
    world: &mut BevyoutWorld,
    reference: String,
    expected_count: u32,
) {
    let holder = &world.canonical_ledger.holders()[&item_transaction::HolderId::Actor {
        reference_form_id: parse_hex(&reference),
    }];
    assert_eq!(holder.items.len(), 1);
    assert_eq!(holder.items[0].count, expected_count);
}

// ---------------------------------------------------------------------
// nav_locomotion.feature (issue #188): the pure achieved-motion ->
// locomotion-clip policy that drives a nav-bound actor's animation.
// ---------------------------------------------------------------------

fn parse_locomotion_state(label: &str) -> locomotion::LocomotionState {
    match label {
        "idle" => locomotion::LocomotionState::Idle,
        "walk" => locomotion::LocomotionState::Walk,
        "run" => locomotion::LocomotionState::Run,
        "turn_left" => locomotion::LocomotionState::TurnLeft,
        "turn_right" => locomotion::LocomotionState::TurnRight,
        other => panic!("unknown locomotion state {other:?}"),
    }
}

fn step_locomotion(
    world: &mut BevyoutWorld,
    observation: locomotion::LocomotionObservation,
) -> locomotion::LocomotionState {
    let previous = world.nav_locomotion_state;
    let next = locomotion::next_locomotion_state(previous, observation);
    if next != previous {
        world.nav_locomotion_changed = true;
    }
    world.nav_locomotion_state = next;
    next
}

#[given(regex = r"^a bound actor currently in the (\w+) locomotion state$")]
async fn given_locomotion_state(world: &mut BevyoutWorld, state: String) {
    world.nav_locomotion_state = parse_locomotion_state(&state);
    world.nav_locomotion_changed = false;
}

#[when(regex = r"^its achieved horizontal speed is ([\d.]+) metres per second$")]
async fn when_locomotion_speed(world: &mut BevyoutWorld, speed: f32) {
    step_locomotion(
        world,
        locomotion::LocomotionObservation {
            achieved_horizontal_speed: speed,
            yaw_rate: 0.0,
        },
    );
}

#[when(regex = r"^it is stationary and its yaw rate is (-?[\d.]+) radians per second$")]
async fn when_locomotion_yaw(world: &mut BevyoutWorld, yaw_rate: f32) {
    step_locomotion(
        world,
        locomotion::LocomotionObservation {
            achieved_horizontal_speed: 0.0,
            yaw_rate,
        },
    );
}

#[when(
    regex = r"^its achieved horizontal speed is ([\d.]+) metres per second and its yaw rate is (-?[\d.]+) radians per second$"
)]
async fn when_locomotion_speed_and_yaw(world: &mut BevyoutWorld, speed: f32, yaw_rate: f32) {
    step_locomotion(
        world,
        locomotion::LocomotionObservation {
            achieved_horizontal_speed: speed,
            yaw_rate,
        },
    );
}

#[when(regex = r"^navigation desires ([\d.]+) metres per second but the KCC achieves ([\d.]+)$")]
async fn when_locomotion_wedged(world: &mut BevyoutWorld, _desired: f32, achieved: f32) {
    // The desired speed is deliberately not an input to the policy: a
    // wedged agent must not stride on the spot. Accepting it here and
    // discarding it keeps that fact visible in the scenario.
    step_locomotion(
        world,
        locomotion::LocomotionObservation {
            achieved_horizontal_speed: achieved,
            yaw_rate: 0.0,
        },
    );
}

#[when(
    regex = r"^its achieved horizontal speed oscillates (\d+) times between ([\d.]+) and ([\d.]+) metres per second$"
)]
async fn when_locomotion_oscillates(world: &mut BevyoutWorld, cycles: u32, low: f32, high: f32) {
    for _ in 0..cycles {
        for speed in [low, high] {
            step_locomotion(
                world,
                locomotion::LocomotionObservation {
                    achieved_horizontal_speed: speed,
                    yaw_rate: 0.0,
                },
            );
        }
    }
}

#[then(regex = r"^its locomotion state becomes (\w+)$")]
async fn then_locomotion_state(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.nav_locomotion_state,
        parse_locomotion_state(&expected),
        "locomotion state"
    );
}

#[then(regex = r"^its locomotion state never changed$")]
async fn then_locomotion_never_changed(world: &mut BevyoutWorld) {
    assert!(
        !world.nav_locomotion_changed,
        "the locomotion state flapped: {:?}",
        world.nav_locomotion_state
    );
}

// ---------------------------------------------------------------------
// nav_derived_doors.feature (issue #189, M4 walking-actors wave, lane B) --
// appended section, do not interleave. Steps for the independent
// interior-polygon invariant path (feature 3), which reads the blocker's
// collision triangles rather than the derivation's footprint.
// ---------------------------------------------------------------------

#[given(
    regex = r"^blocker 0x([0-9a-fA-F]{8}) has a mis-derived footprint from (-?[\d.]+), (-?[\d.]+) to (-?[\d.]+), (-?[\d.]+)$"
)]
async fn given_derived_door_blocker_wrong_footprint(
    world: &mut BevyoutWorld,
    form_id: String,
    min_x: f32,
    min_z: f32,
    max_x: f32,
    max_z: f32,
) {
    let form_id = u32::from_str_radix(&form_id, 16).expect("hex blocker FormID");
    // Only the footprint moves; `collision_triangles` keeps the blocker's real
    // solid, so the derivation and the invariant now disagree -- which is the
    // whole point of the independent path.
    derived_door_blocker(world, form_id).footprint = vec![
        [min_x, min_z],
        [max_x, min_z],
        [max_x, max_z],
        [min_x, max_z],
    ];
}

#[then(
    regex = r"^polygon (\d+) of mesh 0x([0-9a-fA-F]{8}) is reported unreported inside blocker 0x([0-9a-fA-F]{8})$"
)]
async fn then_polygon_is_reported_unreported(
    world: &mut BevyoutWorld,
    index: u32,
    mesh_form_id: String,
    blocker_form_id: String,
) {
    let mesh_form_id = u32::from_str_radix(&mesh_form_id, 16).expect("hex mesh FormID");
    let blocker_form_id = u32::from_str_radix(&blocker_form_id, 16).expect("hex blocker FormID");
    let unreported = nav_doors::unreported_interior_polygons(
        &world.nav_derived_door_meshes,
        &world.nav_derived_door_blockers,
        derived_door_associations(world),
    );
    assert!(
        unreported.contains(&(mesh_form_id, index, blocker_form_id)),
        "{unreported:?}"
    );
}

// ---------------------------------------------------------------------
// console_qol.feature (issue #201)
// ---------------------------------------------------------------------

#[given(regex = r#"^console history contains \"([^\"]*)\"$"#)]
async fn given_console_history(world: &mut BevyoutWorld, commands: String) {
    world.console_history =
        console_openmw_ui::CommandHistory::from_entries(commands.split(',').map(str::to_owned));
}

#[given(regex = r#"^the console transcript contains \"([^\"]*)\"$"#)]
async fn given_console_transcript(world: &mut BevyoutWorld, lines: String) {
    for line in lines.split(',') {
        world.console_transcript.push(line);
    }
}

#[given("an empty console transcript")]
async fn given_empty_console_transcript(world: &mut BevyoutWorld) {
    world.console_transcript.clear();
}

#[when(regex = r#"^the console submission \"([^\"]*)\" is applied$"#)]
async fn when_console_submission_applied(world: &mut BevyoutWorld, command: String) {
    world.console_history.record(&command);
    if console_openmw_ui::is_clear_submission(&command) {
        world.console_transcript.clear();
    }
}

#[when(regex = r"^(\d+) numbered console lines are appended$")]
async fn when_numbered_console_lines_appended(world: &mut BevyoutWorld, count: usize) {
    for index in 0..count {
        world.console_transcript.push(format!("line {index}"));
    }
}

#[then("the console transcript is empty")]
async fn then_console_transcript_empty(world: &mut BevyoutWorld) {
    assert!(world.console_transcript.is_empty());
}

#[then(regex = r#"^console history is \"([^\"]*)\"$"#)]
async fn then_console_history_is(world: &mut BevyoutWorld, commands: String) {
    assert_eq!(
        world
            .console_history
            .entries()
            .collect::<Vec<_>>()
            .join(","),
        commands
    );
}

#[then(regex = r"^the console transcript contains (\d+) lines$")]
async fn then_console_transcript_line_count(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.console_transcript.len(), count);
}

#[then(regex = r#"^the first retained console line is \"([^\"]*)\"$"#)]
async fn then_first_retained_console_line(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.console_transcript.lines().next(),
        Some(expected.as_str())
    );
}

#[then(regex = r#"^the last retained console line is \"([^\"]*)\"$"#)]
async fn then_last_retained_console_line(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.console_transcript.lines().last(),
        Some(expected.as_str())
    );
}

// ---------------------------------------------------------------------
// pause_menu.feature -- pure ESC pause menu selection state
// ---------------------------------------------------------------------

#[given(regex = r"^a fresh pause menu$")]
async fn given_fresh_pause_menu(world: &mut BevyoutWorld) {
    world.pause_menu = pause_menu::PauseMenuState::new();
    world.pause_menu_action = None;
}

#[when(regex = r"^the pause menu moves up$")]
async fn when_pause_menu_moves_up(world: &mut BevyoutWorld) {
    world.pause_menu.move_up();
}

#[when(regex = r"^the pause menu moves down$")]
async fn when_pause_menu_moves_down(world: &mut BevyoutWorld) {
    world.pause_menu.move_down();
}

#[when(regex = r"^the pause menu selects (Continue|Save|Load|Settings|Help|Quit)$")]
async fn when_pause_menu_selects(world: &mut BevyoutWorld, option: String) {
    world.pause_menu.select(parse_pause_menu_option(&option));
}

#[then(regex = r"^the pause menu selection is (Continue|Save|Load|Settings|Help|Quit)$")]
async fn then_pause_menu_selection(world: &mut BevyoutWorld, option: String) {
    assert_eq!(
        world.pause_menu.selected(),
        parse_pause_menu_option(&option),
        "pause menu selection"
    );
}

#[then(regex = r"^activating the pause menu yields Continue$")]
async fn then_pause_menu_yields_continue(world: &mut BevyoutWorld) {
    world.pause_menu_action = Some(world.pause_menu.activate());
    assert_eq!(
        world.pause_menu_action,
        Some(Some(pause_menu::PauseMenuAction::Continue))
    );
}

#[then(regex = r"^activating the pause menu yields Quit$")]
async fn then_pause_menu_yields_quit(world: &mut BevyoutWorld) {
    world.pause_menu_action = Some(world.pause_menu.activate());
    assert_eq!(
        world.pause_menu_action,
        Some(Some(pause_menu::PauseMenuAction::Quit))
    );
}

#[then(regex = r"^activating the pause menu yields nothing$")]
async fn then_pause_menu_yields_nothing(world: &mut BevyoutWorld) {
    world.pause_menu_action = Some(world.pause_menu.activate());
    assert_eq!(world.pause_menu_action, Some(None));
}

#[then(
    regex = r#"^pause menu option (Continue|Save|Load|Settings|Help|Quit) is labeled "([^"]+)"$"#
)]
async fn then_pause_menu_option_label(world: &mut BevyoutWorld, option: String, label: String) {
    let _ = world;
    assert_eq!(parse_pause_menu_option(&option).label(), label);
}

fn parse_pause_menu_option(label: &str) -> pause_menu::PauseMenuOption {
    match label {
        "Continue" => pause_menu::PauseMenuOption::Continue,
        "Save" => pause_menu::PauseMenuOption::Save,
        "Load" => pause_menu::PauseMenuOption::Load,
        "Settings" => pause_menu::PauseMenuOption::Settings,
        "Help" => pause_menu::PauseMenuOption::Help,
        "Quit" => pause_menu::PauseMenuOption::Quit,
        other => panic!("unknown pause menu option {other:?}"),
    }
}

// ---------------------------------------------------------------------
// ai_package_selection.feature / ai_package_lifecycle.feature /
// ai_package_resolution.feature (issues #193/#194/#195) -- appended
// section, do not interleave. Each drives the pure runtime module directly.
// ---------------------------------------------------------------------

/// Builds a minimal 20-byte CTDA for the selection steps: operator in the top
/// three bits of byte 0, the comparison value at offset 4, function index at
/// offset 8 -- the exact header `ai_selection::decode_condition` reads.
fn build_ctda(operator: &str, comparison_value: f32, function_index: u16) -> Vec<u8> {
    let op_bits: u8 = match operator {
        "equal" => 0,
        "not-equal" => 1,
        "greater" => 2,
        "greater-or-equal" => 3,
        "less" => 4,
        "less-or-equal" => 5,
        other => panic!("unknown CTDA operator {other:?}"),
    };
    let mut bytes = vec![0u8; 20];
    bytes[0] = op_bits << 5;
    bytes[4..8].copy_from_slice(&comparison_value.to_le_bytes());
    bytes[8..10].copy_from_slice(&function_index.to_le_bytes());
    bytes
}

fn ai_sel_candidate_mut(
    world: &mut BevyoutWorld,
    form_id: u32,
) -> &mut ai_selection::PackageCandidate {
    if let Some(index) = world
        .ai_sel_candidates
        .iter()
        .position(|candidate| candidate.form_id == form_id)
    {
        &mut world.ai_sel_candidates[index]
    } else {
        world
            .ai_sel_candidates
            .push(ai_selection::PackageCandidate {
                form_id,
                ..ai_selection::PackageCandidate::default()
            });
        world.ai_sel_candidates.last_mut().unwrap()
    }
}

#[given(regex = r"^a selection game hour of ([\d.]+)$")]
async fn given_selection_game_hour(world: &mut BevyoutWorld, hour: f32) {
    world.ai_sel_hour = hour;
}

#[given(regex = r"^a package candidate 0x([0-9a-fA-F]+) of type (\d+)$")]
async fn given_package_candidate(world: &mut BevyoutWorld, hex: String, package_type: u8) {
    let form_id = parse_hex(&hex);
    ai_sel_candidate_mut(world, form_id).package_type = package_type;
}

#[given(regex = r"^candidate 0x([0-9a-fA-F]+) has schedule time (-?\d+) duration (-?\d+)$")]
async fn given_candidate_schedule(world: &mut BevyoutWorld, hex: String, time: i8, duration: i32) {
    let form_id = parse_hex(&hex);
    ai_sel_candidate_mut(world, form_id).schedule = Some(ai_selection::PackageSchedule {
        time,
        duration,
        ..ai_selection::PackageSchedule::default()
    });
}

#[given(regex = r"^condition function (\d+) returns ([\d.]+)$")]
async fn given_condition_function_returns(world: &mut BevyoutWorld, index: u16, value: f32) {
    world.ai_sel_functions.insert(index, value);
}

#[given(
    regex = r"^candidate 0x([0-9a-fA-F]+) requires function (\d+) (equal|not-equal|greater|greater-or-equal|less|less-or-equal) ([\d.]+)$"
)]
async fn given_candidate_condition(
    world: &mut BevyoutWorld,
    hex: String,
    function_index: u16,
    operator: String,
    value: f32,
) {
    let form_id = parse_hex(&hex);
    let ctda = build_ctda(&operator, value, function_index);
    ai_sel_candidate_mut(world, form_id).conditions.push(ctda);
}

/// A boundary backed by the World's `(function_index) -> value` map.
struct MapFunctions(std::collections::HashMap<u16, f32>);

impl ai_selection::ConditionFunctions for MapFunctions {
    fn evaluate(&self, function_index: u16, _param1: u32, _param2: u32) -> Option<f32> {
        self.0.get(&function_index).copied()
    }
}

#[when("the actor's package is selected")]
async fn when_actor_package_selected(world: &mut BevyoutWorld) {
    let now = ai_selection::GameInstant {
        hour: world.ai_sel_hour,
        ..ai_selection::GameInstant::default()
    };
    let boundary = MapFunctions(world.ai_sel_functions.clone());
    world.ai_sel_report = Some(ai_selection::select_package(
        &world.ai_sel_candidates,
        now,
        &boundary,
    ));
}

fn ai_sel_report(world: &BevyoutWorld) -> &ai_selection::SelectionReport {
    world
        .ai_sel_report
        .as_ref()
        .expect("the package must be selected first")
}

#[then(regex = r"^the selected package is 0x([0-9a-fA-F]+)$")]
async fn then_selected_package_is(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(ai_sel_report(world).selected, Some(parse_hex(&hex)));
}

#[then("no package is selected")]
async fn then_no_package_selected(world: &mut BevyoutWorld) {
    assert_eq!(ai_sel_report(world).selected, None);
}

#[then(regex = r#"^package candidate 0x([0-9a-fA-F]+) was rejected as "([^"]*)"$"#)]
async fn then_candidate_rejected_as(world: &mut BevyoutWorld, hex: String, reason: String) {
    let form_id = parse_hex(&hex);
    let evaluation = ai_sel_report(world)
        .evaluations
        .iter()
        .find(|evaluation| evaluation.form_id == form_id)
        .unwrap_or_else(|| panic!("no evaluation for candidate {hex}"));
    match evaluation.outcome {
        ai_selection::CandidateOutcome::Rejected(actual) => assert_eq!(actual.label(), reason),
        ai_selection::CandidateOutcome::Selected => {
            panic!("candidate {hex} was selected, expected rejection {reason:?}")
        }
    }
}

#[then(
    regex = r"^the selection counts unsupported_type (\d+) out_of_schedule (\d+) conditions_false (\d+) conditions_unevaluable (\d+) schedule_gap (\d+)$"
)]
async fn then_selection_counts(
    world: &mut BevyoutWorld,
    unsupported_type: usize,
    out_of_schedule: usize,
    conditions_false: usize,
    conditions_unevaluable: usize,
    schedule_gap: usize,
) {
    let counters = ai_sel_report(world).counters;
    assert_eq!(
        counters.unsupported_type, unsupported_type,
        "unsupported_type"
    );
    assert_eq!(counters.out_of_schedule, out_of_schedule, "out_of_schedule");
    assert_eq!(
        counters.conditions_false, conditions_false,
        "conditions_false"
    );
    assert_eq!(
        counters.conditions_unevaluable, conditions_unevaluable,
        "conditions_unevaluable"
    );
    assert_eq!(counters.schedule_gap, schedule_gap, "schedule_gap");
}

// -- ai_package_lifecycle.feature --

#[given("a fresh package lifecycle")]
async fn given_fresh_lifecycle(world: &mut BevyoutWorld) {
    world.ai_lifecycle = ai_lifecycle::PackageLifecycle::new();
    world.ai_lifecycle_checkpoint = None;
}

#[given(regex = r"^a fresh package lifecycle with backoff ([\d.]+) and max retries (\d+)$")]
async fn given_fresh_lifecycle_with_policy(
    world: &mut BevyoutWorld,
    backoff: f32,
    max_retries: u32,
) {
    world.ai_lifecycle =
        ai_lifecycle::PackageLifecycle::new().with_retry_policy(max_retries, backoff);
    world.ai_lifecycle_checkpoint = None;
}

#[when(regex = r"^package 0x([0-9a-fA-F]+) is selected$")]
async fn when_lifecycle_package_selected(world: &mut BevyoutWorld, hex: String) {
    world.ai_lifecycle.on_select(Some(parse_hex(&hex)));
}

#[when("no package is selected for the lifecycle")]
async fn when_lifecycle_no_package(world: &mut BevyoutWorld) {
    world.ai_lifecycle.on_select(None);
}

#[when(regex = r"^the active package advances (\d+) steps$")]
async fn when_active_advances(world: &mut BevyoutWorld, steps: u32) {
    for _ in 0..steps {
        world.ai_lifecycle.advance_step();
    }
}

#[when("the active package completes")]
async fn when_active_completes(world: &mut BevyoutWorld) {
    world.ai_lifecycle.complete();
}

#[when("the active package fails")]
async fn when_active_fails(world: &mut BevyoutWorld) {
    world.ai_lifecycle.fail();
}

#[when(regex = r"^the lifecycle ticks ([\d.]+) seconds$")]
async fn when_lifecycle_ticks(world: &mut BevyoutWorld, seconds: f32) {
    world.ai_lifecycle.tick(seconds);
}

#[when("the lifecycle checkpoint is persisted")]
async fn when_lifecycle_checkpoint_persisted(world: &mut BevyoutWorld) {
    // Route the checkpoint through the same serde the save format's ACTR
    // record uses (`ActorInstanceState`), proving the persisted shape
    // survives a save/load round-trip deterministically.
    let checkpoint = world
        .ai_lifecycle
        .to_checkpoint()
        .expect("a running lifecycle snapshots");
    let mut state =
        bevyout_core::actor_state::ActorInstanceState::new(1, actor_state::ActorLifeState::Alive);
    state.package = Some(checkpoint);
    let json = serde_json::to_string(&state).expect("serialize actor state");
    let restored: bevyout_core::actor_state::ActorInstanceState =
        serde_json::from_str(&json).expect("deserialize actor state");
    world.ai_lifecycle_checkpoint = restored.package;
}

#[when("the lifecycle is rebuilt from the checkpoint")]
async fn when_lifecycle_rebuilt(world: &mut BevyoutWorld) {
    let checkpoint = world
        .ai_lifecycle_checkpoint
        .expect("a checkpoint must be persisted first");
    world.ai_lifecycle = ai_lifecycle::PackageLifecycle::from_checkpoint(checkpoint);
}

#[then(regex = r"^the lifecycle phase is (idle|running|paused|completed|awaiting-retry|failed)$")]
async fn then_lifecycle_phase(world: &mut BevyoutWorld, phase: String) {
    assert_eq!(world.ai_lifecycle.phase().label(), phase);
}

#[then(regex = r"^the active package is 0x([0-9a-fA-F]+)$")]
async fn then_active_package_is(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(world.ai_lifecycle.active_form_id(), Some(parse_hex(&hex)));
}

#[then(regex = r"^the paused package is 0x([0-9a-fA-F]+)$")]
async fn then_paused_package_is(world: &mut BevyoutWorld, hex: String) {
    assert_eq!(world.ai_lifecycle.paused_form_id(), Some(parse_hex(&hex)));
}

#[then(regex = r"^the active step is (\d+)$")]
async fn then_active_step_is(world: &mut BevyoutWorld, step_index: u32) {
    assert_eq!(world.ai_lifecycle.step(), Some(step_index));
}

#[then(regex = r"^the active elapsed is ([\d.]+)$")]
async fn then_active_elapsed_is(world: &mut BevyoutWorld, elapsed: f32) {
    assert_eq!(world.ai_lifecycle.elapsed_seconds(), Some(elapsed));
}

#[then(regex = r"^the retry count is (\d+)$")]
async fn then_retry_count_is(world: &mut BevyoutWorld, count: u32) {
    assert_eq!(world.ai_lifecycle.retry_count(), Some(count));
}

// -- ai_package_resolution.feature --

#[given(regex = r"^the resolving actor is at ([\d.-]+) ([\d.-]+) ([\d.-]+)$")]
async fn given_resolving_actor_at(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.ai_res_context.actor_position = [x, y, z];
}

#[given(
    regex = r"^a resolvable reference 0x([0-9a-fA-F]+) of base 0x([0-9a-fA-F]+) at ([\d.-]+) ([\d.-]+) ([\d.-]+)$"
)]
async fn given_resolvable_reference(
    world: &mut BevyoutWorld,
    reference_hex: String,
    base_hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let reference_form_id = parse_hex(&reference_hex);
    let base_form_id = parse_hex(&base_hex);
    world.ai_res_context.references.insert(
        reference_form_id,
        ai_resolution::ResolvedReference {
            reference_form_id,
            base_form_id,
            cell_form_id: 0x1000,
            position: [x, y, z],
            entity: Some(u64::from(reference_form_id)),
            linked_reference: None,
            orientation_yaw: None,
        },
    );
    world
        .ai_res_context
        .bases
        .entry(base_form_id)
        .or_default()
        .push(reference_form_id);
}

#[given(regex = r"^a package location of type (\d+) referencing 0x([0-9a-fA-F]+) radius (-?\d+)$")]
async fn given_package_location_input(
    world: &mut BevyoutWorld,
    location_type: u32,
    form_hex: String,
    radius: i32,
) {
    let raw = parse_hex(&form_hex);
    world.ai_res_location = Some(ai_resolution::PackageLocation {
        location_type,
        form_id: (raw != 0).then_some(raw),
        raw_value: raw,
        radius,
    });
}

#[given(
    regex = r"^a package target of type (-?\d+) referencing 0x([0-9a-fA-F]+) distance (-?\d+)$"
)]
async fn given_package_target_input(
    world: &mut BevyoutWorld,
    target_type: i32,
    form_hex: String,
    distance: i32,
) {
    let raw = parse_hex(&form_hex);
    world.ai_res_target = Some(ai_resolution::PackageTarget {
        target_type,
        form_id: (raw != 0).then_some(raw),
        raw_value: raw,
        count_or_distance: distance,
    });
}

#[when("the package location is resolved")]
async fn when_location_resolved(world: &mut BevyoutWorld) {
    let location = world.ai_res_location.expect("a location must be set first");
    world.ai_res_result = Some(ai_resolution::resolve_location(
        &location,
        &world.ai_res_context,
    ));
}

#[when("the package target is resolved")]
async fn when_target_resolved(world: &mut BevyoutWorld) {
    let target = world.ai_res_target.expect("a target must be set first");
    world.ai_res_result = Some(ai_resolution::resolve_target(
        &target,
        &world.ai_res_context,
    ));
}

fn ai_res_point(world: &BevyoutWorld) -> &ai_resolution::ResolvedPoint {
    match world
        .ai_res_result
        .as_ref()
        .expect("resolution must run first")
    {
        Ok(point) => point,
        Err(diagnostic) => panic!("expected a resolved point, got diagnostic: {diagnostic}"),
    }
}

#[then(regex = r"^the location resolves to ([\d.-]+) ([\d.-]+) ([\d.-]+)$")]
async fn then_location_resolves_to(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    assert_eq!(ai_res_point(world).position, [x, y, z]);
}

#[then(regex = r"^the target resolves to ([\d.-]+) ([\d.-]+) ([\d.-]+)$")]
async fn then_target_resolves_to(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    assert_eq!(ai_res_point(world).position, [x, y, z]);
}

#[then(regex = r#"^the location resolves via "([^"]*)"$"#)]
async fn then_location_resolves_via(world: &mut BevyoutWorld, label: String) {
    assert_eq!(ai_res_point(world).source.label(), label);
}

#[then(regex = r"^the target radius is ([\d.]+)$")]
async fn then_target_radius_is(world: &mut BevyoutWorld, radius: f32) {
    assert!((ai_res_point(world).radius - radius).abs() < 1e-5);
}

#[then(regex = r#"^the location is unresolved with diagnostic containing "([^"]*)"$"#)]
async fn then_location_unresolved(world: &mut BevyoutWorld, expected: String) {
    match world
        .ai_res_result
        .as_ref()
        .expect("resolution must run first")
    {
        Ok(point) => panic!("expected an unresolved location, got {point:?}"),
        Err(diagnostic) => assert!(
            diagnostic.message.contains(&expected),
            "diagnostic {:?} did not contain {expected:?}",
            diagnostic.message
        ),
    }
}

#[then(regex = r#"^the target is unresolved with diagnostic containing "([^"]*)"$"#)]
async fn then_target_unresolved(world: &mut BevyoutWorld, expected: String) {
    match world
        .ai_res_result
        .as_ref()
        .expect("resolution must run first")
    {
        Ok(point) => panic!("expected an unresolved target, got {point:?}"),
        Err(diagnostic) => assert!(
            diagnostic.message.contains(&expected),
            "diagnostic {:?} did not contain {expected:?}",
            diagnostic.message
        ),
    }
}

// ============================================================================
// issue #116: faction disposition/hostility and target perception/awareness.
// Pure-policy steps driving `bevyout_core::{faction, disposition, perception}`.
// ============================================================================

fn parse_faction_id(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).expect("faction id must be hex")
}

fn parse_aggression(label: &str) -> disposition::Aggression {
    match label {
        "unaggressive" => disposition::Aggression::Unaggressive,
        "aggressive" => disposition::Aggression::Aggressive,
        "very_aggressive" => disposition::Aggression::VeryAggressive,
        "frenzied" => disposition::Aggression::Frenzied,
        other => panic!("unknown aggression {other:?}"),
    }
}

fn register_known_faction(table: &mut faction::FactionRelationTable, id: u32) {
    table
        .factions
        .entry(id)
        .or_insert_with(|| faction::PreparedFaction {
            form_id: id,
            ..Default::default()
        });
}

fn upsert_relation(
    table: &mut faction::FactionRelationTable,
    from: u32,
    to: u32,
    modifier: i32,
    reaction: faction::GroupCombatReaction,
) {
    register_known_faction(table, to);
    let entry = table
        .factions
        .entry(from)
        .or_insert_with(|| faction::PreparedFaction {
            form_id: from,
            ..Default::default()
        });
    entry
        .relations
        .retain(|relation| relation.faction_form_id != to);
    entry.relations.push(faction::FactionRelation {
        faction_form_id: to,
        modifier,
        reaction,
    });
}

fn observer_member(world: &mut BevyoutWorld, id: u32) {
    world
        .hostility_observer
        .factions
        .push(disposition::FactionMembership {
            faction_form_id: id,
            rank: 0,
        });
}

#[given(regex = r#"^faction "(\w+)" and "(\w+)" are enemies$"#)]
async fn given_factions_enemies(world: &mut BevyoutWorld, a: String, b: String) {
    let (a, b) = (parse_faction_id(&a), parse_faction_id(&b));
    upsert_relation(
        &mut world.hostility_table,
        a,
        b,
        -80,
        faction::GroupCombatReaction::Enemy,
    );
    upsert_relation(
        &mut world.hostility_table,
        b,
        a,
        -80,
        faction::GroupCombatReaction::Enemy,
    );
}

#[given(regex = r#"^faction "(\w+)" and "(\w+)" are allies$"#)]
async fn given_factions_allies(world: &mut BevyoutWorld, a: String, b: String) {
    let (a, b) = (parse_faction_id(&a), parse_faction_id(&b));
    upsert_relation(
        &mut world.hostility_table,
        a,
        b,
        50,
        faction::GroupCombatReaction::Ally,
    );
    upsert_relation(
        &mut world.hostility_table,
        b,
        a,
        50,
        faction::GroupCombatReaction::Ally,
    );
}

#[given(
    regex = r#"^faction "(\w+)" applies a disposition modifier (-?\d+) toward faction "(\w+)"$"#
)]
async fn given_faction_modifier(
    world: &mut BevyoutWorld,
    from: String,
    modifier: String,
    to: String,
) {
    let (from, to) = (parse_faction_id(&from), parse_faction_id(&to));
    let modifier: i32 = modifier.parse().expect("modifier must be an integer");
    upsert_relation(
        &mut world.hostility_table,
        from,
        to,
        modifier,
        faction::GroupCombatReaction::Neutral,
    );
    register_known_faction(&mut world.hostility_table, from);
}

#[given(
    regex = r#"^a hostility observer in faction "(\w+)" with base disposition (-?\d+) and aggression "(\w+)"$"#
)]
async fn given_observer_in_faction(
    world: &mut BevyoutWorld,
    faction_id: String,
    base: String,
    aggression: String,
) {
    let id = parse_faction_id(&faction_id);
    register_known_faction(&mut world.hostility_table, id);
    world.hostility_observer.base_disposition = base.parse().expect("base disposition");
    world.hostility_observer.aggression = parse_aggression(&aggression);
    observer_member(world, id);
}

#[given(
    regex = r#"^a hostility observer in unknown faction "(\w+)" with base disposition (-?\d+) and aggression "(\w+)"$"#
)]
async fn given_observer_in_unknown_faction(
    world: &mut BevyoutWorld,
    faction_id: String,
    base: String,
    aggression: String,
) {
    // Deliberately does not register the faction in the table -> unresolved.
    let id = parse_faction_id(&faction_id);
    world.hostility_observer.base_disposition = base.parse().expect("base disposition");
    world.hostility_observer.aggression = parse_aggression(&aggression);
    observer_member(world, id);
}

#[given(regex = r#"^a hostility observer with base disposition (-?\d+) and aggression "(\w+)"$"#)]
async fn given_observer_no_faction(world: &mut BevyoutWorld, base: String, aggression: String) {
    world.hostility_observer.base_disposition = base.parse().expect("base disposition");
    world.hostility_observer.aggression = parse_aggression(&aggression);
}

#[given(regex = r#"^the target is in faction "(\w+)"$"#)]
async fn given_target_in_faction(world: &mut BevyoutWorld, faction_id: String) {
    let id = parse_faction_id(&faction_id);
    register_known_faction(&mut world.hostility_table, id);
    world
        .hostility_target
        .factions
        .push(disposition::FactionMembership {
            faction_form_id: id,
            rank: 0,
        });
}

#[given(regex = r"^the target is the player$")]
async fn given_target_is_player(world: &mut BevyoutWorld) {
    world.hostility_target = disposition::DispositionTarget::default();
}

#[given(regex = r"^the target is the observer itself$")]
async fn given_target_is_self(world: &mut BevyoutWorld) {
    world.hostility_target.is_self = true;
}

#[when(regex = r"^hostility is resolved$")]
async fn when_hostility_resolved(world: &mut BevyoutWorld) {
    let thresholds = disposition::DispositionThresholds::default();
    world.hostility_result = Some(disposition::resolve_disposition(
        &world.hostility_observer,
        &world.hostility_target,
        &world.hostility_table,
        &thresholds,
    ));
}

#[then(regex = r#"^the hostility verdict is "(\w+)"$"#)]
async fn then_hostility_verdict(world: &mut BevyoutWorld, expected: String) {
    let result = world.hostility_result.as_ref().expect("resolve first");
    assert_eq!(result.hostility.label(), expected);
}

#[then(regex = r#"^the deciding rule is "(\w+)"$"#)]
async fn then_deciding_rule(world: &mut BevyoutWorld, expected: String) {
    let result = world.hostility_result.as_ref().expect("resolve first");
    assert_eq!(result.decided_by.label(), expected);
}

#[then(regex = r"^the resolved disposition is (-?\d+)$")]
async fn then_resolved_disposition(world: &mut BevyoutWorld, expected: String) {
    let expected: i32 = expected.parse().expect("disposition integer");
    let result = world.hostility_result.as_ref().expect("resolve first");
    assert_eq!(result.disposition, expected);
}

#[then(regex = r#"^a hostility diagnostic mentions "([^"]+)"$"#)]
async fn then_hostility_diagnostic(world: &mut BevyoutWorld, needle: String) {
    let result = world.hostility_result.as_ref().expect("resolve first");
    assert!(
        result.diagnostics.iter().any(|d| d.contains(&needle)),
        "no diagnostic mentioning {needle:?} in {:?}",
        result.diagnostics
    );
}

fn fast_perception_config() -> perception::PerceptionConfig {
    perception::PerceptionConfig {
        sight_range: 40.0,
        view_cone_half_angle: std::f32::consts::FRAC_PI_2,
        acquire_confidence: 0.5,
        lose_confidence: 0.1,
        gain_per_second: 1.0,
        decay_per_second: 1.0,
        forget_seconds: 2.0,
    }
}

#[given(regex = r"^a perception observer$")]
async fn given_perception_observer(world: &mut BevyoutWorld) {
    world.perception_config = Some(fast_perception_config());
    world.perception_state = perception::AwarenessState::default();
    world.perception_candidates.clear();
}

#[given(regex = r"^a perception observer that has acquired the player$")]
async fn given_perception_observer_acquired(world: &mut BevyoutWorld) {
    world.perception_config = Some(fast_perception_config());
    world.perception_state = perception::AwarenessState {
        confidence: 1.0,
        acquired: Some(perception::TargetId::player()),
        ..Default::default()
    };
    world.perception_candidates.clear();
}

fn player_candidate(distance: f32, angle: f32, los: bool) -> perception::PerceptionInputs {
    perception::PerceptionInputs {
        target: perception::TargetId::player(),
        position: [0.0, 0.0, -distance],
        distance,
        angle_to_target: angle,
        has_line_of_sight: los,
        detectable: true,
    }
}

#[given(regex = r"^a player target ([0-9.]+) metres ahead in clear view$")]
async fn given_player_ahead_clear(world: &mut BevyoutWorld, distance: String) {
    let distance: f32 = distance.parse().expect("distance");
    world.perception_candidates = vec![player_candidate(distance, 0.0, true)];
}

#[given(regex = r"^a player target ([0-9.]+) metres ahead but occluded$")]
async fn given_player_ahead_occluded(world: &mut BevyoutWorld, distance: String) {
    let distance: f32 = distance.parse().expect("distance");
    world.perception_candidates = vec![player_candidate(distance, 0.0, false)];
}

#[given(regex = r"^a player target ([0-9.]+) metres behind in clear view$")]
async fn given_player_behind_clear(world: &mut BevyoutWorld, distance: String) {
    let distance: f32 = distance.parse().expect("distance");
    world.perception_candidates = vec![player_candidate(distance, std::f32::consts::PI, true)];
}

#[given(regex = r"^the target has disappeared$")]
async fn given_target_disappeared(world: &mut BevyoutWorld) {
    world.perception_candidates.clear();
}

#[when(regex = r"^perception advances for ([0-9.]+) seconds$")]
async fn when_perception_advances(world: &mut BevyoutWorld, seconds: String) {
    let seconds: f32 = seconds.parse().expect("seconds");
    let config = world.perception_config.expect("configure observer first");
    world.perception_last_event = Some(world.perception_state.update(
        &world.perception_candidates,
        &config,
        seconds,
    ));
}

#[when(regex = r"^the awareness state is serialized and reloaded$")]
async fn when_awareness_round_trip(world: &mut BevyoutWorld) {
    let text = ron::ser::to_string(&world.perception_state).expect("serialize awareness");
    world.perception_state = ron::de::from_str(&text).expect("deserialize awareness");
}

#[then(regex = r"^the observer has acquired the player$")]
async fn then_observer_acquired_player(world: &mut BevyoutWorld) {
    assert_eq!(
        world.perception_state.target(),
        Some(perception::TargetId::player())
    );
}

#[then(regex = r"^the observer has not acquired a target$")]
async fn then_observer_not_acquired(world: &mut BevyoutWorld) {
    assert!(!world.perception_state.is_aware());
}

#[then(regex = r"^the observer has lost its target$")]
async fn then_observer_lost_target(world: &mut BevyoutWorld) {
    assert!(!world.perception_state.is_aware());
    assert_eq!(
        world.perception_last_event,
        Some(perception::AwarenessEvent::Lost(
            perception::TargetId::player()
        ))
    );
}

// ---------------------------------------------------------------------
// nav_door_access.feature (issue #185) -- appended section, do not
// interleave.
// ---------------------------------------------------------------------

#[given(regex = r"^a door with lock level (none|\d+), key (none|\d+), and (trapped|untrapped)$")]
async fn given_door_access_door(
    world: &mut BevyoutWorld,
    lock_level: String,
    key: String,
    trap: String,
) {
    world.nav_door_access_observation = openmw_doors::DoorAccessObservation {
        lock_level: (lock_level != "none").then(|| lock_level.parse::<i8>().unwrap()),
        key_form_id: (key != "none").then(|| key.parse::<u32>().unwrap()),
        trapped: trap == "trapped",
        holder_has_key: false,
    };
}

#[given(regex = r"^the actor (holds|does not hold) the door's key$")]
async fn given_door_access_key_possession(world: &mut BevyoutWorld, possession: String) {
    world.nav_door_access_observation.holder_has_key = possession == "holds";
}

#[then(regex = r"^the door is (openable|not openable)$")]
async fn then_door_access_result(world: &mut BevyoutWorld, expected: String) {
    world.nav_door_access_result = Some(openmw_doors::door_openable(
        world.nav_door_access_observation,
    ));
    assert_eq!(world.nav_door_access_result, Some(expected == "openable"));
}

// ---------------------------------------------------------------------
// package_families.feature (issues #196 Travel/Patrol, #197 Idle/Eat/Sleep)
// -- appended step section. Drives the pure `ai_families::FamilyDriver`
// dispatch directly; observable outcomes (requests + lifecycle signals),
// never internal bookkeeping.
// ---------------------------------------------------------------------

/// Parses a `(x, y, z)` / `(x,y,z)` triple from a scenario token.
fn pf_parse_point(text: &str) -> [f32; 3] {
    let cleaned: String = text
        .chars()
        .map(|c| if c == '(' || c == ')' { ' ' } else { c })
        .collect();
    let coords: Vec<f32> = cleaned
        .split(',')
        .map(|part| part.trim().parse::<f32>().expect("coordinate"))
        .collect();
    assert_eq!(coords.len(), 3, "expected three coordinates in {text:?}");
    [coords[0], coords[1], coords[2]]
}

fn pf_driver(world: &mut BevyoutWorld) -> &mut ai_families::FamilyDriver {
    world.pf_driver.as_mut().expect("configure a family first")
}

fn pf_tick(world: &mut BevyoutWorld, position: [f32; 3], nav_reached: bool) {
    let observation = ai_families::FamilyObservation::new(position, nav_reached, false);
    let step = pf_driver(world).tick(&observation, 0.1);
    world.pf_step = Some(step);
}

fn pf_step(world: &BevyoutWorld) -> ai_families::FamilyStep {
    world.pf_step.expect("a family tick has run")
}

#[given(regex = r"^a travel family targeting \(([^)]*)\) with tolerance ([0-9.]+)$")]
async fn given_travel_family(world: &mut BevyoutWorld, point: String, tolerance: String) {
    let position = pf_parse_point(&point);
    let tolerance: f32 = tolerance.parse().expect("tolerance");
    world.pf_markers = vec![position];
    world.pf_driver = Some(ai_families::FamilyDriver::new(
        ai_families::PackageFamily::Travel,
        vec![ai_families::Waypoint::at(position)],
        tolerance,
    ));
}

#[given(regex = r"^an idle family at \(([^)]*)\) with tolerance ([0-9.]+)$")]
async fn given_idle_family(world: &mut BevyoutWorld, point: String, tolerance: String) {
    let position = pf_parse_point(&point);
    let tolerance: f32 = tolerance.parse().expect("tolerance");
    world.pf_markers = vec![position];
    world.pf_driver = Some(ai_families::FamilyDriver::new(
        ai_families::PackageFamily::Idle,
        vec![ai_families::Waypoint::at(position)],
        tolerance,
    ));
}

#[given(regex = r"^a patrol family over markers \(([^)]*)\) then \(([^)]*)\) then \(([^)]*)\)$")]
async fn given_patrol_family(world: &mut BevyoutWorld, a: String, b: String, c: String) {
    let markers = [pf_parse_point(&a), pf_parse_point(&b), pf_parse_point(&c)];
    world.pf_markers = markers.to_vec();
    let waypoints = markers
        .iter()
        .map(|position| ai_families::Waypoint::at(*position))
        .collect();
    world.pf_driver = Some(ai_families::FamilyDriver::new(
        ai_families::PackageFamily::Patrol,
        waypoints,
        0.5,
    ));
}

#[given(regex = r"^an? (eat|sleep) family at interaction point ([0-9]+) located at \(([^)]*)\)$")]
async fn given_occupy_family(
    world: &mut BevyoutWorld,
    kind: String,
    point_id: String,
    position: String,
) {
    let position = pf_parse_point(&position);
    let interaction_point: u32 = point_id.parse().expect("interaction point id");
    let family = match kind.as_str() {
        "eat" => ai_families::PackageFamily::Eat,
        "sleep" => ai_families::PackageFamily::Sleep,
        other => panic!("unexpected occupy family {other:?}"),
    };
    world.pf_markers = vec![position];
    world.pf_driver = Some(ai_families::FamilyDriver::new(
        family,
        vec![ai_families::Waypoint {
            position,
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(interaction_point),
        }],
        0.5,
    ));
}

#[when(regex = r"^the actor is at \(([^)]*)\) still en route$")]
async fn when_actor_en_route(world: &mut BevyoutWorld, point: String) {
    let position = pf_parse_point(&point);
    pf_tick(world, position, false);
}

#[when(regex = r"^the actor arrives at \(([^)]*)\)$")]
async fn when_actor_arrives(world: &mut BevyoutWorld, point: String) {
    let position = pf_parse_point(&point);
    pf_tick(world, position, true);
}

#[when(regex = r"^the actor arrives at patrol marker ([0-9]+)$")]
async fn when_actor_arrives_marker(world: &mut BevyoutWorld, index: String) {
    let index: usize = index.parse().expect("marker index");
    let position = world.pf_markers[index];
    pf_tick(world, position, true);
}

#[when(regex = r"^the actor arrives at the interaction point$")]
async fn when_actor_arrives_interaction(world: &mut BevyoutWorld) {
    let position = world.pf_markers[0];
    pf_tick(world, position, true);
}

#[when(regex = r"^the sleep package is preempted$")]
async fn when_sleep_preempted(world: &mut BevyoutWorld) {
    // Preemption releases the family's occupancy claim (what the lifecycle's
    // pause + the `runpackage stop` path both call).
    let released = pf_driver(world).release();
    assert!(
        released.is_some(),
        "preempt should release an occupied point"
    );
}

#[then(regex = r"^the family requests a route to \(([^)]*)\)$")]
async fn then_requests_route(world: &mut BevyoutWorld, point: String) {
    let expected = pf_parse_point(&point);
    assert_eq!(
        pf_step(world).request,
        Some(ai_families::FamilyRequest::Route(expected))
    );
}

#[then(regex = r"^the family stops routing and completes the package$")]
async fn then_stops_and_completes(world: &mut BevyoutWorld) {
    let step = pf_step(world);
    assert_eq!(step.request, Some(ai_families::FamilyRequest::Stop));
    assert_eq!(step.signal, ai_families::LifecycleSignal::Complete);
}

#[then(regex = r"^the family advances to patrol marker ([0-9]+)$")]
async fn then_advances_to_marker(world: &mut BevyoutWorld, index: String) {
    let index: usize = index.parse().expect("marker index");
    assert_eq!(
        pf_step(world).signal,
        ai_families::LifecycleSignal::AdvanceStep
    );
    assert_eq!(pf_driver(world).marker_index(), index);
}

#[then(regex = r"^the family requests the idle animation$")]
async fn then_requests_idle(world: &mut BevyoutWorld) {
    assert_eq!(
        pf_step(world).request,
        Some(ai_families::FamilyRequest::Play(
            ai_families::FamilyAnimation::Idle
        ))
    );
}

#[then(regex = r"^the family occupies interaction point ([0-9]+)$")]
async fn then_occupies_point(world: &mut BevyoutWorld, point_id: String) {
    let point_id: u32 = point_id.parse().expect("interaction point id");
    assert_eq!(pf_driver(world).occupied_point(), Some(point_id));
}

#[then(regex = r"^the family requests the eat animation$")]
async fn then_requests_eat(world: &mut BevyoutWorld) {
    assert_eq!(
        pf_step(world).request,
        Some(ai_families::FamilyRequest::Play(
            ai_families::FamilyAnimation::Eat
        ))
    );
}

#[then(regex = r"^the family releases interaction point ([0-9]+)$")]
async fn then_releases_point(world: &mut BevyoutWorld, _point_id: String) {
    // `when_sleep_preempted` performed and asserted the release; confirm the
    // driver no longer holds the claim.
    assert_eq!(pf_driver(world).occupied_point(), None);
}

// ---------------------------------------------------------------------
// ai_follow_sandbox.feature (issue #198 Follow + Sandbox) -- appended step
// section. Drives the same pure `ai_families::FamilyDriver`, feeding the
// dynamic follow leader / blocking door / roam bounds through observations.
// Observable outcomes only (requests, signals, named door), never internal
// bookkeeping.
// ---------------------------------------------------------------------

#[given(regex = r"^a follow family with band ([0-9.]+) to ([0-9.]+) and tolerance ([0-9.]+)$")]
async fn given_follow_family(
    world: &mut BevyoutWorld,
    band_min: String,
    band_max: String,
    tolerance: String,
) {
    let band_min: f32 = band_min.parse().expect("band min");
    let band_max: f32 = band_max.parse().expect("band max");
    let tolerance: f32 = tolerance.parse().expect("tolerance");
    world.pf_driver = Some(ai_families::FamilyDriver::follow(
        band_min, band_max, tolerance,
    ));
    world.fs_leader = None;
    world.fs_blocking_door = None;
}

#[when(regex = r"^the leader is at \(([^)]*)\)$")]
async fn when_leader_at(world: &mut BevyoutWorld, point: String) {
    world.fs_leader = Some(pf_parse_point(&point));
    world.fs_blocking_door = None;
}

#[when(regex = r"^the leader is lost$")]
async fn when_leader_lost(world: &mut BevyoutWorld) {
    world.fs_leader = None;
}

#[when(regex = r"^the route is blocked by locked door ([0-9a-fA-F]+)$")]
async fn when_route_blocked(world: &mut BevyoutWorld, door: String) {
    world.fs_blocking_door = Some(u32::from_str_radix(&door, 16).expect("door form id"));
}

#[when(regex = r"^the follower at \(([^)]*)\) ticks$")]
async fn when_follower_ticks(world: &mut BevyoutWorld, actor: String) {
    let actor = pf_parse_point(&actor);
    // A blocking door surfaces alongside the nav route failure that produced it.
    let route_failed = world.fs_blocking_door.is_some();
    let observation = ai_families::FamilyObservation {
        target_position: world.fs_leader,
        blocking_door: world.fs_blocking_door,
        ..ai_families::FamilyObservation::new(actor, false, route_failed)
    };
    let step = pf_driver(world).tick(&observation, 0.1);
    world.pf_step = Some(step);
}

#[then(regex = r"^the follow family keeps closing without stopping$")]
async fn then_follow_keeps_closing(world: &mut BevyoutWorld) {
    let step = pf_step(world);
    assert_ne!(step.request, Some(ai_families::FamilyRequest::Stop));
    assert_eq!(step.signal, ai_families::LifecycleSignal::Continue);
    assert_eq!(pf_driver(world).step_label(), "routing");
}

#[then(regex = r"^the follow family stops routing$")]
async fn then_follow_stops(world: &mut BevyoutWorld) {
    assert_eq!(
        pf_step(world).request,
        Some(ai_families::FamilyRequest::Stop)
    );
    assert_eq!(pf_driver(world).step_label(), "idling");
}

#[then(regex = r"^the follow family names blocking door ([0-9a-fA-F]+) and abandons$")]
async fn then_follow_names_door(world: &mut BevyoutWorld, door: String) {
    let door = u32::from_str_radix(&door, 16).expect("door form id");
    let step = pf_step(world);
    assert_eq!(step.request, Some(ai_families::FamilyRequest::Stop));
    assert_eq!(step.signal, ai_families::LifecycleSignal::Fail);
    assert_eq!(pf_driver(world).blocked_door(), Some(door));
    assert_eq!(pf_driver(world).step_label(), "blocked");
}

#[then(regex = r"^the follow family stops routing and keeps the package running$")]
async fn then_follow_target_loss(world: &mut BevyoutWorld) {
    let step = pf_step(world);
    assert_eq!(step.request, Some(ai_families::FamilyRequest::Stop));
    assert_eq!(step.signal, ai_families::LifecycleSignal::Continue);
}

#[given(regex = r"^a sandbox family roaming within ([0-9.]+) of \(([^)]*)\) seeded ([0-9]+)$")]
async fn given_sandbox_family(
    world: &mut BevyoutWorld,
    radius: String,
    center: String,
    seed: String,
) {
    let radius: f32 = radius.parse().expect("radius");
    let center = pf_parse_point(&center);
    let seed: u64 = seed.parse().expect("seed");
    world.fs_roam_center = center;
    world.fs_roam_radius = radius;
    world.pf_driver = Some(ai_families::FamilyDriver::wander(
        center, radius, 2.0, seed, 0.5,
    ));
}

#[when(regex = r"^the sandbox actor ticks at \(([^)]*)\)$")]
async fn when_sandbox_ticks(world: &mut BevyoutWorld, actor: String) {
    let actor = pf_parse_point(&actor);
    let observation = ai_families::FamilyObservation::new(actor, false, false);
    let step = pf_driver(world).tick(&observation, 0.1);
    world.pf_step = Some(step);
}

#[when(regex = r"^the sandbox actor arrives at its roam point$")]
async fn when_sandbox_arrives(world: &mut BevyoutWorld) {
    let point = pf_driver(world)
        .current_target()
        .expect("a roam point has been drawn");
    let observation = ai_families::FamilyObservation::new(point, true, false);
    let step = pf_driver(world).tick(&observation, 0.1);
    world.pf_step = Some(step);
}

#[then(regex = r"^the sandbox family routes within the roam radius$")]
async fn then_sandbox_routes_within_radius(world: &mut BevyoutWorld) {
    let center = world.fs_roam_center;
    let radius = world.fs_roam_radius;
    let Some(ai_families::FamilyRequest::Route(point)) = pf_step(world).request else {
        panic!("expected a roam route request");
    };
    let dx = point[0] - center[0];
    let dz = point[2] - center[2];
    assert!(
        (dx * dx + dz * dz).sqrt() <= radius + 1e-3,
        "roam point {point:?} escaped the radius"
    );
    assert_eq!(point[1], center[1], "roam stays on the ground plane");
}

// =======================================================================
// -- ai_package_points.feature (issue #213) --
// Merge-seam addition: appended at the end of the file per
// docs/plans/README.md's traceability convention. Reuses
// `given_resolvable_reference`/`given_package_location_input`/
// `when_location_resolved`/`then_location_resolves_to`/
// `then_location_resolves_via` from the `ai_package_resolution.feature`
// step section above.
// =======================================================================

#[given(regex = r"^the resolving actor's editor location is ([\d.-]+) ([\d.-]+) ([\d.-]+)$")]
async fn given_actor_editor_location(world: &mut BevyoutWorld, x: f32, y: f32, z: f32) {
    world.ai_res_context.actor_editor_location = Some([x, y, z]);
}

#[given(regex = r"^reference 0x([0-9a-fA-F]+) is linked to 0x([0-9a-fA-F]+)$")]
async fn given_reference_linked_to(world: &mut BevyoutWorld, from_hex: String, to_hex: String) {
    let from = parse_hex(&from_hex);
    let to = parse_hex(&to_hex);
    let reference = world
        .ai_res_context
        .references
        .get_mut(&from)
        .unwrap_or_else(|| panic!("reference {from:08x} must be declared first"));
    reference.linked_reference = Some(to);
}

#[when(regex = r"^the linked-reference chain is walked from 0x([0-9a-fA-F]+)$")]
async fn when_linked_reference_chain_walked(world: &mut BevyoutWorld, start_hex: String) {
    let start = parse_hex(&start_hex);
    world.pp_chain_result = ai_resolution::linked_reference_chain(&world.ai_res_context, start);
}

#[then(regex = r"^the chain has (\d+) markers?$")]
async fn then_chain_has_n_markers(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.pp_chain_result.len(), count);
}

#[then(regex = r"^chain marker (\d+) resolves to ([\d.-]+) ([\d.-]+) ([\d.-]+)$")]
async fn then_chain_marker_resolves_to(
    world: &mut BevyoutWorld,
    index: usize,
    x: f32,
    y: f32,
    z: f32,
) {
    let point = world
        .pp_chain_result
        .get(index - 1)
        .unwrap_or_else(|| panic!("chain has no marker #{index}"));
    assert_eq!(point.position, [x, y, z]);
}

// =======================================================================
// -- player_weapon.feature (M5 wave 1, issues #235-#238) --
// Pure action/damage policy used by the Bevy weapon adapter.
// =======================================================================

#[given(regex = r"^an idle weapon with damage ([\d.]+) and range ([\d.]+) metres$")]
async fn given_idle_weapon(world: &mut BevyoutWorld, damage: f32, range: f32) {
    world.weapon_state = Some(weapon::WeaponState::new(weapon::WeaponDefinition::new(
        damage, range,
    )));
    world.weapon_last_fire = None;
}

#[given("weapon ammunition accounting is disabled")]
async fn given_ammunition_accounting_disabled(world: &mut BevyoutWorld) {
    world.weapon_ammo_consumed = 0;
}

#[when("the weapon fire action is requested")]
async fn when_weapon_fire_requested(world: &mut BevyoutWorld) {
    world.weapon_last_fire = Some(
        world
            .weapon_state
            .as_mut()
            .expect("weapon must be configured")
            .request_fire(),
    );
}

#[when("the weapon reload action is requested")]
async fn when_weapon_reload_requested(world: &mut BevyoutWorld) {
    world
        .weapon_state
        .as_mut()
        .expect("weapon must be configured")
        .request_reload();
}

#[when(regex = r"^the weapon advances by ([\d.]+) seconds$")]
async fn when_weapon_advances(world: &mut BevyoutWorld, seconds: f32) {
    world
        .weapon_state
        .as_mut()
        .expect("weapon must be configured")
        .advance(seconds);
}

#[then("the weapon action is firing")]
async fn then_weapon_action_firing(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .weapon_state
            .as_ref()
            .expect("weapon must be configured")
            .action(),
        weapon::WeaponAction::Firing
    );
}

#[then("the weapon action is reloading")]
async fn then_weapon_action_reloading(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .weapon_state
            .as_ref()
            .expect("weapon must be configured")
            .action(),
        weapon::WeaponAction::Reloading
    );
}

#[then(regex = r"^(\d+) shots? (?:has|have) been accepted$")]
async fn then_shots_accepted(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(
        world
            .weapon_state
            .as_ref()
            .expect("weapon must be configured")
            .shots_fired(),
        expected
    );
}

#[then("the fire request is blocked by reload")]
async fn then_fire_blocked_by_reload(world: &mut BevyoutWorld) {
    assert_eq!(
        world.weapon_last_fire,
        Some(weapon::FireDecision::BlockedReloading)
    );
}

#[then("no ammunition has been consumed")]
async fn then_no_ammunition_consumed(world: &mut BevyoutWorld) {
    assert_eq!(world.weapon_ammo_consumed, 0);
}

#[given(regex = r"^an alive actor with base health ([\d.]+)$")]
async fn given_alive_actor_with_health(world: &mut BevyoutWorld, health: f32) {
    let mut definition = actor_state::ActorDefinition {
        base_form_id: 0x10,
        reference_form_id: 0x20,
        ..Default::default()
    };
    definition
        .base_values
        .insert(actor_state::ActorValue::Health, health);
    world.weapon_actor_definition = definition;
    world.weapon_actor_instance =
        actor_state::ActorInstanceState::new(0x20, actor_state::ActorLifeState::Alive);
    world.weapon_damage_outcome = None;
}

#[when(regex = r"^the actor receives ([\d.]+) weapon damage$")]
async fn when_actor_receives_weapon_damage(world: &mut BevyoutWorld, damage: f32) {
    world.weapon_damage_outcome = Some(
        weapon::apply_actor_damage(
            &world.weapon_actor_definition,
            &mut world.weapon_actor_instance,
            damage,
        )
        .expect("synthetic weapon damage must be valid"),
    );
}

#[then(regex = r"^the weapon-damaged actor health is ([\d.]+)$")]
async fn then_effective_actor_health(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .weapon_actor_definition
        .resolve_value(
            &world.weapon_actor_instance,
            actor_state::ActorValue::Health,
        )
        .effective;
    assert!((actual - expected).abs() < 1e-4, "{actual} != {expected}");
}

#[then("the actor remains alive")]
async fn then_actor_remains_alive(world: &mut BevyoutWorld) {
    assert_eq!(
        world.weapon_actor_instance.life_state,
        actor_state::ActorLifeState::Alive
    );
}

#[then("the actor is dead")]
async fn then_actor_is_dead(world: &mut BevyoutWorld) {
    assert_eq!(
        world.weapon_actor_instance.life_state,
        actor_state::ActorLifeState::Dead
    );
}

// ---------------------------------------------------------------------
// reflection_probes.feature
// ---------------------------------------------------------------------

#[given(regex = r#"^reflection-probe region areas "([^"]*)"$"#)]
async fn given_reflection_probe_region_areas(world: &mut BevyoutWorld, areas: String) {
    world.reflection_probe_region_areas = areas
        .split(',')
        .map(|area| area.trim().parse().expect("region area must be numeric"))
        .collect();
}

#[when(regex = r"^probes are allocated with spacing ([\d.]+) and cap (\d+)$")]
async fn when_reflection_probes_are_allocated(world: &mut BevyoutWorld, spacing: f32, cap: usize) {
    world.reflection_probe_counts = reflection_probe_distribution::allocate_probe_counts(
        &world.reflection_probe_region_areas,
        spacing,
        cap,
    );
}

#[then(regex = r#"^the reflection-probe counts are "([^"]*)"$"#)]
async fn then_reflection_probe_counts_are(world: &mut BevyoutWorld, counts: String) {
    let expected = counts
        .split(',')
        .map(|count| count.trim().parse().expect("probe count must be numeric"))
        .collect::<Vec<usize>>();
    assert_eq!(world.reflection_probe_counts, expected);
}

// ---------------------------------------------------------------------
// day_night_lighting.feature
// ---------------------------------------------------------------------

#[given("an ordinary interior cell")]
async fn given_ordinary_interior(world: &mut BevyoutWorld) {
    world.day_night_interior = true;
    world.day_night_behave_like_exterior = false;
}

#[given("an interior cell that behaves like exterior")]
async fn given_exterior_like_interior(world: &mut BevyoutWorld) {
    world.day_night_interior = true;
    world.day_night_behave_like_exterior = true;
}

#[when(regex = r"^day-night preview is (enabled|disabled)$")]
async fn when_day_night_preview(world: &mut BevyoutWorld, state: String) {
    world.day_night_preview = state == "enabled";
    world.day_night_dynamic = time_of_day::uses_dynamic_lighting(
        world.day_night_interior,
        world.day_night_behave_like_exterior,
        world.day_night_preview,
    );
}

#[then(regex = r"^dynamic day-night lighting is (enabled|disabled)$")]
async fn then_day_night_dynamic(world: &mut BevyoutWorld, state: String) {
    assert_eq!(world.day_night_dynamic, state == "enabled");
}

#[given(regex = r"^a Fallout clock at hour ([\d.]+) with timescale ([\d.]+)$")]
async fn given_fallout_clock(world: &mut BevyoutWorld, hour: f32, timescale: f32) {
    world.game_hour = hour;
    world.game_timescale = timescale;
}

#[when(regex = r"^([\d.]+) real seconds elapse$")]
async fn when_real_seconds_elapse(world: &mut BevyoutWorld, seconds: f32) {
    world.game_hour =
        time_of_day::advance_game_hour(world.game_hour, world.game_timescale, seconds);
}

#[then(regex = r"^the Fallout clock reads hour ([\d.]+)$")]
async fn then_fallout_clock_reads(world: &mut BevyoutWorld, expected: f32) {
    assert!((world.game_hour - expected).abs() < 1e-4);
}

#[given(
    regex = r"^scalar weather colors night ([\d.]+) sunrise ([\d.]+) day ([\d.]+) sunset ([\d.]+)$"
)]
async fn given_scalar_weather_colors(
    world: &mut BevyoutWorld,
    night: f32,
    sunrise: f32,
    day: f32,
    sunset: f32,
) {
    world.weather_keyframes = time_of_day::ColorKeyframes {
        sunrise: [sunrise; 4],
        day: [day; 4],
        sunset: [sunset; 4],
        night: [night; 4],
    };
}

#[when(regex = r"^weather color is sampled at hour ([\d.]+)$")]
async fn when_weather_color_sampled(world: &mut BevyoutWorld, hour: f32) {
    world.sampled_weather_color = time_of_day::interpolate_keyframes(
        world.weather_keyframes,
        time_of_day::DayNightTimings::default(),
        hour,
    );
}

#[then(regex = r"^the sampled weather color is ([\d.]+)$")]
async fn then_sampled_weather_color(world: &mut BevyoutWorld, expected: f32) {
    assert!((world.sampled_weather_color[0] - expected).abs() < 1e-4);
}

#[given(regex = r#"^preview weather candidates "([^"]*)"$"#)]
async fn given_preview_weather_candidates(world: &mut BevyoutWorld, candidates: String) {
    world.preview_weather_candidates = candidates
        .split(',')
        .map(|candidate| {
            let (form_id, editor_id) = candidate
                .split_once(':')
                .expect("weather candidate must be FORMID:EditorID");
            (
                u32::from_str_radix(form_id, 16).expect("weather FormID must be hexadecimal"),
                Some(editor_id.into()),
            )
        })
        .collect();
}

#[when("preview fallback weather is selected")]
async fn when_preview_fallback_weather_selected(world: &mut BevyoutWorld) {
    world.selected_preview_weather =
        time_of_day::select_preview_weather(&world.preview_weather_candidates);
}

#[then(regex = r"^preview weather ([0-9a-fA-F]{8}) is selected$")]
async fn then_preview_weather_selected(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.selected_preview_weather,
        Some(u32::from_str_radix(&expected, 16).unwrap())
    );
}

// ---------------------------------------------------------------------
// script_inventory.feature — issue #252
// ---------------------------------------------------------------------

#[given(regex = r#"^record ([0-9a-fA-F]{8}) from ([^ ]+) carries payload "([^"]*)"$"#)]
async fn given_script_record_version(
    world: &mut BevyoutWorld,
    form_id: String,
    source: String,
    payload: String,
) {
    world.script_record_versions.push((
        u32::from_str_radix(&form_id, 16).unwrap(),
        source,
        Some(payload),
    ));
}

#[given(regex = r"^record ([0-9a-fA-F]{8}) is deleted by ([^ ]+)$")]
async fn given_script_record_deletion(world: &mut BevyoutWorld, form_id: String, source: String) {
    world
        .script_record_versions
        .push((u32::from_str_radix(&form_id, 16).unwrap(), source, None));
}

#[when("the record versions are collected in load order")]
async fn when_script_records_are_collected(world: &mut BevyoutWorld) {
    let mut winners = record_winners::WinningRecords::default();
    for (form_id, source, payload) in std::mem::take(&mut world.script_record_versions) {
        match payload {
            Some(payload) => winners.upsert(form_id, source, payload),
            None => winners.delete(form_id),
        }
    }
    world.script_record_winners = Some(winners);
}

#[then(regex = r#"^record ([0-9a-fA-F]{8}) has winning payload "([^"]*)"$"#)]
async fn then_script_record_has_payload(
    world: &mut BevyoutWorld,
    form_id: String,
    expected: String,
) {
    let form_id = u32::from_str_radix(&form_id, 16).unwrap();
    let winner = world
        .script_record_winners
        .as_ref()
        .and_then(|winners| winners.get(form_id))
        .expect("record should have a winner");
    assert_eq!(winner.value, expected);
}

#[then(regex = r#"^record ([0-9a-fA-F]{8}) has provenance "([^"]*)"$"#)]
async fn then_script_record_has_provenance(
    world: &mut BevyoutWorld,
    form_id: String,
    expected: String,
) {
    let form_id = u32::from_str_radix(&form_id, 16).unwrap();
    let winner = world
        .script_record_winners
        .as_ref()
        .and_then(|winners| winners.get(form_id))
        .expect("record should have a winner");
    assert_eq!(winner.provenance.join(","), expected);
}

#[then(regex = r"^record ([0-9a-fA-F]{8}) is absent from the winning records$")]
async fn then_script_record_is_absent(world: &mut BevyoutWorld, form_id: String) {
    let form_id = u32::from_str_radix(&form_id, 16).unwrap();
    assert!(
        world
            .script_record_winners
            .as_ref()
            .is_some_and(|winners| winners.get(form_id).is_none())
    );
}

#[given("a synthetic object script with one local and one reference")]
async fn given_structural_object_script(world: &mut BevyoutWorld) {
    let mut header = Vec::new();
    header.extend(0_u32.to_le_bytes());
    header.extend(1_u32.to_le_bytes());
    header.extend(3_u32.to_le_bytes());
    header.extend(1_u32.to_le_bytes());
    header.extend(0_u16.to_le_bytes());
    header.extend(1_u16.to_le_bytes());
    let mut local = vec![0; 24];
    local[0..4].copy_from_slice(&3_u32.to_le_bytes());
    local[16] = 1;
    world.script_structural_inputs = vec![
        ("EDID".into(), b"FixtureScript\0".to_vec(), 0),
        ("SCHR".into(), header, 16),
        ("SCDA".into(), vec![1, 2, 3], 42),
        ("SCTX".into(), b"begin GameMode\0".to_vec(), 51),
        ("SLSD".into(), local, 73),
        ("SCVR".into(), b"Counter\0".to_vec(), 103),
        ("SCRO".into(), 0x10_u32.to_le_bytes().to_vec(), 117),
    ];
}

#[given(
    regex = r"^a synthetic script with a malformed header and unknown subrecord ([A-Z0-9_]{4})$"
)]
async fn given_malformed_structural_script(world: &mut BevyoutWorld, unknown: String) {
    world.script_structural_inputs =
        vec![("SCHR".into(), vec![0; 3], 0), (unknown, vec![9, 8, 7], 9)];
}

#[when("the structural script record is decoded")]
async fn when_structural_script_is_decoded(world: &mut BevyoutWorld) {
    let inputs = world
        .script_structural_inputs
        .iter()
        .map(|(signature, data, offset)| record::ScriptSubrecordInput {
            signature,
            data,
            offset: *offset,
        })
        .collect::<Vec<_>>();
    world.script_decoded_record = Some(record::decode_script_record(
        bevyout_core::form_id::FormId(0x400),
        "fixture.esp",
        &inputs,
        bevyout_core::form_id::FormId,
    ));
}

#[then("the script kind is object")]
async fn then_structural_script_is_object(world: &mut BevyoutWorld) {
    assert_eq!(
        world.script_decoded_record.as_ref().unwrap().record.kind,
        Some(record::ScriptKind::Object)
    );
}

#[then("the compiled script bytes are preserved")]
async fn then_compiled_script_bytes_are_preserved(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .script_decoded_record
            .as_ref()
            .unwrap()
            .record
            .compiled_data
            .as_deref(),
        Some([1, 2, 3].as_slice())
    );
}

#[then(regex = r#"^local slot (\d+) is named "([^"]*)"$"#)]
async fn then_script_local_is_named(world: &mut BevyoutWorld, slot: u32, name: String) {
    let local = &world.script_decoded_record.as_ref().unwrap().record.locals[0];
    assert_eq!(local.slot, slot);
    assert_eq!(local.name.as_deref(), Some(name.as_str()));
}

#[then(regex = r"^script reference ([0-9a-fA-F]{8}) is resolved$")]
async fn then_script_reference_is_resolved(world: &mut BevyoutWorld, form_id: String) {
    let expected = u32::from_str_radix(&form_id, 16).unwrap();
    assert!(
        world
            .script_decoded_record
            .as_ref()
            .unwrap()
            .record
            .references
            .contains(&record::ScriptReference::Form(
                bevyout_core::form_id::FormId(expected)
            ))
    );
}

#[then("the script has a SCHR size diagnostic")]
async fn then_script_has_header_size_diagnostic(world: &mut BevyoutWorld) {
    assert!(
        world
            .script_decoded_record
            .as_ref()
            .unwrap()
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.subrecord.as_deref() == Some("SCHR")
                && diagnostic.message.contains("20 bytes"))
    );
}

#[then(regex = r"^unknown script subrecord ([A-Z0-9_]{4}) is preserved$")]
async fn then_unknown_script_subrecord_is_preserved(world: &mut BevyoutWorld, signature: String) {
    assert!(
        world
            .script_decoded_record
            .as_ref()
            .unwrap()
            .record
            .unknown_subrecords
            .iter()
            .any(|subrecord| subrecord.signature == signature)
    );
}

#[given(regex = r"^a synthetic owner record with SCRI ([0-9a-fA-F]{8})$")]
async fn given_owner_with_script_attachment(world: &mut BevyoutWorld, form_id: String) {
    world.script_owner_signature = "ACTI".into();
    world.script_structural_inputs = vec![(
        "SCRI".into(),
        u32::from_str_radix(&form_id, 16)
            .unwrap()
            .to_le_bytes()
            .to_vec(),
        12,
    )];
}

#[when(regex = r"^the owner's script attachments are decoded with plugin index (\d+)$")]
async fn when_owner_attachments_are_decoded(world: &mut BevyoutWorld, plugin_index: u8) {
    world.script_owner_plugin_index = plugin_index;
    let inputs = world
        .script_structural_inputs
        .iter()
        .map(|(signature, data, offset)| record::ScriptSubrecordInput {
            signature,
            data,
            offset: *offset,
        })
        .collect::<Vec<_>>();
    let resolver = bevyout_core::form_id::FormIdResolver::new(plugin_index, vec![0]);
    world.script_extracted_owner = Some(attachments::extract_owner_scripts(
        bevyout_core::form_id::FormId(0x500),
        &world.script_owner_signature,
        "fixture.esp",
        &inputs,
        |raw| resolver.resolve(bevyout_core::form_id::FormId(raw)),
    ));
}

#[then(regex = r"^attachment slot (\d+) targets script ([0-9a-fA-F]{8})$")]
async fn then_attachment_targets_script(world: &mut BevyoutWorld, slot: u32, form_id: String) {
    let attachment = &world.script_extracted_owner.as_ref().unwrap().attachments[0];
    assert_eq!(
        attachment.slot,
        attachments::ScriptAttachmentSlot::Direct(slot)
    );
    assert_eq!(
        attachment.script,
        record::ScriptAssetId::Record(bevyout_core::form_id::FormId(
            u32::from_str_radix(&form_id, 16).unwrap()
        ))
    );
}

#[given("a synthetic package with OnBegin and OnEnd embedded scripts")]
async fn given_package_with_embedded_scripts(world: &mut BevyoutWorld) {
    let mut header = Vec::new();
    header.extend(0_u32.to_le_bytes());
    header.extend(0_u32.to_le_bytes());
    header.extend(1_u32.to_le_bytes());
    header.extend(0_u32.to_le_bytes());
    header.extend(0_u16.to_le_bytes());
    header.extend(0_u16.to_le_bytes());
    world.script_owner_signature = "PACK".into();
    world.script_structural_inputs = vec![
        ("POBA".into(), Vec::new(), 0),
        ("SCHR".into(), header.clone(), 6),
        ("SCDA".into(), vec![0xaa], 32),
        ("TNAM".into(), vec![0; 4], 39),
        ("POEA".into(), Vec::new(), 49),
        ("SCHR".into(), header, 55),
        ("SCDA".into(), vec![0xbb], 81),
    ];
}

#[when("the package's embedded scripts are decoded")]
async fn when_package_embedded_scripts_are_decoded(world: &mut BevyoutWorld) {
    let inputs = world
        .script_structural_inputs
        .iter()
        .map(|(signature, data, offset)| record::ScriptSubrecordInput {
            signature,
            data,
            offset: *offset,
        })
        .collect::<Vec<_>>();
    world.script_extracted_owner = Some(attachments::extract_owner_scripts(
        bevyout_core::form_id::FormId(0x600),
        "PACK",
        "fixture.esp",
        &inputs,
        bevyout_core::form_id::FormId,
    ));
}

#[then("the package has embedded script slots OnBegin, OnEnd")]
async fn then_package_has_named_embedded_slots(world: &mut BevyoutWorld) {
    let ids = world
        .script_extracted_owner
        .as_ref()
        .unwrap()
        .embedded
        .iter()
        .map(|script| script.record.id)
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        [
            record::ScriptAssetId::Embedded {
                owner: bevyout_core::form_id::FormId(0x600),
                slot: record::EmbeddedScriptSlot::Package(record::PackageScriptSlot::Begin),
            },
            record::ScriptAssetId::Embedded {
                owner: bevyout_core::form_id::FormId(0x600),
                slot: record::EmbeddedScriptSlot::Package(record::PackageScriptSlot::End),
            },
        ]
    );
}

#[then("each embedded script preserves its compiled bytes")]
async fn then_embedded_scripts_preserve_compiled_bytes(world: &mut BevyoutWorld) {
    let embedded = &world.script_extracted_owner.as_ref().unwrap().embedded;
    assert_eq!(
        embedded[0].record.compiled_data.as_deref(),
        Some(&[0xaa][..])
    );
    assert_eq!(
        embedded[1].record.compiled_data.as_deref(),
        Some(&[0xbb][..])
    );
}

#[given("a synthetic script inventory report")]
async fn given_synthetic_script_inventory_report(world: &mut BevyoutWorld) {
    world.script_inventory_report = Some(report_schema::CompatibilityReport {
        schema_version: report_schema::CURRENT_REPORT_SCHEMA_VERSION,
        source_plugin: "Fixture.esm".into(),
        source_fingerprint: "source".into(),
        entries: Vec::new(),
        script_inventory: report_schema::ScriptInventoryReport {
            content_fingerprint: "content".into(),
            totals: report_schema::ScriptInventoryTotals {
                top_level: 1,
                embedded: 1,
                attachments: 1,
                compiled_bytes: 4,
                variables: 2,
                references: 1,
                diagnostics: 0,
            },
            by_kind: std::collections::BTreeMap::from([("object".into(), 2)]),
            by_representation: std::collections::BTreeMap::from([("scda_sctx".into(), 2)]),
            attachment_owner_signatures: std::collections::BTreeMap::from([("PACK".into(), 1)]),
            scripts: Vec::new(),
            attachments: Vec::new(),
            diagnostics: Vec::new(),
        },
    });
}

#[when("the script inventory report is rendered twice")]
async fn when_script_inventory_report_is_rendered_twice(world: &mut BevyoutWorld) {
    let report = world.script_inventory_report.as_ref().unwrap();
    world.script_inventory_renders = vec![report.to_json(), report.to_json()];
}

#[then("both script inventory JSON renders are byte-identical")]
async fn then_script_inventory_renders_are_identical(world: &mut BevyoutWorld) {
    assert_eq!(
        world.script_inventory_renders[0],
        world.script_inventory_renders[1]
    );
}

#[then("the script inventory summary reports 1 top-level and 1 embedded script")]
async fn then_script_inventory_summary_has_totals(world: &mut BevyoutWorld) {
    assert!(world.script_inventory_report.as_ref().unwrap().summary().contains(
        "scripts: top-level=1 embedded=1 attachments=1 compiled-bytes=4 variables=2 references=1 diagnostics=0"
    ));
}
// =======================================================================
// -- autonomous_actors.feature (issues #218/#224) --
// Merge-seam addition: appended at the end of the file per
// docs/plans/README.md's traceability convention.
// =======================================================================

#[given(regex = r#"^an actor with life state "(alive|dead)"$"#)]
async fn given_actor_life_state(world: &mut BevyoutWorld, state: String) {
    world.autonomous_life_state_alive = state == "alive";
    world.autonomous_already_nav_bound = false;
    world.autonomous_already_has_controller = false;
    world.autonomous_eligible = None;
}

#[given("the actor is already nav-bound")]
async fn given_actor_already_nav_bound(world: &mut BevyoutWorld) {
    world.autonomous_already_nav_bound = true;
}

#[given("the actor already has a running package controller")]
async fn given_actor_already_has_controller(world: &mut BevyoutWorld) {
    world.autonomous_already_has_controller = true;
}

#[given(regex = r"^package type (\d+) dispatches to the Patrol family$")]
async fn given_package_type_dispatches_to_patrol(world: &mut BevyoutWorld, package_type: u8) {
    let _ = world; // no World state to record; this is a standalone assertion.
    assert_eq!(
        ai_families::PackageFamily::from_package_type(package_type),
        Some(ai_families::PackageFamily::Patrol),
        "package type {package_type} must dispatch to the Patrol family"
    );
}

#[when("the autonomous package driver evaluates the actor")]
async fn when_autonomous_driver_evaluates(world: &mut BevyoutWorld) {
    world.autonomous_eligible = Some(autonomous_gate::eligible_for_autonomous_start(
        world.autonomous_life_state_alive,
        world.autonomous_already_nav_bound,
        world.autonomous_already_has_controller,
    ));
}

#[then("the actor is selected for autonomous bind and start")]
async fn then_actor_is_selected(world: &mut BevyoutWorld) {
    assert_eq!(world.autonomous_eligible, Some(true));
}

#[then("the actor is not selected for autonomous bind and start")]
async fn then_actor_is_not_selected(world: &mut BevyoutWorld) {
    assert_eq!(world.autonomous_eligible, Some(false));
}

#[when(
    regex = r"^its achieved horizontal velocity alternates direction at full route speed for (\d+) ticks, smoothed before classification$"
)]
async fn when_locomotion_alternates_smoothed(world: &mut BevyoutWorld, ticks: u32) {
    const WARMUP_TICKS: u32 = 32;
    let mut window = locomotion::VelocityWindow::default();
    world.nav_locomotion_changes_after_warmup = 0;
    for tick in 0..ticks {
        let raw = if tick % 2 == 0 {
            [locomotion::ROUTE_SPEED_METRES_PER_SECOND, 0.0]
        } else {
            [-locomotion::ROUTE_SPEED_METRES_PER_SECOND, 0.0]
        };
        let net_velocity = window.push(raw);
        let previous = world.nav_locomotion_state;
        let achieved_horizontal_speed =
            if window.coherence() >= locomotion::MOTION_COHERENCE_THRESHOLD {
                (net_velocity[0] * net_velocity[0] + net_velocity[1] * net_velocity[1]).sqrt()
            } else {
                0.0
            };
        let next = step_locomotion(
            world,
            locomotion::LocomotionObservation {
                achieved_horizontal_speed,
                yaw_rate: 0.0,
            },
        );
        if tick >= WARMUP_TICKS && next != previous {
            world.nav_locomotion_changes_after_warmup += 1;
        }
    }
}

#[when(
    regex = r"^its achieved yaw rate alternates sign at full facing rate for (\d+) ticks, smoothed before classification$"
)]
async fn when_locomotion_yaw_alternates_smoothed(world: &mut BevyoutWorld, ticks: u32) {
    const WARMUP_TICKS: u32 = 32;
    let mut window = locomotion::YawRateWindow::default();
    world.nav_locomotion_changes_after_warmup = 0;
    for tick in 0..ticks {
        let raw = if tick % 2 == 0 {
            std::f32::consts::PI
        } else {
            -std::f32::consts::PI
        };
        let net_yaw_rate = window.push(raw);
        let previous = world.nav_locomotion_state;
        let yaw_rate = if window.coherence() >= locomotion::MOTION_COHERENCE_THRESHOLD {
            net_yaw_rate
        } else {
            0.0
        };
        let next = step_locomotion(
            world,
            locomotion::LocomotionObservation {
                achieved_horizontal_speed: 0.0,
                yaw_rate,
            },
        );
        if tick >= WARMUP_TICKS && next != previous {
            world.nav_locomotion_changes_after_warmup += 1;
        }
    }
}

#[then("its locomotion state is idle after the smoothing warms up")]
async fn then_locomotion_is_idle_after_warmup(world: &mut BevyoutWorld) {
    assert_eq!(
        world.nav_locomotion_state,
        locomotion::LocomotionState::Idle
    );
    assert_eq!(world.nav_locomotion_changes_after_warmup, 0);
}

// -- dialogue.feature review hardening --

#[given("missing authored voice with a recorded mapping manifest")]
async fn given_missing_authored_voice_with_mapping(world: &mut BevyoutWorld) {
    let key = bevyout_core::dialogue::DialogueLineKey::new("Start:0");
    world.dialogue_catalog = Some(dialogue::PreparedDialogueCatalog {
        revision: dialogue::PREPARED_DIALOGUE_CATALOG_REVISION.into(),
        source_paths: vec!["authored/guard.yarn".into()],
        authored_voice_manifest_paths: vec!["dialogue/voice/guard.ron".into()],
        line_keys: std::collections::BTreeSet::from([key.clone()]),
        voice_requirements: vec![dialogue::PreparedDialogueVoiceRequirement {
            line_key: key,
            speaker_form_id: None,
            source_path: "authored/guard.yarn".into(),
            origin: dialogue::DialogueVoiceRequirementOrigin::Authored,
        }],
        ..Default::default()
    });
}

#[when("dialogue voice repair guidance is rendered")]
async fn when_dialogue_voice_repair_guidance_is_rendered(world: &mut BevyoutWorld) {
    let catalog = world.dialogue_catalog.as_ref().unwrap();
    let coverage =
        dialogue::coverage::assess_voice_coverage(std::path::Path::new("."), catalog, None);
    world.dialogue_repair_guidance = Some(dialogue::coverage::voice_repair_guidance(
        "FixtureCell",
        catalog,
        &coverage,
    ));
}

#[then("the guidance contains an executable prepare command")]
async fn then_guidance_contains_prepare_command(world: &mut BevyoutWorld) {
    assert_eq!(
        world.dialogue_repair_guidance.as_deref(),
        Some(
            "next command: cargo run-dev -- prepare FixtureCell --dialogue-source dialogue/authored/guard.yarn --dialogue-voice-manifest dialogue/voice/guard.ron"
        )
    );
}

#[given("a prepared dialogue bundle with a stale voice index revision")]
async fn given_prepared_dialogue_bundle_with_stale_voice_index(world: &mut BevyoutWorld) {
    let catalog = dialogue::prepare_catalog(vec![synthetic_dialogue_source()]);
    let index = bevyout_core::dialogue::PreparedDialogueVoiceIndex {
        revision: "dialogue-voice-stale".into(),
        ..Default::default()
    };
    let bundle = bevyout_core::dialogue::PreparedDialogueBundleRef {
        revision: bevyout_core::dialogue::DIALOGUE_BUNDLE_REVISION.into(),
        source_paths: catalog.source_paths.clone(),
        voice_index_path: Some("dialogue/voice_index.ron".into()),
        content_fingerprint: dialogue::dialogue_bundle_fingerprint(&catalog, Some(&index)),
        ..Default::default()
    };
    world.dialogue_catalog = Some(catalog);
    world.dialogue_voice_index = Some(index);
    world.dialogue_bundle = Some(bundle);
}

#[when("dialogue bundle metadata is validated")]
async fn when_dialogue_bundle_metadata_is_validated(world: &mut BevyoutWorld) {
    world.dialogue_metadata_valid = Some(
        dialogue::validate_dialogue_bundle_metadata(
            world.dialogue_bundle.as_ref().unwrap(),
            world.dialogue_catalog.as_ref().unwrap(),
            world.dialogue_voice_index.as_ref(),
            None,
        )
        .is_ok(),
    );
}

#[then("the dialogue bundle metadata is rejected")]
async fn then_dialogue_bundle_metadata_is_rejected(world: &mut BevyoutWorld) {
    assert_eq!(world.dialogue_metadata_valid, Some(false));
}

// =======================================================================
// -- image_space.feature (#96) --
// Merge-seam addition: appended at the end of the file per
// docs/plans/README.md's traceability convention.
// =======================================================================

fn screen_fx_definition_with_values(
    form_id: u32,
    duration_ms: u32,
    values: screen_fx_policy::ScreenFxValues,
) -> screen_fx_policy::ScreenFxDefinition {
    screen_fx_policy::ScreenFxDefinition {
        modifier_form_id: form_id,
        duration_ms,
        static_values: values,
        curves: Vec::new(),
        color_keyframes: Vec::new(),
        fade_color_keyframes: Vec::new(),
    }
}

#[given(regex = r"^an IMAD blood curve from (\d+) ms at ([\d.]+) to (\d+) ms at ([\d.]+)$")]
async fn given_imad_blood_curve(
    world: &mut BevyoutWorld,
    first_time_ms: u32,
    first_value: f32,
    last_time_ms: u32,
    last_value: f32,
) {
    world.screen_fx_definition = Some(screen_fx_policy::ScreenFxDefinition {
        modifier_form_id: 0x96,
        duration_ms: last_time_ms,
        static_values: screen_fx_policy::ScreenFxValues::neutral(),
        curves: vec![screen_fx_policy::ScreenFxCurve {
            property: screen_fx_policy::ScreenFxProperty::Blood,
            operation: screen_fx_policy::ScreenFxCurveOperation::Additive,
            keyframes: vec![
                screen_fx_policy::ScreenFxKeyframe {
                    time_ms: first_time_ms,
                    value: first_value,
                },
                screen_fx_policy::ScreenFxKeyframe {
                    time_ms: last_time_ms,
                    value: last_value,
                },
            ],
        }],
        color_keyframes: Vec::new(),
        fade_color_keyframes: Vec::new(),
    });
}

#[given("a neutral screen effect base")]
async fn given_neutral_screen_effect_base(world: &mut BevyoutWorld) {
    world.screen_fx = screen_fx_policy::ScreenFxPolicy::default();
}

#[given(regex = r"^a screen modifier 0x([0-9A-Fa-f]+) with priority (-?\d+) and blood ([\d.]+)$")]
async fn given_screen_modifier(
    world: &mut BevyoutWorld,
    form_id: String,
    priority: i32,
    blood: f32,
) {
    let form_id = u32::from_str_radix(&form_id, 16).expect("screen modifier FormID");
    world.screen_fx.start(screen_fx_policy::ScreenFxStart::new(
        screen_fx_policy::ScreenFxSource::Gameplay,
        form_id,
        priority,
        0,
        screen_fx_definition_with_values(
            form_id,
            1_000,
            screen_fx_policy::ScreenFxValues {
                blood,
                ..screen_fx_policy::ScreenFxValues::neutral()
            },
        ),
    ));
}

#[given(regex = r"^a screen modifier 0x([0-9A-Fa-f]+) with duration (\d+) ms and blood ([\d.]+)$")]
async fn given_timed_screen_modifier(
    world: &mut BevyoutWorld,
    form_id: String,
    duration_ms: u32,
    blood: f32,
) {
    let form_id = u32::from_str_radix(&form_id, 16).expect("screen modifier FormID");
    world.screen_fx.start(screen_fx_policy::ScreenFxStart::new(
        screen_fx_policy::ScreenFxSource::Gameplay,
        form_id,
        0,
        0,
        screen_fx_definition_with_values(
            form_id,
            duration_ms,
            screen_fx_policy::ScreenFxValues {
                blood,
                ..screen_fx_policy::ScreenFxValues::neutral()
            },
        ),
    ));
}

#[given("screen blood and distortion are disabled")]
async fn given_screen_blood_and_distortion_disabled(world: &mut BevyoutWorld) {
    world
        .screen_fx
        .set_settings(screen_fx_policy::ScreenFxSettings {
            overall_intensity: 1.0,
            screen_blood: 0.0,
            flashes: 1.0,
            motion_and_distortion: 0.0,
        });
}

#[given(
    regex = r"^a weapon-hit screen modifier 0x([0-9A-Fa-f]+) with blood ([\d.]+) and double vision ([\d.]+)$"
)]
async fn given_weapon_hit_screen_modifier(
    world: &mut BevyoutWorld,
    form_id: String,
    blood: f32,
    double_vision: f32,
) {
    let form_id = u32::from_str_radix(&form_id, 16).expect("screen modifier FormID");
    world.screen_fx.start(screen_fx_policy::ScreenFxStart::new(
        screen_fx_policy::ScreenFxSource::WeaponHit,
        form_id,
        0,
        0,
        screen_fx_definition_with_values(
            form_id,
            1_000,
            screen_fx_policy::ScreenFxValues {
                blood,
                double_vision,
                ..screen_fx_policy::ScreenFxValues::neutral()
            },
        ),
    ));
}

#[when(regex = r"^the IMAD curve is sampled at (\d+) ms$")]
async fn when_imad_curve_sampled(world: &mut BevyoutWorld, elapsed_ms: u64) {
    let definition = world
        .screen_fx_definition
        .as_ref()
        .expect("IMAD definition");
    world.screen_fx_sample = Some(world.screen_fx.sample(definition, elapsed_ms));
}

#[when("the same screen modifier is started again with blood 0.75")]
async fn when_same_screen_modifier_restarted(world: &mut BevyoutWorld) {
    let form_id = 0x30;
    world.screen_fx.start(screen_fx_policy::ScreenFxStart::new(
        screen_fx_policy::ScreenFxSource::Gameplay,
        form_id,
        0,
        0,
        screen_fx_definition_with_values(
            form_id,
            1_000,
            screen_fx_policy::ScreenFxValues {
                blood: 0.75,
                ..screen_fx_policy::ScreenFxValues::neutral()
            },
        ),
    ));
}

#[when(regex = r"^screen time advances to (\d+) ms$")]
async fn when_screen_time_advances(world: &mut BevyoutWorld, now_ms: u64) {
    world.screen_fx.advance_to(now_ms);
}

#[when("the screen modifiers are composed")]
async fn when_screen_modifiers_composed(world: &mut BevyoutWorld) {
    world.screen_fx_sample = Some(world.screen_fx.snapshot());
}

#[when("screen effects are cleared for a cell transition")]
async fn when_screen_effects_cleared_for_cell_transition(world: &mut BevyoutWorld) {
    world
        .screen_fx
        .apply(screen_fx_policy::ScreenFxRequest::Clear {
            reason: screen_fx_policy::ScreenFxClearReason::CellTransition,
        });
}

#[then(regex = r"^the sampled screen blood is ([\d.]+)$")]
async fn then_sampled_screen_blood(world: &mut BevyoutWorld, expected: f32) {
    let actual = world.screen_fx_sample.expect("screen effect sample").blood;
    assert!(
        (actual - expected).abs() < 0.000_1,
        "actual blood {actual} != {expected}"
    );
}

#[then(regex = r"^(\d+) screen modifier is active$")]
async fn then_screen_modifier_count(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(world.screen_fx.active_len(), expected);
}

#[then(regex = r"^no screen modifier is active$")]
async fn then_no_screen_modifier_is_active(world: &mut BevyoutWorld) {
    assert_eq!(world.screen_fx.active_len(), 0);
}

#[then(regex = r"^the composed screen blood is ([\d.]+)$")]
async fn then_composed_screen_blood(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .screen_fx_sample
        .get_or_insert_with(|| world.screen_fx.snapshot())
        .blood;
    assert!(
        (actual - expected).abs() < 0.000_1,
        "actual blood {actual} != {expected}"
    );
}

#[then(regex = r"^the composed double vision is ([\d.]+)$")]
async fn then_composed_double_vision(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .screen_fx_sample
        .get_or_insert_with(|| world.screen_fx.snapshot())
        .double_vision;
    assert!(
        (actual - expected).abs() < 0.000_1,
        "actual double vision {actual} != {expected}"
    );
}
// =======================================================================
// -- m5_wave3_condition.feature (#262, #263) --
// Pure canonical condition/jam/RNG transactions used by the viewer adapter.
// =======================================================================

#[given(
    regex = r"^a canonical combat weapon item (\d+) form ([0-9A-Fa-f]+) with condition (\d+) of (\d+) and (\d+) loaded rounds?$"
)]
async fn given_canonical_combat_weapon(
    world: &mut BevyoutWorld,
    item_id: u64,
    form_id: String,
    condition: u32,
    max_condition: u32,
    loaded: u16,
) {
    let form_id =
        u32::from_str_radix(form_id.trim_start_matches("0x"), 16).expect("combat weapon FormID");
    let mut weapon = ItemInstance::new(
        ItemInstanceId(item_id),
        form_id,
        1,
        ItemState {
            condition: Some(condition),
            ..Default::default()
        },
    )
    .expect("combat weapon instance");
    weapon.state.combat.magazine.ammo_form_id = Some(0x0000_4241);
    weapon.state.combat.magazine.loaded = loaded;
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(
            HolderId::Player,
            ItemHolderState {
                items: vec![weapon],
                ..Default::default()
            },
        )
        .expect("combat player holder");
    ledger
        .equip(HolderId::Player, ItemInstanceId(item_id))
        .expect("combat weapon equip");
    world.combat_ledger = Some(ledger);
    world.combat_policy = WeaponConditionPolicy::with_degradation(Some(max_condition), 1);
    world.combat_receipt = None;
    world.combat_snapshot_before = None;
    world.combat_rng_before = None;
}

#[given(regex = r"^combat RNG seed (\d+)$")]
async fn given_combat_rng_seed(world: &mut BevyoutWorld, seed: u64) {
    world.combat_rng = CombatRngState::from_seed(seed);
}

#[given("the canonical combat weapon is jammed for fire")]
async fn given_canonical_combat_weapon_jammed_for_fire(world: &mut BevyoutWorld) {
    world
        .combat_ledger
        .as_mut()
        .expect("combat ledger")
        .holders_mut()
        .get_mut(&HolderId::Player)
        .expect("combat player holder")
        .items[0]
        .state
        .combat
        .jam = Some(bevyout_core::combat::JamReason::Fire);
}

#[given(regex = r"^the canonical combat weapon has (\d+) reserve rounds?$")]
async fn given_canonical_combat_reserve(world: &mut BevyoutWorld, count: u32) {
    let ledger = world.combat_ledger.as_mut().expect("combat ledger");
    ledger
        .insert_new_item(HolderId::Player, 0x0000_4241, count, ItemState::default())
        .expect("combat reserve rounds");
}

#[when(regex = r"^combat fire transaction (\d+) is (committed|attempted|repeated)$")]
async fn when_combat_fire_transaction(world: &mut BevyoutWorld, id: u64, _mode: String) {
    world.combat_snapshot_before = Some(
        world
            .combat_ledger
            .as_ref()
            .expect("combat ledger")
            .snapshot(),
    );
    world.combat_rng_before = Some(world.combat_rng.clone());
    let policy = world.combat_policy;
    let ledger = world.combat_ledger.as_mut().expect("combat ledger");
    world.combat_receipt = Some(ledger.fire_weapon_with_policy(
        TransactionId(id),
        HolderId::Player,
        ItemInstanceId(1),
        9.0,
        policy,
        &mut world.combat_rng,
    ));
}

#[when(regex = r"^a deterministic reload jam transaction (\d+) is committed$")]
async fn when_deterministic_reload_jam(world: &mut BevyoutWorld, id: u64) {
    let seed = (0..10_000u64)
        .find(|seed| {
            let mut rng = CombatRngState::from_seed(*seed);
            let draw = rng.draw(CombatRngDomain::ReloadJam).expect("reload draw");
            draw.value % BASIS_POINT_DENOMINATOR < MAX_JAM_CHANCE_BASIS_POINTS
        })
        .expect("reload jam seed");
    world.combat_rng = CombatRngState::from_seed(seed);
    let policy = world.combat_policy;
    let ledger = world.combat_ledger.as_mut().expect("combat ledger");
    world.combat_receipt = Some(ledger.reload_weapon_with_policy(
        TransactionId(id),
        HolderId::Player,
        ItemInstanceId(1),
        WeaponReloadRequest {
            ammo_form_id: 0x0000_4241,
            capacity: 12,
            policy,
        },
        &mut world.combat_rng,
    ));
}

#[when(regex = r"^combat clear-jam transaction (\d+) is committed$")]
async fn when_combat_clear_jam(world: &mut BevyoutWorld, id: u64) {
    world.combat_receipt = Some(
        world
            .combat_ledger
            .as_mut()
            .expect("combat ledger")
            .clear_weapon_jam_with_id(TransactionId(id), HolderId::Player, ItemInstanceId(1)),
    );
}

#[then("combat fire outcome is fired")]
async fn then_combat_fire_outcome_fired(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .combat_receipt
            .as_ref()
            .expect("combat receipt")
            .as_ref()
            .unwrap()
            .outcome,
        CombatTransactionOutcome::Fired
    );
}

#[then(regex = r"^combat condition is (\d+)$")]
async fn then_combat_condition(world: &mut BevyoutWorld, expected: u32) {
    let item = world
        .combat_ledger
        .as_ref()
        .expect("combat ledger")
        .holders()
        .get(&HolderId::Player)
        .expect("combat player")
        .find(ItemInstanceId(1))
        .expect("combat weapon");
    assert_eq!(item.state.condition, Some(expected));
}

#[then(regex = r"^combat damage is (\d+)$")]
async fn then_combat_damage(world: &mut BevyoutWorld, expected: u32) {
    assert_eq!(
        world
            .combat_receipt
            .as_ref()
            .expect("combat receipt")
            .as_ref()
            .expect("accepted combat action")
            .damage_milli,
        Some(expected * 1_000)
    );
}

#[then(regex = r"^combat draw index is (\d+)$")]
async fn then_combat_draw_index(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(world.combat_rng.draw_index, expected);
}

#[then("combat action is rejected as jammed")]
async fn then_combat_action_rejected_as_jammed(world: &mut BevyoutWorld) {
    assert_eq!(
        world.combat_receipt.as_ref().expect("combat receipt"),
        &Err(TransactionError::Jammed(
            bevyout_core::combat::JamReason::Fire
        ))
    );
}

#[then("the combat snapshot is unchanged")]
async fn then_combat_snapshot_unchanged(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .combat_ledger
            .as_ref()
            .expect("combat ledger")
            .snapshot(),
        world
            .combat_snapshot_before
            .as_ref()
            .expect("pre-action combat snapshot")
            .clone()
    );
    assert_eq!(
        world.combat_rng,
        world
            .combat_rng_before
            .as_ref()
            .expect("pre-action combat RNG")
            .clone()
    );
}

#[then("combat reload outcome is jammed")]
async fn then_combat_reload_outcome_jammed(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .combat_receipt
            .as_ref()
            .expect("combat receipt")
            .as_ref()
            .unwrap()
            .outcome,
        CombatTransactionOutcome::Jammed
    );
}

#[then("combat weapon 1 is jammed for reload")]
async fn then_combat_weapon_jammed_for_reload(world: &mut BevyoutWorld) {
    let item = world
        .combat_ledger
        .as_ref()
        .expect("combat ledger")
        .holders()
        .get(&HolderId::Player)
        .expect("combat player")
        .find(ItemInstanceId(1))
        .expect("combat weapon");
    assert_eq!(
        item.state.combat.jam,
        Some(bevyout_core::combat::JamReason::Reload)
    );
}

#[then("combat clear-jam outcome is cleared")]
async fn then_combat_clear_jam_outcome_cleared(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .combat_receipt
            .as_ref()
            .expect("combat receipt")
            .as_ref()
            .unwrap()
            .outcome,
        CombatTransactionOutcome::Cleared
    );
}

#[then("combat weapon 1 is not jammed")]
async fn then_combat_weapon_not_jammed(world: &mut BevyoutWorld) {
    let item = world
        .combat_ledger
        .as_ref()
        .expect("combat ledger")
        .holders()
        .get(&HolderId::Player)
        .expect("combat player")
        .find(ItemInstanceId(1))
        .expect("combat weapon");
    assert_eq!(item.state.combat.jam, None);
}

// ---------------------------------------------------------------------
// realtime_shadow_policy.feature (issue #267, PERF wave 1) -- appended
// section, do not interleave.
// ---------------------------------------------------------------------

#[given("the realtime shadow setting changed this frame")]
async fn given_realtime_shadow_setting_changed(world: &mut BevyoutWorld) {
    world.realtime_shadow_settings_changed = true;
}

#[given("the realtime shadow setting was not changed this frame")]
async fn given_realtime_shadow_setting_unchanged(world: &mut BevyoutWorld) {
    world.realtime_shadow_settings_changed = false;
}

#[given("no realtime shadow light is selected")]
async fn given_no_realtime_shadow_selection(world: &mut BevyoutWorld) {
    world.realtime_shadow_selection_active = false;
}

#[given("a realtime shadow light is selected")]
async fn given_realtime_shadow_selection(world: &mut BevyoutWorld) {
    world.realtime_shadow_selection_active = true;
}

#[when("the disabled realtime-shadow write gate runs")]
async fn when_realtime_shadow_write_gate_runs(world: &mut BevyoutWorld) {
    world.realtime_shadow_writes_needed =
        Some(realtime_shadow_policy::disabled_shadow_writes_needed(
            world.realtime_shadow_settings_changed,
            world.realtime_shadow_selection_active,
        ));
}

#[then("the disabled realtime-shadow pass is skipped")]
async fn then_realtime_shadow_pass_skipped(world: &mut BevyoutWorld) {
    assert_eq!(world.realtime_shadow_writes_needed, Some(false));
}

#[then("the disabled realtime-shadow pass clears candidate state")]
async fn then_realtime_shadow_pass_cleans(world: &mut BevyoutWorld) {
    assert_eq!(world.realtime_shadow_writes_needed, Some(true));
}

// ---------------------------------------------------------------------
// scene_classification.feature (issue #270, PERF wave 1) -- appended
// section, do not interleave.
// ---------------------------------------------------------------------

#[when(regex = r#"^the glow card naming policy is asked about \"([^\"]*)\"$"#)]
async fn when_glow_card_naming_asked(world: &mut BevyoutWorld, name: String) {
    world.glow_name_check = Some(glow_card_policy::is_glow_card_mesh_name(&name));
}

#[then("the glow card naming policy reports a glow card")]
async fn then_glow_card_naming_matches(world: &mut BevyoutWorld) {
    assert_eq!(world.glow_name_check, Some(true));
}

#[then("the glow card naming policy does not report a glow card")]
async fn then_glow_card_naming_rejects(world: &mut BevyoutWorld) {
    assert_eq!(world.glow_name_check, Some(false));
}

#[given("a fresh AO eligibility tracker")]
async fn given_fresh_ao_tracker(world: &mut BevyoutWorld) {
    world.scene_ao_tracker = ao_policy::AoEligibilityTracker::default();
}

#[when(
    regex = r#"^mesh entity (\d+) is discovered with mesh asset (\d+) as (eligible|ineligible)$"#
)]
async fn when_ao_entity_discovered(
    world: &mut BevyoutWorld,
    entity: u32,
    mesh: u32,
    outcome: String,
) {
    world
        .scene_ao_tracker
        .discover(entity, mesh, outcome == "eligible");
}

#[when(regex = r#"^mesh entity (\d+) releases its mesh$"#)]
async fn when_ao_entity_released(world: &mut BevyoutWorld, entity: u32) {
    world.scene_ao_tracker.release(entity);
}

#[when(regex = r#"^mesh asset (\d+) is added to the asset store$"#)]
async fn when_ao_asset_added(world: &mut BevyoutWorld, mesh: u32) {
    world.scene_ao_tracker.asset_added(mesh);
}

#[when(regex = r#"^AO processing resolves mesh asset (\d+)$"#)]
async fn when_ao_processing_resolves(world: &mut BevyoutWorld, mesh: u32) {
    world.scene_ao_tracker.resolve_pending(mesh);
}

#[then(regex = r#"^mesh asset (\d+) is pending for AO processing$"#)]
async fn then_ao_mesh_pending(world: &mut BevyoutWorld, mesh: u32) {
    assert!(world.scene_ao_tracker.is_pending(mesh));
}

#[then(regex = r#"^mesh asset (\d+) is tracked as eligible$"#)]
async fn then_ao_mesh_eligible(world: &mut BevyoutWorld, mesh: u32) {
    assert!(world.scene_ao_tracker.is_eligible(mesh));
}

#[then(regex = r#"^mesh asset (\d+) is not tracked as eligible$"#)]
async fn then_ao_mesh_not_eligible(world: &mut BevyoutWorld, mesh: u32) {
    assert!(!world.scene_ao_tracker.is_eligible(mesh));
}

#[then("the AO tracker has no pending meshes")]
async fn then_ao_tracker_quiet(world: &mut BevyoutWorld) {
    assert!(!world.scene_ao_tracker.has_pending());
}

// ---------------------------------------------------------------------
// material_clamp_policy.feature (issue #269, PERF wave 1) -- appended
// section, do not interleave.
// ---------------------------------------------------------------------

/// One simulated `apply_material_clamps` frame over the spec's integer-keyed
/// material store: a full pass when (and only when) the settings revision
/// moved. Load/remove steps feed the incremental path directly.
fn material_clamp_frame(world: &mut BevyoutWorld) {
    let full_pass = world.clamp_store.needs_full_pass(&world.clamp_settings);
    world.clamp_full_pass_ran = Some(full_pass);
    if !full_pass {
        return;
    }
    let ids: Vec<u32> = world.clamp_materials.keys().copied().collect();
    for id in ids {
        let current = world.clamp_materials[&id];
        let mut baseline = world.clamp_store.take(id);
        let target = material_clamp_policy::decide(&world.clamp_settings, &mut baseline, current);
        world.clamp_materials.insert(id, target);
        world.clamp_store.record(id, baseline);
    }
    world.clamp_store.prune_disengaged(&world.clamp_settings);
    world.clamp_store.mark_applied(&world.clamp_settings);
}

fn material_clamp_factors(
    metallic: f32,
    reflectance: f32,
    roughness: f32,
) -> material_clamp_policy::MaterialFactors {
    material_clamp_policy::MaterialFactors {
        metallic,
        reflectance,
        perceptual_roughness: roughness,
    }
}

#[given("a fresh material clamp policy")]
async fn given_fresh_material_clamp_policy(world: &mut BevyoutWorld) {
    world.clamp_settings = material_clamp_policy::ClampSettings::default();
    world.clamp_store = material_clamp_policy::ClampStore::default();
    world.clamp_materials = std::collections::HashMap::new();
    world.clamp_full_pass_ran = None;
}

#[given(
    regex = r#"^clamp material (\d+) has metallic ([\d.]+), reflectance ([\d.]+), and roughness ([\d.]+)$"#
)]
async fn given_clamp_material(
    world: &mut BevyoutWorld,
    id: u32,
    metallic: f32,
    reflectance: f32,
    roughness: f32,
) {
    world
        .clamp_materials
        .insert(id, material_clamp_factors(metallic, reflectance, roughness));
}

#[when(regex = r#"^the clamp \"(metallic|dielectric_specular)\" gate engages$"#)]
async fn when_clamp_gate_engages(world: &mut BevyoutWorld, gate: String) {
    match gate.as_str() {
        "metallic" => world.clamp_settings.set_metallic_enabled(false),
        "dielectric_specular" => world.clamp_settings.set_dielectric_enabled(false),
        other => panic!("unknown clamp gate {other:?}"),
    }
    material_clamp_frame(world);
}

#[when(regex = r#"^the clamp \"(metallic|dielectric_specular)\" gate disengages$"#)]
async fn when_clamp_gate_disengages(world: &mut BevyoutWorld, gate: String) {
    match gate.as_str() {
        "metallic" => world.clamp_settings.set_metallic_enabled(true),
        "dielectric_specular" => world.clamp_settings.set_dielectric_enabled(true),
        other => panic!("unknown clamp gate {other:?}"),
    }
    material_clamp_frame(world);
}

#[when(regex = r#"^the clamp roughness scale becomes ([\d.]+)$"#)]
async fn when_clamp_roughness_scale(world: &mut BevyoutWorld, scale: f32) {
    world.clamp_settings.set_roughness_scale(scale);
    material_clamp_frame(world);
}

/// The incremental `AssetEvent::Added` path: baseline + clamp inside the
/// already-engaged pass, without touching any other material.
#[when(
    regex = r#"^clamp material (\d+) loads with metallic ([\d.]+), reflectance ([\d.]+), and roughness ([\d.]+)$"#
)]
async fn when_clamp_material_loads(
    world: &mut BevyoutWorld,
    id: u32,
    metallic: f32,
    reflectance: f32,
    roughness: f32,
) {
    assert!(
        !world.clamp_store.needs_full_pass(&world.clamp_settings),
        "load while engaged is incremental: the settings must be settled"
    );
    let current = material_clamp_factors(metallic, reflectance, roughness);
    let stored = if world.clamp_settings.any_engaged() {
        let mut baseline = world.clamp_store.take(id);
        let target = material_clamp_policy::decide(&world.clamp_settings, &mut baseline, current);
        world.clamp_store.record(id, baseline);
        target
    } else {
        current
    };
    world.clamp_materials.insert(id, stored);
    world.clamp_full_pass_ran = Some(false);
}

#[when(regex = r#"^clamp material (\d+) is removed from the asset store$"#)]
async fn when_clamp_material_removed(world: &mut BevyoutWorld, id: u32) {
    world.clamp_materials.remove(&id);
    world.clamp_store.release(id);
}

#[when("a steady clamp frame runs")]
async fn when_steady_clamp_frame_runs(world: &mut BevyoutWorld) {
    material_clamp_frame(world);
}

#[then(
    regex = r#"^clamp material (\d+) has metallic ([\d.]+), reflectance ([\d.]+), and roughness ([\d.]+)$"#
)]
async fn then_clamp_material_factors(
    world: &mut BevyoutWorld,
    id: u32,
    metallic: f32,
    reflectance: f32,
    roughness: f32,
) {
    let actual = world
        .clamp_materials
        .get(&id)
        .expect("clamp material must exist in the spec store");
    assert_eq!(
        *actual,
        material_clamp_factors(metallic, reflectance, roughness)
    );
}

#[then("the clamp store holds no baselines")]
async fn then_clamp_store_holds_no_baselines(world: &mut BevyoutWorld) {
    assert_eq!(world.clamp_store.baseline_count(), 0);
}

#[then(regex = r#"^the clamp store keeps (\d+) baseline entry$"#)]
async fn then_clamp_store_keeps_entries(world: &mut BevyoutWorld, count: usize) {
    assert_eq!(world.clamp_store.baseline_count(), count);
}

#[then("a steady clamp frame performs no full material pass")]
async fn then_steady_clamp_frame_performs_no_full_pass(world: &mut BevyoutWorld) {
    material_clamp_frame(world);
    assert_eq!(world.clamp_full_pass_ran, Some(false));
}

#[then("the steady clamp frame needed no full material pass")]
async fn then_steady_clamp_frame_needed_no_full_pass(world: &mut BevyoutWorld) {
    assert_eq!(world.clamp_full_pass_ran, Some(false));
}
// =======================================================================
// -- M4 gate wave: #222/#231/#242 ----------------------------------------
// Appended merge-seam steps for the pure resolver and patrol regressions.
// =======================================================================

#[then(regex = r"^the location radius is ([\d.]+)$")]
async fn then_location_radius_is(world: &mut BevyoutWorld, radius: f32) {
    assert!((ai_res_point(world).radius - radius).abs() < 1e-5);
}

#[given(
    regex = r"^a patrol family over markers \(([^)]*)\) then \(([^)]*)\) then \(([^)]*)\) with tolerance ([0-9.]+)$"
)]
async fn given_short_tolerance_patrol_family(
    world: &mut BevyoutWorld,
    a: String,
    b: String,
    c: String,
    tolerance: String,
) {
    let markers = [pf_parse_point(&a), pf_parse_point(&b), pf_parse_point(&c)];
    let tolerance: f32 = tolerance.parse().expect("tolerance");
    world.pf_markers = markers.to_vec();
    let waypoints = markers
        .iter()
        .map(|position| ai_families::Waypoint::at(*position))
        .collect();
    world.pf_driver = Some(ai_families::FamilyDriver::new(
        ai_families::PackageFamily::Patrol,
        waypoints,
        tolerance,
    ));
}

#[given(
    regex = r"^a patrol family with default marker dwell over markers \(([^)]*)\) then \(([^)]*)\) then \(([^)]*)\)$"
)]
async fn given_default_dwell_patrol_family(
    world: &mut BevyoutWorld,
    a: String,
    b: String,
    c: String,
) {
    let markers = [pf_parse_point(&a), pf_parse_point(&b), pf_parse_point(&c)];
    world.pf_markers = markers.to_vec();
    let waypoints = markers
        .iter()
        .map(|position| ai_families::Waypoint::patrol_marker(*position, None))
        .collect();
    world.pf_driver = Some(ai_families::FamilyDriver::new(
        ai_families::PackageFamily::Patrol,
        waypoints,
        ai_families::DEFAULT_ARRIVAL_TOLERANCE,
    ));
}

#[when(regex = r"^the actor remains at patrol marker ([0-9]+) for (\d+) ticks$")]
async fn when_actor_remains_at_patrol_marker(world: &mut BevyoutWorld, index: usize, ticks: usize) {
    let position = world.pf_markers[index];
    for _ in 0..ticks {
        pf_tick(world, position, true);
    }
}

#[when(regex = r"^the patrol family ticks ([\d.]+) seconds at patrol marker ([0-9]+)$")]
async fn when_patrol_family_ticks_at_marker(world: &mut BevyoutWorld, seconds: f32, index: usize) {
    let position = world.pf_markers[index];
    let observation = ai_families::FamilyObservation::new(position, true, false);
    let step = pf_driver(world).tick(&observation, seconds);
    world.pf_step = Some(step);
}

#[then(regex = r"^the family remains at patrol marker ([0-9]+)$")]
async fn then_family_remains_at_patrol_marker(world: &mut BevyoutWorld, index: usize) {
    assert_eq!(pf_driver(world).marker_index(), index);
    assert_ne!(
        pf_step(world).signal,
        ai_families::LifecycleSignal::AdvanceStep,
        "stationary patrol must not advance repeatedly"
    );
}

#[then(regex = r"^the family stops at patrol marker ([0-9]+) for its dwell$")]
async fn then_family_stops_at_patrol_marker(world: &mut BevyoutWorld, index: usize) {
    assert_eq!(pf_driver(world).marker_index(), index);
    assert_eq!(pf_driver(world).step_label(), "waiting");
    assert_eq!(
        pf_step(world).request,
        Some(ai_families::FamilyRequest::Stop)
    );
    assert_eq!(
        pf_step(world).signal,
        ai_families::LifecycleSignal::Continue
    );
}

#[then(regex = r"^the family holds idle at patrol marker ([0-9]+)$")]
async fn then_family_holds_idle_at_patrol_marker(world: &mut BevyoutWorld, index: usize) {
    assert_eq!(pf_driver(world).marker_index(), index);
    assert_eq!(pf_driver(world).step_label(), "waiting");
    assert_eq!(
        pf_step(world).request,
        Some(ai_families::FamilyRequest::Play(
            ai_families::FamilyAnimation::Idle
        ))
    );
    assert_eq!(
        pf_step(world).signal,
        ai_families::LifecycleSignal::Continue
    );
}
// ---------------------------------------------------------------------
// m6_wave2_exterior.feature -- generation-aware reversal and process memory.
// ---------------------------------------------------------------------

#[given(regex = r"^the exterior index contains cell 0x([0-9a-fA-F]+) at grid \((-?\d+),(-?\d+)\)$")]
async fn given_m6_exterior_index_cell(
    world: &mut BevyoutWorld,
    form_hex: String,
    x: String,
    y: String,
) {
    let grid = bevyout_core::manifest::exterior::GridCoordinate::new(
        x.parse().expect("exterior grid x must be an integer"),
        y.parse().expect("exterior grid y must be an integer"),
    );
    world.m6_exterior_index.insert(grid, parse_hex(&form_hex));
}

#[given(
    regex = r"^the exterior stream is at grid \((-?\d+),(-?\d+)\) moving toward \((-?\d+),(-?\d+)\)$"
)]
async fn given_m6_exterior_stream_direction(
    world: &mut BevyoutWorld,
    x: String,
    y: String,
    velocity_x: String,
    velocity_y: String,
) {
    world.m6_exterior_input = Some(bevyout_core::manifest::exterior::ExteriorResidencyInput {
        current_grid: bevyout_core::manifest::exterior::GridCoordinate::new(
            x.parse().expect("exterior current x must be an integer"),
            y.parse().expect("exterior current y must be an integer"),
        ),
        velocity_grid: (
            velocity_x
                .parse()
                .expect("exterior velocity x must be an integer"),
            velocity_y
                .parse()
                .expect("exterior velocity y must be an integer"),
        ),
        resident_budget: 4,
        byte_budget: u64::MAX,
        near_radius: 1,
        prefetch_radius: 1,
        distant_radius: None,
    });
}

#[given(
    regex = r"^exterior cell 0x([0-9a-fA-F]+) at grid \((-?\d+),(-?\d+)\) is Evicting at generation (\d+)$"
)]
async fn given_m6_exterior_evicting_cell(
    world: &mut BevyoutWorld,
    form_hex: String,
    x: String,
    y: String,
    generation: u64,
) {
    world
        .m6_exterior_states
        .push(bevyout_core::manifest::exterior::ExteriorCellState {
            cell_form_id: parse_hex(&form_hex),
            grid: bevyout_core::manifest::exterior::GridCoordinate::new(
                x.parse().expect("exterior cell x must be an integer"),
                y.parse().expect("exterior cell y must be an integer"),
            ),
            lifecycle: bevyout_core::manifest::exterior::ExteriorCellLifecycle::Evicting,
            generation,
            pinned: false,
            estimated_bytes: 1_024,
            failed_attempts: 0,
        });
}

#[when("the exterior residency plan is computed")]
async fn when_m6_exterior_plan_computed(world: &mut BevyoutWorld) {
    let input = world
        .m6_exterior_input
        .expect("exterior stream input must be configured");
    world.m6_exterior_plan = Some(bevyout_core::manifest::exterior::plan_residency(
        input,
        &world.m6_exterior_index,
        &world.m6_exterior_states,
    ));
}

#[then(regex = r"^the exterior plan cancels cell 0x([0-9a-fA-F]+) at generation (\d+)$")]
async fn then_m6_exterior_plan_cancels(
    world: &mut BevyoutWorld,
    form_hex: String,
    generation: u64,
) {
    let form_id = parse_hex(&form_hex);
    let action = world
        .m6_exterior_plan
        .as_ref()
        .expect("exterior residency plan must be computed")
        .actions
        .iter()
        .find(|action| action.form_id == form_id)
        .unwrap_or_else(|| panic!("no exterior action was emitted for {form_hex}"));
    assert_eq!(
        action.action,
        bevyout_core::manifest::exterior::ExteriorLoadAction::Cancel
    );
    assert_eq!(action.generation, generation);
}

#[given(regex = r#"^an exterior process-memory trace with samples "([^"]+)"$"#)]
async fn given_m6_exterior_memory_trace(world: &mut BevyoutWorld, samples: String) {
    let mut memory = exterior_diagnostics::ProcessMemoryDiagnostics::supported_for_tests();
    memory.begin_trace();
    for sample in samples.split(',') {
        memory.record_sample(
            sample
                .parse()
                .expect("process-memory samples must be unsigned integers"),
        );
    }
    memory.finish_trace();
    world.m6_exterior_memory = Some(memory);
}

#[given(regex = r"^the exterior package estimates are resident (\d+) and peak (\d+)$")]
async fn given_m6_exterior_package_estimates(world: &mut BevyoutWorld, resident: u64, peak: u64) {
    world.m6_exterior_memory_state = Some(lifecycle::ExteriorStreamState {
        resident_bytes: resident,
        peak_memory: peak,
        ..Default::default()
    });
}

#[when("the exterior process-memory report is rendered")]
async fn when_m6_exterior_memory_report_rendered(world: &mut BevyoutWorld) {
    let state = world
        .m6_exterior_memory_state
        .as_ref()
        .expect("exterior package estimates must be configured");
    let memory = world
        .m6_exterior_memory
        .as_ref()
        .expect("exterior memory trace must be configured");
    world.m6_exterior_memory_report = Some(exterior_diagnostics::status_with_memory(state, memory));
}

#[then(regex = r#"^the exterior report identifies process memory as "([^"]+)"$"#)]
async fn then_m6_exterior_report_measurement(world: &mut BevyoutWorld, expected: String) {
    let report = world
        .m6_exterior_memory_report
        .as_ref()
        .expect("exterior memory report must be rendered");
    assert_eq!(
        report["memory_measurement"].as_str(),
        Some(expected.as_str())
    );
}

#[then(regex = r"^exterior process memory is resident (\d+), peak (\d+), ending (\d+)$")]
async fn then_m6_exterior_process_memory(
    world: &mut BevyoutWorld,
    resident: u64,
    peak: u64,
    ending: u64,
) {
    let report = world
        .m6_exterior_memory_report
        .as_ref()
        .expect("exterior memory report must be rendered");
    assert_eq!(report["resident_bytes"].as_u64(), Some(resident));
    assert_eq!(report["peak_memory"].as_u64(), Some(peak));
    assert_eq!(report["ending_memory"].as_u64(), Some(ending));
}

#[then(regex = r"^exterior package estimates remain resident (\d+) and peak (\d+)$")]
async fn then_m6_exterior_package_estimates(world: &mut BevyoutWorld, resident: u64, peak: u64) {
    let report = world
        .m6_exterior_memory_report
        .as_ref()
        .expect("exterior memory report must be rendered");
    assert_eq!(
        report["resident_package_bytes_estimate"].as_u64(),
        Some(resident)
    );
    assert_eq!(report["peak_package_bytes_estimate"].as_u64(), Some(peak));
}

// ---------------------------------------------------------------------
// m6_wave3_policy.feature -- canonical actor residency and resident NAVM.
// ---------------------------------------------------------------------

#[given(
    regex = r"^actor identity runtime 0x([0-9a-fA-F]+) prepared 0x([0-9a-fA-F]+) and saved state 0x([0-9a-fA-F]+)$"
)]
async fn given_m6_actor_identity(
    world: &mut BevyoutWorld,
    runtime: String,
    prepared: String,
    saved: String,
) {
    world.m6_actor_identity = Some(
        actor_residency::resolve_actor_identity(
            parse_hex(&runtime),
            parse_hex(&prepared),
            Some(parse_hex(&saved)),
        )
        .expect("actor identity must agree across runtime, prepared, and saved state"),
    );
}

#[given("the actor has no resident owner")]
async fn given_m6_actor_has_no_resident_owner(world: &mut BevyoutWorld) {
    world.m6_actor_owners.clear();
}

fn m6_actor_owner(cell: &str, generation: u64) -> actor_residency::ActorResidencyOwner {
    actor_residency::ActorResidencyOwner::new(parse_hex(cell), generation)
        .expect("actor residency cell must be non-zero")
}

#[given(regex = r"^the actor is observed in cell 0x([0-9a-fA-F]+) generation (\d+)$")]
async fn given_m6_actor_observed_in_cell(world: &mut BevyoutWorld, cell: String, generation: u64) {
    world.m6_actor_owners = vec![m6_actor_owner(&cell, generation)];
}

#[given(
    regex = r"^the actor has resident owners in cell 0x([0-9a-fA-F]+) generation (\d+) and cell 0x([0-9a-fA-F]+) generation (\d+)$"
)]
async fn given_m6_actor_duplicate_resident_owners(
    world: &mut BevyoutWorld,
    first_cell: String,
    first_generation: u64,
    second_cell: String,
    second_generation: u64,
) {
    world.m6_actor_owners = vec![
        m6_actor_owner(&first_cell, first_generation),
        m6_actor_owner(&second_cell, second_generation),
    ];
}

fn m6_actor_identity(world: &BevyoutWorld) -> actor_residency::ActorIdentity {
    world
        .m6_actor_identity
        .expect("actor identity must be configured first")
}

fn m6_decide_actor(world: &mut BevyoutWorld, request: actor_residency::ActorResidencyRequest) {
    world.m6_actor_decision = Some(actor_residency::decide_actor_residency(
        m6_actor_identity(world),
        &world.m6_actor_owners,
        request,
    ));
}

#[when(regex = r"^actor residency bind is requested for cell 0x([0-9a-fA-F]+) generation (\d+)$")]
async fn when_m6_actor_bind_requested(world: &mut BevyoutWorld, cell: String, generation: u64) {
    m6_decide_actor(
        world,
        actor_residency::ActorResidencyRequest::Bind {
            destination: m6_actor_owner(&cell, generation),
        },
    );
}

#[when(regex = r"^actor residency retain is requested for cell 0x([0-9a-fA-F]+) generation (\d+)$")]
async fn when_m6_actor_retain_requested(world: &mut BevyoutWorld, cell: String, generation: u64) {
    m6_decide_actor(
        world,
        actor_residency::ActorResidencyRequest::Retain {
            owner: m6_actor_owner(&cell, generation),
        },
    );
}

#[when(
    regex = r"^actor residency handoff is requested from cell 0x([0-9a-fA-F]+) generation (\d+) to cell 0x([0-9a-fA-F]+) generation (\d+)$"
)]
async fn when_m6_actor_handoff_requested(
    world: &mut BevyoutWorld,
    source_cell: String,
    source_generation: u64,
    destination_cell: String,
    destination_generation: u64,
) {
    m6_decide_actor(
        world,
        actor_residency::ActorResidencyRequest::Handoff {
            source: m6_actor_owner(&source_cell, source_generation),
            destination: m6_actor_owner(&destination_cell, destination_generation),
        },
    );
}

#[when(
    regex = r"^actor residency unload is requested from cell 0x([0-9a-fA-F]+) generation (\d+)$"
)]
async fn when_m6_actor_unload_requested(world: &mut BevyoutWorld, cell: String, generation: u64) {
    m6_decide_actor(
        world,
        actor_residency::ActorResidencyRequest::Unload {
            source: m6_actor_owner(&cell, generation),
        },
    );
}

#[when(
    regex = r"^actor residency restore is requested for cell 0x([0-9a-fA-F]+) generation (\d+)$"
)]
async fn when_m6_actor_restore_requested(world: &mut BevyoutWorld, cell: String, generation: u64) {
    m6_decide_actor(
        world,
        actor_residency::ActorResidencyRequest::Restore {
            destination: m6_actor_owner(&cell, generation),
        },
    );
}

#[when("the actor projection is cleared")]
async fn when_m6_actor_projection_cleared(world: &mut BevyoutWorld) {
    world.m6_actor_owners.clear();
}

fn m6_actor_decision(world: &BevyoutWorld) -> actor_residency::ActorResidencyDecision {
    world
        .m6_actor_decision
        .expect("actor residency decision must be computed first")
}

#[then("actor residency accepts a Bind transition")]
async fn then_m6_actor_accepts_bind(world: &mut BevyoutWorld) {
    let actor_residency::ActorResidencyDecision::Apply(plan) = m6_actor_decision(world) else {
        panic!("expected an accepted actor Bind decision");
    };
    assert!(matches!(
        plan.transition(),
        actor_residency::ActorResidencyTransition::Bind { .. }
    ));
}

#[then("actor residency accepts a Handoff transition")]
async fn then_m6_actor_accepts_handoff(world: &mut BevyoutWorld) {
    let actor_residency::ActorResidencyDecision::Apply(plan) = m6_actor_decision(world) else {
        panic!("expected an accepted actor Handoff decision");
    };
    assert!(matches!(
        plan.transition(),
        actor_residency::ActorResidencyTransition::Handoff { .. }
    ));
}

#[then("actor residency accepts an Unload transition")]
async fn then_m6_actor_accepts_unload(world: &mut BevyoutWorld) {
    let actor_residency::ActorResidencyDecision::Apply(plan) = m6_actor_decision(world) else {
        panic!("expected an accepted actor Unload decision");
    };
    assert!(matches!(
        plan.transition(),
        actor_residency::ActorResidencyTransition::Unload { .. }
    ));
}

#[then("actor residency accepts a Restore transition")]
async fn then_m6_actor_accepts_restore(world: &mut BevyoutWorld) {
    let actor_residency::ActorResidencyDecision::Apply(plan) = m6_actor_decision(world) else {
        panic!("expected an accepted actor Restore decision");
    };
    assert!(matches!(
        plan.transition(),
        actor_residency::ActorResidencyTransition::Restore { .. }
    ));
}

#[then(regex = r"^the canonical actor holder is actor reference 0x([0-9a-fA-F]+)$")]
async fn then_m6_canonical_actor_holder(world: &mut BevyoutWorld, reference: String) {
    let actor_residency::ActorResidencyDecision::Apply(plan) = m6_actor_decision(world) else {
        panic!("canonical actor holder requires an accepted residency decision");
    };
    let reference_form_id = parse_hex(&reference);
    assert_eq!(
        plan.actor().reference_form_id(),
        reference_form_id,
        "the residency plan must retain the prepared/runtime reference identity"
    );
    assert_eq!(plan.holder(), HolderId::Actor { reference_form_id });
}

#[then("actor residency rejects a StaleSource decision")]
async fn then_m6_actor_rejects_stale(world: &mut BevyoutWorld) {
    assert!(matches!(
        m6_actor_decision(world),
        actor_residency::ActorResidencyDecision::Reject(
            actor_residency::ActorResidencyRejection::StaleSource { .. }
        )
    ));
}

#[then("actor residency rejects a DuplicateOwner decision")]
async fn then_m6_actor_rejects_duplicate(world: &mut BevyoutWorld) {
    assert!(matches!(
        m6_actor_decision(world),
        actor_residency::ActorResidencyDecision::Reject(
            actor_residency::ActorResidencyRejection::DuplicateOwner { .. }
        )
    ));
}

fn m6_resident_meshes() -> Vec<landmass_graph::MeshInput> {
    let mesh = landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
        polygons: vec![
            landmass_graph::PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            landmass_graph::PolygonInput {
                index: 1,
                vertex_indices: [1, 3, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    };
    let mut second = mesh.clone();
    second.form_id = 0x20;
    vec![mesh, second]
}

fn m6_resident_key(cell_form_id: &str, generation: u64) -> landmass_graph::ResidentNavCellKey {
    landmass_graph::ResidentNavCellKey {
        cell_form_id: parse_hex(cell_form_id),
        generation,
    }
}

fn m6_set_resident_nav_state(
    world: &mut BevyoutWorld,
    cell_form_id: String,
    generation: u64,
    state: &str,
) {
    let key = m6_resident_key(&cell_form_id, generation);
    let state = match state {
        "navigation-ready" => landmass_graph::ResidentNavState::Valid {
            navigation_ready: true,
        },
        "Loading" => landmass_graph::ResidentNavState::Loading,
        "Evicting" => landmass_graph::ResidentNavState::Evicting,
        other => panic!("unknown resident NAVM state {other:?}"),
    };
    // The fixture models the lifecycle owner’s current resident set: a
    // reload replaces the old generation for the same cell FormID instead of
    // leaving both generations live in the observation.
    world
        .m6_nav_residents
        .retain(|resident| resident.key.cell_form_id != key.cell_form_id);
    world
        .m6_nav_residents
        .push(landmass_graph::ResidentNavCell { key, state });
}

#[given(
    regex = r"^resident NAVM side 0x([0-9a-fA-F]+) generation (\d+) is (navigation-ready|Loading|Evicting)$"
)]
async fn given_m6_resident_nav_state(
    world: &mut BevyoutWorld,
    cell_form_id: String,
    generation: u64,
    state: String,
) {
    m6_set_resident_nav_state(world, cell_form_id, generation, &state);
}

#[given(
    regex = r"^a resident NAVM portal joins side 0x([0-9a-fA-F]+) generation (\d+) to side 0x([0-9a-fA-F]+) generation (\d+)$"
)]
async fn given_m6_resident_nav_portal(
    world: &mut BevyoutWorld,
    side_a_cell: String,
    side_a_generation: u64,
    side_b_cell: String,
    side_b_generation: u64,
) {
    world.m6_nav_portals.push(landmass_graph::ResidentPortal {
        side_a: m6_resident_key(&side_a_cell, side_a_generation),
        side_b: m6_resident_key(&side_b_cell, side_b_generation),
        merge: landmass_graph::MergeInput {
            mesh_a_form_id: 0x10,
            triangle_a: 0,
            mesh_b_form_id: 0x20,
            triangle_b: 1,
            interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            interval_b: [[2.0, 0.0, 0.0], [3.0, 0.0, 0.0]],
        },
    });
}

#[when("the resident NAVM topology is rebuilt")]
async fn when_m6_resident_nav_topology_rebuilt(world: &mut BevyoutWorld) {
    let meshes = m6_resident_meshes();
    world
        .m6_nav_topology
        .rebuild(&world.m6_nav_residents, &meshes, &world.m6_nav_portals);
}

#[then(regex = r"^resident NAVM has (\d+) ready cells? and (\d+) cross-cell links?$")]
async fn then_m6_resident_nav_topology_counts(
    world: &mut BevyoutWorld,
    resident_count: usize,
    link_count: usize,
) {
    let archipelago = world.m6_nav_topology.archipelago.as_ref();
    assert_eq!(
        archipelago.map_or(0, |archipelago| archipelago.resident_cells.len()),
        resident_count
    );
    assert_eq!(
        archipelago.map_or(0, |archipelago| archipelago.merge_links.len()),
        link_count
    );
}

// =======================================================================
// M4 Craterside NPC animation repair, Lane B package seams.
// These steps are appended-only shared feature steps for #292.
// =======================================================================

fn lane_b_pkdt_payload(
    length: usize,
    package_type: u8,
    behavior_flags: u16,
    general_flags: u32,
) -> Vec<u8> {
    let mut data = general_flags.to_le_bytes().to_vec();
    match length {
        4 => {}
        8 => {
            data.extend_from_slice(&[package_type, 0]);
            data.extend_from_slice(&behavior_flags.to_le_bytes());
        }
        12 => {
            data.extend_from_slice(&[package_type, 0]);
            data.extend_from_slice(&behavior_flags.to_le_bytes());
            data.extend_from_slice(&0_u16.to_le_bytes());
            data.extend_from_slice(&[0, 0]);
        }
        other => panic!("unsupported PKDT fixture length {other}"),
    }
    data
}

#[given(regex = r"^a 4-byte PKDT package 0x([0-9a-fA-F]+) with general flags 0x([0-9a-fA-F]+)$")]
async fn given_lane_b_pkdt_four(world: &mut BevyoutWorld, form_hex: String, general_hex: String) {
    world
        .package_pkdt_fixtures
        .push((parse_hex(&form_hex), 4, 0, 0, parse_hex(&general_hex)));
}

#[given(
    regex = r"^an 8-byte PKDT package 0x([0-9a-fA-F]+) with type (\d+) and behavior flags 0x([0-9a-fA-F]+)$"
)]
async fn given_lane_b_pkdt_eight(
    world: &mut BevyoutWorld,
    form_hex: String,
    package_type: u8,
    behavior_hex: String,
) {
    world.package_pkdt_fixtures.push((
        parse_hex(&form_hex),
        8,
        package_type,
        u16::try_from(parse_hex(&behavior_hex)).expect("PKDT behavior flags fit u16"),
        0,
    ));
}

#[given(regex = r"^a 12-byte PKDT package 0x([0-9a-fA-F]+) with type (\d+)$")]
async fn given_lane_b_pkdt_twelve(world: &mut BevyoutWorld, form_hex: String, package_type: u8) {
    world
        .package_pkdt_fixtures
        .push((parse_hex(&form_hex), 12, package_type, 0, 0));
}

#[given(
    regex = r#"^package 0x([0-9a-fA-F]+) has idle collection flags 0x([0-9a-fA-F]+) timer ([\d.]+) seconds animations \"([^\"]*)\"$"#
)]
async fn given_lane_b_idle_collection(
    world: &mut BevyoutWorld,
    form_hex: String,
    flags_hex: String,
    timer_seconds: f32,
    animations: String,
) {
    world.package_pkdt_collections.insert(
        parse_hex(&form_hex),
        package_catalog::PreparedPackageIdleCollection {
            flags: u8::try_from(parse_hex(&flags_hex)).expect("IDLF flags fit u8"),
            timer_seconds,
            animation_form_ids: parse_hex_list(&animations),
        },
    );
}

fn lane_b_package_input(record: &openmw_esm4::PackageRecord) -> package_catalog::PackageInput {
    package_catalog::PackageInput {
        form_id: record.form_id,
        editor_id: record.editor_id.clone(),
        general_flags: record.general_flags,
        package_type: record.package_type,
        location: record
            .location
            .map(|location| package_catalog::PackageLocationInput {
                location_type: location.location_type,
                form_id: location.form_id,
                raw_value: location.raw_value,
                radius: location.radius,
            }),
        schedule: record
            .schedule
            .map(|schedule| package_catalog::PackageScheduleInput {
                month: schedule.month,
                day_of_week: schedule.day_of_week,
                date: schedule.date,
                time: schedule.time,
                duration: schedule.duration,
            }),
        target: record
            .target
            .map(|target| package_catalog::PackageTargetInput {
                target_type: target.target_type,
                form_id: target.form_id,
                raw_value: target.raw_value,
                count_or_distance: target.count_or_distance,
            }),
        idle_collection: record.idle_collection.as_ref().map(|collection| {
            package_catalog::PreparedPackageIdleCollection {
                flags: collection.flags,
                timer_seconds: collection.timer_seconds,
                animation_form_ids: collection.animation_form_ids.clone(),
            }
        }),
        conditions: record.conditions.clone(),
        unsupported_subrecords: record.ignored_subrecords.clone(),
        diagnostics: record.diagnostics.clone(),
    }
}

#[when("the PKDT package fixtures are parsed and cataloged")]
async fn when_lane_b_pkdt_fixtures_are_cataloged(world: &mut BevyoutWorld) {
    let mut plugin = nav_tes4(&[]);
    let cell_data = [
        nav_subrecord(b"EDID", b"LaneBPackageCell\0"),
        nav_subrecord(b"DATA", &[1]),
    ]
    .concat();
    plugin.extend(nav_record(b"CELL", 0, 1, &cell_data));
    for (form_id, length, package_type, behavior_flags, general_flags) in
        &world.package_pkdt_fixtures
    {
        let mut data = nav_subrecord(b"EDID", format!("LaneBPackage{form_id:08x}\0").as_bytes());
        data.extend(nav_subrecord(
            b"PKDT",
            &lane_b_pkdt_payload(*length, *package_type, *behavior_flags, *general_flags),
        ));
        if let Some(collection) = world.package_pkdt_collections.get(form_id) {
            data.extend(nav_subrecord(b"IDLF", &[collection.flags]));
            data.extend(nav_subrecord(
                b"IDLC",
                &[collection.animation_form_ids.len() as u8],
            ));
            data.extend(nav_subrecord(
                b"IDLT",
                &collection.timer_seconds.to_le_bytes(),
            ));
            let idla = collection
                .animation_form_ids
                .iter()
                .flat_map(|form_id| form_id.to_le_bytes())
                .collect::<Vec<_>>();
            data.extend(nav_subrecord(b"IDLA", &idla));
        }
        plugin.extend(nav_record(b"PACK", 0, *form_id, &data));
    }
    let parsed = openmw_esm4::parse_content_set(
        &[openmw_esm4::PluginSource {
            name: "LaneBPackageTests.esm",
            bytes: &plugin,
        }],
        &CellSelector::FormId(1),
    )
    .expect("synthetic package fixtures parse");
    let known_form_ids = parsed
        .bases
        .keys()
        .copied()
        .chain(parsed.references.iter().map(|reference| reference.form_id))
        .collect();
    let packages = parsed
        .packages
        .values()
        .map(|record| (record.form_id, lane_b_package_input(record)))
        .collect();
    world.package_catalog_result = Some(package_catalog::build_package_catalog(
        &package_catalog::PackageCatalogInputs {
            packages,
            known_form_ids,
        },
        "lane-b-fixture",
    ));
}

#[then(regex = r"^package 0x([0-9a-fA-F]+) has catalog package type (\d+)$")]
async fn then_lane_b_catalog_package_type(
    world: &mut BevyoutWorld,
    form_hex: String,
    expected: u8,
) {
    assert_eq!(
        package_catalog_entry(world, &form_hex).package_type,
        expected
    );
}

#[then(
    regex = r#"^package 0x([0-9a-fA-F]+) has idle collection flags 0x([0-9a-fA-F]+) timer ([\d.]+) seconds animations \"([^\"]*)\"$"#
)]
async fn then_lane_b_idle_collection(
    world: &mut BevyoutWorld,
    form_hex: String,
    flags_hex: String,
    timer_seconds: f32,
    animations: String,
) {
    let collection = package_catalog_entry(world, &form_hex)
        .idle_collection
        .as_ref()
        .expect("package idle collection");
    assert_eq!(collection.flags, parse_hex(&flags_hex) as u8);
    assert!((collection.timer_seconds - timer_seconds).abs() < f32::EPSILON);
    assert_eq!(collection.animation_form_ids, parse_hex_list(&animations));
}

#[given(regex = r#"^actor package links \"([^\"]*)\"$"#)]
async fn given_lane_b_actor_package_links(world: &mut BevyoutWorld, packages: String) {
    world
        .package_point_actor_packages
        .push(parse_hex_list(&packages));
}

#[given(regex = r"^package 0x([0-9a-fA-F]+) has a Near Reference point 0x([0-9a-fA-F]+)$")]
async fn given_lane_b_near_reference_point(
    world: &mut BevyoutWorld,
    package_hex: String,
    reference_hex: String,
) {
    let package_form_id = parse_hex(&package_hex);
    let reference_form_id = parse_hex(&reference_hex);
    let package = world
        .package_point_packages
        .entry(package_form_id)
        .or_insert_with(|| package_catalog::PackageInput {
            form_id: package_form_id,
            ..Default::default()
        });
    package.location = Some(package_catalog::PackageLocationInput {
        location_type: 0,
        form_id: Some(reference_form_id),
        raw_value: reference_form_id,
        radius: 0,
    });
}

#[given(regex = r"^package 0x([0-9a-fA-F]+) has a Specific Reference target 0x([0-9a-fA-F]+)$")]
async fn given_lane_b_specific_reference_target(
    world: &mut BevyoutWorld,
    package_hex: String,
    reference_hex: String,
) {
    let package_form_id = parse_hex(&package_hex);
    let reference_form_id = parse_hex(&reference_hex);
    let package = world
        .package_point_packages
        .entry(package_form_id)
        .or_insert_with(|| package_catalog::PackageInput {
            form_id: package_form_id,
            ..Default::default()
        });
    package.target = Some(package_catalog::PackageTargetInput {
        target_type: 0,
        form_id: Some(reference_form_id),
        raw_value: reference_form_id,
        count_or_distance: 0,
    });
}

#[given(
    regex = r"^a current-cell editor marker 0x([0-9a-fA-F]+) at ([\d.-]+) ([\d.-]+) ([\d.-]+)$"
)]
async fn given_lane_b_editor_marker(
    world: &mut BevyoutWorld,
    reference_hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world
        .package_point_references
        .push(package_catalog::PackagePointReference {
            reference_form_id: parse_hex(&reference_hex),
            position: [x, y, z],
            is_editor_marker: true,
        });
}

#[given(
    regex = r"^an unrelated current-cell editor marker 0x([0-9a-fA-F]+) at ([\d.-]+) ([\d.-]+) ([\d.-]+)$"
)]
async fn given_lane_b_unrelated_editor_marker(
    world: &mut BevyoutWorld,
    reference_hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    world
        .package_point_references
        .push(package_catalog::PackagePointReference {
            reference_form_id: parse_hex(&reference_hex),
            position: [x, y, z],
            is_editor_marker: true,
        });
}

#[when("package-linked marker points are retained")]
async fn when_lane_b_package_points_retained(world: &mut BevyoutWorld) {
    world.package_point_result = package_catalog::retain_package_marker_points(
        &world.package_point_actor_packages,
        &world.package_point_packages,
        &world.package_point_references,
    );
}

#[then(
    regex = r"^package marker 0x([0-9a-fA-F]+) is retained as a nonvisual point at ([\d.-]+) ([\d.-]+) ([\d.-]+)$"
)]
async fn then_lane_b_package_marker_retained(
    world: &mut BevyoutWorld,
    reference_hex: String,
    x: f32,
    y: f32,
    z: f32,
) {
    let reference_form_id = parse_hex(&reference_hex);
    let point = world
        .package_point_result
        .iter()
        .find(|point| point.reference_form_id == reference_form_id)
        .expect("package marker should be retained");
    assert!(point.is_editor_marker);
    assert_eq!(point.position, [x, y, z]);
}

#[then(regex = r"^unrelated editor marker 0x([0-9a-fA-F]+) is omitted$")]
async fn then_lane_b_unrelated_marker_omitted(world: &mut BevyoutWorld, reference_hex: String) {
    assert!(
        !world
            .package_point_result
            .iter()
            .any(|point| point.reference_form_id == parse_hex(&reference_hex))
    );
}

// ---------------------------------------------------------------------
// actor_animation_catalog.feature / actor_animation_conversion.feature
// (M4 Craterside Lane C): authored IDLE preparation. Append-only section.
// ---------------------------------------------------------------------

fn lane_c_parse_ids(value: &str) -> Vec<u32> {
    value
        .split(',')
        .filter(|part| !part.trim().is_empty())
        .map(|part| parse_hex(part.trim().trim_start_matches("0x")))
        .collect()
}

fn lane_c_idle_definition(form_id: u32) -> actor_animation::PreparedActorIdleDefinition {
    actor_animation::PreparedActorIdleDefinition {
        form_id,
        ..Default::default()
    }
}

fn lane_c_idle_catalog(world: &BevyoutWorld) -> actor_animation::PreparedActorAnimationCatalog {
    world.actor_animation_catalog.clone().unwrap_or_else(|| {
        actor_animation::build_actor_animation_catalog(
            "actor-animations-v4-authored-idle-definitions",
            "lane-c-fixture",
            &world.actor_animation_discovery_inputs,
            &world.actor_animation_assets,
        )
    })
}

fn lane_c_prepare_idle_catalog(world: &mut BevyoutWorld) {
    let mut catalog = lane_c_idle_catalog(world);
    actor_animation::attach_actor_idle_definitions(
        &mut catalog,
        world.authored_idle_definitions.clone(),
    );
    world.authored_idle_definitions = catalog.idle_definitions.clone();
    world.authored_idle_diagnostics.extend(
        catalog
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.clone()),
    );
    world.authored_idle_revision = Some(catalog.revision.clone());
    world.actor_animation_catalog = Some(catalog);
}

#[given(
    regex = r"^an authored IDLE winner set with override 0x([0-9a-fA-F]+) and deleted 0x([0-9a-fA-F]+)$"
)]
async fn given_lane_c_idle_winners(
    world: &mut BevyoutWorld,
    override_hex: String,
    deleted_hex: String,
) {
    world.authored_idle_definitions = vec![
        lane_c_idle_definition(0x10),
        lane_c_idle_definition(parse_hex(&override_hex)),
    ];
    assert!(
        !world
            .authored_idle_definitions
            .iter()
            .any(|definition| definition.form_id == parse_hex(&deleted_hex))
    );
}

#[when("authored IDLE definitions are prepared")]
async fn when_lane_c_idle_definitions_prepared(world: &mut BevyoutWorld) {
    lane_c_prepare_idle_catalog(world);
}

#[then(regex = r#"^prepared authored IDLE FormIDs are \"([^\"]*)\"$"#)]
async fn then_lane_c_idle_form_ids(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .authored_idle_definitions
        .iter()
        .map(|definition| format!("0x{:08x}", definition.form_id))
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[given(regex = r"^an authored IDLE folder root 0x([0-9a-fA-F]+) without a KF path$")]
async fn given_lane_c_idle_root(world: &mut BevyoutWorld, form_hex: String) {
    world.authored_idle_definitions = vec![lane_c_idle_definition(parse_hex(&form_hex))];
}

#[then(regex = r#"^authored IDLE 0x([0-9a-fA-F]+) has no clip$"#)]
async fn then_lane_c_idle_has_no_clip(world: &mut BevyoutWorld, form_hex: String) {
    let definition = world
        .authored_idle_definitions
        .iter()
        .find(|definition| definition.form_id == parse_hex(&form_hex))
        .expect("authored IDLE definition");
    assert!(definition.source_kf_path.is_none());
    assert!(definition.clip_name.is_none());
}

#[given(regex = r#"^an authored IDLE 0x([0-9a-fA-F]+) references KF \"([^\"]*)\"$"#)]
async fn given_lane_c_idle_kf(world: &mut BevyoutWorld, form_hex: String, path: String) {
    let mut definition = lane_c_idle_definition(parse_hex(&form_hex));
    definition.source_kf_path = Some(path);
    world.authored_idle_definitions.push(definition);
}

#[when("the prepared actor animation catalog is built with authored IDLE definitions")]
async fn when_lane_c_catalog_with_idles(world: &mut BevyoutWorld) {
    let mut catalog = actor_animation::build_actor_animation_catalog(
        "actor-animations-v4-authored-idle-definitions",
        "lane-c-fixture",
        &world.actor_animation_discovery_inputs,
        &world.actor_animation_assets,
    );
    actor_animation::attach_actor_idle_definitions(
        &mut catalog,
        world.authored_idle_definitions.clone(),
    );
    world.authored_idle_definitions = catalog.idle_definitions.clone();
    world.actor_animation_catalog = Some(catalog);
}

#[then(regex = r#"^authored IDLE 0x([0-9a-fA-F]+) resolves clip \"([^\"]*)\"$"#)]
async fn then_lane_c_idle_clip(world: &mut BevyoutWorld, form_hex: String, expected: String) {
    let definition = world
        .authored_idle_definitions
        .iter()
        .find(|definition| definition.form_id == parse_hex(&form_hex))
        .expect("authored IDLE definition");
    assert_eq!(definition.clip_name.as_deref(), Some(expected.as_str()));
}

#[given(
    regex = r#"^an authored IDLE child 0x([0-9a-fA-F]+) has parent 0x([0-9a-fA-F]+) and previous sibling 0x([0-9a-fA-F]+)$"#
)]
async fn given_lane_c_idle_links(
    world: &mut BevyoutWorld,
    child_hex: String,
    parent_hex: String,
    previous_hex: String,
) {
    let mut definition = lane_c_idle_definition(parse_hex(&child_hex));
    definition.parent_form_id = Some(parse_hex(&parent_hex));
    definition.previous_sibling_form_id = Some(parse_hex(&previous_hex));
    world.authored_idle_definitions.push(definition);
}

#[then(
    regex = r#"^authored IDLE 0x([0-9a-fA-F]+) keeps parent 0x([0-9a-fA-F]+) and previous sibling 0x([0-9a-fA-F]+)$"#
)]
async fn then_lane_c_idle_links(
    world: &mut BevyoutWorld,
    child_hex: String,
    parent_hex: String,
    previous_hex: String,
) {
    let definition = world
        .authored_idle_definitions
        .iter()
        .find(|definition| definition.form_id == parse_hex(&child_hex))
        .expect("authored IDLE definition");
    assert_eq!(definition.parent_form_id, Some(parse_hex(&parent_hex)));
    assert_eq!(
        definition.previous_sibling_form_id,
        Some(parse_hex(&previous_hex))
    );
}

#[given(regex = r#"^authored IDLE siblings are supplied in FormID order \"([^\"]*)\"$"#)]
async fn given_lane_c_idle_siblings(world: &mut BevyoutWorld, ids: String) {
    let ids = lane_c_parse_ids(&ids);
    let mut authored_order = ids.clone();
    authored_order.sort_unstable();
    world.authored_idle_definitions = ids
        .iter()
        .map(|form_id| {
            let mut definition = lane_c_idle_definition(*form_id);
            definition.parent_form_id = Some(0x100);
            definition.previous_sibling_form_id = authored_order
                .iter()
                .position(|candidate| candidate == form_id)
                .and_then(|index| index.checked_sub(1))
                .and_then(|previous| authored_order.get(previous).copied());
            definition
        })
        .collect();
}

#[when("the authored IDLE sibling order is reconstructed")]
async fn when_lane_c_idle_order(world: &mut BevyoutWorld) {
    world.authored_idle_order = Some(actor_animation::reconstruct_idle_sibling_order(
        &world.authored_idle_definitions,
    ));
}

#[then(regex = r#"^authored IDLE sibling order is \"([^\"]*)\"$"#)]
async fn then_lane_c_idle_order(world: &mut BevyoutWorld, expected: String) {
    let order = world
        .authored_idle_order
        .as_ref()
        .expect("authored IDLE order");
    let actual = order
        .children_by_parent
        .values()
        .flatten()
        .map(|form_id| format!("0x{:08x}", form_id))
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[given(regex = r#"^authored IDLE raw group bytes \"([^\"]*)\"$"#)]
async fn given_lane_c_idle_groups(world: &mut BevyoutWorld, groups: String) {
    world.authored_idle_definitions = lane_c_parse_ids(&groups)
        .into_iter()
        .enumerate()
        .map(
            |(index, raw)| actor_animation::PreparedActorIdleDefinition {
                form_id: index as u32 + 1,
                group_section_raw: raw as u8,
                group_section: actor_animation::canonical_idle_group_section(raw as u8),
                ..Default::default()
            },
        )
        .collect();
}

#[then(regex = r#"^authored IDLE canonical group sections are \"([^\"]*)\"$"#)]
async fn then_lane_c_idle_groups(world: &mut BevyoutWorld, expected: String) {
    let actual = world
        .authored_idle_definitions
        .iter()
        .map(|definition| definition.group_section.to_string())
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
    assert_eq!(
        world
            .authored_idle_definitions
            .iter()
            .map(|definition| definition.group_section_raw)
            .collect::<Vec<_>>(),
        [0x47, 0x87, 0x54]
    );
}

#[given("an authored IDLE has truncated DATA and an unknown field")]
async fn given_lane_c_idle_diagnostics(world: &mut BevyoutWorld) {
    world.authored_idle_definitions = vec![lane_c_idle_definition(1)];
    world.authored_idle_diagnostics = vec![
        "IDLE 00000001 DATA malformed: expected 6 or 8 bytes, got 3".into(),
        "IDLE 00000001 ignored unsupported WHAT subrecord".into(),
    ];
}

#[then(regex = r#"^authored IDLE preparation has diagnostic \"([^\"]*)\"$"#)]
async fn then_lane_c_idle_diagnostic(world: &mut BevyoutWorld, expected: String) {
    assert!(
        world
            .authored_idle_diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains(&expected))
    );
}

#[given(regex = r#"^an authored IDLE has CTDA payloads \"([^\"]*)\"$"#)]
async fn given_lane_c_idle_conditions(world: &mut BevyoutWorld, payloads: String) {
    let mut definition = lane_c_idle_definition(1);
    definition.conditions = payloads
        .split(',')
        .map(|payload| {
            (0..payload.len())
                .step_by(2)
                .map(|offset| u8::from_str_radix(&payload[offset..offset + 2], 16).unwrap())
                .collect()
        })
        .collect();
    world.authored_idle_definitions = vec![definition];
}

#[then(regex = r#"^authored IDLE CTDA payloads remain \"([^\"]*)\"$"#)]
async fn then_lane_c_idle_conditions(world: &mut BevyoutWorld, expected: String) {
    let actual = world.authored_idle_definitions[0]
        .conditions
        .iter()
        .map(|payload| {
            payload
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(actual, expected);
}

#[given(regex = r#"^authored IDLE definitions have FormIDs \"([^\"]*)\"$"#)]
async fn given_lane_c_idle_ids(world: &mut BevyoutWorld, ids: String) {
    world.authored_idle_definitions = lane_c_parse_ids(&ids)
        .into_iter()
        .map(lane_c_idle_definition)
        .collect();
}

#[when("authored IDLE definitions are prepared twice")]
async fn when_lane_c_idle_twice(world: &mut BevyoutWorld) {
    let mut first = lane_c_idle_catalog(world);
    actor_animation::attach_actor_idle_definitions(
        &mut first,
        world.authored_idle_definitions.clone(),
    );
    let first_serialized = ron::to_string(&first).expect("first authored IDLE catalog serializes");
    let mut second = lane_c_idle_catalog(world);
    actor_animation::attach_actor_idle_definitions(
        &mut second,
        world.authored_idle_definitions.clone(),
    );
    let second_serialized =
        ron::to_string(&second).expect("second authored IDLE catalog serializes");
    world.authored_idle_definitions = first.idle_definitions.clone();
    world.authored_idle_hash = Some(paths::fingerprint(first_serialized.as_bytes()));
    assert_eq!(
        world.authored_idle_hash.as_deref(),
        Some(paths::fingerprint(second_serialized.as_bytes()).as_str())
    );
}

#[then("authored IDLE catalog ordering and hash match")]
async fn then_lane_c_idle_hash(world: &mut BevyoutWorld) {
    let ids = world
        .authored_idle_definitions
        .iter()
        .map(|definition| definition.form_id)
        .collect::<Vec<_>>();
    assert_eq!(ids, [0x10, 0x20]);
    assert!(world.authored_idle_hash.is_some());
}

#[given(regex = r#"^an animation set contains one KF \"([^\"]*)\"$"#)]
async fn given_lane_c_conversion_set(world: &mut BevyoutWorld, path: String) {
    world.actor_animation_discovery_inputs = vec![actor_animation::ActorAnimationDiscoveryInput {
        reference_form_id: 1,
        base_form_id: 2,
        model_path: "meshes/characters/_male/skeleton.nif".into(),
        skeleton_path: "meshes/characters/_male/skeleton.nif".into(),
        skeleton_fingerprint: "skeleton".into(),
        explicit_kf_paths: vec![path.clone()],
        ..Default::default()
    }];
    world.actor_animation_assets = vec![actor_animation::ActorAnimationAsset {
        path,
        fingerprint: "idle".into(),
        state: actor_animation::ActorAnimationAssetState::Compatible,
    }];
}

#[given(regex = r#"^an authored IDLE references the existing KF \"([^\"]*)\"$"#)]
async fn given_lane_c_conversion_idle(world: &mut BevyoutWorld, path: String) {
    world.authored_idle_definitions = vec![actor_animation::PreparedActorIdleDefinition {
        form_id: 1,
        source_kf_path: Some(path),
        ..Default::default()
    }];
}

#[when("authored IDLE conversion is staged")]
async fn when_lane_c_conversion_staged(world: &mut BevyoutWorld) {
    let mut catalog = actor_animation::build_actor_animation_catalog(
        "actor-animations-v4-authored-idle-definitions",
        "lane-c-fixture",
        &world.actor_animation_discovery_inputs,
        &world.actor_animation_assets,
    );
    actor_animation::attach_actor_idle_definitions(
        &mut catalog,
        world.authored_idle_definitions.clone(),
    );
    world.authored_idle_conversion_set_count = catalog.animation_sets.len();
    world.authored_idle_conversion_clip_count = catalog
        .animation_sets
        .iter()
        .flat_map(|set| set.clips.iter())
        .filter(|clip| clip.status == actor_animation::PreparedActorAnimationClipStatus::Ready)
        .count();
    world.authored_idle_definitions = catalog.idle_definitions;
}

#[then(regex = r"^authored IDLE conversion uses (\d+) animation set and (\d+) clip$")]
async fn then_lane_c_conversion_counts(world: &mut BevyoutWorld, sets: usize, clips: usize) {
    assert_eq!(world.authored_idle_conversion_set_count, sets);
    assert_eq!(world.authored_idle_conversion_clip_count, clips);
}

#[then("authored IDLE conversion invokes no duplicate pack job")]
async fn then_lane_c_no_duplicate_pack_job(world: &mut BevyoutWorld) {
    assert_eq!(world.authored_idle_conversion_set_count, 1);
    assert_eq!(world.authored_idle_conversion_clip_count, 1);
}

#[given("the authored IDLE catalog revision is inspected")]
async fn given_lane_c_revision(world: &mut BevyoutWorld) {
    world.authored_idle_revision = Some("actor-animations-v4-authored-idle-definitions".into());
}

#[then(regex = r#"^the authored IDLE catalog revision is \"([^\"]*)\"$"#)]
async fn then_lane_c_revision(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.authored_idle_revision.as_deref(),
        Some(expected.as_str())
    );
}

// ---------------------------------------------------------------------
// actor_animation_gameflow.feature Lane D idle authority/selection seam.
// ---------------------------------------------------------------------

#[derive(Default)]
struct FeatureIdleConditions {
    unsupported: bool,
    false_condition: bool,
}

impl actor_idle_policy::IdleConditionEvaluator for FeatureIdleConditions {
    fn evaluate(
        &self,
        conditions: &[Vec<u8>],
        _random_percent: u8,
        _facts: &actor_idle_policy::IdleRuntimeFacts,
    ) -> actor_idle_policy::IdleConditionOutcome {
        if self.unsupported && !conditions.is_empty() {
            actor_idle_policy::IdleConditionOutcome::Unevaluable
        } else if self.false_condition && !conditions.is_empty() {
            actor_idle_policy::IdleConditionOutcome::False
        } else {
            actor_idle_policy::IdleConditionOutcome::True
        }
    }
}

fn feature_idle_definition(form_id: u32) -> actor_animation::PreparedActorIdleDefinition {
    actor_animation::PreparedActorIdleDefinition {
        form_id,
        clip_name: Some(format!("idle_{form_id:08x}")),
        group_section_raw: actor_idle_policy::SPECIAL_IDLE_GROUP,
        group_section: actor_idle_policy::SPECIAL_IDLE_GROUP,
        ..Default::default()
    }
}

fn feature_idle_evaluator(world: &BevyoutWorld) -> FeatureIdleConditions {
    FeatureIdleConditions {
        unsupported: world
            .idle_global_definitions
            .iter()
            .any(|definition| definition.conditions == vec![vec![0xff]]),
        false_condition: world
            .idle_global_definitions
            .iter()
            .any(|definition| definition.conditions == vec![vec![0xfe]]),
    }
}

fn feature_idle_select(
    world: &mut BevyoutWorld,
    now_seconds: f32,
    trigger: actor_idle_policy::IdleEvaluationTrigger,
) {
    let evaluator = feature_idle_evaluator(world);
    world.idle_decision = Some(world.idle_authority.select(
        0x0000_1234,
        now_seconds,
        world.idle_lifecycle,
        &actor_idle_policy::IdleRuntimeFacts::default(),
        world.idle_package.as_ref(),
        &world.idle_global_definitions,
        trigger,
        &evaluator,
    ));
    if let Some(actor_idle_policy::IdleDecision::Selected(selection)) = &world.idle_decision {
        world.idle_selected = Some(selection.clone());
    }
}

#[given("a stationary alive loaded actor with no equipment transition")]
async fn given_idle_stationary_actor(world: &mut BevyoutWorld) {
    world.idle_lifecycle = actor_idle_policy::IdleLifecycleFacts {
        alive: true,
        loaded: true,
        ..Default::default()
    };
}

#[given("the actor lifecycle is moving, dead, ragdolled, unloaded, or equipment-transitioning")]
async fn given_idle_invalid_lifecycle(world: &mut BevyoutWorld) {
    world.idle_lifecycle.moving = true;
    world.idle_lifecycle.alive = false;
    world.idle_lifecycle.ragdolled = true;
    world.idle_lifecycle.loaded = false;
    world.idle_lifecycle.equipment_transition = true;
    world.idle_global_definitions = vec![feature_idle_definition(1)];
}

#[when("special idle eligibility is evaluated")]
async fn when_idle_eligibility(world: &mut BevyoutWorld) {
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
}

#[then(regex = r#"^special idle selection is rejected with reason \"([^\"]*)\"$"#)]
async fn then_idle_rejected(world: &mut BevyoutWorld, expected: String) {
    let actual = match &world.idle_decision {
        Some(actor_idle_policy::IdleDecision::Rejected(reason)) => *reason,
        other => panic!("expected idle rejection, got {other:?}"),
    };
    if expected == "invalid_lifecycle" {
        assert!(matches!(
            actual,
            actor_idle_policy::IdleRejectionReason::Moving
                | actor_idle_policy::IdleRejectionReason::Dead
                | actor_idle_policy::IdleRejectionReason::Ragdolled
                | actor_idle_policy::IdleRejectionReason::Unloaded
                | actor_idle_policy::IdleRejectionReason::EquipmentTransition
        ));
    } else {
        assert_eq!(
            actual,
            actor_idle_policy::IdleRejectionReason::from_label(&expected)
                .unwrap_or_else(|| panic!("unknown idle rejection {expected}")),
        );
    }
}

#[then(regex = r#"^the second special idle selection is rejected with reason \"([^\"]*)\"$"#)]
async fn then_second_idle_rejected(world: &mut BevyoutWorld, expected: String) {
    then_idle_rejected(world, expected).await;
}

#[given(regex = r#"^an active package with No idle anims and a global authored idle$"#)]
async fn given_idle_no_idle_package(world: &mut BevyoutWorld) {
    world.idle_package = Some(actor_idle_policy::IdlePackageContext {
        form_id: 2,
        general_flags: actor_idle_policy::NO_IDLE_ANIMS_FLAG,
        collection: None,
    });
    world.idle_global_definitions = vec![feature_idle_definition(1)];
}

#[when("automatic special idle selection is evaluated")]
async fn when_automatic_idle_selection(world: &mut BevyoutWorld) {
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
}

#[given(
    regex = r#"^an active package idle collection \"([^\"]*)\" and a global authored idle \"([^\"]*)\"$"#
)]
async fn given_idle_package_over_global(
    world: &mut BevyoutWorld,
    package_id: String,
    global_id: String,
) {
    let package_id = parse_hex(package_id.trim_start_matches("0x"));
    let global_id = parse_hex(global_id.trim_start_matches("0x"));
    world.idle_package = Some(actor_idle_policy::IdlePackageContext {
        form_id: 2,
        general_flags: 0,
        collection: Some(actor_idle_policy::IdlePackageCollection {
            flags: actor_idle_policy::RUN_IN_SEQUENCE_FLAG,
            timer_seconds: 0.0,
            animation_form_ids: vec![package_id],
        }),
    });
    world.idle_global_definitions = vec![
        feature_idle_definition(package_id),
        feature_idle_definition(global_id),
    ];
    world
        .idle_authority
        .on_package_entry(Some(2), 0.0, 0.0, true);
}

#[then(regex = r#"^the selected special idle is \"([^\"]*)\" from source \"([^\"]*)\"$"#)]
async fn then_idle_selected(world: &mut BevyoutWorld, form_id: String, source: String) {
    let selected = world.idle_selected.as_ref().expect("selected idle");
    assert_eq!(
        selected.form_id,
        parse_hex(form_id.trim_start_matches("0x"))
    );
    assert_eq!(selected.source.label(), source);
}

#[given(regex = r#"^a sequence do-once package idle collection \"([^\"]*)\"$"#)]
async fn given_idle_sequence_collection(world: &mut BevyoutWorld, ids: String) {
    let ids = ids
        .split(',')
        .map(|id| parse_hex(id.trim_start_matches("0x")))
        .collect::<Vec<_>>();
    world.idle_package = Some(actor_idle_policy::IdlePackageContext {
        form_id: 2,
        general_flags: 0,
        collection: Some(actor_idle_policy::IdlePackageCollection {
            flags: actor_idle_policy::RUN_IN_SEQUENCE_FLAG | actor_idle_policy::DO_ONCE_FLAG,
            timer_seconds: 0.0,
            animation_form_ids: ids.clone(),
        }),
    });
    world.idle_global_definitions = ids.into_iter().map(feature_idle_definition).collect();
    world
        .idle_authority
        .on_package_entry(Some(2), 0.0, 0.0, true);
}

#[when("the package idle collection is evaluated twice")]
async fn when_idle_collection_twice(world: &mut BevyoutWorld) {
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
    let first = world.idle_selected.clone().expect("first idle");
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
    let second = world.idle_selected.clone().expect("second idle");
    world.idle_same_epoch_selection = Some((
        actor_idle_policy::IdleDecision::Selected(first),
        actor_idle_policy::IdleDecision::Selected(second),
    ));
}

#[then(regex = r#"^the selected special idles are \"([^\"]*)\"$"#)]
async fn then_idle_collection_values(world: &mut BevyoutWorld, expected: String) {
    let actual = match world.idle_same_epoch_selection.as_ref() {
        Some((
            actor_idle_policy::IdleDecision::Selected(first),
            actor_idle_policy::IdleDecision::Selected(second),
        )) => {
            format!("0x{:08x},0x{:08x}", first.form_id, second.form_id)
        }
        other => panic!("expected two idle selections, got {other:?}"),
    };
    assert_eq!(actual, expected);
}

#[then("the package idle collection is exhausted")]
async fn then_idle_collection_exhausted(world: &mut BevyoutWorld) {
    assert!(world.idle_authority.do_once_exhausted);
}

#[given(regex = r#"^a random package idle collection \"([^\"]*)\"$"#)]
async fn given_idle_random_collection(world: &mut BevyoutWorld, ids: String) {
    let ids = ids
        .split(',')
        .map(|id| parse_hex(id.trim_start_matches("0x")))
        .collect::<Vec<_>>();
    world.idle_package = Some(actor_idle_policy::IdlePackageContext {
        form_id: 2,
        general_flags: 0,
        collection: Some(actor_idle_policy::IdlePackageCollection {
            flags: 0,
            timer_seconds: 0.0,
            animation_form_ids: ids.clone(),
        }),
    });
    world.idle_global_definitions = ids.into_iter().map(feature_idle_definition).collect();
    world
        .idle_authority
        .on_package_entry(Some(2), 0.0, 0.0, true);
}

#[when("the same package idle selection is evaluated twice with the same epoch")]
async fn when_idle_random_same_epoch(world: &mut BevyoutWorld) {
    let evaluator = feature_idle_evaluator(world);
    let package = world.idle_package.clone();
    let definitions = world.idle_global_definitions.clone();
    let mut left = actor_idle_policy::IdleAuthority::default();
    let mut right = actor_idle_policy::IdleAuthority::default();
    left.selection_epoch = 7;
    right.selection_epoch = 7;
    let left_result = left.select(
        0x1234,
        0.0,
        world.idle_lifecycle,
        &Default::default(),
        package.as_ref(),
        &definitions,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
        &evaluator,
    );
    let right_result = right.select(
        0x1234,
        0.0,
        world.idle_lifecycle,
        &Default::default(),
        package.as_ref(),
        &definitions,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
        &evaluator,
    );
    world.idle_same_epoch_selection = Some((left_result, right_result));
}

#[then("the random package idle selections match")]
async fn then_idle_random_match(world: &mut BevyoutWorld) {
    assert_eq!(
        world.idle_same_epoch_selection.as_ref().unwrap().0,
        world.idle_same_epoch_selection.as_ref().unwrap().1
    );
}

#[given(regex = r"^an active package idle timer of ([\d.]+) seconds$")]
async fn given_idle_timer(world: &mut BevyoutWorld, seconds: f32) {
    world.idle_package = Some(actor_idle_policy::IdlePackageContext {
        form_id: 2,
        general_flags: 0,
        collection: Some(actor_idle_policy::IdlePackageCollection {
            flags: actor_idle_policy::RUN_IN_SEQUENCE_FLAG,
            timer_seconds: seconds,
            animation_form_ids: vec![1],
        }),
    });
    world.idle_global_definitions = vec![feature_idle_definition(1)];
    world
        .idle_authority
        .on_package_entry(Some(2), 0.0, seconds, true);
}

#[when(regex = r"^package idle selection is evaluated at ([\d.]+) seconds$")]
async fn when_idle_at_time(world: &mut BevyoutWorld, seconds: f32) {
    feature_idle_select(
        world,
        seconds,
        actor_idle_policy::IdleEvaluationTrigger::PackageTimer,
    );
}

#[then("package idle selection is eligible")]
async fn then_idle_package_eligible(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.idle_decision,
        Some(actor_idle_policy::IdleDecision::Selected(_))
    ));
}

#[given(
    regex = r#"^a global authored idle tree with parent conditions and siblings \"([^\"]*)\"$"#
)]
async fn given_idle_global_tree(world: &mut BevyoutWorld, ids: String) {
    let ids = ids
        .split(',')
        .map(|id| parse_hex(id.trim_start_matches("0x")))
        .collect::<Vec<_>>();
    let mut parent = feature_idle_definition(0x10);
    parent.clip_name = None;
    let mut first = feature_idle_definition(ids[0]);
    first.parent_form_id = Some(parent.form_id);
    first.conditions = vec![vec![0xfe]];
    let mut second = feature_idle_definition(ids[1]);
    second.parent_form_id = Some(parent.form_id);
    second.previous_sibling_form_id = Some(first.form_id);
    world.idle_global_definitions = vec![second, parent, first];
}

#[when("global special idle selection is evaluated")]
async fn when_idle_global(world: &mut BevyoutWorld) {
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
}

#[given("a global authored idle with an unsupported CTDA condition")]
async fn given_idle_unsupported_condition(world: &mut BevyoutWorld) {
    let mut definition = feature_idle_definition(1);
    definition.conditions = vec![vec![0xff]];
    world.idle_global_definitions = vec![definition];
}

#[given(regex = r"^a global authored idle with a (\d+) second replay delay$")]
async fn given_idle_replay_delay(world: &mut BevyoutWorld, seconds: i16) {
    let mut definition = feature_idle_definition(1);
    definition.replay_delay_seconds = seconds;
    world.idle_global_definitions = vec![definition];
}

#[when("global special idle selection is evaluated twice immediately")]
async fn when_idle_global_twice(world: &mut BevyoutWorld) {
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
}

#[given(regex = r"^an authored special idle with loop bounds (\d+) and (\d+)$")]
async fn given_idle_loop_bounds(world: &mut BevyoutWorld, min: u8, max: u8) {
    let mut definition = feature_idle_definition(1);
    definition.loop_min = min;
    definition.loop_max = max;
    world.idle_global_definitions = vec![definition];
}

#[when("the special idle loop count is selected")]
async fn when_idle_loop_count(world: &mut BevyoutWorld) {
    feature_idle_select(
        world,
        0.0,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
    );
    world.idle_loop_count = world
        .idle_selected
        .as_ref()
        .map(|selection| selection.loop_count);
}

#[then(regex = r"^the loop count is between (\d+) and (\d+) inclusive$")]
async fn then_idle_loop_range(world: &mut BevyoutWorld, min: u8, max: u8) {
    assert!((min..=max).contains(&world.idle_loop_count.expect("loop count")));
}

#[given(regex = r"^an authored idle with group section (\d+)$")]
async fn given_idle_group(world: &mut BevyoutWorld, group: u8) {
    let mut definition = feature_idle_definition(1);
    definition.group_section = group;
    definition.group_section_raw = group;
    world.idle_global_definitions = vec![definition];
}

#[when("forced special idle validation is evaluated")]
async fn when_idle_forced_validation(world: &mut BevyoutWorld) {
    let evaluator = FeatureIdleConditions::default();
    world.idle_decision = Some(world.idle_authority.force_select(
        0x1234,
        0.0,
        world.idle_lifecycle,
        &Default::default(),
        &world.idle_global_definitions,
        1,
        &evaluator,
    ));
}

#[given("a forced authored idle with a false condition and active cooldown")]
async fn given_forced_idle(world: &mut BevyoutWorld) {
    let mut definition = feature_idle_definition(1);
    definition.conditions = vec![vec![0xfe]];
    definition.replay_delay_seconds = 20;
    world.idle_global_definitions = vec![definition];
    world.idle_authority.replay_cooldowns.insert(1, 20.0);
}

#[when("forced special idle selection is evaluated")]
async fn when_forced_idle(world: &mut BevyoutWorld) {
    let evaluator = FeatureIdleConditions::default();
    world.idle_decision = Some(world.idle_authority.force_select(
        0x1234,
        0.0,
        world.idle_lifecycle,
        &Default::default(),
        &world.idle_global_definitions,
        1,
        &evaluator,
    ));
    if let Some(actor_idle_policy::IdleDecision::Selected(selection)) = &world.idle_decision {
        world.idle_selected = Some(selection.clone());
    }
}

#[then(regex = r#"^the selected special idle source is \"([^\"]*)\"$"#)]
async fn then_idle_source(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.idle_selected.as_ref().unwrap().source.label(),
        expected
    );
}

#[given("a special idle is playing for a stationary actor")]
async fn given_playing_idle(world: &mut BevyoutWorld) {
    world.idle_global_definitions = vec![feature_idle_definition(1)];
    let evaluator = FeatureIdleConditions::default();
    let decision = world.idle_authority.select(
        0x1234,
        0.0,
        world.idle_lifecycle,
        &Default::default(),
        None,
        &world.idle_global_definitions,
        actor_idle_policy::IdleEvaluationTrigger::BaseIdleLoop,
        &evaluator,
    );
    assert!(matches!(
        decision,
        actor_idle_policy::IdleDecision::Selected(_)
    ));
}

#[when("special idle playback completes or movement begins")]
async fn when_idle_interrupted(world: &mut BevyoutWorld) {
    world
        .idle_authority
        .stop(Some(actor_idle_policy::IdleRejectionReason::Moving));
}

#[then("base locomotion resumes without a static state")]
async fn then_idle_base_resumes(world: &mut BevyoutWorld) {
    assert!(world.idle_authority.current_idle_form_id.is_none());
    assert_eq!(
        world.idle_authority.source,
        actor_idle_policy::IdleSource::IdleManager
    );
}

// ---------------------------------------------------------------------
// actor_fallback.feature -- M4 static NPC FaceGen reconstruction.
// ---------------------------------------------------------------------

fn facegen_fixture_bytes(count: usize, value: f32) -> Vec<u8> {
    (0..count)
        .flat_map(|_| value.to_le_bytes())
        .collect::<Vec<_>>()
}

fn canonical_facegen_raw(value: f32) -> facegen::FaceGenRaw {
    facegen::FaceGenRaw {
        geometry_symmetric: Some(facegen_fixture_bytes(
            facegen::GEOMETRY_SYMMETRIC_COEFFICIENTS,
            value,
        )),
        geometry_asymmetric: Some(facegen_fixture_bytes(
            facegen::GEOMETRY_ASYMMETRIC_COEFFICIENTS,
            value,
        )),
        texture_symmetric: Some(facegen_fixture_bytes(
            facegen::TEXTURE_SYMMETRIC_COEFFICIENTS,
            value,
        )),
    }
}

#[given("canonical FaceGen coefficient payloads")]
async fn given_canonical_facegen_payloads(world: &mut BevyoutWorld) {
    world.facegen_feature.actor = canonical_facegen_raw(1.0);
    world.facegen_feature.race = None;
}

#[given("a FaceGen geometry payload with an unsupported length")]
async fn given_unsupported_facegen_payload(world: &mut BevyoutWorld) {
    world.facegen_feature.actor = facegen::FaceGenRaw {
        geometry_symmetric: Some(vec![0; 4]),
        ..facegen::FaceGenRaw::default()
    };
    world.facegen_feature.race = None;
}

#[given("canonical FaceGen race defaults")]
async fn given_canonical_facegen_race_defaults(world: &mut BevyoutWorld) {
    world.facegen_feature.race = Some(canonical_facegen_raw(1.0));
}

#[given("canonical FaceGen actor traits")]
async fn given_canonical_facegen_actor_traits(world: &mut BevyoutWorld) {
    world.facegen_feature.actor = canonical_facegen_raw(2.0);
}

#[when("FaceGen coefficients are decoded")]
async fn when_facegen_coefficients_are_decoded(world: &mut BevyoutWorld) {
    world.facegen_feature.result = match facegen::resolve_facegen(
        &world.facegen_feature.actor,
        world.facegen_feature.race.as_ref(),
    ) {
        Ok(Some(resolved)) => Some(Ok(resolved)),
        Ok(None) => None,
        Err(error) => Some(Err(error)),
    };
}

#[then(regex = r"^FaceGen coefficient policy is (Authored|RestPoseFallback)$")]
async fn then_facegen_coefficient_policy_is(world: &mut BevyoutWorld, expected: String) {
    let actual = match world.facegen_feature.result.as_ref() {
        Some(Ok(_)) => "Authored",
        Some(Err(_)) => "RestPoseFallback",
        None => "NotAuthored",
    };
    assert_eq!(actual, expected);
}

#[then(
    regex = r"^FaceGen (geometry symmetric|geometry asymmetric|texture symmetric) coefficient count is (\d+)$"
)]
async fn then_facegen_coefficient_count(
    world: &mut BevyoutWorld,
    component: String,
    expected: usize,
) {
    let resolved = match world.facegen_feature.result.as_ref() {
        Some(Ok(resolved)) => resolved,
        Some(Err(error)) => panic!("FaceGen coefficients failed: {error}"),
        None => panic!("FaceGen coefficients were not decoded"),
    };
    let component = match component.as_str() {
        "geometry symmetric" => facegen::FaceGenComponent::GeometrySymmetric,
        "geometry asymmetric" => facegen::FaceGenComponent::GeometryAsymmetric,
        "texture symmetric" => facegen::FaceGenComponent::TextureSymmetric,
        other => panic!("unknown FaceGen component {other}"),
    };
    assert_eq!(resolved.coefficients.component(component).len(), expected);
}

#[then(regex = r####"^FaceGen diagnostic "([^"]+)" is recorded$"####)]
async fn then_facegen_diagnostic_is_recorded(world: &mut BevyoutWorld, expected: String) {
    let diagnostic = match world.facegen_feature.result.as_ref() {
        Some(Err(diagnostic)) => diagnostic,
        Some(Ok(_)) => panic!("FaceGen decode unexpectedly succeeded"),
        None => panic!("FaceGen decode was not run"),
    };
    assert_eq!(diagnostic.code(), expected);
}

#[then(regex = r"^combined FaceGen geometry symmetric coefficient (\d+) is (-?[\d.]+)$")]
async fn then_combined_facegen_coefficient_is(
    world: &mut BevyoutWorld,
    index: usize,
    expected: f32,
) {
    let resolved = match world.facegen_feature.result.as_ref() {
        Some(Ok(resolved)) => resolved,
        Some(Err(error)) => panic!("FaceGen coefficients failed: {error}"),
        None => panic!("FaceGen coefficients were not decoded"),
    };
    let actual = resolved.coefficients.geometry_symmetric[index];
    assert!((actual - expected).abs() < 1.0e-6, "{actual} != {expected}");
}

// -- asset_materials.feature legacy world shading --

#[given(regex = r"^the master Chan strength is (-?[\d.]+)$")]
async fn given_master_chan_strength(world: &mut BevyoutWorld, strength: f32) {
    world.legacy_chan_master_strength = strength;
}

#[given(regex = r"^a legacy Fallout material with glossiness exponent (-?[\d.]+)$")]
async fn given_legacy_glossiness_exponent(world: &mut BevyoutWorld, exponent: f32) {
    world.legacy_glossiness_exponent = Some(exponent);
}

#[given("a legacy Fallout material with a non-finite glossiness exponent")]
async fn given_non_finite_legacy_glossiness_exponent(world: &mut BevyoutWorld) {
    world.legacy_glossiness_exponent = Some(f32::NAN);
}

#[when("its legacy world shading policy is evaluated")]
async fn when_legacy_world_shading_policy_is_evaluated(world: &mut BevyoutWorld) {
    let exponent = assets::sanitized_glossiness_exponent(
        world
            .legacy_glossiness_exponent
            .or(world.material_glossiness.flatten()),
    );
    world.legacy_glossiness_exponent = Some(exponent);
    world.legacy_micro_roughness = Some(assets::legacy_micro_roughness_from_glossiness(Some(
        exponent,
    )));
    world.legacy_chan_weight = Some(assets::legacy_chan_weight(
        Some(exponent),
        world.legacy_chan_master_strength,
    ));
}

#[then(regex = r"^its preserved glossiness exponent is approximately ([\d.]+)$")]
async fn then_preserved_glossiness_exponent_is(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .legacy_glossiness_exponent
        .expect("legacy glossiness exponent should be evaluated");
    assert!((actual - expected).abs() < 1.0e-6, "{actual} != {expected}");
}

#[then(regex = r"^its legacy micro-roughness is approximately ([\d.]+)$")]
async fn then_legacy_micro_roughness_is(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .legacy_micro_roughness
        .expect("legacy micro-roughness should be evaluated");
    assert!((actual - expected).abs() < 1.0e-3, "{actual} != {expected}");
}

#[then(regex = r"^its automatic Chan weight is approximately ([\d.]+)$")]
async fn then_effective_chan_weight_is(world: &mut BevyoutWorld, expected: f32) {
    let actual = world
        .legacy_chan_weight
        .expect("legacy Chan weight should be evaluated");
    assert!((actual - expected).abs() < 1.0e-3, "{actual} != {expected}");
}

// -- fallout_overlays.feature --

#[given(regex = r#"^a Fallout static named \"([^\"]+)\" using model \"([^\"]+)\"$"#)]
async fn given_fallout_overlay_candidate(
    world: &mut BevyoutWorld,
    editor_id: String,
    model: String,
) {
    world.fallout_overlay_editor_id = Some(editor_id);
    world.fallout_overlay_model = Some(model);
}

#[when("its flat-overlay policy is evaluated")]
async fn when_flat_overlay_policy_is_evaluated(world: &mut BevyoutWorld) {
    world.fallout_overlay_kind = Some(overlay_policy::classify_fallout_overlay(
        world.fallout_overlay_editor_id.as_deref(),
        world.fallout_overlay_model.as_deref(),
    ));
}

#[then(regex = r#"^its overlay kind is \"([^\"]+)\"$"#)]
async fn then_overlay_kind_is(world: &mut BevyoutWorld, expected: String) {
    let actual = match world
        .fallout_overlay_kind
        .expect("overlay policy not evaluated")
    {
        overlay_policy::FalloutOverlayKind::None => "none",
        overlay_policy::FalloutOverlayKind::Decal => "decal",
        overlay_policy::FalloutOverlayKind::Debris => "debris",
    };
    assert_eq!(actual, expected);
}

#[then("the placement is excluded from static lighting inputs")]
async fn then_overlay_is_excluded_from_static_lighting(world: &mut BevyoutWorld) {
    assert_ne!(
        world.fallout_overlay_kind,
        Some(overlay_policy::FalloutOverlayKind::None)
    );
}

// ---------------------------------------------------------------------
// cache_stats.feature -- deterministic prepared-cache accounting.
// ---------------------------------------------------------------------

#[given("a fresh prepared cache inventory")]
async fn given_fresh_prepared_cache_inventory(world: &mut BevyoutWorld) {
    world.cache_stat_files.clear();
    world.cache_stat_summary = None;
}

#[given(
    regex = r#"^cache file \"([^\"]+)\" has (\d+) logical bytes, (\d+) allocated bytes, and payload \"([^\"]+)\"$"#
)]
async fn given_cache_file(
    world: &mut BevyoutWorld,
    path: String,
    logical_bytes: u64,
    allocated_bytes: u64,
    payload: String,
) {
    world
        .cache_stat_files
        .push(cache_stats_policy::CacheFileFacts {
            relative_path: path,
            logical_bytes,
            allocated_bytes,
            payload_id: payload,
        });
}

#[when("cache storage is summarized")]
async fn when_cache_storage_is_summarized(world: &mut BevyoutWorld) {
    world.cache_stat_summary = Some(cache_stats_policy::summarize_cache_files(
        &world.cache_stat_files,
    ));
}

#[then(regex = r"^cache logical bytes are (\d+)$")]
async fn then_cache_logical_bytes(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(
        world.cache_stat_summary.as_ref().unwrap().logical_bytes,
        expected
    );
}

#[then(regex = r"^cache allocated bytes are (\d+)$")]
async fn then_cache_allocated_bytes(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(
        world.cache_stat_summary.as_ref().unwrap().allocated_bytes,
        expected
    );
}

#[then(regex = r"^cache unique payload bytes are (\d+)$")]
async fn then_cache_unique_payload_bytes(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(
        world
            .cache_stat_summary
            .as_ref()
            .unwrap()
            .unique_payload_bytes,
        expected
    );
}

#[then(regex = r"^cache duplicate logical bytes are (\d+)$")]
async fn then_cache_duplicate_logical_bytes(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(
        world
            .cache_stat_summary
            .as_ref()
            .unwrap()
            .duplicate_logical_bytes,
        expected
    );
}

#[then(regex = r"^cache duplicate physical bytes are (\d+)$")]
async fn then_cache_duplicate_physical_bytes(world: &mut BevyoutWorld, expected: u64) {
    assert_eq!(
        world
            .cache_stat_summary
            .as_ref()
            .unwrap()
            .duplicate_allocated_bytes,
        expected
    );
}

#[then(regex = r"^cache duplicate cluster count is (\d+)$")]
async fn then_cache_duplicate_cluster_count(world: &mut BevyoutWorld, expected: usize) {
    assert_eq!(
        world
            .cache_stat_summary
            .as_ref()
            .unwrap()
            .duplicate_clusters
            .len(),
        expected
    );
}

#[then(regex = r#"^cache category \"([^\"]+)\" has (\d+) logical bytes$"#)]
async fn then_cache_category_logical_bytes(
    world: &mut BevyoutWorld,
    category: String,
    expected: u64,
) {
    let actual = world
        .cache_stat_summary
        .as_ref()
        .unwrap()
        .categories
        .iter()
        .find(|entry| entry.category.as_str() == category)
        .unwrap_or_else(|| panic!("missing cache category {category}"));
    assert_eq!(actual.logical_bytes, expected);
}

#[then("cache categories are ordered alphabetically")]
async fn then_cache_categories_are_ordered(world: &mut BevyoutWorld) {
    let categories = &world.cache_stat_summary.as_ref().unwrap().categories;
    assert!(
        categories
            .windows(2)
            .all(|pair| pair[0].category <= pair[1].category)
    );
}

#[then("cache duplicate clusters are ordered by recoverable bytes then payload")]
async fn then_cache_duplicate_clusters_are_ordered(world: &mut BevyoutWorld) {
    let clusters = &world
        .cache_stat_summary
        .as_ref()
        .unwrap()
        .duplicate_clusters;
    assert!(clusters.windows(2).all(|pair| {
        pair[0].duplicate_allocated_bytes > pair[1].duplicate_allocated_bytes
            || (pair[0].duplicate_allocated_bytes == pair[1].duplicate_allocated_bytes
                && pair[0].payload_id <= pair[1].payload_id)
    }));
}

#[then("paths inside each duplicate cluster are ordered alphabetically")]
async fn then_cache_duplicate_paths_are_ordered(world: &mut BevyoutWorld) {
    for cluster in &world
        .cache_stat_summary
        .as_ref()
        .unwrap()
        .duplicate_clusters
    {
        assert!(cluster.paths.windows(2).all(|pair| pair[0] <= pair[1]));
    }
}

// ---------------------------------------------------------------------
// day_night_lighting.feature -- shared exterior weather catalog.
// ---------------------------------------------------------------------

#[given("an exterior environment with sunrise from hour 5 to 9 and no embedded weather catalog")]
async fn given_shared_weather_environment(world: &mut BevyoutWorld) {
    world.shared_weather_environment =
        bevyout_core::manifest::exterior::PreparedExteriorEnvironment {
            timings: bevyout_core::time_of_day::DayNightTimings {
                sunrise_begin_hour: 5.0,
                sunrise_end_hour: 9.0,
                sunset_begin_hour: 16.0,
                sunset_end_hour: 20.0,
            },
            weather_profiles: Vec::new(),
            ..Default::default()
        };
    world.shared_weather_catalog.clear();
    world.shared_weather_resolved = None;
}

#[given(regex = r"^shared weather ([0-9a-fA-F]{8}) has scalar ambient day color ([\d.]+)$")]
async fn given_shared_weather_color(world: &mut BevyoutWorld, form_id: String, day: f32) {
    let ambient = bevyout_core::time_of_day::ColorKeyframes {
        day: [day; 4],
        ..Default::default()
    };
    world.shared_weather_catalog.push(
        bevyout_core::manifest::exterior::PreparedWeatherCatalogEntry {
            form_id: u32::from_str_radix(&form_id, 16).unwrap(),
            editor_id: Some("SharedWeather".into()),
            sky_upper: Default::default(),
            sky_lower: Default::default(),
            ambient,
            sunlight: Default::default(),
        },
    );
}

#[when(regex = r"^shared exterior weather ([0-9a-fA-F]{8}) is resolved$")]
async fn when_shared_weather_is_resolved(world: &mut BevyoutWorld, form_id: String) {
    world.shared_weather_resolved =
        bevyout_core::manifest::exterior::resolve_prepared_weather_profile(
            &world.shared_weather_environment,
            &world.shared_weather_catalog,
            u32::from_str_radix(&form_id, 16).unwrap(),
        );
}

#[then(regex = r"^the resolved exterior weather uses sunrise hours ([\d.]+) to ([\d.]+)$")]
async fn then_shared_weather_uses_cell_timings(world: &mut BevyoutWorld, begin: f32, end: f32) {
    let profile = world.shared_weather_resolved.as_ref().unwrap();
    assert_eq!(profile.timings.sunrise_begin_hour, begin);
    assert_eq!(profile.timings.sunrise_end_hour, end);
}

#[then(regex = r"^the resolved exterior weather ambient day color is ([\d.]+)$")]
async fn then_shared_weather_uses_catalog_color(world: &mut BevyoutWorld, expected: f32) {
    assert_eq!(
        world.shared_weather_resolved.as_ref().unwrap().ambient.day,
        [expected; 4]
    );
}

#[then("the exterior cell package embeds zero weather profiles")]
async fn then_exterior_package_embeds_no_weather(world: &mut BevyoutWorld) {
    assert!(world.shared_weather_environment.weather_profiles.is_empty());
}

// ---------------------------------------------------------------------
// cache_object_store.feature -- canonical recipe identities.
// ---------------------------------------------------------------------

#[given(regex = r#"^prepared source path \"([^\"]+)\"$"#)]
async fn given_prepared_source_path(world: &mut BevyoutWorld, path: String) {
    world.prepared_source_path = path;
    world.normalized_prepared_source_path = None;
}

#[when("the prepared source path is normalized")]
async fn when_prepared_source_path_is_normalized(world: &mut BevyoutWorld) {
    world.normalized_prepared_source_path = Some(
        cache_store_policy::normalize_source_path(&world.prepared_source_path)
            .map_err(|error| error.to_string()),
    );
}

#[then(regex = r#"^the normalized prepared source path is \"([^\"]+)\"$"#)]
async fn then_normalized_prepared_source_path_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .normalized_prepared_source_path
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap(),
        &expected
    );
}

#[then("prepared source path normalization is rejected")]
async fn then_prepared_source_path_normalization_is_rejected(world: &mut BevyoutWorld) {
    assert!(
        world
            .normalized_prepared_source_path
            .as_ref()
            .unwrap()
            .is_err()
    );
}

#[given(regex = r#"^a prepared GLB recipe for \"([^\"]+)\"$"#)]
async fn given_prepared_glb_recipe(world: &mut BevyoutWorld, path: String) {
    world.prepared_source_path = path;
    world.prepared_recipe_ids.clear();
}

#[when("its prepared recipe identities are calculated")]
async fn when_prepared_recipe_identities_are_calculated(world: &mut BevyoutWorld) {
    let normalized =
        cache_store_policy::normalize_source_path(&world.prepared_source_path).unwrap();
    let base = cache_store_policy::recipe_identity(
        "glb",
        1,
        &normalized,
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()],
        "converter-v1",
        "format-v1",
        b"settings",
    )
    .unwrap();
    let unchanged = cache_store_policy::recipe_identity(
        "glb",
        1,
        &normalized,
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()],
        "converter-v1",
        "format-v1",
        b"settings",
    )
    .unwrap();
    let converter = cache_store_policy::recipe_identity(
        "glb",
        1,
        &normalized,
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()],
        "converter-v2",
        "format-v1",
        b"settings",
    )
    .unwrap();
    let format = cache_store_policy::recipe_identity(
        "glb",
        1,
        &normalized,
        &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()],
        "converter-v1",
        "format-v2",
        b"settings",
    )
    .unwrap();
    world.prepared_recipe_ids = vec![base, unchanged, converter, format];
}

#[then("the unchanged prepared recipe identity is stable")]
async fn then_unchanged_prepared_recipe_identity_is_stable(world: &mut BevyoutWorld) {
    assert_eq!(world.prepared_recipe_ids[0], world.prepared_recipe_ids[1]);
}

#[then("changing the converter revision changes the prepared recipe identity")]
async fn then_converter_revision_changes_prepared_recipe_identity(world: &mut BevyoutWorld) {
    assert_ne!(world.prepared_recipe_ids[0], world.prepared_recipe_ids[2]);
}

// ---------------------------------------------------------------------
// exterior_scene_storage.feature -- compact exterior scene roots.
// ---------------------------------------------------------------------

#[given(regex = r"^exterior cell ([0-9a-fA-F]{8}) belongs to worldspace ([0-9a-fA-F]{8})$")]
async fn given_exterior_scene_storage_ids(
    world: &mut BevyoutWorld,
    cell: String,
    worldspace: String,
) {
    world.exterior_storage_cell = u32::from_str_radix(&cell, 16).unwrap();
    world.exterior_storage_worldspace = u32::from_str_radix(&worldspace, 16).unwrap();
    world.exterior_storage_plan = None;
}

#[when("its exterior scene storage is planned")]
async fn when_exterior_scene_storage_is_planned(world: &mut BevyoutWorld) {
    world.exterior_storage_plan = Some(exterior_scene_policy::exterior_scene_storage_plan(
        world.exterior_storage_worldspace,
        world.exterior_storage_cell,
    ));
}

#[then(regex = r#"^the exterior package path is \"([^\"]+)\"$"#)]
async fn then_exterior_package_path_is(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.exterior_storage_plan.as_ref().unwrap().package_path,
        expected
    );
}

#[then("the scene root does not embed the exterior package")]
async fn then_exterior_scene_does_not_embed_package(world: &mut BevyoutWorld) {
    assert!(!world.exterior_storage_plan.as_ref().unwrap().embed_package);
}

#[then("the scene root does not embed content-wide diagnostics")]
async fn then_exterior_scene_does_not_embed_content_diagnostics(world: &mut BevyoutWorld) {
    assert!(
        !world
            .exterior_storage_plan
            .as_ref()
            .unwrap()
            .embed_content_diagnostics
    );
}

#[then("changing the format policy changes the prepared recipe identity")]
async fn then_format_policy_changes_prepared_recipe_identity(world: &mut BevyoutWorld) {
    assert_ne!(world.prepared_recipe_ids[0], world.prepared_recipe_ids[3]);
}

// ---------------------------------------------------------------------
// m6_wave3_runtime.feature -- exterior gameplay-actor residency sequencing.
// ---------------------------------------------------------------------

fn w3c_form_id(text: &str) -> u32 {
    u32::from_str_radix(text, 16).expect("hexadecimal FormID")
}

fn w3c_owner(cell: &str, generation: u64) -> exterior_actor_plan::PlannedOwner {
    exterior_actor_plan::PlannedOwner {
        cell_form_id: w3c_form_id(cell),
        generation,
    }
}

#[given(
    regex = r"^resident exterior cell ([0-9a-fA-F]{8}) at generation (\d+) covers grid (-?\d+),(-?\d+)$"
)]
async fn given_w3c_resident_cell(
    world: &mut BevyoutWorld,
    cell: String,
    generation: u64,
    x: i32,
    y: i32,
) {
    world.w3c_cells.push(exterior_actor_plan::PlannedCell {
        cell_form_id: w3c_form_id(&cell),
        generation,
        grid: (x, y),
        projectable: true,
        evicting: false,
        actors: Vec::new(),
    });
}

#[given(regex = r"^exterior cell ([0-9a-fA-F]{8}) prepares actor ([0-9a-fA-F]{8})$")]
async fn given_w3c_prepared_actor(world: &mut BevyoutWorld, cell: String, actor: String) {
    let cell_form_id = w3c_form_id(&cell);
    let entry = exterior_actor_plan::PlannedActorEntry {
        reference_form_id: w3c_form_id(&actor),
        has_saved_state: false,
    };
    for planned in &mut world.w3c_cells {
        if planned.cell_form_id == cell_form_id {
            planned.actors.push(entry);
        }
    }
}

#[when("exterior actor residency is planned")]
async fn when_w3c_residency_is_planned(world: &mut BevyoutWorld) {
    world.w3c_plan = exterior_actor_plan::plan_actor_residency(&world.w3c_cells, &world.w3c_live);
    world.w3c_max_owners = world.w3c_max_owners.max(world.w3c_live.len());
}

#[when(
    regex = r"^actor ([0-9a-fA-F]{8}) is projected in cell ([0-9a-fA-F]{8}) at generation (\d+) on grid (-?\d+),(-?\d+)$"
)]
async fn when_w3c_actor_is_projected(
    world: &mut BevyoutWorld,
    actor: String,
    cell: String,
    generation: u64,
    x: i32,
    y: i32,
) {
    let reference_form_id = w3c_form_id(&actor);
    world
        .w3c_live
        .retain(|live| live.reference_form_id != reference_form_id);
    world.w3c_live.push(exterior_actor_plan::PlannedLiveActor {
        reference_form_id,
        owner: w3c_owner(&cell, generation),
        grid: (x, y),
    });
}

#[when(regex = r"^actor ([0-9a-fA-F]{8}) crosses into grid (-?\d+),(-?\d+)$")]
async fn when_w3c_actor_crosses(world: &mut BevyoutWorld, actor: String, x: i32, y: i32) {
    let reference_form_id = w3c_form_id(&actor);
    for live in &mut world.w3c_live {
        if live.reference_form_id == reference_form_id {
            live.grid = (x, y);
        }
    }
}

#[when(regex = r"^exterior cell ([0-9a-fA-F]{8}) begins evicting at generation (\d+)$")]
async fn when_w3c_cell_evicts(world: &mut BevyoutWorld, cell: String, generation: u64) {
    let cell_form_id = w3c_form_id(&cell);
    for planned in &mut world.w3c_cells {
        if planned.cell_form_id == cell_form_id {
            planned.evicting = true;
            planned.generation = generation;
        }
    }
}

#[when(regex = r"^actor ([0-9a-fA-F]{8}) is no longer projected$")]
async fn when_w3c_actor_is_gone(world: &mut BevyoutWorld, actor: String) {
    let reference_form_id = w3c_form_id(&actor);
    world
        .w3c_live
        .retain(|live| live.reference_form_id != reference_form_id);
}

#[when(regex = r"^actor ([0-9a-fA-F]{8}) has saved canonical state$")]
async fn when_w3c_actor_has_saved_state(world: &mut BevyoutWorld, actor: String) {
    let reference_form_id = w3c_form_id(&actor);
    for planned in &mut world.w3c_cells {
        for entry in &mut planned.actors {
            if entry.reference_form_id == reference_form_id {
                entry.has_saved_state = true;
            }
        }
    }
}

#[when(regex = r"^exterior cell ([0-9a-fA-F]{8}) reloads at generation (\d+)$")]
async fn when_w3c_cell_reloads(world: &mut BevyoutWorld, cell: String, generation: u64) {
    let cell_form_id = w3c_form_id(&cell);
    for planned in &mut world.w3c_cells {
        if planned.cell_form_id == cell_form_id {
            planned.evicting = false;
            planned.projectable = true;
            planned.generation = generation;
        }
    }
}

#[then(regex = r"^the plan binds actor ([0-9a-fA-F]{8}) to cell ([0-9a-fA-F]{8})$")]
async fn then_w3c_plan_binds(world: &mut BevyoutWorld, actor: String, cell: String) {
    assert!(
        world
            .w3c_plan
            .contains(&exterior_actor_plan::PlannedRequest::Bind {
                reference_form_id: w3c_form_id(&actor),
                destination: world
                    .w3c_cells
                    .iter()
                    .find(|planned| planned.cell_form_id == w3c_form_id(&cell))
                    .map(|planned| exterior_actor_plan::PlannedOwner {
                        cell_form_id: planned.cell_form_id,
                        generation: planned.generation,
                    })
                    .expect("planned cell"),
            })
    );
}

#[then(regex = r"^the plan restores actor ([0-9a-fA-F]{8}) to cell ([0-9a-fA-F]{8})$")]
async fn then_w3c_plan_restores(world: &mut BevyoutWorld, actor: String, cell: String) {
    assert!(
        world
            .w3c_plan
            .contains(&exterior_actor_plan::PlannedRequest::Restore {
                reference_form_id: w3c_form_id(&actor),
                destination: world
                    .w3c_cells
                    .iter()
                    .find(|planned| planned.cell_form_id == w3c_form_id(&cell))
                    .map(|planned| exterior_actor_plan::PlannedOwner {
                        cell_form_id: planned.cell_form_id,
                        generation: planned.generation,
                    })
                    .expect("planned cell"),
            })
    );
}

#[then(
    regex = r"^the plan hands actor ([0-9a-fA-F]{8}) from cell ([0-9a-fA-F]{8}) to cell ([0-9a-fA-F]{8})$"
)]
async fn then_w3c_plan_hands_off(
    world: &mut BevyoutWorld,
    actor: String,
    source: String,
    destination: String,
) {
    let reference_form_id = w3c_form_id(&actor);
    let matched = world.w3c_plan.iter().any(|request| {
        matches!(
            request,
            exterior_actor_plan::PlannedRequest::Handoff {
                reference_form_id: reference,
                source: from,
                destination: to,
            } if *reference == reference_form_id
                && from.cell_form_id == w3c_form_id(&source)
                && to.cell_form_id == w3c_form_id(&destination)
        )
    });
    assert!(matched, "expected a handoff in {:?}", world.w3c_plan);
}

#[then(regex = r"^the plan unloads actor ([0-9a-fA-F]{8}) from cell ([0-9a-fA-F]{8})$")]
async fn then_w3c_plan_unloads(world: &mut BevyoutWorld, actor: String, cell: String) {
    let reference_form_id = w3c_form_id(&actor);
    let matched = world.w3c_plan.iter().any(|request| {
        matches!(
            request,
            exterior_actor_plan::PlannedRequest::Unload {
                reference_form_id: reference,
                source,
            } if *reference == reference_form_id && source.cell_form_id == w3c_form_id(&cell)
        )
    });
    assert!(matched, "expected an unload in {:?}", world.w3c_plan);
}

#[then(regex = r"^the plan leaves actor ([0-9a-fA-F]{8}) alone$")]
async fn then_w3c_plan_is_a_no_op(world: &mut BevyoutWorld, actor: String) {
    let reference_form_id = w3c_form_id(&actor);
    assert!(
        !world
            .w3c_plan
            .iter()
            .any(|request| request.reference_form_id() == reference_form_id),
        "a settled actor must produce no residency request: {:?}",
        world.w3c_plan
    );
}

#[then(regex = r"^no more than one live owner claimed actor ([0-9a-fA-F]{8})$")]
async fn then_w3c_single_owner(world: &mut BevyoutWorld, actor: String) {
    let reference_form_id = w3c_form_id(&actor);
    let owners = world
        .w3c_live
        .iter()
        .filter(|live| live.reference_form_id == reference_form_id)
        .count();
    assert!(
        owners <= 1,
        "actor {reference_form_id:08x} has {owners} owners"
    );
    assert!(
        !world.w3c_plan.iter().any(|request| matches!(
            request,
            exterior_actor_plan::PlannedRequest::Duplicate { .. }
        )),
        "no duplicate projection may ever be observed"
    );
    assert!(world.w3c_max_owners <= 1);
}

// ---------------------------------------------------------------------
// rpg_stats.feature -- M9 W1 pure SPECIAL/skill/derived/leveling kernels.
// ---------------------------------------------------------------------

/// Maps a Gherkin skill label like "small guns" onto the core enum.
fn rpg_skill(label: &str) -> actor_state::ActorSkill {
    match actor_state::ActorValue::parse(&label.replace(' ', "_")) {
        Some(actor_state::ActorValue::Skill(skill)) => skill,
        other => panic!("unknown skill label {label:?} (parsed {other:?})"),
    }
}

/// Maps a Gherkin attribute label like "strength" onto the core enum.
fn rpg_special(label: &str) -> actor_state::SpecialAttribute {
    match actor_state::ActorValue::parse(label) {
        Some(actor_state::ActorValue::Special(attribute)) => attribute,
        other => panic!("unknown SPECIAL label {label:?} (parsed {other:?})"),
    }
}

#[given(regex = r"^a player sheet with all SPECIAL at 5 and luck at (\d+)$")]
async fn given_rpg_player_sheet(world: &mut BevyoutWorld, luck: String) {
    world.rpg_sheet = stats::CharacterSheet::default();
    world.rpg_settings = stats::GmstSettings::default();
    world.rpg_last_award = None;
    // Also starts every rpg_perks scenario from a clean perk state.
    world.rpg_perk_defs = std::collections::BTreeMap::new();
    world.rpg_perk_progression = perks::PerkProgression::default();
    world.rpg_perk_modifiers = perks::PerkModifiers::default();
    world.rpg_perk_eligibility = None;
    world.rpg_sheet.set_special(
        actor_state::SpecialAttribute::Luck,
        luck.parse::<u8>().unwrap(),
    );
}

#[when(regex = r"^the player tags the ([a-z ]+?) skill$")]
async fn when_rpg_tag_skill(world: &mut BevyoutWorld, label: String) {
    world.rpg_sheet.tagged_skills.insert(rpg_skill(&label));
}

#[when(regex = r"^the player reaches level (\d+) with endurance (\d+) and strength (\d+)$")]
async fn when_rpg_reaches_level(
    world: &mut BevyoutWorld,
    level: String,
    endurance: String,
    strength: String,
) {
    world.rpg_sheet.level = level.parse::<u8>().unwrap();
    world.rpg_sheet.set_special(
        actor_state::SpecialAttribute::Endurance,
        endurance.parse::<u8>().unwrap(),
    );
    world.rpg_sheet.set_special(
        actor_state::SpecialAttribute::Strength,
        strength.parse::<u8>().unwrap(),
    );
}

#[when(regex = r"^the player is awarded (\d+) XP$")]
async fn when_rpg_award_xp(world: &mut BevyoutWorld, amount: String) {
    let outcome = stats::award_xp(
        &mut world.rpg_sheet,
        amount.parse::<u32>().unwrap(),
        perks::NEUTRAL_XP_MULTIPLIER_BPS,
        &world.rpg_settings,
    );
    world.rpg_last_award = Some(outcome);
}

#[when(regex = r"^the player strength is raised by (\d+) and reduced by (\d+)$")]
async fn when_rpg_strength_raised_and_reduced(
    world: &mut BevyoutWorld,
    raised: String,
    reduced: String,
) {
    world.rpg_sheet.mod_special(
        actor_state::SpecialAttribute::Strength,
        raised.parse::<i16>().unwrap(),
    );
    world.rpg_sheet.mod_special(
        actor_state::SpecialAttribute::Strength,
        -reduced.parse::<i16>().unwrap(),
    );
}

#[when(regex = r"^the player spends (\d+) skill points on the ([a-z ]+?) skill$")]
async fn when_rpg_spend_skill_points(world: &mut BevyoutWorld, points: String, label: String) {
    world
        .rpg_sheet
        .add_skill_points(rpg_skill(&label), points.parse::<i16>().unwrap());
}

#[when(regex = r"^damage resistance (\d+) basis points is clamped$")]
async fn when_rpg_clamp_resistance(world: &mut BevyoutWorld, total: String) {
    world.rpg_clamped_resistance_bps = stats::clamp_resistance_bps(total.parse::<u32>().unwrap());
}

#[then(regex = r"^the ([a-z ]+?) skill base is (\d+)$")]
async fn then_rpg_skill_base(world: &mut BevyoutWorld, label: String, expected: String) {
    assert_eq!(
        world.rpg_sheet.skill_base(rpg_skill(&label)),
        expected.parse::<u8>().unwrap(),
        "{label} base"
    );
}

#[then(regex = r"^the ([a-z ]+?) skill value is (\d+)$")]
async fn then_rpg_skill_value(world: &mut BevyoutWorld, label: String, expected: String) {
    assert_eq!(
        world.rpg_sheet.skill_value(rpg_skill(&label)),
        expected.parse::<u8>().unwrap(),
        "{label} value"
    );
}

#[then(regex = r"^the derived max health is (\d+)$")]
async fn then_rpg_max_health(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_sheet.derived(&world.rpg_settings).max_health,
        expected.parse::<f32>().unwrap()
    );
}

#[then(regex = r"^the derived max action points is (\d+)$")]
async fn then_rpg_max_action_points(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .rpg_sheet
            .derived(&world.rpg_settings)
            .max_action_points,
        expected.parse::<f32>().unwrap()
    );
}

#[then(regex = r"^the derived carry weight is (\d+)$")]
async fn then_rpg_carry_weight(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_sheet.derived(&world.rpg_settings).carry_weight,
        expected.parse::<f32>().unwrap()
    );
}

#[then(regex = r"^the derived critical chance is (\d+) basis points$")]
async fn then_rpg_critical_chance(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .rpg_sheet
            .derived(&world.rpg_settings)
            .critical_chance_bps,
        expected.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the player is level (\d+) with (\d+) XP$")]
async fn then_rpg_level_with_xp(world: &mut BevyoutWorld, level: String, into_level: String) {
    assert_eq!(
        u32::from(world.rpg_sheet.level),
        level.parse::<u32>().unwrap()
    );
    assert_eq!(
        world.rpg_sheet.xp_into_level(&world.rpg_settings),
        into_level.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the level-up granted (\d+) skill points$")]
async fn then_rpg_skill_points_granted(world: &mut BevyoutWorld, expected: String) {
    let award = world
        .rpg_last_award
        .expect("an XP award must have happened");
    assert_eq!(award.skill_points_gained, expected.parse::<u16>().unwrap());
}

#[then(regex = r"^the player is level (\d+)$")]
async fn then_rpg_level(world: &mut BevyoutWorld, level: String) {
    assert_eq!(
        u32::from(world.rpg_sheet.level),
        level.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the player XP never exceeds the level (\d+) threshold$")]
async fn then_rpg_xp_below_threshold(world: &mut BevyoutWorld, level: String) {
    let threshold = stats::xp_threshold(level.parse::<u8>().unwrap(), &world.rpg_settings);
    assert!(world.rpg_sheet.xp <= threshold);
}

#[then(regex = r"^the effective ([a-z]+) is (\d+)$")]
async fn then_rpg_effective_special(world: &mut BevyoutWorld, label: String, expected: String) {
    assert_eq!(
        world.rpg_sheet.effective_special(rpg_special(&label)),
        expected.parse::<u8>().unwrap()
    );
}

#[then(regex = r"^the clamped damage resistance is (\d+) basis points$")]
async fn then_rpg_clamped_resistance(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_clamped_resistance_bps,
        expected.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the base poison resistance from endurance (\d+) is (\d+) basis points$")]
async fn then_rpg_base_poison_resistance(
    _world: &mut BevyoutWorld,
    endurance: String,
    expected: String,
) {
    assert_eq!(
        stats::base_poison_rad_resistance_bps(endurance.parse::<u8>().unwrap()),
        expected.parse::<u32>().unwrap()
    );
}

// ---------------------------------------------------------------------
// rpg_perks.feature -- M9 W2 pure perk eligibility and modifier kernels.
// ---------------------------------------------------------------------

/// Parses an 8-digit hex FormID step argument.
fn perk_form_id(raw: &str) -> u32 {
    u32::from_str_radix(raw, 16).expect("step provides 8-digit hex form ids")
}

/// Creates the perk definition if absent, or returns it for amendment.
fn perk_def_or_create(
    world: &mut BevyoutWorld,
    form_id: u32,
    min_level: u8,
    ranks: u8,
) -> &mut perks::PerkDefinition {
    world
        .rpg_perk_defs
        .entry(form_id)
        .or_insert_with(|| perks::PerkDefinition {
            form_id,
            editor_id: format!("Perk{form_id:08x}"),
            min_level,
            ranks,
            playable: true,
            ..perks::PerkDefinition::default()
        })
}

#[given(regex = r"^a perk ([0-9a-f]{8}) requiring level (\d+) with (\d+) ranks?$")]
async fn given_rpg_perk(world: &mut BevyoutWorld, form_id: String, level: String, ranks: String) {
    let def = perk_def_or_create(
        world,
        perk_form_id(&form_id),
        level.parse::<u8>().unwrap(),
        ranks.parse::<u8>().unwrap(),
    );
    def.editor_id = format!("Perk{}", form_id);
}

#[given(regex = r"^a perk ([0-9a-f]{8}) gated on condition index (\d+) at (\d+)$")]
async fn given_rpg_perk_condition(
    world: &mut BevyoutWorld,
    form_id: String,
    index: String,
    threshold: String,
) {
    // Resolve through the same probed engine-enum mapping the prepare
    // boundary conversion uses; an unmapped index fails the step loudly.
    let index = index.parse::<u32>().unwrap();
    let actor_value = perks::actor_value_from_condition_index(index)
        .unwrap_or_else(|| panic!("condition index {index} has no mapped actor value"));
    let def = perk_def_or_create(world, perk_form_id(&form_id), 1, 1);
    def.conditions.push(perks::PerkCondition {
        actor_value,
        threshold: threshold.parse::<u8>().unwrap(),
    });
}

#[given(regex = r"^a perk ([0-9a-f]{8}) with an unknown condition$")]
async fn given_rpg_perk_unknown_condition(world: &mut BevyoutWorld, form_id: String) {
    let def = perk_def_or_create(world, perk_form_id(&form_id), 1, 1);
    def.unknown_conditions += 1;
}

#[given(regex = r"^a perk ([0-9a-f]{8}) entry rank (\d+) with XP multiplier (\d+\.\d+)$")]
async fn given_rpg_perk_xp_entry(
    world: &mut BevyoutWorld,
    form_id: String,
    rank: String,
    multiplier: String,
) {
    let def = perk_def_or_create(world, perk_form_id(&form_id), 1, 3);
    def.entries.push(perks::PerkEntry::EntryPoint {
        rank: rank.parse::<u8>().unwrap(),
        code: perks::ENTRY_CODE_XP_AWARD_MULTIPLIER,
        param_count: 0,
        priority: 0,
        payload: perks::EntryPointPayload::Value(multiplier.parse::<f32>().unwrap()),
    });
}

#[given(regex = r"^a perk ([0-9a-f]{8}) entry rank (\d+) granting (\d+) bonus skill points$")]
async fn given_rpg_perk_skill_entry(
    world: &mut BevyoutWorld,
    form_id: String,
    rank: String,
    bonus: String,
) {
    let def = perk_def_or_create(world, perk_form_id(&form_id), 1, 1);
    def.entries.push(perks::PerkEntry::EntryPoint {
        rank: rank.parse::<u8>().unwrap(),
        code: perks::ENTRY_CODE_BONUS_SKILL_POINTS,
        param_count: 0,
        priority: 0,
        payload: perks::EntryPointPayload::Value(f32::from(bonus.parse::<u16>().unwrap())),
    });
}

#[given(regex = r"^the player owns perk ([0-9a-f]{8}) at rank (\d+)$")]
async fn given_rpg_perk_owned(world: &mut BevyoutWorld, form_id: String, rank: String) {
    world
        .rpg_perk_progression
        .set_rank(perk_form_id(&form_id), rank.parse::<u8>().unwrap());
}

#[when(regex = r"^the perk eligibility for ([0-9a-f]{8}) is evaluated$")]
async fn when_rpg_perk_eligibility(world: &mut BevyoutWorld, form_id: String) {
    let form_id = perk_form_id(&form_id);
    let def = world
        .rpg_perk_defs
        .get(&form_id)
        .unwrap_or_else(|| panic!("perk {form_id:08x} was not declared by the scenario"));
    let eligibility = perks::can_take_perk(&world.rpg_sheet, def, &world.rpg_perk_progression);
    world.rpg_perk_eligibility = Some((form_id, eligibility));
}

#[when(regex = r"^the active perk modifiers are recomputed$")]
async fn when_rpg_perk_modifiers(world: &mut BevyoutWorld) {
    world.rpg_perk_modifiers =
        perks::active_perk_modifiers(&world.rpg_perk_progression, &world.rpg_perk_defs);
}

#[when(regex = r"^the player is awarded (\d+) XP with active perk modifiers$")]
async fn when_rpg_award_xp_with_perks(world: &mut BevyoutWorld, amount: String) {
    let modifiers = perks::active_perk_modifiers(&world.rpg_perk_progression, &world.rpg_perk_defs);
    let mut outcome = stats::award_xp(
        &mut world.rpg_sheet,
        amount.parse::<u32>().unwrap(),
        modifiers.xp_award_multiplier_bps,
        &world.rpg_settings,
    );
    // The kernel keeps exactly one parameter each (#313), so the adapter
    // adds the per-level perk bonus exactly like the console's apply_award.
    let bonus = u16::from(outcome.levels_gained).saturating_mul(modifiers.bonus_skill_points);
    outcome.skill_points_gained = outcome.skill_points_gained.saturating_add(bonus);
    world.rpg_last_award = Some(outcome);
}

#[then(regex = r"^the perk ([0-9a-f]{8}) is eligible$")]
async fn then_rpg_perk_eligible(world: &mut BevyoutWorld, form_id: String) {
    let (evaluated, eligibility) = world
        .rpg_perk_eligibility
        .as_ref()
        .expect("an eligibility evaluation must have happened");
    assert_eq!(
        *evaluated,
        perk_form_id(&form_id),
        "evaluated the right perk"
    );
    assert!(
        eligibility.is_eligible(),
        "expected eligible, blocked by {:?}",
        eligibility.reasons()
    );
}

#[then(
    regex = r"^the perk ([0-9a-f]{8}) is blocked with reason (min_level|max_ranks|condition|unknown_condition)$"
)]
async fn then_rpg_perk_blocked(world: &mut BevyoutWorld, form_id: String, reason: String) {
    let (evaluated, eligibility) = world
        .rpg_perk_eligibility
        .as_ref()
        .expect("an eligibility evaluation must have happened");
    assert_eq!(
        *evaluated,
        perk_form_id(&form_id),
        "evaluated the right perk"
    );
    let kinds: Vec<&str> = eligibility.reasons().iter().map(|r| r.kind()).collect();
    assert!(
        kinds.contains(&reason.as_str()),
        "expected a {reason} block, reasons were {kinds:?}"
    );
}

#[then(regex = r"^the XP award multiplier is (\d+) basis points$")]
async fn then_rpg_xp_multiplier(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_perk_modifiers.xp_award_multiplier_bps,
        expected.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the perk skill point bonus is (\d+)$")]
async fn then_rpg_skill_bonus(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_perk_modifiers.bonus_skill_points,
        expected.parse::<u16>().unwrap()
    );
}

// ---------------------------------------------------------------------
// rpg_effects.feature -- M9 W3 pure active-effect/radiation/addiction
// kernels (#317).
// ---------------------------------------------------------------------

#[given(regex = r"^a clean effect ledger$")]
async fn given_rpg_clean_ledger(world: &mut BevyoutWorld) {
    world.rpg_effect_ledger = effects::ActiveEffectsLedger::default();
    world.rpg_expired_effects = Vec::new();
}

#[when(regex = r"^the player takes a timed chem effect (\w+) ([+-]\d+) lasting (\d+) seconds$")]
async fn when_rpg_take_chem_effect(
    world: &mut BevyoutWorld,
    label: String,
    magnitude: String,
    seconds: String,
) {
    let attribute = match actor_state::ActorValue::parse(&label) {
        Some(actor_state::ActorValue::Special(attribute)) => attribute,
        other => panic!("unknown SPECIAL label {label:?} (parsed {other:?})"),
    };
    world.rpg_effect_ledger.apply(effects::ActiveEffect {
        source: effects::EffectSource::Chem,
        actor_value: actor_state::ActorValue::Special(attribute),
        magnitude: magnitude.parse::<f32>().unwrap(),
        remaining_ms: seconds.parse::<u32>().unwrap() * 1000,
    });
}

#[when(
    regex = r"^the player takes a timed (action_points|rad_resist) effect ([+-]\d+) lasting (\d+) seconds$"
)]
async fn when_rpg_take_actor_value_effect(
    world: &mut BevyoutWorld,
    label: String,
    magnitude: String,
    seconds: String,
) {
    let actor_value = actor_state::ActorValue::parse(&label).expect("known actor value");
    world.rpg_effect_ledger.apply(effects::ActiveEffect {
        source: effects::EffectSource::Chem,
        actor_value,
        magnitude: magnitude.parse::<f32>().unwrap(),
        remaining_ms: seconds.parse::<u32>().unwrap() * 1000,
    });
}

#[when(regex = r"^the ledger ticks (\d+) milliseconds$")]
async fn when_rpg_tick_ledger(world: &mut BevyoutWorld, delta_ms: String) {
    let expired = world
        .rpg_effect_ledger
        .tick(delta_ms.parse::<u32>().unwrap());
    world.rpg_expired_effects.extend(expired);
}

#[then(regex = r"^the tick expires (\d+) effects$")]
async fn then_rpg_expired_count(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_expired_effects.len(),
        expected.parse::<usize>().unwrap()
    );
}

#[then(regex = r"^the ledger holds (\d+) active effects$")]
async fn then_rpg_ledger_len(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_effect_ledger.len(),
        expected.parse::<usize>().unwrap()
    );
}

#[then(
    regex = r"^the (strength|perception|endurance|charisma|intelligence|agility|luck) modifier is ([+-]?\d+)$"
)]
async fn then_rpg_modifier(world: &mut BevyoutWorld, label: String, expected: String) {
    let attribute = rpg_special(&label);
    let modifier = world
        .rpg_effect_ledger
        .modifier_for(actor_state::ActorValue::Special(attribute));
    assert_eq!(modifier, expected.parse::<f32>().unwrap());
}

#[given(regex = r"^a radiation pool of (\d+) rads$")]
async fn given_rpg_radiation_pool(world: &mut BevyoutWorld, rads: String) {
    world.rpg_radiation_pool = radiation::RadiationPool::new(rads.parse::<u16>().unwrap());
    world.rpg_last_dose = None;
    world.rads_removed = 0;
}

#[when(regex = r"^effective SPECIAL is projected$")]
async fn when_rpg_project_special(world: &mut BevyoutWorld) {
    // The projection itself is asserted through the per-attribute steps;
    // this step exists to make the Gherkin read like the runtime contract.
    let sheet = stats::CharacterSheet::default();
    world.rpg_projected_special = effects::projected_special(
        &sheet,
        &world.rpg_effect_ledger,
        world.rpg_radiation_pool.rads,
    );
}

#[then(
    regex = r"^effective (strength|perception|endurance|charisma|intelligence|agility|luck) is (\d+)$"
)]
async fn then_rpg_projected_special_value(
    world: &mut BevyoutWorld,
    label: String,
    expected: String,
) {
    let attribute = rpg_special(&label);
    assert_eq!(
        world
            .rpg_projected_special
            .get(&attribute)
            .copied()
            .expect("projection covers every SPECIAL attribute"),
        expected.parse::<u8>().unwrap()
    );
}

#[when(regex = r"^derived actor values are projected$")]
async fn when_rpg_project_derived(world: &mut BevyoutWorld) {
    world.rpg_projected_derived = effects::projected_derived(
        &stats::CharacterSheet::default(),
        &world.rpg_effect_ledger,
        world.rpg_radiation_pool.rads,
        &stats::GmstSettings::default(),
    );
}

#[then(regex = r"^projected maximum health is (\d+)$")]
async fn then_rpg_projected_health(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_projected_derived.max_health,
        expected.parse::<f32>().unwrap()
    );
}

#[then(regex = r"^projected maximum action points is (\d+)$")]
async fn then_rpg_projected_ap(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_projected_derived.max_action_points,
        expected.parse::<f32>().unwrap()
    );
}

#[then(regex = r"^active radiation resistance is (\d+) basis points$")]
async fn then_rpg_rad_resistance(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        effects::active_rad_resistance_bps(&world.rpg_effect_ledger),
        expected.parse::<u32>().unwrap()
    );
}

#[given(regex = r"^the player (does not own|owns) perk ([0-9a-fA-F]{8})$")]
async fn given_rpg_effect_perk(world: &mut BevyoutWorld, ownership: String, form_id: String) {
    let form_id = u32::from_str_radix(&form_id, 16).unwrap();
    world
        .rpg_perk_progression_effects
        .set_rank(form_id, u8::from(ownership == "owns"));
}

#[then(regex = r"^the Stimpak (30|36) health condition is (true|false)$")]
async fn then_rpg_stimpak_condition(world: &mut BevyoutWorld, magnitude: String, expected: String) {
    let condition = effects::IngestibleCondition {
        oper: effects::CONDITION_OPER_EQUAL,
        comparison_value: if magnitude == "30" { 0.0 } else { 1.0 },
        function: effects::CONDITION_FUNCTION_HAS_PERK,
        param1: 0x0009_4ebf,
    };
    let actual =
        effects::evaluate_ingestible_condition(&condition, &world.rpg_perk_progression_effects);
    assert_eq!(
        actual == effects::IngestibleConditionOutcome::True,
        expected == "true"
    );
}

#[then(regex = r"^the highest radiation threshold is (\d+)$")]
async fn then_rpg_rad_threshold(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        radiation::threshold_reached(world.rpg_radiation_pool.rads),
        expected.parse::<u16>().unwrap()
    );
}

#[then(regex = r"^the radiation pool is( not)? fatal$")]
async fn then_rpg_rad_fatal(world: &mut BevyoutWorld, negated: String) {
    let fatal = world.rpg_radiation_pool.is_fatal();
    assert_eq!(!fatal, !negated.is_empty(), "fatal = {fatal}");
}

#[when(regex = r"^(\d+) rads are applied with (\d+) basis points of resistance$")]
async fn when_rpg_apply_dose(world: &mut BevyoutWorld, dose: String, resistance: String) {
    world.rpg_last_dose = Some(radiation::apply_radiation(
        &mut world.rpg_radiation_pool,
        dose.parse::<u16>().unwrap(),
        resistance.parse::<u32>().unwrap(),
    ));
}

#[then(regex = r"^the absorbed dose is (\d+) rads$")]
async fn then_rpg_absorbed(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .rpg_last_dose
            .as_ref()
            .expect("a dose must have been applied")
            .absorbed_rads,
        expected.parse::<u16>().unwrap()
    );
}

#[then(regex = r"^the radiation pool is (\d+) rads$")]
async fn then_rpg_rad_value(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world.rpg_radiation_pool.rads,
        expected.parse::<u16>().unwrap()
    );
}

#[when(regex = r"^(\d+) rads are removed$")]
async fn when_rpg_remove_rads(world: &mut BevyoutWorld, amount: String) {
    world.rads_removed = radiation::remove_rads(
        &mut world.rpg_radiation_pool,
        amount.parse::<u16>().unwrap(),
    );
}

#[then(regex = r"^only (\d+) rads were actually removed by the oversized dose$")]
async fn then_rpg_removed_amount(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(world.rads_removed, expected.parse::<u16>().unwrap());
}

#[given(regex = r"^a fresh addiction rng seeded with (\d+)$")]
async fn given_rpg_rng(world: &mut BevyoutWorld, seed: String) {
    world.rpg_rng = chems::RpgRngState::new(seed.parse::<u64>().unwrap());
    world.rpg_addiction_roll = None;
}

#[when(regex = r"^an addiction roll is made with (\d+) bps chance and (\d+) bps resistance$")]
async fn when_rpg_roll_addiction(world: &mut BevyoutWorld, chance: String, resistance: String) {
    let rolled = chems::roll_addiction(
        chance.parse::<u32>().unwrap(),
        resistance.parse::<u32>().unwrap(),
        &mut world.rpg_rng,
    );
    world.rpg_addiction_roll = Some((rolled, world.rpg_rng.draw_index));
}

#[then(regex = r"^the addiction roll is (true|false)$")]
async fn then_rpg_roll_outcome(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .rpg_addiction_roll
            .as_ref()
            .expect("a roll must have been made")
            .0,
        expected == "true"
    );
}

#[then(regex = r"^(\d+) rng draws were (?:consumed|still consumed in total)$")]
async fn then_rpg_draw_index(world: &mut BevyoutWorld, expected: String) {
    assert_eq!(
        world
            .rpg_addiction_roll
            .as_ref()
            .expect("a roll must have been made")
            .1,
        expected.parse::<u32>().unwrap()
    );
}

#[given(regex = r"^a clean addiction state$")]
async fn given_rpg_clean_addictions(world: &mut BevyoutWorld) {
    world.rpg_addictions = chems::Addictions::default();
}

fn withdrawal_form_id(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).expect("withdrawal FormID must be hex")
}

#[then(regex = r"^the withdrawal ([0-9a-f]{8}) addiction phase is (clean|addicted|withdrawing)$")]
async fn then_rpg_phase(world: &mut BevyoutWorld, form_id: String, expected: String) {
    let id = withdrawal_form_id(&form_id);
    let actual = match world.rpg_addictions.0.get(&id).copied() {
        Some(chems::AddictionPhase::Addicted) => "addicted",
        Some(chems::AddictionPhase::Withdrawing) => "withdrawing",
        Some(chems::AddictionPhase::Clean) | None => "clean",
    };
    assert_eq!(actual, expected);
}

#[when(regex = r"^the player becomes addicted to withdrawal ([0-9a-f]{8})$")]
async fn when_rpg_addict(world: &mut BevyoutWorld, form_id: String) {
    world.rpg_addictions.addict(withdrawal_form_id(&form_id));
}

#[when(regex = r"^the chem wears off for withdrawal ([0-9a-f]{8})$")]
async fn when_rpg_begin_withdrawal(world: &mut BevyoutWorld, form_id: String) {
    assert!(
        world
            .rpg_addictions
            .begin_withdrawal(withdrawal_form_id(&form_id)),
        "withdrawal only starts while addicted"
    );
}

#[when(regex = r"^the withdrawal ([0-9a-f]{8}) is cured$")]
async fn when_rpg_cure(world: &mut BevyoutWorld, form_id: String) {
    assert!(world.rpg_addictions.cure(withdrawal_form_id(&form_id)));
}

// ---------------------------------------------------------------------
// rpg_limbs.feature -- M9 W4 semantic body parts, cripple, medical aid.
// ---------------------------------------------------------------------

fn rpg_body_part(label: &str) -> bevyout_core::combat::BodyPartId {
    bevyout_core::combat::BodyPartId::parse(label)
        .unwrap_or_else(|| panic!("unknown body part {label:?}"))
}

#[given(regex = r"^a healthy limb state$")]
async fn given_rpg_healthy_limbs(world: &mut BevyoutWorld) {
    world.rpg_limbs = bevyout_core::combat::LimbState::healthy();
    world.rpg_last_limb_impact = None;
    world.rpg_last_restoration = None;
    world.rpg_mapped_part = None;
}

#[then(regex = r"^the limb state has (\d+) parts$")]
async fn then_rpg_limb_part_count(world: &mut BevyoutWorld, count: String) {
    assert_eq!(world.rpg_limbs.parts.len(), count.parse::<usize>().unwrap());
}

#[then(
    regex = r"^the (head|torso|left arm|right arm|left leg|right leg) limb is (\d+) milli and (not )?crippled$"
)]
async fn then_rpg_limb_condition(
    world: &mut BevyoutWorld,
    part: String,
    milli: String,
    not_crippled: String,
) {
    let condition = world.rpg_limbs.part(rpg_body_part(&part));
    assert_eq!(condition.current_milli, milli.parse::<u32>().unwrap());
    assert_eq!(condition.crippled, not_crippled.is_empty());
}

#[then(regex = r"^the locomotion speed is (\d+) basis points$")]
async fn then_rpg_locomotion(world: &mut BevyoutWorld, bps: String) {
    assert_eq!(
        world.rpg_limbs.locomotion_speed_bps(),
        bps.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the arm reload multiplier is (\d+) basis points$")]
async fn then_rpg_arm_reload(world: &mut BevyoutWorld, bps: String) {
    assert_eq!(
        world.rpg_limbs.arm_reload_multiplier_bps(),
        bps.parse::<u32>().unwrap()
    );
}

#[then(regex = r"^the arm spread penalty is (\d+) basis points$")]
async fn then_rpg_arm_spread(world: &mut BevyoutWorld, bps: String) {
    assert_eq!(
        world.rpg_limbs.arm_spread_penalty_bps(),
        bps.parse::<u32>().unwrap()
    );
}

#[when(regex = r"^an unmarked node is mapped to a body part$")]
async fn when_rpg_unmarked_node(world: &mut BevyoutWorld) {
    world.rpg_mapped_part = Some(bevyout_core::combat::BodyPartId::from_node_name(""));
}

#[when(regex = r#"^the unknown node "([^"]+)" is mapped to a body part$"#)]
async fn when_rpg_named_node(world: &mut BevyoutWorld, name: String) {
    world.rpg_mapped_part = Some(bevyout_core::combat::BodyPartId::from_node_name(&name));
}

#[then(regex = r"^the mapped body part is (head|torso|left arm|right arm|left leg|right leg)$")]
async fn then_rpg_mapped_part(world: &mut BevyoutWorld, part: String) {
    assert_eq!(world.rpg_mapped_part, Some(rpg_body_part(&part)));
}

#[when(
    regex = r"^limb impact shot (\d+) hits the (head|torso|left arm|right arm|left leg|right leg) for (\d+) milli$"
)]
async fn when_rpg_limb_impact(world: &mut BevyoutWorld, shot: String, part: String, milli: String) {
    world.rpg_last_limb_impact = Some(bevyout_core::combat::apply_limb_impact(
        &mut world.rpg_limbs,
        bevyout_core::combat::LimbImpact {
            shot_id: bevyout_core::combat::ShotId(shot.parse::<u64>().unwrap()),
            target: perception::TargetId::player(),
            part: rpg_body_part(&part),
            final_damage_milli: milli.parse::<u32>().unwrap(),
        },
    ));
}

#[then(regex = r"^the last limb impact newly crippled$")]
async fn then_rpg_newly_crippled(world: &mut BevyoutWorld) {
    assert!(
        world
            .rpg_last_limb_impact
            .expect("limb impact")
            .newly_crippled
    );
}

#[then(regex = r"^the last limb impact did not newly cripple$")]
async fn then_rpg_not_newly_crippled(world: &mut BevyoutWorld) {
    assert!(
        !world
            .rpg_last_limb_impact
            .expect("limb impact")
            .newly_crippled
    );
}

#[then(regex = r"^the last limb impact requested head blur$")]
async fn then_rpg_head_blur(world: &mut BevyoutWorld) {
    assert!(world.rpg_last_limb_impact.expect("limb impact").head_blur);
}

#[when(regex = r"^effective SPECIAL is projected with limbs$")]
async fn when_rpg_project_special_with_limbs(world: &mut BevyoutWorld) {
    world.rpg_projected_special = effects::projected_special_with_limbs(
        &world.rpg_sheet,
        &world.rpg_effect_ledger,
        world.rpg_radiation_pool.rads,
        Some(&world.rpg_limbs),
    );
}

#[given(regex = r"^a healthy actor with (\d+) health$")]
async fn given_rpg_healthy_actor(world: &mut BevyoutWorld, health: String) {
    let health = health.parse::<f32>().unwrap();
    world.rpg_actor_definition = actor_state::ActorDefinition {
        base_form_id: 1,
        reference_form_id: 2,
        ..Default::default()
    };
    world
        .rpg_actor_definition
        .base_values
        .insert(actor_state::ActorValue::Health, health);
    world.rpg_actor_state =
        actor_state::ActorInstanceState::new(2, actor_state::ActorLifeState::Alive);
    world.rpg_limbs = bevyout_core::combat::LimbState::healthy();
    world.rpg_last_impact_outcome = None;
}

#[when(
    regex = r"^weapon impact shot (\d+) hits the (head|torso|left arm|right arm|left leg|right leg) at ([0-9.]+) meters$"
)]
async fn when_rpg_weapon_impact(
    world: &mut BevyoutWorld,
    shot: String,
    part: String,
    distance: String,
) {
    let evidence = weapon::ImpactEvidence {
        distance_meters: distance.parse::<f32>().unwrap(),
        body_part: Some(rpg_body_part(&part)),
        shot_id: Some(bevyout_core::combat::ShotId(shot.parse::<u64>().unwrap())),
        target: Some(perception::TargetId {
            class: perception::TargetClass::Actor,
            form_id: world.rpg_actor_state.reference_form_id,
        }),
    };
    let outcome = weapon::resolve_actor_impact(
        weapon::WeaponDefinition::new(10.0, 100.0),
        evidence,
        &world.rpg_actor_definition,
        &mut world.rpg_actor_state,
    )
    .expect("impact resolves");
    world.rpg_limbs = world.rpg_actor_state.limbs.clone();
    world.rpg_last_impact_outcome = Some(outcome);
}

#[then(regex = r"^the actor remaining health is ([0-9.]+)$")]
async fn then_rpg_actor_health(world: &mut BevyoutWorld, remaining: String) {
    let actual = world
        .rpg_actor_definition
        .resolve_value(&world.rpg_actor_state, actor_state::ActorValue::Health)
        .effective;
    assert_eq!(actual, remaining.parse::<f32>().unwrap());
}

#[then(regex = r"^the impact was duplicate$")]
async fn then_rpg_duplicate_impact(world: &mut BevyoutWorld) {
    assert!(matches!(
        world.rpg_last_impact_outcome,
        Some(weapon::ImpactOutcome::Duplicate)
    ));
}

#[when(
    regex = r"^a targeted stimpak restores the (head|torso|left arm|right arm|left leg|right leg) at game time (\d+)$"
)]
async fn when_rpg_stimpak(world: &mut BevyoutWorld, part: String, time: String) {
    world.rpg_last_restoration = Some(bevyout_core::combat::restore_limbs(
        &mut world.rpg_limbs,
        bevyout_core::combat::MedicalSource::TargetedStimpak,
        Some(rpg_body_part(&part)),
        bevyout_core::time::GameTime::from_ms(time.parse::<u64>().unwrap()),
    ));
}

#[when(regex = r"^a doctor restores all limbs at game time (\d+)$")]
async fn when_rpg_doctor(world: &mut BevyoutWorld, time: String) {
    world.rpg_last_restoration = Some(bevyout_core::combat::restore_limbs(
        &mut world.rpg_limbs,
        bevyout_core::combat::MedicalSource::Doctor,
        None,
        bevyout_core::time::GameTime::from_ms(time.parse::<u64>().unwrap()),
    ));
}

#[when(regex = r"^an owned bed restores all limbs at game time (\d+)$")]
async fn when_rpg_owned_bed(world: &mut BevyoutWorld, time: String) {
    world.rpg_last_restoration = Some(bevyout_core::combat::restore_limbs(
        &mut world.rpg_limbs,
        bevyout_core::combat::MedicalSource::OwnedBed,
        None,
        bevyout_core::time::GameTime::from_ms(time.parse::<u64>().unwrap()),
    ));
}

#[then(regex = r"^the last restoration consumed a targeted stimpak$")]
async fn then_rpg_stimpak_source(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rpg_last_restoration.expect("restoration").source,
        bevyout_core::combat::MedicalSource::TargetedStimpak
    );
}

#[then(regex = r"^the last restoration used a doctor$")]
async fn then_rpg_doctor_source(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rpg_last_restoration.expect("restoration").source,
        bevyout_core::combat::MedicalSource::Doctor
    );
}

#[then(regex = r"^the last restoration used an owned bed at game time (\d+)$")]
async fn then_rpg_bed_source(world: &mut BevyoutWorld, time: String) {
    let outcome = world.rpg_last_restoration.expect("restoration");
    assert_eq!(
        outcome.source,
        bevyout_core::combat::MedicalSource::OwnedBed
    );
    assert_eq!(outcome.at.as_ms(), time.parse::<u64>().unwrap());
}

#[when(regex = r"^the limb state is serialized and restored$")]
async fn when_rpg_limbs_round_trip(world: &mut BevyoutWorld) {
    let encoded = serde_json::to_string(&world.rpg_limbs).expect("encode limbs");
    world.rpg_limbs = serde_json::from_str(&encoded).expect("decode limbs");
}

// ---------------------------------------------------------------------
// rpg_repair_barter.feature (M9 W5 repair, craft, barter)
// ---------------------------------------------------------------------

fn rpg_player_item(world: &BevyoutWorld, id: ItemInstanceId) -> &ItemInstance {
    world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .and_then(|state| state.find(id))
        .expect("canonical player item")
}

fn rpg_append_player_item(world: &mut BevyoutWorld, item: ItemInstance) {
    let mut state = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .cloned()
        .unwrap_or_default();
    state.items.push(item);
    world
        .canonical_ledger
        .insert_holder(HolderId::Player, state)
        .unwrap();
}

#[given(
    regex = r"^the canonical player also holds item 0x([0-9a-fA-F]+) form 0x([0-9a-fA-F]+) x(\d+) condition (none|\d+)$"
)]
async fn given_canonical_player_also_holds(
    world: &mut BevyoutWorld,
    item_hex: String,
    form_hex: String,
    count: u32,
    condition: String,
) {
    let item = ItemInstance::new(
        parse_item_instance_id(&item_hex),
        parse_hex(&form_hex),
        count,
        ItemState {
            condition: (condition != "none").then(|| condition.parse().unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    rpg_append_player_item(world, item);
}

#[given(regex = r"^the repair max condition is (\d+)$")]
async fn given_rpg_repair_max_condition(world: &mut BevyoutWorld, max_condition: u32) {
    world.rpg_repair_max_condition = max_condition;
}

#[given(regex = r"^the player repair skill is (\d+)$")]
async fn given_rpg_repair_skill(world: &mut BevyoutWorld, skill: u8) {
    world.rpg_repair_skill = skill;
}

#[given(regex = r"^the canonical player equips item 0x([0-9a-fA-F]+)$")]
async fn given_canonical_player_equips(world: &mut BevyoutWorld, item_hex: String) {
    world
        .canonical_ledger
        .equip(HolderId::Player, parse_item_instance_id(&item_hex))
        .unwrap();
}

fn rpg_run_repair(
    world: &mut BevyoutWorld,
    target: ItemInstanceId,
    donor: ItemInstanceId,
    id: TransactionId,
) {
    let expected = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .map(|state| state.revision)
        .unwrap_or(0);
    world.rpg_repair_result = Some(bevyout_core::repair::repair(
        &mut world.canonical_ledger,
        bevyout_core::repair::RepairRequest {
            transaction_id: id,
            holder: HolderId::Player,
            target,
            donor,
            repair_skill: world.rpg_repair_skill,
            max_condition: world.rpg_repair_max_condition,
            expected_holder_revision: expected,
        },
    ));
}

#[when(regex = r"^repairing target 0x([0-9a-fA-F]+) with donor 0x([0-9a-fA-F]+)$")]
async fn when_rpg_repair(world: &mut BevyoutWorld, target: String, donor: String) {
    let id = world.canonical_ledger.next_transaction_id();
    rpg_run_repair(
        world,
        parse_item_instance_id(&target),
        parse_item_instance_id(&donor),
        id,
    );
}

#[when(
    regex = r"^repairing target 0x([0-9a-fA-F]+) with donor 0x([0-9a-fA-F]+) using transaction (\d+)$"
)]
async fn when_rpg_repair_with_id(
    world: &mut BevyoutWorld,
    target: String,
    donor: String,
    transaction: u64,
) {
    rpg_run_repair(
        world,
        parse_item_instance_id(&target),
        parse_item_instance_id(&donor),
        TransactionId(transaction),
    );
}

#[then("the repair succeeds")]
async fn then_rpg_repair_succeeds(world: &mut BevyoutWorld) {
    world
        .rpg_repair_result
        .as_ref()
        .expect("repair was not run")
        .as_ref()
        .expect("repair failed");
}

#[then("the repair is rejected as same item")]
async fn then_rpg_repair_same_item(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .rpg_repair_result
            .as_ref()
            .expect("repair was not run"),
        &Err(bevyout_core::repair::RepairError::SameItem)
    );
}

#[then("the repair is rejected as incompatible")]
async fn then_rpg_repair_incompatible(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .rpg_repair_result
            .as_ref()
            .expect("repair was not run"),
        &Err(bevyout_core::repair::RepairError::Incompatible)
    );
}

#[then("the repair is rejected as equipped donor")]
async fn then_rpg_repair_equipped_donor(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .rpg_repair_result
            .as_ref()
            .expect("repair was not run"),
        &Err(bevyout_core::repair::RepairError::EquippedDonor)
    );
}

#[then(regex = r"^the canonical player item 0x([0-9a-fA-F]+) has condition (\d+)$")]
async fn then_canonical_player_item_condition(
    world: &mut BevyoutWorld,
    item_hex: String,
    condition: u32,
) {
    assert_eq!(
        rpg_player_item(world, parse_item_instance_id(&item_hex))
            .state
            .condition,
        Some(condition)
    );
}

#[given(
    regex = r"^a known schematic 0x([0-9a-fA-F]+) requiring (\d+) of 0x([0-9a-fA-F]+) and producing (\d+) of 0x([0-9a-fA-F]+)$"
)]
async fn given_rpg_schematic(
    world: &mut BevyoutWorld,
    recipe_hex: String,
    ingredient_count: u32,
    ingredient_hex: String,
    output_count: u32,
    output_hex: String,
) {
    let form_id = parse_hex(&recipe_hex);
    world.rpg_craft_recipes.insert(
        form_id,
        bevyout_core::crafting::RecipeDefinition {
            form_id,
            skill: 0,
            level: 0,
            ingredients: vec![bevyout_core::crafting::RecipeItem {
                item_form_id: parse_hex(&ingredient_hex),
                quantity: ingredient_count,
                order: 0,
            }],
            outputs: vec![bevyout_core::crafting::RecipeItem {
                item_form_id: parse_hex(&output_hex),
                quantity: output_count,
                order: 0,
            }],
            has_conditions: false,
        },
    );
}

#[given(
    regex = r"^a known schematic 0x([0-9a-fA-F]+) with an opaque condition requiring (\d+) of 0x([0-9a-fA-F]+) and producing (\d+) of 0x([0-9a-fA-F]+)$"
)]
async fn given_rpg_schematic_opaque(
    world: &mut BevyoutWorld,
    recipe_hex: String,
    ingredient_count: u32,
    ingredient_hex: String,
    output_count: u32,
    output_hex: String,
) {
    let form_id = parse_hex(&recipe_hex);
    world.rpg_craft_recipes.insert(
        form_id,
        bevyout_core::crafting::RecipeDefinition {
            form_id,
            skill: 0,
            level: 0,
            ingredients: vec![bevyout_core::crafting::RecipeItem {
                item_form_id: parse_hex(&ingredient_hex),
                quantity: ingredient_count,
                order: 0,
            }],
            outputs: vec![bevyout_core::crafting::RecipeItem {
                item_form_id: parse_hex(&output_hex),
                quantity: output_count,
                order: 0,
            }],
            has_conditions: true,
        },
    );
}

#[when(regex = r"^crafting recipe 0x([0-9a-fA-F]+) once$")]
async fn when_rpg_craft(world: &mut BevyoutWorld, recipe_hex: String) {
    world.rpg_next_item_id_before = Some(world.canonical_ledger.next_item_id());
    let recipe = world
        .rpg_craft_recipes
        .get(&parse_hex(&recipe_hex))
        .cloned()
        .expect("unknown schematic");
    let expected = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .map(|state| state.revision)
        .unwrap_or(0);
    let id = world.canonical_ledger.next_transaction_id();
    world.rpg_craft_result = Some(bevyout_core::crafting::craft(
        &mut world.canonical_ledger,
        bevyout_core::crafting::CraftRequest {
            transaction_id: id,
            holder: HolderId::Player,
            recipe: &recipe,
            count: 1,
            expected_holder_revision: expected,
            actor: bevyout_core::crafting::CraftingActorSnapshot { skill_value: 100 },
            schematic_tier: bevyout_core::crafting::SchematicTier::V1,
        },
    ));
}

#[then("the craft succeeds")]
async fn then_rpg_craft_succeeds(world: &mut BevyoutWorld) {
    world
        .rpg_craft_result
        .as_ref()
        .expect("craft was not run")
        .as_ref()
        .expect("craft failed");
}

#[then("the craft is rejected as unsupported condition")]
async fn then_rpg_craft_unsupported(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rpg_craft_result.as_ref().expect("craft was not run"),
        &Err(bevyout_core::crafting::CraftError::UnsupportedCondition)
    );
}

#[then("the craft is rejected as missing ingredients")]
async fn then_rpg_craft_missing(world: &mut BevyoutWorld) {
    assert_eq!(
        world.rpg_craft_result.as_ref().expect("craft was not run"),
        &Err(bevyout_core::crafting::CraftError::MissingIngredients)
    );
}

#[then(regex = r"^the canonical player holds (\d+) of form 0x([0-9a-fA-F]+)$")]
async fn then_canonical_player_holds_form(world: &mut BevyoutWorld, count: u32, form_hex: String) {
    let form = parse_hex(&form_hex);
    let actual = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .map(|state| {
            state
                .items
                .iter()
                .filter(|item| item.base_form_id == form)
                .map(|item| item.count)
                .sum::<u32>()
        })
        .unwrap_or(0);
    assert_eq!(actual, count);
}

#[then("the next canonical item id is unchanged")]
async fn then_next_item_id_unchanged(world: &mut BevyoutWorld) {
    assert_eq!(
        world.canonical_ledger.next_item_id(),
        world.rpg_next_item_id_before.expect("craft was not run")
    );
}

#[given(regex = r"^a catalog item 0x([0-9a-fA-F]+) worth (\d+) caps$")]
async fn given_rpg_catalog_value(world: &mut BevyoutWorld, _form_hex: String, value: u64) {
    world.rpg_barter_catalog_value = value;
}

#[given(regex = r"^the player barter skill is (\d+)$")]
async fn given_rpg_barter_skill(world: &mut BevyoutWorld, skill: u8) {
    world.rpg_barter_skill = skill;
}

#[given(
    regex = r"^the canonical merchant 0x([0-9a-fA-F]+) holds item 0x([0-9a-fA-F]+) form 0x([0-9a-fA-F]+) x(\d+) condition (none|\d+) with (\d+) caps$"
)]
async fn given_canonical_merchant_item(
    world: &mut BevyoutWorld,
    merchant_hex: String,
    item_hex: String,
    form_hex: String,
    count: u32,
    condition: String,
    caps: u64,
) {
    let item = ItemInstance::new(
        parse_item_instance_id(&item_hex),
        parse_hex(&form_hex),
        count,
        ItemState {
            condition: (condition != "none").then(|| condition.parse().unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    world
        .canonical_ledger
        .insert_holder(
            HolderId::FixtureMerchant {
                reference_form_id: parse_hex(&merchant_hex),
            },
            ItemHolderState {
                items: vec![item],
                caps,
                revision: 0,
            },
        )
        .unwrap();
}

#[given(regex = r"^the canonical player has (\d+) caps$")]
async fn given_canonical_player_caps(world: &mut BevyoutWorld, caps: u64) {
    world
        .canonical_ledger
        .holders_mut()
        .entry(HolderId::Player)
        .or_default()
        .caps = caps;
}

#[when(regex = r"^quoting a (buy|sell) of (\d+) of item 0x([0-9a-fA-F]+)$")]
async fn when_rpg_quote_catalog(
    world: &mut BevyoutWorld,
    direction: String,
    count: u32,
    item_hex: String,
) {
    world.rpg_barter_quote = Some(
        bevyout_core::barter::quote_barter(bevyout_core::barter::BarterQuoteInput {
            direction: if direction == "buy" {
                bevyout_core::barter::BarterDirection::Buy
            } else {
                bevyout_core::barter::BarterDirection::Sell
            },
            merchant: HolderId::FixtureMerchant {
                reference_form_id: 9,
            },
            player: HolderId::Player,
            item_id: parse_item_instance_id(&item_hex),
            count,
            base_value: world.rpg_barter_catalog_value,
            player_barter: world.rpg_barter_skill,
            player_revision: 0,
            merchant_revision: 0,
        })
        .expect("quote"),
    );
}

#[when(regex = r"^quoting a buy of item 0x([0-9a-fA-F]+) from merchant 0x([0-9a-fA-F]+)$")]
async fn when_rpg_quote_merchant(world: &mut BevyoutWorld, item_hex: String, merchant_hex: String) {
    let merchant = HolderId::FixtureMerchant {
        reference_form_id: parse_hex(&merchant_hex),
    };
    let player_revision = world
        .canonical_ledger
        .holders()
        .get(&HolderId::Player)
        .map(|state| state.revision)
        .unwrap_or(0);
    let merchant_revision = world
        .canonical_ledger
        .holders()
        .get(&merchant)
        .map(|state| state.revision)
        .unwrap_or(0);
    world.rpg_barter_quote = Some(
        bevyout_core::barter::quote_barter(bevyout_core::barter::BarterQuoteInput {
            direction: bevyout_core::barter::BarterDirection::Buy,
            merchant,
            player: HolderId::Player,
            item_id: parse_item_instance_id(&item_hex),
            count: 1,
            base_value: world.rpg_barter_catalog_value,
            player_barter: world.rpg_barter_skill,
            player_revision,
            merchant_revision,
        })
        .expect("quote"),
    );
}

#[when("the merchant holder revision changes")]
async fn when_rpg_merchant_revision_changes(world: &mut BevyoutWorld) {
    let merchant = world.rpg_barter_quote.as_ref().expect("quote").merchant;
    world
        .canonical_ledger
        .holders_mut()
        .get_mut(&merchant)
        .expect("merchant")
        .revision += 1;
}

#[when("committing the last barter quote")]
async fn when_rpg_commit_quote(world: &mut BevyoutWorld) {
    let quote = world.rpg_barter_quote.clone().expect("quote");
    let id = world.canonical_ledger.next_transaction_id();
    world.rpg_barter_commit = Some(bevyout_core::barter::commit_barter(
        &mut world.canonical_ledger,
        id,
        &quote,
    ));
}

#[then(regex = r"^the barter unit price is (\d+)$")]
async fn then_rpg_barter_unit_price(world: &mut BevyoutWorld, price: u64) {
    assert_eq!(
        world.rpg_barter_quote.as_ref().expect("quote").unit_price,
        price
    );
}

#[then(regex = r"^the barter total is (\d+)$")]
async fn then_rpg_barter_total(world: &mut BevyoutWorld, total: u64) {
    assert_eq!(world.rpg_barter_quote.as_ref().expect("quote").total, total);
}

#[then("the barter commit is rejected as stale")]
async fn then_rpg_barter_stale(world: &mut BevyoutWorld) {
    assert_eq!(
        world
            .rpg_barter_commit
            .as_ref()
            .expect("commit was not run"),
        &Err(bevyout_core::barter::BarterError::StaleQuote)
    );
}

#[given(regex = r"^a merchant restock last due at (\d+) ms$")]
async fn given_rpg_restock(world: &mut BevyoutWorld, last: u64) {
    world.rpg_restock_state = bevyout_core::barter::MerchantRestockState {
        generation: 0,
        last_restock_game_ms: last,
        next_restock_game_ms: last
            .saturating_add(bevyout_core::barter::MERCHANT_RESTOCK_INTERVAL_MS),
        seed_state: bevyout_core::chems::RpgRngState::default(),
    };
}

#[when(regex = r"^restock is evaluated at (\d+) ms$")]
async fn when_rpg_restock(world: &mut BevyoutWorld, now: u64) {
    world.rpg_restock_outcome = Some(bevyout_core::barter::restock_if_due(
        bevyout_core::time::GameTime::from_ms(now),
        &mut world.rpg_restock_state,
        &bevyout_core::barter::MerchantStockCatalog::default(),
    ));
}

#[then("restock is not due")]
async fn then_rpg_restock_not_due(world: &mut BevyoutWorld) {
    assert!(!world.rpg_restock_outcome.as_ref().expect("restock").due);
}

#[then("restock is due")]
async fn then_rpg_restock_due(world: &mut BevyoutWorld) {
    assert!(world.rpg_restock_outcome.as_ref().expect("restock").due);
}

#[then(regex = r"^the restock generation is (\d+)$")]
async fn then_rpg_restock_generation(world: &mut BevyoutWorld, generation: u32) {
    assert_eq!(
        world
            .rpg_restock_outcome
            .as_ref()
            .expect("restock")
            .generation,
        generation
    );
}
