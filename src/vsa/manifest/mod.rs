use std::collections::BTreeMap;

use anyhow::{Result, bail};
use bevy::math::{EulerRot, Quat};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::physics::PreparedPhysicsSource;

pub(crate) const CURRENT_MANIFEST_SCHEMA_VERSION: u32 = 15;
pub(crate) const CURRENT_PREPARE_REVISION: &str = "prepare-items-v1";
pub(crate) const CURRENT_BAKE_REVISION: &str = "rust-cpu-irradiance-v12-seam-stitch";
pub(crate) const STATIC_POINT_SHADOW_REVISION: &str = "bvh-d32-v6";

mod compatibility;

pub(crate) use compatibility::*;

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub(crate) struct PreparedSceneManifest {
    pub(crate) schema_version: u32,
    #[serde(default)]
    pub(crate) prepare_revision: Option<String>,
    #[serde(default)]
    pub(crate) converter_revision: Option<String>,
    #[serde(default)]
    pub(crate) physics_schema_version: Option<u32>,
    pub(crate) asset_root: String,
    pub(crate) source_plugin: String,
    pub(crate) source_fingerprint: String,
    /// Content-fingerprinted item catalogue relative to `asset_root`.
    #[serde(default)]
    pub(crate) item_catalog_path: Option<String>,
    #[serde(default)]
    pub(crate) item_catalog_revision: Option<String>,
    #[serde(default)]
    pub(crate) item_catalog_hash: Option<String>,
    /// Content-fingerprinted recipe catalogue relative to `asset_root`.
    /// Serde defaults keep manifests produced before recipe preparation
    /// readable and explicitly distinguish them from an empty catalogue.
    #[serde(default)]
    pub(crate) recipe_catalog_path: Option<String>,
    #[serde(default)]
    pub(crate) recipe_catalog_revision: Option<String>,
    #[serde(default)]
    pub(crate) recipe_catalog_hash: Option<String>,
    #[serde(default)]
    pub(crate) source_plugins: Vec<PreparedPluginSource>,
    pub(crate) cell: CellInfo,
    pub(crate) placements: Vec<PreparedPlacement>,
    pub(crate) lights: Vec<PreparedLight>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    #[serde(default)]
    pub(crate) visual_issues: Vec<PreparedVisualIssue>,
    #[serde(default)]
    pub(crate) navmeshes: Vec<PreparedNavMeshSource>,
    #[serde(default)]
    pub(crate) cell_audio: PreparedCellAudio,
    #[serde(default)]
    pub(crate) audio_clips: Vec<PreparedAudioClip>,
    #[serde(default)]
    pub(crate) footstep_sets: Vec<PreparedFootstepSet>,
    #[serde(default)]
    pub(crate) hard_landing_clips: Vec<String>,
    #[serde(default)]
    pub(crate) bake: Option<PreparedBake>,
    #[serde(default)]
    pub(crate) static_point_shadows: Option<PreparedStaticPointShadows>,
    /// QA counts of `PreparedRuntimeMutability` classifications across
    /// `placements`, computed once at prepare time (F38.4).
    #[serde(default)]
    pub(crate) mutability_summary: PreparedMutabilitySummary,
    /// Leveled lists (`LVLI`/`LVLN`/`LVLC`) transitively reachable from
    /// prepared container inventory entries flagged `leveled: true`, keyed
    /// by FormID (issue #74, F74.2). Old manifests without this field
    /// deserialize with an empty map.
    #[serde(default)]
    pub(crate) leveled_lists: BTreeMap<u32, PreparedLeveledList>,
}

/// A single prepared leveled list's body: `LVLD` chance-none percentage,
/// `LVLF` flags, and its `LVLO` entries. See
/// `openmw_esm4::records::LeveledListData` for the parsed source shape this
/// mirrors (issue #74).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedLeveledList {
    pub(crate) chance_none: u8,
    pub(crate) flags: u8,
    pub(crate) entries: Vec<PreparedLeveledEntry>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedLeveledEntry {
    pub(crate) level: u16,
    pub(crate) base_form_id: u32,
    pub(crate) count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub(crate) struct PreparedItemCatalog {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    pub(crate) items: Vec<PreparedItemDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedItemDefinition {
    pub(crate) base_form_id: u32,
    pub(crate) record_kind: String,
    pub(crate) category: PreparedItemCategory,
    pub(crate) editor_id: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) source_model_path: Option<String>,
    pub(crate) icon_asset_path: Option<String>,
    pub(crate) world_asset_path: Option<String>,
    pub(crate) physics_asset_path: Option<String>,
    #[serde(default)]
    pub(crate) drop_collider: PreparedDropCollider,
    pub(crate) value: Option<i32>,
    pub(crate) weight: Option<f32>,
    /// Base-record quest-item header flag (issue #81). Serde-default so
    /// wave 1/2 catalogs deserialize as ordinary items.
    #[serde(default)]
    pub(crate) quest_item: bool,
    pub(crate) stats: PreparedItemStats,
    #[serde(default)]
    pub(crate) audio: PreparedPlacementAudio,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) enum PreparedDropCollider {
    Authored,
    BoundsProxy {
        center: [f32; 3],
        half_extents: [f32; 3],
    },
    #[default]
    Missing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum PreparedItemCategory {
    Weapons,
    Apparel,
    Aid,
    Misc,
    Ammo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) enum PreparedItemStats {
    Weapon {
        damage: Option<u16>,
        max_condition: Option<u32>,
        clip_size: Option<u8>,
        speed: Option<f32>,
        reach: Option<f32>,
        /// Issue #98 (F98.1): base form id of the ammo this weapon takes.
        /// Serde-defaulted so wave 1/2 catalogs keep deserializing.
        #[serde(default)]
        ammo_form_id: Option<u32>,
    },
    Apparel {
        armor_rating: Option<f32>,
        max_condition: Option<u32>,
        /// Issue #98 (F98.1): FO3 `ARMO.BMDT` biped-slot mask. Serde-defaulted
        /// so wave 1/2 catalogs keep deserializing.
        #[serde(default)]
        biped_slot_mask: Option<u32>,
    },
    Ammo {
        damage: Option<f32>,
        speed: Option<f32>,
    },
    Aid {
        effects: Vec<PreparedItemEffect>,
    },
    Book {
        flags: Option<u8>,
        text: Option<String>,
    },
    Note {
        text: Option<String>,
    },
    Key,
    #[default]
    Misc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedItemEffect {
    pub(crate) form_id: u32,
    pub(crate) label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedVisualIssue {
    pub(crate) code: String,
    pub(crate) severity: String,
    pub(crate) model_path: String,
    pub(crate) base_form_ids: Vec<u32>,
    pub(crate) reference_form_ids: Vec<u32>,
    pub(crate) message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedStaticPointShadows {
    pub(crate) revision: String,
    pub(crate) source_fingerprint: String,
    pub(crate) asset_path: String,
    pub(crate) resolution: u32,
    pub(crate) near_z: f32,
    pub(crate) lights: Vec<PreparedStaticPointShadowLight>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedStaticPointShadowLight {
    pub(crate) reference_form_id: u32,
    pub(crate) layer: u32,
    pub(crate) translation: [f32; 3],
    pub(crate) range: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedPluginSource {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedBake {
    #[serde(default)]
    pub(crate) bake_revision: Option<String>,
    pub(crate) source_fingerprint: String,
    pub(crate) scene_path: String,
    #[serde(default)]
    pub(crate) irradiance_volume: Option<PreparedIrradianceVolume>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedIrradianceVolume {
    pub(crate) asset_path: String,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: [f32; 3],
    pub(crate) resolution: [u32; 3],
    pub(crate) intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CellInfo {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) interior: bool,
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) directional_rgba: [f32; 4],
    #[serde(default)]
    pub(crate) image_space_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) image_space: Option<ImageSpaceInfo>,
    #[serde(default)]
    pub(crate) lighting_template_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) lighting_template_flags: u32,
    #[serde(default)]
    pub(crate) lighting_template: Option<PreparedLightingTemplate>,
    #[serde(default)]
    pub(crate) raw_lighting: Option<PreparedCellLighting>,
    #[serde(default)]
    pub(crate) effective_lighting: Option<PreparedCellLighting>,
    #[serde(default)]
    pub(crate) water_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) water_height: Option<f32>,
    /// Exterior `XCLC` grid coordinates (`None` for interiors, or exteriors
    /// missing the subrecord). See `openmw_esm4::records::parse_grid`.
    #[serde(default)]
    pub(crate) grid: Option<(i32, i32)>,
    /// FormID of the `WRLD` record this cell was discovered under (a
    /// group-type-1 "world children" GRUP), `None` for interiors.
    #[serde(default)]
    pub(crate) worldspace_form_id: Option<u32>,
}

pub(crate) fn cell_label(cell: &CellInfo) -> String {
    match cell.editor_id.as_deref() {
        Some(editor_id) => format!("{editor_id} ({:08x})", cell.form_id),
        None => format!("{:08x}", cell.form_id),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedLightingTemplate {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) directional_rgba: [f32; 4],
    pub(crate) fog_rgba: [f32; 4],
    pub(crate) fog_near: f32,
    pub(crate) fog_far: f32,
    pub(crate) fog_directional_fade: f32,
    pub(crate) fog_clip_distance: f32,
    pub(crate) fog_power: f32,
    #[serde(default)]
    pub(crate) directional_rotation_xy: i32,
    #[serde(default)]
    pub(crate) directional_rotation_z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub(crate) struct PreparedCellLighting {
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) directional_rgba: [f32; 4],
    pub(crate) fog_rgba: [f32; 4],
    pub(crate) fog_near: f32,
    pub(crate) fog_far: f32,
    pub(crate) directional_rotation_xy: i32,
    pub(crate) directional_rotation_z: i32,
    pub(crate) directional_fade: f32,
    pub(crate) fog_clip_distance: f32,
    pub(crate) fog_power: f32,
}

impl Default for PreparedCellLighting {
    fn default() -> Self {
        Self {
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            fog_rgba: [0.0; 4],
            fog_near: 0.0,
            fog_far: 0.0,
            directional_rotation_xy: 0,
            directional_rotation_z: 0,
            directional_fade: 0.0,
            fog_clip_distance: 0.0,
            fog_power: 1.0,
        }
    }
}

impl PreparedCellLighting {
    pub(crate) fn directional_rotation_xyzw(&self) -> [f32; 4] {
        let rotation = Quat::from_euler(
            EulerRot::YXZ,
            (self.directional_rotation_z as f32).to_radians(),
            (self.directional_rotation_xy as f32).to_radians(),
            0.0,
        );
        rotation.to_array()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct ImageSpaceInfo {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) eye_adapt_speed: f32,
    pub(crate) hdr_blur_radius: f32,
    pub(crate) hdr_blur_passes: f32,
    pub(crate) hdr_emissive_multiplier: f32,
    pub(crate) hdr_target_lum: f32,
    pub(crate) hdr_upper_lum_clamp: f32,
    pub(crate) hdr_bright_scale: f32,
    pub(crate) hdr_bright_clamp: f32,
    pub(crate) hdr_lum_ramp_no_tex: f32,
    pub(crate) hdr_lum_ramp_min: f32,
    pub(crate) hdr_lum_ramp_max: f32,
    pub(crate) hdr_sunlight_dimmer: f32,
    pub(crate) hdr_grass_dimmer: f32,
    pub(crate) hdr_tree_dimmer: f32,
    pub(crate) hdr_skin_dimmer: f32,
    pub(crate) bloom_blur_radius: f32,
    pub(crate) bloom_alpha_mult_interior: f32,
    pub(crate) bloom_alpha_mult_exterior: f32,
    pub(crate) get_hit_blur_radius: f32,
    pub(crate) get_hit_blur_damping_constant: f32,
    pub(crate) get_hit_damping_constant: f32,
    pub(crate) night_eye_tint_rgb: [f32; 3],
    pub(crate) brightness: f32,
    pub(crate) cinematic_saturation: f32,
    pub(crate) cinematic_contrast_avg_lum: f32,
    pub(crate) cinematic_contrast: f32,
    pub(crate) cinematic_brightness_tint_rgb: [f32; 3],
    pub(crate) cinematic_brightness_tint_value: f32,
    pub(crate) flags: u8,
}

impl Default for ImageSpaceInfo {
    fn default() -> Self {
        Self {
            form_id: 0,
            editor_id: None,
            eye_adapt_speed: 0.5,
            hdr_blur_radius: 7.0,
            hdr_blur_passes: 1.0,
            hdr_emissive_multiplier: 1.0,
            hdr_target_lum: 1.0,
            hdr_upper_lum_clamp: 1.0,
            hdr_bright_scale: 1.0,
            hdr_bright_clamp: 0.225,
            hdr_lum_ramp_no_tex: 1.0,
            hdr_lum_ramp_min: 0.0,
            hdr_lum_ramp_max: 0.0,
            hdr_sunlight_dimmer: 1.0,
            hdr_grass_dimmer: 1.0,
            hdr_tree_dimmer: 1.0,
            hdr_skin_dimmer: 1.0,
            bloom_blur_radius: 0.0,
            bloom_alpha_mult_interior: 1.0,
            bloom_alpha_mult_exterior: 1.0,
            get_hit_blur_radius: 0.0,
            get_hit_blur_damping_constant: 0.0,
            get_hit_damping_constant: 0.0,
            night_eye_tint_rgb: [0.0, 0.0, 0.0],
            brightness: 1.0,
            cinematic_saturation: 1.0,
            cinematic_contrast_avg_lum: 0.5,
            cinematic_contrast: 1.0,
            cinematic_brightness_tint_rgb: [1.0, 1.0, 1.0],
            cinematic_brightness_tint_value: 0.0,
            flags: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedPlacement {
    pub(crate) reference_form_id: u32,
    pub(crate) base_form_id: u32,
    pub(crate) asset_path: Option<String>,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: f32,
    pub(crate) error: Option<String>,
    #[serde(default)]
    pub(crate) physics_asset_path: Option<String>,
    #[serde(default)]
    pub(crate) physics_source: Option<PreparedPhysicsSource>,
    #[serde(default)]
    pub(crate) physics_classification: PreparedPhysicsClassification,
    #[serde(default)]
    pub(crate) step_support: bool,
    /// Conservative runtime mutability classification (F38.1). Manifests
    /// prepared before this field existed round-trip as `Unknown`, never
    /// `Immutable` (F38.3).
    #[serde(default)]
    pub(crate) mutability: PreparedRuntimeMutability,
    /// Shared "visibility root" FormID for `EnableGroup` placements: the
    /// top-most ancestor of this reference's enable-parent chain (F38.3,
    /// F38.5). `None` for every other classification.
    #[serde(default)]
    pub(crate) mutability_root_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) reference_kind: String,
    #[serde(default)]
    pub(crate) base_kind: String,
    #[serde(default)]
    pub(crate) editor_id: Option<String>,
    #[serde(default)]
    pub(crate) display_name: Option<String>,
    #[serde(default = "default_count")]
    pub(crate) count: i32,
    #[serde(default)]
    pub(crate) semantic: PreparedSemantic,
    #[serde(default = "default_true")]
    pub(crate) initially_enabled: bool,
    #[serde(default)]
    pub(crate) enable_parent: Option<PreparedEnableParent>,
    #[serde(default)]
    pub(crate) owner_form_id: Option<u32>,
    #[serde(default)]
    pub(crate) owner_faction_rank: Option<i32>,
    #[serde(default)]
    pub(crate) inventory: Vec<PreparedInventoryEntry>,
    #[serde(default)]
    pub(crate) audio: PreparedPlacementAudio,
    /// Conversion profile used for the source mesh's optional quick AO pass.
    /// The alias keeps older manifests readable.
    #[serde(default = "default_ao_mode", alias = "vertex_color_mode")]
    pub(crate) ao_mode: String,
}

/// Record kinds that represent world-loot items rather than scenery. Keep
/// this shared by irradiance and point-shadow preparation so a pickup cannot
/// leak into one baked lighting path while being excluded from the other.
pub(crate) fn is_pickup_record_kind(kind: &str) -> bool {
    [
        "WEAP", "AMMO", "ARMO", "ALCH", "MISC", "BOOK", "NOTE", "KEYM",
    ]
    .iter()
    .any(|candidate| kind.eq_ignore_ascii_case(candidate))
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum PreparedPhysicsClassification {
    #[default]
    Static,
    Kinematic,
    Dynamic,
}

/// Conservative classification of how a prepared placement may change at
/// runtime, used later to address content from `PersistentWorldState`.
///
/// Classification is deliberately conservative: anything uncertain lands in
/// `Unknown`, never silently `Immutable` (see `classify_runtime_mutability`
/// in `vsa::prepare::placements`).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum PreparedRuntimeMutability {
    /// Static scenery: no enable-parent chain, no known script reachability.
    Immutable,
    /// Part of an XESP enable-parent chain; toggled as a group. The chain's
    /// root FormID is preserved on the placement (`mutability_root_form_id`).
    EnableGroup,
    /// A record kind commonly reachable/mutated by game scripts (doors,
    /// activators, containers, furniture, actors, pickups).
    ScriptAddressable,
    /// Uncertain: an unrecognized record kind, a placement that failed to
    /// resolve, or an enable-parent chain that could not be resolved.
    #[default]
    Unknown,
}

/// QA counts of `PreparedRuntimeMutability` classifications across a
/// prepared scene's placements (F38.4).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedMutabilitySummary {
    pub(crate) immutable: usize,
    pub(crate) enable_group: usize,
    pub(crate) script_addressable: usize,
    pub(crate) unknown: usize,
}

fn default_count() -> i32 {
    1
}

fn default_true() -> bool {
    true
}

fn default_ao_mode() -> String {
    "ao-none".into()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedEnableParent {
    pub(crate) reference_form_id: u32,
    pub(crate) inverted: bool,
    pub(crate) pop_in: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedInventoryEntry {
    pub(crate) base_form_id: u32,
    pub(crate) count: i32,
    pub(crate) record_kind: String,
    pub(crate) editor_id: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) leveled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedPlacementAudio {
    pub(crate) loop_sound_form_id: Option<u32>,
    pub(crate) activate_sound_form_id: Option<u32>,
    pub(crate) open_sound_form_id: Option<u32>,
    pub(crate) close_sound_form_id: Option<u32>,
    pub(crate) pickup_sound_form_id: Option<u32>,
    pub(crate) drop_sound_form_id: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedCellAudio {
    pub(crate) acoustic_space_form_id: Option<u32>,
    pub(crate) acoustic_environment_type: Option<u32>,
    pub(crate) ambient_loop_sound_form_ids: Vec<u32>,
    pub(crate) sound_region_form_id: Option<u32>,
    pub(crate) music_form_id: Option<u32>,
    pub(crate) music_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedAudioClip {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) source_path: String,
    pub(crate) asset_path: Option<String>,
    pub(crate) flags: u16,
    pub(crate) min_attenuation: u8,
    pub(crate) max_attenuation: u8,
    pub(crate) frequency_adjustment: i8,
    pub(crate) static_attenuation_hundredths_db: u16,
    pub(crate) looping: bool,
    pub(crate) is_2d: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedFootstepSet {
    pub(crate) surface: String,
    pub(crate) left: Vec<String>,
    pub(crate) right: Vec<String>,
    #[serde(default)]
    pub(crate) land: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) enum PreparedSemantic {
    #[default]
    Static,
    Pickup(PreparedPickup),
    Container,
    /// A staged actor corpse is a loot holder, not an actor-simulation
    /// state. Its stable identity is the placement's reference FormID and
    /// its contents ride the same prepared inventory/runtime state as a
    /// container. Actor death and conversion into this semantic remain
    /// outside this slice.
    Corpse,
    Door(PreparedDoor),
    Activator,
    Furniture,
    Npc(PreparedActor),
    Creature(PreparedActor),
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedPickup {
    pub(crate) category: String,
    pub(crate) value: Option<i32>,
    pub(crate) weight: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedActor {
    pub(crate) base_template_form_id: Option<u32>,
}

impl PreparedSemantic {
    /// Whether this placement owns a transfer-able, FormID-keyed loot
    /// inventory. Kept on the manifest semantic so persistence and runtime
    /// activation cannot accidentally diverge on the corpse/container set.
    pub(crate) fn is_loot_holder(&self) -> bool {
        matches!(self, Self::Container | Self::Corpse)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedDoor {
    pub(crate) lock_level: Option<i8>,
    pub(crate) key_form_id: Option<u32>,
    pub(crate) destination: Option<PreparedDoorDestination>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct PreparedDoorDestination {
    pub(crate) door_reference_form_id: u32,
    pub(crate) cell_form_id: u32,
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedNavMeshSource {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) version: Option<u32>,
    pub(crate) asset_path: String,
    pub(crate) chunks: Vec<PreparedNavMeshChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedNavMeshChunk {
    pub(crate) signature: String,
    pub(crate) byte_len: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedLight {
    #[serde(default)]
    pub(crate) reference_form_id: u32,
    #[serde(default)]
    pub(crate) base_form_id: u32,
    pub(crate) translation: [f32; 3],
    #[serde(default = "default_rotation")]
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) color_rgba: [f32; 4],
    pub(crate) radius: f32,
    #[serde(default)]
    pub(crate) intensity_lumens: f32,
    #[serde(default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) flags: u32,
    #[serde(default = "default_true")]
    pub(crate) initially_enabled: bool,
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Diagnostic {
    pub(crate) severity: String,
    pub(crate) message: String,
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
