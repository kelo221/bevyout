use super::*;

use super::tests_support::*;

#[test]
fn hud_agent_lines_from_rows_sorts_roster_first_then_caps() {
    let row = |roster_index, entity_index, tag: &str| HudAgentRow {
        roster_index,
        entity_index,
        line: tag.to_string(),
    };
    let lines = hud_agent_lines_from_rows(vec![
        row(None, 7, "anon-7"),
        row(Some(1), 3, "roster-1"),
        row(Some(0), 9, "roster-0"),
        row(None, 1, "anon-1"),
    ]);
    assert_eq!(lines, vec!["roster-0", "roster-1", "anon-1", "anon-7"]);

    let capped = hud_agent_lines_from_rows(
        (0..(HUD_AGENT_LINE_LIMIT as u32 + 2))
            .map(|index| row(None, index, &format!("agent-{index:02}")))
            .collect(),
    );
    assert_eq!(capped.len(), HUD_AGENT_LINE_LIMIT + 1);
    assert_eq!(capped[0], "agent-00");
    assert_eq!(capped[HUD_AGENT_LINE_LIMIT - 1], "agent-07");
    assert_eq!(capped[HUD_AGENT_LINE_LIMIT], "nav agent +2 more");
}

#[test]
fn hud_projection_is_empty_without_a_roster_resource() {
    let mut world = World::new();
    world.spawn((NavAgent, GlobalTransform::default()));
    assert!(hud_projection_lines(&mut world).is_empty());
}

#[test]
fn hud_projection_formats_lines_like_tna_status() {
    let mut world = harness_world();
    let agent = world
        .spawn((
            NavAgent,
            AgentRuntime::default(),
            AgentKcc::default(),
            AgentState::default(),
            GlobalTransform::from(Transform::from_xyz(1.0, 2.0, 3.0)),
        ))
        .id();
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(agent);

    assert_eq!(
        hud_projection_lines(&mut world),
        vec![
            "nav agent 0 status=idle position=(1.00,2.00,3.00) grounded=false stuck=false blocked=false"
                .to_string()
        ]
    );
}

#[test]
fn hud_projection_orders_roster_agents_first_and_caps_the_tail() {
    let mut world = harness_world();
    let late_roster_agent = {
        let mut spawned = Vec::new();
        for _ in 0..(HUD_AGENT_LINE_LIMIT + 1) {
            spawned.push(
                world
                    .spawn((
                        NavAgent,
                        AgentRuntime::default(),
                        AgentKcc::default(),
                        AgentState::default(),
                        GlobalTransform::default(),
                    ))
                    .id(),
            );
        }
        *spawned.last().unwrap()
    };
    // The console-addressed agent was spawned last (highest entity index) but
    // must still sort first; the anonymous tail is what the cap truncates.
    world.resource_mut::<DebugAgentRoster>().entities[0] = Some(late_roster_agent);

    let lines = hud_projection_lines(&mut world);
    assert_eq!(lines.len(), HUD_AGENT_LINE_LIMIT + 1);
    assert!(
        lines[0].starts_with("nav agent 0 status=idle"),
        "roster index 0 leads regardless of spawn order: {}",
        lines[0]
    );
    for line in &lines[1..HUD_AGENT_LINE_LIMIT] {
        assert!(
            line.starts_with("nav agent e"),
            "unrosterred agents fall back to entity ids: {line}"
        );
    }
    assert_eq!(lines[HUD_AGENT_LINE_LIMIT], "nav agent +1 more");
}
