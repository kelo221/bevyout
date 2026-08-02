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

    // -- Issue #268: change-driven debug info HUD ---------------------------
    //
    // The pre-#268 exclusive (`&mut World`) system rewrote the HUD `Text`
    // every single frame, on or off. The new contract: the off line is only
    // (re)written on a toggle transition, enabled content refreshes on a
    // bounded 5-10 Hz timer, and no frame may assign `Text` while the
    // composed string is unchanged. Change detection on the `Text` component
    // (via a spy system running just after the update system) is the exact
    // signal a relayout would key off, so it is what these tests count.

    use bevy::time::TimeUpdateStrategy;
    use std::time::Duration;

    #[derive(Resource, Default)]
    struct DebugInfoWriteLog(Vec<bool>);

    fn record_debug_info_writes(
        hud: Query<Ref<Text>, With<DebugInfoHud>>,
        mut log: ResMut<DebugInfoWriteLog>,
    ) {
        let changed = hud.iter().next().is_some_and(|text| text.is_changed());
        log.0.push(changed);
    }

    /// Minimal deterministic app: fixed 50 ms frames, no render/UI plugins.
    fn debug_info_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                50,
            )))
            .init_resource::<DebugInfoState>()
            .init_resource::<DebugInfoWriteLog>()
            .add_systems(Startup, spawn_debug_info_hud)
            .add_systems(
                Update,
                (update_debug_info_hud, record_debug_info_writes).chain(),
            );
        app
    }

    fn hud_text_now(app: &mut App) -> String {
        app.world_mut()
            .query_filtered::<&Text, With<DebugInfoHud>>()
            .single(app.world())
            .unwrap()
            .0
            .clone()
    }

    #[test]
    fn steady_off_state_never_mutates_the_hud_text() {
        let mut app = debug_info_app();
        app.update();
        assert_eq!(hud_text_now(&mut app), DEBUG_INFO_OFF_LINE);
        app.world_mut()
            .resource_mut::<DebugInfoWriteLog>()
            .0
            .clear();

        for _ in 0..5 {
            app.update();
        }

        let log = &app.world().resource::<DebugInfoWriteLog>().0;
        assert_eq!(log.len(), 5);
        assert!(
            log.iter().all(|changed| !changed),
            "steady off-state frames must not touch the HUD text: {log:?}"
        );
        assert_eq!(hud_text_now(&mut app), DEBUG_INFO_OFF_LINE);
    }

    #[test]
    fn each_toggle_transition_writes_exactly_once() {
        let mut app = debug_info_app();
        app.update();
        app.world_mut()
            .resource_mut::<DebugInfoWriteLog>()
            .0
            .clear();

        app.world_mut().resource_mut::<DebugInfoState>().enabled = true;
        app.update();
        // Settled enabled frames (including timer fires, which compose the
        // identical string while nothing moves) must not write again.
        for _ in 0..4 {
            app.update();
        }
        assert_eq!(
            app.world().resource::<DebugInfoWriteLog>().0,
            &[true, false, false, false, false],
            "off->on writes exactly once"
        );
        assert!(hud_text_now(&mut app).contains("Debug info: On"));

        app.world_mut().resource_mut::<DebugInfoState>().enabled = false;
        app.update();
        app.update();
        let log = &app.world().resource::<DebugInfoWriteLog>().0;
        assert_eq!(&log[5..], &[true, false], "on->off writes exactly once");
        assert_eq!(hud_text_now(&mut app), DEBUG_INFO_OFF_LINE);
    }

    #[test]
    fn enabled_refresh_waits_for_the_timer_and_then_coalesces() {
        let refresh = DEBUG_INFO_REFRESH_INTERVAL;
        assert!(
            (Duration::from_millis(100)..=Duration::from_millis(200)).contains(&refresh),
            "enabled refresh must stay within the 5-10 Hz band (100-200 ms)"
        );

        let mut app = debug_info_app();
        app.update();
        let player = app
            .world_mut()
            .spawn((Transform::default(), player::FpsPlayer::default()))
            .id();
        app.world_mut().resource_mut::<DebugInfoState>().enabled = true;
        app.update(); // toggle: writes immediately with the origin position
        assert!(hud_text_now(&mut app).contains("player pos=(0.00,0.00,0.00)"));
        app.world_mut()
            .resource_mut::<DebugInfoWriteLog>()
            .0
            .clear();

        // Move the player; pre-timer frames must not touch the HUD (the
        // 125 ms timer restarted when the toggle ran, and each update ticks
        // 50 ms, so the first two frames stay below the interval).
        app.world_mut()
            .get_mut::<Transform>(player)
            .unwrap()
            .translation = Vec3::new(1.5, 2.5, 3.5);
        app.update(); // elapsed 50 ms
        app.update(); // elapsed 100 ms
        assert!(
            !hud_text_now(&mut app).contains("player pos=(1.50,2.50,3.50)"),
            "no refresh before the timer fires"
        );
        assert_eq!(
            app.world().resource::<DebugInfoWriteLog>().0,
            &[false, false]
        );

        app.update(); // elapsed 150 ms >= 125 ms: timer fired, one write
        assert!(hud_text_now(&mut app).contains("player pos=(1.50,2.50,3.50)"));
        assert_eq!(
            app.world().resource::<DebugInfoWriteLog>().0,
            &[false, false, true]
        );

        // A second move mid-interval is coalesced into the next timer fire
        // (the repeating timer wraps with 25 ms carried, so the next fire
        // lands two frames later).
        app.world_mut()
            .get_mut::<Transform>(player)
            .unwrap()
            .translation = Vec3::new(-4.0, -5.0, -6.0);
        app.update(); // elapsed 75 ms
        assert_eq!(
            app.world().resource::<DebugInfoWriteLog>().0,
            &[false, false, true, false]
        );
        assert!(!hud_text_now(&mut app).contains("player pos=(-4.00,-5.00,-6.00)"));
        app.update(); // elapsed 125 ms: next fire
        assert!(hud_text_now(&mut app).contains("player pos=(-4.00,-5.00,-6.00)"));
        assert_eq!(
            app.world().resource::<DebugInfoWriteLog>().0,
            &[false, false, true, false, true]
        );
    }
}

#[cfg(test)]
mod convergence_report_tests {
    use super::*;

    const EXPECTED_DOMAINS: [&str; 8] = [
        "streaming_lifecycle",
        "actor_navigation",
        "travel_save",
        "environment",
        "presentation",
        "cache_preparation",
        "frame_timing",
        "process_memory",
    ];

    fn domain<'a>(report: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
        report["domains"]
            .as_array()
            .expect("report has an ordered domains array")
            .iter()
            .find(|domain| domain["name"] == name)
            .unwrap_or_else(|| panic!("report has domain {name}"))
    }

    #[test]
    fn empty_report_has_stable_domain_order_and_bytes() {
        let mut first_world = World::new();
        let mut second_world = World::new();

        let first = convergence_report(&mut first_world);
        let second = convergence_report(&mut second_world);

        let names = first["domains"]
            .as_array()
            .expect("report has domains")
            .iter()
            .map(|domain| domain["name"].as_str().expect("domain name"))
            .collect::<Vec<_>>();
        assert_eq!(names, EXPECTED_DOMAINS);
        assert_eq!(
            serde_json::to_string(&first).expect("first report serializes"),
            serde_json::to_string(&second).expect("second report serializes")
        );
    }

    #[test]
    fn unrun_domains_keep_status_separate_from_value() {
        let mut world = World::new();
        let report = convergence_report(&mut world);

        assert_eq!(report["schema"], "m6-convergence-v1");
        for name in [
            "streaming_lifecycle",
            "actor_navigation",
            "travel_save",
            "environment",
            "cache_preparation",
        ] {
            let domain = domain(&report, name);
            assert_eq!(domain["status"], "not_run", "domain={name}");
            assert_eq!(domain["value"], serde_json::Value::Null, "domain={name}");
        }

        let frame = domain(&report, "frame_timing");
        assert_eq!(frame["status"], "not_run");
        assert_eq!(frame["value"], serde_json::Value::Null);

        let memory = domain(&report, "process_memory");
        assert_eq!(memory["status"], "not_run");
        assert_eq!(memory["value"], serde_json::Value::Null);
    }

    #[test]
    fn streaming_summary_projection_keeps_existing_live_surfaces() {
        let mut world = World::new();
        world.insert_resource(super::super::super::world::exterior::ExteriorStreamState {
            initialized: true,
            worldspace_form_id: Some(0x3c),
            resident_budget: 25,
            ..Default::default()
        });

        let report = convergence_report(&mut world);

        assert_eq!(report["streaming"]["initialized"], true);
        assert_eq!(report["streaming"]["worldspace"], 0x3c);
        assert_eq!(
            report["presentation"]["terrain"]["collision"],
            "full_land_mesh"
        );
        assert_eq!(domain(&report, "streaming_lifecycle")["status"], "measured");
        assert_eq!(domain(&report, "presentation")["status"], "measured");
        let expected_memory_status = match report["streaming"]["memory_measurement_status"].as_str()
        {
            Some("supported") => "measured",
            Some("not_yet_sampled") => "not_yet_sampled",
            Some("unsupported") => "unsupported",
            _ => "not_run",
        };
        assert_eq!(
            domain(&report, "process_memory")["status"],
            expected_memory_status
        );
        assert_eq!(
            domain(&report, "process_memory")["method"],
            report["streaming"]["memory_measurement_method"]
        );
    }
}
