//! Pure authored Fallout IDLE selection and per-actor authority.
//!
//! This module deliberately has no Bevy dependency. It owns lifecycle gates,
//! package collection traversal, deterministic random/loop choices, replay
//! cooldowns, and stable rejection labels. The runtime adapter supplies facts
//! and a CTDA evaluator at the boundary; it does not grow a second condition
//! decoder here.

use std::collections::{BTreeMap, BTreeSet};

use bevyout_core::actor_animation::PreparedActorIdleDefinition;
use serde::{Deserialize, Serialize};

pub const SPECIAL_IDLE_GROUP: u8 = 7;
pub const WHOLE_BODY_GROUP: u8 = 20;
pub const NO_IDLE_ANIMS_FLAG: u32 = 0x0100_0000;
pub const RUN_IN_SEQUENCE_FLAG: u8 = 0x01;
pub const UNKNOWN_COLLECTION_FLAG: u8 = 0x02;
pub const DO_ONCE_FLAG: u8 = 0x04;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdleSource {
    #[default]
    IdleManager,
    Package,
    Forced,
}

impl IdleSource {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Package => "package",
            Self::IdleManager => "idle_manager",
            Self::Forced => "forced",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IdleEvaluationTrigger {
    PackageTimer,
    #[default]
    BaseIdleLoop,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdleRejectionReason {
    InvalidLifecycle,
    Moving,
    Dead,
    Ragdolled,
    Unloaded,
    EquipmentTransition,
    PackageTransition,
    NoIdleAnims,
    PackageTimer,
    NotDue,
    ReplayCooldown,
    DoOnceExhausted,
    UnsupportedGroup,
    MissingIdle,
    MissingClip,
    ConditionsFalse,
    UnsupportedCondition,
    NoEligibleIdle,
    UnknownCollectionFlag,
}

impl IdleRejectionReason {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::InvalidLifecycle => "invalid_lifecycle",
            Self::Moving => "moving",
            Self::Dead => "dead",
            Self::Ragdolled => "ragdolled",
            Self::Unloaded => "unloaded",
            Self::EquipmentTransition => "equipment_transition",
            Self::PackageTransition => "package_transition",
            Self::NoIdleAnims => "no_idle_anims",
            Self::PackageTimer => "package_timer",
            Self::NotDue => "not_due",
            Self::ReplayCooldown => "replay_cooldown",
            Self::DoOnceExhausted => "do_once_exhausted",
            Self::UnsupportedGroup => "unsupported_group",
            Self::MissingIdle => "missing_idle",
            Self::MissingClip => "missing_clip",
            Self::ConditionsFalse => "conditions_false",
            Self::UnsupportedCondition => "unsupported_condition",
            Self::NoEligibleIdle => "no_eligible_idle",
            Self::UnknownCollectionFlag => "unknown_collection_flag",
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        [
            Self::InvalidLifecycle,
            Self::Moving,
            Self::Dead,
            Self::Ragdolled,
            Self::Unloaded,
            Self::EquipmentTransition,
            Self::PackageTransition,
            Self::NoIdleAnims,
            Self::PackageTimer,
            Self::NotDue,
            Self::ReplayCooldown,
            Self::DoOnceExhausted,
            Self::UnsupportedGroup,
            Self::MissingIdle,
            Self::MissingClip,
            Self::ConditionsFalse,
            Self::UnsupportedCondition,
            Self::NoEligibleIdle,
            Self::UnknownCollectionFlag,
        ]
        .into_iter()
        .find(|reason| reason.label() == label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleLifecycleFacts {
    pub moving: bool,
    pub alive: bool,
    pub ragdolled: bool,
    pub loaded: bool,
    pub equipment_transition: bool,
}

impl Default for IdleLifecycleFacts {
    fn default() -> Self {
        Self {
            moving: false,
            alive: true,
            ragdolled: false,
            loaded: true,
            equipment_transition: false,
        }
    }
}

impl IdleLifecycleFacts {
    #[must_use]
    pub const fn special_idle_eligible(self) -> bool {
        self.alive && self.loaded && !self.moving && !self.ragdolled && !self.equipment_transition
    }

    #[must_use]
    pub const fn rejection(self) -> IdleRejectionReason {
        if !self.alive {
            IdleRejectionReason::Dead
        } else if !self.loaded {
            IdleRejectionReason::Unloaded
        } else if self.ragdolled {
            IdleRejectionReason::Ragdolled
        } else if self.equipment_transition {
            IdleRejectionReason::EquipmentTransition
        } else if self.moving {
            IdleRejectionReason::Moving
        } else {
            IdleRejectionReason::InvalidLifecycle
        }
    }
}

/// Runtime facts exposed to supported IDLE CTDA functions. No ECS or entity
/// identifiers are allowed through this boundary.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IdleRuntimeFacts {
    pub weapon_out: bool,
    pub factions: BTreeSet<u32>,
    pub equipped_item_form_ids: BTreeSet<u32>,
    pub last_idle_played: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IdlePackageCollection {
    pub flags: u8,
    pub timer_seconds: f32,
    pub animation_form_ids: Vec<u32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IdlePackageContext {
    pub form_id: u32,
    pub general_flags: u32,
    pub collection: Option<IdlePackageCollection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleConditionOutcome {
    True,
    False,
    Unevaluable,
}

/// The runtime adapter implements this with the shared package CTDA boundary
/// (`ai::selection::evaluate_conditions`). Keeping the callback here means
/// this policy remains std/serde-only and the package OR/operator semantics
/// cannot silently fork.
pub trait IdleConditionEvaluator {
    fn evaluate(
        &self,
        conditions: &[Vec<u8>],
        random_percent: u8,
        facts: &IdleRuntimeFacts,
    ) -> IdleConditionOutcome;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdleSelection {
    pub form_id: u32,
    pub source: IdleSource,
    pub loop_count: u8,
    pub random_percent: u8,
    pub diagnostic: Option<IdleRejectionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdleDecision {
    Selected(IdleSelection),
    Rejected(IdleRejectionReason),
}

#[allow(dead_code)]
impl IdleDecision {
    #[must_use]
    pub const fn selected_form_id(&self) -> Option<u32> {
        match self {
            Self::Selected(selection) => Some(selection.form_id),
            Self::Rejected(_) => None,
        }
    }

    #[must_use]
    pub const fn rejection(&self) -> Option<IdleRejectionReason> {
        match self {
            Self::Selected(_) => None,
            Self::Rejected(reason) => Some(*reason),
        }
    }
}

/// Per-actor authority for authored special idles. The runtime component owns
/// one value; this type owns no Bevy entities, asset handles, or graph state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IdleAuthority {
    pub selection_epoch: u64,
    pub current_idle_form_id: Option<u32>,
    pub last_idle_form_id: Option<u32>,
    pub next_eligible_evaluation_seconds: f32,
    pub replay_cooldowns: BTreeMap<u32, f32>,
    pub active_package_form_id: Option<u32>,
    pub package_collection_cursor: usize,
    pub do_once_exhausted: bool,
    pub do_once_played: BTreeSet<u32>,
    pub source: IdleSource,
    pub last_rejection: Option<IdleRejectionReason>,
    pub package_stationary_since_seconds: Option<f32>,
}

impl IdleAuthority {
    /// Starts/restarts package collection state. `timer_seconds` is in game
    /// seconds and begins only when the actor is stationary.
    pub fn on_package_entry(
        &mut self,
        package_form_id: Option<u32>,
        now_seconds: f32,
        timer_seconds: f32,
        stationary: bool,
    ) {
        if self.active_package_form_id != package_form_id {
            self.active_package_form_id = package_form_id;
            self.package_collection_cursor = 0;
            self.do_once_exhausted = false;
            self.do_once_played.clear();
            self.package_stationary_since_seconds = None;
        }
        if stationary && self.package_stationary_since_seconds.is_none() {
            self.package_stationary_since_seconds = Some(now_seconds);
            self.next_eligible_evaluation_seconds = now_seconds + timer_seconds.max(0.0);
        } else if !stationary {
            self.package_stationary_since_seconds = None;
        }
    }

    /// Marks the stationary transition used by global base-idle evaluation.
    pub fn on_stationary_entry(&mut self, stationary: bool, now_seconds: f32) {
        if stationary && self.package_stationary_since_seconds.is_none() {
            self.package_stationary_since_seconds = Some(now_seconds);
            self.next_eligible_evaluation_seconds = now_seconds;
        } else if !stationary {
            self.package_stationary_since_seconds = None;
        }
    }

    pub fn tick(&mut self, now_seconds: f32) {
        self.replay_cooldowns
            .retain(|_, eligible_at| *eligible_at > now_seconds);
    }

    #[must_use]
    pub fn cooldown_remaining(&self, form_id: u32, now_seconds: f32) -> f32 {
        self.replay_cooldowns
            .get(&form_id)
            .map_or(0.0, |eligible_at| (*eligible_at - now_seconds).max(0.0))
    }

    pub fn schedule_next_evaluation(&mut self, now_seconds: f32) {
        self.schedule_evaluation_after(now_seconds, 1.0);
    }

    pub fn schedule_evaluation_after(&mut self, now_seconds: f32, delay_seconds: f32) {
        let delay = if delay_seconds.is_finite() && delay_seconds > 0.0 {
            delay_seconds
        } else {
            1.0
        };
        self.next_eligible_evaluation_seconds = now_seconds + delay;
    }

    pub fn stop(&mut self, reason: Option<IdleRejectionReason>) {
        if self.current_idle_form_id.is_some() {
            self.last_idle_form_id = self.current_idle_form_id;
        }
        self.current_idle_form_id = None;
        self.source = IdleSource::IdleManager;
        self.last_rejection = reason;
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn select(
        &mut self,
        actor_form_id: u32,
        now_seconds: f32,
        lifecycle: IdleLifecycleFacts,
        facts: &IdleRuntimeFacts,
        package: Option<&IdlePackageContext>,
        global_definitions: &[PreparedActorIdleDefinition],
        trigger: IdleEvaluationTrigger,
        evaluator: &dyn IdleConditionEvaluator,
    ) -> IdleDecision {
        self.tick(now_seconds);
        if !lifecycle.special_idle_eligible() {
            return self.reject(lifecycle.rejection());
        }
        let package_collection = package
            .and_then(|package| package.collection.as_ref())
            .filter(|collection| !collection.animation_form_ids.is_empty());
        if package.is_some_and(|package| package.general_flags & NO_IDLE_ANIMS_FLAG != 0) {
            return self.reject(IdleRejectionReason::NoIdleAnims);
        }
        let due = now_seconds + f32::EPSILON >= self.next_eligible_evaluation_seconds;
        if !due
            && self
                .current_idle_form_id
                .is_some_and(|form_id| self.cooldown_remaining(form_id, now_seconds) > 0.0)
        {
            return self.reject(IdleRejectionReason::ReplayCooldown);
        }
        if let Some(collection) = package_collection {
            if !due && trigger != IdleEvaluationTrigger::Retry {
                return self.reject(IdleRejectionReason::PackageTimer);
            }
            let package = package.expect("collection implies package");
            return self.select_package(
                actor_form_id,
                now_seconds,
                package,
                collection,
                global_definitions,
                facts,
                evaluator,
            );
        }
        if trigger == IdleEvaluationTrigger::PackageTimer {
            return self.reject(IdleRejectionReason::NotDue);
        }
        if !due && trigger != IdleEvaluationTrigger::Retry {
            return self.reject(IdleRejectionReason::NotDue);
        }
        self.selection_epoch = self.selection_epoch.wrapping_add(1);
        let epoch = self.selection_epoch;
        let result = select_global(
            actor_form_id,
            epoch,
            now_seconds,
            global_definitions,
            facts,
            evaluator,
            self,
        );
        match result {
            Ok((definition, random_percent, diagnostic)) => self.commit_selection(
                definition,
                IdleSource::IdleManager,
                random_percent,
                now_seconds,
                diagnostic,
            ),
            Err(reason) => {
                self.schedule_next_evaluation(now_seconds);
                self.reject(reason)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn force_select(
        &mut self,
        actor_form_id: u32,
        now_seconds: f32,
        lifecycle: IdleLifecycleFacts,
        _facts: &IdleRuntimeFacts,
        definitions: &[PreparedActorIdleDefinition],
        idle_form_id: u32,
        evaluator: &dyn IdleConditionEvaluator,
    ) -> IdleDecision {
        self.tick(now_seconds);
        if !lifecycle.special_idle_eligible() {
            return self.reject(lifecycle.rejection());
        }
        let Some(definition) = definitions
            .iter()
            .find(|definition| definition.form_id == idle_form_id)
        else {
            return self.reject(IdleRejectionReason::MissingIdle);
        };
        if !supported_group(definition.group_section) {
            return self.reject(IdleRejectionReason::UnsupportedGroup);
        }
        if definition.clip_name.is_none() {
            return self.reject(IdleRejectionReason::MissingClip);
        }
        // Conditions and replay cooldown are intentionally not consulted by
        // this path. The evaluator argument is retained so forced selection
        // shares the same public seam and can be extended without a second
        // request type; it is not called here by contract.
        let _ = evaluator;
        self.selection_epoch = self.selection_epoch.wrapping_add(1);
        self.commit_selection(
            definition,
            IdleSource::Forced,
            deterministic_percent(actor_form_id, 0, self.selection_epoch, idle_form_id, 0),
            now_seconds,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn select_package(
        &mut self,
        actor_form_id: u32,
        now_seconds: f32,
        package: &IdlePackageContext,
        collection: &IdlePackageCollection,
        definitions: &[PreparedActorIdleDefinition],
        facts: &IdleRuntimeFacts,
        evaluator: &dyn IdleConditionEvaluator,
    ) -> IdleDecision {
        self.selection_epoch = self.selection_epoch.wrapping_add(1);
        let epoch = self.selection_epoch;
        if collection.flags & UNKNOWN_COLLECTION_FLAG != 0 {
            // The raw flag remains in the prepared collection. Selection is
            // still deterministic, but the selected result carries a stable
            // diagnostic so inspect/console surfaces do not hide it.
            self.last_rejection = Some(IdleRejectionReason::UnknownCollectionFlag);
        }
        let sequence = collection.flags & RUN_IN_SEQUENCE_FLAG != 0;
        let do_once = collection.flags & DO_ONCE_FLAG != 0;
        if do_once
            && collection
                .animation_form_ids
                .iter()
                .all(|form_id| self.do_once_played.contains(form_id))
        {
            self.do_once_exhausted = true;
            return self.reject(IdleRejectionReason::DoOnceExhausted);
        }
        let mut order = collection.animation_form_ids.clone();
        if !sequence && !order.is_empty() {
            let offset = usize::from(deterministic_percent(
                actor_form_id,
                package.form_id,
                epoch,
                0,
                0,
            )) % order.len();
            order.rotate_left(offset);
        } else if sequence && !order.is_empty() {
            let offset = self.package_collection_cursor.min(order.len());
            order.rotate_left(offset);
        }
        let mut saw_cooldown = false;
        let mut last_reason = None;
        let random_percent = deterministic_percent(actor_form_id, package.form_id, epoch, 0, 0);
        for form_id in order {
            if do_once && self.do_once_played.contains(&form_id) {
                continue;
            }
            if self.cooldown_remaining(form_id, now_seconds) > 0.0 {
                saw_cooldown = true;
                continue;
            }
            let Some(definition) = definitions
                .iter()
                .find(|definition| definition.form_id == form_id)
            else {
                last_reason = Some(IdleRejectionReason::MissingIdle);
                continue;
            };
            match eligible_definition(definition, random_percent, facts, evaluator) {
                Ok(()) => {
                    let diagnostic = (collection.flags & UNKNOWN_COLLECTION_FLAG != 0)
                        .then_some(IdleRejectionReason::UnknownCollectionFlag);
                    if sequence {
                        self.package_collection_cursor = self
                            .package_collection_cursor
                            .saturating_add(1)
                            .min(collection.animation_form_ids.len());
                    }
                    if do_once {
                        self.do_once_played.insert(form_id);
                        self.do_once_exhausted =
                            self.do_once_played.len() >= collection.animation_form_ids.len();
                    }
                    let decision = self.commit_selection(
                        definition,
                        IdleSource::Package,
                        random_percent,
                        now_seconds,
                        diagnostic,
                    );
                    self.next_eligible_evaluation_seconds =
                        now_seconds + collection.timer_seconds.max(0.0);
                    return decision;
                }
                Err(reason) => last_reason = Some(reason),
            }
        }
        self.schedule_next_evaluation(now_seconds);
        self.reject(if saw_cooldown {
            IdleRejectionReason::ReplayCooldown
        } else {
            last_reason.unwrap_or(IdleRejectionReason::NoEligibleIdle)
        })
    }

    fn commit_selection(
        &mut self,
        definition: &PreparedActorIdleDefinition,
        source: IdleSource,
        random_percent: u8,
        now_seconds: f32,
        diagnostic: Option<IdleRejectionReason>,
    ) -> IdleDecision {
        self.last_idle_form_id = self.current_idle_form_id;
        self.current_idle_form_id = Some(definition.form_id);
        self.source = source;
        self.last_rejection = diagnostic;
        let delay = f32::from(definition.replay_delay_seconds.max(0));
        self.replay_cooldowns
            .insert(definition.form_id, now_seconds + delay);
        self.next_eligible_evaluation_seconds = now_seconds + 1.0;
        IdleDecision::Selected(IdleSelection {
            form_id: definition.form_id,
            source,
            loop_count: deterministic_loop_count(definition, self.selection_epoch, random_percent),
            random_percent,
            diagnostic,
        })
    }

    fn reject(&mut self, reason: IdleRejectionReason) -> IdleDecision {
        self.last_rejection = Some(reason);
        IdleDecision::Rejected(reason)
    }
}

fn supported_group(section: u8) -> bool {
    matches!(section, SPECIAL_IDLE_GROUP | WHOLE_BODY_GROUP)
}

fn eligible_definition(
    definition: &PreparedActorIdleDefinition,
    random_percent: u8,
    facts: &IdleRuntimeFacts,
    evaluator: &dyn IdleConditionEvaluator,
) -> Result<(), IdleRejectionReason> {
    if !supported_group(definition.group_section) {
        return Err(IdleRejectionReason::UnsupportedGroup);
    }
    if definition.clip_name.is_none() {
        return Err(IdleRejectionReason::MissingClip);
    }
    match evaluator.evaluate(&definition.conditions, random_percent, facts) {
        IdleConditionOutcome::True => Ok(()),
        IdleConditionOutcome::False => Err(IdleRejectionReason::ConditionsFalse),
        IdleConditionOutcome::Unevaluable => Err(IdleRejectionReason::UnsupportedCondition),
    }
}

fn deterministic_loop_count(
    definition: &PreparedActorIdleDefinition,
    epoch: u64,
    random_percent: u8,
) -> u8 {
    let min = definition.loop_min;
    let max = definition.loop_max.max(min);
    if min == 0 && max == 0 {
        return 1;
    }
    let span = u16::from(max) - u16::from(min) + 1;
    min.saturating_add(
        (u16::from(deterministic_percent(
            u32::from(random_percent),
            definition.form_id,
            epoch,
            0x4c4f_4f50,
            0,
        )) % span) as u8,
    )
}

fn deterministic_percent(actor: u32, package: u32, epoch: u64, group: u32, depth: u32) -> u8 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for value in [
        u64::from(actor),
        u64::from(package),
        epoch,
        u64::from(group),
        u64::from(depth),
    ] {
        for byte in value.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    (hash % 100) as u8
}

fn ordered_children(definitions: &[PreparedActorIdleDefinition], parent: Option<u32>) -> Vec<u32> {
    let mut ids = definitions
        .iter()
        .filter(|definition| definition.parent_form_id == parent)
        .map(|definition| definition.form_id)
        .collect::<Vec<_>>();
    ids.sort_unstable();
    if ids.len() < 2 {
        return ids;
    }
    let id_set = ids.iter().copied().collect::<BTreeSet<_>>();
    let mut successor = BTreeMap::new();
    for definition in definitions
        .iter()
        .filter(|definition| definition.parent_form_id == parent)
    {
        if let Some(previous) = definition.previous_sibling_form_id
            && id_set.contains(&previous)
        {
            successor.insert(previous, definition.form_id);
        }
    }
    let mut ordered = Vec::with_capacity(ids.len());
    let root = definitions
        .iter()
        .filter(|definition| definition.parent_form_id == parent)
        .find(|definition| definition.previous_sibling_form_id.is_none())
        .map(|definition| definition.form_id)
        .or_else(|| ids.first().copied());
    let mut current = root;
    while let Some(id) = current {
        if ordered.contains(&id) {
            break;
        }
        ordered.push(id);
        current = successor.get(&id).copied();
    }
    for id in ids {
        if !ordered.contains(&id) {
            ordered.push(id);
        }
    }
    ordered
}

fn select_global<'a>(
    actor_form_id: u32,
    epoch: u64,
    now_seconds: f32,
    definitions: &'a [PreparedActorIdleDefinition],
    facts: &IdleRuntimeFacts,
    evaluator: &dyn IdleConditionEvaluator,
    authority: &IdleAuthority,
) -> Result<
    (
        &'a PreparedActorIdleDefinition,
        u8,
        Option<IdleRejectionReason>,
    ),
    IdleRejectionReason,
> {
    #[allow(clippy::too_many_arguments, clippy::collapsible_if)]
    fn visit<'a>(
        actor_form_id: u32,
        epoch: u64,
        now_seconds: f32,
        parent: Option<u32>,
        depth: u32,
        definitions: &'a [PreparedActorIdleDefinition],
        facts: &IdleRuntimeFacts,
        evaluator: &dyn IdleConditionEvaluator,
        authority: &IdleAuthority,
    ) -> Result<
        (
            &'a PreparedActorIdleDefinition,
            u8,
            Option<IdleRejectionReason>,
        ),
        IdleRejectionReason,
    > {
        let children = ordered_children(definitions, parent);
        if children.is_empty() {
            return Err(IdleRejectionReason::NoEligibleIdle);
        }
        let roll = deterministic_percent(
            actor_form_id,
            parent.unwrap_or_default(),
            epoch,
            parent.unwrap_or_default(),
            depth,
        );
        let mut last_reason = None;
        for id in children {
            let Some(definition) = definitions
                .iter()
                .find(|definition| definition.form_id == id)
            else {
                continue;
            };
            match evaluator.evaluate(&definition.conditions, roll, facts) {
                IdleConditionOutcome::False => {
                    last_reason = Some(IdleRejectionReason::ConditionsFalse);
                    continue;
                }
                IdleConditionOutcome::Unevaluable => {
                    last_reason = Some(IdleRejectionReason::UnsupportedCondition);
                    continue;
                }
                IdleConditionOutcome::True => {}
            }
            let nested = definitions
                .iter()
                .any(|candidate| candidate.parent_form_id == Some(id));
            if nested {
                if let Ok(result) = visit(
                    actor_form_id,
                    epoch,
                    now_seconds,
                    Some(id),
                    depth + 1,
                    definitions,
                    facts,
                    evaluator,
                    authority,
                ) {
                    return Ok(result);
                }
            }
            if definition.clip_name.is_none() {
                last_reason = Some(IdleRejectionReason::MissingClip);
                continue;
            }
            if !supported_group(definition.group_section) {
                last_reason = Some(IdleRejectionReason::UnsupportedGroup);
                continue;
            }
            if authority.cooldown_remaining(definition.form_id, now_seconds) > 0.0 {
                last_reason = Some(IdleRejectionReason::ReplayCooldown);
                continue;
            }
            return Ok((definition, roll, None));
        }
        Err(last_reason.unwrap_or(IdleRejectionReason::NoEligibleIdle))
    }

    visit(
        actor_form_id,
        epoch,
        now_seconds,
        None,
        0,
        definitions,
        facts,
        evaluator,
        authority,
    )
}
