use std::collections::{BTreeMap, BTreeSet, VecDeque};

use bevy::prelude::*;
use bevyout_core::dialogue::{DialogueError, DialogueErrorCode, NarrativeValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HostFunctionDescriptor {
    pub name: String,
    pub support: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HostCommand {
    RunAction { key: String, action: String },
    SetQuestStage { quest: u32, stage: i32 },
    AddItem { form_id: u32, count: i32 },
    RemoveItem { form_id: u32, count: i32 },
    SetEnabled { reference: u32, enabled: bool },
    EndDialogue,
}

#[derive(Debug, Resource, Default)]
pub(crate) struct DialogueHostState {
    pub item_counts: BTreeMap<u32, i32>,
    pub globals: BTreeMap<String, i64>,
    pub quest_stages: BTreeMap<u32, i32>,
    pub actor_values: BTreeMap<(u32, String), i32>,
    pub references_enabled: BTreeMap<u32, bool>,
    pub applied_actions: BTreeSet<String>,
    pub trace: Vec<String>,
}

#[derive(Debug, Resource, Default)]
pub(crate) struct YarnHostBridge {
    pub functions: BTreeMap<String, HostFunctionDescriptor>,
    pub commands: BTreeMap<String, HostFunctionDescriptor>,
    pub pending: VecDeque<HostCommand>,
    pub diagnostics: Vec<String>,
    pub completion_handles: BTreeSet<String>,
}

impl YarnHostBridge {
    pub(crate) fn install_bevyout_yarn_api(&mut self) -> Vec<String> {
        let names = [
            "bo_has_item",
            "bo_item_count",
            "bo_global",
            "bo_quest_stage",
            "bo_quest_complete",
            "bo_actor_value",
            "bo_skill_check",
            "bo_reference_enabled",
        ];
        for name in names {
            self.functions.insert(
                name.into(),
                HostFunctionDescriptor {
                    name: name.into(),
                    support: "pure".into(),
                },
            );
        }
        for name in [
            "bo_run_action",
            "bo_set_quest_stage",
            "bo_add_item",
            "bo_remove_item",
            "bo_enable",
            "bo_disable",
            "bo_end_dialogue",
        ] {
            self.commands.insert(
                name.into(),
                HostFunctionDescriptor {
                    name: name.into(),
                    support: "deferred".into(),
                },
            );
        }
        Vec::new()
    }

    pub(crate) fn item_count(&self, state: &DialogueHostState, form_id: u32) -> i32 {
        state.item_counts.get(&form_id).copied().unwrap_or_default()
    }

    pub(crate) fn global(&self, state: &DialogueHostState, key: &str) -> i64 {
        state.globals.get(key).copied().unwrap_or_default()
    }

    pub(crate) fn evaluate_function(
        &self,
        state: &DialogueHostState,
        name: &str,
        args: &[&str],
    ) -> Result<NarrativeValue, DialogueError> {
        let value = match name {
            "bo_has_item" => {
                let form_id = parse_u32(args, 0, name)?;
                NarrativeValue::Bool(self.item_count(state, form_id) > 0)
            }
            "bo_item_count" => {
                let form_id = parse_u32(args, 0, name)?;
                NarrativeValue::Number(i64::from(self.item_count(state, form_id)))
            }
            "bo_global" => {
                let key = args.first().copied().ok_or_else(|| unsupported(name))?;
                NarrativeValue::Number(self.global(state, key))
            }
            "bo_quest_stage" => {
                let quest = parse_u32(args, 0, name)?;
                NarrativeValue::Number(i64::from(
                    state.quest_stages.get(&quest).copied().unwrap_or_default(),
                ))
            }
            "bo_quest_complete" => {
                let quest = parse_u32(args, 0, name)?;
                NarrativeValue::Bool(
                    state.quest_stages.get(&quest).copied().unwrap_or_default() >= 100,
                )
            }
            "bo_actor_value" => {
                let actor = parse_u32(args, 0, name)?;
                let key = args.get(1).copied().ok_or_else(|| unsupported(name))?;
                NarrativeValue::Number(i64::from(
                    state
                        .actor_values
                        .get(&(actor, key.to_owned()))
                        .copied()
                        .unwrap_or_default(),
                ))
            }
            "bo_reference_enabled" => {
                let reference = parse_u32(args, 0, name)?;
                NarrativeValue::Bool(
                    state
                        .references_enabled
                        .get(&reference)
                        .copied()
                        .unwrap_or(true),
                )
            }
            _ => return Err(unsupported(name)),
        };
        Ok(value)
    }

    pub(crate) fn command_from_text(
        &self,
        command: &str,
        source_key: &str,
    ) -> Result<HostCommand, DialogueError> {
        let mut parts = command.split_whitespace();
        let name = parts.next().ok_or_else(|| unsupported("<empty>"))?;
        let args: Vec<_> = parts.collect();
        match name {
            "bo_run_action" => Ok(HostCommand::RunAction {
                key: source_key.into(),
                action: args.first().copied().unwrap_or_default().into(),
            }),
            "bo_set_quest_stage" => Ok(HostCommand::SetQuestStage {
                quest: parse_u32(&args, 0, name)?,
                stage: parse_i32(&args, 1, name)?,
            }),
            "bo_add_item" => Ok(HostCommand::AddItem {
                form_id: parse_u32(&args, 0, name)?,
                count: parse_i32(&args, 1, name)?,
            }),
            "bo_remove_item" => Ok(HostCommand::RemoveItem {
                form_id: parse_u32(&args, 0, name)?,
                count: parse_i32(&args, 1, name)?,
            }),
            "bo_enable" | "bo_disable" => Ok(HostCommand::SetEnabled {
                reference: parse_u32(&args, 0, name)?,
                enabled: name == "bo_enable",
            }),
            "bo_end_dialogue" => Ok(HostCommand::EndDialogue),
            _ => Err(unsupported(name)),
        }
    }

    pub(crate) fn enqueue(&mut self, command: HostCommand) {
        self.pending.push_back(command);
    }

    pub(crate) fn enqueue_async(
        &mut self,
        command: HostCommand,
        handle: impl Into<String>,
    ) -> String {
        let handle = handle.into();
        self.completion_handles.insert(handle.clone());
        self.enqueue(command);
        handle
    }

    pub(crate) fn complete_async(&mut self, handle: &str) -> bool {
        self.completion_handles.remove(handle)
    }
}

fn unsupported(name: &str) -> DialogueError {
    DialogueError::new(
        DialogueErrorCode::UnsupportedHostApi,
        format!("unsupported Yarn host API {name}"),
    )
}

fn parse_u32(args: &[&str], index: usize, name: &str) -> Result<u32, DialogueError> {
    args.get(index)
        .ok_or_else(|| unsupported(name))?
        .parse()
        .map_err(|_| unsupported(name))
}

fn parse_i32(args: &[&str], index: usize, name: &str) -> Result<i32, DialogueError> {
    args.get(index)
        .ok_or_else(|| unsupported(name))?
        .parse()
        .map_err(|_| unsupported(name))
}

pub(crate) fn apply_host_commands(
    mut bridge: ResMut<YarnHostBridge>,
    mut state: ResMut<DialogueHostState>,
) {
    while let Some(command) = bridge.pending.pop_front() {
        match command {
            HostCommand::RunAction { key, action } => {
                if state.applied_actions.insert(key.clone()) {
                    state.trace.push(format!("action:{key}:{action}"));
                }
            }
            HostCommand::SetQuestStage { quest, stage } => {
                state.quest_stages.insert(quest, stage);
                state.trace.push(format!("quest:{quest}:{stage}"));
            }
            HostCommand::AddItem { form_id, count } => {
                *state.item_counts.entry(form_id).or_default() += count;
                state.trace.push(format!("add:{form_id:08x}:{count}"));
            }
            HostCommand::RemoveItem { form_id, count } => {
                *state.item_counts.entry(form_id).or_default() -= count;
                state.trace.push(format!("remove:{form_id:08x}:{count}"));
            }
            HostCommand::SetEnabled { reference, enabled } => {
                state.references_enabled.insert(reference, enabled);
                state
                    .trace
                    .push(format!("enabled:{reference:08x}:{enabled}"));
            }
            HostCommand::EndDialogue => state.trace.push("end_dialogue".into()),
        }
    }
}
