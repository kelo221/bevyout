//! Navigation visualization and test-agent console adapters.

use super::*;

pub(super) fn register(registry: &mut ConsoleRegistry) {
    for command in [
        ConsoleCommand::new(
            "tnm",
            "tnm",
            "Toggle navigation-mesh triangle visualization.",
            nav_overlay::toggle_nav_mesh,
        )
        .aliases(&["togglenavmesh"])
        .mutating(),
        ConsoleCommand::new(
            "tna",
            "tna spawn [<index>]|goto [<index>] <x> <y> <z>|goto [<index>] player|travel [<index>] <door-formid>|status [<index>]|despawn [<index>]|solverate [<n>]",
            "Test nav agent (issues #112/#114/#134): spawn/goto/travel/status/despawn a physics-authoritative bevy_landmass-driven agent (bounded multi-agent, index defaults to 0) with intercell handoff, plus fixed-tick movement, player-avoidance, and a solverate knob for the nav solve interval.",
            nav::agent::tna_command,
        )
        .mutating(),
    ] {
        registry
            .register(command)
            .expect("navigation console command is unique");
    }
}
