use super::idle_policy::*;
use bevyout_core::actor_animation::PreparedActorIdleDefinition;
use std::cell::RefCell;
use std::collections::BTreeSet;

#[derive(Default)]
struct TestConditions {
    false_ids: BTreeSet<u32>,
    unsupported: bool,
}

impl IdleConditionEvaluator for TestConditions {
    fn evaluate(
        &self,
        conditions: &[Vec<u8>],
        _random_percent: u8,
        _facts: &IdleRuntimeFacts,
    ) -> IdleConditionOutcome {
        if self.unsupported && !conditions.is_empty() {
            return IdleConditionOutcome::Unevaluable;
        }
        if conditions.iter().any(|condition| {
            condition
                .first()
                .is_some_and(|form_id| self.false_ids.contains(&u32::from(*form_id)))
        }) {
            IdleConditionOutcome::False
        } else {
            IdleConditionOutcome::True
        }
    }
}

struct RecordingConditions {
    rolls: RefCell<Vec<u8>>,
}

impl IdleConditionEvaluator for RecordingConditions {
    fn evaluate(
        &self,
        conditions: &[Vec<u8>],
        random_percent: u8,
        _facts: &IdleRuntimeFacts,
    ) -> IdleConditionOutcome {
        self.rolls.borrow_mut().push(random_percent);
        if conditions.iter().any(|condition| condition == &[0]) {
            IdleConditionOutcome::False
        } else {
            IdleConditionOutcome::True
        }
    }
}

fn lifecycle() -> IdleLifecycleFacts {
    IdleLifecycleFacts {
        alive: true,
        loaded: true,
        ..IdleLifecycleFacts::default()
    }
}

fn definition(form_id: u32) -> PreparedActorIdleDefinition {
    PreparedActorIdleDefinition {
        form_id,
        clip_name: Some(format!("idle_{form_id:08x}")),
        group_section: SPECIAL_IDLE_GROUP,
        group_section_raw: SPECIAL_IDLE_GROUP,
        ..PreparedActorIdleDefinition::default()
    }
}

fn package(form_id: u32, flags: u8, ids: &[u32], timer_seconds: f32) -> IdlePackageContext {
    IdlePackageContext {
        form_id,
        general_flags: 0,
        collection: Some(IdlePackageCollection {
            flags,
            timer_seconds,
            animation_form_ids: ids.to_vec(),
        }),
    }
}

fn select(
    authority: &mut IdleAuthority,
    now: f32,
    package: Option<&IdlePackageContext>,
    definitions: &[PreparedActorIdleDefinition],
    evaluator: &dyn IdleConditionEvaluator,
) -> IdleDecision {
    authority.select(
        0x1234,
        now,
        lifecycle(),
        &IdleRuntimeFacts::default(),
        package,
        definitions,
        IdleEvaluationTrigger::BaseIdleLoop,
        evaluator,
    )
}

#[test]
fn lifecycle_gate_rejects_every_unsafe_state() {
    let evaluator = TestConditions::default();
    for mutate in [
        |facts: &mut IdleLifecycleFacts| facts.moving = true,
        |facts: &mut IdleLifecycleFacts| facts.alive = false,
        |facts: &mut IdleLifecycleFacts| facts.ragdolled = true,
        |facts: &mut IdleLifecycleFacts| facts.loaded = false,
        |facts: &mut IdleLifecycleFacts| facts.equipment_transition = true,
    ] {
        let mut facts = lifecycle();
        mutate(&mut facts);
        let mut authority = IdleAuthority::default();
        let result = authority.select(
            1,
            0.0,
            facts,
            &IdleRuntimeFacts::default(),
            None,
            &[definition(1)],
            IdleEvaluationTrigger::BaseIdleLoop,
            &evaluator,
        );
        assert_eq!(result.rejection(), Some(facts.rejection()));
    }
}

#[test]
fn package_no_idle_flag_blocks_package_and_global_selection() {
    let evaluator = TestConditions::default();
    let mut authority = IdleAuthority::default();
    let package = IdlePackageContext {
        form_id: 2,
        general_flags: NO_IDLE_ANIMS_FLAG,
        collection: None,
    };
    assert_eq!(
        select(
            &mut authority,
            0.0,
            Some(&package),
            &[definition(1)],
            &evaluator
        ),
        IdleDecision::Rejected(IdleRejectionReason::NoIdleAnims)
    );
}

#[test]
fn nonempty_package_collection_overrides_global_tree() {
    let evaluator = TestConditions::default();
    let package = package(2, RUN_IN_SEQUENCE_FLAG, &[2], 0.0);
    let mut authority = IdleAuthority::default();
    authority.on_package_entry(Some(2), 0.0, 0.0, true);
    let result = select(
        &mut authority,
        0.0,
        Some(&package),
        &[definition(1), definition(2)],
        &evaluator,
    );
    let IdleDecision::Selected(selection) = result else {
        panic!("expected package selection");
    };
    assert_eq!(selection.form_id, 2);
    assert_eq!(selection.source, IdleSource::Package);
}

#[test]
fn sequence_advances_and_do_once_exhausts_without_restart() {
    let evaluator = TestConditions::default();
    let package = package(2, RUN_IN_SEQUENCE_FLAG | DO_ONCE_FLAG, &[1, 2], 0.0);
    let definitions = [definition(1), definition(2)];
    let mut authority = IdleAuthority::default();
    authority.on_package_entry(Some(2), 0.0, 0.0, true);
    let first = select(
        &mut authority,
        0.0,
        Some(&package),
        &definitions,
        &evaluator,
    );
    let second = select(
        &mut authority,
        0.0,
        Some(&package),
        &definitions,
        &evaluator,
    );
    assert_eq!(
        [first.selected_form_id(), second.selected_form_id()],
        [Some(1), Some(2)]
    );
    assert!(authority.do_once_exhausted);
    assert_eq!(
        select(
            &mut authority,
            0.0,
            Some(&package),
            &definitions,
            &evaluator
        ),
        IdleDecision::Rejected(IdleRejectionReason::DoOnceExhausted)
    );
}

#[test]
fn random_selection_is_stable_for_actor_package_and_epoch() {
    let evaluator = TestConditions::default();
    let package = package(2, 0, &[1, 2, 3], 0.0);
    let definitions = [definition(1), definition(2), definition(3)];
    let mut left = IdleAuthority::default();
    let mut right = IdleAuthority::default();
    left.selection_epoch = 17;
    right.selection_epoch = 17;
    left.on_package_entry(Some(2), 0.0, 0.0, true);
    right.on_package_entry(Some(2), 0.0, 0.0, true);
    assert_eq!(
        select(&mut left, 0.0, Some(&package), &definitions, &evaluator),
        select(&mut right, 0.0, Some(&package), &definitions, &evaluator)
    );
}

#[test]
fn package_timer_is_seconds_from_stationary_entry() {
    let evaluator = TestConditions::default();
    let package = package(2, RUN_IN_SEQUENCE_FLAG, &[1], 3.0);
    let definitions = [definition(1)];
    let mut authority = IdleAuthority::default();
    authority.on_package_entry(Some(2), 0.0, 3.0, true);
    assert_eq!(
        authority.select(
            1,
            2.0,
            lifecycle(),
            &IdleRuntimeFacts::default(),
            Some(&package),
            &definitions,
            IdleEvaluationTrigger::PackageTimer,
            &evaluator,
        ),
        IdleDecision::Rejected(IdleRejectionReason::PackageTimer)
    );
    assert!(matches!(
        authority.select(
            1,
            3.0,
            lifecycle(),
            &IdleRuntimeFacts::default(),
            Some(&package),
            &definitions,
            IdleEvaluationTrigger::PackageTimer,
            &evaluator,
        ),
        IdleDecision::Selected(_)
    ));
}

#[test]
fn global_tree_uses_previous_sibling_order_and_parent_conditions() {
    let evaluator = TestConditions {
        false_ids: [1].into_iter().collect(),
        ..TestConditions::default()
    };
    let mut parent = definition(10);
    parent.clip_name = None;
    let mut first = definition(20);
    first.parent_form_id = Some(10);
    first.previous_sibling_form_id = None;
    first.conditions = vec![vec![1]];
    let mut second = definition(30);
    second.parent_form_id = Some(10);
    second.previous_sibling_form_id = Some(20);
    let mut authority = IdleAuthority::default();
    let result = select(
        &mut authority,
        0.0,
        None,
        &[second, parent, first],
        &evaluator,
    );
    assert_eq!(result.selected_form_id(), Some(30));
    assert_eq!(result.source(), Some(IdleSource::IdleManager));
}

#[test]
fn one_roll_is_reused_for_siblings_and_children_get_a_new_roll() {
    let evaluator = RecordingConditions {
        rolls: RefCell::new(Vec::new()),
    };
    let mut root = definition(1);
    root.clip_name = None;
    let mut branch = definition(2);
    branch.parent_form_id = Some(1);
    branch.clip_name = None;
    let mut rejected_leaf = definition(4);
    rejected_leaf.parent_form_id = Some(2);
    rejected_leaf.conditions = vec![vec![0]];
    let mut selected_leaf = definition(5);
    selected_leaf.parent_form_id = Some(2);
    selected_leaf.previous_sibling_form_id = Some(4);
    let mut authority = IdleAuthority::default();
    assert!(matches!(
        select(
            &mut authority,
            0.0,
            None,
            &[selected_leaf, root, rejected_leaf, branch],
            &evaluator,
        ),
        IdleDecision::Selected(_)
    ));
    let rolls = evaluator.rolls.borrow();
    assert_eq!(rolls.len(), 4);
    assert_eq!(rolls[2], rolls[3]);
    assert_ne!(rolls[1], rolls[2]);
}

#[test]
fn unsupported_conditions_are_rejected_not_true() {
    let evaluator = TestConditions {
        unsupported: true,
        ..TestConditions::default()
    };
    let mut unsupported = definition(1);
    unsupported.conditions = vec![vec![1]];
    let mut authority = IdleAuthority::default();
    assert_eq!(
        select(&mut authority, 0.0, None, &[unsupported], &evaluator),
        IdleDecision::Rejected(IdleRejectionReason::UnsupportedCondition)
    );
}

#[test]
fn replay_delay_blocks_immediate_replay() {
    let evaluator = TestConditions::default();
    let mut idle = definition(1);
    idle.replay_delay_seconds = 5;
    let definitions = [idle];
    let mut authority = IdleAuthority::default();
    assert!(matches!(
        select(&mut authority, 0.0, None, &definitions, &evaluator),
        IdleDecision::Selected(_)
    ));
    authority.schedule_next_evaluation(0.0);
    assert_eq!(
        select(&mut authority, 0.0, None, &definitions, &evaluator),
        IdleDecision::Rejected(IdleRejectionReason::ReplayCooldown)
    );
    assert!(matches!(
        select(&mut authority, 5.0, None, &definitions, &evaluator),
        IdleDecision::Selected(_)
    ));
}

#[test]
fn loop_bounds_are_inclusive_and_zero_zero_plays_once() {
    let evaluator = TestConditions::default();
    let mut idle = definition(1);
    idle.loop_min = 2;
    idle.loop_max = 4;
    let mut authority = IdleAuthority::default();
    let decision = select(&mut authority, 0.0, None, &[idle], &evaluator);
    let IdleDecision::Selected(selection) = decision else {
        panic!("expected selection");
    };
    assert!((2..=4).contains(&selection.loop_count));

    let mut once = definition(2);
    once.loop_min = 0;
    once.loop_max = 0;
    let mut authority = IdleAuthority::default();
    let IdleDecision::Selected(selection) = select(&mut authority, 0.0, None, &[once], &evaluator)
    else {
        panic!("expected selection");
    };
    assert_eq!(selection.loop_count, 1);
}

#[test]
fn unsupported_groups_are_rejected_and_forced_selection_bypasses_conditions_and_cooldown() {
    let evaluator = TestConditions {
        unsupported: true,
        ..TestConditions::default()
    };
    let mut unsupported = definition(1);
    unsupported.group_section = 8;
    unsupported.group_section_raw = 8;
    let mut authority = IdleAuthority::default();
    assert_eq!(
        authority.force_select(
            1,
            0.0,
            lifecycle(),
            &IdleRuntimeFacts::default(),
            &[unsupported],
            1,
            &evaluator,
        ),
        IdleDecision::Rejected(IdleRejectionReason::UnsupportedGroup)
    );

    let mut forced = definition(2);
    forced.conditions = vec![vec![9]];
    forced.replay_delay_seconds = 20;
    let mut authority = IdleAuthority::default();
    authority.replay_cooldowns.insert(2, 20.0);
    let result = authority.force_select(
        1,
        0.0,
        lifecycle(),
        &IdleRuntimeFacts::default(),
        &[forced],
        2,
        &evaluator,
    );
    assert_eq!(result.source(), Some(IdleSource::Forced));
}

#[test]
fn stop_returns_authority_to_base_locomotion_and_retains_last_idle() {
    let evaluator = TestConditions::default();
    let mut authority = IdleAuthority::default();
    assert!(matches!(
        select(&mut authority, 0.0, None, &[definition(1)], &evaluator),
        IdleDecision::Selected(_)
    ));
    authority.stop(Some(IdleRejectionReason::Moving));
    assert_eq!(authority.current_idle_form_id, None);
    assert_eq!(authority.last_idle_form_id, Some(1));
    assert_eq!(authority.last_rejection, Some(IdleRejectionReason::Moving));
}

trait DecisionExt {
    fn source(&self) -> Option<IdleSource>;
}

impl DecisionExt for IdleDecision {
    fn source(&self) -> Option<IdleSource> {
        match self {
            Self::Selected(selection) => Some(selection.source),
            Self::Rejected(_) => None,
        }
    }
}
