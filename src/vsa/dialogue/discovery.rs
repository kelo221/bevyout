//! Cell-scoped Fallout dialogue voice discovery.
//!
//! This module is preparation-only. It reads resolved plugin records and
//! voice archives, then returns Bevy-free generated dialogue sources plus
//! content-addressed WAV/OGG input for the normal dialogue bundle writer.

use anyhow::{Context, Result, bail};
use hound::WavReader;
use lewton::inside_ogg::OggStreamReader;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Cursor;
use std::path::Path;

use bevyout_core::dialogue::{
    DIALOGUE_VOICE_DEMAND_REVISION, DialogueVoiceDiagnostic, PreparedDialogueVoiceDemand,
    PreparedDialogueVoiceDemandReport,
};

use crate::vsa::audio_assets::{
    AudioArchive, AudioAssetOrigin, ResolvedAudioAsset, resolve_audio_asset,
};
use crate::vsa::dialogue::{
    DialogueSource, DialogueSourceKind, DialogueVoiceInput, DialogueVoiceInputEntry,
    FalloutDialogueRecord, PreparedDialogueActorBinding,
};
use crate::vsa::openmw_esm4::{
    ParsedPlugin, PluginSource, ReferenceKind, cstring, parse_subrecords,
    parse_subrecords_with_offsets, sub,
};

#[derive(Clone, Debug, Default)]
struct DialRecord {
    editor_id: Option<String>,
    display_text: Option<String>,
    quest_form_id: Option<u32>,
    top_level: bool,
}

#[derive(Clone, Debug, Default)]
struct QuestRecord {
    editor_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct InfoResponse {
    response_number: u8,
    sound_form_id: Option<u32>,
    text: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct InfoRecord {
    plugin: String,
    form_id: u32,
    topic_form_id: Option<u32>,
    quest_form_id: Option<u32>,
    speaker_form_id: Option<u32>,
    links: Vec<u32>,
    link_from: Vec<u32>,
    responses: Vec<InfoResponse>,
}

#[derive(Clone, Debug, Default)]
struct DialogueRecordSet {
    dials: HashMap<u32, DialRecord>,
    quests: HashMap<u32, QuestRecord>,
    voice_types: HashMap<u32, String>,
    infos: HashMap<u32, InfoRecord>,
    diagnostics: Vec<DialogueVoiceDiagnostic>,
}

#[derive(Clone, Debug, Default)]
struct VoiceDemand {
    actor_reference_form_id: u32,
    actor_base_form_id: u32,
    actor_editor_id: Option<String>,
    actor_display_name: Option<String>,
    voice_type_form_id: Option<u32>,
    voice_type_editor_id: Option<String>,
}

#[derive(Debug)]
pub(crate) struct DialogueVoiceDiscovery {
    pub(crate) generated_sources: Vec<DialogueSource>,
    pub(crate) actor_bindings: Vec<PreparedDialogueActorBinding>,
    pub(crate) voice_input: DialogueVoiceInput,
    pub(crate) demand_report: PreparedDialogueVoiceDemandReport,
}

pub(crate) fn discover_dialogue_voice(
    cell_form_id: u32,
    parsed: &ParsedPlugin,
    sources: &[PluginSource<'_>],
    data_root: &Path,
    archives: &[AudioArchive],
) -> Result<DialogueVoiceDiscovery> {
    let records = collect_dialogue_records(sources)?;
    let mut diagnostics = records.diagnostics.clone();
    let demands = collect_actor_demands(parsed, &records.voice_types, &mut diagnostics);
    let mut actor_records = BTreeMap::<u32, BTreeMap<String, Vec<FalloutDialogueRecord>>>::new();
    let mut entries = Vec::new();
    let mut demand_counts = HashMap::<u32, u32>::new();

    let mut info_ids = records.infos.keys().copied().collect::<Vec<_>>();
    info_ids.sort_unstable();
    for info_id in info_ids {
        let info = records
            .infos
            .get(&info_id)
            .expect("sorted INFO ids must remain indexed");
        let matching_demands = match info.speaker_form_id {
            Some(speaker_form_id) => demands
                .iter()
                .filter(|demand| demand.actor_base_form_id == speaker_form_id)
                .collect::<Vec<_>>(),
            // INFO.ANAM is optional in Fallout 3. When it is absent, the
            // exact voice-type path is the authoritative discriminator: try
            // only voice types belonging to actors present in this cell and
            // accept a demand only when its named asset resolves.
            None => demands.iter().collect::<Vec<_>>(),
        };
        if matching_demands.is_empty() {
            continue;
        }

        let topic_form_id = info.topic_form_id.or(info.quest_form_id);
        let topic = topic_form_id
            .and_then(|form_id| records.dials.get(&form_id))
            .and_then(|dial| dial.editor_id.clone())
            .or_else(|| topic_form_id.map(|form_id| format!("Topic_{form_id:08x}")))
            .unwrap_or_else(|| format!("Topic_{:08x}", info.form_id));

        for response in &info.responses {
            let voice_key = format!(
                "fallout:{}:{:08x}:{}",
                info.plugin.to_ascii_lowercase(),
                info.form_id,
                response.response_number
            );

            let dial = topic_form_id.and_then(|form_id| records.dials.get(&form_id));
            let quest_editor_id = info
                .quest_form_id
                .or_else(|| dial.and_then(|dial| dial.quest_form_id))
                .and_then(|form_id| records.quests.get(&form_id))
                .and_then(|quest| quest.editor_id.as_deref())
                .unwrap_or(topic.as_str());
            let topic_editor_id = dial
                .and_then(|dial| dial.editor_id.as_deref())
                .unwrap_or(topic.as_str());

            let linked_topics = info
                .links
                .iter()
                .map(|form_id| dialogue_topic_key(*form_id, &records))
                .collect::<Vec<_>>();
            let link_from_topics = info
                .link_from
                .iter()
                .map(|form_id| dialogue_topic_key(*form_id, &records))
                .collect::<Vec<_>>();
            let mut resolved_any = false;
            let mut resolution_errors = Vec::new();
            for demand in &matching_demands {
                let Some(voice_type) = demand.voice_type_editor_id.as_deref() else {
                    continue;
                };
                match resolve_voice_asset(
                    info,
                    response,
                    quest_editor_id,
                    topic_editor_id,
                    voice_type,
                    data_root,
                    archives,
                    parsed,
                ) {
                    Ok(Some(resolved)) => {
                        resolved_any = true;
                        let (source_path, source_origin, original_bytes, voice_bytes) = resolved;
                        actor_records
                            .entry(demand.actor_reference_form_id)
                            .or_default()
                            .entry(topic.clone())
                            .or_default()
                            .push(FalloutDialogueRecord {
                                plugin: info.plugin.clone(),
                                form_id: info.form_id,
                                signature: "INFO".into(),
                                editor_id: None,
                                speaker_form_id: info.speaker_form_id,
                                text: response.text.clone(),
                                topic_key: topic.clone(),
                                conditions: Vec::new(),
                                actions: Vec::new(),
                                links: linked_topics.clone(),
                                link_from: link_from_topics.clone(),
                                deleted: false,
                                overridden: false,
                                voice_key: Some(voice_key.clone()),
                            });
                        *demand_counts
                            .entry(demand.actor_reference_form_id)
                            .or_default() += 1;
                        entries.push(DialogueVoiceInputEntry {
                            line_key: voice_key.clone().into(),
                            source_path,
                            bytes: voice_bytes,
                            source_fingerprint: Some(hash_bytes(&original_bytes)),
                            source_origin: Some(source_origin),
                            speaker_form_id: Some(demand.actor_reference_form_id),
                            voice_type_form_id: demand.voice_type_form_id,
                        });
                    }
                    Ok(None) => {}
                    Err(error) => resolution_errors.push(error.to_string()),
                }
            }
            if !resolved_any {
                let voice_types = matching_demands
                    .iter()
                    .filter_map(|demand| demand.voice_type_editor_id.as_deref())
                    .collect::<Vec<_>>();
                let had_resolution_error = !resolution_errors.is_empty();
                let diagnostic_message = resolution_errors.into_iter().next().unwrap_or_else(|| {
                    if info.speaker_form_id.is_none() {
                        format!(
                            "INFO {:08x} has no ANAM speaker and no exact voice asset for present voice types [{}]",
                            info.form_id,
                            voice_types.join(", ")
                        )
                    } else {
                        format!(
                            "no voice asset found for {} voice type {}",
                            voice_key,
                            voice_types.first().copied().unwrap_or("unknown")
                        )
                    }
                });
                let diagnostic_code = if had_resolution_error {
                    "invalid_voice_asset"
                } else if info.speaker_form_id.is_none() {
                    "missing_speaker"
                } else {
                    "missing_voice_asset"
                };
                diagnostics.push(diagnostic(
                    diagnostic_code,
                    Some(voice_key.clone()),
                    diagnostic_message,
                ));
                // An explicit INFO speaker identifies a real required line even
                // when its exact source asset is absent. Keep that line in the
                // generated conversation so readiness reports a Fallout
                // discovery failure instead of making it disappear.
                if let Some(speaker) = info.speaker_form_id
                    && let Some(demand) = matching_demands
                        .iter()
                        .find(|demand| demand.actor_base_form_id == speaker)
                {
                    actor_records
                        .entry(demand.actor_reference_form_id)
                        .or_default()
                        .entry(topic.clone())
                        .or_default()
                        .push(FalloutDialogueRecord {
                            plugin: info.plugin.clone(),
                            form_id: info.form_id,
                            signature: "INFO".into(),
                            editor_id: None,
                            speaker_form_id: info.speaker_form_id,
                            text: response.text.clone(),
                            topic_key: topic.clone(),
                            conditions: Vec::new(),
                            actions: Vec::new(),
                            links: linked_topics.clone(),
                            link_from: link_from_topics.clone(),
                            deleted: false,
                            overridden: false,
                            voice_key: Some(voice_key.clone()),
                        });
                }
            }
        }
    }

    entries.sort_by(|left, right| {
        left.line_key
            .cmp(&right.line_key)
            .then_with(|| left.speaker_form_id.cmp(&right.speaker_form_id))
    });
    entries.dedup_by(|left, right| {
        left.line_key == right.line_key && left.speaker_form_id == right.speaker_form_id
    });

    let mut generated_sources = Vec::new();
    let mut actor_bindings = Vec::new();
    for demand in &demands {
        let Some(topics) = actor_records.remove(&demand.actor_reference_form_id) else {
            continue;
        };
        let (source, binding) = generate_actor_conversation(demand, topics, &records.dials)?;
        generated_sources.push(source);
        actor_bindings.push(binding);
    }
    let emitted_voice_keys = generated_sources
        .iter()
        .zip(&actor_bindings)
        .flat_map(|(source, binding)| {
            source.content.lines().filter_map(move |line| {
                line.trim()
                    .strip_prefix("// bo_line_key:")
                    .map(|key| (key.trim().to_owned(), binding.actor_reference_form_id))
            })
        })
        .collect::<HashSet<_>>();
    entries.retain(|entry| {
        entry.speaker_form_id.is_some_and(|speaker| {
            emitted_voice_keys.contains(&(entry.line_key.to_string(), speaker))
        })
    });
    demand_counts.clear();
    for entry in &entries {
        if let Some(speaker) = entry.speaker_form_id {
            *demand_counts.entry(speaker).or_default() += 1;
        }
    }

    let mut prepared_demands = demands
        .iter()
        .map(|demand| PreparedDialogueVoiceDemand {
            actor_reference_form_id: demand.actor_reference_form_id,
            actor_base_form_id: demand.actor_base_form_id,
            voice_type_form_id: demand.voice_type_form_id,
            voice_type_editor_id: demand.voice_type_editor_id.clone(),
            matched_line_count: demand_counts
                .get(&demand.actor_reference_form_id)
                .copied()
                .unwrap_or_default(),
        })
        .collect::<Vec<_>>();
    prepared_demands.sort_by_key(|demand| {
        (
            demand.actor_reference_form_id,
            demand.actor_base_form_id,
            demand.voice_type_form_id,
        )
    });

    let source_fingerprint = discovery_fingerprint(cell_form_id, sources);
    let demand_report = PreparedDialogueVoiceDemandReport {
        revision: DIALOGUE_VOICE_DEMAND_REVISION.into(),
        cell_form_id,
        source_fingerprint: source_fingerprint.clone(),
        demands: prepared_demands,
        diagnostics,
    };
    Ok(DialogueVoiceDiscovery {
        generated_sources,
        actor_bindings,
        voice_input: DialogueVoiceInput {
            manifest_path: format!("fallout-dialogue-discovery:{cell_form_id:08x}"),
            cell_form_id: Some(cell_form_id),
            entries,
        },
        demand_report,
    })
}

fn dialogue_topic_key(form_id: u32, records: &DialogueRecordSet) -> String {
    records
        .dials
        .get(&form_id)
        .and_then(|dial| dial.editor_id.clone())
        .unwrap_or_else(|| format!("Topic_{form_id:08x}"))
}

fn generate_actor_conversation(
    demand: &VoiceDemand,
    topics: BTreeMap<String, Vec<FalloutDialogueRecord>>,
    dials: &HashMap<u32, DialRecord>,
) -> Result<(DialogueSource, PreparedDialogueActorBinding)> {
    let root_topic = topics
        .keys()
        .filter(|topic| topic.eq_ignore_ascii_case("GREETING"))
        .min()
        .cloned()
        .or_else(|| topics.keys().next().cloned())
        .context("actor voice discovery produced no dialogue topics")?;
    let dialogue_key = format!("fallout_actor_{:08x}", demand.actor_reference_form_id);
    let source_path = format!(
        "dialogue/generated/actors/{:08x}.yarn",
        demand.actor_reference_form_id
    );
    let mut source = String::new();
    let mut pending = vec![root_topic.clone()];
    let mut visited = HashSet::new();

    while let Some(topic) = pending.pop() {
        if !visited.insert(topic.clone()) {
            continue;
        }
        let Some(topic_records) = topics.get(&topic) else {
            continue;
        };
        let selected = selected_info_records(topic_records);
        if selected.is_empty() {
            continue;
        }
        let node_name = actor_topic_node_name(&dialogue_key, &root_topic, &topic);
        source.push_str(&format!("title: {node_name}\nmode: imported\n---\n"));
        let mut selected = selected;
        selected.sort_by_key(|record| {
            (
                record.form_id,
                record
                    .voice_key
                    .as_deref()
                    .and_then(|key| key.rsplit(':').next())
                    .and_then(|value| value.parse::<u32>().ok())
                    .unwrap_or_default(),
            )
        });
        for record in &selected {
            let Some(voice_key) = record.voice_key.as_deref() else {
                continue;
            };
            source.push_str(&format!("// bo_line_key: {voice_key}\n"));
            source.push_str(&format!(
                "Speaker{:08x}: {}\n",
                demand.actor_reference_form_id,
                record
                    .text
                    .as_deref()
                    .unwrap_or("[missing localized string]")
            ));
        }

        let mut links = selected
            .iter()
            .flat_map(|record| record.links.iter().cloned())
            .filter(|linked| linked != &topic && topics.contains_key(linked))
            .collect::<Vec<_>>();
        links.extend(
            topics
                .iter()
                .filter(|(_candidate_topic, candidate_records)| {
                    candidate_records
                        .iter()
                        .any(|record| record.link_from.iter().any(|source| source == &topic))
                })
                .map(|(candidate_topic, _candidate_records)| candidate_topic.clone()),
        );
        if topic == root_topic {
            links.extend(dials.values().filter_map(|dial| {
                if dial.top_level {
                    dial.editor_id
                        .clone()
                        .filter(|candidate_topic| topics.contains_key(candidate_topic))
                } else {
                    None
                }
            }));
        }
        links.sort();
        links.dedup();
        links.retain(|linked| {
            linked != &topic
                && topics.contains_key(linked)
                && dials.values().any(|dial| {
                    dial.editor_id.as_deref() == Some(linked.as_str())
                        && dial.display_text.as_deref().is_some_and(|text| {
                            !text.trim().is_empty()
                                && dial.editor_id.as_deref() != Some(text.trim())
                        })
                })
        });
        for linked in &links {
            let destination = actor_topic_node_name(&dialogue_key, &root_topic, linked);
            let option_text = dials
                .values()
                .find(|dial| dial.editor_id.as_deref() == Some(linked.as_str()))
                .and_then(|dial| dial.display_text.as_deref())
                .expect("retained dialogue choice has non-empty DIAL FULL text");
            source.push_str(&format!("-> {option_text} -> {destination}\n"));
        }
        source.push_str("===\n\n");
        for linked in links.into_iter().rev() {
            pending.push(linked);
        }
    }

    Ok((
        DialogueSource {
            relative_path: source_path.clone(),
            kind: DialogueSourceKind::ImportedGenerated,
            content: source,
        },
        PreparedDialogueActorBinding {
            actor_reference_form_id: demand.actor_reference_form_id,
            actor_base_form_id: demand.actor_base_form_id,
            actor_editor_id: demand.actor_editor_id.clone(),
            actor_display_name: demand.actor_display_name.clone(),
            dialogue: dialogue_key.into(),
            source_path,
        },
    ))
}

fn selected_info_records(records: &[FalloutDialogueRecord]) -> Vec<&FalloutDialogueRecord> {
    let mut grouped = BTreeMap::<u32, Vec<&FalloutDialogueRecord>>::new();
    for record in records {
        grouped.entry(record.form_id).or_default().push(record);
    }
    grouped
        .into_iter()
        .min_by_key(|(form_id, _records)| *form_id)
        .map(|(_, records)| records)
        .unwrap_or_default()
}

fn actor_topic_node_name(dialogue_key: &str, root_topic: &str, topic: &str) -> String {
    if topic == root_topic {
        return dialogue_key.into();
    }
    let suffix = topic
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    format!("{dialogue_key}_{suffix}")
}

fn collect_dialogue_records(sources: &[PluginSource<'_>]) -> Result<DialogueRecordSet> {
    let mut result = DialogueRecordSet::default();
    crate::vsa::record_stream::walk_resolved_records(sources, |record| {
        let payload = match record.payload {
            crate::vsa::record_stream::RecordPayload::Decoded(payload) => payload,
            crate::vsa::record_stream::RecordPayload::Unavailable(_error) => {
                return;
            }
        };
        let parsed = match parse_subrecords(payload) {
            Ok(parsed) => parsed,
            Err(_) => return,
        };
        match record.signature {
            "DIAL" => {
                if record.deleted {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("deleted_record", &record));
                    result.dials.remove(&record.form_id.0);
                    return;
                }
                if result.dials.contains_key(&record.form_id.0) {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("overridden_record", &record));
                }
                let editor_id = sub(&parsed, "EDID").map(cstring);
                result.dials.insert(
                    record.form_id.0,
                    DialRecord {
                        editor_id,
                        display_text: sub(&parsed, "FULL").map(cstring),
                        quest_form_id: sub_form_id(&parsed, "QSTI", &record)
                            .or_else(|| sub_form_id(&parsed, "QSTR", &record)),
                        top_level: sub(&parsed, "DATA")
                            .and_then(|data| data.get(1))
                            .is_some_and(|flags| flags & 0x02 != 0),
                    },
                );
            }
            "QUST" => {
                if record.deleted {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("deleted_record", &record));
                    result.quests.remove(&record.form_id.0);
                    return;
                }
                if result.quests.contains_key(&record.form_id.0) {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("overridden_record", &record));
                }
                result.quests.insert(
                    record.form_id.0,
                    QuestRecord {
                        editor_id: sub(&parsed, "EDID").map(cstring),
                    },
                );
            }
            "VTYP" => {
                if record.deleted {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("deleted_record", &record));
                    result.voice_types.remove(&record.form_id.0);
                    return;
                }
                if let Some(editor_id) = sub(&parsed, "EDID").map(cstring) {
                    if result.voice_types.contains_key(&record.form_id.0) {
                        result
                            .diagnostics
                            .push(record_resolution_diagnostic("overridden_record", &record));
                    }
                    result.voice_types.insert(record.form_id.0, editor_id);
                }
            }
            "INFO" => {
                if record.deleted {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("deleted_record", &record));
                    result.infos.remove(&record.form_id.0);
                    return;
                }
                if result.infos.contains_key(&record.form_id.0) {
                    result
                        .diagnostics
                        .push(record_resolution_diagnostic("overridden_record", &record));
                }
                let responses = parse_info_responses(payload, &record);
                result.infos.insert(
                    record.form_id.0,
                    InfoRecord {
                        plugin: record.source_plugin.into(),
                        form_id: record.form_id.0,
                        topic_form_id: sub_form_id(&parsed, "TPIC", &record)
                            .or_else(|| sub_form_id(&parsed, "NAME", &record))
                            .or_else(|| record.topic_group_form_id.map(|form_id| form_id.0)),
                        quest_form_id: sub_form_id(&parsed, "QSTI", &record),
                        speaker_form_id: sub_form_id(&parsed, "ANAM", &record),
                        links: sub_form_ids(payload, "TCLT", &record),
                        link_from: sub_form_ids(payload, "TCLF", &record),
                        responses,
                    },
                );
            }
            _ => {}
        }
    })?;
    Ok(result)
}

fn record_resolution_diagnostic(
    code: &str,
    record: &crate::vsa::record_stream::RecordEnvelope<'_>,
) -> DialogueVoiceDiagnostic {
    DialogueVoiceDiagnostic {
        severity: "info".into(),
        code: code.into(),
        line_key: Some(
            format!(
                "fallout:{}:{:08x}",
                record.source_plugin.to_ascii_lowercase(),
                record.form_id.0
            )
            .into(),
        ),
        source_path: None,
        message: format!(
            "{} {} {:08x} from {} was {} by load-order resolution",
            record.signature,
            code.trim_end_matches("_record"),
            record.form_id.0,
            record.source_plugin,
            if code == "deleted_record" {
                "deleted"
            } else {
                "overridden"
            }
        ),
    }
}

fn sub_form_id(
    subs: &[crate::vsa::openmw_esm4::Subrecord],
    signature: &str,
    record: &crate::vsa::record_stream::RecordEnvelope<'_>,
) -> Option<u32> {
    let data = sub(subs, signature)?;
    let raw = u32::from_le_bytes(data.get(..4)?.try_into().ok()?);
    let resolved = record.resolve_form_id(raw).0;
    (resolved != 0).then_some(resolved)
}

fn sub_form_ids(
    payload: &[u8],
    signature: &str,
    record: &crate::vsa::record_stream::RecordEnvelope<'_>,
) -> Vec<u32> {
    parse_subrecords_with_offsets(payload)
        .unwrap_or_default()
        .iter()
        .filter(|subrecord| subrecord.signature == signature)
        .filter_map(|subrecord| {
            let raw = u32::from_le_bytes(subrecord.data.get(..4)?.try_into().ok()?);
            let resolved = record.resolve_form_id(raw).0;
            (resolved != 0).then_some(resolved)
        })
        .collect()
}

fn parse_info_responses(
    payload: &[u8],
    record: &crate::vsa::record_stream::RecordEnvelope<'_>,
) -> Vec<InfoResponse> {
    let Ok(subrecords) = parse_subrecords_with_offsets(payload) else {
        return Vec::new();
    };
    let mut responses = Vec::new();
    let mut current = None;
    for subrecord in subrecords {
        match subrecord.signature.as_str() {
            "TRDT" if subrecord.data.len() >= 20 => {
                let response_number = subrecord.data[12];
                let raw_sound = u32::from_le_bytes(
                    subrecord.data[16..20]
                        .try_into()
                        .expect("TRDT sound has four bytes"),
                );
                current = Some(InfoResponse {
                    response_number,
                    sound_form_id: (raw_sound != 0).then(|| record.resolve_form_id(raw_sound).0),
                    text: None,
                });
            }
            "NAM1" => {
                if let Some(response) = current.as_mut() {
                    response.text = Some(cstring(&subrecord.data));
                }
            }
            _ => {}
        }
        if subrecord.signature == "NAM1"
            && let Some(response) = current.take()
        {
            responses.push(response);
        }
    }
    if let Some(response) = current {
        responses.push(response);
    }
    responses
}

fn collect_actor_demands(
    parsed: &ParsedPlugin,
    voice_types: &HashMap<u32, String>,
    diagnostics: &mut Vec<DialogueVoiceDiagnostic>,
) -> Vec<VoiceDemand> {
    let mut demands = Vec::new();
    let mut seen = HashSet::new();
    for reference in parsed.references.iter().filter(|reference| {
        reference.initially_enabled
            && matches!(reference.kind, ReferenceKind::Npc | ReferenceKind::Creature)
    }) {
        let Some(base) = parsed.bases.get(&reference.base_form_id) else {
            diagnostics.push(diagnostic(
                "missing_actor_base",
                Some(format!("{:08x}", reference.form_id)),
                format!(
                    "actor reference {:08x} has unresolved base {:08x}",
                    reference.form_id, reference.base_form_id
                ),
            ));
            continue;
        };
        let Some(voice_type_form_id) = base.actor.as_ref().and_then(|actor| actor.voice_form_id)
        else {
            diagnostics.push(diagnostic(
                "missing_voice_type",
                Some(format!("{:08x}", reference.form_id)),
                format!(
                    "actor reference {:08x} has no VTCK voice type",
                    reference.form_id
                ),
            ));
            continue;
        };
        let voice_type_editor_id = voice_types.get(&voice_type_form_id).cloned();
        if voice_type_editor_id.is_none() {
            diagnostics.push(diagnostic(
                "unresolved_voice_type",
                Some(format!("{:08x}", reference.form_id)),
                format!(
                    "actor reference {:08x} references unresolved VTYP {:08x}",
                    reference.form_id, voice_type_form_id
                ),
            ));
            continue;
        }
        if seen.insert(reference.form_id) {
            demands.push(VoiceDemand {
                actor_reference_form_id: reference.form_id,
                actor_base_form_id: reference.base_form_id,
                actor_editor_id: base.editor_id.clone(),
                actor_display_name: base.name.clone(),
                voice_type_form_id: Some(voice_type_form_id),
                voice_type_editor_id,
            });
        }
    }
    demands
}

type PreparedVoiceBytes = (String, String, Vec<u8>, Vec<u8>);

#[allow(clippy::too_many_arguments)]
fn resolve_voice_asset(
    info: &InfoRecord,
    response: &InfoResponse,
    quest_editor_id: &str,
    topic_editor_id: &str,
    voice_type: &str,
    data_root: &Path,
    archives: &[AudioArchive],
    parsed: &ParsedPlugin,
) -> Result<Option<PreparedVoiceBytes>> {
    // Retail files use both the compact topic stem and the older
    // quest_topic stem. Try those exact INFO-derived names only; never scan
    // an archive by filename or response text.
    let filename_stems = [
        format!(
            "{}_{}_{:08x}_{}",
            quest_editor_id, topic_editor_id, info.form_id, response.response_number
        ),
        format!(
            "{}_{:08x}_{}",
            topic_editor_id, info.form_id, response.response_number
        ),
    ];
    let plugin = info.plugin.to_ascii_lowercase();
    for filename in filename_stems {
        for extension in ["wav", "ogg"] {
            let path = format!("sound/voice/{plugin}/{voice_type}/{filename}.{extension}");
            if let Some(asset) = resolve_audio_asset(data_root, archives, &path)? {
                let validated = validate_voice_asset(&asset)?;
                return Ok(Some((
                    asset.source_path,
                    asset_origin(&asset.origin),
                    asset.bytes,
                    validated,
                )));
            }
        }
    }

    let Some(sound_form_id) = response.sound_form_id else {
        return Ok(None);
    };
    let Some(sound) = parsed.sounds.get(&sound_form_id) else {
        return Ok(None);
    };
    let Some(path) = sound.file.as_deref() else {
        return Ok(None);
    };
    if !voice_path_matches_type(path, voice_type) {
        return Ok(None);
    }
    let Some(asset) = resolve_audio_asset(data_root, archives, path)? else {
        return Ok(None);
    };
    let validated = validate_voice_asset(&asset)?;
    Ok(Some((
        asset.source_path,
        asset_origin(&asset.origin),
        asset.bytes,
        validated,
    )))
}

fn voice_path_matches_type(path: &str, voice_type: &str) -> bool {
    let expected = voice_type.to_ascii_lowercase();
    crate::vsa::paths::normalize_asset_path(path)
        .split('/')
        .any(|segment| segment == expected)
}

fn asset_origin(origin: &AudioAssetOrigin) -> String {
    match origin {
        AudioAssetOrigin::Loose(path) => format!("loose:{}", path.display()),
        AudioAssetOrigin::Archive(path) => format!("archive:{}", path.display()),
    }
}

fn validate_voice_asset(asset: &ResolvedAudioAsset) -> Result<Vec<u8>> {
    let extension = asset
        .source_path
        .rsplit_once('.')
        .map(|(_, extension)| extension.to_ascii_lowercase())
        .unwrap_or_default();
    if extension == "wav" {
        let reader = WavReader::new(Cursor::new(asset.bytes.as_slice()))
            .with_context(|| format!("reading dialogue WAV {}", asset.source_path))?;
        if reader.spec().channels == 0 || reader.spec().sample_rate == 0 || reader.duration() == 0 {
            bail!(
                "dialogue WAV has no playable samples: {}",
                asset.source_path
            );
        }
        return Ok(asset.bytes.clone());
    }
    if extension != "ogg" {
        bail!("unsupported dialogue voice format: {}", asset.source_path);
    }
    let mut reader = OggStreamReader::new(Cursor::new(asset.bytes.as_slice()))
        .with_context(|| format!("reading dialogue OGG {}", asset.source_path))?;
    let sample_rate = reader.ident_hdr.audio_sample_rate;
    let channels = reader.ident_hdr.audio_channels;
    if sample_rate == 0 || channels == 0 {
        bail!(
            "dialogue OGG has invalid audio parameters: {}",
            asset.source_path
        );
    }
    let mut sample_count = 0_usize;
    while let Some(packet) = reader.read_dec_packet_itl()? {
        sample_count = sample_count.saturating_add(packet.len());
    }
    if sample_count == 0 {
        bail!("dialogue OGG has no decoded samples: {}", asset.source_path);
    }
    Ok(asset.bytes.clone())
}

fn diagnostic(code: &str, line_key: Option<String>, message: String) -> DialogueVoiceDiagnostic {
    DialogueVoiceDiagnostic {
        severity: "warning".into(),
        code: code.into(),
        line_key: line_key.map(Into::into),
        source_path: None,
        message,
    }
}

fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn discovery_fingerprint(cell_form_id: u32, sources: &[PluginSource<'_>]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(cell_form_id.to_le_bytes());
    for source in sources {
        hasher.update(source.name.as_bytes());
        hasher.update([0]);
        hasher.update(source.bytes);
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
#[path = "tests/discovery.rs"]
mod tests;
