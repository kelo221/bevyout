//! Prepared dialogue content and compatibility reports.
//!
//! This module owns filesystem preparation and imported Fallout normalization;
//! it deliberately exposes only serialized Bevy-free data to the viewer.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use bevyout_core::dialogue::{
    DIALOGUE_BUNDLE_REVISION, DialogueActionKey, DialogueChoiceId, DialogueKey, DialogueLineKey,
    PreparedDialogueBundleRef,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const PREPARED_DIALOGUE_CATALOG_REVISION: &str = "dialogue-catalog-v2";
pub const GENERATED_DIALOGUE_REVISION: &str = "dialogue-yarn-generated-v1";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DialogueSourceKind {
    #[default]
    Authored,
    ImportedGenerated,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueSource {
    pub relative_path: String,
    pub kind: DialogueSourceKind,
    pub content: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueDiagnostic {
    pub severity: String,
    pub code: String,
    pub source_path: Option<String>,
    pub line: Option<u32>,
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedDialogueLine {
    pub key: DialogueLineKey,
    pub speaker: Option<String>,
    pub text: String,
    pub source_path: String,
    pub source_line: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedDialogueOption {
    pub choice: DialogueChoiceId,
    pub text: String,
    pub destination: Option<String>,
    pub condition_set_key: Option<String>,
    pub action_set_key: Option<String>,
    pub source_path: String,
    pub source_line: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedDialogueNode {
    pub name: String,
    pub source_path: String,
    pub source_line: u32,
    pub lines: Vec<PreparedDialogueLine>,
    pub options: Vec<PreparedDialogueOption>,
    pub commands: Vec<DialogueActionKey>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedConversation {
    pub key: DialogueKey,
    pub mode: DialogueSourceKind,
    pub speaker_form_id: Option<u32>,
    pub source_paths: Vec<String>,
    pub nodes: BTreeMap<String, PreparedDialogueNode>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialogueSourceMapping {
    pub key: String,
    pub source_path: String,
    pub source_line: u32,
    pub signature: Option<String>,
    pub form_id: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreparedDialogueCatalog {
    pub revision: String,
    pub source_fingerprint: String,
    #[serde(default)]
    pub source_paths: Vec<String>,
    pub conversations: BTreeMap<DialogueKey, PreparedConversation>,
    pub condition_set_keys: BTreeSet<String>,
    pub action_set_keys: BTreeSet<String>,
    pub line_keys: BTreeSet<DialogueLineKey>,
    pub source_mappings: Vec<DialogueSourceMapping>,
    pub diagnostics: Vec<DialogueDiagnostic>,
}

impl PreparedDialogueCatalog {
    pub fn is_ready(&self) -> bool {
        !self.conversations.is_empty()
            && !self
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.severity == "error")
    }

    pub fn bundle_hash(&self) -> String {
        fingerprint(self)
    }

    pub fn conversation(&self, key: &DialogueKey) -> Option<&PreparedConversation> {
        self.conversations.get(key)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DialoguePreparationOutput {
    pub catalog: PreparedDialogueCatalog,
    pub bundle: Option<PreparedDialogueBundleRef>,
}

/// Parses explicit sources in path order. Authored and generated inputs are
/// never merged into one mutable file; they remain separate source entries.
pub fn prepare_catalog(mut sources: Vec<DialogueSource>) -> PreparedDialogueCatalog {
    sources.sort_by(|left, right| {
        left.relative_path
            .cmp(&right.relative_path)
            .then_with(|| source_kind_rank(&left.kind).cmp(&source_kind_rank(&right.kind)))
    });

    let mut catalog = PreparedDialogueCatalog {
        revision: PREPARED_DIALOGUE_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint(&sources),
        source_paths: sources
            .iter()
            .map(|source| source.relative_path.clone())
            .collect(),
        ..Default::default()
    };

    for source in &sources {
        parse_source(source, &mut catalog);
    }

    catalog.source_mappings.sort_by(|left, right| {
        left.key
            .cmp(&right.key)
            .then_with(|| left.source_path.cmp(&right.source_path))
            .then_with(|| left.source_line.cmp(&right.source_line))
    });
    catalog
}

/// Reads only the explicit source list, validates containment, and writes a
/// deterministic catalog and node index below `asset_root`.
pub fn prepare_dialogue_bundle(
    asset_root: impl AsRef<Path>,
    sources: Vec<DialogueSource>,
) -> Result<DialoguePreparationOutput> {
    let asset_root = asset_root.as_ref();
    fs::create_dir_all(asset_root).with_context(|| format!("creating {}", asset_root.display()))?;
    let mut normalized = Vec::with_capacity(sources.len());
    let mut source_paths = BTreeSet::new();
    for source in sources {
        let path = Path::new(&source.relative_path);
        if path.is_absolute()
            || path
                .components()
                .any(|component| component == std::path::Component::ParentDir)
        {
            bail!(
                "dialogue source path escapes prepared asset root: {}",
                source.relative_path
            );
        }
        if !source_paths.insert(source.relative_path.clone()) {
            bail!(
                "dialogue source path is listed more than once: {}",
                source.relative_path
            );
        }
        normalized.push(source);
    }

    let catalog = prepare_catalog(normalized.clone());
    let dialogue_dir = asset_root.join("dialogue");
    let authored_dir = dialogue_dir.join("authored");
    let generated_dir = dialogue_dir.join("generated");
    fs::create_dir_all(&authored_dir)?;
    fs::create_dir_all(&generated_dir)?;

    for source in &normalized {
        let relative = source
            .relative_path
            .strip_prefix("dialogue/")
            .unwrap_or(&source.relative_path);
        let source_path = dialogue_dir.join(relative);
        if source_path.parent().is_none() {
            bail!(
                "dialogue source has no parent directory: {}",
                source.relative_path
            );
        }
        fs::create_dir_all(source_path.parent().expect("checked parent"))?;
        fs::write(&source_path, source.content.as_bytes())?;
    }

    let catalog_path = dialogue_dir.join("catalog.ron");
    let node_index_path = dialogue_dir.join("node_index.ron");
    fs::write(
        &catalog_path,
        ron::ser::to_string_pretty(&catalog, ron::ser::PrettyConfig::default())?,
    )?;

    let mut index = Vec::new();
    for conversation in catalog.conversations.values() {
        for node in conversation.nodes.values() {
            index.push(bevyout_core::dialogue::DialogueNodeIndexEntry {
                node: node.name.clone(),
                source_path: node.source_path.clone(),
                source_line: node.source_line,
                source_key: conversation.key.to_string(),
            });
        }
    }
    index.sort_by(|left, right| {
        left.node
            .cmp(&right.node)
            .then_with(|| left.source_path.cmp(&right.source_path))
    });
    fs::write(
        &node_index_path,
        ron::ser::to_string_pretty(&index, ron::ser::PrettyConfig::default())?,
    )?;

    let relative = |path: &Path| -> String {
        path.strip_prefix(asset_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    };
    let bundle = PreparedDialogueBundleRef {
        revision: DIALOGUE_BUNDLE_REVISION.into(),
        catalog_path: relative(&catalog_path),
        source_paths: catalog.source_paths.clone(),
        node_index_path: relative(&node_index_path),
        voice_index_path: None,
        localization_index_path: None,
        content_fingerprint: catalog.bundle_hash(),
    };
    Ok(DialoguePreparationOutput {
        catalog,
        bundle: Some(bundle),
    })
}

fn parse_source(source: &DialogueSource, catalog: &mut PreparedDialogueCatalog) {
    let lines: Vec<&str> = source.content.lines().collect();
    let mut index = 0usize;
    let mut parsed_nodes = Vec::new();
    while index < lines.len() {
        let trimmed = lines[index].trim();
        if !trimmed.starts_with("title:") {
            index += 1;
            continue;
        }
        let title = trimmed[6..].trim();
        let title_line = index as u32 + 1;
        index += 1;
        let mut found_separator = false;
        while index < lines.len() && lines[index].trim() != "---" {
            index += 1;
        }
        if index < lines.len() {
            found_separator = true;
            index += 1;
        }
        if title.is_empty() || !found_separator {
            catalog.diagnostics.push(error(
                source,
                title_line,
                "malformed_node",
                "node needs a non-empty title and --- separator",
            ));
            continue;
        }

        let mut node = PreparedDialogueNode {
            name: title.into(),
            source_path: source.relative_path.clone(),
            source_line: title_line,
            ..Default::default()
        };
        let mut option_index = 0usize;
        while index < lines.len() {
            let line_number = index as u32 + 1;
            let line = lines[index].trim();
            if line == "===" {
                index += 1;
                break;
            }
            if line.is_empty() {
                index += 1;
                continue;
            }
            if let Some(command) = line
                .strip_prefix("<<")
                .and_then(|value| value.strip_suffix(">>"))
            {
                let command_key = DialogueActionKey::new(command.trim());
                node.commands.push(command_key.clone());
                catalog.action_set_keys.insert(command_key.to_string());
            } else if let Some(option) = line.strip_prefix("->") {
                let (text, destination) = parse_option(option.trim());
                let choice = DialogueChoiceId::new(format!("{}:{option_index}", title));
                option_index += 1;
                node.options.push(PreparedDialogueOption {
                    choice,
                    text,
                    destination,
                    condition_set_key: None,
                    action_set_key: None,
                    source_path: source.relative_path.clone(),
                    source_line: line_number,
                });
            } else {
                let (speaker, text) = parse_speaker_line(line);
                let line_key = DialogueLineKey::new(format!("{}:{}", title, node.lines.len()));
                catalog.line_keys.insert(line_key.clone());
                node.lines.push(PreparedDialogueLine {
                    key: line_key,
                    speaker,
                    text,
                    source_path: source.relative_path.clone(),
                    source_line: line_number,
                });
            }
            index += 1;
        }

        catalog.source_mappings.push(DialogueSourceMapping {
            key: title.into(),
            source_path: source.relative_path.clone(),
            source_line: title_line,
            signature: Some("NODE".into()),
            form_id: None,
        });
        parsed_nodes.push((title.to_owned(), node));
    }

    let Some((root_name, _)) = parsed_nodes.first() else {
        catalog.diagnostics.push(error(
            source,
            1,
            "missing_node",
            "source contains no Yarn nodes",
        ));
        return;
    };
    let root_key = DialogueKey::new(root_name);
    let mut conversation =
        catalog
            .conversations
            .remove(&root_key)
            .unwrap_or_else(|| PreparedConversation {
                key: root_key.clone(),
                mode: source.kind.clone(),
                speaker_form_id: None,
                source_paths: Vec::new(),
                nodes: BTreeMap::new(),
            });
    if !conversation.source_paths.contains(&source.relative_path) {
        conversation.source_paths.push(source.relative_path.clone());
        conversation.source_paths.sort();
    }
    for (name, node) in parsed_nodes {
        if conversation.nodes.insert(name.clone(), node).is_some() {
            catalog.diagnostics.push(error(
                source,
                1,
                "duplicate_node",
                format!("duplicate Yarn node {name}"),
            ));
        }
        if !catalog
            .source_mappings
            .iter()
            .any(|mapping| mapping.key == name && mapping.source_path == source.relative_path)
        {
            catalog.source_mappings.push(DialogueSourceMapping {
                key: name,
                source_path: source.relative_path.clone(),
                source_line: 0,
                signature: Some("NODE".into()),
                form_id: None,
            });
        }
    }
    catalog.conversations.insert(root_key, conversation.clone());
    for node_name in conversation.nodes.keys() {
        let alias_key = DialogueKey::new(node_name);
        catalog
            .conversations
            .entry(alias_key)
            .or_insert_with(|| conversation.clone());
    }
}

fn parse_option(value: &str) -> (String, Option<String>) {
    if let Some((text, destination)) = value.rsplit_once("->") {
        (text.trim().to_owned(), Some(destination.trim().to_owned()))
    } else {
        (value.trim().to_owned(), None)
    }
}

fn parse_speaker_line(value: &str) -> (Option<String>, String) {
    if let Some((speaker, text)) = value.split_once(':')
        && !speaker.trim().is_empty()
        && !speaker.contains(' ')
    {
        return (Some(speaker.trim().to_owned()), text.trim().to_owned());
    }
    (None, value.to_owned())
}

fn source_kind_rank(kind: &DialogueSourceKind) -> u8 {
    match kind {
        DialogueSourceKind::Authored => 0,
        DialogueSourceKind::ImportedGenerated => 1,
    }
}

fn source_fingerprint(sources: &[DialogueSource]) -> String {
    let mut hasher = Sha256::new();
    for source in sources {
        hasher.update(source.relative_path.as_bytes());
        hasher.update([0]);
        hasher.update([source_kind_rank(&source.kind)]);
        hasher.update([0]);
        hasher.update(source.content.as_bytes());
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}

fn fingerprint<T: Serialize>(value: &T) -> String {
    let bytes = ron::ser::to_string(value).unwrap_or_default();
    format!("{:x}", Sha256::digest(bytes.as_bytes()))
}

fn error(
    source: &DialogueSource,
    line: u32,
    code: &str,
    message: impl Into<String>,
) -> DialogueDiagnostic {
    DialogueDiagnostic {
        severity: "error".into(),
        code: code.into(),
        source_path: Some(source.relative_path.clone()),
        line: Some(line),
        message: message.into(),
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FalloutDialogueRecord {
    pub plugin: String,
    pub form_id: u32,
    pub signature: String,
    pub editor_id: Option<String>,
    pub speaker_form_id: Option<u32>,
    pub text: Option<String>,
    pub topic_key: String,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
    pub links: Vec<String>,
    pub deleted: bool,
    pub overridden: bool,
    pub voice_key: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FalloutDialogueInventory {
    pub topic_total: usize,
    pub dial_total: usize,
    pub info_total: usize,
    pub player_choice_total: usize,
    pub response_line_total: usize,
    pub condition_frequencies: BTreeMap<String, usize>,
    pub action_frequencies: BTreeMap<String, usize>,
    pub topic_links: BTreeMap<String, Vec<String>>,
    pub cycles: Vec<Vec<String>>,
    pub unreachable: Vec<String>,
    pub deleted: Vec<String>,
    pub overridden: Vec<String>,
    pub missing_speakers: Vec<String>,
    pub missing_localized_strings: Vec<String>,
    pub voice_coverage: BTreeMap<String, usize>,
    pub unsupported: Vec<DialogueSourceMapping>,
}

pub fn inventory_fallout_dialogue(records: &[FalloutDialogueRecord]) -> FalloutDialogueInventory {
    let mut report = FalloutDialogueInventory::default();
    let mut topics = BTreeSet::new();
    for record in records {
        let identity = format!(
            "{}:{:08x}:{}",
            record.plugin, record.form_id, record.signature
        );
        match record.signature.as_str() {
            "DIAL" => report.dial_total += 1,
            "INFO" => {
                report.info_total += 1;
                if record
                    .text
                    .as_deref()
                    .is_some_and(|text| text.starts_with("[choice]"))
                {
                    report.player_choice_total += 1;
                } else {
                    report.response_line_total += 1;
                }
            }
            _ => report.unsupported.push(DialogueSourceMapping {
                key: record.topic_key.clone(),
                source_path: record.plugin.clone(),
                source_line: 0,
                signature: Some(record.signature.clone()),
                form_id: Some(record.form_id),
            }),
        }
        if record.deleted {
            report.deleted.push(identity.clone());
        }
        if record.overridden {
            report.overridden.push(identity.clone());
        }
        if record.speaker_form_id.is_none() {
            report.missing_speakers.push(identity.clone());
        }
        if record.text.is_none() {
            report.missing_localized_strings.push(identity.clone());
        }
        let voice_bucket = if record.voice_key.is_some() {
            "voiced"
        } else {
            "missing"
        };
        *report
            .voice_coverage
            .entry(voice_bucket.into())
            .or_default() += 1;
        topics.insert(record.topic_key.clone());
        report
            .topic_links
            .entry(record.topic_key.clone())
            .or_default()
            .extend(record.links.clone());
        for condition in &record.conditions {
            *report
                .condition_frequencies
                .entry(condition.clone())
                .or_default() += 1;
        }
        for action in &record.actions {
            *report.action_frequencies.entry(action.clone()).or_default() += 1;
        }
    }
    for links in report.topic_links.values_mut() {
        links.sort();
        links.dedup();
    }
    report.topic_total = topics.len();
    let mut incoming = BTreeSet::new();
    for links in report.topic_links.values() {
        incoming.extend(links.iter().cloned());
    }
    let roots: Vec<String> = topics
        .iter()
        .filter(|topic| !incoming.contains(*topic))
        .cloned()
        .collect();
    let mut reachable = BTreeSet::new();
    let mut stack = if roots.is_empty() {
        topics.iter().cloned().collect()
    } else {
        roots
    };
    while let Some(topic) = stack.pop() {
        if !reachable.insert(topic.clone()) {
            continue;
        }
        if let Some(links) = report.topic_links.get(&topic) {
            stack.extend(links.iter().cloned());
        }
    }
    report.unreachable = topics.difference(&reachable).cloned().collect();
    for topic in &topics {
        let mut path = Vec::new();
        let mut current = topic.clone();
        let mut seen = BTreeSet::new();
        let mut closed_cycle = false;
        while seen.insert(current.clone()) {
            path.push(current.clone());
            let Some(next) = report
                .topic_links
                .get(&current)
                .and_then(|links| links.first())
            else {
                break;
            };
            current = next.clone();
        }
        if seen.contains(&current) && report.topic_links.contains_key(&current) {
            closed_cycle = true;
        }
        if closed_cycle {
            let mut cycle = path;
            if let Some(start) = cycle.iter().position(|entry| entry == &current) {
                cycle = cycle.split_off(start);
            }
            if !cycle.is_empty() {
                let canonical = canonical_cycle(cycle);
                if !report.cycles.contains(&canonical) {
                    report.cycles.push(canonical);
                }
            }
        }
    }
    report.cycles.sort();
    report
}

fn canonical_cycle(mut cycle: Vec<String>) -> Vec<String> {
    let mut best = cycle.clone();
    for _ in 1..cycle.len() {
        cycle.rotate_left(1);
        if cycle < best {
            best = cycle.clone();
        }
    }
    best
}

/// Converts the supported initial Fallout slice into generated Yarn while
/// preserving every record identity in a deterministic source index.
pub fn generate_fallout_conversation(
    topic_key: &str,
    records: &[FalloutDialogueRecord],
) -> Result<(DialogueSource, Vec<DialogueSourceMapping>)> {
    let mut selected: Vec<_> = records
        .iter()
        .filter(|record| record.topic_key == topic_key && !record.deleted)
        .collect();
    selected.sort_by_key(|record| record.form_id);
    if selected.is_empty() {
        bail!("no supported Fallout dialogue records for topic {topic_key}");
    }
    let mut source = format!("title: {topic_key}\nmode: imported\n---\n");
    let mut mappings = Vec::new();
    let mut choice_nodes = Vec::new();
    for record in selected {
        let line = record
            .text
            .clone()
            .unwrap_or_else(|| "[missing localized string]".into());
        if line.starts_with("[choice]") {
            let destination = format!("{}_{:08x}", topic_key, record.form_id);
            source.push_str(&format!(
                "-> {} -> {destination}\n",
                line.trim_start_matches("[choice]").trim()
            ));
            choice_nodes.push((destination, (*record).clone(), line));
        } else {
            source.push_str(&format!("Speaker{}: {}\n", record.form_id, line));
        }
        mappings.push(DialogueSourceMapping {
            key: format!("{topic_key}:{}", record.form_id),
            source_path: record.plugin.clone(),
            source_line: 0,
            signature: Some(record.signature.clone()),
            form_id: Some(record.form_id),
        });
        for action in &record.actions {
            source.push_str(&format!("<<bo_run_action {action}>>\n"));
        }
    }
    source.push_str("===\n");
    for (node_name, record, line) in choice_nodes {
        source.push_str(&format!(
            "\ntitle: {node_name}\nmode: imported\n---\nSpeaker{}: {}\n",
            record.form_id,
            line.trim_start_matches("[choice]").trim()
        ));
        for action in &record.actions {
            source.push_str(&format!("<<bo_run_action {action}>>\n"));
        }
        source.push_str("===\n");
    }
    Ok((
        DialogueSource {
            relative_path: format!("dialogue/generated/{topic_key}.yarn"),
            kind: DialogueSourceKind::ImportedGenerated,
            content: source,
        },
        mappings,
    ))
}

pub fn generated_source_fingerprint(source: &DialogueSource) -> String {
    let mut hasher = Sha256::new();
    hasher.update(GENERATED_DIALOGUE_REVISION.as_bytes());
    hasher.update([0]);
    hasher.update(source_fingerprint(std::slice::from_ref(source)).as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(feature = "dialogue-yarn")]
pub fn validate_with_yarn(sources: &[DialogueSource]) -> Vec<DialogueDiagnostic> {
    use yarnspinner::prelude::{YarnCompiler, YarnFile};
    let mut compiler = YarnCompiler::new();
    for source in sources {
        compiler.add_file(YarnFile {
            file_name: source.relative_path.clone(),
            source: source.content.clone(),
        });
    }
    match compiler.compile() {
        Ok(_) => Vec::new(),
        Err(error) => vec![DialogueDiagnostic {
            severity: "error".into(),
            code: "yarn_compile".into(),
            source_path: None,
            line: None,
            message: error.to_string(),
        }],
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
