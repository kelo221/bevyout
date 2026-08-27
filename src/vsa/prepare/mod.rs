use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

mod actor_animation;
mod actor_animation_cache;
mod actor_appearance;
mod actor_catalog;
mod audio;
mod audio_resolve;
mod batch_cache;
mod cache;
mod container_audio;
mod container_audio_policy;
mod dialogue_discovery;
mod exterior_scene_policy;
mod facegen;
mod fingerprints;
mod gmst_catalog;
mod imad;
mod image_space;
mod interface;
mod items;
mod jobs;
mod native;
mod native_hair;
mod native_policy;
mod nav_clearance;
mod nav_clip;
mod nav_doors;
mod nav_graph;
mod navmesh;
mod package_catalog;
mod placements;
mod plugins;
mod recipes;
mod reflection_probe_distribution;
mod reflection_probe_policy;
mod reflection_probes;
mod selectors;
mod session;
mod static_shadows;
mod visual;
mod worldspace_lod;

pub(crate) use actor_animation::*;
pub(crate) use actor_animation_cache::*;
pub(crate) use actor_appearance::*;
pub(crate) use actor_catalog::*;
pub(crate) use audio::*;
pub(crate) use audio_resolve::*;
pub(crate) use batch_cache::*;
pub(crate) use cache::*;
pub(crate) use container_audio::*;
pub(crate) use container_audio_policy::*;
pub(crate) use dialogue_discovery::*;
pub(crate) use exterior_scene_policy::*;
pub(crate) use facegen::*;
pub(crate) use fingerprints::*;
pub(crate) use gmst_catalog::*;
pub(crate) use imad::*;
pub(crate) use image_space::*;
pub(crate) use interface::*;
pub(crate) use items::*;
pub(crate) use jobs::*;
pub(crate) use native::*;
pub(crate) use native_policy::*;
pub(crate) use nav_clearance::*;
pub(crate) use nav_doors::*;
pub(crate) use nav_graph::*;
pub(crate) use navmesh::*;
pub(crate) use package_catalog::*;
pub(crate) use placements::*;
pub(crate) use plugins::*;
pub(crate) use recipes::*;
pub(crate) use reflection_probe_distribution::*;
pub(crate) use reflection_probe_policy::*;
pub(crate) use reflection_probes::*;
pub(crate) use selectors::*;
pub(crate) use session::*;
pub(crate) use static_shadows::*;
pub(crate) use visual::*;
pub(crate) use worldspace_lod::*;

mod orchestrator;

pub use orchestrator::prepare;

use super::assets::{
    ActorAnimationClipJob, ActorAnimationPackJob, ActorAnimationPackReport, ActorApparelInput,
    ActorAssemblyDescriptor, ActorBodyPartInput, ActorFaceGenDescriptor, AssetConversion, AssetJob,
    AssetJobKind, NATIVE_ACTOR_CONVERTER_REVISION, NATIVE_NIF_CONVERTER_REVISION,
    NATIVE_PREPARED_CONVERTER_REVISION, RootTransformPolicy, actor_animation_pack_fingerprint,
    asset_conversion, audit_glb_visuals, canonical_actor_assembly, content_addressed_glb_name,
    convert_staged_textures, load_archives, read_actor_animation_report,
    read_glb_animation_sound_cues, resolve_asset, root_transform_policy,
    run_native_actor_animation_batch, stage_textures, validate_actor_animation_glb,
    validate_actor_glb, validate_asset_cache_pair, validate_glb_images,
};
use super::audio_assets::{load_audio_archives, resolve_audio_asset, stage_audio_asset};
use super::manifest::{
    CURRENT_MANIFEST_SCHEMA_VERSION, CURRENT_PREPARE_REVISION, Diagnostic, PreparedActor,
    PreparedAudioClip, PreparedCellAudio, PreparedCellLighting, PreparedDayNightProfile,
    PreparedDayNightProfileSource, PreparedDoor, PreparedDoorDestination, PreparedDropCollider,
    PreparedEnableParent, PreparedInventoryEntry, PreparedItemCatalog, PreparedItemCategory,
    PreparedItemDefinition, PreparedItemEffect, PreparedItemStats, PreparedLight,
    PreparedLightingTemplate, PreparedMutabilitySummary, PreparedNavGraphSource,
    PreparedNavMeshChunk, PreparedNavMeshSource, PreparedPhysicsClassification, PreparedPickup,
    PreparedPlacement, PreparedPlacementAudio, PreparedPluginSource, PreparedReflectionProbe,
    PreparedReflectionProbeSet, PreparedRuntimeMutability, PreparedSceneManifest, PreparedSemantic,
    PreparedStaticPointShadowLight, PreparedStaticPointShadows, PreparedVisualIssue,
    REFLECTION_PROBE_REVISION, STATIC_POINT_SHADOW_REVISION,
};
use super::openmw_esm4::{
    ClimateRecord, LightingData, OpenMwItemStats, ParsedContentSet, WeatherRecord, WorldspaceRecord,
};
use super::paths::{
    FO3_SCALE, absolutize, fingerprint, is_editor_marker, is_non_rendering_effect,
    normalize_asset_path, parse_cell_selector, placement_transform, placement_transform_parts,
};
use super::physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, PreparedPhysicsAsset, classify_placement, dynamic_proxy_bounds,
    dynamic_rejection_reason, physics_sidecar_name, read_physics_asset,
};
#[cfg(test)]
pub(crate) use super::plugin::parse_content_set;
use super::plugin::{
    ActorBaseConfig, BaseRecord, ParsedPlugin, PluginSource, RECORD_DELETED, RECORD_DISABLED,
    RecipeItemRecord, RecipeRecord, ReferenceKind, ReferenceRecord, SoundRecord,
    SoundReferenceRecord, read_master_names,
};
use crate::cli::PrepareArgs;

fn prepared_lighting(lighting: LightingData) -> PreparedCellLighting {
    PreparedCellLighting {
        ambient_rgba: lighting.ambient_rgba,
        directional_rgba: lighting.directional_rgba,
        fog_rgba: lighting.fog_rgba,
        fog_near: lighting.fog_near,
        fog_far: lighting.fog_far,
        directional_rotation_xy: lighting.rotation_xy,
        directional_rotation_z: lighting.rotation_z,
        directional_fade: lighting.fog_directional_fade,
        fog_clip_distance: lighting.fog_clip_distance,
        fog_power: lighting.fog_power,
    }
}

fn legacy_lighting(cell: &super::manifest::CellInfo) -> LightingData {
    LightingData {
        ambient_rgba: cell.ambient_rgba,
        directional_rgba: cell.directional_rgba,
        fog_rgba: [0.0; 4],
        fog_near: 0.0,
        fog_far: 0.0,
        rotation_xy: 0,
        rotation_z: 0,
        fog_directional_fade: 0.0,
        fog_clip_distance: 0.0,
        fog_power: 1.0,
    }
}

#[derive(Debug)]
pub(crate) struct LoadedPlugin {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssetCacheDecision {
    Reuse,
    BuildMissing,
    RebuildInvalid,
    RebuildRequested,
}

pub(crate) struct AudioDescriptor {
    source_path: String,
    editor_id: Option<String>,
    flags: u16,
    min_attenuation: u8,
    max_attenuation: u8,
    frequency_adjustment: i8,
    static_attenuation_hundredths_db: u16,
    looping: bool,
    is_2d: bool,
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
