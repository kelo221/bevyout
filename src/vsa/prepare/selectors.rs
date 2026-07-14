//! Pure batch cell selection for `prepare --all` / `--all-interiors` /
//! `--worldspace` / multiple positional selectors (issue #46).
//!
//! Deliberately depends only on `std`/`anyhow` so this file can be pulled in
//! verbatim (via `#[path]`, see `tests/features.rs`) without dragging in
//! `bevy`, `serde`, or the ESM4 parser -- mirroring how `vsa::paths` is
//! included in that suite today.

use std::collections::BTreeSet;

use anyhow::{Result, bail};

/// A minimal per-cell summary the selection resolver needs. Sourced from
/// `CellCatalogEntry` (`vsa::catalog`) at the real CLI call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellSummary {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) interior: bool,
    pub(crate) worldspace_form_id: Option<u32>,
}

/// The batch selection requested on the CLI, translated from `PrepareArgs`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SelectionSpec {
    /// Selects every cell in the catalogue; subsumes every other field.
    pub(crate) all: bool,
    /// Selects every interior cell. Combinable with `worldspace`/`explicit`.
    pub(crate) all_interiors: bool,
    /// Selects every cell belonging to this worldspace (EditorID or FormID).
    pub(crate) worldspace: Option<String>,
    /// Explicit EditorID/FormID selectors, in the order given on the CLI.
    pub(crate) explicit: Vec<String>,
}

impl SelectionSpec {
    fn is_empty(&self) -> bool {
        !self.all && !self.all_interiors && self.worldspace.is_none() && self.explicit.is_empty()
    }
}

enum ParsedSelector {
    FormId(u32),
    EditorId(String),
}

/// Parses a selector the same way `vsa::paths::parse_cell_selector` does:
/// an (optionally `0x`-prefixed) string of up to eight hex digits is a
/// FormID, anything else is an EditorID. Duplicated rather than imported
/// because `paths.rs` pulls in `bevy` and this module must not.
fn parse_selector(value: &str) -> Result<ParsedSelector> {
    let value = value.trim();
    if value.is_empty() {
        bail!("cell selector must be a GECK EditorID or hexadecimal FormID");
    }
    let form_id_digits = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X"));
    let looks_like_form_id = form_id_digits.is_some_and(|digits| {
        !digits.is_empty() && digits.len() <= 8 && digits.chars().all(|c| c.is_ascii_hexdigit())
    }) || (value.len() <= 8 && value.chars().all(|c| c.is_ascii_hexdigit()));
    if looks_like_form_id {
        let digits = form_id_digits.unwrap_or(value);
        let form_id = u32::from_str_radix(digits, 16)
            .map_err(|error| anyhow::anyhow!("cell must be a hexadecimal FormID: {error}"))?;
        return Ok(ParsedSelector::FormId(form_id));
    }
    Ok(ParsedSelector::EditorId(value.to_owned()))
}

/// Resolves a [`SelectionSpec`] against a cell catalogue into a FormID-sorted,
/// deduplicated list of cell FormIDs. Pure: no I/O, no game data.
pub(crate) fn resolve_selection(
    cells: &[CellSummary],
    worldspace_names: &[(u32, String)],
    spec: &SelectionSpec,
) -> Result<Vec<u32>> {
    if spec.is_empty() {
        bail!(
            "no cell selection was given; pass --all, --all-interiors, --worldspace, \
             or one or more EditorID/FormID selectors"
        );
    }

    if spec.all {
        let mut ids: Vec<u32> = cells.iter().map(|cell| cell.form_id).collect();
        ids.sort_unstable();
        ids.dedup();
        return Ok(ids);
    }

    let mut selected = BTreeSet::new();

    if spec.all_interiors {
        selected.extend(cells.iter().filter(|cell| cell.interior).map(|cell| cell.form_id));
    }

    if let Some(worldspace) = &spec.worldspace {
        let worldspace_form_id = resolve_worldspace(worldspace, worldspace_names)?;
        selected.extend(
            cells
                .iter()
                .filter(|cell| cell.worldspace_form_id == Some(worldspace_form_id))
                .map(|cell| cell.form_id),
        );
    }

    for raw_selector in &spec.explicit {
        selected.insert(resolve_explicit_cell(raw_selector, cells)?);
    }

    Ok(selected.into_iter().collect())
}

fn resolve_worldspace(worldspace: &str, worldspace_names: &[(u32, String)]) -> Result<u32> {
    let parsed = parse_selector(worldspace)?;
    let matched = match parsed {
        ParsedSelector::FormId(form_id) => worldspace_names
            .iter()
            .find(|(candidate, _)| *candidate == form_id)
            .map(|(form_id, _)| *form_id),
        ParsedSelector::EditorId(name) => worldspace_names
            .iter()
            .find(|(_, candidate)| candidate.eq_ignore_ascii_case(&name))
            .map(|(form_id, _)| *form_id),
    };
    matched.ok_or_else(|| {
        let mut available: Vec<&str> = worldspace_names
            .iter()
            .map(|(_, name)| name.as_str())
            .collect();
        available.sort_unstable();
        if available.is_empty() {
            anyhow::anyhow!(
                "unknown worldspace '{worldspace}'; no worldspaces are available in this content set"
            )
        } else {
            anyhow::anyhow!(
                "unknown worldspace '{worldspace}'; available worldspaces: {}",
                available.join(", ")
            )
        }
    })
}

fn resolve_explicit_cell(raw_selector: &str, cells: &[CellSummary]) -> Result<u32> {
    let parsed = parse_selector(raw_selector)?;
    let found = match parsed {
        ParsedSelector::FormId(form_id) => cells.iter().find(|cell| cell.form_id == form_id),
        ParsedSelector::EditorId(name) => cells.iter().find(|cell| {
            cell.editor_id
                .as_deref()
                .is_some_and(|editor_id| editor_id.eq_ignore_ascii_case(&name))
        }),
    };
    found.map(|cell| cell.form_id).ok_or_else(|| {
        let candidates = near_candidates(cells, raw_selector);
        if candidates.is_empty() {
            anyhow::anyhow!("unknown cell selector '{raw_selector}'")
        } else {
            anyhow::anyhow!(
                "unknown cell selector '{raw_selector}'; did you mean: {}",
                candidates.join(", ")
            )
        }
    })
}

/// Up to five EditorIDs containing `query` (case-insensitively), sorted, as
/// an actionable hint on an unknown-selector error.
fn near_candidates(cells: &[CellSummary], query: &str) -> Vec<String> {
    let needle = query.to_ascii_lowercase();
    let mut matches: Vec<String> = cells
        .iter()
        .filter_map(|cell| cell.editor_id.as_deref())
        .filter(|editor_id| editor_id.to_ascii_lowercase().contains(&needle))
        .take(5)
        .map(str::to_owned)
        .collect();
    matches.sort();
    matches.dedup();
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(form_id: u32, editor_id: &str, interior: bool) -> CellSummary {
        CellSummary {
            form_id,
            editor_id: Some(editor_id.to_owned()),
            name: None,
            interior,
            worldspace_form_id: None,
        }
    }

    fn cell_in_worldspace(form_id: u32, editor_id: &str, worldspace_form_id: u32) -> CellSummary {
        CellSummary {
            form_id,
            editor_id: Some(editor_id.to_owned()),
            name: None,
            interior: false,
            worldspace_form_id: Some(worldspace_form_id),
        }
    }

    // T46.1
    #[test]
    fn all_interiors_yields_exactly_the_interior_subset_sorted() {
        let cells = vec![
            cell(0x00000005, "ExtB", false),
            cell(0x00000001, "IntA", true),
            cell(0x00000003, "IntC", true),
        ];
        let spec = SelectionSpec {
            all_interiors: true,
            ..Default::default()
        };
        let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");
        assert_eq!(resolved, vec![0x00000001, 0x00000003]);
    }

    // T46.2
    #[test]
    fn explicit_list_mixing_editor_id_and_form_id_resolves_dedupes_and_sorts() {
        let cells = vec![
            cell(0x00000010, "VaultDoor", true),
            cell(0x00000002, "Wasteland", false),
        ];
        let spec = SelectionSpec {
            explicit: vec![
                "Wasteland".to_string(),
                "00000010".to_string(),
                "wasteland".to_string(),
            ],
            ..Default::default()
        };
        let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");
        assert_eq!(resolved, vec![0x00000002, 0x00000010]);
    }

    // T46.3
    #[test]
    fn unknown_worldspace_errors_and_names_available_worldspaces() {
        let cells = vec![cell_in_worldspace(0x00000001, "Cell1", 0x00000100)];
        let worldspace_names = vec![(0x00000100, "Capital Wasteland".to_string())];
        let spec = SelectionSpec {
            worldspace: Some("Nowhere".to_string()),
            ..Default::default()
        };
        let error = resolve_selection(&cells, &worldspace_names, &spec).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("Nowhere"), "{message}");
        assert!(message.contains("Capital Wasteland"), "{message}");
    }

    #[test]
    fn all_subsumes_every_other_selector_and_is_sorted() {
        let cells = vec![
            cell(0x00000005, "ExtB", false),
            cell(0x00000001, "IntA", true),
        ];
        let spec = SelectionSpec {
            all: true,
            explicit: vec!["nonexistent".to_string()],
            ..Default::default()
        };
        let resolved = resolve_selection(&cells, &[], &spec).expect("resolves");
        assert_eq!(resolved, vec![0x00000001, 0x00000005]);
    }

    #[test]
    fn all_interiors_and_worldspace_and_explicit_selectors_union() {
        let cells = vec![
            cell(0x00000001, "IntA", true),
            cell_in_worldspace(0x00000002, "ExtInWorld", 0x00000100),
            cell(0x00000003, "Explicit", false),
        ];
        let worldspace_names = vec![(0x00000100, "Capital Wasteland".to_string())];
        let spec = SelectionSpec {
            all_interiors: true,
            worldspace: Some("Capital Wasteland".to_string()),
            explicit: vec!["Explicit".to_string()],
            ..Default::default()
        };
        let resolved = resolve_selection(&cells, &worldspace_names, &spec).expect("resolves");
        assert_eq!(resolved, vec![0x00000001, 0x00000002, 0x00000003]);
    }

    #[test]
    fn unknown_explicit_selector_names_near_candidates() {
        let cells = vec![cell(0x00000001, "SuperDuperMart", false)];
        let spec = SelectionSpec {
            explicit: vec!["SuperDuper".to_string()],
            ..Default::default()
        };
        let error = resolve_selection(&cells, &[], &spec).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("SuperDuper"), "{message}");
        assert!(message.contains("SuperDuperMart"), "{message}");
    }

    #[test]
    fn empty_selection_spec_is_an_error() {
        assert!(resolve_selection(&[], &[], &SelectionSpec::default()).is_err());
    }
}
