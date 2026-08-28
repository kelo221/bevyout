use super::*;

fn pins(count: u32) -> ItemLedger {
    let mut ledger = ItemLedger::new();
    grant_bobby_pins(&mut ledger, count).expect("pins");
    ledger
}

fn cfg(difficulty: u8, skill: u8, sweet: i32, tolerance: u32) -> LockpickConfig {
    LockpickConfig {
        difficulty,
        skill,
        sweet_spot_milli: sweet,
        tolerance_milli: tolerance,
        owner_form_id: None,
    }
}

fn step(
    session: &mut LockpickSession,
    input: LockpickInput,
    items: &mut ItemLedger,
    rng: &mut MinigameRngState,
) -> Result<MinigameCommit, MinigameError> {
    let mut crime = CrimeLedger::default();
    step_lockpick(session, input, items, rng, &mut crime, &mut [])
}

#[test]
fn pick_angle_bounds_are_integer_and_rejected_outside() {
    assert!(PickAngleMilliDegrees::new(-90_000).is_ok());
    assert!(PickAngleMilliDegrees::new(90_000).is_ok());
    assert_eq!(
        PickAngleMilliDegrees::new(90_001),
        Err(MinigameError::PickAngleOutOfRange)
    );
}

#[test]
fn zero_tolerance_only_matches_exact_sweet_spot() {
    let config = cfg(25, 25, 15_000, 0);
    assert!(config.in_sweet_spot(PickAngleMilliDegrees(15_000)));
    assert!(!config.in_sweet_spot(PickAngleMilliDegrees(15_001)));
}

#[test]
fn torque_sequence_is_deterministic() {
    let mut first_session = LockpickSession::new(MinigameSessionId(1), cfg(50, 50, 0, 500));
    let mut second_session = first_session.clone();
    let mut first_items = pins(2);
    let mut second_items = pins(2);
    let mut first_rng = MinigameRngState::from_seed(0);
    let mut second_rng = MinigameRngState::from_seed(0);
    for session_items_rng in [
        (&mut first_session, &mut first_items, &mut first_rng),
        (&mut second_session, &mut second_items, &mut second_rng),
    ] {
        let (session, items, rng) = session_items_rng;
        step(
            session,
            LockpickInput::SetPickAngle(PickAngleMilliDegrees(20_000)),
            items,
            rng,
        )
        .unwrap();
        step(
            session,
            LockpickInput::ApplyTorque { delta_ms: 100 },
            items,
            rng,
        )
        .unwrap();
        step(
            session,
            LockpickInput::ApplyTorque { delta_ms: 100 },
            items,
            rng,
        )
        .unwrap();
    }
    assert_eq!(first_session.stress, second_session.stress);
    assert_eq!(first_session.cylinder, second_session.cylinder);
    assert_eq!(first_session.stress, PinStress(2000));
    assert_eq!(first_session.cylinder, CylinderAngleMilliDegrees(0));
}

#[test]
fn in_spot_torque_unlocks_without_pin_break() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(50, 75, 0, 5_000));
    let mut items = pins(2);
    let mut rng = MinigameRngState::from_seed(0);
    step(
        &mut session,
        LockpickInput::SetPickAngle(PickAngleMilliDegrees(0)),
        &mut items,
        &mut rng,
    )
    .unwrap();
    let commit = step(
        &mut session,
        LockpickInput::ApplyTorque { delta_ms: 1_000 },
        &mut items,
        &mut rng,
    )
    .unwrap();
    assert!(commit.lock_unlocked);
    assert!(!commit.pin_consumed);
    assert_eq!(bobby_pin_count(&items), 2);
    assert_eq!(rng.draw_index(), 0);
}

#[test]
fn rejected_angle_does_not_draw_rng() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(50, 50, 0, 1_000));
    let mut items = pins(2);
    let mut rng = MinigameRngState::from_seed(0);
    let error = step(
        &mut session,
        LockpickInput::SetPickAngle(PickAngleMilliDegrees(180_000)),
        &mut items,
        &mut rng,
    )
    .unwrap_err();
    assert_eq!(error, MinigameError::PickAngleOutOfRange);
    assert_eq!(rng.draw_index(), 0);
}

#[test]
fn pin_break_consumes_exactly_one_canonical_pin() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(75, 10, 0, 100));
    let mut items = pins(2);
    let mut rng = MinigameRngState::from_seed(0);
    step(
        &mut session,
        LockpickInput::SetPickAngle(PickAngleMilliDegrees(80_000)),
        &mut items,
        &mut rng,
    )
    .unwrap();
    let commit = step(
        &mut session,
        LockpickInput::ApplyTorque { delta_ms: 1_000 },
        &mut items,
        &mut rng,
    )
    .unwrap();
    assert!(commit.pin_consumed);
    assert!(!commit.lock_unlocked);
    assert_eq!(bobby_pin_count(&items), 1);
}

#[test]
fn cancel_does_not_unlock_or_consume() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(50, 50, 0, 1_000));
    let mut items = pins(2);
    let mut rng = MinigameRngState::from_seed(0);
    step(
        &mut session,
        LockpickInput::SetPickAngle(PickAngleMilliDegrees(0)),
        &mut items,
        &mut rng,
    )
    .unwrap();
    step(
        &mut session,
        LockpickInput::ApplyTorque { delta_ms: 100 },
        &mut items,
        &mut rng,
    )
    .unwrap();
    step(&mut session, LockpickInput::Cancel, &mut items, &mut rng).unwrap();
    assert_eq!(session.phase, LockpickPhase::Cancelled);
    assert!(!session.unlocked());
    assert_eq!(bobby_pin_count(&items), 2);
}

#[test]
fn force_lock_chance_is_inspectable() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(100, 1, 0, 0));
    let mut items = pins(2);
    let mut rng = MinigameRngState::from_seed(0);
    step(&mut session, LockpickInput::ForceLock, &mut items, &mut rng).unwrap();
    assert_eq!(session.last_force_chance_bps, Some(500));
    assert_eq!(rng.draw_index(), 1);
    assert!(!session.unlocked());
}

#[test]
fn easy_force_lock_unlocks_without_pin() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(0, 100, 0, 0));
    let mut items = pins(2);
    let mut rng = MinigameRngState::from_seed(0);
    let commit = step(&mut session, LockpickInput::ForceLock, &mut items, &mut rng).unwrap();
    assert!(commit.lock_unlocked);
    assert_eq!(bobby_pin_count(&items), 2);
    assert_eq!(rng.draw_index(), 1);
}

#[test]
fn likeness_is_position_exact() {
    assert_eq!(word_likeness("PASS", "PASS"), 4);
    assert_eq!(word_likeness("WORD", "PASS"), 0);
    assert_eq!(word_likeness("PAXX", "PASS"), 2);
}

#[test]
fn generated_board_has_one_password_and_equal_lengths() {
    let mut rng = MinigameRngState::from_seed(11);
    let board = generate_hacking_board(
        &[
            "VENT".into(),
            "DOOR".into(),
            "LOCK".into(),
            "SAFE".into(),
            "KEYS".into(),
        ],
        "VENT",
        &mut rng,
    )
    .unwrap();
    assert_eq!(
        board
            .words
            .iter()
            .filter(|word| word.text == "VENT")
            .count(),
        1
    );
    assert!(board.words.iter().all(|word| word.text.len() == 4));
    assert_eq!(board.password(), "VENT");
    assert!(rng.draw_index() > 0);
}

#[test]
fn fourth_failed_word_locks_out() {
    let board = HackingBoard {
        words: ["PASS", "WORD", "XXXX", "DUDY"]
            .into_iter()
            .map(|text| HackingWord {
                text: text.to_string(),
            })
            .collect(),
        password_index: 0,
        brackets: Vec::new(),
    };
    let mut session = HackingSession::new(MinigameSessionId(2), board);
    let mut rng = MinigameRngState::from_seed(0);
    for index in [1, 2, 3, 1] {
        step_hacking(&mut session, HackingInput::GuessWord { index }, &mut rng).unwrap();
    }
    assert!(session.locked_out());
    assert_eq!(session.attempts_remaining, 0);
}

#[test]
fn brackets_are_single_use_and_reset_does_not_exceed_max() {
    let board = HackingBoard {
        words: ["PASS", "WORD", "XXXX"]
            .into_iter()
            .map(|text| HackingWord {
                text: text.to_string(),
            })
            .collect(),
        password_index: 0,
        brackets: vec![
            BracketPair {
                id: 1,
                kind: BracketKind::Dud,
                used: false,
            },
            BracketPair {
                id: 2,
                kind: BracketKind::ResetAttempts,
                used: false,
            },
        ],
    };
    let mut session = HackingSession::new(MinigameSessionId(3), board);
    let mut rng = MinigameRngState::from_seed(1);
    step_hacking(&mut session, HackingInput::GuessWord { index: 1 }, &mut rng).unwrap();
    assert_eq!(session.attempts_remaining, 3);
    step_hacking(&mut session, HackingInput::UseBracket { pair: 2 }, &mut rng).unwrap();
    assert_eq!(session.attempts_remaining, 4);
    assert_eq!(
        step_hacking(&mut session, HackingInput::UseBracket { pair: 2 }, &mut rng),
        Err(MinigameError::BracketUsed)
    );
}

#[test]
fn owned_unlock_reports_trespass_once() {
    let mut session = LockpickSession::new(
        MinigameSessionId(4),
        LockpickConfig {
            owner_form_id: Some(0x1A2B3),
            ..cfg(0, 100, 0, 0)
        },
    );
    let mut items = pins(1);
    let mut rng = MinigameRngState::from_seed(0);
    let mut crime = CrimeLedger::default();
    let mut witnesses = [WitnessEvidence {
        witness: TargetId {
            class: TargetClass::Actor,
            form_id: 0x41600,
        },
        has_line_of_sight: true,
        distance_mm: 1_000,
        alive: true,
        enabled: true,
        hostile_to_victim: false,
    }];
    let commit = step_lockpick(
        &mut session,
        LockpickInput::ForceLock,
        &mut items,
        &mut rng,
        &mut crime,
        &mut witnesses,
    )
    .unwrap();
    assert!(commit.lock_unlocked);
    assert!(commit.crime.is_some());
    assert_eq!(crime.bounty, crate::crime::TRESPASS_BOUNTY);
    assert_eq!(crime.karma, crate::crime::TRESPASS_KARMA);
}

#[test]
fn saving_is_blocked_only_while_active() {
    let mut session = LockpickSession::new(MinigameSessionId(1), cfg(50, 50, 0, 1_000));
    assert!(saving_blocked(Some(&session), None));
    session.phase = LockpickPhase::Cancelled;
    assert!(!saving_blocked(Some(&session), None));
}
