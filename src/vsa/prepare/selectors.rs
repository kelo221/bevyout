//! Pure batch cell selection for `prepare --all` / `--all-interiors` /
//! `--worldspace` / `--exterior-radius` / multiple positional selectors.
//!
//! Depends only on `std`/`anyhow` plus `vsa::paths` for the shared
//! EditorID/FormID selector grammar (`parse_cell_selector`), reached through
//! a relative `super::super::` import so this file can be pulled in verbatim
//! (via `#[path]`, see `tests/features.rs`, which nests it one module deep so
//! the relative path lands on its included copy of `paths`).

use std::collections::BTreeSet;

use anyhow::{Context, Result, bail};

/// A minimal per-cell summary the selection resolver needs. Sourced from
/// `CellCatalogEntry` (`vsa::catalog`) at the real CLI call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellSummary {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) interior: bool,
    pub(crate) worldspace_form_id: Option<u32>,
    pub(crate) grid: Option<(i32, i32)>,
}

/// The batch selection requested on the CLI, translated from `PrepareArgs`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SelectionSpec {
    /// Selects every cell in the catalogue; subsumes every other field.
    pub(crate) all: bool,
    /// Selects every interior cell. Combinable with `worldspace`/`explicit`.
    pub(crate) all_interiors: bool,
    /// Selects every exterior cell, excluding interiors regardless of whether
    /// worldspace or grid metadata is present.
    pub(crate) all_exteriors: bool,
    /// Selects every cell belonging to this worldspace (EditorID or FormID).
    pub(crate) worldspace: Option<String>,
    /// Selects a square exterior-cell neighborhood around the sole explicit
    /// selector, using Chebyshev distance in the anchor's worldspace grid.
    pub(crate) exterior_radius: Option<u32>,
    /// Explicit EditorID/FormID selectors, in the order given on the CLI.
    pub(crate) explicit: Vec<String>,
}

impl SelectionSpec {
    /// True when nothing was selected. `prepare --retry-failed` with no other
    /// selector is the one place this is a valid, meaningful state rather than
    /// a CLI usage error (F48.3): it means "every failed cell in the manifest".
    pub(crate) fn is_empty(&self) -> bool {
        !self.all
            && !self.all_interiors
            && !self.all_exteriors
            && self.worldspace.is_none()
            && self.exterior_radius.is_none()
            && self.explicit.is_empty()
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
            "no cell selection was given; pass --all, --all-interiors, --all-exteriors, \
             --worldspace, --exterior-radius with one anchor, \
             or one or more EditorID/FormID selectors"
        );
    }

    if spec.all {
        let mut ids: Vec<u32> = cells.iter().map(|cell| cell.form_id).collect();
        ids.sort_unstable();
        ids.dedup();
        return Ok(ids);
    }

    if spec.all_exteriors {
        let mut ids: Vec<u32> = cells
            .iter()
            .filter(|cell| !cell.interior)
            .map(|cell| cell.form_id)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        return Ok(ids);
    }

    if let Some(radius) = spec.exterior_radius {
        if spec.explicit.len() != 1 {
            bail!("--exterior-radius requires exactly one positional exterior cell anchor");
        }
        let raw_anchor = &spec.explicit[0];
        let anchor = find_explicit_cell(raw_anchor, cells)?;
        if anchor.interior {
            bail!(
                "--exterior-radius anchor '{raw_anchor}' resolves to interior cell {:08x}",
                anchor.form_id
            );
        }
        let worldspace_form_id = anchor.worldspace_form_id.with_context(|| {
            format!(
                "--exterior-radius anchor '{raw_anchor}' ({:08x}) is missing worldspace metadata",
                anchor.form_id
            )
        })?;
        let (anchor_x, anchor_y) = anchor.grid.with_context(|| {
            format!(
                "--exterior-radius anchor '{raw_anchor}' ({:08x}) is missing exterior grid metadata",
                anchor.form_id
            )
        })?;
        let mut ids = cells
            .iter()
            .filter(|cell| !cell.interior)
            .filter(|cell| cell.worldspace_form_id == Some(worldspace_form_id))
            .filter(|cell| {
                cell.grid.is_some_and(|(x, y)| {
                    x.abs_diff(anchor_x) <= radius && y.abs_diff(anchor_y) <= radius
                })
            })
            .map(|cell| cell.form_id)
            .collect::<Vec<_>>();
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
    find_explicit_cell(raw_selector, cells).map(|cell| cell.form_id)
}

fn find_explicit_cell<'a>(raw_selector: &str, cells: &'a [CellSummary]) -> Result<&'a CellSummary> {
    let parsed = parse_cell_selector(raw_selector)?;
    let found = match parsed {
        CellSelector::FormId(form_id) => cells.iter().find(|cell| cell.form_id == form_id),
        CellSelector::EditorId(name) => cells.iter().find(|cell| {
            cell.editor_id
                .as_deref()
                .is_some_and(|editor_id| editor_id.eq_ignore_ascii_case(&name))
        }),
    };
    found.ok_or_else(|| {
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
