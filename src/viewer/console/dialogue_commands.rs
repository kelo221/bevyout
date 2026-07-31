//! Agent-bridge and console controls for the active dialogue session.

use bevyout_core::dialogue::{DialogueKey, DialogueStartRequest, DialogueStartSource};

use super::*;

pub(super) struct DialogueCommandProvider;

impl ConsoleCommandProvider for DialogueCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(
            ConsoleCommand::new(
                "dialoguestart",
                "dialoguestart <dialogue>",
                "Start a prepared dialogue by its stable conversation key.",
                dialogue_start,
            )
            .mutating(),
        )?;
        registry.register(ConsoleCommand::new(
            "dialoguestate",
            "dialoguestate",
            "Inspect dialogue readiness, modal/input state, current line, and options.",
            dialogue_state,
        ))?;
        registry.register(
            ConsoleCommand::new(
                "dialoguecontinue",
                "dialoguecontinue",
                "Advance the currently presented dialogue line.",
                dialogue_continue,
            )
            .mutating(),
        )?;
        registry.register(
            ConsoleCommand::new(
                "dialoguechoice",
                "dialoguechoice <index>",
                "Select a visible dialogue option by its one-based index.",
                dialogue_choice,
            )
            .mutating(),
        )?;
        registry.register(
            ConsoleCommand::new(
                "dialoguereload",
                "dialoguereload <prepared-authored-source>...",
                "Reload explicit authored dialogue sources from the active prepared bundle.",
                dialogue_reload,
            )
            .mutating(),
        )?;
        Ok(())
    }
}

pub(super) fn dialogue_reload(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "dialoguereload requires at least one prepared authored source path",
        ));
    }
    world.write_message(crate::viewer::dialogue::DialogueHotReloadRequested {
        source_paths: invocation.args.clone(),
    });
    Ok(ConsoleCommandResult::new(
        json!({ "queued": true, "source_paths": invocation.args }),
        vec![format!(
            "Dialogue reload queued: {}.",
            invocation.args.join(", ")
        )],
    ))
}

pub(super) fn dialogue_start(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [dialogue] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "dialoguestart requires exactly one dialogue key",
        ));
    };
    if dialogue.trim().is_empty() {
        return Err(ConsoleError::new(
            "bad_value",
            "dialogue key cannot be empty",
        ));
    }
    world.write_message(crate::viewer::dialogue::DialogueStartRequested(
        DialogueStartRequest {
            dialogue: DialogueKey::new(dialogue),
            speaker: None,
            listener: None,
            source: DialogueStartSource::Agent,
        },
    ));
    Ok(ConsoleCommandResult::new(
        json!({ "queued": true, "dialogue": dialogue }),
        vec![format!("Dialogue start queued: {dialogue}.")],
    ))
}

pub(super) fn dialogue_state(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let runtime = world
        .get_resource::<crate::viewer::dialogue::DialogueRuntime>()
        .ok_or_else(|| ConsoleError::new("unavailable", "dialogue runtime is not installed"))?;
    let modal = world
        .get_resource::<State<GameplayModal>>()
        .map(|state| format!("{:?}", state.get()))
        .unwrap_or_else(|| "None".into());
    let options: Vec<Value> = runtime
        .presentation
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            json!({
                "index": index + 1,
                "choice": option.choice.to_string(),
                "text": option.text,
                "enabled": option.enabled,
            })
        })
        .collect();
    let line = runtime.presentation.line.as_ref().map(|line| {
        json!({
            "key": line.line_key.to_string(),
            "speaker": line.speaker.display_name,
            "text": line.text,
        })
    });
    let value = json!({
        "readiness": format!("{:?}", runtime.readiness),
        "phase": format!("{:?}", runtime.phase),
        "ui_phase": format!("{:?}", runtime.ui_phase),
        "modal": modal,
        "active": runtime.is_active(),
        "input_gated": runtime.input_gated,
        "camera_focused": runtime.camera_focused,
        "bundle_hash": runtime.bundle_hash,
        "voice_anchor": runtime.voice_anchor.label(),
        "voice_spatial": runtime.voice_anchor.is_spatial(),
        "voice_state": runtime.voice_timing.label(),
        "timing_source": runtime.voice_timing.timing_source(),
        "line": line,
        "options": options,
        "trace_tail": runtime.trace.iter().rev().take(8).cloned().collect::<Vec<_>>(),
        "diagnostic_count": runtime.diagnostics.len(),
    });
    Ok(ConsoleCommandResult::new(value, Vec::new()))
}

pub(super) fn dialogue_continue(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let active = world
        .get_resource::<crate::viewer::dialogue::DialogueRuntime>()
        .is_some_and(|runtime| runtime.is_active());
    if !active {
        return Err(ConsoleError::new(
            "not_active",
            "no dialogue session is active",
        ));
    }
    world.write_message(crate::viewer::dialogue::DialogueContinueRequested);
    Ok(ConsoleCommandResult::new(
        json!({ "queued": true }),
        vec!["Dialogue continuation queued.".into()],
    ))
}

pub(super) fn dialogue_choice(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw_index] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "dialoguechoice requires exactly one option index",
        ));
    };
    let index = raw_index
        .parse::<usize>()
        .map_err(|_| ConsoleError::new("bad_type", "dialogue option index must be a number"))?;
    if index == 0 {
        return Err(ConsoleError::new(
            "out_of_range",
            "dialogue option indexes start at 1",
        ));
    }
    let choice = world
        .get_resource::<crate::viewer::dialogue::DialogueRuntime>()
        .and_then(|runtime| runtime.presentation.options.get(index - 1))
        .filter(|option| option.enabled)
        .map(|option| option.choice.clone())
        .ok_or_else(|| {
            ConsoleError::new(
                "out_of_range",
                format!("no enabled dialogue option at index {index}"),
            )
        })?;
    world.write_message(crate::viewer::dialogue::DialogueChoiceSelected(
        choice.clone(),
    ));
    Ok(ConsoleCommandResult::new(
        json!({ "queued": true, "index": index, "choice": choice.to_string() }),
        vec![format!("Dialogue option {index} queued.")],
    ))
}
