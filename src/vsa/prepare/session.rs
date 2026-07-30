//! Batch session (issue #47): parses the plugin chain and indexes BSA/audio
//! archives once per `prepare` invocation, then shares that state -- plus a
//! session-level physics sidecar cache (`batch_cache::KeyedBatchCache`) --
//! across every cell prepared in the run.
//!
//! Before this issue, `prepare_batch` (batch selectors, issue #46) called
//! `prepare_one` once per selected cell, and `prepare_one` re-read the
//! plugin chain from disk, re-validated it with esplugin, re-indexed every
//! BSA, and re-staged the (cell-independent) footstep clip set on every
//! single call -- all repeated work for a batch of N cells that only needs
//! doing once.
//!
//! `BatchSession::new` does each of those exactly once. Its constructor
//! takes the already-loaded plugin chain (`Vec<LoadedPlugin>`) rather than a
//! path to load one from, so `prepare_cell` in `orchestrator.rs` -- which
//! only ever receives `&BatchSession`, never a plugin path -- cannot
//! reload the chain even by accident. That is a type-level guarantee, not
//! just a convention (T47.2). The single-cell CLI path builds a one-cell
//! `BatchSession` too, so its plugin/BSA/audio/footstep loading and output
//! are unchanged from before this issue (F47.1).
//!
//! Issue #48 adds a bounded worker pool that runs several cells'
//! `prepare_cell` concurrently against one shared `&BatchSession` (no
//! longer `&mut`, so several worker threads can hold it at once). The
//! session's only fields any cell ever mutates -- `physics_cache` and
//! `asset_totals` -- are wrapped in `Mutex` so that still compiles and
//! stays correct under concurrent access; `blender_lock` is a new field
//! with no counterpart before #48, guarding the one part of `prepare_cell`
//! that is not provably safe to run for two cells at once (see its use in
//! `orchestrator.rs`).

use super::*;

pub(crate) struct BatchSession {
    pub(crate) loaded_plugins: Vec<LoadedPlugin>,
    pub(crate) fingerprint: String,
    /// esplugin validation diagnostics for the selected plugin, computed
    /// once. Same content `prepare_one` produced before this issue;
    /// `prepare_cell` extends it into each cell's diagnostics at the same
    /// relative position the original inline validation occupied.
    pub(crate) plugin_diagnostics: Vec<Diagnostic>,
    pub(crate) archives: Vec<crate::vsa::bsa::BsaArchive>,
    pub(crate) archive_diagnostics: Vec<Diagnostic>,
    pub(crate) audio_archives: Vec<crate::vsa::audio_assets::AudioArchive>,
    pub(crate) audio_diagnostics: Vec<Diagnostic>,
    pub(crate) footstep_sets: Vec<crate::vsa::manifest::PreparedFootstepSet>,
    pub(crate) hard_landing_clips: Vec<String>,
    pub(crate) footstep_diagnostics: Vec<Diagnostic>,
    /// Session-level physics sidecar cache (F47.3): a sidecar read once for
    /// one cell is reused, as a hit, by every later cell in the batch that
    /// references the same content-addressed physics asset. `Mutex`-wrapped
    /// (#48) so concurrent workers share one cache instead of racing on it.
    pub(crate) physics_cache: Mutex<KeyedBatchCache<PreparedPhysicsAsset>>,
    pub(crate) asset_totals: Mutex<BatchAssetTotals>,
    /// Serializes the one part of `prepare_cell` this issue did not judge
    /// provably safe to run concurrently: staging/converting textures and
    /// running Blender both touch the whole shared `staging_dir` (a fixed
    /// `blender_jobs.ron` filename, and `convert_staged_textures` walking
    /// every `.dds` under the directory rather than just the current
    /// cell's). See the long comment at that call site in `orchestrator.rs`
    /// for the full argument (F48.4).
    pub(crate) blender_lock: Mutex<()>,
}

impl BatchSession {
    /// Builds a session from an already-loaded plugin chain. `cache_dir` is
    /// only used here to stage the (cell-independent) footstep clip set
    /// once; every other cache subdirectory is still resolved per cell in
    /// `prepare_cell`, matching `prepare_one`'s prior behavior.
    pub(crate) fn new(
        plugin_path: &Path,
        data_root: &Path,
        cache_dir: &Path,
        loaded_plugins: Vec<LoadedPlugin>,
        fingerprint: String,
    ) -> Result<Self> {
        let mut plugin_diagnostics = Vec::new();
        let mut validator = esplugin::Plugin::new(esplugin::GameId::Fallout3, plugin_path);
        if let Err(error) = validator.parse_file(esplugin::ParseOptions::header_only()) {
            plugin_diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!("esplugin validation failed: {error}"),
            });
        }

        let mut archive_diagnostics = Vec::new();
        let archives = load_archives(data_root, &mut archive_diagnostics)?;

        let audio_plugin_names = loaded_plugins
            .iter()
            .rev()
            .map(|plugin| plugin.name.clone())
            .collect::<Vec<_>>();
        let audio_archive_load = load_audio_archives(data_root, &audio_plugin_names);
        let audio_diagnostics = audio_archive_load
            .diagnostics
            .into_iter()
            .map(|message| Diagnostic {
                severity: "info".into(),
                message,
            })
            .collect::<Vec<_>>();

        let mut footstep_diagnostics = Vec::new();
        let (footstep_sets, hard_landing_clips) = stage_footsteps(
            data_root,
            &audio_archive_load.archives,
            &mut footstep_diagnostics,
            &cache_dir.join("audio"),
        )?;

        Ok(Self {
            loaded_plugins,
            fingerprint,
            plugin_diagnostics,
            archives,
            archive_diagnostics,
            audio_archives: audio_archive_load.archives,
            audio_diagnostics,
            footstep_sets,
            hard_landing_clips,
            footstep_diagnostics,
            physics_cache: Mutex::new(KeyedBatchCache::default()),
            asset_totals: Mutex::new(BatchAssetTotals::default()),
            blender_lock: Mutex::new(()),
        })
    }

    /// Plugin sources for `parse_content_set`, rebuilt per cell. This is
    /// cheap -- only reference pointers into `loaded_plugins`, no I/O and no
    /// re-parse of the plugin chain itself -- since `parse_content_set`
    /// needs a fresh borrow per call.
    pub(crate) fn plugin_sources(&self) -> Vec<PluginSource<'_>> {
        self.loaded_plugins
            .iter()
            .map(|plugin| PluginSource {
                name: &plugin.name,
                bytes: &plugin.bytes,
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "tests/session.rs"]
mod tests;
