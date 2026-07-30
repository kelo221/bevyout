//! Pure batch cell selection for `prepare --all` / `--all-interiors` /
//! `--worldspace` / multiple positional selectors (issue #46).
//!
//! Depends only on `std`/`anyhow` plus `vsa::paths` for the shared
//! EditorID/FormID selector grammar (`parse_cell_selector`), reached through
//! a relative `super::super::` import so this file can be pulled in verbatim
//! (via `#[path]`, see `tests/features.rs`, which nests it one module deep so
//! the relative path lands on its included copy of `paths`).

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
    /// True when nothing was selected -- no `--all`/`--all-interiors`, no
    /// `--worldspace`, and no explicit selectors. `prepare --retry-failed`
    /// with no other selector is the one place this is a valid, meaningful
    /// state rather than a CLI usage error (F48.3): it means "every failed
    /// cell in the manifest", so the orchestrator checks this instead of
    /// calling `resolve_selection` (which treats an empty spec as an error).
    pub(crate) fn is_empty(&self) -> bool {
        !self.all && !self.all_interiors && self.worldspace.is_none() && self.explicit.is_empty()
    }
}

use super::super::paths::{CellSelector, parse_cell_selector};

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
        selected.extend(
            cells
                .iter()
                .filter(|cell| cell.interior)
                .map(|cell| cell.form_id),
        );
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
    let parsed = parse_cell_selector(worldspace)?;
    let matched = match parsed {
        CellSelector::FormId(form_id) => worldspace_names
            .iter()
            .find(|(candidate, _)| *candidate == form_id)
            .map(|(form_id, _)| *form_id),
        CellSelector::EditorId(name) => worldspace_names
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
    let parsed = parse_cell_selector(raw_selector)?;
    let found = match parsed {
        CellSelector::FormId(form_id) => cells.iter().find(|cell| cell.form_id == form_id),
        CellSelector::EditorId(name) => cells.iter().find(|cell| {
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
#[path = "tests/selectors.rs"]
mod tests;
