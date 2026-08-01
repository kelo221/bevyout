use super::*;
use crate::console::{ConsolePlugin, RefRegistry};

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin))
        .init_resource::<bevy::diagnostic::DiagnosticsStore>()
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(AgentBridgeInfo {
            port: 15_702,
            session_id: "bridge-test".into(),
            runtime_kind: "viewer",
            headless: true,
            physics_enabled: Some(false),
            capabilities: json!({ "scene_snapshot": 2 }),
        });
    let entity = app
        .world_mut()
        .spawn(Transform::from_xyz(1.0, 2.0, 3.0))
        .id();
    app.world_mut()
        .resource_mut::<RefRegistry>()
        .register(entity, 1, Some("TestRef"));
    app.update();
    app
}

#[test]
fn capabilities_report_bridge_build_and_runtime_contract() {
    let app = app();
    let value = bridge_metadata(app.world().resource::<AgentBridgeInfo>());
    assert_eq!(value["bridge_api_version"], 2);
    assert_eq!(value["runtime"]["kind"], "viewer");
    assert_eq!(value["capabilities"]["scene_snapshot"], 2);
    assert_eq!(value["mutation_policy"], "runtime_only");
}

#[test]
fn brp_console_result_matches_direct_executor_shape() {
    let mut direct = app();
    let expected = serde_json::to_value(ConsoleExecutor::execute(
        direct.world_mut(),
        ConsoleRequest {
            session: ConsoleSessionId::new("same"),
            line: "00000001.getpos".into(),
        },
    ))
    .unwrap();

    let mut remote = app();
    let actual = console_exec(
        In(Some(
            json!({ "session": "same", "line": "00000001.getpos" }),
        )),
        remote.world_mut(),
    )
    .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn brp_console_help_exposes_registry_metadata() {
    let app = app();
    let value = console_help(In(None), app.world()).unwrap();
    assert!(value
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["name"] == "getpos"));
}

#[test]
fn performance_snapshot_exposes_a_bounded_empty_window() {
    let mut app = app();
    let value = performance_snapshot(
        In(Some(json!({
            "latest_limit": 32,
            "budget_ms": 20.0,
            "include_samples": true,
        }))),
        app.world_mut(),
    )
    .unwrap();
    assert_eq!(value["window"]["sample_count"], 0);
    assert_eq!(value["window"]["budget_ms"], 20.0);
    assert_eq!(value["raw_samples_included"], true);
    assert!(value["world"]["entities"].as_u64().unwrap() >= 1);
}

#[test]
fn schedule_snapshot_reports_initialized_system_metadata() {
    let app = app();
    let value =
        schedule_snapshot(In(Some(json!({ "include_systems": true }))), app.world()).unwrap();
    assert!(value["schedule_count"].as_u64().unwrap() > 0);
    assert!(value["schedules"]
        .as_array()
        .unwrap()
        .iter()
        .any(|schedule| schedule["initialized"] == true));
}
