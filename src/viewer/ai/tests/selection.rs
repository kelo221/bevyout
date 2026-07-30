use super::*;
use std::collections::HashMap;

/// A boundary backed by a fixed `(function_index) -> value` map.
struct MapFunctions(HashMap<u16, f32>);

impl ConditionFunctions for MapFunctions {
    fn evaluate(&self, function_index: u16, _param1: u32, _param2: u32) -> Option<f32> {
        self.0.get(&function_index).copied()
    }
}

/// Builds a minimal 20-byte CTDA: operator in the top 3 bits, OR flag in
/// bit 0, `comparison_value` at offset 4, `function_index` at offset 8.
fn ctda(op_bits: u8, or_with_next: bool, comparison_value: f32, function_index: u16) -> Vec<u8> {
    let mut bytes = vec![0u8; CTDA_MIN_LEN];
    bytes[0] = (op_bits << 5) | u8::from(or_with_next);
    bytes[4..8].copy_from_slice(&comparison_value.to_le_bytes());
    bytes[8..10].copy_from_slice(&function_index.to_le_bytes());
    bytes
}

fn candidate(form_id: u32) -> PackageCandidate {
    PackageCandidate {
        form_id,
        ..PackageCandidate::default()
    }
}

#[test]
fn max_known_type_tracks_the_catalog() {
    assert_eq!(MAX_KNOWN_PACKAGE_TYPE, 16);
}

#[test]
fn first_eligible_wins_and_higher_priority_rejections_are_reported() {
    let mut unsupported = candidate(0x10);
    unsupported.package_type = 200;
    let eligible = candidate(0x20);
    let lower = candidate(0x30);
    let report = select_package(
        &[unsupported, eligible, lower],
        GameInstant::default(),
        &NoConditionFunctions,
    );
    assert_eq!(report.selected, Some(0x20));
    assert_eq!(
        report.evaluations[0].outcome,
        CandidateOutcome::Rejected(RejectionReason::UnsupportedType)
    );
    assert_eq!(report.evaluations[1].outcome, CandidateOutcome::Selected);
    assert_eq!(
        report.evaluations[2].outcome,
        CandidateOutcome::Rejected(RejectionReason::LowerPriority)
    );
    assert_eq!(report.counters.unsupported_type, 1);
    assert_eq!(report.counters.schedule_gap, 0);
}

#[test]
fn priority_tie_between_two_eligible_packages_takes_the_earlier() {
    let report = select_package(
        &[candidate(0xAA), candidate(0xBB)],
        GameInstant::default(),
        &NoConditionFunctions,
    );
    assert_eq!(report.selected, Some(0xAA));
    assert_eq!(
        report.evaluations[1].outcome,
        CandidateOutcome::Rejected(RejectionReason::LowerPriority)
    );
}

#[test]
fn a_schedule_gap_selects_nothing_and_is_counted() {
    let mut only = candidate(0x10);
    only.schedule = Some(PackageSchedule {
        time: 8,
        duration: 60,
        ..PackageSchedule::default()
    });
    let now = GameInstant {
        hour: 20.0,
        ..GameInstant::default()
    };
    let report = select_package(&[only], now, &NoConditionFunctions);
    assert_eq!(report.selected, None);
    assert_eq!(report.counters.out_of_schedule, 1);
    assert_eq!(report.counters.schedule_gap, 1);
}

#[test]
fn no_schedule_is_always_in_window() {
    let report = select_package(
        &[candidate(0x10)],
        GameInstant::default(),
        &NoConditionFunctions,
    );
    assert_eq!(report.selected, Some(0x10));
}

#[test]
fn schedule_boundaries_are_half_open() {
    let schedule = PackageSchedule {
        time: 8,
        duration: 120, // 08:00..10:00
        ..PackageSchedule::default()
    };
    let at = |hour| {
        schedule.evaluate(GameInstant {
            hour,
            ..GameInstant::default()
        })
    };
    assert_eq!(at(7.99), ScheduleMatch::OutOfWindow);
    assert_eq!(at(8.0), ScheduleMatch::InWindow); // inclusive start
    assert_eq!(at(9.99), ScheduleMatch::InWindow);
    assert_eq!(at(10.0), ScheduleMatch::OutOfWindow); // exclusive end
}

#[test]
fn schedule_wraps_past_midnight() {
    let schedule = PackageSchedule {
        time: 22,
        duration: 240, // 22:00..02:00
        ..PackageSchedule::default()
    };
    let at = |hour| {
        schedule.evaluate(GameInstant {
            hour,
            ..GameInstant::default()
        })
    };
    assert_eq!(at(23.0), ScheduleMatch::InWindow);
    assert_eq!(at(0.5), ScheduleMatch::InWindow);
    assert_eq!(at(1.99), ScheduleMatch::InWindow);
    assert_eq!(at(2.0), ScheduleMatch::OutOfWindow);
    assert_eq!(at(12.0), ScheduleMatch::OutOfWindow);
}

#[test]
fn month_date_and_day_gates_reject() {
    let schedule = PackageSchedule {
        month: 3,
        date: 15,
        day_of_week: 2,
        time: 0,
        duration: 1440,
    };
    let base = GameInstant {
        month: 3,
        date: 15,
        day_of_week: 2,
        hour: 6.0,
    };
    assert_eq!(schedule.evaluate(base), ScheduleMatch::InWindow);
    assert_eq!(
        schedule.evaluate(GameInstant { month: 4, ..base }),
        ScheduleMatch::OutOfWindow
    );
    assert_eq!(
        schedule.evaluate(GameInstant { date: 16, ..base }),
        ScheduleMatch::OutOfWindow
    );
    assert_eq!(
        schedule.evaluate(GameInstant {
            day_of_week: 3,
            ..base
        }),
        ScheduleMatch::OutOfWindow
    );
}

#[test]
fn unscheduled_time_sentinel_is_always_on() {
    let schedule = PackageSchedule {
        time: -1,
        month: 5,
        ..PackageSchedule::default()
    };
    assert_eq!(
        schedule.evaluate(GameInstant::default()),
        ScheduleMatch::Unscheduled
    );
}

#[test]
fn a_true_condition_selects_and_a_false_one_rejects() {
    let boundary = MapFunctions(HashMap::from([(100u16, 1.0f32)]));
    let mut yes = candidate(0x10);
    yes.conditions = vec![ctda(0, false, 1.0, 100)];
    let report = select_package(&[yes], GameInstant::default(), &boundary);
    assert_eq!(report.selected, Some(0x10));

    let mut no = candidate(0x10);
    no.conditions = vec![ctda(0, false, 5.0, 100)];
    let report = select_package(&[no], GameInstant::default(), &boundary);
    assert_eq!(report.selected, None);
    assert_eq!(report.counters.conditions_false, 1);
}

#[test]
fn an_unsupported_function_is_unevaluable_and_routed_out() {
    let mut c = candidate(0x10);
    c.conditions = vec![ctda(0, false, 1.0, 4242)];
    let report = select_package(&[c], GameInstant::default(), &NoConditionFunctions);
    assert_eq!(report.selected, None);
    assert_eq!(report.counters.conditions_unevaluable, 1);
}

#[test]
fn or_group_is_true_if_any_member_is_true() {
    let boundary = MapFunctions(HashMap::from([(1u16, 0.0f32), (2u16, 1.0f32)]));
    let mut c = candidate(0x10);
    c.conditions = vec![ctda(0, true, 1.0, 1), ctda(0, false, 1.0, 2)];
    let report = select_package(&[c], GameInstant::default(), &boundary);
    assert_eq!(report.selected, Some(0x10));
}

#[test]
fn and_of_groups_rejects_when_one_group_is_false() {
    let boundary = MapFunctions(HashMap::from([(1u16, 1.0f32), (2u16, 0.0f32)]));
    let mut c = candidate(0x10);
    c.conditions = vec![ctda(0, false, 1.0, 1), ctda(0, false, 1.0, 2)];
    let report = select_package(&[c], GameInstant::default(), &boundary);
    assert_eq!(report.selected, None);
    assert_eq!(report.counters.conditions_false, 1);
}

#[test]
fn a_malformed_ctda_is_unevaluable_not_a_panic() {
    let mut c = candidate(0x10);
    c.conditions = vec![vec![0u8; 4]]; // too short
    let report = select_package(&[c], GameInstant::default(), &NoConditionFunctions);
    assert_eq!(report.selected, None);
    assert_eq!(report.counters.conditions_unevaluable, 1);
}

#[test]
fn empty_candidate_list_is_a_gap() {
    let report = select_package(&[], GameInstant::default(), &NoConditionFunctions);
    assert_eq!(report.selected, None);
    assert_eq!(report.counters.total, 0);
    assert_eq!(report.counters.schedule_gap, 1);
}
