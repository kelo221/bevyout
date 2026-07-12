use bevy::math::{EulerRot, Quat};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub(crate) struct PreparedSceneManifest {
    pub(crate) schema_version: u32,
    pub(crate) asset_root: String,
    pub(crate) source_plugin: String,
    pub(crate) source_fingerprint: String,
    #[serde(default)]
    pub(crate) source_plugins: Vec<PreparedPluginSource>,
    pub(crate) cell: CellInfo,
    pub(crate) placements: Vec<PreparedPlacement>,
    pub(crate) lights: Vec<PreparedLight>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    #[serde(default)]
    pub(crate) navmeshes: Vec<PreparedNavMeshSource>,
    #[serde(default)]
    pub(crate) cell_audio: PreparedCellAudio,
    #[serde(default)]
    pub(crate) audio_clips: Vec<PreparedAudioClip>,
    #[serde(default)]
    pub(crate) bake: Option<PreparedBake>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparedPluginSource {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedBake {
    pub(crate) source_fingerprint: String,
    pub(crate) scene_path: String,
    pub(crate) lightmaps: Vec<PreparedLightmapPage>,
    pub(crate) bindings: Vec<PreparedLightmapBinding>,
    pub(crate) lightmap_exposure: f32,
    /// Quick/direct bakes still need the runtime ambient and point lights to
    /// provide the missing indirect fill. Final bakes can disable them on
    /// lightmapped meshes because they contain the complete transport.
    #[serde(default)]
    pub(crate) runtime_lighting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedLightmapPage {
    pub(crate) asset_path: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PreparedLightmapBinding {
    pub(crate) mesh_name: String,
    pub(crate) page: usize,
    pub(crate) uv_rect: [f32; 4],
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
}

fn default_count() -> i32 {
    1
}

fn default_true() -> bool {
    true
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub(crate) enum PreparedSemantic {
    #[default]
    Static,
    Pickup(PreparedPickup),
    Container,
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
mod tests {
    use super::*;

    #[test]
    fn schema_one_through_four_manifests_remain_readable_without_new_metadata() {
        let template = r#"(
            schema_version: SCHEMA,
            asset_root: "cache",
            source_plugin: "Fallout3.esm",
            source_fingerprint: "fingerprint",
            cell: (
                form_id: 1,
                editor_id: None,
                name: None,
                interior: true,
                ambient_rgba: (0.0, 0.0, 0.0, 0.0),
                directional_rgba: (0.0, 0.0, 0.0, 0.0),
            ),
            placements: [],
            lights: [],
            diagnostics: [],
        )"#;
        for version in [1, 2, 3, 4] {
            let text = template.replace("SCHEMA", &version.to_string());
            let manifest: PreparedSceneManifest = ron::de::from_str(&text).unwrap();
            assert_eq!(manifest.schema_version, version);
            assert!(manifest.bake.is_none());
            assert!(manifest.source_plugins.is_empty());
            assert!(manifest.navmeshes.is_empty());
        }
    }

    #[test]
    fn legacy_placement_defaults_to_static_semantics() {
        let text = r#"(
            reference_form_id: 1,
            base_form_id: 2,
            asset_path: None,
            translation: (0.0, 0.0, 0.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 1.0,
            error: None,
        )"#;
        let placement: PreparedPlacement = ron::de::from_str(text).unwrap();
        assert_eq!(placement.semantic, PreparedSemantic::Static);
        assert_eq!(placement.count, 1);
        assert!(placement.initially_enabled);
        assert!(placement.inventory.is_empty());
        assert_eq!(placement.audio, PreparedPlacementAudio::default());
    }

    #[test]
    fn schema_five_lighting_round_trip_and_legacy_defaults() {
        let text = r#"(
            schema_version: 5,
            asset_root: "cache",
            source_plugin: "Fallout3.esm",
            source_fingerprint: "fingerprint",
            cell: (
                form_id: 1,
                editor_id: None,
                name: Some("Lit Cell"),
                interior: true,
                ambient_rgba: (0.1, 0.2, 0.3, 1.0),
                directional_rgba: (0.4, 0.5, 0.6, 1.0),
                raw_lighting: None,
                effective_lighting: Some((
                    ambient_rgba: (0.1, 0.2, 0.3, 1.0),
                    directional_rgba: (0.4, 0.5, 0.6, 1.0),
                    fog_rgba: (0.01, 0.02, 0.03, 1.0),
                    fog_near: 10.0,
                    fog_far: 100.0,
                    directional_rotation_xy: 5,
                    directional_rotation_z: 15,
                    directional_fade: 0.75,
                    fog_clip_distance: 80.0,
                    fog_power: 2.0,
                )),
            ),
            placements: [],
            lights: [],
            diagnostics: [],
        )"#;
        let manifest: PreparedSceneManifest = ron::de::from_str(text).unwrap();
        assert_eq!(manifest.schema_version, 5);
        assert_eq!(
            manifest.cell.effective_lighting.as_ref().unwrap().fog_far,
            100.0
        );
        let encoded = ron::ser::to_string(&manifest).unwrap();
        let decoded: PreparedSceneManifest = ron::de::from_str(&encoded).unwrap();
        assert_eq!(
            decoded.cell.effective_lighting,
            manifest.cell.effective_lighting
        );

        let legacy = r#"(
            form_id: 1,
            editor_id: None,
            name: None,
            interior: true,
            ambient_rgba: (0.0, 0.0, 0.0, 0.0),
            directional_rgba: (0.0, 0.0, 0.0, 0.0),
        )"#;
        let legacy_cell: CellInfo = ron::de::from_str(legacy).unwrap();
        assert!(legacy_cell.raw_lighting.is_none());
        assert!(legacy_cell.effective_lighting.is_none());
    }

    #[test]
    fn schema_three_semantics_round_trip_through_ron() {
        let placement = PreparedPlacement {
            reference_form_id: 1,
            base_form_id: 2,
            asset_path: Some("assets/door.glb".into()),
            translation: [1.0, 2.0, 3.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            reference_kind: "REFR".into(),
            base_kind: "DOOR".into(),
            editor_id: Some("TestDoor".into()),
            display_name: Some("Test Door".into()),
            count: 1,
            semantic: PreparedSemantic::Door(PreparedDoor {
                lock_level: Some(75),
                key_form_id: Some(3),
                destination: Some(PreparedDoorDestination {
                    door_reference_form_id: 4,
                    cell_form_id: 5,
                    translation: [4.0, 5.0, 6.0],
                    rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                }),
            }),
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: PreparedPlacementAudio::default(),
        };
        let text = ron::ser::to_string(&placement).unwrap();
        let decoded: PreparedPlacement = ron::de::from_str(&text).unwrap();
        assert_eq!(decoded.semantic, placement.semantic);
        assert_eq!(decoded.reference_kind, "REFR");
        assert_eq!(decoded.base_kind, "DOOR");
    }
}
