//! Focused Rust adaptation of OpenMW's ESM4 reader and record layouts.
//!
//! This is private to the Fallout cell vertical slice. See README.md and
//! NOTICE.md in this directory for provenance and license information.

use anyhow::{Context, Result, anyhow, bail};
use flate2::read::ZlibDecoder;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::io::{Cursor, Read};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use bevyout_core::form_id::FormIdResolver;
use bevyout_core::image_space::ImageSpaceModifier;
use bevyout_core::time_of_day::{ColorKeyframes, DayNightTimings};

use super::manifest::{CellInfo, ImageSpaceInfo};
use super::paths::CellSelector;

mod actor_support;
mod actors;
mod binary;
mod enable;
mod idle;
mod inventory;
mod navmesh;
mod reader;
mod records;

pub(crate) use actor_support::*;
pub(crate) use actors::*;
pub(crate) use binary::*;
pub(crate) use enable::*;
pub(crate) use idle::*;
pub(crate) use inventory::*;
pub(crate) use navmesh::*;
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

#[derive(Debug, Clone, Default)]
pub(crate) struct BaseRecord {
    pub(crate) kind: String,
    /// Record header flags (quest item is `0x0000_0400`), stamped by the
    /// reader after `parse_base` since only the header carries them.
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) model: Option<String>,
    /// Fallout 3 ARMO keeps worn and world models in separate fields. `model`
    /// remains the standalone world model used by pickup preparation; actor
    /// appearance consumes this sex-specific set instead.
    pub(crate) apparel_models: Option<ApparelModelSet>,
    pub(crate) icon: Option<String>,
    pub(crate) mini_icon: Option<String>,
    pub(crate) value: Option<i32>,
    pub(crate) weight: Option<f32>,
    pub(crate) item_stats: OpenMwItemStats,
    pub(crate) base_template_form_id: Option<u32>,
    pub(crate) light: Option<LightData>,
    pub(crate) inventory: Vec<InventoryItemRecord>,
    pub(crate) audio: BaseAudioRecord,
    /// Present only for `LVLI`/`LVLN`/`LVLC` base records (issue #74). See
    /// `records::LeveledListData` for the parsed `LVLD`/`LVLF`/`LVLO` body.
    pub(crate) leveled: Option<LeveledListData>,
    /// Present only for `NPC_`/`CREA` base records (issue #103, M4 wave 1
    /// task A). See `actors::ActorData` for the parsed actor subrecords.
    /// Consumed by task C's actor-catalog resolution
    /// (`prepare::orchestrator::actor_record_input`).
    pub(crate) actor: Option<ActorData>,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ApparelModelSet {
    pub(crate) male_worn: Option<String>,
    pub(crate) male_world: Option<String>,
    pub(crate) female_worn: Option<String>,
    pub(crate) female_world: Option<String>,
}

/// Item fields decoded from the exact OpenMW ESM4 layouts listed in
/// `NOTICE.md`. Presentation-independent so preparation can map them into the
/// stable item catalogue without importing OpenMW runtime classes.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum OpenMwItemStats {
    Weapon {
        damage: Option<u16>,
        max_condition: Option<u32>,
        clip_size: Option<u8>,
        speed: Option<f32>,
        reach: Option<f32>,
        /// Issue #98 (F98.1): the `WEAP.NAM0` ammo base form id (FO3's `Ammo`
        /// field, `ESM4::Weapon` in OpenMW's `components/esm4/loadweap.cpp`
        /// treats it as an opaque skipped subrecord; this port decodes it as
        /// a FormID reference the way `YNAM`/`ZNAM` already are).
        ammo_form_id: Option<u32>,
        /// Fallout 3 `WEAP.DNAM` animation type (the first 32-bit field).
        animation_type: Option<u32>,
        /// Fallout 3 `WEAP.WNAM` first-person model-object FormID.
        first_person_model_object_form_id: Option<u32>,
    },
    Apparel {
        armor_rating: Option<f32>,
        max_condition: Option<u32>,
        /// Issue #98 (F98.1): the FO3 `ARMO.BMDT` biped-slot mask (first four
        /// bytes of the subrecord in both the FO3 and TES4 layouts --
        /// `ESM4::Armor::mArmorFlags` in OpenMW's `components/esm4/loadarmo.cpp`).
        biped_slot_mask: Option<u32>,
    },
    Ammo {
        damage: Option<f32>,
        speed: Option<f32>,
    },
    Aid {
        effect_form_ids: Vec<u32>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InventoryItemRecord {
    pub(crate) item_form_id: u32,
    pub(crate) count: u32,
}

/// One `RCIL`/`RCOD` + `RCQY` pair in an ESM4 recipe.  The source format
/// stores quantities as unsigned 32-bit values; preparation narrows them to
/// the positive signed count used by the existing item/transfer seams and
/// diagnoses values that cannot be represented there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecipeItemRecord {
    pub(crate) item_form_id: u32,
    pub(crate) quantity: i32,
    pub(crate) order: u32,
}

/// Fallout 3/New Vegas `RCPE` recipe metadata.  `CTDA` payloads remain opaque
/// until a later condition evaluator exists; retaining them keeps preparation
/// lossless without pretending to execute recipe conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecipeRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) skill: i32,
    pub(crate) level: u32,
    pub(crate) category_form_id: Option<u32>,
    pub(crate) sub_category_form_id: Option<u32>,
    pub(crate) ingredients: Vec<RecipeItemRecord>,
    pub(crate) outputs: Vec<RecipeItemRecord>,
    pub(crate) conditions: Vec<Vec<u8>>,
    pub(crate) ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BaseAudioRecord {
    pub(crate) loop_sound_form_id: Option<u32>,
    pub(crate) activation_sound_form_id: Option<u32>,
    pub(crate) open_sound_form_id: Option<u32>,
    pub(crate) close_sound_form_id: Option<u32>,
    pub(crate) pickup_sound_form_id: Option<u32>,
    pub(crate) drop_sound_form_id: Option<u32>,
    /// Fallout 3 `WEAP.SNAM` spatial attack sound.
    pub(crate) weapon_fire_3d_sound_form_id: Option<u32>,
    /// Fallout 3 `WEAP.XNAM` player-local attack sound.
    pub(crate) weapon_fire_2d_sound_form_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub(crate) struct LightData {
    pub(crate) radius: f32,
    pub(crate) color_rgba: [f32; 4],
    pub(crate) flags: u32,
    pub(crate) falloff_exponent: f32,
    pub(crate) fov: f32,
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
    /// Root FormID of this reference's enable-parent chain (the top-most
    /// ancestor with no `enable_parent` of its own), resolved when
    /// `enable_parent` is `Some`. `None` when there is no chain, or the
    /// chain could not be resolved (unresolved/cyclic XESP).
    pub(crate) enable_root_form_id: Option<u32>,
    /// `XLKR`: the FormID of this reference's linked reference (issue #213),
    /// resolver-adjusted and `!= 0`-filtered like every other `sub_form_id`
    /// read. Authored on both actor placements (the first patrol marker) and
    /// on the markers themselves (the next marker in the chain) -- confirmed
    /// empirically against real Fallout3.esm `SuperDuperMart` data at plan
    /// time (`00017f37`'s `LvlRaiderMelee` ACHR `00041609` links to marker
    /// `0004160a`, which links to `0004160b`, which links to `00041617`,
    /// which has no further link).
    pub(crate) linked_reference_form_id: Option<u32>,
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
    /// Issue #185: see `PreparedDoor::trapped`'s doc comment -- always
    /// `false` from real ESM data today (`records::parse_reference`), no
    /// known FO3 trap subrecord to source it from yet.
    pub(crate) trapped: bool,
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

/// Resolves an `XTEL` teleport target against the content set's references,
/// shared by the per-cell resolution in `ParsedContentSet::select` and the
/// content-set-wide door graph in `ParsedContentSet::door_edges` so the
/// "look up the destination reference's parent cell" logic exists once.
fn resolve_teleport_destination(
    teleport: &TeleportRecord,
    all_references: &HashMap<u32, ReferenceRecord>,
) -> Option<DoorDestinationRecord> {
    all_references
        .get(&teleport.door_reference_form_id)
        .map(|destination| DoorDestinationRecord {
            door_reference_form_id: teleport.door_reference_form_id,
            cell_form_id: destination.parent_cell_form_id,
            position: teleport.position,
            rotation: teleport.rotation,
        })
}

/// A `WRLD` worldspace record (FormID, EDID, FULL). See
/// `records::parse_worldspace` for provenance.
#[derive(Debug, Clone)]
pub(crate) struct WorldspaceRecord {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) parent_form_id: Option<u32>,
    pub(crate) parent_flags: u8,
    pub(crate) climate_form_id: Option<u32>,
}

impl WorldspaceRecord {
    pub(crate) const USE_PARENT_CLIMATE: u8 = 0x10;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ClimateWeatherEntry {
    pub(crate) weather_form_id: u32,
    pub(crate) chance: i32,
}

#[derive(Debug, Clone)]
pub(crate) struct ClimateRecord {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) weather_entries: Vec<ClimateWeatherEntry>,
    pub(crate) timings: DayNightTimings,
}

#[derive(Debug, Clone)]
pub(crate) struct WeatherRecord {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) sky_upper: ColorKeyframes,
    pub(crate) sky_lower: ColorKeyframes,
    pub(crate) ambient: ColorKeyframes,
    pub(crate) sunlight: ColorKeyframes,
}

/// Prepared source data from one exterior `LAND` record.  The parser keeps
/// this compact, source-shaped representation; terrain mesh generation lives
/// in the exterior preparation slice.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct LandRecord {
    pub(crate) form_id: u32,
    pub(crate) cell_form_id: u32,
    pub(crate) heights: Vec<f32>,
    pub(crate) normals: Vec<[i8; 3]>,
    pub(crate) colors: Vec<[u8; 3]>,
    pub(crate) texture_layers: Vec<u32>,
    pub(crate) texture_assignments: Vec<LandTextureAssignment>,
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LandTextureAssignment {
    pub(crate) form_id: u32,
    pub(crate) quadrant: u8,
    /// `ATXT.layerIndex` is a little-endian `u16`; byte 5 in the record is
    /// padding/unknown data, not the layer number.
    pub(crate) layer: u16,
    pub(crate) base: bool,
    pub(crate) weights: Vec<LandTextureWeight>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LandTextureWeight {
    pub(crate) position: u16,
    pub(crate) opacity: f32,
}

/// The focused LTEX/TXST fields needed by the prepared exterior material.
/// FO3 stores the authoritative diffuse source in TXST.TX00 and the
/// DirectX tangent-space normal/specular source in TXST.TX01.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct LandscapeTextureRecord {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) texture_set_form_id: Option<u32>,
    pub(crate) diffuse_path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct TextureSetRecord {
    pub(crate) form_id: u32,
    pub(crate) diffuse_path: Option<String>,
    pub(crate) normal_path: Option<String>,
}

impl LandRecord {
    pub(crate) const GRID_SIZE: usize = 33;

    pub(crate) fn is_complete(&self) -> bool {
        let count = Self::GRID_SIZE * Self::GRID_SIZE;
        self.heights.len() == count
    }
}

/// A directed door edge in the content-set-wide connectivity graph produced
/// by `ParsedContentSet::door_edges` (issue #45, F45.3): one per resolvable
/// `XTEL` teleport, independent of which single cell is being prepared.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DoorEdgeRecord {
    pub(crate) source_cell_form_id: u32,
    pub(crate) door_reference_form_id: u32,
    pub(crate) destination_cell_form_id: u32,
    pub(crate) destination_door_reference_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) rotation: [f32; 3],
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

/// Decoded `GMST` game setting (M9 wave 1 #308). The EditorID is the
/// setting name; the typed `DATA` value follows its prefix.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct GmstRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) value: Option<bevyout_core::stats::GmstValue>,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// Decoded `AVIF` actor-value metadata (M9 wave 1 #308).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct AvifRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// One raw `PERK` `CTDA` condition (M9 wave 2 #312): the wire words are
/// kept as-is because only `GetActorValue` (function `0x1EF`) conditions
/// with the greater-or-equal oper (`0x60`) are resolvable against a
/// character sheet; the AV index itself is engine-internal (see
/// `bevyout_core::perks::actor_value_from_condition_index`).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PerkConditionWire {
    pub(crate) oper: u8,
    pub(crate) comparison_value: f32,
    pub(crate) function: u32,
    pub(crate) param1: u32,
}

/// One raw `PERK` entry between `PRKE`/`PRKF` (M9 wave 2 #312), typed by
/// the `PRKE` entry kind. Inner payloads stay raw; the perk catalog's
/// boundary conversion interprets entry-point parameters.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PerkEntryWire {
    /// `PRKE` type 0: quest FormID plus an undecoded second word.
    Quest {
        rank: u8,
        priority: u8,
        quest_form_id: u32,
        unknown: u32,
    },
    /// `PRKE` type 1: the ability `SPEL` FormID.
    Ability {
        rank: u8,
        priority: u8,
        spell_form_id: u32,
    },
    /// `PRKE` type 2: entry-point code plus `EPFT`/`EPFD` (the `EPFD`
    /// bytes are kept as a raw word; only `EPFT == 1` floats are
    /// interpreted later).
    EntryPoint {
        rank: u8,
        priority: u8,
        code: u8,
        param_count: u8,
        entry_priority: u8,
        function: Option<u8>,
        data: Option<u32>,
    },
}

/// Decoded `PERK` record (M9 wave 2 #312): `DATA` level/rank gates, raw
/// `CTDA` conditions, and typed `PRKE`..`PRKF` entries.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct PerkRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) min_level: u8,
    pub(crate) ranks: u8,
    pub(crate) playable: bool,
    pub(crate) hidden: bool,
    pub(crate) conditions: Vec<PerkConditionWire>,
    pub(crate) entries: Vec<PerkEntryWire>,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// One raw `ALCH` `CTDA` condition (M9 wave 3 #316). Same 28-byte wire
/// layout as `PerkConditionWire`; kept as its own type because ingestible
/// conditions gate effect items, not perk ownership, and wave 3 stores
/// them without running them.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EffectConditionWire {
    pub(crate) oper: u8,
    pub(crate) comparison_value: f32,
    pub(crate) function: u32,
    pub(crate) param1: u32,
}

/// `ALCH.ENIT` body (M9 wave 3 #316), verified against the real GOTY
/// `Fallout3.esm` with fopdoc's Fallout 3 layout: 20 bytes =
/// `{ i32 value_caps, u8 flags, [3 pad 0xCD], u32 withdrawal_spell_formid,
/// f32 addiction_chance, u32 consume_sound_formid }`.
/// Ground truth: Jet's withdrawal SPEL `WithdrawalJet` (`00033067`) with a
/// 20% chance and the consume sound `NPCHumanUsingJet` (`0008c77b`);
/// Stimpak has zero chance and no withdrawal effect.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AlchEnit {
    pub(crate) value_caps: u32,
    /// 0x01 no auto-calc, 0x02 food item, 0x04 medicine.
    pub(crate) flags: u8,
    pub(crate) withdrawal_spell_form_id: u32,
    /// Base percentage chance of addiction as authored (0.0..=1.0 scale
    /// observed: 0.2 = 20% for Jet).
    pub(crate) addiction_chance: f32,
    pub(crate) consume_sound_form_id: u32,
}

/// One `ALCH` effect item: an `EFID`/`EFIT` pair plus an optional trailing
/// `CTDA` (M9 wave 3 #316). `EFIT` is 20 bytes =
/// `{ i32 magnitude, u32 area, u32 duration_seconds, i32 range, u32
/// primary_actor_value_index }`; the trailing index duplicates the MGEF's
/// primary actor value (Jet's `ChemIncAPJet` carries 12 = ActionPoints on
/// both the MGEF and every `EFIT`).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AlchEffectWire {
    pub(crate) mgef_form_id: u32,
    pub(crate) magnitude: i32,
    pub(crate) area: u32,
    /// Authored duration in game seconds (Jet's AP boost is 240; the
    /// engine-builtin UMON monitor effect carries 108000).
    pub(crate) duration_seconds: u32,
    pub(crate) range: i32,
    pub(crate) condition: Option<EffectConditionWire>,
}

/// Decoded `ALCH` ingestible record (M9 wave 3 #316). `DATA` is the weight
/// f32 (also read by `parse_value_weight` for the item catalog); `ENIT`
/// carries the addiction facts the chem engine (#317) keys on.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct AlchRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) weight: Option<f32>,
    pub(crate) enit: Option<AlchEnit>,
    pub(crate) effects: Vec<AlchEffectWire>,
    pub(crate) ignored_subrecords: Vec<String>,
}

/// Decoded `MGEF` base-effect record (M9 wave 3 #316). `DATA` is 72 bytes;
/// the fields this slice consumes are the flags dword at offset 0 (low
/// bits: 0x02 Recover, 0x04 Detrimental, delivery bits 0x10..0x40), the
/// base-cost f32 at 4, the archetype u32 at 64 (0 Value Modifier, 1
/// Script, 34 Value And Parts -- Stimpak's restore), and the primary
/// actor-value i32 at 68 using the engine AV index family also seen in
/// perk `CTDA` conditions (see
/// `bevyout_core::effects::actor_value_from_effect_index`).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct MgefRecord {
    pub(crate) form_id: u32,
    pub(crate) record_flags: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) flags: u32,
    pub(crate) base_cost: f32,
    pub(crate) archetype: u32,
    pub(crate) actor_value_index: i32,
    pub(crate) ignored_subrecords: Vec<String>,
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
    pub(crate) climate_form_id: Option<u32>,
    pub(crate) acoustic_space_form_id: Option<u32>,
    pub(crate) music_form_id: Option<u32>,
    pub(crate) lighting_template_form_id: Option<u32>,
    pub(crate) lighting_template_flags: u32,
    pub(crate) water_form_id: Option<u32>,
    pub(crate) water_height: Option<f32>,
    pub(crate) lighting: Option<LightingData>,
    ignored_subrecords: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SharedCatalog<K, V>(Arc<HashMap<K, V>>);

impl<K, V> Default for SharedCatalog<K, V> {
    fn default() -> Self {
        Self(Arc::new(HashMap::new()))
    }
}

impl<K, V> From<HashMap<K, V>> for SharedCatalog<K, V> {
    fn from(values: HashMap<K, V>) -> Self {
        Self(Arc::new(values))
    }
}

impl<K, V> Deref for SharedCatalog<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for SharedCatalog<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl<'a, K, V> IntoIterator for &'a SharedCatalog<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<K, V> SharedCatalog<K, V> {
    #[cfg(test)]
    fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Default)]
pub(crate) struct ParsedPlugin {
    pub(crate) bases: SharedCatalog<u32, BaseRecord>,
    pub(crate) recipes: SharedCatalog<u32, RecipeRecord>,
    // M4 wave 1 task B (#103): decoded for the actor catalog resolver built
    // in task C (`prepare::orchestrator::build_actor_catalog_inputs`).
    pub(crate) races: SharedCatalog<u32, RaceRecord>,
    pub(crate) classes: SharedCatalog<u32, ClassRecord>,
    pub(crate) factions: SharedCatalog<u32, FactionRecord>,
    pub(crate) packages: SharedCatalog<u32, PackageRecord>,
    pub(crate) idles: SharedCatalog<u32, IdleRecord>,
    pub(crate) image_spaces: SharedCatalog<u32, ImageSpaceInfo>,
    pub(crate) image_space_modifiers: SharedCatalog<u32, ImageSpaceModifier>,
    pub(crate) sounds: SharedCatalog<u32, SoundRecord>,
    pub(crate) sound_references: SharedCatalog<u32, SoundReferenceRecord>,
    pub(crate) acoustic_spaces: SharedCatalog<u32, AcousticSpaceRecord>,
    pub(crate) music: SharedCatalog<u32, MusicRecord>,
    // M9 wave 1 (#308): game settings and actor-value metadata feed the
    // content-set-wide GMST catalog; last-loader-wins by FormID like the
    // other shared catalogs.
    pub(crate) gmsts: SharedCatalog<u32, GmstRecord>,
    pub(crate) actor_values: SharedCatalog<u32, AvifRecord>,
    // M9 wave 2 (#312): perks feed the content-set-wide perk catalog.
    pub(crate) perks: SharedCatalog<u32, PerkRecord>,
    // M9 wave 3 (#316): ingestibles and base effects feed the
    // content-set-wide effect catalog.
    pub(crate) alchs: SharedCatalog<u32, AlchRecord>,
    pub(crate) mgefs: SharedCatalog<u32, MgefRecord>,
    pub(crate) lighting_templates: SharedCatalog<u32, LightingTemplateRecord>,
    pub(crate) climates: SharedCatalog<u32, ClimateRecord>,
    pub(crate) weathers: SharedCatalog<u32, WeatherRecord>,
    pub(crate) landscape_textures: SharedCatalog<u32, LandscapeTextureRecord>,
    pub(crate) texture_sets: SharedCatalog<u32, TextureSetRecord>,
    pub(crate) worldspaces: SharedCatalog<u32, WorldspaceRecord>,
    pub(crate) land: Option<LandRecord>,
    pub(crate) road_count: usize,
    pub(crate) references: Vec<ReferenceRecord>,
    pub(crate) navmeshes: Vec<NavMeshRecord>,
    /// Content-set-wide `NAVI` singleton (issue #111, M4 wave 2):
    /// last-loader-wins like `WRLD`/`IMGS`, independent of which cell is
    /// selected.
    pub(crate) navigation: Option<NaviRecord>,
    pub(crate) cell: Option<CellInfo>,
    pub(crate) cell_metadata: Option<CellMetadata>,
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct ParsedContentSet {
    state: ParsedState,
}

impl ParsedContentSet {
    pub(crate) fn cells(&self) -> impl Iterator<Item = (&u32, &CellInfo)> {
        self.state.cells.iter()
    }

    pub(crate) fn cell_winning_plugin(&self, form_id: u32) -> Option<&str> {
        self.state
            .cell_winning_plugins
            .get(&form_id)
            .map(String::as_str)
    }

    pub(crate) fn cell_provenance(&self, form_id: u32) -> &[String] {
        self.state
            .cell_provenance
            .get(&form_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub(crate) fn worldspaces(&self) -> impl Iterator<Item = (&u32, &WorldspaceRecord)> {
        self.state.worldspaces.iter()
    }

    pub(crate) fn weathers(&self) -> impl Iterator<Item = (&u32, &WeatherRecord)> {
        self.state.weathers.iter()
    }

    pub(crate) fn land_for_cell(&self, cell_form_id: u32) -> Option<&LandRecord> {
        self.state.lands.get(&cell_form_id).map(|(_, land)| land)
    }

    pub(crate) fn road_count_for_cell(&self, cell_form_id: u32) -> usize {
        self.state
            .road_counts
            .get(&cell_form_id)
            .copied()
            .unwrap_or(0)
    }

    pub(crate) fn base_model(&self, base_form_id: u32) -> Option<String> {
        self.state
            .bases
            .get(&base_form_id)
            .and_then(|base| base.model.clone())
    }

    pub(crate) fn references_by_cell(&self) -> HashMap<u32, Vec<&ReferenceRecord>> {
        let mut grouped = HashMap::<u32, Vec<&ReferenceRecord>>::new();
        for reference in self.state.references.values() {
            grouped
                .entry(reference.parent_cell_form_id)
                .or_default()
                .push(reference);
        }
        for references in grouped.values_mut() {
            references.sort_by_key(|reference| reference.form_id);
        }
        grouped
    }

    pub(crate) fn navmesh_counts_by_cell(&self) -> HashMap<u32, usize> {
        let mut counts = HashMap::<u32, usize>::new();
        for (cell_form_id, _) in self.state.navmeshes.values() {
            *counts.entry(*cell_form_id).or_default() += 1;
        }
        counts
    }

    /// Content-set-wide door connectivity graph (issue #45, F45.3):
    /// generalises the per-cell `XTEL` resolution in `select` below to walk
    /// every reference with a door+teleport in the whole loaded content set,
    /// not just the cell being prepared. Returns the resolved directed edges
    /// (sorted by `(source_cell_form_id, door_reference_form_id)` for
    /// deterministic output) plus a count of teleports whose destination
    /// reference could not be found -- unresolved, not fatal.
    pub(crate) fn door_edges(&self) -> (Vec<DoorEdgeRecord>, u32) {
        let mut edges = Vec::new();
        let mut unresolved = 0_u32;
        for reference in self.state.references.values() {
            let Some(door) = reference.door.as_ref() else {
                continue;
            };
            let Some(teleport) = door.teleport.as_ref() else {
                continue;
            };
            match resolve_teleport_destination(teleport, &self.state.references) {
                Some(destination) => edges.push(DoorEdgeRecord {
                    source_cell_form_id: reference.parent_cell_form_id,
                    door_reference_form_id: reference.form_id,
                    destination_cell_form_id: destination.cell_form_id,
                    destination_door_reference_form_id: destination.door_reference_form_id,
                    position: destination.position,
                    rotation: destination.rotation,
                }),
                None => unresolved += 1,
            }
        }
        edges.sort_by_key(|edge| (edge.source_cell_form_id, edge.door_reference_form_id));
        (edges, unresolved)
    }

    #[cfg(test)]
    pub(crate) fn select(self, selector: &CellSelector) -> Result<ParsedPlugin> {
        self.select_from(selector)
    }

    fn select_from(&self, selector: &CellSelector) -> Result<ParsedPlugin> {
        let state = &self.state;
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
                    [] => {
                        bail!("GECK EditorID '{editor_id}' was not found in the loaded content set")
                    }
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

        let all_references = &state.references;
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
            door.destination = resolve_teleport_destination(teleport, all_references);
        }
        for reference in &mut references {
            match resolve_initially_enabled(reference.form_id, all_references) {
                Ok(enabled) => reference.initially_enabled = enabled,
                Err(error) => {
                    reference.initially_enabled = true;
                    reference.ignored_subrecords.push(format!("XESP:{error}"));
                }
            }
            if reference.enable_parent.is_some() {
                match resolve_enable_root(reference.form_id, all_references) {
                    Ok(root) => reference.enable_root_form_id = Some(root),
                    Err(error) => {
                        reference
                            .ignored_subrecords
                            .push(format!("XESP-root:{error}"));
                    }
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
                } else if let Some(error) = signature.strip_prefix("XESP-root:") {
                    *ignored
                        .entry(("XESP root resolution".into(), error.into()))
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
        let mut navmeshes = state
            .navmeshes
            .values()
            .filter(|(cell, _)| *cell == target_cell)
            .map(|(_, navmesh)| navmesh.clone())
            .collect::<Vec<_>>();
        navmeshes.sort_by_key(|navmesh| navmesh.form_id);

        let mut diagnostics = ignored
            .into_iter()
            .map(|((record, subrecord), count)| {
                if record == "XESP resolution" {
                    format!(
                        "{subrecord} while resolving {count} target-cell placement(s); defaulted visible"
                    )
                } else if record == "XESP root resolution" {
                    format!(
                        "{subrecord} while resolving {count} enable-parent chain root(s); mutability left Unknown"
                    )
                } else {
                    format!(
                        "ignored unsupported {record}.{subrecord} subrecord while importing {count} target-cell record(s)"
                    )
                }
            })
            .collect::<Vec<_>>();
        diagnostics.extend(state.recipe_diagnostics.iter().cloned());
        diagnostics.extend(state.actor_support_diagnostics.iter().cloned());
        for idle in state.idles.values() {
            for message in &idle.diagnostics {
                diagnostics.push(format!("IDLE {:08x}: {message}", idle.form_id));
            }
        }
        diagnostics.extend(state.navigation_diagnostics.iter().cloned());
        for navmesh in &navmeshes {
            for message in &navmesh.diagnostics {
                diagnostics.push(format!("NAVM {:08x}: {message}", navmesh.form_id));
            }
        }
        diagnostics.sort();

        Ok(ParsedPlugin {
            bases: state.bases.clone(),
            recipes: state.recipes.clone(),
            races: state.races.clone(),
            classes: state.classes.clone(),
            factions: state.factions.clone(),
            packages: state.packages.clone(),
            idles: state.idles.clone(),
            image_spaces: state.image_spaces.clone(),
            image_space_modifiers: state.image_space_modifiers.clone(),
            sounds: state.sounds.clone(),
            sound_references: state.sound_references.clone(),
            acoustic_spaces: state.acoustic_spaces.clone(),
            music: state.music.clone(),
            gmsts: state.gmsts.clone(),
            actor_values: state.actor_values.clone(),
            perks: state.perks.clone(),
            alchs: state.alchs.clone(),
            mgefs: state.mgefs.clone(),
            lighting_templates: state.lighting_templates.clone(),
            climates: state.climates.clone(),
            weathers: state.weathers.clone(),
            landscape_textures: state.landscape_textures.clone(),
            texture_sets: state.texture_sets.clone(),
            worldspaces: state.worldspaces.clone(),
            land: state.lands.get(&target_cell).map(|(_, land)| land.clone()),
            road_count: state.road_counts.get(&target_cell).copied().unwrap_or(0),
            references,
            navmeshes,
            navigation: state.navigation.clone(),
            cell: state.cells.get(&target_cell).cloned(),
            cell_metadata: state.cell_metadata.get(&target_cell).cloned(),
            diagnostics,
        })
    }

    /// Selects a cell from a content set that is shared by a batch session.
    ///
    /// Global parsed catalogs are Arc-backed and cloned by pointer. Only the
    /// selected cell's references, navigation, land, and diagnostics are
    /// projected into worker-owned values.
    pub(crate) fn select_shared(&self, selector: &CellSelector) -> Result<ParsedPlugin> {
        self.select_from(selector)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PluginSource<'a> {
    pub(crate) name: &'a str,
    pub(crate) bytes: &'a [u8],
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ParsedState {
    bases: SharedCatalog<u32, BaseRecord>,
    recipes: SharedCatalog<u32, RecipeRecord>,
    races: SharedCatalog<u32, RaceRecord>,
    classes: SharedCatalog<u32, ClassRecord>,
    factions: SharedCatalog<u32, FactionRecord>,
    packages: SharedCatalog<u32, PackageRecord>,
    idles: SharedCatalog<u32, IdleRecord>,
    image_spaces: SharedCatalog<u32, ImageSpaceInfo>,
    image_space_modifiers: SharedCatalog<u32, ImageSpaceModifier>,
    sounds: SharedCatalog<u32, SoundRecord>,
    sound_references: SharedCatalog<u32, SoundReferenceRecord>,
    acoustic_spaces: SharedCatalog<u32, AcousticSpaceRecord>,
    music: SharedCatalog<u32, MusicRecord>,
    gmsts: SharedCatalog<u32, GmstRecord>,
    actor_values: SharedCatalog<u32, AvifRecord>,
    perks: SharedCatalog<u32, PerkRecord>,
    alchs: SharedCatalog<u32, AlchRecord>,
    mgefs: SharedCatalog<u32, MgefRecord>,
    lighting_templates: SharedCatalog<u32, LightingTemplateRecord>,
    climates: SharedCatalog<u32, ClimateRecord>,
    weathers: SharedCatalog<u32, WeatherRecord>,
    landscape_textures: SharedCatalog<u32, LandscapeTextureRecord>,
    texture_sets: SharedCatalog<u32, TextureSetRecord>,
    references: HashMap<u32, ReferenceRecord>,
    navmeshes: HashMap<u32, (u32, NavMeshRecord)>,
    navigation: Option<NaviRecord>,
    cells: HashMap<u32, CellInfo>,
    cell_metadata: HashMap<u32, CellMetadata>,
    cell_winning_plugins: HashMap<u32, String>,
    cell_provenance: HashMap<u32, Vec<String>>,
    worldspaces: SharedCatalog<u32, WorldspaceRecord>,
    lands: HashMap<u32, (u32, LandRecord)>,
    road_counts: HashMap<u32, usize>,
    recipe_diagnostics: Vec<String>,
    actor_support_diagnostics: Vec<String>,
    navigation_diagnostics: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct Subrecord {
    signature: String,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct LocatedSubrecord {
    pub(crate) signature: String,
    pub(crate) data: Vec<u8>,
    pub(crate) offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubrecordParseError {
    pub(crate) offset: usize,
    pub(crate) signature: Option<String>,
    pub(crate) message: String,
}

impl fmt::Display for SubrecordParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at byte offset {}", self.message, self.offset)
    }
}

impl std::error::Error for SubrecordParseError {}

#[cfg(test)]
pub(crate) fn parse_content_set(
    sources: &[PluginSource<'_>],
    selector: &CellSelector,
) -> Result<ParsedPlugin> {
    parse_content_set_all(sources)?.select(selector)
}

pub(crate) fn parse_content_set_all(sources: &[PluginSource<'_>]) -> Result<ParsedContentSet> {
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
        let resolver = FormIdResolver::new(index as u8, master_indices);
        walk_container(
            source.bytes,
            0,
            source.bytes.len(),
            GroupContext::default(),
            &resolver,
            &mut state,
            source.name,
        )
        .with_context(|| format!("parsing {}", source.name))?;
    }

    Ok(ParsedContentSet { state })
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
