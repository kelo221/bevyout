//! Save-game console command adapter.

use super::*;

pub(super) fn register(registry: &mut ConsoleRegistry) {
    registry
        .register(
            ConsoleCommand::new(
                "save",
                "save <slot>",
                "Capture the active cell state and write it to a named save slot.",
                save_slot,
            )
            .mutating(),
        )
        .expect("save console command is unique");
}

/// Issue #60 (F60.3): captures the active cell into `ActiveSaveState` and
/// writes the named slot through `SaveStore`, responding deterministically
/// with the written path.
pub(super) fn save_slot(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [slot] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "save requires exactly one slot name",
        ));
    };
    let path = super::super::world::write_save_slot(world, slot)
        .map_err(|error| ConsoleError::new("save_failed", format!("{error:#}")))?;
    Ok(ConsoleCommandResult::new(
        json!({ "slot": slot, "path": path }),
        vec![format!("Save written to {}.", path.display())],
    ))
}
