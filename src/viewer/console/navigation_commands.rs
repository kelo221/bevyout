//! Navigation visualization and test-agent console adapters.

use super::*;

pub(super) struct NavigationCommandProvider;

impl ConsoleCommandProvider for NavigationCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
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
            "nav",
            "nav exterior|borders",
            "Report the active prepared exterior navigation tile or its border portals.",
            nav::debug::exterior_command,
        ),
        ConsoleCommand::new(
            "tna",
            "tna spawn [<index>]|goto [<index>] <x> <y> <z>|goto [<index>] player|travel [<index>] <door-formid>|status [<index>]|despawn [<index>]|solverate [<n>]",
            "Test nav agent (issues #112/#114/#134): spawn/goto/travel/status/despawn a physics-authoritative bevy_landmass-driven agent (bounded multi-agent, index defaults to 0) with intercell handoff, plus fixed-tick movement, player-avoidance, and a solverate knob for the nav solve interval.",
            nav::debug::tna_command,
        )
        .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}
