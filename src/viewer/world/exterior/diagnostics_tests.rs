use bevy::prelude::World;

use super::super::lifecycle::ExteriorStreamState;
use super::{ProcessMemoryDiagnostics, status, status_with_memory};

#[cfg(target_os = "macos")]
#[test]
fn macos_process_memory_uses_native_resident_set_sampler() {
    assert_eq!(super::PROCESS_MEMORY_METHOD, "libproc_process_resident_set");
    assert!(super::read_current_process_memory().is_some());
}

#[test]
fn supported_memory_stays_not_yet_sampled_until_a_real_sample_is_recorded() {
    let state = ExteriorStreamState::default();
    let memory = ProcessMemoryDiagnostics::supported_for_tests();

    let report = status_with_memory(&state, &memory);

    assert_eq!(report["memory_measurement"], "unmeasured");
    assert_eq!(report["memory_measurement_status"], "not_yet_sampled");
    assert_eq!(
        report["memory_measurement_method"],
        super::PROCESS_MEMORY_METHOD
    );
    assert_eq!(report["memory_measurement_platform"], std::env::consts::OS);
    assert_eq!(report["resident_bytes"], serde_json::Value::Null);
    assert_eq!(report["peak_memory"], serde_json::Value::Null);
    assert_eq!(report["ending_memory"], serde_json::Value::Null);
    assert_eq!(report["process_memory"]["status"], "not_yet_sampled");
    assert_eq!(report["process_memory"]["value"], serde_json::Value::Null);
}

#[test]
fn unsupported_memory_is_explicit_and_unmeasured() {
    let state = ExteriorStreamState::default();
    let memory = ProcessMemoryDiagnostics::unsupported_for_tests();

    let report = status_with_memory(&state, &memory);

    assert_eq!(report["memory_measurement"], "unmeasured");
    assert_eq!(report["memory_measurement_status"], "unsupported");
    assert_eq!(report["memory_measurement_method"], "unsupported");
    assert_eq!(report["memory_measurement_platform"], std::env::consts::OS);
    assert_eq!(report["resident_bytes"], serde_json::Value::Null);
    assert_eq!(report["peak_memory"], serde_json::Value::Null);
    assert_eq!(report["ending_memory"], serde_json::Value::Null);
    assert_eq!(report["process_memory"]["status"], "unsupported");
    assert_eq!(report["process_memory"]["value"], serde_json::Value::Null);
}

#[test]
fn process_memory_tracks_current_peak_and_ending_samples() {
    let state = ExteriorStreamState::default();
    let mut memory = ProcessMemoryDiagnostics::supported_for_tests();
    memory.begin_trace();
    memory.record_sample(1_024);
    memory.record_sample(4_096);
    memory.record_sample(2_048);
    memory.finish_trace();

    let report = status_with_memory(&state, &memory);

    assert_eq!(report["resident_bytes"], 2_048);
    assert_eq!(report["peak_memory"], 4_096);
    assert_eq!(report["ending_memory"], 2_048);
    assert_eq!(report["memory_sample_count"], 3);
    assert_eq!(report["memory_measurement"], "process_resident_set");
    assert_eq!(report["memory_measurement_status"], "supported");
    assert_eq!(report["process_memory"]["status"], "supported");
    assert_eq!(report["process_memory"]["value"]["resident_bytes"], 2_048);
    assert_eq!(report["process_memory"]["value"]["peak_bytes"], 4_096);
    assert_eq!(report["process_memory"]["value"]["ending_bytes"], 2_048);
}

#[test]
fn package_estimates_never_populate_process_memory_fields() {
    let state = ExteriorStreamState {
        resident_bytes: 256,
        peak_memory: 512,
        invalid_unload_count: 3,
        ..Default::default()
    };
    let memory = ProcessMemoryDiagnostics::supported_for_tests();

    let report = status_with_memory(&state, &memory);

    assert_eq!(report["resident_package_bytes_estimate"], 256);
    assert_eq!(report["peak_package_bytes_estimate"], 512);
    assert_eq!(report["resident_bytes"], serde_json::Value::Null);
    assert_eq!(report["peak_memory"], serde_json::Value::Null);
    assert_eq!(report["ending_memory"], serde_json::Value::Null);
    assert_eq!(report["invalid_unload_count"], 3);
    assert_eq!(report["process_memory"]["status"], "not_yet_sampled");
    assert_eq!(report["process_memory"]["value"], serde_json::Value::Null);
}

#[test]
fn nested_memory_projection_keeps_process_and_package_values_separate() {
    let state = ExteriorStreamState {
        resident_bytes: 256,
        peak_memory: 512,
        ..Default::default()
    };
    let mut memory = ProcessMemoryDiagnostics::supported_for_tests();
    memory.record_sample(2_048);

    let report = status_with_memory(&state, &memory);

    assert_eq!(report["process_memory"]["status"], "supported");
    assert_eq!(report["process_memory"]["value"]["resident_bytes"], 2_048);
    assert_eq!(report["process_memory"]["value"]["peak_bytes"], 2_048);
    assert_eq!(
        report["process_memory"]["value"]["ending_bytes"],
        serde_json::Value::Null
    );
    assert_eq!(report["package_estimate"]["status"], "estimated");
    assert_eq!(report["package_estimate"]["value"]["resident_bytes"], 256);
    assert_eq!(report["package_estimate"]["value"]["peak_bytes"], 512);
    assert_eq!(
        report["package_estimate"]["kind"],
        "estimated_package_serialization"
    );
}

#[test]
fn process_memory_report_json_is_deterministic() {
    let state = ExteriorStreamState::default();
    let mut first_memory = ProcessMemoryDiagnostics::supported_for_tests();
    first_memory.record_sample(2_048);
    let mut second_memory = ProcessMemoryDiagnostics::supported_for_tests();
    second_memory.record_sample(2_048);

    let first = serde_json::to_string(&status_with_memory(&state, &first_memory))
        .expect("first report serializes");
    let second = serde_json::to_string(&status_with_memory(&state, &second_memory))
        .expect("second report serializes");

    assert_eq!(first, second);
}

#[test]
fn world_status_source_owns_sampling_and_closes_trace_windows() {
    let mut world = World::new();
    world.insert_resource(ExteriorStreamState {
        resident_bytes: 256,
        peak_memory: 512,
        trace: true,
        ..Default::default()
    });

    let started = status(&mut world);
    assert!(
        world
            .get_resource::<ProcessMemoryDiagnostics>()
            .is_some_and(|memory| memory.trace_active)
    );
    assert_eq!(started["resident_package_bytes_estimate"], 256);
    assert_eq!(started["peak_package_bytes_estimate"], 512);

    world.resource_mut::<ExteriorStreamState>().trace = false;
    let ended = status(&mut world);
    assert_eq!(ended["memory_trace_active"], false);
    assert_eq!(ended["resident_package_bytes_estimate"], 256);
    assert_eq!(ended["peak_package_bytes_estimate"], 512);
}
