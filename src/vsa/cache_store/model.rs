use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

use super::policy::recipe_identity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum PreparedObjectKind {
    Glb,
    Texture,
    Physics,
    Audio,
    Shadow,
    Bake,
    Catalog,
    Navigation,
    Other,
}

impl PreparedObjectKind {
    pub(crate) const fn tag(self) -> &'static str {
        match self {
            Self::Glb => "glb",
            Self::Texture => "texture",
            Self::Physics => "physics",
            Self::Audio => "audio",
            Self::Shadow => "shadow",
            Self::Bake => "bake",
            Self::Catalog => "catalog",
            Self::Navigation => "navigation",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PreparedObjectRef {
    pub(crate) kind: PreparedObjectKind,
    pub(crate) sha256: String,
    pub(crate) byte_len: u64,
    pub(crate) extension: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PreparedRecipeInputs {
    pub(crate) recipe_version: u32,
    pub(crate) kind: PreparedObjectKind,
    pub(crate) source_identity: String,
    pub(crate) input_hashes: Vec<String>,
    pub(crate) converter_revision: String,
    pub(crate) format_policy_revision: String,
    pub(crate) canonical_settings: Vec<u8>,
}

impl PreparedRecipeInputs {
    pub(crate) fn try_id(&self) -> Result<String> {
        Ok(recipe_identity(
            self.kind.tag(),
            self.recipe_version,
            &self.source_identity,
            &self.input_hashes,
            &self.converter_revision,
            &self.format_policy_revision,
            &self.canonical_settings,
        )?)
    }

    #[cfg(test)]
    pub(crate) fn id(&self) -> String {
        self.try_id()
            .expect("prepared recipe inputs must be canonical")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PreparedRecipeRecord {
    pub(crate) recipe: PreparedRecipeInputs,
    pub(crate) output: PreparedObjectRef,
}

#[derive(Debug, Clone)]
pub(super) enum CandidatePayload {
    #[allow(dead_code)]
    Path(PathBuf),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub(crate) struct CandidateObject {
    pub(crate) kind: PreparedObjectKind,
    pub(crate) extension: String,
    pub(super) payload: CandidatePayload,
}

impl CandidateObject {
    #[allow(dead_code)]
    pub(crate) fn new(
        kind: PreparedObjectKind,
        extension: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            kind,
            extension: extension.into(),
            payload: CandidatePayload::Path(path.into()),
        }
    }

    pub(crate) fn from_bytes(
        kind: PreparedObjectKind,
        extension: impl Into<String>,
        bytes: Vec<u8>,
    ) -> Self {
        Self {
            kind,
            extension: extension.into(),
            payload: CandidatePayload::Bytes(bytes),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Verification {
    pub(crate) valid: bool,
    pub(crate) actual_sha256: String,
    pub(crate) actual_byte_len: u64,
}

#[allow(dead_code)]
pub(crate) trait PreparedObjectStore {
    fn resolve_recipe(&self, recipe_id: &str) -> Result<Option<PreparedObjectRef>>;
    fn publish(
        &self,
        recipe: &PreparedRecipeInputs,
        candidate: CandidateObject,
    ) -> Result<PreparedObjectRef>;
    fn open(&self, object: &PreparedObjectRef) -> Result<File>;
    fn verify(&self, object: &PreparedObjectRef) -> Result<Verification>;
}
