//! Pure AI package selection policy (issue #193, M4 package-runtime wave).
//!
//! Given an actor's prepared package list already in authored priority order
//! (`ActorBlueprint::package_form_ids`, unchanged by this module), pick the
//! first package that is currently *eligible*: a supported package type,
//! whose `PSDT` schedule matches a deterministic game-time source, and whose
//! `CTDA` conditions evaluate true through the M4 condition boundary. Every
//! higher-priority package that was skipped carries a deterministic
//! rejection reason so the `showpackages` surface can explain *why* it lost.
//!
//! This module is deliberately std/serde-only (no Bevy imports), mirroring
//! `vsa::prepare::package_catalog`'s own module doc comment, so it compiles
//! verbatim into `tests/features.rs` via `#[path]`. The thin Bevy console
//! adapter (`viewer::console::ai_package_commands`) maps the prepared
//! `PreparedPackageEntry`/`PackageScheduleInput` values onto the plain input
//! types below -- exactly the `PackageInput`-mirrors-`PackageRecord` seam
//! `package_catalog.rs` already established.
//!
//! Condition evaluation routes through a caller-supplied
//! [`ConditionFunctions`] boundary: this module decodes the `CTDA` header
//! (operator, comparison value, function index, params) and applies the
//! comparison, but it does **not** implement any GECK function. A function
//! index the boundary does not support returns `None`, routing that function
//! to #15's GECK registry and leaving the condition *unevaluable* here -- no
//! condition VM grows in this wave.

use serde::{Deserialize, Serialize};

/// Minimum `CTDA` payload this decoder reads: operator byte, 3 pad, 4-byte
/// comparison value, 2-byte function index, 2 pad, then two 4-byte params.
const CTDA_MIN_LEN: usize = 20;

/// Deterministic game-time source for `PSDT` schedule matching. `hour` is in
/// `0.0..24.0`; `month` is `0..=11`; `day_of_week` is `0..=6`; `date` is the
/// day of month `1..=31`. FO3 `PSDT` "any" sentinels live on the schedule
/// fields, not on the instant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameInstant {
    pub month: i8,
    pub day_of_week: i8,
    pub date: u8,
    pub hour: f32,
}

impl Default for GameInstant {
    /// A neutral instant (noon) used when no explicit game clock is supplied.
    /// Deterministic so `showpackages` output is reproducible.
    fn default() -> Self {
        Self {
            month: 0,
            day_of_week: 0,
            date: 1,
            hour: 12.0,
        }
    }
}

/// Plain mirror of `PackageScheduleInput` (`PSDT`). `-1`/`0` sentinels mean
/// "any" per fopdoc's Fallout3 PACK page; `time == -1` is an unscheduled
/// package (always in schedule); `duration` is in minutes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageSchedule {
    pub month: i8,
    pub day_of_week: i8,
    pub date: u8,
    pub time: i8,
    pub duration: i32,
}

/// Whether a package's schedule admits the given instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleMatch {
    /// No `PSDT`, or an unscheduled (`time == -1`) package -- always on.
    Unscheduled,
    /// The instant falls inside the schedule window (date/day/month gates
    /// pass and the hour is within `[time, time + duration)` with
    /// past-midnight wraparound).
    InWindow,
    /// A gate or the hour window rejected the instant.
    OutOfWindow,
}

impl PackageSchedule {
    /// Evaluates this schedule against `now`. Deterministic; the only source
    /// of nondeterminism a schedule could have -- the current time -- is an
    /// explicit input.
    #[must_use]
    pub fn evaluate(&self, now: GameInstant) -> ScheduleMatch {
        if self.time < 0 {
            return ScheduleMatch::Unscheduled;
        }
        if self.month >= 0 && self.month != now.month {
            return ScheduleMatch::OutOfWindow;
        }
        if self.date > 0 && self.date != now.date {
            return ScheduleMatch::OutOfWindow;
        }
        if self.day_of_week >= 0 && self.day_of_week != now.day_of_week {
            return ScheduleMatch::OutOfWindow;
        }
        let start = f32::from(self.time);
        // A non-positive duration means an open-ended start; treat it as
        // active from `start` to the end of the day rather than an empty
        // window (fopdoc leaves 0-duration packages "until re-evaluated").
        let span_hours = if self.duration <= 0 {
            24.0
        } else {
            self.duration as f32 / 60.0
        };
        let end = start + span_hours;
        let in_window = if end <= 24.0 {
            now.hour >= start && now.hour < end
        } else {
            // Past-midnight wraparound: active from `start` to 24:00 and from
            // 00:00 to `end - 24`.
            now.hour >= start || now.hour < (end - 24.0)
        };
        if in_window {
            ScheduleMatch::InWindow
        } else {
            ScheduleMatch::OutOfWindow
        }
    }
}

/// The M4 condition boundary: the caller resolves a `CTDA` function index and
/// its two params to the function's runtime value, or returns `None` to route
/// the function to #15's GECK registry (unevaluable in this wave). This trait
/// is the *only* way conditions reach game state -- this module never
/// implements a GECK function itself.
pub trait ConditionFunctions {
    fn evaluate(&self, function_index: u16, param1: u32, param2: u32) -> Option<f32>;
}

/// A boundary that supports no functions -- every condition is unevaluable.
/// The default the console surface uses until an evaluator is wired.
pub struct NoConditionFunctions;

impl ConditionFunctions for NoConditionFunctions {
    fn evaluate(&self, _function_index: u16, _param1: u32, _param2: u32) -> Option<f32> {
        None
    }
}

/// The comparison operator encoded in the top three bits of a `CTDA`'s first
/// byte (fopdoc's Fallout3 CTDA subrecord).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompareOp {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

impl CompareOp {
    fn from_type_byte(byte: u8) -> Option<Self> {
        match byte >> 5 {
            0 => Some(Self::Equal),
            1 => Some(Self::NotEqual),
            2 => Some(Self::Greater),
            3 => Some(Self::GreaterOrEqual),
            4 => Some(Self::Less),
            5 => Some(Self::LessOrEqual),
            _ => None,
        }
    }

    fn apply(self, lhs: f32, rhs: f32) -> bool {
        match self {
            Self::Equal => lhs == rhs,
            Self::NotEqual => lhs != rhs,
            Self::Greater => lhs > rhs,
            Self::GreaterOrEqual => lhs >= rhs,
            Self::Less => lhs < rhs,
            Self::LessOrEqual => lhs <= rhs,
        }
    }
}

/// One decoded `CTDA`: the OR flag chains this condition into an OR group with
/// the following one (fopdoc: bit 0 of the type byte).
struct DecodedCondition {
    operator: CompareOp,
    or_with_next: bool,
    comparison_value: f32,
    function_index: u16,
    param1: u32,
    param2: u32,
}

fn decode_condition(bytes: &[u8]) -> Option<DecodedCondition> {
    if bytes.len() < CTDA_MIN_LEN {
        return None;
    }
    let type_byte = bytes[0];
    let operator = CompareOp::from_type_byte(type_byte)?;
    let or_with_next = type_byte & 0x01 != 0;
    let comparison_value = f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let function_index = u16::from_le_bytes([bytes[8], bytes[9]]);
    let param1 = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
    let param2 = u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    Some(DecodedCondition {
        operator,
        or_with_next,
        comparison_value,
        function_index,
        param1,
        param2,
    })
}

/// The truth of a single condition, or that it cannot be decided here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionOutcome {
    True,
    False,
    /// The `CTDA` was malformed, or its function index is not supported by
    /// the boundary (routed to #15).
    Unevaluable,
}

fn evaluate_one(
    condition: &DecodedCondition,
    boundary: &dyn ConditionFunctions,
) -> ConditionOutcome {
    match boundary.evaluate(condition.function_index, condition.param1, condition.param2) {
        Some(value) => {
            if condition.operator.apply(value, condition.comparison_value) {
                ConditionOutcome::True
            } else {
                ConditionOutcome::False
            }
        }
        None => ConditionOutcome::Unevaluable,
    }
}

fn finish_group(has_true: bool, has_unevaluable: bool) -> ConditionOutcome {
    if has_true {
        ConditionOutcome::True
    } else if has_unevaluable {
        ConditionOutcome::Unevaluable
    } else {
        ConditionOutcome::False
    }
}

/// Evaluates a package's full `CTDA` list. Conditions AND together; the OR
/// flag chains consecutive conditions into an OR group. A group is `True` if
/// any member is `True`, `Unevaluable` if no member is `True` but some member
/// is `Unevaluable`, else `False`. The overall result is `False` if any group
/// is `False`, otherwise `Unevaluable` if any group is `Unevaluable`, else
/// `True`. An empty list is vacuously `True`.
#[must_use]
pub fn evaluate_conditions(
    conditions: &[Vec<u8>],
    boundary: &dyn ConditionFunctions,
) -> ConditionOutcome {
    let mut any_group_unevaluable = false;
    let mut group_has_true = false;
    let mut group_has_unevaluable = false;
    let mut group_open = false;

    for raw in conditions {
        group_open = true;
        let (member, or_with_next) = match decode_condition(raw) {
            Some(decoded) => (evaluate_one(&decoded, boundary), decoded.or_with_next),
            None => (ConditionOutcome::Unevaluable, false),
        };
        match member {
            ConditionOutcome::True => group_has_true = true,
            ConditionOutcome::Unevaluable => group_has_unevaluable = true,
            ConditionOutcome::False => {}
        }
        if !or_with_next {
            match finish_group(group_has_true, group_has_unevaluable) {
                ConditionOutcome::False => return ConditionOutcome::False,
                ConditionOutcome::Unevaluable => any_group_unevaluable = true,
                ConditionOutcome::True => {}
            }
            group_has_true = false;
            group_has_unevaluable = false;
            group_open = false;
        }
    }
    // A trailing OR group (last condition had the OR flag set) still closes.
    if group_open {
        match finish_group(group_has_true, group_has_unevaluable) {
            ConditionOutcome::False => return ConditionOutcome::False,
            ConditionOutcome::Unevaluable => any_group_unevaluable = true,
            ConditionOutcome::True => {}
        }
    }
    if any_group_unevaluable {
        ConditionOutcome::Unevaluable
    } else {
        ConditionOutcome::True
    }
}

/// One package to evaluate, already in authored priority order. Mirrors the
/// prepared `PreparedPackageEntry` fields selection actually reads.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackageCandidate {
    pub form_id: u32,
    pub package_type: u8,
    pub schedule: Option<PackageSchedule>,
    pub conditions: Vec<Vec<u8>>,
}

/// Why a package was not the selected one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectionReason {
    /// `package_type` above the highest fopdoc-documented FO3 value.
    UnsupportedType,
    /// The `PSDT` schedule window excludes the current instant.
    OutOfSchedule,
    /// `CTDA` conditions evaluated false.
    ConditionsFalse,
    /// `CTDA` conditions could not be decided (routed to #15).
    ConditionsUnevaluable,
    /// Eligible, but a higher-priority package was already selected.
    LowerPriority,
}

impl RejectionReason {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::UnsupportedType => "unsupported-type",
            Self::OutOfSchedule => "out-of-schedule",
            Self::ConditionsFalse => "conditions-false",
            Self::ConditionsUnevaluable => "conditions-unevaluable",
            Self::LowerPriority => "lower-priority",
        }
    }
}

/// The per-package outcome of a selection pass, in priority order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateOutcome {
    Selected,
    Rejected(RejectionReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageEvaluation {
    pub form_id: u32,
    pub outcome: CandidateOutcome,
}

/// Deterministic diagnostic counters over one selection pass, following the
/// prepared catalog's aggregate-counter convention.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SelectionCounters {
    pub total: usize,
    pub unsupported_type: usize,
    pub out_of_schedule: usize,
    pub conditions_false: usize,
    pub conditions_unevaluable: usize,
    /// No package was eligible at this instant (a genuine schedule/condition
    /// gap). `1` when `selected` is `None`, else `0`.
    pub schedule_gap: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionReport {
    /// The selected package FormID, or `None` for a schedule gap.
    pub selected: Option<u32>,
    /// Per-package outcomes in authored priority order.
    pub evaluations: Vec<PackageEvaluation>,
    pub counters: SelectionCounters,
}

/// Highest FO3-documented `PKDT.type` value, kept in lockstep with
/// `package_catalog::MAX_KNOWN_PACKAGE_TYPE` (fopdoc Fallout3 PACK page).
pub const MAX_KNOWN_PACKAGE_TYPE: u8 = 16;

/// Evaluates `candidates` (already in authored priority order) at `now`,
/// selecting the first eligible package. Every candidate is reported: the
/// ones before the selected package with their concrete rejection reason, the
/// selected one, and any after it as `LowerPriority`. Deterministic -- same
/// inputs always yield the same report.
#[must_use]
pub fn select_package(
    candidates: &[PackageCandidate],
    now: GameInstant,
    boundary: &dyn ConditionFunctions,
) -> SelectionReport {
    let mut counters = SelectionCounters {
        total: candidates.len(),
        ..SelectionCounters::default()
    };
    let mut selected: Option<u32> = None;
    let mut evaluations = Vec::with_capacity(candidates.len());

    for candidate in candidates {
        if selected.is_some() {
            evaluations.push(PackageEvaluation {
                form_id: candidate.form_id,
                outcome: CandidateOutcome::Rejected(RejectionReason::LowerPriority),
            });
            continue;
        }
        if candidate.package_type > MAX_KNOWN_PACKAGE_TYPE {
            counters.unsupported_type += 1;
            evaluations.push(PackageEvaluation {
                form_id: candidate.form_id,
                outcome: CandidateOutcome::Rejected(RejectionReason::UnsupportedType),
            });
            continue;
        }
        let schedule_match = candidate
            .schedule
            .map_or(ScheduleMatch::Unscheduled, |schedule| {
                schedule.evaluate(now)
            });
        if schedule_match == ScheduleMatch::OutOfWindow {
            counters.out_of_schedule += 1;
            evaluations.push(PackageEvaluation {
                form_id: candidate.form_id,
                outcome: CandidateOutcome::Rejected(RejectionReason::OutOfSchedule),
            });
            continue;
        }
        match evaluate_conditions(&candidate.conditions, boundary) {
            ConditionOutcome::True => {
                selected = Some(candidate.form_id);
                evaluations.push(PackageEvaluation {
                    form_id: candidate.form_id,
                    outcome: CandidateOutcome::Selected,
                });
            }
            ConditionOutcome::False => {
                counters.conditions_false += 1;
                evaluations.push(PackageEvaluation {
                    form_id: candidate.form_id,
                    outcome: CandidateOutcome::Rejected(RejectionReason::ConditionsFalse),
                });
            }
            ConditionOutcome::Unevaluable => {
                counters.conditions_unevaluable += 1;
                evaluations.push(PackageEvaluation {
                    form_id: candidate.form_id,
                    outcome: CandidateOutcome::Rejected(RejectionReason::ConditionsUnevaluable),
                });
            }
        }
    }
    if selected.is_none() {
        counters.schedule_gap = 1;
    }
    SelectionReport {
        selected,
        evaluations,
        counters,
    }
}

#[cfg(test)]
mod tests {
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
    fn ctda(
        op_bits: u8,
        or_with_next: bool,
        comparison_value: f32,
        function_index: u16,
    ) -> Vec<u8> {
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
}
