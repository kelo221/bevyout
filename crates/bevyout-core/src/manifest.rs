use std::collections::BTreeMap;

use glam::{EulerRot, Quat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum PreparedPhysicsSource {
    AuthoredHavok,
    #[default]
    GeneratedRender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedSceneManifest {
    pub schema_version: u32,
    #[serde(default)]
    pub prepare_revision: Option<String>,
    #[serde(default)]
    pub converter_revision: Option<String>,
    #[serde(default)]
    pub physics_schema_version: Option<u32>,
    pub asset_root: String,
    pub source_plugin: String,
    pub source_fingerprint: String,
    /// Content-fingerprinted item catalogue relative to `asset_root`.
    #[serde(default)]
    pub item_catalog_path: Option<String>,
    #[serde(default)]
    pub item_catalog_revision: Option<String>,
    #[serde(default)]
    pub item_catalog_hash: Option<String>,
    /// Content-fingerprinted recipe catalogue relative to `asset_root`.
    /// Serde defaults keep manifests produced before recipe preparation
    /// readable and explicitly distinguish them from an empty catalogue.
    #[serde(default)]
    pub recipe_catalog_path: Option<String>,
    #[serde(default)]
    pub recipe_catalog_revision: Option<String>,
    #[serde(default)]
    pub recipe_catalog_hash: Option<String>,
    /// Per-cell actor catalogue relative to `asset_root`
    /// (`scenes/<cell>/actors.ron`, issue #103, M4 wave 1 task C). Unlike
    /// the content-set-wide item/recipe catalogs it embeds this cell's
    /// ACHR/ACRE placements, so it lives next to `scene.ron` rather than
    /// under the shared fingerprint-keyed `catalogs/` directory. Serde
    /// defaults keep manifests produced before actor-catalog preparation
    /// readable.
    #[serde(default)]
    pub actor_catalog_path: Option<String>,
    #[serde(default)]
    pub actor_catalog_revision: Option<String>,
    #[serde(default)]
    pub actor_catalog_hash: Option<String>,
    #[serde(default)]
    pub source_plugins: Vec<PreparedPluginSource>,
    pub cell: CellInfo,
    pub placements: Vec<PreparedPlacement>,
    pub lights: Vec<PreparedLight>,
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default)]
    pub visual_issues: Vec<PreparedVisualIssue>,
    #[serde(default)]
    pub navmeshes: Vec<PreparedNavMeshSource>,
    /// Decoded per-cell polygon navigation graph (issue #111, M4 wave 2).
    /// `None` only for manifests prepared before this wave (serde default)
    /// or when the cell has no NAVM records at all.
    #[serde(default)]
    pub nav_graph: Option<PreparedNavGraphSource>,
    #[serde(default)]
    pub cell_audio: PreparedCellAudio,
    #[serde(default)]
    pub audio_clips: Vec<PreparedAudioClip>,
    #[serde(default)]
    pub footstep_sets: Vec<PreparedFootstepSet>,
    #[serde(default)]
    pub hard_landing_clips: Vec<String>,
    #[serde(default)]
    pub bake: Option<PreparedBake>,
    #[serde(default)]
    pub static_point_shadows: Option<PreparedStaticPointShadows>,
    /// QA counts of `PreparedRuntimeMutability` classifications across
    /// `placements`, computed once at prepare time (F38.4).
    #[serde(default)]
    pub mutability_summary: PreparedMutabilitySummary,
    /// Leveled lists (`LVLI`/`LVLN`/`LVLC`) transitively reachable from
    /// prepared container inventory entries flagged `leveled: true`, keyed
    /// by FormID (issue #74, F74.2). Old manifests without this field
    /// deserialize with an empty map.
    #[serde(default)]
    pub leveled_lists: BTreeMap<u32, PreparedLeveledList>,
}

/// A single prepared leveled list's body: `LVLD` chance-none percentage,
/// `LVLF` flags, and its `LVLO` entries. See
/// `openmw_esm4::records::LeveledListData` for the parsed source shape this
/// mirrors (issue #74).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedLeveledList {
    pub chance_none: u8,
    pub flags: u8,
    pub entries: Vec<PreparedLeveledEntry>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedLeveledEntry {
    pub level: u16,
    pub base_form_id: u32,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedItemDefinition {
    pub base_form_id: u32,
    pub record_kind: String,
    pub category: PreparedItemCategory,
    pub editor_id: Option<String>,
    pub display_name: Option<String>,
    pub source_model_path: Option<String>,
    pub icon_asset_path: Option<String>,
    pub world_asset_path: Option<String>,
    pub physics_asset_path: Option<String>,
    #[serde(default)]
    pub drop_collider: PreparedDropCollider,
    pub value: Option<i32>,
    pub weight: Option<f32>,
    /// Base-record quest-item header flag (issue #81). Serde-default so
    /// wave 1/2 catalogs deserialize as ordinary items.
    #[serde(default)]
    pub quest_item: bool,
    pub stats: PreparedItemStats,
    #[serde(default)]
    pub audio: PreparedPlacementAudio,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum PreparedDropCollider {
    Authored,
    BoundsProxy {
        center: [f32; 3],
        half_extents: [f32; 3],
    },
    #[default]
    Missing,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PreparedItemCategory {
    Weapons,
    Apparel,
    Aid,
    Misc,
    Ammo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum PreparedItemStats {
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
pub struct PreparedItemEffect {
    pub form_id: u32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedVisualIssue {
    pub code: String,
    pub severity: String,
    pub model_path: String,
    pub base_form_ids: Vec<u32>,
    pub reference_form_ids: Vec<u32>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedStaticPointShadows {
    pub revision: String,
    pub source_fingerprint: String,
    pub asset_path: String,
    pub resolution: u32,
    pub near_z: f32,
    pub lights: Vec<PreparedStaticPointShadowLight>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedStaticPointShadowLight {
    pub reference_form_id: u32,
    pub layer: u32,
    pub translation: [f32; 3],
    pub range: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedPluginSource {
    pub name: String,
    pub path: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedBake {
    #[serde(default)]
    pub bake_revision: Option<String>,
    pub source_fingerprint: String,
    pub scene_path: String,
    #[serde(default)]
    pub irradiance_volume: Option<PreparedIrradianceVolume>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedIrradianceVolume {
    pub asset_path: String,
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: [f32; 3],
    pub resolution: [u32; 3],
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellInfo {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    pub interior: bool,
    pub ambient_rgba: [f32; 4],
    pub directional_rgba: [f32; 4],
    #[serde(default)]
    pub image_space_form_id: Option<u32>,
    #[serde(default)]
    pub image_space: Option<ImageSpaceInfo>,
    #[serde(default)]
    pub lighting_template_form_id: Option<u32>,
    #[serde(default)]
    pub lighting_template_flags: u32,
    #[serde(default)]
    pub lighting_template: Option<PreparedLightingTemplate>,
    #[serde(default)]
    pub raw_lighting: Option<PreparedCellLighting>,
    #[serde(default)]
    pub effective_lighting: Option<PreparedCellLighting>,
    #[serde(default)]
    pub water_form_id: Option<u32>,
    #[serde(default)]
    pub water_height: Option<f32>,
    /// Exterior `XCLC` grid coordinates (`None` for interiors, or exteriors
    /// missing the subrecord). See `openmw_esm4::records::parse_grid`.
    #[serde(default)]
    pub grid: Option<(i32, i32)>,
    /// FormID of the `WRLD` record this cell was discovered under (a
    /// group-type-1 "world children" GRUP), `None` for interiors.
    #[serde(default)]
    pub worldspace_form_id: Option<u32>,
}

pub fn cell_label(cell: &CellInfo) -> String {
    match cell.editor_id.as_deref() {
        Some(editor_id) => format!("{editor_id} ({:08x})", cell.form_id),
        None => format!("{:08x}", cell.form_id),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedLightingTemplate {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub ambient_rgba: [f32; 4],
    pub directional_rgba: [f32; 4],
    pub fog_rgba: [f32; 4],
    pub fog_near: f32,
    pub fog_far: f32,
    pub fog_directional_fade: f32,
    pub fog_clip_distance: f32,
    pub fog_power: f32,
    #[serde(default)]
    pub directional_rotation_xy: i32,
    #[serde(default)]
    pub directional_rotation_z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PreparedCellLighting {
    pub ambient_rgba: [f32; 4],
    pub directional_rgba: [f32; 4],
    pub fog_rgba: [f32; 4],
    pub fog_near: f32,
    pub fog_far: f32,
    pub directional_rotation_xy: i32,
    pub directional_rotation_z: i32,
    pub directional_fade: f32,
    pub fog_clip_distance: f32,
    pub fog_power: f32,
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
    pub fn directional_rotation_xyzw(&self) -> [f32; 4] {
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
pub struct ImageSpaceInfo {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub eye_adapt_speed: f32,
    pub hdr_blur_radius: f32,
    pub hdr_blur_passes: f32,
    pub hdr_emissive_multiplier: f32,
    pub hdr_target_lum: f32,
    pub hdr_upper_lum_clamp: f32,
    pub hdr_bright_scale: f32,
    pub hdr_bright_clamp: f32,
    pub hdr_lum_ramp_no_tex: f32,
    pub hdr_lum_ramp_min: f32,
    pub hdr_lum_ramp_max: f32,
    pub hdr_sunlight_dimmer: f32,
    pub hdr_grass_dimmer: f32,
    pub hdr_tree_dimmer: f32,
    pub hdr_skin_dimmer: f32,
    pub bloom_blur_radius: f32,
    pub bloom_alpha_mult_interior: f32,
    pub bloom_alpha_mult_exterior: f32,
    pub get_hit_blur_radius: f32,
    pub get_hit_blur_damping_constant: f32,
    pub get_hit_damping_constant: f32,
    pub night_eye_tint_rgb: [f32; 3],
    pub brightness: f32,
    pub cinematic_saturation: f32,
    pub cinematic_contrast_avg_lum: f32,
    pub cinematic_contrast: f32,
    pub cinematic_brightness_tint_rgb: [f32; 3],
    pub cinematic_brightness_tint_value: f32,
    pub flags: u8,
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
pub struct PreparedPlacement {
    pub reference_form_id: u32,
    pub base_form_id: u32,
    pub asset_path: Option<String>,
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: f32,
    pub error: Option<String>,
    #[serde(default)]
    pub physics_asset_path: Option<String>,
    #[serde(default)]
    pub physics_source: Option<PreparedPhysicsSource>,
    #[serde(default)]
    pub physics_classification: PreparedPhysicsClassification,
    #[serde(default)]
    pub step_support: bool,
    /// Conservative runtime mutability classification (F38.1). Manifests
    /// prepared before this field existed round-trip as `Unknown`, never
    /// `Immutable` (F38.3).
    #[serde(default)]
    pub mutability: PreparedRuntimeMutability,
    /// Shared "visibility root" FormID for `EnableGroup` placements: the
    /// top-most ancestor of this reference's enable-parent chain (F38.3,
    /// F38.5). `None` for every other classification.
    #[serde(default)]
    pub mutability_root_form_id: Option<u32>,
    #[serde(default)]
    pub reference_kind: String,
    #[serde(default)]
    pub base_kind: String,
    #[serde(default)]
    pub editor_id: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default = "default_count")]
    pub count: i32,
    #[serde(default)]
    pub semantic: PreparedSemantic,
    #[serde(default = "default_true")]
    pub initially_enabled: bool,
    #[serde(default)]
    pub enable_parent: Option<PreparedEnableParent>,
    #[serde(default)]
    pub owner_form_id: Option<u32>,
    #[serde(default)]
    pub owner_faction_rank: Option<i32>,
    #[serde(default)]
    pub inventory: Vec<PreparedInventoryEntry>,
    #[serde(default)]
    pub audio: PreparedPlacementAudio,
    /// Conversion profile used for the source mesh's optional quick AO pass.
    /// The alias keeps older manifests readable.
    #[serde(default = "default_ao_mode", alias = "vertex_color_mode")]
    pub ao_mode: String,
}

/// Record kinds that represent world-loot items rather than scenery. Keep
/// this shared by irradiance and point-shadow preparation so a pickup cannot
/// leak into one baked lighting path while being excluded from the other.
pub fn is_pickup_record_kind(kind: &str) -> bool {
    [
        "WEAP", "AMMO", "ARMO", "ALCH", "MISC", "BOOK", "NOTE", "KEYM",
    ]
    .iter()
    .any(|candidate| kind.eq_ignore_ascii_case(candidate))
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum PreparedPhysicsClassification {
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
pub enum PreparedRuntimeMutability {
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
pub struct PreparedMutabilitySummary {
    pub immutable: usize,
    pub enable_group: usize,
    pub script_addressable: usize,
    pub unknown: usize,
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
pub struct PreparedEnableParent {
    pub reference_form_id: u32,
    pub inverted: bool,
    pub pop_in: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedInventoryEntry {
    pub base_form_id: u32,
    pub count: i32,
    pub record_kind: String,
    pub editor_id: Option<String>,
    pub display_name: Option<String>,
    pub leveled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedPlacementAudio {
    pub loop_sound_form_id: Option<u32>,
    pub activate_sound_form_id: Option<u32>,
    pub open_sound_form_id: Option<u32>,
    pub close_sound_form_id: Option<u32>,
    pub pickup_sound_form_id: Option<u32>,
    pub drop_sound_form_id: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedCellAudio {
    pub acoustic_space_form_id: Option<u32>,
    pub acoustic_environment_type: Option<u32>,
    pub ambient_loop_sound_form_ids: Vec<u32>,
    pub sound_region_form_id: Option<u32>,
    pub music_form_id: Option<u32>,
    pub music_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedAudioClip {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub source_path: String,
    pub asset_path: Option<String>,
    pub flags: u16,
    pub min_attenuation: u8,
    pub max_attenuation: u8,
    pub frequency_adjustment: i8,
    pub static_attenuation_hundredths_db: u16,
    pub looping: bool,
    pub is_2d: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedFootstepSet {
    pub surface: String,
    pub left: Vec<String>,
    pub right: Vec<String>,
    #[serde(default)]
    pub land: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum PreparedSemantic {
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
pub struct PreparedPickup {
    pub category: String,
    pub value: Option<i32>,
    pub weight: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedActor {
    pub base_template_form_id: Option<u32>,
}

impl PreparedSemantic {
    /// Whether this placement owns a transfer-able, FormID-keyed loot
    /// inventory. Kept on the manifest semantic so persistence and runtime
    /// activation cannot accidentally diverge on the corpse/container set.
    pub fn is_loot_holder(&self) -> bool {
        matches!(self, Self::Container | Self::Corpse)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedDoor {
    pub lock_level: Option<i8>,
    pub key_form_id: Option<u32>,
    pub destination: Option<PreparedDoorDestination>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreparedDoorDestination {
    pub door_reference_form_id: u32,
    pub cell_form_id: u32,
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedNavMeshSource {
    pub form_id: u32,
    pub record_flags: u32,
    pub version: Option<u32>,
    pub asset_path: String,
    pub chunks: Vec<PreparedNavMeshChunk>,
}

/// Pointer to the decoded per-cell navigation-graph asset
/// (`scenes/<cell>/navmesh/navgraph.ron`, issue #111, M4 wave 2). The
/// viewer-side consumers (#112/#113) need only this asset -- never the raw
/// ESM bytes -- so the counts/diagnostic summary here are QA metadata, not
/// runtime inputs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PreparedNavGraphSource {
    pub asset_path: String,
    pub revision: String,
    pub hash: String,
    pub mesh_count: usize,
    pub polygon_count: usize,
    pub vertex_count: usize,
    pub door_count: usize,
    pub external_connection_count: usize,
    /// Same-cell cross-mesh connections derived from boundary geometry
    /// (issue #113, M4 wave 4); see `PreparedNavMeshMerge`.
    #[serde(default)]
    pub mesh_merge_count: usize,
    pub diagnostics_warning: usize,
    pub diagnostics_error: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreparedNavMeshChunk {
    pub signature: String,
    pub byte_len: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedLight {
    #[serde(default)]
    pub reference_form_id: u32,
    #[serde(default)]
    pub base_form_id: u32,
    pub translation: [f32; 3],
    #[serde(default = "default_rotation")]
    pub rotation_xyzw: [f32; 4],
    pub color_rgba: [f32; 4],
    pub radius: f32,
    #[serde(default)]
    pub intensity_lumens: f32,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub flags: u32,
    #[serde(default = "default_true")]
    pub initially_enabled: bool,
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: String,
    pub message: String,
}
