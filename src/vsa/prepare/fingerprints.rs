//! Fingerprint validation (issue #49): four independent fingerprints -- the
//! plugin content-set, the NIF/GLB converter, the physics classification
//! pipeline, and this prepare pipeline itself -- are recorded per completed
//! cell in the resumable job manifest (`jobs.rs`, issue #48) and re-checked
//! before a later batch run skips that cell as already-valid (F49.1). Any
//! single stale component re-prepares exactly that cell (F49.2); a legacy
//! manifest entry recorded before this issue has no fingerprints at all and
//! counts as stale in every component, never as a parse error (F49.4).
//!
//! Pure std/serde, like its sibling `jobs.rs`: this module reaches into
//! `vsa::assets` for `NIF_CONVERTER_REVISION` through a relative
//! `super::super::` import -- the same pattern `selectors.rs` and
//! `batch_cache.rs` use to reach `vsa::paths`/`vsa::cell_map` -- so it stays
//! includable verbatim by `tests/features.rs` (see that file's module
//! comment; `vsa::assets` is already included there as-is).

use serde::{Deserialize, Serialize};

#[cfg(test)]
use super::super::assets::PREPARED_CONVERTER_REVISION;
use super::super::assets::material_policy_identity;

/// Physics classification/sidecar pipeline revision (F49.1). Bump whenever
/// `classify_placement`, `dynamic_rejection_reason`, or
/// `retain_static_step_support` (driven from `orchestrator.rs`) change how
/// a physics sidecar turns into a placement's physics classification --
/// distinct from `physics::PHYSICS_ASSET_SCHEMA_VERSION`, which versions the
/// sidecar file format itself, not this pipeline's interpretation of it.
pub(crate) const PHYSICS_PIPELINE_REVISION: &str = "physics-classification-v1";

/// This prepare pipeline's own revision (F49.1), independent of
/// `manifest::CURRENT_PREPARE_REVISION` (which gates *manifest schema*
/// compatibility for the viewer). Bump whenever `prepare_cell`'s staging,
/// lighting, or manifest-assembly logic changes in a way that should
/// invalidate a previously prepared cell even though its plugin content,
/// converter, and physics pipeline are unchanged. This revision combines
/// two independent bumps that raced on parallel branches: issue #120 (M4
/// wave 6) reclassifies source-dead NPC references as
/// `PreparedSemantic::Corpse` instead of `Npc`, and M4 wave 7 (#160)
/// corrected nested actor appearance resolution, M4 wave 10 (#104)
/// added the prepared actor-animation catalog, and M4 wave 12 (#106) adds
/// authored weapon animation types to the item catalog — all change what
/// `prepare_cell` produces from the same source content, so a resumable
/// *batch* prepare run must not skip a cell completed under either old
/// logic.
pub(crate) const PREPARE_PIPELINE_REVISION: &str = "prepare-pipeline-v9-static-facegen-reconstruction-corpse-nested-actors-actor-animations-weapon-animation-type-ktx2-textures-image-space-layouts-hud-sprites";

/// The four fingerprints recorded for one completed cell (F49.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CellFingerprints {
    pub(crate) plugin_content_set: String,
    pub(crate) converter: String,
    pub(crate) physics: String,
    pub(crate) prepare_pipeline: String,
}

impl CellFingerprints {
    /// The fingerprints for the current toolchain/session, given the batch
    /// session's plugin content-set fingerprint (`BatchSession::fingerprint`).
    #[cfg(test)]
    pub(crate) fn current(plugin_content_set: impl Into<String>) -> Self {
        Self::current_with_converter(plugin_content_set, PREPARED_CONVERTER_REVISION)
    }

    pub(crate) fn current_with_converter(
        plugin_content_set: impl Into<String>,
        converter_revision: &str,
    ) -> Self {
        Self {
            plugin_content_set: plugin_content_set.into(),
            converter: material_policy_identity(converter_revision),
            physics: PHYSICS_PIPELINE_REVISION.into(),
            prepare_pipeline: PREPARE_PIPELINE_REVISION.into(),
        }
    }
}

/// One of the four fingerprint components; `label()` is the token that
/// appears in the deterministic `fingerprint: cell ... stale (<component>)`
/// log line (F49.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FingerprintComponent {
    Plugin,
    Converter,
    Physics,
    PreparePipeline,
}

/// FormID-keyed stale-component report, as returned by
/// `jobs::filter_resume_checked` and used for the report-only
/// `--check-fingerprints` mode (F49.2, F49.3).
pub(crate) type StaleCells = Vec<(u32, Vec<FingerprintComponent>)>;

impl FingerprintComponent {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Plugin => "plugin",
            Self::Converter => "converter",
            Self::Physics => "physics",
            Self::PreparePipeline => "prepare_pipeline",
        }
    }
}

/// Every component in `recorded` that differs from `current`, in a fixed
/// `Plugin, Converter, Physics, PreparePipeline` order (F49.2, T49.2). A
/// cell with no recorded fingerprints at all (`recorded = None`) is a
/// legacy manifest entry (F49.4): every component counts as stale, never a
/// parse error.
pub(crate) fn stale_components(
    recorded: Option<&CellFingerprints>,
    current: &CellFingerprints,
) -> Vec<FingerprintComponent> {
    let Some(recorded) = recorded else {
        return vec![
            FingerprintComponent::Plugin,
            FingerprintComponent::Converter,
            FingerprintComponent::Physics,
            FingerprintComponent::PreparePipeline,
        ];
    };
    let mut stale = Vec::new();
    if recorded.plugin_content_set != current.plugin_content_set {
        stale.push(FingerprintComponent::Plugin);
    }
    if recorded.converter != current.converter {
        stale.push(FingerprintComponent::Converter);
    }
    if recorded.physics != current.physics {
        stale.push(FingerprintComponent::Physics);
    }
    if recorded.prepare_pipeline != current.prepare_pipeline {
        stale.push(FingerprintComponent::PreparePipeline);
    }
    stale
}

/// `fingerprint: cell <formid:08x> stale (<component>[, <component>...])`
/// (F49.2, the CLI logging convention). `components` comes from
/// `stale_components`'s fixed order, so the joined list is deterministic;
/// callers only invoke this for a non-empty `components`.
pub(crate) fn stale_cell_line(form_id: u32, components: &[FingerprintComponent]) -> String {
    let joined = components
        .iter()
        .map(|component| component.label())
        .collect::<Vec<_>>()
        .join(", ");
    format!("fingerprint: cell {form_id:08x} stale ({joined})")
}

/// `fingerprint: <n> cells valid, <m> stale` (F49.2): always printed once
/// per batch run, mirroring the always-printed `done_count done, failed`
/// summary in `orchestrator.rs`.
pub(crate) fn summary_line(valid: usize, stale: usize) -> String {
    format!("fingerprint: {valid} cells valid, {stale} stale")
}

#[cfg(test)]
#[path = "tests/fingerprints.rs"]
mod tests;
