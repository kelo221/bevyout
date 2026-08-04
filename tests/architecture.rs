use std::fs;
use std::path::Path;

fn rust_files_below(root: &Path, output: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(root).expect("read architecture directory") {
        let path = entry.expect("read architecture entry").path();
        if path.is_dir() {
            rust_files_below(&path, output);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            output.push(path);
        }
    }
}

#[test]
fn core_crate_has_no_bevy_dependency() {
    let manifest = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/bevyout-core/Cargo.toml"),
    )
    .expect("read bevyout-core manifest");
    assert!(
        !manifest.lines().any(|line| {
            line.trim_start().starts_with("bevy =") || line.trim_start().starts_with("bevy_")
        }),
        "bevyout-core must remain engine-independent"
    );
}

#[test]
fn core_normal_dependencies_remain_serde_and_glam_only() {
    let manifest = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/bevyout-core/Cargo.toml"),
    )
    .expect("read bevyout-core manifest");
    let dependencies = manifest
        .split("[dependencies]")
        .nth(1)
        .expect("dependencies section")
        .split("[dev-dependencies]")
        .next()
        .expect("normal dependency body");
    let names = dependencies
        .lines()
        .filter_map(|line| line.split_once('=').map(|(name, _)| name.trim()))
        .filter(|name| !name.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(names, ["glam", "serde"].into_iter().collect());
}

#[test]
fn core_sources_exclude_engine_and_json_adapter_imports() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/bevyout-core/src");
    let mut files = Vec::new();
    rust_files_below(&root, &mut files);
    let offenders = files
        .into_iter()
        .filter(|path| {
            let source = fs::read_to_string(path).expect("read core source");
            source.contains("use bevy")
                || source.contains("serde_json")
                || source.contains("bevy_rapier")
                || source.contains("avian")
        })
        .collect::<Vec<_>>();
    assert!(
        offenders.is_empty(),
        "core contains engine or adapter imports: {offenders:?}"
    );
}

#[test]
fn hitscan_adapter_reports_evidence_instead_of_owning_damage_policy() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/weapon/hitscan.rs"),
    )
    .expect("read hitscan adapter");
    assert!(!source.contains("apply_actor_damage"));
    assert!(source.contains("resolve_actor_impact"));
    assert!(source.contains("impact_is_in_range"));
}

#[test]
fn preparation_does_not_depend_on_viewer() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/vsa");
    let mut files = Vec::new();
    rust_files_below(&root, &mut files);
    let offenders = files
        .into_iter()
        .filter(|path| {
            fs::read_to_string(path)
                .expect("read Rust source")
                .contains("crate::viewer")
        })
        .collect::<Vec<_>>();
    assert!(
        offenders.is_empty(),
        "preparation/runtime dependency inversion in: {offenders:?}"
    );
}

#[test]
fn interaction_coordinator_stays_small_and_delegates_by_behavior() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer");
    let coordinator =
        fs::read_to_string(root.join("interaction.rs")).expect("read interaction coordinator");
    assert!(
        coordinator.lines().count() <= 250,
        "interaction.rs is a composition boundary, not a feature dumping ground"
    );

    for module in [
        "activation.rs",
        "door.rs",
        "focus.rs",
        "items.rs",
        "presentation.rs",
        "scripted.rs",
        "state.rs",
        "ui.rs",
    ] {
        assert!(
            root.join("interaction").join(module).is_file(),
            "missing interaction behavior module {module}"
        );
    }
}

#[test]
fn viewer_console_coordinator_stays_small_and_delegates_by_command_family() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer");
    let coordinator =
        fs::read_to_string(root.join("console.rs")).expect("read viewer console coordinator");
    assert!(
        coordinator.lines().count() <= 150,
        "viewer/console.rs must remain a thin Bevy plugin and registration boundary"
    );

    for module in [
        "common.rs",
        "item_commands.rs",
        "navigation_commands.rs",
        "persistence_commands.rs",
        "player_commands.rs",
        "render_commands.rs",
        "ui_commands.rs",
        "world_commands.rs",
        "tests.rs",
    ] {
        assert!(
            root.join("console").join(module).is_file(),
            "missing viewer console capability module {module}"
        );
    }
}

#[test]
fn ai_uses_the_navigation_api_boundary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/ai");
    let mut files = Vec::new();
    rust_files_below(&root, &mut files);
    let offenders = files
        .into_iter()
        .filter(|path| {
            let source = fs::read_to_string(path).expect("read AI source");
            source.contains("viewer::nav::agent")
                || source.contains("nav::agent")
                || source.contains("nav\\agent")
        })
        .collect::<Vec<_>>();
    assert!(
        offenders.is_empty(),
        "AI must depend on nav::api, not nav::agent internals: {offenders:?}"
    );
}

#[test]
fn navigation_agent_uses_the_composition_directory_and_named_markers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/nav");
    assert!(
        !root.join("agent.rs").exists(),
        "agent root must be a module directory"
    );
    for module in [
        "mod.rs",
        "components.rs",
        "actor_binding.rs",
        "locomotion.rs",
        "fall_guard.rs",
    ] {
        assert!(
            root.join("agent").join(module).is_file(),
            "missing agent capability module {module}"
        );
    }
    let source = fs::read_to_string(root.join("agent/mod.rs")).expect("read nav agent root");
    assert!(!source.contains("TestNavAgentMarker"));
    assert!(!source.contains("TestNavAgentState"));
    let components =
        fs::read_to_string(root.join("agent/components.rs")).expect("read nav agent components");
    assert!(components.contains("NavAgent"));
    assert!(components.contains("DebugAgentRoster"));
}

#[test]
fn navigation_api_does_not_leak_backend_types() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/nav/api.rs");
    let source = fs::read_to_string(path).expect("read navigation API");
    assert!(!source.contains("bevy_landmass"));
    assert!(!source.contains("bevy_boxddd"));
    assert!(!source.contains("Boxddd"));
}

#[test]
fn navigation_slice_has_named_capability_boundaries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/nav");
    let nav_root = fs::read_to_string(root.join("mod.rs")).expect("read navigation root");
    let agent_root = fs::read_to_string(root.join("agent/mod.rs")).expect("read agent root");

    assert!(
        nav_root.lines().count() <= 120,
        "nav/mod.rs is a composition boundary"
    );
    assert!(
        agent_root.lines().count() <= 150,
        "agent/mod.rs is a composition boundary"
    );
    assert!(!nav_root.contains("pub(crate) fn read_nav_graph"));
    assert!(!agent_root.contains("agent_part_"));

    let requirements: &[(&str, &[&str])] = &[
        (
            "world",
            &[
                "mod.rs",
                "state.rs",
                "build.rs",
                "exterior.rs",
                "portals.rs",
                "links.rs",
                "player_obstacle.rs",
            ],
        ),
        (
            "doors",
            &[
                "mod.rs",
                "access.rs",
                "availability.rs",
                "runtime.rs",
                "traversal.rs",
                "travel.rs",
                "fsm.rs",
            ],
        ),
        ("handoff", &["mod.rs", "ledger.rs", "cell_transition.rs"]),
        ("diagnostics", &["mod.rs", "logging.rs", "hud.rs"]),
        (
            "debug",
            &[
                "mod.rs",
                "command.rs",
                "roster.rs",
                "capsule.rs",
                "probes.rs",
            ],
        ),
    ];
    for (directory, modules) in requirements {
        for module in (*modules).iter() {
            assert!(
                root.join(directory).join(module).is_file(),
                "missing navigation capability module {directory}/{module}"
            );
        }
    }
}

#[test]
fn navigation_runtime_state_and_console_errors_have_one_owner() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/nav");
    let mut files = Vec::new();
    rust_files_below(&root, &mut files);
    let state_definitions = files
        .iter()
        .filter_map(|path| {
            let source = fs::read_to_string(path).ok()?;
            source
                .contains("struct NavArchipelagoState")
                .then_some(path)
        })
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(
        state_definitions,
        vec![root.join("world/state.rs")],
        "active archipelago state must have one definition"
    );

    for path in files {
        let relative = path.strip_prefix(&root).expect("nav path is below root");
        let source = fs::read_to_string(&path).expect("read nav source");
        if relative.starts_with("world")
            || relative.starts_with("handoff")
            || relative.starts_with("traversal")
            || relative.starts_with("doors")
        {
            assert!(
                !source.contains("ConsoleError"),
                "runtime policy modules must return NavError or domain values: {}",
                relative.display()
            );
        }
    }

    let movement = fs::read_to_string(root.join("agent/movement.rs")).expect("read movement");
    assert_eq!(
        movement.matches("transform.translation =").count(),
        1,
        "movement owns the single agent translation write"
    );
    for path in [
        root.join("doors/traversal.rs"),
        root.join("traversal/merge.rs"),
    ] {
        assert!(
            !fs::read_to_string(path)
                .expect("read traversal source")
                .contains("transform.translation ="),
            "traversal adapters use the movement translation seam"
        );
    }
}

#[test]
fn navigation_tests_are_split_by_capability() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/viewer/nav/tests");
    assert!(!root.join("agent.rs").exists());
    for module in [
        "support.rs",
        "agent_world.rs",
        "agent_debug.rs",
        "agent_movement.rs",
        "agent_traversal.rs",
        "agent_doors.rs",
        "agent_handoff.rs",
        "agent_wedge.rs",
        "agent_diagnostics.rs",
    ] {
        assert!(
            root.join(module).is_file(),
            "missing split navigation test {module}"
        );
    }
}
