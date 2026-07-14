//! Deterministic Fallout cell catalogue.
//!
//! OpenMW keeps a static cell store separate from runtime cell loading. This
//! slice provides the corresponding static catalogue for the preparation
//! pipeline; runtime streaming remains a later concern.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::openmw_esm4::{PluginSource, parse_content_set_all};
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellCatalog {
    pub(crate) content_fingerprint: String,
    pub(crate) entries: Vec<CellCatalogEntry>,
}

impl CellCatalog {
    pub(crate) fn build(sources: &[PluginSource<'_>], content_fingerprint: String) -> Result<Self> {
        let parsed = parse_content_set_all(sources)?;
        let mut entries = parsed
            .cells()
            .map(|(form_id, cell)| CellCatalogEntry {
                form_id: *form_id,
                editor_id: cell.editor_id.clone(),
                name: cell.name.clone(),
                interior: cell.interior,
                winning_plugin: parsed
                    .cell_winning_plugin(*form_id)
                    .unwrap_or("<unknown>")
                    .to_string(),
                provenance: parsed.cell_provenance(*form_id).to_vec(),
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.form_id);
        Ok(Self {
            content_fingerprint,
            entries,
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
    let catalog = CellCatalog::build(&sources, fingerprint)?;
    println!("{}", catalog.output(args.interiors_only));
    Ok(())
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
                },
                CellCatalogEntry {
                    form_id: 1,
                    editor_id: Some("A".into()),
                    name: Some("First".into()),
                    interior: true,
                    winning_plugin: "Fallout3.esm".into(),
                    provenance: vec!["Fallout3.esm".into()],
                },
            ],
        };
        assert!(
            catalog
                .output(true)
                .contains("00000001\tinterior\tA\tFirst")
        );
        assert!(!catalog.output(true).contains("00000002"));
        assert_eq!(display_field(Some("line\nvalue")), "line value");
    }
}
