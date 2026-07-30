//! Winning top-level SCPT catalog built during the shared content-index scan.

use std::collections::BTreeMap;

use anyhow::Result;

use crate::vsa::content_index::{ContentIndex, PluginSource};
use crate::vsa::openmw_esm4::parse_subrecords_with_offsets;
use crate::vsa::record_stream::{RecordEnvelope, RecordPayload, winners::WinningRecords};

use super::attachments::{ExtractedOwnerScripts, ScriptAttachmentSlot, extract_owner_scripts};
use super::record::{
    DecodedScriptRecord, RawScriptPayload, ScriptAssetId, ScriptRecord, ScriptRecordDiagnostic,
    ScriptSubrecordInput, decode_script_record,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogScript {
    pub(crate) record: ScriptRecord,
    pub(crate) winning_plugin: String,
    pub(crate) provenance: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptAttachment {
    pub(crate) owner: bevyout_core::form_id::FormId,
    pub(crate) owner_signature: String,
    pub(crate) slot: ScriptAttachmentSlot,
    pub(crate) script: ScriptAssetId,
    pub(crate) source_offset: usize,
    pub(crate) winning_plugin: String,
    pub(crate) provenance: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct ScriptCatalog {
    scripts: BTreeMap<ScriptAssetId, CatalogScript>,
    attachments: BTreeMap<(bevyout_core::form_id::FormId, ScriptAttachmentSlot), ScriptAttachment>,
    diagnostics: Vec<ScriptRecordDiagnostic>,
    content_fingerprint: String,
}

impl ScriptCatalog {
    pub(crate) fn build(sources: &[PluginSource<'_>]) -> Result<(ContentIndex, ScriptCatalog)> {
        let mut builder = ScriptCatalogBuilder::default();
        let index = ContentIndex::build_with(sources, |record| builder.observe(record));
        let index = index?;
        let catalog = builder.finish(&index);
        Ok((index, catalog))
    }

    pub(crate) fn get(&self, id: ScriptAssetId) -> Option<&CatalogScript> {
        self.scripts.get(&id)
    }

    pub(crate) fn scripts(&self) -> impl Iterator<Item = &CatalogScript> {
        self.scripts.values()
    }

    pub(crate) fn attachments(&self) -> impl Iterator<Item = &ScriptAttachment> {
        self.attachments.values()
    }

    pub(crate) fn diagnostics(&self) -> &[ScriptRecordDiagnostic] {
        &self.diagnostics
    }

    pub(crate) fn content_fingerprint(&self) -> &str {
        &self.content_fingerprint
    }
}

#[derive(Debug, Default)]
struct ScriptCatalogBuilder {
    scripts: WinningRecords<DecodedScriptRecord>,
    owners: WinningRecords<ExtractedOwnerScripts>,
}

impl ScriptCatalogBuilder {
    fn observe(&mut self, envelope: RecordEnvelope<'_>) {
        if envelope.deleted {
            if envelope.signature == "SCPT" {
                self.scripts.delete(envelope.form_id.0);
            }
            self.owners.delete(envelope.form_id.0);
            return;
        }

        match envelope.payload {
            RecordPayload::Decoded(payload) => match parse_subrecords_with_offsets(payload) {
                Ok(subrecords) => {
                    let inputs = subrecords
                        .iter()
                        .map(|subrecord| ScriptSubrecordInput {
                            signature: &subrecord.signature,
                            data: &subrecord.data,
                            offset: subrecord.offset,
                        })
                        .collect::<Vec<_>>();
                    if envelope.signature == "SCPT" {
                        let decoded = decode_script_record(
                            envelope.form_id,
                            envelope.source_plugin,
                            &inputs,
                            |raw| envelope.resolve_form_id(raw),
                        );
                        self.scripts.upsert(
                            envelope.form_id.0,
                            envelope.source_plugin.to_string(),
                            decoded,
                        );
                    }
                    let extracted = extract_owner_scripts(
                        envelope.form_id,
                        envelope.signature,
                        envelope.source_plugin,
                        &inputs,
                        |raw| envelope.resolve_form_id(raw),
                    );
                    if !extracted.attachments.is_empty()
                        || !extracted.embedded.is_empty()
                        || !extracted.diagnostics.is_empty()
                        || self.owners.get(envelope.form_id.0).is_some()
                    {
                        self.owners.upsert(
                            envelope.form_id.0,
                            envelope.source_plugin.to_string(),
                            extracted,
                        );
                    }
                }
                Err(error) => self.observe_malformed(envelope, Some(payload), error),
            },
            RecordPayload::Unavailable(error) => {
                if envelope.signature == "SCPT" {
                    let decoded = malformed_record(envelope, None, None, None, error.into());
                    self.scripts.upsert(
                        envelope.form_id.0,
                        envelope.source_plugin.to_string(),
                        decoded,
                    );
                }
                if self.owners.get(envelope.form_id.0).is_some() {
                    self.owners.upsert(
                        envelope.form_id.0,
                        envelope.source_plugin.to_string(),
                        ExtractedOwnerScripts::default(),
                    );
                }
            }
        }
    }

    fn observe_malformed(
        &mut self,
        envelope: RecordEnvelope<'_>,
        payload: Option<&[u8]>,
        error: crate::vsa::openmw_esm4::SubrecordParseError,
    ) {
        if envelope.signature == "SCPT" {
            let decoded = malformed_record(
                envelope,
                payload.map(|payload| RawScriptPayload {
                    data: payload.to_vec(),
                    error_offset: error.offset,
                }),
                error.signature.clone(),
                Some(error.offset),
                error.to_string(),
            );
            self.scripts.upsert(
                envelope.form_id.0,
                envelope.source_plugin.to_string(),
                decoded,
            );
        }
        let mut owner = ExtractedOwnerScripts::default();
        if envelope.signature == "PACK" {
            let message = error.to_string();
            owner.diagnostics.push(ScriptRecordDiagnostic {
                script: ScriptAssetId::Record(envelope.form_id),
                source_plugin: envelope.source_plugin.into(),
                subrecord: error.signature,
                offset: Some(error.offset),
                message,
            });
        }
        // Mirror the decoded-payload branch: only track owners that carry
        // script data or already exist, so a malformed unrelated record does
        // not falsely register (and later get refreshed) as a script owner.
        if !owner.diagnostics.is_empty() || self.owners.get(envelope.form_id.0).is_some() {
            self.owners.upsert(
                envelope.form_id.0,
                envelope.source_plugin.to_string(),
                owner,
            );
        }
    }

    fn finish(self, index: &ContentIndex) -> ScriptCatalog {
        let mut diagnostics = Vec::new();
        let mut script_winners = self.scripts.into_iter().collect::<Vec<_>>();
        script_winners.sort_by_key(|(form_id, _winner)| *form_id);
        let mut scripts = BTreeMap::new();
        for (_form_id, winner) in script_winners {
            diagnostics.extend(winner.value.diagnostics.iter().cloned());
            let id = winner.value.record.id;
            let winning_plugin = winner
                .provenance
                .last()
                .cloned()
                .expect("winning script always has provenance");
            scripts.insert(
                id,
                CatalogScript {
                    record: winner.value.record,
                    winning_plugin,
                    provenance: winner.provenance,
                },
            );
        }
        let mut owner_winners = self.owners.into_iter().collect::<Vec<_>>();
        owner_winners.sort_by_key(|(form_id, _winner)| *form_id);
        let mut attachments = BTreeMap::new();
        for (owner_id, winner) in owner_winners {
            diagnostics.extend(winner.value.diagnostics);
            if winner.value.attachments.is_empty() && winner.value.embedded.is_empty() {
                continue;
            }
            let owner_record = index
                .get(bevyout_core::form_id::FormId(owner_id))
                .expect("winning script owner remains in the content index");
            let winning_plugin = owner_record.winning_plugin.clone();
            let provenance = owner_record.provenance.clone();
            for embedded in winner.value.embedded {
                scripts.insert(
                    embedded.record.id,
                    CatalogScript {
                        record: embedded.record,
                        winning_plugin: winning_plugin.clone(),
                        provenance: provenance.clone(),
                    },
                );
            }
            for attachment in winner.value.attachments {
                attachments.insert(
                    (attachment.owner, attachment.slot),
                    ScriptAttachment {
                        owner: attachment.owner,
                        owner_signature: attachment.owner_signature,
                        slot: attachment.slot,
                        script: attachment.script,
                        source_offset: attachment.source_offset,
                        winning_plugin: winning_plugin.clone(),
                        provenance: provenance.clone(),
                    },
                );
            }
        }
        for attachment in attachments.values() {
            if !scripts.contains_key(&attachment.script) {
                diagnostics.push(ScriptRecordDiagnostic {
                    script: attachment.script,
                    source_plugin: attachment.winning_plugin.clone(),
                    subrecord: Some("SCRI".into()),
                    offset: Some(attachment.source_offset),
                    message: format!(
                        "{} {} attachment target {} is missing from the winning script catalog",
                        attachment.owner_signature,
                        attachment.owner,
                        display_script_id(attachment.script)
                    ),
                });
            }
        }
        ScriptCatalog {
            scripts,
            attachments,
            diagnostics,
            content_fingerprint: index.fingerprint().to_string(),
        }
    }
}

fn display_script_id(id: ScriptAssetId) -> String {
    match id {
        ScriptAssetId::Record(form_id) => form_id.to_string(),
        ScriptAssetId::Embedded { owner, slot } => format!("{owner}/{slot:?}"),
    }
}

fn malformed_record(
    envelope: RecordEnvelope<'_>,
    unparsed_payload: Option<RawScriptPayload>,
    subrecord: Option<String>,
    offset: Option<usize>,
    message: String,
) -> DecodedScriptRecord {
    let mut decoded = decode_script_record(envelope.form_id, envelope.source_plugin, &[], |raw| {
        envelope.resolve_form_id(raw)
    });
    decoded.record.unparsed_payload = unparsed_payload;
    decoded.diagnostics.push(ScriptRecordDiagnostic {
        script: ScriptAssetId::Record(envelope.form_id),
        source_plugin: envelope.source_plugin.into(),
        subrecord,
        offset,
        message,
    });
    decoded
}

#[cfg(test)]
#[path = "tests/catalog.rs"]
mod tests;
