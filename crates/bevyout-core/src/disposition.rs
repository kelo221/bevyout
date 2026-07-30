//! Deterministic disposition and hostility policy (issue #116).
//!
//! Pure, `std`/`serde`-only. Given an observer actor, a target, and the
//! prepared [`FactionRelationTable`], this resolves one disposition value and
//! one hostility verdict with an *explicit rule precedence* and stable
//! tie-breaking, so the same inputs always produce the same verdict and the
//! deciding rule is reported (never a black box). This module owns the
//! awareness/relationship policy only; attack, damage, and death belong to a
//! later milestone and are deliberately absent.

use serde::{Deserialize, Serialize};

use crate::faction::{FactionRelationTable, GroupCombatReaction};

/// Fallout 3 `AIDT` aggression level (`0..=3`). Higher is more willing to
/// start combat unprompted.
#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Aggression {
    #[default]
    Unaggressive,
    Aggressive,
    VeryAggressive,
    Frenzied,
}

impl Aggression {
    #[must_use]
    pub const fn from_raw(value: u8) -> Self {
        match value {
            0 => Self::Unaggressive,
            1 => Self::Aggressive,
            2 => Self::VeryAggressive,
            _ => Self::Frenzied,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unaggressive => "unaggressive",
            Self::Aggressive => "aggressive",
            Self::VeryAggressive => "very_aggressive",
            Self::Frenzied => "frenzied",
        }
    }
}

/// One faction membership relevant to hostility resolution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FactionMembership {
    pub faction_form_id: u32,
    pub rank: i8,
}

/// The observer whose disposition toward a target is being resolved.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DispositionActor {
    /// Faction memberships, evaluated in ascending faction-FormID order for
    /// deterministic tie-breaking regardless of authored order.
    pub factions: Vec<FactionMembership>,
    /// `ACBS` base disposition (Fallout 3 `0..=100`, default 50).
    pub base_disposition: i32,
    pub aggression: Aggression,
    /// Optional authored per-race disposition adjustment (default 0). Fallout 3
    /// expresses race reaction through factions; this hook lets a resolved race
    /// adjustment participate without guessing a value when none is authored.
    pub race_disposition_adjust: i32,
}

/// The target a disposition is resolved toward. Weapon/damage state is
/// deliberately absent (no combat coupling).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DispositionTarget {
    pub factions: Vec<FactionMembership>,
    /// True when the target is the observer itself (an actor is never hostile
    /// to itself).
    pub is_self: bool,
}

/// Tunable disposition bucket thresholds. Defaults follow the Fallout 3
/// `0..=100` disposition scale.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DispositionThresholds {
    /// Disposition strictly below this is hostile on the fallback rule.
    pub hostile_below: i32,
    /// Disposition at or above this is friendly on the fallback rule.
    pub friendly_at_or_above: i32,
    /// An aggressive (but not frenzied) actor attacks a non-ally whose
    /// disposition is at or below this.
    pub aggressive_attack_at_or_below: i32,
    /// Disposition is clamped into `[min, max]` after modifiers.
    pub min: i32,
    pub max: i32,
}

impl Default for DispositionThresholds {
    fn default() -> Self {
        Self {
            hostile_below: 30,
            friendly_at_or_above: 70,
            aggressive_attack_at_or_below: 50,
            min: 0,
            max: 100,
        }
    }
}

/// The hostility verdict.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Hostility {
    Friendly,
    Neutral,
    Hostile,
}

impl Hostility {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Friendly => "friendly",
            Self::Neutral => "neutral",
            Self::Hostile => "hostile",
        }
    }
}

/// Which rule, in precedence order, decided the verdict. Reported so the
/// decision is auditable on real data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HostilityRule {
    /// Target is the observer itself.
    SameActor,
    /// Observer is frenzied and attacks everything.
    Frenzied,
    /// Observer and target share a faction (same team).
    SharedFaction,
    /// A faction relation declares the two sides Ally or Friend.
    FactionAllyOrFriend,
    /// A faction relation declares the two sides Enemy.
    FactionEnemy,
    /// Observer is aggressive and the target's disposition is low enough.
    Aggressive,
    /// Fallback: the resolved disposition value crossed a bucket threshold.
    DispositionThreshold,
}

impl HostilityRule {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::SameActor => "same_actor",
            Self::Frenzied => "frenzied",
            Self::SharedFaction => "shared_faction",
            Self::FactionAllyOrFriend => "faction_ally_or_friend",
            Self::FactionEnemy => "faction_enemy",
            Self::Aggressive => "aggressive",
            Self::DispositionThreshold => "disposition_threshold",
        }
    }
}

/// The resolved disposition/hostility, plus the deciding rule and any
/// deterministic diagnostics for unresolved factions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DispositionResult {
    pub disposition: i32,
    pub hostility: Hostility,
    pub decided_by: HostilityRule,
    /// Sorted, deduplicated diagnostic strings (unresolved factions, unknown
    /// combat reactions). Empty when every faction resolved cleanly.
    pub diagnostics: Vec<String>,
}

fn sorted_factions(factions: &[FactionMembership]) -> Vec<u32> {
    let mut ids: Vec<u32> = factions.iter().map(|f| f.faction_form_id).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

/// Resolves the observer's disposition toward the target and the hostility
/// verdict. Deterministic: factions are evaluated in ascending FormID order,
/// and rules fire in a fixed precedence.
#[must_use]
pub fn resolve_disposition(
    observer: &DispositionActor,
    target: &DispositionTarget,
    table: &FactionRelationTable,
    thresholds: &DispositionThresholds,
) -> DispositionResult {
    let mut diagnostics: Vec<String> = Vec::new();
    let observer_factions = sorted_factions(&observer.factions);
    let target_factions = sorted_factions(&target.factions);

    // --- disposition value ---
    let mut disposition = observer.base_disposition + observer.race_disposition_adjust;
    for &of in &observer_factions {
        if !table.is_known(of) {
            diagnostics.push(format!("unresolved observer faction {of:08x}"));
        }
        for &tf in &target_factions {
            disposition += table.modifier(of, tf);
        }
    }
    for &tf in &target_factions {
        if !table.is_known(tf) {
            diagnostics.push(format!("unresolved target faction {tf:08x}"));
        }
    }
    disposition = disposition.clamp(thresholds.min, thresholds.max);

    // --- relationship scan (deterministic ascending order) ---
    let mut shared_faction = false;
    let mut ally_or_friend = false;
    let mut enemy = false;
    for &of in &observer_factions {
        if target_factions.binary_search(&of).is_ok() {
            shared_faction = true;
        }
        for &tf in &target_factions {
            match table.combat_reaction(of, tf) {
                GroupCombatReaction::Enemy => enemy = true,
                GroupCombatReaction::Ally | GroupCombatReaction::Friend => ally_or_friend = true,
                GroupCombatReaction::Unknown(raw) => {
                    diagnostics.push(format!(
                        "unknown combat reaction {raw} between {of:08x} and {tf:08x}"
                    ));
                }
                GroupCombatReaction::Neutral => {}
            }
        }
    }

    diagnostics.sort();
    diagnostics.dedup();

    // --- hostility precedence ---
    let (hostility, decided_by) = if target.is_self {
        (Hostility::Friendly, HostilityRule::SameActor)
    } else if observer.aggression == Aggression::Frenzied {
        (Hostility::Hostile, HostilityRule::Frenzied)
    } else if shared_faction {
        (Hostility::Friendly, HostilityRule::SharedFaction)
    } else if ally_or_friend {
        (Hostility::Friendly, HostilityRule::FactionAllyOrFriend)
    } else if enemy {
        (Hostility::Hostile, HostilityRule::FactionEnemy)
    } else if matches!(
        observer.aggression,
        Aggression::Aggressive | Aggression::VeryAggressive
    ) && disposition <= thresholds.aggressive_attack_at_or_below
    {
        (Hostility::Hostile, HostilityRule::Aggressive)
    } else if disposition < thresholds.hostile_below {
        (Hostility::Hostile, HostilityRule::DispositionThreshold)
    } else if disposition >= thresholds.friendly_at_or_above {
        (Hostility::Friendly, HostilityRule::DispositionThreshold)
    } else {
        (Hostility::Neutral, HostilityRule::DispositionThreshold)
    };

    DispositionResult {
        disposition,
        hostility,
        decided_by,
        diagnostics,
    }
}

#[cfg(test)]
#[path = "tests/disposition.rs"]
mod tests;
