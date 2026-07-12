//! Focused Rust adaptation of OpenMW's ESM4 reader and record layouts.
//!
//! This is private to the Fallout cell vertical slice. See README.md and
//! NOTICE.md in this directory for provenance and license information.

use anyhow::{Context, Result, anyhow, bail};
use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};

use super::manifest::{CellInfo, ImageSpaceInfo};

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
struct ParsedState {
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
struct FormIdResolver {
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
struct Subrecord {
    signature: String,
    data: Vec<u8>,
}

pub(crate) fn read_master_names(bytes: &[u8]) -> Result<Vec<String>> {
    if bytes.len() < 24 || &bytes[0..4] != b"TES4" {
        bail!("plugin does not start with a TES4 header")
    }
    let size = read_u32(bytes, 4)? as usize;
    let end = 24usize
        .checked_add(size)
        .filter(|end| *end <= bytes.len())
        .context("TES4 header exceeds plugin")?;
    Ok(parse_subrecords(&bytes[24..end])?
        .into_iter()
        .filter(|sub| sub.signature == "MAST")
        .map(|sub| cstring(&sub.data))
        .collect())
}

#[cfg(test)]
fn parse_plugin(bytes: &[u8], target_cell: u32) -> Result<ParsedPlugin> {
    parse_content_set(
        &[PluginSource {
            name: "Fallout3.esm",
            bytes,
        }],
        target_cell,
    )
}

pub(crate) fn parse_content_set(
    sources: &[PluginSource<'_>],
    target_cell: u32,
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
enum EnableResolutionError {
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

fn resolve_initially_enabled(
    reference_form_id: u32,
    references: &HashMap<u32, ReferenceRecord>,
) -> std::result::Result<bool, EnableResolutionError> {
    fn visit(
        form_id: u32,
        references: &HashMap<u32, ReferenceRecord>,
        visiting: &mut Vec<u32>,
        memo: &mut HashMap<u32, bool>,
    ) -> std::result::Result<bool, EnableResolutionError> {
        if let Some(enabled) = memo.get(&form_id) {
            return Ok(*enabled);
        }
        if visiting.contains(&form_id) {
            return Err(EnableResolutionError::Cycle(form_id));
        }
        let reference = references
            .get(&form_id)
            .ok_or(EnableResolutionError::Unresolved(form_id))?;
        visiting.push(form_id);
        let enabled = if let Some(enable_parent) = reference.enable_parent {
            let parent_enabled = visit(
                enable_parent.parent_reference_form_id,
                references,
                visiting,
                memo,
            )?;
            if enable_parent.is_inverted() {
                !parent_enabled
            } else {
                parent_enabled
            }
        } else {
            reference.flags & RECORD_DISABLED == 0
        };
        visiting.pop();
        memo.insert(form_id, enabled);
        Ok(enabled)
    }

    visit(
        reference_form_id,
        references,
        &mut Vec::new(),
        &mut HashMap::new(),
    )
}

fn walk_container(
    bytes: &[u8],
    mut offset: usize,
    end: usize,
    current_cell: Option<u32>,
    resolver: &FormIdResolver,
    state: &mut ParsedState,
) -> Result<()> {
    while offset + 4 <= end {
        let signature = &bytes[offset..offset + 4];
        if signature == b"GRUP" {
            if offset + 24 > end {
                bail!("truncated GRUP header")
            }
            let size = read_u32(bytes, offset + 4)? as usize;
            if size < 24 || offset + size > end {
                bail!("invalid GRUP size")
            }
            let label = read_u32(bytes, offset + 8)?;
            let group_type = read_i32(bytes, offset + 12)?;
            let child_cell = if group_type == 6 {
                Some(resolver.adjust(label))
            } else {
                current_cell
            };
            walk_container(
                bytes,
                offset + 24,
                offset + size,
                child_cell,
                resolver,
                state,
            )?;
            offset += size;
            continue;
        }

        if offset + 24 > end {
            bail!("truncated record header")
        }
        let data_size = read_u32(bytes, offset + 4)? as usize;
        let flags = read_u32(bytes, offset + 8)?;
        let raw_form_id = read_u32(bytes, offset + 12)?;
        let form_id = resolver.adjust(raw_form_id);
        let record_end = offset + 24 + data_size;
        if record_end > end {
            bail!("record exceeds containing group")
        }
        let sig = String::from_utf8_lossy(signature).to_string();
        let data = record_payload(&bytes[offset + 24..record_end], flags, &sig, form_id)?;
        let subs = parse_subrecords(&data)?;

        match sig.as_str() {
            "CELL" => {
                if flags & RECORD_DELETED != 0 {
                    state.cells.remove(&form_id);
                    state.cell_metadata.remove(&form_id);
                } else {
                    state
                        .cells
                        .insert(form_id, parse_cell(&subs, form_id, resolver)?);
                    state
                        .cell_metadata
                        .insert(form_id, parse_cell_metadata(&subs, resolver));
                }
            }
            "IMGS" => {
                if flags & RECORD_DELETED != 0 {
                    state.image_spaces.remove(&form_id);
                } else if let Some(image_space) = parse_image_space(&subs, form_id) {
                    state.image_spaces.insert(form_id, image_space);
                }
            }
            "SOUN" => {
                if flags & RECORD_DELETED != 0 {
                    state.sounds.remove(&form_id);
                } else {
                    state
                        .sounds
                        .insert(form_id, parse_sound(&subs, form_id, flags));
                }
            }
            "SNDR" => {
                if flags & RECORD_DELETED != 0 {
                    state.sound_references.remove(&form_id);
                } else {
                    state.sound_references.insert(
                        form_id,
                        parse_sound_reference(&subs, form_id, flags, resolver),
                    );
                }
            }
            "ASPC" => {
                if flags & RECORD_DELETED != 0 {
                    state.acoustic_spaces.remove(&form_id);
                } else {
                    state.acoustic_spaces.insert(
                        form_id,
                        parse_acoustic_space(&subs, form_id, flags, resolver),
                    );
                }
            }
            "MUSC" => {
                if flags & RECORD_DELETED != 0 {
                    state.music.remove(&form_id);
                } else {
                    state
                        .music
                        .insert(form_id, parse_music(&subs, form_id, flags));
                }
            }
            "LGTM" => {
                if flags & RECORD_DELETED != 0 {
                    state.lighting_templates.remove(&form_id);
                } else {
                    state
                        .lighting_templates
                        .insert(form_id, parse_lighting_template(&subs, form_id, flags));
                }
            }
            "REFR" | "ACHR" | "ACRE" if current_cell.is_some() => {
                if flags & RECORD_DELETED != 0 {
                    state.references.remove(&form_id);
                } else if let Some(reference) = parse_reference(
                    &subs,
                    form_id,
                    current_cell.unwrap_or_default(),
                    flags,
                    match sig.as_str() {
                        "ACHR" => ReferenceKind::Npc,
                        "ACRE" => ReferenceKind::Creature,
                        _ => ReferenceKind::Object,
                    },
                    resolver,
                )? {
                    state.references.insert(form_id, reference);
                }
            }
            "NAVM" if current_cell.is_some() => {
                if flags & RECORD_DELETED != 0 {
                    state.navmeshes.remove(&form_id);
                } else {
                    state.navmeshes.insert(
                        form_id,
                        (
                            current_cell.unwrap_or_default(),
                            parse_navmesh(&subs, form_id, flags, data),
                        ),
                    );
                }
            }
            _ => {
                if flags & RECORD_DELETED != 0 {
                    state.bases.remove(&form_id);
                } else if let Some(base) = parse_base(&sig, &subs, resolver) {
                    state.bases.insert(form_id, base);
                }
            }
        }
        offset = record_end;
    }
    Ok(())
}

fn record_payload(data: &[u8], flags: u32, sig: &str, form_id: u32) -> Result<Vec<u8>> {
    if flags & RECORD_COMPRESSED == 0 {
        return Ok(data.to_vec());
    }
    if data.len() < 4 {
        bail!("compressed record is missing decompressed size")
    }
    let expected = read_u32(data, 0)? as usize;
    let mut decoder = ZlibDecoder::new(Cursor::new(&data[4..]));
    let mut decompressed = Vec::with_capacity(expected);
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|error| anyhow!("decompressing {sig} record {form_id:08x}: {error}"))?;
    if decompressed.len() != expected {
        bail!(
            "decompressed {sig} record {form_id:08x} to {} bytes, expected {expected}",
            decompressed.len()
        )
    }
    Ok(decompressed)
}

fn parse_subrecords(data: &[u8]) -> Result<Vec<Subrecord>> {
    let mut result = Vec::new();
    let mut offset = 0;
    let mut extended = None;
    while offset < data.len() {
        if offset + 6 > data.len() {
            bail!("truncated subrecord")
        }
        let signature = String::from_utf8_lossy(&data[offset..offset + 4]).to_string();
        let size = u16::from_le_bytes(data[offset + 4..offset + 6].try_into().unwrap()) as usize;
        offset += 6;
        if signature == "XXXX" {
            if size != 4 || offset + 4 > data.len() {
                bail!("invalid XXXX")
            }
            extended = Some(read_u32(data, offset)? as usize);
            offset += 4;
            continue;
        }
        let actual_size = extended.take().unwrap_or(size);
        if offset + actual_size > data.len() {
            bail!("subrecord exceeds record")
        }
        result.push(Subrecord {
            signature,
            data: data[offset..offset + actual_size].to_vec(),
        });
        offset += actual_size;
    }
    Ok(result)
}

fn parse_base(sig: &str, subs: &[Subrecord], resolver: &FormIdResolver) -> Option<BaseRecord> {
    if !is_supported_base(sig) {
        return None;
    }
    let model_signature = if sig == "ARMO" { "MOD2" } else { "MODL" };
    let model = sub(subs, model_signature)
        .or_else(|| sub(subs, "MODL"))
        .map(cstring)
        .filter(|value| !value.is_empty());
    let light = (sig == "LIGH").then(|| parse_light_data(subs)).flatten();
    let (value, weight) = parse_value_weight(sig, subs);
    let inventory = subs
        .iter()
        .filter(|subrecord| subrecord.signature == "CNTO")
        .filter_map(|subrecord| parse_inventory_item(&subrecord.data, resolver))
        .collect();
    let audio = parse_base_audio(sig, subs, resolver);
    Some(BaseRecord {
        kind: sig.to_string(),
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        model,
        value,
        weight,
        base_template_form_id: sub_form_id(subs, "TPLT", resolver),
        light,
        inventory,
        audio,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FULL", "MODL", "MOD2", "DATA", "ENIT", "TPLT", "CNTO", "SNAM", "ANAM",
                "BNAM", "QNAM", "VNAM", "YNAM", "ZNAM",
            ],
        ),
    })
}

fn parse_inventory_item(data: &[u8], resolver: &FormIdResolver) -> Option<InventoryItemRecord> {
    let item = data.get(..4)?;
    let count = data.get(4..8)?;
    Some(InventoryItemRecord {
        item_form_id: resolver.adjust(u32::from_le_bytes(item.try_into().ok()?)),
        count: u32::from_le_bytes(count.try_into().ok()?),
    })
}

fn parse_base_audio(sig: &str, subs: &[Subrecord], resolver: &FormIdResolver) -> BaseAudioRecord {
    let mut audio = BaseAudioRecord::default();
    match sig {
        "DOOR" => {
            audio.open_sound_form_id = sub_form_id(subs, "SNAM", resolver);
            audio.close_sound_form_id = sub_form_id(subs, "ANAM", resolver);
            audio.loop_sound_form_id = sub_form_id(subs, "BNAM", resolver);
        }
        "CONT" => {
            audio.open_sound_form_id = sub_form_id(subs, "SNAM", resolver);
            audio.close_sound_form_id = sub_form_id(subs, "QNAM", resolver);
        }
        "ACTI" => {
            audio.loop_sound_form_id = sub_form_id(subs, "SNAM", resolver);
            audio.activation_sound_form_id = sub_form_id(subs, "VNAM", resolver);
        }
        "MSTT" | "TACT" | "LIGH" => {
            audio.loop_sound_form_id = sub_form_id(subs, "SNAM", resolver);
        }
        "TERM" => {
            audio.activation_sound_form_id = sub_form_id(subs, "SNAM", resolver);
        }
        _ if is_pickup_base(sig) => {
            audio.pickup_sound_form_id = sub_form_id(subs, "YNAM", resolver);
            audio.drop_sound_form_id = sub_form_id(subs, "ZNAM", resolver);
        }
        _ => {}
    }
    audio
}

fn is_pickup_base(sig: &str) -> bool {
    matches!(
        sig,
        "WEAP" | "AMMO" | "ARMO" | "ALCH" | "MISC" | "BOOK" | "NOTE" | "KEYM"
    )
}

fn is_supported_base(sig: &str) -> bool {
    matches!(
        sig,
        "STAT"
            | "MSTT"
            | "LIGH"
            | "DOOR"
            | "CONT"
            | "ACTI"
            | "TACT"
            | "FURN"
            | "TERM"
            | "WEAP"
            | "AMMO"
            | "ARMO"
            | "ALCH"
            | "MISC"
            | "BOOK"
            | "NOTE"
            | "KEYM"
            | "NPC_"
            | "CREA"
            | "LVLI"
            | "LVLN"
            | "LVLC"
    )
}

fn parse_value_weight(sig: &str, subs: &[Subrecord]) -> (Option<i32>, Option<f32>) {
    let data = sub(subs, "DATA");
    match sig {
        "MISC" | "KEYM" => data
            .filter(|data| data.len() >= 8)
            .map(|data| (i32_at(data, 0), f32_at_option(data, 4)))
            .unwrap_or((None, None)),
        "WEAP" => match data.map(<[u8]>::len) {
            Some(10) => (
                data.and_then(|data| i32_at(data, 0)),
                data.and_then(|data| f32_at_option(data, 4)),
            ),
            Some(15) => (
                data.and_then(|data| i32_at(data, 0)),
                data.and_then(|data| f32_at_option(data, 8)),
            ),
            Some(length) if length >= 30 => (
                data.and_then(|data| i32_at(data, 16)),
                data.and_then(|data| f32_at_option(data, 24)),
            ),
            _ => (None, None),
        },
        "AMMO" => data
            .filter(|data| data.len() >= 12)
            .map(|data| (i32_at(data, 8), None))
            .unwrap_or((None, None)),
        "ARMO" => data
            .filter(|data| data.len() >= 12)
            .map(|data| (i32_at(data, 0), f32_at_option(data, 8)))
            .unwrap_or((None, None)),
        "ALCH" => (
            sub(subs, "ENIT").and_then(|data| i32_at(data, 0)),
            data.and_then(|data| f32_at_option(data, 0)),
        ),
        "BOOK" => data
            .filter(|data| data.len() >= 10)
            .map(|data| (i32_at(data, 2), f32_at_option(data, 6)))
            .unwrap_or((None, None)),
        _ => (None, None),
    }
}

fn parse_light_data(subs: &[Subrecord]) -> Option<LightData> {
    let data = sub(subs, "DATA")?;
    if data.len() < 12 {
        return None;
    }
    Some(LightData {
        radius: u32::from_le_bytes(data[4..8].try_into().ok()?) as f32,
        color_rgba: [
            data[8] as f32 / 255.0,
            data[9] as f32 / 255.0,
            data[10] as f32 / 255.0,
            1.0,
        ],
    })
}

fn parse_music(subs: &[Subrecord], form_id: u32, record_flags: u32) -> MusicRecord {
    MusicRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        file: sub(subs, "FNAM")
            .or_else(|| sub(subs, "MNAM"))
            .map(cstring)
            .filter(|value| !value.is_empty()),
        ignored_subrecords: ignored_signatures(subs, &["EDID", "FNAM", "MNAM", "DATA"]),
    }
}

fn parse_sound(subs: &[Subrecord], form_id: u32, record_flags: u32) -> SoundRecord {
    let mut parameters = None;
    let mut extra = None;
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "SNDX" => parameters = parse_sound_parameters(&subrecord.data),
            "SNDD" => {
                parameters = parse_sound_parameters(&subrecord.data);
                extra = parse_sound_extra(&subrecord.data);
            }
            _ => {}
        }
    }
    SoundRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        file: sub(subs, "FNAM")
            .map(cstring)
            .filter(|value| !value.is_empty()),
        parameters,
        extra,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FNAM", "SNDX", "SNDD", "OBND", "SDSC", "ANAM", "GNAM", "HNAM", "RNAM",
                "REPT",
            ],
        ),
    }
}

fn parse_sound_parameters(data: &[u8]) -> Option<SoundParameters> {
    if data.len() < 8 {
        return None;
    }
    Some(SoundParameters {
        byte_len: data.len() as u32,
        min_attenuation: data[0],
        max_attenuation: data[1],
        frequency_adjustment: data[2] as i8,
        flags: u16::from_le_bytes(data[4..6].try_into().ok()?),
        static_attenuation: data
            .get(8..10)
            .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap())),
        stop_time: data.get(10).copied(),
        start_time: data.get(11).copied(),
    })
}

fn parse_sound_extra(data: &[u8]) -> Option<SoundExtraData> {
    let data = data.get(12..36)?;
    Some(SoundExtraData {
        attenuation_points: [
            i16_at(data, 0)?,
            i16_at(data, 2)?,
            i16_at(data, 4)?,
            i16_at(data, 6)?,
            i16_at(data, 8)?,
        ],
        reverb_attenuation_control: i16_at(data, 10)?,
        priority: i32_at(data, 12)?,
        unknown_x: i32_at(data, 16)?,
        unknown_y: i32_at(data, 20)?,
    })
}

fn parse_sound_reference(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> SoundReferenceRecord {
    let bnam = sub(subs, "BNAM");
    SoundReferenceRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        sound_category_form_id: sub_form_id(subs, "GNAM", resolver),
        sound_reference_form_id: sub_form_id(subs, "SNAM", resolver),
        output_model_form_id: sub_form_id(subs, "ONAM", resolver),
        base_descriptor_form_id: bnam
            .filter(|data| data.len() == 4)
            .map(|data| resolver.adjust(u32::from_le_bytes(data.try_into().unwrap())))
            .filter(|form_id| *form_id != 0),
        file: sub(subs, "ANAM")
            .map(cstring)
            .filter(|value| !value.is_empty()),
        loop_info: sub(subs, "LNAM")
            .filter(|data| data.len() >= 4)
            .map(|data| SoundLoopInfo {
                flags: u16::from_le_bytes(data[0..2].try_into().unwrap()),
                unknown: data[2],
                rumble: data[3],
            }),
        sound_info: bnam.filter(|data| data.len() == 6).map(|data| SoundInfo {
            frequency_adjustment: data[0] as i8,
            frequency_variance: data[1],
            priority: data[2],
            decibel_variance: data[3],
            static_attenuation: u16::from_le_bytes(data[4..6].try_into().unwrap()),
        }),
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "GNAM", "SNAM", "ONAM", "ANAM", "LNAM", "BNAM", "CTDA", "CIS1", "CIS2",
                "CNAM", "DNAM", "FNAM", "INTV", "ITMC", "ITME", "ITMS", "NNAM",
            ],
        ),
    }
}

fn parse_acoustic_space(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
    resolver: &FormIdResolver,
) -> AcousticSpaceRecord {
    let mut is_interior = None;
    for subrecord in subs {
        match subrecord.signature.as_str() {
            "INAM" if subrecord.data.len() >= 4 => {
                is_interior =
                    Some(u32::from_le_bytes(subrecord.data[0..4].try_into().unwrap()) != 0)
            }
            "XTRI" if !subrecord.data.is_empty() => is_interior = Some(subrecord.data[0] != 0),
            _ => {}
        }
    }
    AcousticSpaceRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        environment_type: sub(subs, "ANAM").and_then(|data| u32_at(data, 0)),
        ambient_loop_sound_form_ids: subs
            .iter()
            .filter(|subrecord| subrecord.signature == "SNAM")
            .filter_map(|subrecord| u32_at(&subrecord.data, 0))
            .map(|form_id| resolver.adjust(form_id))
            .filter(|form_id| *form_id != 0)
            .collect(),
        sound_region_form_id: sub_form_id(subs, "RDAT", resolver),
        is_interior,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "ANAM", "SNAM", "RDAT", "INAM", "XTRI", "WNAM", "BNAM", "OBND",
            ],
        ),
    }
}

fn parse_lighting_template(
    subs: &[Subrecord],
    form_id: u32,
    record_flags: u32,
) -> LightingTemplateRecord {
    LightingTemplateRecord {
        form_id,
        record_flags,
        editor_id: sub(subs, "EDID").map(cstring),
        lighting: sub(subs, "DATA").and_then(parse_lighting_data),
        ignored_subrecords: ignored_signatures(subs, &["EDID", "DATA", "DALC"]),
    }
}

fn parse_lighting_data(data: &[u8]) -> Option<LightingData> {
    // OpenMW accepts the TES4 36-byte layout and the FO3/FNV 40-byte layout
    // (and skips the tail of newer layouts).  Reject the otherwise ambiguous
    // 37-39 byte range instead of partially applying malformed lighting.
    if data.len() < 36 || (data.len() > 36 && data.len() < 40) {
        return None;
    }
    Some(LightingData {
        ambient_rgba: rgba8(&data[0..4]),
        directional_rgba: rgba8(&data[4..8]),
        fog_rgba: rgba8(&data[8..12]),
        fog_near: f32_at_option(data, 12)?,
        fog_far: f32_at_option(data, 16)?,
        rotation_xy: i32_at(data, 20)?,
        rotation_z: i32_at(data, 24)?,
        fog_directional_fade: f32_at_option(data, 28)?,
        fog_clip_distance: f32_at_option(data, 32)?,
        fog_power: f32_at_option(data, 36).unwrap_or(1.0),
    })
}

fn parse_cell(subs: &[Subrecord], form_id: u32, resolver: &FormIdResolver) -> Result<CellInfo> {
    let interior = sub(subs, "DATA")
        .and_then(|data| data.first())
        .is_some_and(|flags| flags & 1 != 0);
    let mut ambient = [0.18, 0.18, 0.18, 1.0];
    let mut directional = [0.8, 0.8, 0.8, 1.0];
    if let Some(lighting) = sub(subs, "XCLL").and_then(parse_lighting_data) {
        ambient = lighting.ambient_rgba;
        directional = lighting.directional_rgba;
    }
    Ok(CellInfo {
        form_id,
        editor_id: sub(subs, "EDID").map(cstring),
        name: sub(subs, "FULL").map(cstring),
        interior,
        ambient_rgba: ambient,
        directional_rgba: directional,
        image_space_form_id: sub_form_id(subs, "XCIM", resolver),
        image_space: None,
        lighting_template_form_id: sub_form_id(subs, "LTMP", resolver),
        lighting_template_flags: sub(subs, "LNAM")
            .and_then(|data| u32_at(data, 0))
            .unwrap_or_default(),
        lighting_template: None,
        raw_lighting: None,
        effective_lighting: None,
        water_form_id: sub_form_id(subs, "XCWT", resolver),
        water_height: sub(subs, "XCLW").and_then(|data| f32_at_option(data, 0)),
    })
}

fn parse_cell_metadata(subs: &[Subrecord], resolver: &FormIdResolver) -> CellMetadata {
    CellMetadata {
        acoustic_space_form_id: sub_form_id(subs, "XCAS", resolver),
        music_form_id: sub_form_id(subs, "XCMO", resolver),
        lighting_template_form_id: sub_form_id(subs, "LTMP", resolver),
        lighting_template_flags: sub(subs, "LNAM")
            .and_then(|data| u32_at(data, 0))
            .unwrap_or_default(),
        water_form_id: sub_form_id(subs, "XCWT", resolver),
        water_height: sub(subs, "XCLW").and_then(|data| f32_at_option(data, 0)),
        lighting: sub(subs, "XCLL").and_then(parse_lighting_data),
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "FULL", "DATA", "XCLL", "XCIM", "XCAS", "XCMO", "LTMP", "LNAM", "XCWT",
                "XCLW",
            ],
        ),
    }
}

fn parse_reference(
    subs: &[Subrecord],
    form_id: u32,
    parent_cell_form_id: u32,
    flags: u32,
    kind: ReferenceKind,
    resolver: &FormIdResolver,
) -> Result<Option<ReferenceRecord>> {
    let Some(base_form_id) = sub_form_id(subs, "NAME", resolver) else {
        return Ok(None);
    };
    let Some(data) = sub(subs, "DATA").filter(|data| data.len() >= 24) else {
        return Ok(None);
    };
    let teleport = sub(subs, "XTEL")
        .filter(|data| data.len() >= 28)
        .map(|data| TeleportRecord {
            door_reference_form_id: resolver
                .adjust(u32::from_le_bytes(data[0..4].try_into().unwrap())),
            position: [
                f32_at(data, 4).unwrap_or_default(),
                f32_at(data, 8).unwrap_or_default(),
                f32_at(data, 12).unwrap_or_default(),
            ],
            rotation: [
                f32_at(data, 16).unwrap_or_default(),
                f32_at(data, 20).unwrap_or_default(),
                f32_at(data, 24).unwrap_or_default(),
            ],
        });
    let lock = sub(subs, "XLOC").filter(|data| data.len() >= 8);
    let door = (teleport.is_some() || lock.is_some()).then(|| DoorRecord {
        lock_level: lock.map(|data| data[0] as i8),
        key_form_id: lock
            .map(|data| resolver.adjust(u32::from_le_bytes(data[4..8].try_into().unwrap())))
            .filter(|id| *id != 0),
        destination: None,
        teleport,
    });
    Ok(Some(ReferenceRecord {
        kind,
        form_id,
        parent_cell_form_id,
        base_form_id,
        position: [f32_at(data, 0)?, f32_at(data, 4)?, f32_at(data, 8)?],
        rotation: [f32_at(data, 12)?, f32_at(data, 16)?, f32_at(data, 20)?],
        scale: sub(subs, "XSCL")
            .and_then(|data| f32_at_option(data, 0))
            .unwrap_or(1.0),
        count: sub(subs, "XCNT")
            .and_then(|data| i32_at(data, 0))
            .unwrap_or(1),
        flags,
        door,
        owner_form_id: sub_form_id(subs, "XOWN", resolver),
        owner_faction_rank: sub(subs, "XRNK").and_then(|data| i32_at(data, 0)),
        enable_parent: sub(subs, "XESP")
            .filter(|data| data.len() >= 8)
            .map(|data| EnableParentRecord {
                parent_reference_form_id: resolver
                    .adjust(u32::from_le_bytes(data[0..4].try_into().unwrap())),
                flags: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            }),
        initially_enabled: flags & RECORD_DISABLED == 0,
        ignored_subrecords: ignored_signatures(
            subs,
            &[
                "EDID", "NAME", "DATA", "XSCL", "XCNT", "XTEL", "XLOC", "XOWN", "XRNK", "XESP",
            ],
        ),
    }))
}

fn ignored_signatures(subs: &[Subrecord], supported: &[&str]) -> Vec<String> {
    let mut signatures = subs
        .iter()
        .filter(|sub| !supported.contains(&sub.signature.as_str()))
        .map(|sub| sub.signature.clone())
        .collect::<Vec<_>>();
    signatures.sort();
    signatures.dedup();
    signatures
}

fn parse_navmesh(subs: &[Subrecord], form_id: u32, flags: u32, payload: Vec<u8>) -> NavMeshRecord {
    NavMeshRecord {
        form_id,
        flags,
        version: sub(subs, "NVER").and_then(|data| {
            data.get(..4)
                .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
        }),
        chunks: subs
            .iter()
            .map(|sub| NavMeshChunk {
                signature: sub.signature.clone(),
                byte_len: sub.data.len() as u32,
            })
            .collect(),
        payload,
    }
}

fn parse_image_space(subs: &[Subrecord], form_id: u32) -> Option<ImageSpaceInfo> {
    let data = sub(subs, "DNAM")?;
    let mut image_space = ImageSpaceInfo {
        form_id,
        editor_id: sub(subs, "EDID").map(cstring),
        ..ImageSpaceInfo::default()
    };
    image_space.eye_adapt_speed = f32_or(data, 0, image_space.eye_adapt_speed);
    image_space.hdr_blur_radius = f32_or(data, 4, image_space.hdr_blur_radius);
    image_space.hdr_blur_passes = f32_or(data, 8, image_space.hdr_blur_passes);
    image_space.hdr_emissive_multiplier = f32_or(data, 12, image_space.hdr_emissive_multiplier);
    image_space.hdr_target_lum = f32_or(data, 16, image_space.hdr_target_lum);
    image_space.hdr_upper_lum_clamp = f32_or(data, 20, image_space.hdr_upper_lum_clamp);
    image_space.hdr_bright_scale = f32_or(data, 24, image_space.hdr_bright_scale);
    image_space.hdr_bright_clamp = f32_or(data, 28, image_space.hdr_bright_clamp);
    image_space.hdr_lum_ramp_no_tex = f32_or(data, 32, image_space.hdr_lum_ramp_no_tex);
    image_space.hdr_lum_ramp_min = f32_or(data, 36, image_space.hdr_lum_ramp_min);
    image_space.hdr_lum_ramp_max = f32_or(data, 40, image_space.hdr_lum_ramp_max);
    image_space.hdr_sunlight_dimmer = f32_or(data, 44, image_space.hdr_sunlight_dimmer);
    image_space.hdr_grass_dimmer = f32_or(data, 48, image_space.hdr_grass_dimmer);
    image_space.hdr_tree_dimmer = f32_or(data, 52, image_space.hdr_tree_dimmer);
    image_space.hdr_skin_dimmer = f32_or(data, 56, image_space.hdr_skin_dimmer);
    image_space.bloom_blur_radius = f32_or(data, 60, image_space.bloom_blur_radius);
    image_space.bloom_alpha_mult_interior = f32_or(data, 64, image_space.bloom_alpha_mult_interior);
    image_space.bloom_alpha_mult_exterior = f32_or(data, 68, image_space.bloom_alpha_mult_exterior);
    image_space.get_hit_blur_radius = f32_or(data, 72, image_space.get_hit_blur_radius);
    image_space.get_hit_blur_damping_constant =
        f32_or(data, 76, image_space.get_hit_blur_damping_constant);
    image_space.get_hit_damping_constant = f32_or(data, 80, image_space.get_hit_damping_constant);
    image_space.night_eye_tint_rgb = rgb_or(data, 84, image_space.night_eye_tint_rgb);
    image_space.brightness = f32_or(data, 96, image_space.brightness);
    image_space.cinematic_saturation = f32_or(data, 100, image_space.cinematic_saturation);
    image_space.cinematic_contrast_avg_lum =
        f32_or(data, 104, image_space.cinematic_contrast_avg_lum);
    image_space.cinematic_contrast = f32_or(data, 108, image_space.cinematic_contrast);
    image_space.cinematic_brightness_tint_rgb =
        rgb_or(data, 112, image_space.cinematic_brightness_tint_rgb);
    image_space.cinematic_brightness_tint_value =
        f32_or(data, 124, image_space.cinematic_brightness_tint_value);
    image_space.flags = data.get(144).copied().unwrap_or_default();
    Some(image_space)
}

fn sub<'a>(subs: &'a [Subrecord], signature: &str) -> Option<&'a [u8]> {
    subs.iter()
        .find(|sub| sub.signature == signature)
        .map(|sub| sub.data.as_slice())
}

fn sub_form_id(subs: &[Subrecord], signature: &str, resolver: &FormIdResolver) -> Option<u32> {
    sub(subs, signature)
        .and_then(|data| data.get(..4))
        .map(|data| resolver.adjust(u32::from_le_bytes(data.try_into().unwrap())))
        .filter(|id| *id != 0)
}

fn cstring(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .trim_end_matches('\0')
        .to_string()
}

fn rgba8(data: &[u8]) -> [f32; 4] {
    [
        data[0] as f32 / 255.0,
        data[1] as f32 / 255.0,
        data[2] as f32 / 255.0,
        data[3] as f32 / 255.0,
    ]
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32> {
    data.get(offset..offset + 4)
        .context("u32 out of bounds")
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_i32(data: &[u8], offset: usize) -> Result<i32> {
    Ok(read_u32(data, offset)? as i32)
}

fn f32_at(data: &[u8], offset: usize) -> Result<f32> {
    data.get(offset..offset + 4)
        .context("f32 out of bounds")
        .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))
}

fn i32_at(data: &[u8], offset: usize) -> Option<i32> {
    data.get(offset..offset + 4)
        .map(|bytes| i32::from_le_bytes(bytes.try_into().unwrap()))
}

fn i16_at(data: &[u8], offset: usize) -> Option<i16> {
    data.get(offset..offset + 2)
        .map(|bytes| i16::from_le_bytes(bytes.try_into().unwrap()))
}

fn u32_at(data: &[u8], offset: usize) -> Option<u32> {
    data.get(offset..offset + 4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn f32_at_option(data: &[u8], offset: usize) -> Option<f32> {
    data.get(offset..offset + 4)
        .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))
}

fn f32_or(data: &[u8], offset: usize, fallback: f32) -> f32 {
    f32_at_option(data, offset).unwrap_or(fallback)
}

fn rgb_or(data: &[u8], offset: usize, fallback: [f32; 3]) -> [f32; 3] {
    [
        f32_or(data, offset, fallback[0]),
        f32_or(data, offset + 4, fallback[1]),
        f32_or(data, offset + 8, fallback[2]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parses_fo3_lighting_data_in_legacy_and_complete_lengths() {
        let mut data = vec![0_u8; 40];
        data[0..4].copy_from_slice(&[16, 32, 64, 255]);
        data[4..8].copy_from_slice(&[32, 64, 128, 255]);
        data[8..12].copy_from_slice(&[8, 16, 32, 255]);
        data[12..16].copy_from_slice(&1.5_f32.to_le_bytes());
        data[16..20].copy_from_slice(&25.0_f32.to_le_bytes());
        data[20..24].copy_from_slice(&(-10_i32).to_le_bytes());
        data[24..28].copy_from_slice(&20_i32.to_le_bytes());
        data[28..32].copy_from_slice(&0.75_f32.to_le_bytes());
        data[32..36].copy_from_slice(&40.0_f32.to_le_bytes());
        data[36..40].copy_from_slice(&2.0_f32.to_le_bytes());

        let complete = parse_lighting_data(&data).unwrap();
        let legacy = parse_lighting_data(&data[..36]).unwrap();
        assert_eq!(complete.ambient_rgba[0], 16.0 / 255.0);
        assert_eq!(complete.rotation_xy, -10);
        assert_eq!(complete.rotation_z, 20);
        assert_eq!(complete.fog_power, 2.0);
        assert_eq!(legacy.fog_power, 1.0);
        assert!(parse_lighting_data(&data[..35]).is_none());
        let mut ambiguous = data[..37].to_vec();
        assert!(parse_lighting_data(&ambiguous).is_none());
        ambiguous.resize(40, 0);
        assert!(parse_lighting_data(&ambiguous).is_some());
        let template = parse_lighting_template(
            &[Subrecord {
                signature: "DATA".into(),
                data,
            }],
            0x123,
            0,
        );
        assert_eq!(template.lighting.unwrap().fog_power, 2.0);
    }

    #[test]
    fn applies_every_named_lighting_template_field() {
        let mut cell = LightingData {
            ambient_rgba: [1.0; 4],
            directional_rgba: [1.0; 4],
            fog_rgba: [1.0; 4],
            fog_near: 1.0,
            fog_far: 1.0,
            rotation_xy: 1,
            rotation_z: 1,
            fog_directional_fade: 1.0,
            fog_clip_distance: 1.0,
            fog_power: 1.0,
        };
        let template = LightingData {
            ambient_rgba: [2.0; 4],
            directional_rgba: [3.0; 4],
            fog_rgba: [4.0; 4],
            fog_near: 5.0,
            fog_far: 6.0,
            rotation_xy: 7,
            rotation_z: 8,
            fog_directional_fade: 9.0,
            fog_clip_distance: 10.0,
            fog_power: 11.0,
        };
        cell.apply_template(template, 0x1ff);
        assert_eq!(cell, template);
    }

    fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut result = signature.to_vec();
        result.extend_from_slice(&(data.len() as u16).to_le_bytes());
        result.extend_from_slice(data);
        result
    }

    fn record(signature: &[u8; 4], flags: u32, form_id: u32, data: &[u8]) -> Vec<u8> {
        let mut result = signature.to_vec();
        result.extend_from_slice(&(data.len() as u32).to_le_bytes());
        result.extend_from_slice(&flags.to_le_bytes());
        result.extend_from_slice(&form_id.to_le_bytes());
        result.extend_from_slice(&[0; 8]);
        result.extend_from_slice(data);
        result
    }

    fn group(label: u32, group_type: i32, children: &[u8]) -> Vec<u8> {
        let mut result = b"GRUP".to_vec();
        result.extend_from_slice(&((children.len() + 24) as u32).to_le_bytes());
        result.extend_from_slice(&label.to_le_bytes());
        result.extend_from_slice(&group_type.to_le_bytes());
        result.extend_from_slice(&[0; 8]);
        result.extend_from_slice(children);
        result
    }

    fn tes4(masters: &[&str]) -> Vec<u8> {
        let mut data = Vec::new();
        for master in masters {
            data.extend(subrecord(b"MAST", format!("{master}\0").as_bytes()));
            data.extend(subrecord(b"DATA", &[0; 8]));
        }
        record(b"TES4", 0, 0, &data)
    }

    fn transform() -> Vec<u8> {
        [1.0_f32, 2.0, 3.0, 0.1, 0.2, 0.3]
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect()
    }

    #[test]
    fn reads_master_dependencies() {
        assert_eq!(
            read_master_names(&tes4(&["Fallout3.esm", "Update.esp"])).unwrap(),
            ["Fallout3.esm", "Update.esp"]
        );
    }

    #[test]
    fn extended_subrecords_use_xxxx_size() {
        let mut data = subrecord(b"XXXX", &70_000_u32.to_le_bytes());
        data.extend_from_slice(b"BIGG");
        data.extend_from_slice(&0_u16.to_le_bytes());
        data.extend(vec![7; 70_000]);
        let parsed = parse_subrecords(&data).unwrap();
        assert_eq!(parsed[0].signature, "BIGG");
        assert_eq!(parsed[0].data.len(), 70_000);
    }

    #[test]
    fn compressed_records_are_validated_and_decoded() {
        let original = subrecord(b"EDID", b"Compressed\0");
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&original).unwrap();
        let compressed = encoder.finish().unwrap();
        let mut payload = (original.len() as u32).to_le_bytes().to_vec();
        payload.extend(compressed);
        assert_eq!(
            record_payload(&payload, RECORD_COMPRESSED, "TEST", 1).unwrap(),
            original
        );
    }

    #[test]
    fn parses_audio_cell_and_enable_parent_metadata() {
        let resolver = FormIdResolver {
            current_index: 0,
            master_indices: Vec::new(),
        };
        let sound = parse_sound(
            &[
                Subrecord {
                    signature: "EDID".into(),
                    data: b"RoomTone\0".to_vec(),
                },
                Subrecord {
                    signature: "FNAM".into(),
                    data: b"fx/roomtone.wav\0".to_vec(),
                },
                Subrecord {
                    signature: "SNDX".into(),
                    data: [2_u8, 9, 0, 0, 0x50, 0, 0, 0, 0x6a, 0x02, 0, 0].to_vec(),
                },
            ],
            0x100,
            0,
        );
        assert_eq!(sound.file.as_deref(), Some("fx/roomtone.wav"));
        assert!(sound.parameters.unwrap().is_looping());
        assert!(sound.parameters.unwrap().is_two_dimensional());

        let acoustic = parse_acoustic_space(
            &[
                Subrecord {
                    signature: "EDID".into(),
                    data: b"Metal\0".to_vec(),
                },
                Subrecord {
                    signature: "ANAM".into(),
                    data: 7_u32.to_le_bytes().to_vec(),
                },
                Subrecord {
                    signature: "SNAM".into(),
                    data: 0x200_u32.to_le_bytes().to_vec(),
                },
            ],
            0x101,
            0,
            &resolver,
        );
        assert_eq!(acoustic.environment_type, Some(7));
        assert_eq!(acoustic.ambient_loop_sound_form_ids, [0x200]);

        let metadata = parse_cell_metadata(
            &[
                Subrecord {
                    signature: "XCAS".into(),
                    data: 0x101_u32.to_le_bytes().to_vec(),
                },
                Subrecord {
                    signature: "XCMO".into(),
                    data: 0x300_u32.to_le_bytes().to_vec(),
                },
                Subrecord {
                    signature: "XCLW".into(),
                    data: 1.5_f32.to_le_bytes().to_vec(),
                },
            ],
            &resolver,
        );
        assert_eq!(metadata.acoustic_space_form_id, Some(0x101));
        assert_eq!(metadata.music_form_id, Some(0x300));
        assert_eq!(metadata.water_height, Some(1.5));

        let reference = parse_reference(
            &[
                Subrecord {
                    signature: "NAME".into(),
                    data: 0x400_u32.to_le_bytes().to_vec(),
                },
                Subrecord {
                    signature: "DATA".into(),
                    data: transform(),
                },
                Subrecord {
                    signature: "XOWN".into(),
                    data: 0x500_u32.to_le_bytes().to_vec(),
                },
                Subrecord {
                    signature: "XRNK".into(),
                    data: (-2_i32).to_le_bytes().to_vec(),
                },
                Subrecord {
                    signature: "XESP".into(),
                    data: [0x600_u32.to_le_bytes(), 2_u32.to_le_bytes()].concat(),
                },
            ],
            0x700,
            0x800,
            0,
            ReferenceKind::Object,
            &resolver,
        )
        .unwrap()
        .unwrap();
        assert_eq!(reference.owner_form_id, Some(0x500));
        assert_eq!(reference.owner_faction_rank, Some(-2));
        assert_eq!(
            reference.enable_parent.unwrap().parent_reference_form_id,
            0x600
        );
    }

    #[test]
    fn parses_actors_items_doors_and_navmesh() {
        let cell_id = 0x100;
        let destination_cell = 0x200;
        let door_ref = 0x300;
        let destination_ref = 0x301_u32;
        let door_base = 0x400;
        let npc_base = 0x401;
        let item_base = 0x402;
        let creature_base = 0x403;

        let cell = record(
            b"CELL",
            0,
            cell_id,
            &[subrecord(b"EDID", b"TestCell\0"), subrecord(b"DATA", &[1])].concat(),
        );
        let destination = record(
            b"CELL",
            0,
            destination_cell,
            &[
                subrecord(b"EDID", b"Destination\0"),
                subrecord(b"DATA", &[1]),
            ]
            .concat(),
        );
        let door = record(
            b"DOOR",
            0,
            door_base,
            &[
                subrecord(b"EDID", b"DoorBase\0"),
                subrecord(b"FULL", b"Door\0"),
                subrecord(b"MODL", b"door.nif\0"),
            ]
            .concat(),
        );
        let npc = record(
            b"NPC_",
            0,
            npc_base,
            &[
                subrecord(b"FULL", b"Dweller\0"),
                subrecord(b"TPLT", &9_u32.to_le_bytes()),
            ]
            .concat(),
        );
        let item = record(
            b"MISC",
            0,
            item_base,
            &[
                subrecord(b"FULL", b"Scrap\0"),
                subrecord(b"MODL", b"scrap.nif\0"),
                subrecord(
                    b"DATA",
                    &[
                        12_i32.to_le_bytes().as_slice(),
                        2.5_f32.to_le_bytes().as_slice(),
                    ]
                    .concat(),
                ),
            ]
            .concat(),
        );
        let creature = record(
            b"CREA",
            0,
            creature_base,
            &[
                subrecord(b"FULL", b"Mole Rat\0"),
                subrecord(b"MODL", b"molerat.nif\0"),
            ]
            .concat(),
        );
        let mut xtel = destination_ref.to_le_bytes().to_vec();
        xtel.extend(transform());
        let mut lock = vec![50, 0, 0, 0];
        lock.extend(item_base.to_le_bytes());
        lock.extend([0; 12]);
        let placed_door = record(
            b"REFR",
            0,
            door_ref,
            &[
                subrecord(b"NAME", &door_base.to_le_bytes()),
                subrecord(b"XTEL", &xtel),
                subrecord(b"XLOC", &lock),
                subrecord(b"UNKN", &[1, 2, 3]),
                subrecord(b"DATA", &transform()),
            ]
            .concat(),
        );
        let placed_item = record(
            b"REFR",
            0,
            0x302,
            &[
                subrecord(b"NAME", &item_base.to_le_bytes()),
                subrecord(b"XCNT", &3_i32.to_le_bytes()),
                subrecord(b"DATA", &transform()),
            ]
            .concat(),
        );
        let placed_npc = record(
            b"ACHR",
            0,
            0x303,
            &[
                subrecord(b"NAME", &npc_base.to_le_bytes()),
                subrecord(b"DATA", &transform()),
            ]
            .concat(),
        );
        let placed_creature = record(
            b"ACRE",
            0,
            0x304,
            &[
                subrecord(b"NAME", &creature_base.to_le_bytes()),
                subrecord(b"DATA", &transform()),
            ]
            .concat(),
        );
        let navmesh = record(
            b"NAVM",
            0,
            0x500,
            &[
                subrecord(b"NVER", &12_u32.to_le_bytes()),
                subrecord(b"NVVX", &[0; 12]),
            ]
            .concat(),
        );
        let destination_door = record(
            b"REFR",
            0,
            destination_ref,
            &[
                subrecord(b"NAME", &door_base.to_le_bytes()),
                subrecord(b"DATA", &transform()),
            ]
            .concat(),
        );
        let mut plugin = tes4(&[]);
        plugin.extend(cell);
        plugin.extend(destination);
        plugin.extend(door);
        plugin.extend(npc);
        plugin.extend(item);
        plugin.extend(creature);
        plugin.extend(group(
            cell_id,
            6,
            &[
                placed_door,
                placed_item,
                placed_npc,
                placed_creature,
                navmesh,
            ]
            .concat(),
        ));
        plugin.extend(group(destination_cell, 6, &destination_door));

        let parsed = parse_plugin(&plugin, cell_id).unwrap();
        assert_eq!(parsed.references.len(), 4);
        assert_eq!(parsed.navmeshes.len(), 1);
        assert_eq!(parsed.navmeshes[0].version, Some(12));
        let item_ref = parsed
            .references
            .iter()
            .find(|reference| reference.base_form_id == item_base)
            .unwrap();
        assert_eq!(item_ref.count, 3);
        assert_eq!(parsed.bases[&item_base].value, Some(12));
        assert_eq!(parsed.bases[&item_base].weight, Some(2.5));
        let door = parsed
            .references
            .iter()
            .find(|reference| reference.form_id == door_ref)
            .unwrap()
            .door
            .as_ref()
            .unwrap();
        assert_eq!(
            door.destination.as_ref().unwrap().cell_form_id,
            destination_cell
        );
        assert_eq!(door.lock_level, Some(50));
        assert_eq!(door.key_form_id, Some(item_base));
        assert_eq!(
            parsed
                .references
                .iter()
                .find(|reference| reference.kind == ReferenceKind::Npc)
                .unwrap()
                .base_form_id,
            npc_base
        );
        assert!(
            parsed
                .references
                .iter()
                .any(|reference| reference.kind == ReferenceKind::Creature)
        );
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("REFR.UNKN"))
        );
    }

    #[test]
    fn adjusts_master_form_ids_and_applies_later_overrides() {
        let cell_id = 0x100;
        let base_id = 0x400;
        let reference_id = 0x0100_0300;
        let mut master = tes4(&[]);
        master.extend(record(b"CELL", 0, cell_id, &subrecord(b"DATA", &[1])));
        master.extend(record(
            b"MISC",
            0,
            base_id,
            &[
                subrecord(b"FULL", b"Original\0"),
                subrecord(b"DATA", &[0; 8]),
            ]
            .concat(),
        ));

        let mut override_plugin = tes4(&["Master.esm"]);
        override_plugin.extend(record(
            b"MISC",
            0,
            base_id,
            &[
                subrecord(b"FULL", b"Overridden\0"),
                subrecord(b"DATA", &[0; 8]),
            ]
            .concat(),
        ));
        override_plugin.extend(group(
            cell_id,
            6,
            &record(
                b"REFR",
                0,
                reference_id,
                &[
                    subrecord(b"NAME", &base_id.to_le_bytes()),
                    subrecord(b"DATA", &transform()),
                ]
                .concat(),
            ),
        ));

        let parsed = parse_content_set(
            &[
                PluginSource {
                    name: "Master.esm",
                    bytes: &master,
                },
                PluginSource {
                    name: "Patch.esp",
                    bytes: &override_plugin,
                },
            ],
            cell_id,
        )
        .unwrap();
        assert_eq!(parsed.references[0].form_id, reference_id);
        assert_eq!(parsed.references[0].base_form_id, base_id);
        assert_eq!(parsed.bases[&base_id].name.as_deref(), Some("Overridden"));
    }

    #[test]
    #[ignore = "requires BEVYOUT_FALLOUT3_ESM pointing to local Fallout3.esm"]
    fn fallout3_cell_smoke_baselines() {
        let path = std::env::var("BEVYOUT_FALLOUT3_ESM")
            .expect("set BEVYOUT_FALLOUT3_ESM to Fallout3.esm");
        let bytes = std::fs::read(path).unwrap();
        for (cell, refr, achr, acre, navm, teleports) in [
            (0x0001_51e3, 928, 0, 1, 1, 1),
            (0x0001_7f37, 1_756, 10, 1, 1, 2),
            (0x000c_c067, 483, 1, 0, 1, 1),
        ] {
            let parsed = parse_plugin(&bytes, cell).unwrap();
            assert_eq!(
                parsed
                    .references
                    .iter()
                    .filter(|reference| reference.kind == ReferenceKind::Object)
                    .count(),
                refr
            );
            assert_eq!(
                parsed
                    .references
                    .iter()
                    .filter(|reference| reference.kind == ReferenceKind::Npc)
                    .count(),
                achr
            );
            assert_eq!(
                parsed
                    .references
                    .iter()
                    .filter(|reference| reference.kind == ReferenceKind::Creature)
                    .count(),
                acre
            );
            assert_eq!(parsed.navmeshes.len(), navm);
            assert_eq!(
                parsed
                    .references
                    .iter()
                    .filter(|reference| reference
                        .door
                        .as_ref()
                        .and_then(|door| door.destination.as_ref())
                        .is_some())
                    .count(),
                teleports
            );
        }
    }
}
