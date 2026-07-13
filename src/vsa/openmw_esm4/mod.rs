//! Focused Rust adaptation of OpenMW's ESM4 reader and record layouts.
//!
//! This is private to the Fallout cell vertical slice. See README.md and
//! NOTICE.md in this directory for provenance and license information.

use anyhow::{Context, Result, anyhow, bail};
use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};

use super::manifest::{CellInfo, ImageSpaceInfo};
use super::paths::CellSelector;

mod binary;
mod enable;
mod inventory;
mod reader;
mod records;

pub(crate) use binary::*;
pub(crate) use enable::*;
pub(crate) use inventory::*;
pub(crate) use reader::*;
pub(crate) use records::*;

const RECORD_COMPRESSED: u32 = 0x0004_0000;
pub(crate) const RECORD_DELETED: u32 = 0x0000_0020;
pub(crate) const RECORD_DISABLED: u32 = 0x0000_0800;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ReferenceKind {
    #[default]
    Object,
    Npc,
    Creature,
}

impl ReferenceKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Object => "REFR",
            Self::Npc => "ACHR",
            Self::Creature => "ACRE",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BaseRecord {
    pub(crate) kind: String,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) value: Option<i32>,
    pub(crate) weight: Option<f32>,
    pub(crate) base_template_form_id: Option<u32>,
    pub(crate) light: Option<LightData>,
    pub(crate) inventory: Vec<InventoryItemRecord>,
    pub(crate) audio: BaseAudioRecord,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InventoryItemRecord {
    pub(crate) item_form_id: u32,
    pub(crate) count: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BaseAudioRecord {
    pub(crate) loop_sound_form_id: Option<u32>,
    pub(crate) activation_sound_form_id: Option<u32>,
    pub(crate) open_sound_form_id: Option<u32>,
    pub(crate) close_sound_form_id: Option<u32>,
    pub(crate) pickup_sound_form_id: Option<u32>,
    pub(crate) drop_sound_form_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub(crate) struct LightData {
    pub(crate) radius: f32,
    pub(crate) color_rgba: [f32; 4],
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ReferenceRecord {
    pub(crate) kind: ReferenceKind,
    pub(crate) form_id: u32,
    pub(crate) parent_cell_form_id: u32,
    pub(crate) base_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) rotation: [f32; 3],
    pub(crate) scale: f32,
    pub(crate) count: i32,
    pub(crate) flags: u32,
    pub(crate) door: Option<DoorRecord>,
    pub(crate) owner_form_id: Option<u32>,
    pub(crate) owner_faction_rank: Option<i32>,
    pub(crate) enable_parent: Option<EnableParentRecord>,
    pub(crate) initially_enabled: bool,
    pub(crate) ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EnableParentRecord {
    pub(crate) parent_reference_form_id: u32,
    pub(crate) flags: u32,
}

impl EnableParentRecord {
    pub(crate) const INVERTED: u32 = 0x1;
    pub(crate) const POP_IN: u32 = 0x2;

    pub(crate) fn is_inverted(self) -> bool {
        self.flags & Self::INVERTED != 0
    }

    pub(crate) fn is_pop_in(self) -> bool {
        self.flags & Self::POP_IN != 0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DoorRecord {
    pub(crate) lock_level: Option<i8>,
    pub(crate) key_form_id: Option<u32>,
    pub(crate) destination: Option<DoorDestinationRecord>,
    teleport: Option<TeleportRecord>,
}

#[derive(Debug, Clone)]
pub(crate) struct DoorDestinationRecord {
    pub(crate) door_reference_form_id: u32,
    pub(crate) cell_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) rotation: [f32; 3],
}

#[derive(Debug, Clone)]
struct TeleportRecord {
    door_reference_form_id: u32,
    position: [f32; 3],
    rotation: [f32; 3],
}

#[derive(Debug, Clone)]
pub(crate) struct NavMeshRecord {
    pub(crate) form_id: u32,
    pub(crate) flags: u32,
    pub(crate) version: Option<u32>,
    pub(crate) payload: Vec<u8>,
    pub(crate) chunks: Vec<NavMeshChunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NavMeshChunk {
    pub(crate) signature: String,
    pub(crate) byte_len: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct SoundRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) file: Option<String>,
    pub(crate) parameters: Option<SoundParameters>,
    pub(crate) extra: Option<SoundExtraData>,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct SoundParameters {
    pub(crate) byte_len: u32,
    pub(crate) min_attenuation: u8,
    pub(crate) max_attenuation: u8,
    pub(crate) frequency_adjustment: i8,
    pub(crate) flags: u16,
    pub(crate) static_attenuation: Option<u16>,
    pub(crate) stop_time: Option<u8>,
    pub(crate) start_time: Option<u8>,
}

impl SoundParameters {
    pub(crate) const LOOP: u16 = 0x0010;
    pub(crate) const TWO_DIMENSIONAL: u16 = 0x0040;

    pub(crate) fn is_looping(self) -> bool {
        self.flags & Self::LOOP != 0
    }

    pub(crate) fn is_two_dimensional(self) -> bool {
        self.flags & Self::TWO_DIMENSIONAL != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SoundExtraData {
    pub(crate) attenuation_points: [i16; 5],
    pub(crate) reverb_attenuation_control: i16,
    pub(crate) priority: i32,
    pub(crate) unknown_x: i32,
    pub(crate) unknown_y: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct SoundReferenceRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) sound_category_form_id: Option<u32>,
    pub(crate) sound_reference_form_id: Option<u32>,
    pub(crate) output_model_form_id: Option<u32>,
    pub(crate) base_descriptor_form_id: Option<u32>,
    pub(crate) file: Option<String>,
    pub(crate) loop_info: Option<SoundLoopInfo>,
    pub(crate) sound_info: Option<SoundInfo>,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SoundLoopInfo {
    pub(crate) flags: u16,
    pub(crate) unknown: u8,
    pub(crate) rumble: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SoundInfo {
    pub(crate) frequency_adjustment: i8,
    pub(crate) frequency_variance: u8,
    pub(crate) priority: u8,
    pub(crate) decibel_variance: u8,
    pub(crate) static_attenuation: u16,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct AcousticSpaceRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) environment_type: Option<u32>,
    pub(crate) ambient_loop_sound_form_ids: Vec<u32>,
    pub(crate) sound_region_form_id: Option<u32>,
    pub(crate) is_interior: Option<bool>,
    ignored_subrecords: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct MusicRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) file: Option<String>,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LightingData {
    pub(crate) ambient_rgba: [f32; 4],
    pub(crate) directional_rgba: [f32; 4],
    pub(crate) fog_rgba: [f32; 4],
    pub(crate) fog_near: f32,
    pub(crate) fog_far: f32,
    pub(crate) rotation_xy: i32,
    pub(crate) rotation_z: i32,
    pub(crate) fog_directional_fade: f32,
    pub(crate) fog_clip_distance: f32,
    pub(crate) fog_power: f32,
}

/// Fallout 3 `CELL.LNAM` bits selecting fields inherited from `LTMP`.
///
/// These values are kept next to the attributed ESM4 parser because they are
/// part of the record format, rather than a viewer policy.
pub(crate) struct LightingTemplateFlags;

impl LightingTemplateFlags {
    pub(crate) const AMBIENT: u32 = 0x001;
    pub(crate) const DIRECTIONAL: u32 = 0x002;
    pub(crate) const FOG_COLOR: u32 = 0x004;
    pub(crate) const FOG_NEAR: u32 = 0x008;
    pub(crate) const FOG_FAR: u32 = 0x010;
    pub(crate) const DIRECTIONAL_ROTATION: u32 = 0x020;
    pub(crate) const DIRECTIONAL_FADE: u32 = 0x040;
    pub(crate) const FOG_CLIP_DISTANCE: u32 = 0x080;
    pub(crate) const FOG_POWER: u32 = 0x100;
}

impl LightingData {
    pub(crate) fn apply_template(&mut self, template: Self, flags: u32) {
        if flags & LightingTemplateFlags::AMBIENT != 0 {
            self.ambient_rgba = template.ambient_rgba;
        }
        if flags & LightingTemplateFlags::DIRECTIONAL != 0 {
            self.directional_rgba = template.directional_rgba;
        }
        if flags & LightingTemplateFlags::FOG_COLOR != 0 {
            self.fog_rgba = template.fog_rgba;
        }
        if flags & LightingTemplateFlags::FOG_NEAR != 0 {
            self.fog_near = template.fog_near;
        }
        if flags & LightingTemplateFlags::FOG_FAR != 0 {
            self.fog_far = template.fog_far;
        }
        if flags & LightingTemplateFlags::DIRECTIONAL_ROTATION != 0 {
            self.rotation_xy = template.rotation_xy;
            self.rotation_z = template.rotation_z;
        }
        if flags & LightingTemplateFlags::DIRECTIONAL_FADE != 0 {
            self.fog_directional_fade = template.fog_directional_fade;
        }
        if flags & LightingTemplateFlags::FOG_CLIP_DISTANCE != 0 {
            self.fog_clip_distance = template.fog_clip_distance;
        }
        if flags & LightingTemplateFlags::FOG_POWER != 0 {
            self.fog_power = template.fog_power;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct LightingTemplateRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) lighting: Option<LightingData>,
    ignored_subrecords: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct CellMetadata {
    pub(crate) acoustic_space_form_id: Option<u32>,
    pub(crate) music_form_id: Option<u32>,
    pub(crate) lighting_template_form_id: Option<u32>,
    pub(crate) lighting_template_flags: u32,
    pub(crate) water_form_id: Option<u32>,
    pub(crate) water_height: Option<f32>,
    pub(crate) lighting: Option<LightingData>,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct ParsedPlugin {
    pub(crate) bases: HashMap<u32, BaseRecord>,
    pub(crate) image_spaces: HashMap<u32, ImageSpaceInfo>,
    pub(crate) sounds: HashMap<u32, SoundRecord>,
    pub(crate) sound_references: HashMap<u32, SoundReferenceRecord>,
    pub(crate) acoustic_spaces: HashMap<u32, AcousticSpaceRecord>,
    pub(crate) music: HashMap<u32, MusicRecord>,
    pub(crate) lighting_templates: HashMap<u32, LightingTemplateRecord>,
    pub(crate) references: Vec<ReferenceRecord>,
    pub(crate) navmeshes: Vec<NavMeshRecord>,
    pub(crate) cell: Option<CellInfo>,
    pub(crate) cell_metadata: Option<CellMetadata>,
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PluginSource<'a> {
    pub(crate) name: &'a str,
    pub(crate) bytes: &'a [u8],
}

#[derive(Debug, Default)]
pub(crate) struct ParsedState {
    bases: HashMap<u32, BaseRecord>,
    image_spaces: HashMap<u32, ImageSpaceInfo>,
    sounds: HashMap<u32, SoundRecord>,
    sound_references: HashMap<u32, SoundReferenceRecord>,
    acoustic_spaces: HashMap<u32, AcousticSpaceRecord>,
    music: HashMap<u32, MusicRecord>,
    lighting_templates: HashMap<u32, LightingTemplateRecord>,
    references: HashMap<u32, ReferenceRecord>,
    navmeshes: HashMap<u32, (u32, NavMeshRecord)>,
    cells: HashMap<u32, CellInfo>,
    cell_metadata: HashMap<u32, CellMetadata>,
}

#[derive(Debug)]
pub(crate) struct FormIdResolver {
    current_index: u8,
    master_indices: Vec<u8>,
}

impl FormIdResolver {
    fn adjust(&self, raw: u32) -> u32 {
        let local_file_index = (raw >> 24) as usize;
        let object_index = raw & 0x00ff_ffff;
        let global_file_index = self
            .master_indices
            .get(local_file_index)
            .copied()
            .unwrap_or(self.current_index);
        (u32::from(global_file_index) << 24) | object_index
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Subrecord {
    signature: String,
    data: Vec<u8>,
}

pub(crate) fn parse_content_set(
    sources: &[PluginSource<'_>],
    selector: &CellSelector,
) -> Result<ParsedPlugin> {
    if sources.len() > 256 {
        bail!("ESM4 content set exceeds the 256-file FormID limit")
    }
    let source_indices = sources
        .iter()
        .enumerate()
        .map(|(index, source)| (source.name.to_ascii_lowercase(), index as u8))
        .collect::<HashMap<_, _>>();
    let mut state = ParsedState::default();

    for (index, source) in sources.iter().enumerate() {
        let masters = read_master_names(source.bytes)
            .with_context(|| format!("reading masters from {}", source.name))?;
        let master_indices = masters
            .iter()
            .map(|master| {
                source_indices
                    .get(&master.to_ascii_lowercase())
                    .copied()
                    .with_context(|| format!("{} requires unloaded master {master}", source.name))
            })
            .collect::<Result<Vec<_>>>()?;
        let resolver = FormIdResolver {
            current_index: index as u8,
            master_indices,
        };
        walk_container(
            source.bytes,
            0,
            source.bytes.len(),
            None,
            &resolver,
            &mut state,
        )
        .with_context(|| format!("parsing {}", source.name))?;
    }

    let target_cell = match selector {
        CellSelector::FormId(form_id) => *form_id,
        CellSelector::EditorId(editor_id) => {
            let matches = state
                .cells
                .values()
                .filter(|cell| {
                    cell.editor_id
                        .as_deref()
                        .is_some_and(|value| value.eq_ignore_ascii_case(editor_id))
                })
                .map(|cell| cell.form_id)
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [] => bail!("GECK EditorID '{editor_id}' was not found in the loaded content set"),
                [form_id] => *form_id,
                _ => {
                    let form_ids = matches
                        .iter()
                        .map(|form_id| format!("{form_id:08x}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    bail!(
                        "GECK EditorID '{editor_id}' is ambiguous; matching cell FormIDs: {form_ids}"
                    )
                }
            }
        }
    };

    let all_references = state.references;
    let mut references = all_references
        .values()
        .filter(|reference| reference.parent_cell_form_id == target_cell)
        .cloned()
        .collect::<Vec<_>>();
    for reference in &mut references {
        let Some(door) = reference.door.as_mut() else {
            continue;
        };
        let Some(teleport) = door.teleport.as_ref() else {
            continue;
        };
        if let Some(destination) = all_references.get(&teleport.door_reference_form_id) {
            door.destination = Some(DoorDestinationRecord {
                door_reference_form_id: teleport.door_reference_form_id,
                cell_form_id: destination.parent_cell_form_id,
                position: teleport.position,
                rotation: teleport.rotation,
            });
        }
    }
    for reference in &mut references {
        match resolve_initially_enabled(reference.form_id, &all_references) {
            Ok(enabled) => reference.initially_enabled = enabled,
            Err(error) => {
                reference.initially_enabled = true;
                reference.ignored_subrecords.push(format!("XESP:{error}"));
            }
        }
    }
    references.sort_by_key(|reference| reference.form_id);

    let mut ignored = HashMap::<(String, String), usize>::new();
    for reference in &references {
        for signature in &reference.ignored_subrecords {
            if let Some(error) = signature.strip_prefix("XESP:") {
                *ignored
                    .entry(("XESP resolution".into(), error.into()))
                    .or_default() += 1;
            } else {
                *ignored
                    .entry((reference.kind.as_str().into(), signature.clone()))
                    .or_default() += 1;
            }
        }
        if let Some(base) = state.bases.get(&reference.base_form_id) {
            for signature in &base.ignored_subrecords {
                *ignored
                    .entry((base.kind.clone(), signature.clone()))
                    .or_default() += 1;
            }
        }
    }
    if let Some(cell) = state.cell_metadata.get(&target_cell) {
        for signature in &cell.ignored_subrecords {
            *ignored
                .entry(("CELL".into(), signature.clone()))
                .or_default() += 1;
        }
    }
    let mut diagnostics = ignored
        .into_iter()
        .map(|((record, subrecord), count)| {
            if record == "XESP resolution" {
                format!("{subrecord} while resolving {count} target-cell placement(s); defaulted visible")
            } else {
                format!(
                    "ignored unsupported {record}.{subrecord} subrecord while importing {count} target-cell record(s)"
                )
            }
        })
        .collect::<Vec<_>>();
    diagnostics.sort();

    let mut navmeshes = state
        .navmeshes
        .into_values()
        .filter_map(|(cell, navmesh)| (cell == target_cell).then_some(navmesh))
        .collect::<Vec<_>>();
    navmeshes.sort_by_key(|navmesh| navmesh.form_id);

    Ok(ParsedPlugin {
        bases: state.bases,
        image_spaces: state.image_spaces,
        sounds: state.sounds,
        sound_references: state.sound_references,
        acoustic_spaces: state.acoustic_spaces,
        music: state.music,
        lighting_templates: state.lighting_templates,
        references,
        navmeshes,
        cell: state.cells.remove(&target_cell),
        cell_metadata: state.cell_metadata.remove(&target_cell),
        diagnostics,
    })
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum EnableResolutionError {
    Unresolved(u32),
    Cycle(u32),
}

impl std::fmt::Display for EnableResolutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unresolved(form_id) => {
                write!(formatter, "unresolved enable-parent {form_id:08x}")
            }
            Self::Cycle(form_id) => write!(formatter, "enable-parent cycle at {form_id:08x}"),
        }
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
