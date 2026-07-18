//! Deterministic Fallout cell catalogue.
//!
//! OpenMW keeps a static cell store separate from runtime cell loading. This
//! slice provides the corresponding static catalogue for the preparation
//! pipeline; runtime streaming remains a later concern.

use anyhow::{Context, Result};
use bevyout_core::content::{ContentRecordResolver, ContentRecordView};
use bevyout_core::form_id::FormId;
use std::fs;
use std::path::{Path, PathBuf};

use super::cell_map::{CellMap, CellMapEntry, DoorEdge, WorldspaceEntry};
use super::openmw_esm4::{ParsedContentSet, PluginSource, parse_content_set_all};
use super::prepare::{content_set_fingerprint, load_plugin_chain};
use crate::cli::CellsArgs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellCatalogEntry {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) interior: bool,
    pub(crate) winning_plugin: String,
    pub(crate) provenance: Vec<String>,
    pub(crate) worldspace_form_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellCatalog {
    pub(crate) content_fingerprint: String,
    pub(crate) entries: Vec<CellCatalogEntry>,
    /// `(form_id, display name)` per worldspace, preferring EditorID over FULL.
    pub(crate) worldspaces: Vec<(u32, String)>,
}

/// Production adapter from the rich parser state to the stable core lookup
/// contract. Preparation keeps using the already-parsed content set instead
/// of building a second index over the same plugin bytes.
struct ParsedCellResolver<'a>(&'a ParsedContentSet);

impl ContentRecordResolver for ParsedCellResolver<'_> {
    fn resolve_form_id(&self, form_id: FormId) -> Option<ContentRecordView> {
        let (_, cell) = self
            .0
            .cells()
            .find(|(candidate, _)| **candidate == form_id.0)?;
        Some(ContentRecordView {
            form_id,
            record_type: "CELL".into(),
            editor_id: cell.editor_id.clone(),
            winning_source: self.0.cell_winning_plugin(form_id.0).map(str::to_owned),
            provenance: self.0.cell_provenance(form_id.0).to_vec(),
        })
    }

    fn resolve_editor_id(&self, editor_id: &str) -> Vec<ContentRecordView> {
        let mut records = self
            .0
            .cells()
            .filter(|(_, cell)| {
                cell.editor_id
                    .as_deref()
                    .is_some_and(|candidate| candidate.eq_ignore_ascii_case(editor_id))
            })
            .filter_map(|(form_id, _)| self.resolve_form_id(FormId(*form_id)))
            .collect::<Vec<_>>();
        records.sort_by_key(|record| record.form_id);
        records
    }
}

impl CellCatalog {
    pub(crate) fn build(sources: &[PluginSource<'_>], content_fingerprint: String) -> Result<Self> {
        let parsed = parse_content_set_all(sources)?;
        let parsed_resolver = ParsedCellResolver(&parsed);
        let resolver: &dyn ContentRecordResolver = &parsed_resolver;
        let mut entries = parsed
            .cells()
            .map(|(form_id, cell)| {
                let record = resolver
                    .resolve_form_id(FormId(*form_id))
                    .expect("parsed CELL must resolve through the content boundary");
                CellCatalogEntry {
                    form_id: *form_id,
                    editor_id: record.editor_id,
                    name: cell.name.clone(),
                    interior: cell.interior,
                    winning_plugin: record.winning_source.unwrap_or_else(|| "<unknown>".into()),
                    provenance: record.provenance,
                    worldspace_form_id: cell.worldspace_form_id,
                }
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.form_id);
        let mut worldspaces = parsed
            .worldspaces()
            .map(|(form_id, worldspace)| {
                let display = worldspace
                    .editor_id
                    .clone()
                    .or_else(|| worldspace.name.clone())
                    .unwrap_or_else(|| format!("{form_id:08x}"));
                (*form_id, display)
            })
            .collect::<Vec<_>>();
        worldspaces.sort_by_key(|(form_id, _)| *form_id);
        Ok(Self {
            content_fingerprint,
            entries,
            worldspaces,
        })
    }

    fn output(&self, interiors_only: bool) -> String {
        let mut lines = vec!["form_id\tkind\teditor_id\tname\twinning_plugin".to_string()];
        lines.extend(
            self.entries
                .iter()
                .filter(|entry| !interiors_only || entry.interior)
                .map(|entry| {
                    format!(
                        "{:08x}\t{}\t{}\t{}\t{}",
                        entry.form_id,
                        if entry.interior {
                            "interior"
                        } else {
                            "exterior"
                        },
                        display_field(entry.editor_id.as_deref()),
                        display_field(entry.name.as_deref()),
                        display_field(Some(&entry.winning_plugin)),
                    )
                }),
        );
        lines.join("\n")
    }
}

pub fn cells(args: CellsArgs) -> Result<()> {
    let game_root = args
        .game_root
        .context("Fallout 3 is not configured; pass --game-root or create .bevyout/config.toml")?;
    let root = fs::canonicalize(&game_root).context("game root does not exist")?;
    let plugin = args.plugin.unwrap_or_else(|| PathBuf::from("Fallout3.esm"));
    let plugin_path = resolve_plugin_path(&root, plugin);
    let plugin_path = fs::canonicalize(&plugin_path).context("plugin does not exist")?;
    let loaded_plugins = load_plugin_chain(&plugin_path, &root.join("Data"))?;
    let fingerprint = content_set_fingerprint(&loaded_plugins);
    let sources = loaded_plugins
        .iter()
        .map(|plugin| PluginSource {
            name: &plugin.name,
            bytes: &plugin.bytes,
        })
        .collect::<Vec<_>>();
    if args.map {
        let map = build_cell_map(&sources, fingerprint)?;
        let ron_text = map.to_ron().context("serializing cell map to RON")?;
        match &args.out {
            Some(path) => fs::write(path, &ron_text)
                .with_context(|| format!("writing cell map to {}", path.display()))?,
            None => println!("{ron_text}"),
        }
        // Kept off stdout (which may be carrying the RON artifact itself)
        // and matches `prepare`'s one-summary-line-per-run convention
        // (F45.5).
        eprintln!(
            "cell map: {} cells ({} exterior with grid), {} worldspaces, {} door edges, {} unresolved doors",
            map.cells.len(),
            map.cells.iter().filter(|cell| cell.grid.is_some()).count(),
            map.worldspaces.len(),
            map.doors.len(),
            map.unresolved_door_count,
        );
        return Ok(());
    }
    let catalog = CellCatalog::build(&sources, fingerprint)?;
    println!("{}", catalog.output(args.interiors_only));
    Ok(())
}

/// Converts the parser's `ParsedContentSet` (openmw_esm4) into the plain
/// inputs `cell_map::CellMap::build` accepts. This is the boundary that
/// keeps OpenMW-derived types out of `cell_map.rs`'s public surface, the
/// same way `CellCatalog::build` above converts at the boundary rather than
/// letting `CellInfo`/parser types leak into the catalogue type.
///
/// `pub(crate)` (rather than private) so a batch `prepare` run (issue #47)
/// can reuse this exact builder to write `cellmap.ron` into the cache dir
/// from the content set it already parsed, instead of re-implementing the
/// `ParsedContentSet` -> `CellMap` conversion a second time.
pub(crate) fn build_cell_map(
    sources: &[PluginSource<'_>],
    content_fingerprint: String,
) -> Result<CellMap> {
    let parsed = parse_content_set_all(sources)?;
    let cells = parsed
        .cells()
        .map(|(form_id, cell)| CellMapEntry {
            form_id: *form_id,
            editor_id: cell.editor_id.clone(),
            interior: cell.interior,
            worldspace_form_id: cell.worldspace_form_id,
            grid: cell.grid,
        })
        .collect::<Vec<_>>();
    let worldspaces = parsed
        .worldspaces()
        .map(|(_, worldspace)| WorldspaceEntry {
            form_id: worldspace.form_id,
            editor_id: worldspace.editor_id.clone(),
            name: worldspace.name.clone(),
        })
        .collect::<Vec<_>>();
    let (edges, unresolved_door_count) = parsed.door_edges();
    let doors = edges
        .into_iter()
        .map(|edge| DoorEdge {
            source_cell_form_id: edge.source_cell_form_id,
            door_reference_form_id: edge.door_reference_form_id,
            destination_cell_form_id: edge.destination_cell_form_id,
            destination_door_reference_form_id: edge.destination_door_reference_form_id,
            position: edge.position,
            rotation: edge.rotation,
        })
        .collect::<Vec<_>>();
    Ok(CellMap::build(
        content_fingerprint,
        worldspaces,
        cells,
        doors,
        unresolved_door_count,
    ))
}

fn resolve_plugin_path(root: &Path, plugin: PathBuf) -> PathBuf {
    if plugin.is_absolute() {
        plugin
    } else {
        root.join("Data").join(plugin)
    }
}

fn display_field(value: Option<&str>) -> String {
    value
        .unwrap_or("")
        .chars()
        .map(|character| match character {
            '\t' | '\r' | '\n' => ' ',
            character => character,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut result = signature.to_vec();
        result.extend_from_slice(&(data.len() as u16).to_le_bytes());
        result.extend_from_slice(data);
        result
    }

    fn record(signature: &[u8; 4], form_id: u32, data: &[u8]) -> Vec<u8> {
        let mut result = signature.to_vec();
        result.extend_from_slice(&(data.len() as u32).to_le_bytes());
        result.extend_from_slice(&0_u32.to_le_bytes());
        result.extend_from_slice(&form_id.to_le_bytes());
        result.extend_from_slice(&[0; 8]);
        result.extend_from_slice(data);
        result
    }

    fn plugin_with_cell(form_id: u32, editor_id: &str) -> Vec<u8> {
        let mut bytes = record(b"TES4", 0, &[]);
        bytes.extend(record(
            b"CELL",
            form_id,
            &subrecord(b"EDID", format!("{editor_id}\0").as_bytes()),
        ));
        bytes
    }

    #[test]
    fn output_is_deterministic_and_sanitizes_tsv_fields() {
        let catalog = CellCatalog {
            content_fingerprint: "fingerprint".into(),
            entries: vec![
                CellCatalogEntry {
                    form_id: 2,
                    editor_id: Some("B\nCell".into()),
                    name: Some("Second".into()),
                    interior: false,
                    winning_plugin: "Patch.esp".into(),
                    provenance: vec!["Fallout3.esm".into(), "Patch.esp".into()],
                    worldspace_form_id: Some(60),
                },
                CellCatalogEntry {
                    form_id: 1,
                    editor_id: Some("A".into()),
                    name: Some("First".into()),
                    interior: true,
                    winning_plugin: "Fallout3.esm".into(),
                    provenance: vec!["Fallout3.esm".into()],
                    worldspace_form_id: None,
                },
            ],
            worldspaces: vec![(60, "Wasteland".into())],
        };
        assert!(
            catalog
                .output(true)
                .contains("00000001\tinterior\tA\tFirst")
        );
        assert!(!catalog.output(true).contains("00000002"));
        assert_eq!(display_field(Some("line\nvalue")), "line value");
    }

    #[test]
    fn catalog_build_routes_record_identity_through_the_core_resolver_contract() {
        let plugin = plugin_with_cell(0x100, "ResolverCell");
        let sources = [PluginSource {
            name: "synthetic.esp",
            bytes: &plugin,
        }];

        let catalog = CellCatalog::build(&sources, "fixture".into()).unwrap();
        assert_eq!(catalog.entries.len(), 1);
        assert_eq!(catalog.entries[0].form_id, 0x100);
        assert_eq!(
            catalog.entries[0].editor_id.as_deref(),
            Some("ResolverCell")
        );
        assert_eq!(catalog.entries[0].winning_plugin, "synthetic.esp");
        assert_eq!(catalog.entries[0].provenance, ["synthetic.esp"]);
    }
}
