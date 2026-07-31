//! Deterministic prepared dialogue voice coverage and readiness policy.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

use anyhow::{Context, Result};
use bevyout_core::dialogue::{
    DialogueLineKey, DialogueVoiceAsset, PreparedDialogueBundleRef,
    PreparedDialogueVoiceDemandReport, PreparedDialogueVoiceIndex,
};
use sha2::{Digest, Sha256};

use super::{
    DialogueVoiceRequirementOrigin, PreparedDialogueCatalog, PreparedDialogueVoiceRequirement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MissingDialogueVoice {
    pub(crate) line_key: DialogueLineKey,
    pub(crate) speaker_form_id: Option<u32>,
}

impl MissingDialogueVoice {
    pub(crate) fn label(&self) -> String {
        self.speaker_form_id.map_or_else(
            || self.line_key.to_string(),
            |speaker| format!("{}@speaker={speaker:08x}", self.line_key),
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DialogueVoiceCoverage {
    pub(crate) total_lines: usize,
    pub(crate) mapped_lines: usize,
    pub(crate) missing_authored: Vec<MissingDialogueVoice>,
    pub(crate) missing_fallout: Vec<MissingDialogueVoice>,
    pub(crate) invalid_assets: Vec<String>,
}

impl DialogueVoiceCoverage {
    pub(crate) fn missing_lines(&self) -> usize {
        self.total_lines.saturating_sub(self.mapped_lines)
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.missing_lines() == 0 && self.invalid_assets.is_empty()
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "dialogue voice coverage: total={}, mapped={}, missing={}, missing_authored={}, fallout_discovery_failures={}",
            self.total_lines,
            self.mapped_lines,
            self.missing_lines(),
            self.missing_authored.len(),
            self.missing_fallout.len(),
        )
    }

    pub(crate) fn missing_labels(&self) -> Vec<String> {
        self.missing_authored
            .iter()
            .chain(&self.missing_fallout)
            .map(MissingDialogueVoice::label)
            .collect()
    }
}

pub(crate) fn read_prepared_voice_coverage(
    asset_root: &Path,
    bundle: &PreparedDialogueBundleRef,
) -> Result<(PreparedDialogueCatalog, DialogueVoiceCoverage)> {
    let catalog_path = prepared_path(asset_root, &bundle.catalog_path);
    let catalog = ron::de::from_bytes::<PreparedDialogueCatalog>(
        &fs::read(&catalog_path)
            .with_context(|| format!("reading dialogue catalog {}", catalog_path.display()))?,
    )
    .with_context(|| format!("parsing dialogue catalog {}", catalog_path.display()))?;
    let index = bundle
        .voice_index_path
        .as_deref()
        .map(|relative| {
            let path = prepared_path(asset_root, relative);
            ron::de::from_bytes::<PreparedDialogueVoiceIndex>(
                &fs::read(&path)
                    .with_context(|| format!("reading dialogue voice index {}", path.display()))?,
            )
            .with_context(|| format!("parsing dialogue voice index {}", path.display()))
        })
        .transpose()?;
    let demand = bundle
        .voice_demand_path
        .as_deref()
        .map(|relative| {
            let path = prepared_path(asset_root, relative);
            ron::de::from_bytes::<PreparedDialogueVoiceDemandReport>(
                &fs::read(&path)
                    .with_context(|| format!("reading dialogue voice demand {}", path.display()))?,
            )
            .with_context(|| format!("parsing dialogue voice demand {}", path.display()))
        })
        .transpose()?;
    super::validate_dialogue_bundle_metadata(bundle, &catalog, index.as_ref(), demand.as_ref())?;
    let coverage = assess_voice_coverage(asset_root, &catalog, index.as_ref());
    Ok((catalog, coverage))
}

pub(crate) fn assess_voice_coverage(
    asset_root: &Path,
    catalog: &PreparedDialogueCatalog,
    index: Option<&PreparedDialogueVoiceIndex>,
) -> DialogueVoiceCoverage {
    let fallback_requirements;
    let requirements = if catalog.voice_requirements.is_empty() {
        fallback_requirements = catalog
            .line_keys
            .iter()
            .cloned()
            .map(|line_key| PreparedDialogueVoiceRequirement {
                line_key,
                speaker_form_id: None,
                source_path: "<legacy-catalog>".into(),
                origin: DialogueVoiceRequirementOrigin::Authored,
            })
            .collect::<Vec<_>>();
        fallback_requirements.as_slice()
    } else {
        catalog.voice_requirements.as_slice()
    };

    let mut coverage = DialogueVoiceCoverage {
        total_lines: requirements.len(),
        ..Default::default()
    };
    let mut invalid_assets = BTreeSet::new();
    for requirement in requirements {
        let mapped = index.and_then(|index| {
            index.entries.iter().find(|entry| {
                entry.line_key == requirement.line_key
                    && entry.speaker_form_id == requirement.speaker_form_id
            })
        });
        match mapped {
            Some(asset) => match validate_prepared_voice_asset(asset_root, asset) {
                Ok(()) => coverage.mapped_lines += 1,
                Err(error) => {
                    invalid_assets.insert(format!("{}: {error}", requirement.line_key));
                    push_missing(&mut coverage, requirement);
                }
            },
            None => push_missing(&mut coverage, requirement),
        }
    }
    coverage.invalid_assets = invalid_assets.into_iter().collect();
    coverage
}

pub(crate) fn voice_repair_guidance(
    selector: &str,
    catalog: &PreparedDialogueCatalog,
    coverage: &DialogueVoiceCoverage,
) -> String {
    let mut command = format!("cargo run-dev -- prepare {selector}");
    if coverage.missing_authored.is_empty() {
        return format!("next command: {command}");
    }

    let missing_keys = coverage
        .missing_authored
        .iter()
        .map(|missing| (missing.line_key.clone(), missing.speaker_form_id))
        .collect::<BTreeSet<_>>();
    let mut sources = catalog
        .voice_requirements
        .iter()
        .filter(|requirement| {
            requirement.origin == DialogueVoiceRequirementOrigin::Authored
                && missing_keys
                    .contains(&(requirement.line_key.clone(), requirement.speaker_form_id))
        })
        .map(|requirement| requirement.source_path.clone())
        .collect::<BTreeSet<_>>();
    if sources.is_empty() {
        sources.extend(
            catalog
                .source_paths
                .iter()
                .filter(|path| path.starts_with("authored/"))
                .cloned(),
        );
    }
    let display_sources = sources
        .iter()
        .map(|source| source.strip_prefix("dialogue/").unwrap_or(source))
        .map(|source| format!("dialogue/{source}"))
        .collect::<Vec<_>>();
    if catalog.authored_voice_manifest_paths.is_empty() {
        return format!(
            "blocker: exact authored voice mapping manifest missing for sources=[{}]; create the mapping contract, then rerun {command}",
            display_sources.join(", ")
        );
    }
    for source in display_sources {
        command.push_str(&format!(" --dialogue-source {source}"));
    }
    for manifest in &catalog.authored_voice_manifest_paths {
        command.push_str(&format!(" --dialogue-voice-manifest {manifest}"));
    }
    format!("next command: {command}")
}

fn push_missing(
    coverage: &mut DialogueVoiceCoverage,
    requirement: &PreparedDialogueVoiceRequirement,
) {
    let missing = MissingDialogueVoice {
        line_key: requirement.line_key.clone(),
        speaker_form_id: requirement.speaker_form_id,
    };
    match requirement.origin {
        DialogueVoiceRequirementOrigin::Authored => coverage.missing_authored.push(missing),
        DialogueVoiceRequirementOrigin::FalloutDiscovered => coverage.missing_fallout.push(missing),
    }
}

fn validate_prepared_voice_asset(asset_root: &Path, asset: &DialogueVoiceAsset) -> Result<()> {
    let relative = Path::new(&asset.asset_path);
    anyhow::ensure!(
        !relative.is_absolute()
            && !relative.components().any(|component| matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )),
        "asset path escapes the prepared root"
    );
    let extension = relative
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    anyhow::ensure!(
        matches!(extension.as_deref(), Some("wav" | "ogg")),
        "asset is not WAV or OGG"
    );
    let path = asset_root.join(relative);
    let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
    anyhow::ensure!(!bytes.is_empty(), "asset is empty");
    if let Some(expected) = asset.staged_fingerprint.as_deref() {
        let actual = format!("{:x}", Sha256::digest(&bytes));
        anyhow::ensure!(actual == expected, "staged fingerprint mismatch");
    }
    Ok(())
}

fn prepared_path(asset_root: &Path, relative: &str) -> std::path::PathBuf {
    asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))
}

#[cfg(test)]
#[path = "tests/coverage.rs"]
mod tests;
