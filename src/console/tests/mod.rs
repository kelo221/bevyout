use bevy::prelude::*;
use serde_json::json;

use super::*;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ConsolePlugin));
    app.update();
    app
}

fn exec(app: &mut App, session: &str, line: &str) -> ConsoleOutput {
    ConsoleExecutor::execute(
        app.world_mut(),
        ConsoleRequest {
            session: ConsoleSessionId::new(session),
            line: line.to_string(),
        },
    )
}

fn register_entity(app: &mut App, form_id: u32, editor_id: &str, position: Vec3) -> Entity {
    let entity = app
        .world_mut()
        .spawn((
            Name::new(editor_id.to_string()),
            Transform::from_translation(position),
        ))
        .id();
    app.world_mut()
        .resource_mut::<RefRegistry>()
        .register(entity, form_id, Some(editor_id));
    entity
}

#[test]
fn third_party_registration_and_generated_help_work() {
    fn echo(
        _world: &mut World,
        invocation: &ConsoleInvocation,
    ) -> Result<ConsoleCommandResult, ConsoleError> {
        Ok(ConsoleCommandResult::value(json!(invocation.args)))
    }

    let mut app = test_app();
    app.world_mut()
        .resource_mut::<ConsoleRegistry>()
        .register(ConsoleCommand::new(
            "echo",
            "echo <value>",
            "Echo values.",
            echo,
        ))
        .unwrap();
    assert_eq!(exec(&mut app, "a", "echo hello").value, json!(["hello"]));
    let help = exec(&mut app, "a", "help echo");
    assert!(help.ok);
    assert_eq!(help.value["name"], "echo");
}

#[test]
fn sessions_keep_separate_reference_selections() {
    let mut app = test_app();
    register_entity(&mut app, 1, "One", Vec3::new(1.0, 0.0, 0.0));
    register_entity(&mut app, 2, "Two", Vec3::new(2.0, 0.0, 0.0));
    assert!(exec(&mut app, "a", "prid 00000001").ok);
    assert!(exec(&mut app, "b", "prid 00000002").ok);
    assert_eq!(exec(&mut app, "a", "getpos x").value, json!(1.0));
    assert_eq!(exec(&mut app, "b", "getpos x").value, json!(2.0));
}

#[test]
fn invalid_and_ambiguous_commands_do_not_mutate_transforms() {
    let mut app = test_app();
    let first = register_entity(&mut app, 1, "Duplicate", Vec3::new(1.0, 2.0, 3.0));
    register_entity(&mut app, 2, "Duplicate", Vec3::new(4.0, 5.0, 6.0));
    let before = *app.world().get::<Transform>(first).unwrap();
    let ambiguous = exec(&mut app, "a", "prid Duplicate");
    assert_eq!(ambiguous.error.unwrap().code, "ambiguous_reference");
    let bad_value = exec(&mut app, "a", "00000001.setpos x nope");
    assert_eq!(bad_value.error.unwrap().code, "bad_type");
    assert_eq!(*app.world().get::<Transform>(first).unwrap(), before);
}

#[test]
fn transforms_round_trip_in_metres_and_degrees() {
    let mut app = test_app();
    register_entity(&mut app, 1, "One", Vec3::ZERO);
    assert!(exec(&mut app, "a", "00000001.setpos z 1.5").ok);
    assert_eq!(exec(&mut app, "a", "00000001.getpos z").value, json!(1.5));
    assert!(exec(&mut app, "a", "00000001.setangle y 90").ok);
    let angle = exec(&mut app, "a", "00000001.getangle y")
        .value
        .as_f64()
        .unwrap();
    assert!((angle - 90.0).abs() < 0.001);
}

#[test]
fn repeated_fresh_apps_produce_identical_json() {
    fn run() -> String {
        let mut app = test_app();
        register_entity(&mut app, 1, "One", Vec3::new(1.0, 2.0, 3.0));
        serde_json::to_string(&exec(&mut app, "a", "00000001.getpos")).unwrap()
    }
    assert_eq!(run(), run());
}
