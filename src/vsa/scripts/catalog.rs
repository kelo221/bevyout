//! Winning top-level SCPT catalog built during the shared content-index scan.

use std::collections::BTreeMap;

use anyhow::Result;

use crate::vsa::content_index::{ContentIndex, PluginSource};
use crate::vsa::openmw_esm4::parse_subrecords_with_offsets;
use crate::vsa::record_stream::{RecordEnvelope, RecordPayload, winners::WinningRecords};

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

#[derive(Debug, Default)]
pub(crate) struct ScriptCatalog {
    scripts: BTreeMap<ScriptAssetId, CatalogScript>,
    diagnostics: Vec<ScriptRecordDiagnostic>,
    content_fingerprint: String,
}

impl ScriptCatalog {
    pub(crate) fn build(sources: &[PluginSource<'_>]) -> Result<(ContentIndex, ScriptCatalog)> {
        let mut builder = ScriptCatalogBuilder::default();
        let index = ContentIndex::build_with(sources, |record| builder.observe(record));
        let index = index?;
        let catalog = builder.finish(index.fingerprint().to_string());
        Ok((index, catalog))
    }

    pub(crate) fn get(&self, id: ScriptAssetId) -> Option<&CatalogScript> {
        self.scripts.get(&id)
    }

    pub(crate) fn scripts(&self) -> impl Iterator<Item = &CatalogScript> {
        self.scripts.values()
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
    diagnostics: Vec<ScriptRecordDiagnostic>,
}

impl ScriptCatalogBuilder {
    fn observe(&mut self, envelope: RecordEnvelope<'_>) {
        if envelope.signature != "SCPT" {
            return;
        }
        if envelope.deleted {
            self.scripts.delete(envelope.form_id.0);
            return;
        }
        let decoded = match envelope.payload {
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
                    decode_script_record(envelope.form_id, envelope.source_plugin, &inputs, |raw| {
                        envelope.resolve_form_id(raw)
                    })
                }
                Err(error) => {
                    let message = error.to_string();
                    malformed_record(
                        envelope,
                        Some(RawScriptPayload {
                            data: payload.to_vec(),
                            error_offset: error.offset,
                        }),
                        error.signature,
                        Some(error.offset),
                        message,
                    )
                }
            },
            RecordPayload::Unavailable(error) => {
                malformed_record(envelope, None, None, None, error.into())
            }
        };
        self.diagnostics.extend(decoded.diagnostics.iter().cloned());
        self.scripts.upsert(
            envelope.form_id.0,
            envelope.source_plugin.to_string(),
            decoded,
        );
    }

    fn finish(self, content_fingerprint: String) -> ScriptCatalog {
        let scripts = self
            .scripts
            .into_iter()
            .map(|(_form_id, winner)| {
                let id = winner.value.record.id;
                let winning_plugin = winner
                    .provenance
                    .last()
                    .cloned()
                    .expect("winning script always has provenance");
                (
                    id,
                    CatalogScript {
                        record: winner.value.record,
                        winning_plugin,
                        provenance: winner.provenance,
                    },
                )
            })
            .collect();
        ScriptCatalog {
            scripts,
            diagnostics: self.diagnostics,
            content_fingerprint,
        }
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
