use super::*;

#[cfg(test)]
mod debug_info_tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn off_line_is_stable_and_alone() {
        assert_eq!(DEBUG_INFO_OFF_LINE, "Debug info: Off");
    }

    // Real-data smoke-test fix: with the block anchored bottom-left, a
    // multi-line render (a live test nav agent widens it to 4+ lines) grew
    // upward into the same bottom row `ColliderDebugHud`/`StepDebugHud`
    // occupy bottom-right, and the two texts garbled each other on screen.
    // The player transform HUD owns the first top-left row; this diagnostic
    // block begins beneath it. Asserting `bottom`/`right` stay `Val::Auto`
    // guarantees that its variable line count cannot grow into a different
    // corner's fixed diagnostic row.
    #[test]
    fn hud_is_anchored_top_left_never_the_existing_bottom_right_row() {
        let mut world = World::new();
        world.run_system_once(spawn_debug_info_hud).unwrap();
        let mut query = world.query_filtered::<&Node, With<DebugInfoHud>>();
        let node = query.single(&world).unwrap();
        assert_eq!(node.position_type, PositionType::Absolute);
        assert_eq!(node.top, Val::Px(DEBUG_INFO_HUD_TOP_PX));
        assert_eq!(node.left, Val::Px(10.0));
        assert_eq!(
            node.bottom,
            Val::Auto,
            "must remain top-anchored rather than sharing a bottom HUD row"
        );
        assert_eq!(
            node.right,
            Val::Auto,
            "must remain left-anchored rather than sharing a right HUD column"
        );
    }

    #[test]
    fn on_reports_unavailable_player_and_cell_when_absent() {
        let lines = format_debug_info_lines(None, None, &[]);
        assert_eq!(
            lines,
            vec![
                "Debug info: On".to_string(),
                "player pos=unavailable".to_string(),
                "cell=unavailable".to_string(),
            ]
        );
    }

    #[test]
    fn on_reports_player_position_and_cell_identity() {
        let lines = format_debug_info_lines(
            Some(Vec3::new(1.0, 2.5, -3.25)),
            Some((0x0002_8579, Some("VaultAtrium"), Some("Vault 101 Atrium"))),
            &[],
        );
        assert_eq!(
            lines,
            vec![
                "Debug info: On".to_string(),
                "player pos=(1.00,2.50,-3.25)".to_string(),
                "cell=00028579 editor_id=VaultAtrium name=Vault 101 Atrium".to_string(),
            ]
        );
    }

    #[test]
    fn on_reports_cell_with_no_editor_id_or_name_as_none() {
        let lines = format_debug_info_lines(None, Some((0x10, None, None)), &[]);
        assert_eq!(lines[2], "cell=00000010 editor_id=none name=none");
    }

    #[test]
    fn nav_agent_lines_are_appended_verbatim_after_cell() {
        let lines = format_debug_info_lines(
            None,
            None,
            &["nav agent 0 status=Idle position=(0.00,0.00,0.00) grounded=true stuck=false blocked=false".to_string()],
        );
        assert_eq!(lines.len(), 4);
        assert_eq!(
            lines[3],
            "nav agent 0 status=Idle position=(0.00,0.00,0.00) grounded=true stuck=false blocked=false"
        );
    }
}
