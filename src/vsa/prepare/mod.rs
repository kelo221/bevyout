use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

mod audio;
mod audio_resolve;
mod cache;
mod image_space;
mod navmesh;
mod placements;
mod plugins;
mod static_shadows;

pub(crate) use audio::*;
pub(crate) use audio_resolve::*;
pub(crate) use cache::*;
pub(crate) use image_space::*;
pub(crate) use navmesh::*;
pub(crate) use placements::*;
pub(crate) use plugins::*;
pub(crate) use static_shadows::*;

mod orchestrator;

pub use orchestrator::prepare;

use super::assets::{
    BlenderAssetJob, NIF_CONVERTER_REVISION, asset_conversion, content_addressed_glb_name,
    convert_staged_textures, find_blender, load_archives, resolve_asset, run_blender_batch,
    stage_textures, validate_asset_cache_pair,
};
use super::audio_assets::{load_audio_archives, resolve_audio_asset, stage_audio_asset};
use super::manifest::{
    CURRENT_MANIFEST_SCHEMA_VERSION, CURRENT_PREPARE_REVISION, Diagnostic, PreparedActor,
    PreparedAudioClip, PreparedCellAudio, PreparedCellLighting, PreparedDoor,
    PreparedDoorDestination, PreparedEnableParent, PreparedInventoryEntry, PreparedLight,
    PreparedLightingTemplate, PreparedMutabilitySummary, PreparedNavMeshChunk,
    PreparedNavMeshSource, PreparedPhysicsClassification, PreparedPickup, PreparedPlacement,
    PreparedPlacementAudio, PreparedPluginSource, PreparedRuntimeMutability, PreparedSceneManifest,
    PreparedSemantic, PreparedStaticPointShadowLight, PreparedStaticPointShadows,
    STATIC_POINT_SHADOW_REVISION,
};
use super::openmw_esm4::LightingData;
use super::paths::{
    FO3_SCALE, absolutize, fingerprint, is_editor_marker, is_non_rendering_effect,
    normalize_asset_path, parse_cell_selector, placement_transform, placement_transform_parts,
};
use super::physics::{
    PHYSICS_ASSET_SCHEMA_VERSION, classify_placement, dynamic_rejection_reason,
    physics_sidecar_name, read_physics_asset,
};
use super::plugin::{
    BaseRecord, ParsedPlugin, PluginSource, RECORD_DELETED, RECORD_DISABLED, ReferenceKind,
    ReferenceRecord, SoundRecord, SoundReferenceRecord, parse_content_set, read_master_names,
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
