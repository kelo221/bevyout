//! Fallout 3/New Vegas `NAVM`/`NAVI` navigation-mesh subrecord decoding
//! (issue #111, M4 wave 2; `NVMI`'s trailing island block decoded in issue
//! #113, M4 wave 4 -- see `decode_navi_tail`'s doc comment).
//!
//! Field layouts are taken from the fopdoc FalloutNV pages
//! (`Records/NAVM.html`, `Records/NAVI.html`) rather than ported from
//! OpenMW: the supplied `openmw-master` snapshot's
//! `components/esm4/loadnavm.{hpp,cpp}` and `loadnavi.{hpp,cpp}` target the
//! newer (Skyrim+) combined-chunk navmesh format (`NVNM`) and explicitly
//! *skip* the older per-field `NVER`/`DATA`/`NVVX`/`NVTR`/`NVCA`/`NVDP`/
//! `NVGD`/`NVEX` subrecords FO3/FNV actually use for `NAVM`, and
//! unconditionally skip `NVMI`/`NVSI`/`NVCI` decode for `NAVI` on Fallout:
//! New Vegas. See `NOTICE.md` for the confirmed divergence.
//!
//! `NAVM.NVGD`'s trailing per-cell triangle-index grid (after the 36-byte
//! divisor/bounds header) has no documented element count and is not
//! required by the runtime graph (`prepare::nav_graph`), so only that fixed
//! header is decoded; the remainder stays inside the record's retained raw
//! `payload`. `NAVI.NVCI`'s fields are labelled "Unknown" even in fopdoc
//! itself (every field name is literally `Unknown`) with no confirmed
//! element layout, so it -- and any other subrecord besides `NVER`/`NVMI` --
//! is retained as an opaque diagnosed subrecord rather than guessed at.

use super::*;

// ---------------------------------------------------------------------
// NAVM
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct NavMeshTriangle {
    /// `NVTR` "Vertex 1/2/3": indices into the owning `NavMeshRecord`'s
    /// `vertices`. fopdoc types these `int16`; negative/out-of-range values
    /// are retained verbatim here and only diagnosed by `prepare::nav_graph`.
    pub(crate) vertex_indices: [i16; 3],
    /// `NVTR` "Edge - Vertices 1,2/2,3/3,1": intra-mesh neighbour triangle
    /// index for that edge, or a negative sentinel when the edge has no
    /// same-mesh neighbour (mesh boundary, or externally linked -- see
    /// `flags`).
    pub(crate) edge_neighbors: [i16; 3],
    pub(crate) flags: u32,
}

/// Documented `NVTR` flag bits (fopdoc FalloutNV `Records/NAVM.html`,
/// "Flag Values"). `prepare::nav_graph` re-states the walkability bits it
/// consumes locally (it must stay free of `openmw_esm4` imports); these
/// constants are the format-side source of truth for those values.
#[allow(dead_code)]
impl NavMeshTriangle {
    pub(crate) const EDGE0_EXTERNAL: u32 = 0x0000_0001;
    pub(crate) const EDGE1_EXTERNAL: u32 = 0x0000_0002;
    pub(crate) const EDGE2_EXTERNAL: u32 = 0x0000_0004;
    pub(crate) const PREFERRED_PATHING: u32 = 0x0000_0040;
    pub(crate) const WATER: u32 = 0x0000_0200;
    pub(crate) const CONTAINS_DOOR: u32 = 0x0000_0400;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct NavMeshDoor {
    /// `NVDP` "Reference": `None` when the resolved FormID is null.
    pub(crate) door_reference_form_id: Option<u32>,
    pub(crate) triangle: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct NavMeshExternalConnection {
    /// `NVEX` "Navigation Mesh": `None` when null, per fopdoc ("...or
    /// null").
    pub(crate) target_navmesh_form_id: Option<u32>,
    pub(crate) triangle: u16,
}

/// `NVGD`'s fixed 36-byte header only (divisor + 8 bounding floats); the
/// trailing per-cell triangle-index grid has no documented element count and
/// is not decoded (see module doc comment).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct NavMeshGrid {
    pub(crate) divisor: u32,
    pub(crate) max_x_distance: f32,
    pub(crate) max_y_distance: f32,
    pub(crate) min: [f32; 3],
    pub(crate) max: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NavMeshChunk {
    pub(crate) signature: String,
    pub(crate) byte_len: u32,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct NavMeshRecord {
    pub(crate) form_id: u32,
    pub(crate) flags: u32,
    pub(crate) version: Option<u32>,
    /// `DATA` "Cell": FormID of the owning `CELL` record.
    pub(crate) cell_form_id: Option<u32>,
    /// `NVVX`: one `[f32; 3]` per decoded vertex. Decodes independently of
    /// `DATA` -- an absent/malformed `DATA` only skips the declared-count
    /// cross-check (`check_count`), not decoding itself; empty when the
    /// record has no `NVVX` subrecord at all.
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) triangles: Vec<NavMeshTriangle>,
    /// `NVCA` "Unknown, may be triangle IDs" -- retained verbatim, widened
    /// from `int16`; interpreted (and range-checked) only by
    /// `prepare::nav_graph`.
    pub(crate) cover_triangle_ids: Vec<i16>,
    pub(crate) doors: Vec<NavMeshDoor>,
    /// `NVGD` fixed header, retained as validated metadata only; the
    /// runtime graph derives its own bounds from the converted vertices.
    #[allow(dead_code)]
    pub(crate) grid: Option<NavMeshGrid>,
    pub(crate) external_connections: Vec<NavMeshExternalConnection>,
    /// Raw record payload, retained for the on-disk `*.navm.bin` source
    /// artifact (`prepare::navmesh::stage_navmeshes`).
    pub(crate) payload: Vec<u8>,
    pub(crate) chunks: Vec<NavMeshChunk>,
    /// Stable, unprefixed diagnostic messages (malformed/truncated/
    /// count-mismatched subrecords). `reader.rs`/`records.rs::select` add
    /// the `NAVM <form_id>:` prefix, matching the `RCPE`/`RACE` convention.
    pub(crate) diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
struct NavMeshCounts {
    vertices: u32,
    triangles: u32,
    external: u32,
    cover: u32,
    doors: u32,
}

pub(crate) fn parse_navmesh(
    subs: &[Subrecord],
    form_id: u32,
    flags: u32,
    resolver: &FormIdResolver,
    payload: Vec<u8>,
) -> NavMeshRecord {
    let mut diagnostics = Vec::new();
    let version = sub(subs, "NVER").and_then(|data| u32_at(data, 0));

    let mut cell_form_id = None;
    let mut counts = None;
    if let Some(data) = sub(subs, "DATA") {
        if data.len() == 24 {
            cell_form_id = u32_at(data, 0)
                .map(|raw| resolver.adjust(raw))
                .filter(|id| *id != 0);
            counts = Some(NavMeshCounts {
                vertices: u32_at(data, 4).unwrap_or_default(),
                triangles: u32_at(data, 8).unwrap_or_default(),
                external: u32_at(data, 12).unwrap_or_default(),
                cover: u32_at(data, 16).unwrap_or_default(),
                doors: u32_at(data, 20).unwrap_or_default(),
            });
        } else {
            diagnostics.push(format!(
                "DATA malformed: expected 24 bytes, got {}",
                data.len()
            ));
        }
    }

    let vertices = sub(subs, "NVVX")
        .map(|data| decode_vertices(data, counts.map(|c| c.vertices), &mut diagnostics))
        .unwrap_or_default();
    let triangles = sub(subs, "NVTR")
        .map(|data| decode_triangles(data, counts.map(|c| c.triangles), &mut diagnostics))
        .unwrap_or_default();
    let cover_triangle_ids = sub(subs, "NVCA")
        .map(|data| decode_cover(data, counts.map(|c| c.cover), &mut diagnostics))
        .unwrap_or_default();
    let doors = sub(subs, "NVDP")
        .map(|data| decode_doors(data, resolver, counts.map(|c| c.doors), &mut diagnostics))
        .unwrap_or_default();
    let external_connections = sub(subs, "NVEX")
        .map(|data| decode_external(data, resolver, counts.map(|c| c.external), &mut diagnostics))
        .unwrap_or_default();
    let grid = sub(subs, "NVGD").and_then(|data| decode_grid(data, &mut diagnostics));

    let chunks = subs
        .iter()
        .map(|sub| NavMeshChunk {
            signature: sub.signature.clone(),
            byte_len: sub.data.len() as u32,
        })
        .collect();

    NavMeshRecord {
        form_id,
        flags,
        version,
        cell_form_id,
        vertices,
        triangles,
        cover_triangle_ids,
        doors,
        grid,
        external_connections,
        payload,
        chunks,
        diagnostics,
    }
}

fn decode_vertices(
    data: &[u8],
    declared: Option<u32>,
    diagnostics: &mut Vec<String>,
) -> Vec<[f32; 3]> {
    const STRIDE: usize = 12;
    let mut vertices = Vec::with_capacity(data.len() / STRIDE);
    let mut offset = 0;
    while offset + STRIDE <= data.len() {
        vertices.push([
            f32_at(data, offset).unwrap_or_default(),
            f32_at(data, offset + 4).unwrap_or_default(),
            f32_at(data, offset + 8).unwrap_or_default(),
        ]);
        offset += STRIDE;
    }
    if offset != data.len() {
        diagnostics.push(format!(
            "NVVX truncated: {} trailing byte(s) do not form a full {STRIDE}-byte vertex",
            data.len() - offset
        ));
    }
    check_count("NVVX", declared, vertices.len(), diagnostics);
    vertices
}

fn decode_triangles(
    data: &[u8],
    declared: Option<u32>,
    diagnostics: &mut Vec<String>,
) -> Vec<NavMeshTriangle> {
    const STRIDE: usize = 16;
    let mut triangles = Vec::with_capacity(data.len() / STRIDE);
    let mut offset = 0;
    while offset + STRIDE <= data.len() {
        triangles.push(NavMeshTriangle {
            vertex_indices: [
                i16_at(data, offset).unwrap_or_default(),
                i16_at(data, offset + 2).unwrap_or_default(),
                i16_at(data, offset + 4).unwrap_or_default(),
            ],
            edge_neighbors: [
                i16_at(data, offset + 6).unwrap_or_default(),
                i16_at(data, offset + 8).unwrap_or_default(),
                i16_at(data, offset + 10).unwrap_or_default(),
            ],
            flags: u32_at(data, offset + 12).unwrap_or_default(),
        });
        offset += STRIDE;
    }
    if offset != data.len() {
        diagnostics.push(format!(
            "NVTR truncated: {} trailing byte(s) do not form a full {STRIDE}-byte triangle",
            data.len() - offset
        ));
    }
    check_count("NVTR", declared, triangles.len(), diagnostics);
    triangles
}

fn decode_cover(data: &[u8], declared: Option<u32>, diagnostics: &mut Vec<String>) -> Vec<i16> {
    const STRIDE: usize = 2;
    let mut values = Vec::with_capacity(data.len() / STRIDE);
    let mut offset = 0;
    while offset + STRIDE <= data.len() {
        values.push(i16_at(data, offset).unwrap_or_default());
        offset += STRIDE;
    }
    if offset != data.len() {
        diagnostics.push(format!(
            "NVCA truncated: {} trailing byte(s) do not form a full {STRIDE}-byte entry",
            data.len() - offset
        ));
    }
    check_count("NVCA", declared, values.len(), diagnostics);
    values
}

fn decode_doors(
    data: &[u8],
    resolver: &FormIdResolver,
    declared: Option<u32>,
    diagnostics: &mut Vec<String>,
) -> Vec<NavMeshDoor> {
    const STRIDE: usize = 8;
    let mut doors = Vec::with_capacity(data.len() / STRIDE);
    let mut offset = 0;
    while offset + STRIDE <= data.len() {
        let door_reference_form_id = u32_at(data, offset)
            .map(|raw| resolver.adjust(raw))
            .filter(|id| *id != 0);
        let triangle = u16::from_le_bytes(data[offset + 4..offset + 6].try_into().unwrap());
        doors.push(NavMeshDoor {
            door_reference_form_id,
            triangle,
        });
        offset += STRIDE;
    }
    if offset != data.len() {
        diagnostics.push(format!(
            "NVDP truncated: {} trailing byte(s) do not form a full {STRIDE}-byte door",
            data.len() - offset
        ));
    }
    check_count("NVDP", declared, doors.len(), diagnostics);
    doors
}

fn decode_external(
    data: &[u8],
    resolver: &FormIdResolver,
    declared: Option<u32>,
    diagnostics: &mut Vec<String>,
) -> Vec<NavMeshExternalConnection> {
    const STRIDE: usize = 10;
    let mut connections = Vec::with_capacity(data.len() / STRIDE);
    let mut offset = 0;
    while offset + STRIDE <= data.len() {
        // Bytes [offset..offset + 4) are NVEX's undocumented leading
        // "Unknown" field; not retained (no confirmed meaning to preserve
        // beyond the raw record payload, which already keeps every byte).
        let target_navmesh_form_id = u32_at(data, offset + 4)
            .map(|raw| resolver.adjust(raw))
            .filter(|id| *id != 0);
        let triangle = u16::from_le_bytes(data[offset + 8..offset + 10].try_into().unwrap());
        connections.push(NavMeshExternalConnection {
            target_navmesh_form_id,
            triangle,
        });
        offset += STRIDE;
    }
    if offset != data.len() {
        diagnostics.push(format!(
            "NVEX truncated: {} trailing byte(s) do not form a full {STRIDE}-byte connection",
            data.len() - offset
        ));
    }
    check_count("NVEX", declared, connections.len(), diagnostics);
    connections
}

fn decode_grid(data: &[u8], diagnostics: &mut Vec<String>) -> Option<NavMeshGrid> {
    const HEADER_LEN: usize = 36;
    if data.len() < HEADER_LEN {
        diagnostics.push(format!(
            "NVGD malformed: expected at least {HEADER_LEN} bytes, got {}",
            data.len()
        ));
        return None;
    }
    Some(NavMeshGrid {
        divisor: u32_at(data, 0).unwrap_or_default(),
        max_x_distance: f32_at(data, 4).unwrap_or_default(),
        max_y_distance: f32_at(data, 8).unwrap_or_default(),
        min: [
            f32_at(data, 12).unwrap_or_default(),
            f32_at(data, 16).unwrap_or_default(),
            f32_at(data, 20).unwrap_or_default(),
        ],
        max: [
            f32_at(data, 24).unwrap_or_default(),
            f32_at(data, 28).unwrap_or_default(),
            f32_at(data, 32).unwrap_or_default(),
        ],
    })
}

fn check_count(
    signature: &str,
    declared: Option<u32>,
    decoded: usize,
    diagnostics: &mut Vec<String>,
) {
    if let Some(declared) = declared
        && declared as usize != decoded
    {
        diagnostics.push(format!(
            "{signature} count mismatch: DATA declares {declared}, decoded {decoded}"
        ));
    }
}

// ---------------------------------------------------------------------
// NAVI
// ---------------------------------------------------------------------

/// `NVMI`'s optional trailing island block (issue #113, M4 wave 4): an
/// axis-aligned bounding box plus a small local vertex/triangle sub-mesh.
/// Layout verified against real `Fallout3.esm` `NVMI` bytes (see
/// `decode_navi_tail`'s doc comment for the byte-level evidence) -- fopdoc
/// documents none of this (it labels the whole trailer "Unknown uint8[]")
/// and OpenMW's `loadnavi.cpp` unconditionally skips `NVMI` decode for
/// FO3/FNV, so there is no authoritative reference layout; this is a
/// from-scratch, real-data-derived extension like the rest of this module.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct NaviBounds {
    pub(crate) min: [f32; 3],
    pub(crate) max: [f32; 3],
}

/// The island's own small local-space vertex/triangle sub-mesh, `u16`-indexed
/// into `vertices` (verified: every real sample's triangle indices stay
/// within `0..vertices.len()`).
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NaviIsland {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) triangles: Vec<[u16; 3]>,
}

/// One `NVMI` entry: a fixed 16-byte header (`unknown`/`navmesh_form_id`/
/// `location_form_id`/`grid_x`/`grid_y`, decoded in `decode_navi_info`)
/// followed by a variable-length tail (decoded in `decode_navi_tail`) --
/// `center` (always present when the tail is well-formed), then an optional
/// `bounds`+`island` block, then a final 4-byte `trailing` field of
/// unconfirmed meaning (observed zero on every real sample gathered).
/// `tail` retains only bytes `decode_navi_tail` could not place -- empty on
/// every real sample gathered during verification; a truncated/malformed
/// entry keeps its unparsed remainder here rather than panicking or
/// discarding it.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct NaviInfoEntry {
    pub(crate) unknown: [u8; 4],
    pub(crate) navmesh_form_id: Option<u32>,
    /// `NVMI` "Location": FormID of the owning `CELL` or `WRLD` record.
    pub(crate) location_form_id: Option<u32>,
    pub(crate) grid_x: i16,
    pub(crate) grid_y: i16,
    pub(crate) center: Option<[f32; 3]>,
    pub(crate) bounds: Option<NaviBounds>,
    pub(crate) island: Option<NaviIsland>,
    pub(crate) trailing: [u8; 4],
    pub(crate) tail: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct NaviRecord {
    /// Record identity/version metadata, retained for diagnostics and
    /// forward use; only `entries` feeds the prepared nav graph today.
    #[allow(dead_code)]
    pub(crate) form_id: u32,
    #[allow(dead_code)]
    pub(crate) flags: u32,
    #[allow(dead_code)]
    pub(crate) version: Option<u32>,
    pub(crate) entries: Vec<NaviInfoEntry>,
}

/// Decodes one `NAVI` record: `NVER` and every `NVMI` entry. Every other
/// subrecord (including `NVCI`, whose fields fopdoc itself only labels
/// "Unknown") is retained as an opaque, diagnosed subrecord -- see the
/// module doc comment. Returns pre-formatted (but unprefixed) diagnostic
/// messages; the caller (`reader.rs`) adds the `NAVI <form_id>:` prefix.
pub(crate) fn parse_navi(
    subs: &[Subrecord],
    form_id: u32,
    flags: u32,
    resolver: &FormIdResolver,
) -> (NaviRecord, Vec<String>) {
    let mut diagnostics = Vec::new();
    let version = sub(subs, "NVER").and_then(|data| u32_at(data, 0));
    let mut entries = Vec::new();
    for (index, subrecord) in subs.iter().filter(|s| s.signature == "NVMI").enumerate() {
        let mut tail_diagnostics = Vec::new();
        match decode_navi_info(&subrecord.data, resolver, &mut tail_diagnostics) {
            Ok(entry) => entries.push(entry),
            Err(error) => diagnostics.push(format!("NVMI malformed: {error}")),
        }
        for message in tail_diagnostics {
            diagnostics.push(format!("NVMI {index}: {message}"));
        }
    }
    for signature in ignored_signatures(subs, &["EDID", "NVER", "NVMI"]) {
        diagnostics.push(format!(
            "ignored unsupported NAVI.{signature} subrecord; layout not documented for Fallout 3/New Vegas"
        ));
    }
    (
        NaviRecord {
            form_id,
            flags,
            version,
            entries,
        },
        diagnostics,
    )
}

fn decode_navi_info(
    data: &[u8],
    resolver: &FormIdResolver,
    diagnostics: &mut Vec<String>,
) -> Result<NaviInfoEntry, String> {
    const HEADER_LEN: usize = 16;
    if data.len() < HEADER_LEN {
        return Err(format!(
            "expected at least {HEADER_LEN} bytes, got {}",
            data.len()
        ));
    }
    let mut unknown = [0_u8; 4];
    unknown.copy_from_slice(&data[0..4]);
    let navmesh_form_id = u32_at(data, 4)
        .map(|raw| resolver.adjust(raw))
        .filter(|id| *id != 0);
    let location_form_id = u32_at(data, 8)
        .map(|raw| resolver.adjust(raw))
        .filter(|id| *id != 0);
    let grid_x = i16_at(data, 12).unwrap_or_default();
    let grid_y = i16_at(data, 14).unwrap_or_default();
    let decoded = decode_navi_tail(&data[HEADER_LEN..], diagnostics);
    Ok(NaviInfoEntry {
        unknown,
        navmesh_form_id,
        location_form_id,
        grid_x,
        grid_y,
        center: decoded.center,
        bounds: decoded.bounds,
        island: decoded.island,
        trailing: decoded.trailing,
        tail: decoded.tail,
    })
}

/// `decode_navi_tail`'s decoded pieces (see its doc comment for the byte
/// layout each field comes from).
#[derive(Debug, Clone, Default, PartialEq)]
struct DecodedNaviTail {
    center: Option<[f32; 3]>,
    bounds: Option<NaviBounds>,
    island: Option<NaviIsland>,
    trailing: [u8; 4],
    tail: Vec<u8>,
}

/// Decodes `NVMI`'s trailing "Unknown uint8[]" field (fopdoc's name for it;
/// see `NaviInfoEntry`'s doc comment for why there is no authoritative
/// reference layout). This layout was reverse-engineered and cross-checked
/// against real `Fallout3.esm` `NVMI` bytes across multiple entries/cells
/// (interior single-mesh cells, FranklinMetro02's two-NAVM cell, and
/// Capital-Wasteland exterior grid entries) before being trusted here, per
/// this module's verification bar:
///
/// - `center: [f32; 3]` (12 bytes) -- always present when the tail is
///   well-formed. Every sample's value falls inside (or very near) that
///   entry's own navmesh bounds once converted through `FO3_SCALE`, so this
///   is a mesh-space center point, not a world/cell-grid coordinate.
/// - An entry with *exactly* 4 bytes left after `center` has no island block
///   at all: those 4 bytes are `trailing` and decoding stops (this is the
///   common case for a mesh with no recorded island, e.g. `0005429f` in
///   FranklinMetro02, real tail length 16).
/// - Otherwise, an island block follows: `bounds` (two `[f32; 3]` corners,
///   24 bytes -- confirmed as a real min/max pair: every axis of `min` is
///   `<=` the matching axis of `max` on every sample gathered), then two
///   `u16` counts packed into one 4-byte word (`vertex_count`,
///   `triangle_count`, in that order -- confirmed because the byte count of
///   the array data that follows always matches `vertex_count * 12 +
///   triangle_count * 6` exactly, and every triangle's vertex indices stay
///   inside `0..vertex_count` on every sample), then `vertex_count` packed
///   `[f32; 3]` vertices, then `triangle_count` packed `[u16; 3]` triangles
///   (local indices into the vertices just read), then a final 4-byte
///   `trailing` field (observed zero on every real sample gathered; kept
///   raw, not claimed as a flag or count).
///
/// Real examples that pinned this layout exactly (byte-for-byte, with zero
/// leftover): a FranklinMetro02 (`0001a273`) entry with tail length 284
/// (`vertex_count = 14`, `triangle_count = 12`) and a Capital Wasteland
/// exterior grid entry with tail length 194 (`vertex_count = 9`,
/// `triangle_count = 7`); dozens of exterior grid entries with tail length
/// 44 (`vertex_count = 0`, `triangle_count = 0`, i.e. `bounds` present with
/// an empty island).
///
/// Never panics: every multi-byte read is bounds-checked first, and any
/// length that does not cleanly fit the pattern above (truncated header,
/// declared counts needing more bytes than remain, unexpected leftover
/// bytes after a well-formed island) is diagnosed into `diagnostics` and the
/// unparsed remainder is returned as `tail` rather than guessed at or
/// dropped.
fn decode_navi_tail(data: &[u8], diagnostics: &mut Vec<String>) -> DecodedNaviTail {
    const CENTER_LEN: usize = 12;
    const TRAILING_LEN: usize = 4;
    const BOUNDS_LEN: usize = 24;
    const COUNTS_LEN: usize = 4;

    if data.is_empty() {
        return DecodedNaviTail::default();
    }
    if data.len() < CENTER_LEN {
        diagnostics.push(format!(
            "tail truncated: {} byte(s), too short for the {CENTER_LEN}-byte center point",
            data.len()
        ));
        return DecodedNaviTail {
            tail: data.to_vec(),
            ..DecodedNaviTail::default()
        };
    }
    let center = [
        f32_at(data, 0).unwrap_or_default(),
        f32_at(data, 4).unwrap_or_default(),
        f32_at(data, 8).unwrap_or_default(),
    ];
    let offset = CENTER_LEN;
    let remaining = data.len() - offset;

    if remaining == TRAILING_LEN {
        let mut trailing = [0_u8; 4];
        trailing.copy_from_slice(&data[offset..offset + TRAILING_LEN]);
        return DecodedNaviTail {
            center: Some(center),
            trailing,
            ..DecodedNaviTail::default()
        };
    }
    if remaining == 0 {
        return DecodedNaviTail {
            center: Some(center),
            ..DecodedNaviTail::default()
        };
    }
    if remaining < BOUNDS_LEN + COUNTS_LEN + TRAILING_LEN {
        diagnostics.push(format!(
            "tail truncated: {remaining} byte(s) after the center point do not form a complete island block or a bare trailing field"
        ));
        return DecodedNaviTail {
            center: Some(center),
            tail: data[offset..].to_vec(),
            ..DecodedNaviTail::default()
        };
    }

    let bounds = NaviBounds {
        min: [
            f32_at(data, offset).unwrap_or_default(),
            f32_at(data, offset + 4).unwrap_or_default(),
            f32_at(data, offset + 8).unwrap_or_default(),
        ],
        max: [
            f32_at(data, offset + 12).unwrap_or_default(),
            f32_at(data, offset + 16).unwrap_or_default(),
            f32_at(data, offset + 20).unwrap_or_default(),
        ],
    };
    let offset = offset + BOUNDS_LEN;

    let vertex_count = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap()) as usize;
    let triangle_count =
        u16::from_le_bytes(data[offset + 2..offset + 4].try_into().unwrap()) as usize;
    let offset = offset + COUNTS_LEN;

    let island_bytes = vertex_count * 12 + triangle_count * 6;
    if data.len() < offset + island_bytes + TRAILING_LEN {
        diagnostics.push(format!(
            "island truncated: declares {vertex_count} vertex(es)/{triangle_count} triangle(s) needing {island_bytes} byte(s), only {} available",
            data.len().saturating_sub(offset)
        ));
        return DecodedNaviTail {
            center: Some(center),
            bounds: Some(bounds),
            tail: data[offset..].to_vec(),
            ..DecodedNaviTail::default()
        };
    }

    let mut vertices = Vec::with_capacity(vertex_count);
    let mut cursor = offset;
    for _ in 0..vertex_count {
        vertices.push([
            f32_at(data, cursor).unwrap_or_default(),
            f32_at(data, cursor + 4).unwrap_or_default(),
            f32_at(data, cursor + 8).unwrap_or_default(),
        ]);
        cursor += 12;
    }
    let mut triangles = Vec::with_capacity(triangle_count);
    for _ in 0..triangle_count {
        triangles.push([
            u16::from_le_bytes(data[cursor..cursor + 2].try_into().unwrap()),
            u16::from_le_bytes(data[cursor + 2..cursor + 4].try_into().unwrap()),
            u16::from_le_bytes(data[cursor + 4..cursor + 6].try_into().unwrap()),
        ]);
        cursor += 6;
    }

    let mut trailing = [0_u8; 4];
    trailing.copy_from_slice(&data[cursor..cursor + TRAILING_LEN]);
    cursor += TRAILING_LEN;

    let leftover = data[cursor..].to_vec();
    if !leftover.is_empty() {
        diagnostics.push(format!(
            "{} unexpected trailing byte(s) after a well-formed island block",
            leftover.len()
        ));
    }
    DecodedNaviTail {
        center: Some(center),
        bounds: Some(bounds),
        island: Some(NaviIsland {
            vertices,
            triangles,
        }),
        trailing,
        tail: leftover,
    }
}
