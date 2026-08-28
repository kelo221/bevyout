//! Headless lockpicking and terminal hacking (M9 wave 7).
//!
//! Integer millidegrees and basis points only. Domain-separated splitmix64
//! draws live on [`MinigameRngState`]; combat RNG is never used. A session
//! step either mutates the session and any required inventory together, or
//! leaves both unchanged.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::chems::RpgRngState;
use crate::crime::{
    CrimeEvent, CrimeKind, CrimeLedger, CrimeReport, WitnessEvidence, resolve_crime,
};
use crate::item_transaction::{HolderId, ItemInstanceId, ItemLedger, ItemState, TransactionError};
use crate::perception::{TargetClass, TargetId};

pub const BOBBY_PIN_FORM_ID: u32 = 0x0000_000A;
pub const MINIGAME_RNG_REVISION: &str = "m9-minigame-rng-v1";
pub const PICK_ANGLE_MIN_MILLI: i32 = -90_000;
pub const PICK_ANGLE_MAX_MILLI: i32 = 90_000;
pub const CYLINDER_MAX_MILLI: u32 = 90_000;
pub const PIN_STRESS_MAX: u16 = 10_000;
pub const HACKING_MAX_ATTEMPTS: u8 = 4;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MinigameRngDomain {
    LockpickForce,
    HackingBoard,
    HackingDud,
}

impl MinigameRngDomain {
    const fn salt(self) -> u64 {
        match self {
            Self::LockpickForce => 0x4c4f_434b_4652_4301,
            Self::HackingBoard => 0x4841_434b_4252_4402,
            Self::HackingDud => 0x4841_434b_4455_4403,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MinigameRngDraw {
    pub domain: MinigameRngDomain,
    pub index: u32,
    pub value: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MinigameRngState {
    pub revision: String,
    pub inner: RpgRngState,
}

impl Default for MinigameRngState {
    fn default() -> Self {
        Self::from_seed(0)
    }
}

impl MinigameRngState {
    #[must_use]
    pub fn from_seed(seed: u64) -> Self {
        Self {
            revision: MINIGAME_RNG_REVISION.into(),
            inner: RpgRngState::new(seed),
        }
    }

    #[must_use]
    pub fn draw_index(&self) -> u32 {
        self.inner.draw_index
    }

    pub fn draw(&mut self, domain: MinigameRngDomain) -> MinigameRngDraw {
        let mixed = self.inner.next_u64() ^ domain.salt();
        let index = self.inner.draw_index;
        self.inner.draw_index = self.inner.draw_index.saturating_add(1);
        MinigameRngDraw {
            domain,
            index,
            value: (mixed >> 32) as u32,
        }
    }

    pub fn draw_bps(&mut self, domain: MinigameRngDomain, limit_bps: u32) -> MinigameRngDraw {
        let mixed = self.inner.next_u64() ^ domain.salt();
        let index = self.inner.draw_index;
        self.inner.draw_index = self.inner.draw_index.saturating_add(1);
        let limit = u64::from(limit_bps.max(1));
        MinigameRngDraw {
            domain,
            index,
            value: (mixed % limit) as u32,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MinigameSessionId(pub u64);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PickAngleMilliDegrees(pub i32);

impl PickAngleMilliDegrees {
    pub fn new(value: i32) -> Result<Self, MinigameError> {
        if (PICK_ANGLE_MIN_MILLI..=PICK_ANGLE_MAX_MILLI).contains(&value) {
            Ok(Self(value))
        } else {
            Err(MinigameError::PickAngleOutOfRange)
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CylinderAngleMilliDegrees(pub u32);

impl CylinderAngleMilliDegrees {
    pub fn new(value: u32) -> Result<Self, MinigameError> {
        if value <= CYLINDER_MAX_MILLI {
            Ok(Self(value))
        } else {
            Err(MinigameError::CylinderOutOfRange)
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PinStress(pub u16);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LockpickInput {
    SetPickAngle(PickAngleMilliDegrees),
    ApplyTorque { delta_ms: u32 },
    ReleaseTorque,
    ForceLock,
    Cancel,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LockpickPhase {
    Idle,
    Turning,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LockpickConfig {
    pub difficulty: u8,
    pub skill: u8,
    pub sweet_spot_milli: i32,
    pub tolerance_milli: u32,
    pub owner_form_id: Option<u32>,
}

impl LockpickConfig {
    #[must_use]
    pub fn sweet_spot(&self) -> PickAngleMilliDegrees {
        PickAngleMilliDegrees(
            self.sweet_spot_milli
                .clamp(PICK_ANGLE_MIN_MILLI, PICK_ANGLE_MAX_MILLI),
        )
    }

    #[must_use]
    pub fn required_skill(&self) -> u8 {
        self.difficulty.min(100)
    }

    #[must_use]
    pub fn in_sweet_spot(&self, angle: PickAngleMilliDegrees) -> bool {
        let delta = (angle.0 - self.sweet_spot().0).unsigned_abs();
        delta <= self.tolerance_milli
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LockpickSession {
    pub id: MinigameSessionId,
    pub sequence: u32,
    pub config: LockpickConfig,
    pub phase: LockpickPhase,
    pub pick_angle: PickAngleMilliDegrees,
    pub cylinder: CylinderAngleMilliDegrees,
    pub stress: PinStress,
    pub torque_held: bool,
    pub pin_breaks: u32,
    pub last_force_chance_bps: Option<u32>,
    pub last_force_draw: Option<MinigameRngDraw>,
}

impl LockpickSession {
    #[must_use]
    pub fn new(id: MinigameSessionId, config: LockpickConfig) -> Self {
        Self {
            id,
            sequence: 0,
            config,
            phase: LockpickPhase::Idle,
            pick_angle: PickAngleMilliDegrees(0),
            cylinder: CylinderAngleMilliDegrees(0),
            stress: PinStress(0),
            torque_held: false,
            pin_breaks: 0,
            last_force_chance_bps: None,
            last_force_draw: None,
        }
    }

    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(
            self.phase,
            LockpickPhase::Idle | LockpickPhase::Turning | LockpickPhase::Failed
        )
    }

    #[must_use]
    pub fn unlocked(&self) -> bool {
        self.phase == LockpickPhase::Succeeded
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HackingInput {
    GuessWord { index: usize },
    UseBracket { pair: u8 },
    Cancel,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum HackingPhase {
    Idle,
    Succeeded,
    LockedOut,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HackingWord {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BracketPair {
    pub id: u8,
    pub kind: BracketKind,
    pub used: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BracketKind {
    Dud,
    ResetAttempts,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HackingBoard {
    pub words: Vec<HackingWord>,
    pub password_index: usize,
    pub brackets: Vec<BracketPair>,
}

impl HackingBoard {
    #[must_use]
    pub fn password(&self) -> &str {
        &self.words[self.password_index].text
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HackingSession {
    pub id: MinigameSessionId,
    pub sequence: u32,
    pub phase: HackingPhase,
    pub board: HackingBoard,
    pub attempts_remaining: u8,
    pub max_attempts: u8,
    pub last_likeness: Option<u8>,
    pub removed_duds: BTreeSet<usize>,
}

impl HackingSession {
    #[must_use]
    pub fn new(id: MinigameSessionId, board: HackingBoard) -> Self {
        Self {
            id,
            sequence: 0,
            phase: HackingPhase::Idle,
            board,
            attempts_remaining: HACKING_MAX_ATTEMPTS,
            max_attempts: HACKING_MAX_ATTEMPTS,
            last_likeness: None,
            removed_duds: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn is_active(&self) -> bool {
        self.phase == HackingPhase::Idle
    }

    #[must_use]
    pub fn unlocked(&self) -> bool {
        self.phase == HackingPhase::Succeeded
    }

    #[must_use]
    pub fn locked_out(&self) -> bool {
        self.phase == HackingPhase::LockedOut
    }

    pub fn live_words(&self) -> impl Iterator<Item = (usize, &HackingWord)> {
        self.board
            .words
            .iter()
            .enumerate()
            .filter(|(index, _)| !self.removed_duds.contains(index))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MinigameKind {
    Lockpick,
    Hacking,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MinigameCommit {
    pub lock_unlocked: bool,
    pub terminal_unlocked: bool,
    pub terminal_locked_out: bool,
    pub pin_consumed: bool,
    pub crime: Option<CrimeReport>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MinigameError {
    Inactive,
    PickAngleOutOfRange,
    CylinderOutOfRange,
    UnknownWord,
    UnknownBracket,
    BracketUsed,
    NoBobbyPin,
    Transaction(TransactionError),
    SkillTooLow,
}

impl fmt::Display for MinigameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for MinigameError {}

#[must_use]
pub fn force_lock_chance_bps(difficulty: u8, skill: u8) -> u32 {
    let skill = u32::from(skill.min(100));
    let difficulty = u32::from(difficulty.min(100));
    let raw = skill
        .saturating_mul(100)
        .saturating_sub(difficulty.saturating_mul(50))
        .saturating_add(500);
    raw.min(10_000)
}

#[must_use]
pub fn word_likeness(guess: &str, password: &str) -> u8 {
    guess
        .bytes()
        .zip(password.bytes())
        .filter(|(left, right)| left == right)
        .count()
        .min(255) as u8
}

pub fn generate_hacking_board(
    bank: &[String],
    password: &str,
    rng: &mut MinigameRngState,
) -> Result<HackingBoard, MinigameError> {
    let length = password.len();
    let mut words: Vec<String> = bank
        .iter()
        .filter(|word| word.len() == length)
        .cloned()
        .collect();
    if !words.iter().any(|word| word == password) {
        words.push(password.to_string());
    }
    words.sort();
    words.dedup();
    if words.is_empty() {
        return Err(MinigameError::UnknownWord);
    }
    let _draw = rng.draw_bps(MinigameRngDomain::HackingBoard, words.len() as u32);
    let password_index = words.iter().position(|word| word == password).unwrap_or(0);
    Ok(HackingBoard {
        words: words.into_iter().map(|text| HackingWord { text }).collect(),
        password_index,
        brackets: Vec::new(),
    })
}

fn first_player_pin(ledger: &ItemLedger) -> Option<ItemInstanceId> {
    ledger
        .holders()
        .get(&HolderId::Player)?
        .items
        .iter()
        .find(|item| item.base_form_id == BOBBY_PIN_FORM_ID)
        .map(|item| item.id)
}

fn player_pin_count(ledger: &ItemLedger) -> u32 {
    ledger
        .holders()
        .get(&HolderId::Player)
        .map(|holder| {
            holder
                .items
                .iter()
                .filter(|item| item.base_form_id == BOBBY_PIN_FORM_ID)
                .map(|item| item.count)
                .sum()
        })
        .unwrap_or(0)
}

fn consume_one_pin(ledger: &mut ItemLedger) -> Result<ItemInstanceId, MinigameError> {
    let id = first_player_pin(ledger).ok_or(MinigameError::NoBobbyPin)?;
    ledger
        .use_item(HolderId::Player, id)
        .map_err(MinigameError::Transaction)?;
    Ok(id)
}

fn report_trespass(
    ledger: &mut CrimeLedger,
    owner: u32,
    witnesses: &mut [WitnessEvidence],
) -> Option<CrimeReport> {
    let id = CrimeLedger::allocate(TargetId::player(), ledger);
    resolve_crime(
        ledger,
        CrimeEvent {
            id,
            kind: CrimeKind::Trespass,
            victim: TargetId {
                class: TargetClass::Actor,
                form_id: owner,
            },
            item_id: None,
            owner_form_id: Some(owner),
        },
        witnesses,
        None,
    )
}

pub fn step_lockpick(
    session: &mut LockpickSession,
    input: LockpickInput,
    items: &mut ItemLedger,
    rng: &mut MinigameRngState,
    crime: &mut CrimeLedger,
    witnesses: &mut [WitnessEvidence],
) -> Result<MinigameCommit, MinigameError> {
    if !session.is_active() && !matches!(input, LockpickInput::Cancel) {
        return Err(MinigameError::Inactive);
    }
    match input {
        LockpickInput::SetPickAngle(angle) => {
            session.pick_angle = PickAngleMilliDegrees::new(angle.0)?;
            session.sequence = session.sequence.saturating_add(1);
            if session.phase == LockpickPhase::Failed {
                session.phase = LockpickPhase::Idle;
                session.stress = PinStress(0);
                session.cylinder = CylinderAngleMilliDegrees(0);
            }
            Ok(MinigameCommit::default())
        }
        LockpickInput::ReleaseTorque => {
            session.torque_held = false;
            session.sequence = session.sequence.saturating_add(1);
            Ok(MinigameCommit::default())
        }
        LockpickInput::Cancel => {
            session.phase = LockpickPhase::Cancelled;
            session.torque_held = false;
            session.sequence = session.sequence.saturating_add(1);
            Ok(MinigameCommit::default())
        }
        LockpickInput::ForceLock => {
            if session.config.skill < session.config.required_skill()
                && session.config.difficulty > 0
                && session.config.skill == 0
            {
                return Err(MinigameError::SkillTooLow);
            }
            let chance = force_lock_chance_bps(session.config.difficulty, session.config.skill);
            let draw = rng.draw_bps(MinigameRngDomain::LockpickForce, 10_000);
            session.last_force_chance_bps = Some(chance);
            session.last_force_draw = Some(draw);
            session.sequence = session.sequence.saturating_add(1);
            if draw.value < chance {
                session.phase = LockpickPhase::Succeeded;
                let crime_report = session
                    .config
                    .owner_form_id
                    .and_then(|owner| report_trespass(crime, owner, witnesses));
                Ok(MinigameCommit {
                    lock_unlocked: true,
                    crime: crime_report,
                    ..MinigameCommit::default()
                })
            } else {
                Ok(MinigameCommit::default())
            }
        }
        LockpickInput::ApplyTorque { delta_ms } => {
            apply_torque(session, items, crime, witnesses, delta_ms)
        }
    }
}

fn apply_torque(
    session: &mut LockpickSession,
    items: &mut ItemLedger,
    crime: &mut CrimeLedger,
    witnesses: &mut [WitnessEvidence],
    delta_ms: u32,
) -> Result<MinigameCommit, MinigameError> {
    if session.config.skill < session.config.required_skill() && session.config.difficulty >= 100 {
        return Err(MinigameError::SkillTooLow);
    }
    session.torque_held = true;
    session.sequence = session.sequence.saturating_add(1);
    session.phase = LockpickPhase::Turning;
    if session.config.in_sweet_spot(session.pick_angle) {
        let advance = delta_ms.saturating_mul(90);
        session.cylinder = CylinderAngleMilliDegrees(
            session
                .cylinder
                .0
                .saturating_add(advance)
                .min(CYLINDER_MAX_MILLI),
        );
        if session.cylinder.0 >= CYLINDER_MAX_MILLI {
            session.phase = LockpickPhase::Succeeded;
            session.torque_held = false;
            let crime_report = session
                .config
                .owner_form_id
                .and_then(|owner| report_trespass(crime, owner, witnesses));
            return Ok(MinigameCommit {
                lock_unlocked: true,
                crime: crime_report,
                ..MinigameCommit::default()
            });
        }
        return Ok(MinigameCommit::default());
    }
    let delta = (session.pick_angle.0 - session.config.sweet_spot().0).unsigned_abs();
    let gain = ((u64::from(delta.max(1)) * u64::from(delta_ms.max(1))) / 2_000)
        .min(u64::from(PIN_STRESS_MAX));
    let next = u32::from(session.stress.0).saturating_add(gain as u32);
    if next >= u32::from(PIN_STRESS_MAX) {
        consume_one_pin(items)?;
        session.stress = PinStress(0);
        session.cylinder = CylinderAngleMilliDegrees(0);
        session.phase = LockpickPhase::Failed;
        session.pin_breaks = session.pin_breaks.saturating_add(1);
        session.torque_held = false;
        Ok(MinigameCommit {
            pin_consumed: true,
            ..MinigameCommit::default()
        })
    } else {
        session.stress = PinStress(next as u16);
        Ok(MinigameCommit::default())
    }
}

pub fn step_hacking(
    session: &mut HackingSession,
    input: HackingInput,
    rng: &mut MinigameRngState,
) -> Result<MinigameCommit, MinigameError> {
    if !session.is_active() && !matches!(input, HackingInput::Cancel) {
        return Err(MinigameError::Inactive);
    }
    match input {
        HackingInput::Cancel => {
            session.phase = HackingPhase::Cancelled;
            session.sequence = session.sequence.saturating_add(1);
            Ok(MinigameCommit::default())
        }
        HackingInput::GuessWord { index } => {
            if index >= session.board.words.len() || session.removed_duds.contains(&index) {
                return Err(MinigameError::UnknownWord);
            }
            session.sequence = session.sequence.saturating_add(1);
            let guess = &session.board.words[index].text;
            let likeness = word_likeness(guess, session.board.password());
            session.last_likeness = Some(likeness);
            if index == session.board.password_index {
                session.phase = HackingPhase::Succeeded;
                return Ok(MinigameCommit {
                    terminal_unlocked: true,
                    ..MinigameCommit::default()
                });
            }
            session.attempts_remaining = session.attempts_remaining.saturating_sub(1);
            if session.attempts_remaining == 0 {
                session.phase = HackingPhase::LockedOut;
                return Ok(MinigameCommit {
                    terminal_locked_out: true,
                    ..MinigameCommit::default()
                });
            }
            Ok(MinigameCommit::default())
        }
        HackingInput::UseBracket { pair } => {
            let Some(bracket) = session
                .board
                .brackets
                .iter_mut()
                .find(|bracket| bracket.id == pair)
            else {
                return Err(MinigameError::UnknownBracket);
            };
            if bracket.used {
                return Err(MinigameError::BracketUsed);
            }
            bracket.used = true;
            session.sequence = session.sequence.saturating_add(1);
            match bracket.kind {
                BracketKind::ResetAttempts => {
                    session.attempts_remaining = session.max_attempts;
                    Ok(MinigameCommit::default())
                }
                BracketKind::Dud => {
                    let candidates: Vec<usize> = session
                        .board
                        .words
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| {
                            *index != session.board.password_index
                                && !session.removed_duds.contains(index)
                        })
                        .map(|(index, _)| index)
                        .collect();
                    if let Some(chosen) = choose_dud(&candidates, rng) {
                        session.removed_duds.insert(chosen);
                    }
                    Ok(MinigameCommit::default())
                }
            }
        }
    }
}

fn choose_dud(candidates: &[usize], rng: &mut MinigameRngState) -> Option<usize> {
    if candidates.is_empty() {
        return None;
    }
    let draw = rng.draw_bps(MinigameRngDomain::HackingDud, candidates.len() as u32);
    Some(candidates[(draw.value as usize) % candidates.len()])
}

pub fn grant_bobby_pins(
    ledger: &mut ItemLedger,
    count: u32,
) -> Result<ItemInstanceId, MinigameError> {
    if ledger.holders().get(&HolderId::Player).is_none() {
        ledger
            .insert_holder(HolderId::Player, Default::default())
            .map_err(MinigameError::Transaction)?;
    }
    ledger
        .insert_new_item(
            HolderId::Player,
            BOBBY_PIN_FORM_ID,
            count.max(1),
            ItemState::default(),
        )
        .map_err(MinigameError::Transaction)
}

#[must_use]
pub fn bobby_pin_count(ledger: &ItemLedger) -> u32 {
    player_pin_count(ledger)
}

#[must_use]
pub fn saving_blocked(
    lockpick: Option<&LockpickSession>,
    hacking: Option<&HackingSession>,
) -> bool {
    lockpick.is_some_and(LockpickSession::is_active)
        || hacking.is_some_and(HackingSession::is_active)
}

#[cfg(test)]
#[path = "tests/minigames.rs"]
mod tests;
