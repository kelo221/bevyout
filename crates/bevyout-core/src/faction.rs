//! Engine-independent Fallout 3 `FACT` faction relations and the prepared
//! faction relation table (issue #116).
//!
//! The ESM4 reader already decodes `FACT` records (ranks and `XNAM`
//! inter-faction relations); this module is the pure, `std`/`serde`-only
//! contract those decoded relations are prepared into so runtime disposition
//! and hostility resolution has an authoritative, deterministic lookup. It has
//! no Bevy, filesystem, or parser dependencies and is included verbatim into
//! `tests/features.rs` the same way `actor_state` is.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Bump whenever the serialized faction-relation shape changes, even for a
/// serde-defaulted field. The relation table is embedded in the per-cell
/// actor catalog, so a change here is paired with an `ACTOR_CATALOG_REVISION`
/// bump; this constant documents the faction contract independently.
pub const FACTION_TABLE_REVISION: &str = "faction-relations-v1";

/// The Fallout 3 `XNAM` "group combat reaction" enum. Values follow the
/// fopdoc FO3 `FACT` page: `0` Neutral, `1` Enemy, `2` Ally, `3` Friend.
/// Any other raw value is preserved as [`GroupCombatReaction::Unknown`]
/// rather than guessed, so downstream policy can diagnose it.
#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum GroupCombatReaction {
    #[default]
    Neutral,
    Enemy,
    Ally,
    Friend,
    Unknown(u32),
}

impl GroupCombatReaction {
    #[must_use]
    pub const fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::Neutral,
            1 => Self::Enemy,
            2 => Self::Ally,
            3 => Self::Friend,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Enemy => "enemy",
            Self::Ally => "ally",
            Self::Friend => "friend",
            Self::Unknown(_) => "unknown",
        }
    }

    /// A rank from most hostile to most friendly, so combining the two
    /// directions of a relation is deterministic. Unknown reactions rank as
    /// neutral for combination but keep their raw value for diagnostics.
    const fn hostility_rank(self) -> u8 {
        match self {
            Self::Enemy => 0,
            Self::Neutral | Self::Unknown(_) => 1,
            Self::Ally => 2,
            Self::Friend => 3,
        }
    }

    /// The more hostile of two reactions (Enemy dominates Friend). Ties keep
    /// `self` so the result is order-independent for equal ranks.
    #[must_use]
    pub fn most_hostile(self, other: Self) -> Self {
        if other.hostility_rank() < self.hostility_rank() {
            other
        } else {
            self
        }
    }
}

/// One `XNAM` relation entry: the other faction, the disposition modifier
/// this faction applies toward members of that faction, and the combat
/// reaction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FactionRelation {
    pub faction_form_id: u32,
    pub modifier: i32,
    pub reaction: GroupCombatReaction,
}

/// A prepared `FACT` record's runtime-relevant identity and relations. Ranks
/// remain on the actor catalog for title resolution; this type carries only
/// what disposition/hostility resolution needs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PreparedFaction {
    pub form_id: u32,
    pub editor_id: Option<String>,
    pub name: Option<String>,
    /// `XNAM` relations in ascending target-FormID order (deduplicated).
    pub relations: Vec<FactionRelation>,
}

/// Deterministic lookup over prepared factions. Ordered so serialization and
/// iteration are stable.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FactionRelationTable {
    pub factions: BTreeMap<u32, PreparedFaction>,
}

impl FactionRelationTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, faction: PreparedFaction) {
        self.factions.insert(faction.form_id, faction);
    }

    #[must_use]
    pub fn faction(&self, form_id: u32) -> Option<&PreparedFaction> {
        self.factions.get(&form_id)
    }

    #[must_use]
    pub fn is_known(&self, form_id: u32) -> bool {
        self.factions.contains_key(&form_id)
    }

    /// `from`'s authored relation toward `to`, if any.
    #[must_use]
    pub fn relation(&self, from: u32, to: u32) -> Option<&FactionRelation> {
        self.factions
            .get(&from)?
            .relations
            .iter()
            .find(|relation| relation.faction_form_id == to)
    }

    /// The disposition modifier `from` applies toward `to` (0 when there is no
    /// authored relation).
    #[must_use]
    pub fn modifier(&self, from: u32, to: u32) -> i32 {
        self.relation(from, to)
            .map_or(0, |relation| relation.modifier)
    }

    /// The combat reaction between two factions, taking the most hostile of
    /// the two directions so an enemy declaration by either side is honored.
    #[must_use]
    pub fn combat_reaction(&self, a: u32, b: u32) -> GroupCombatReaction {
        let forward = self
            .relation(a, b)
            .map_or(GroupCombatReaction::Neutral, |relation| relation.reaction);
        let backward = self
            .relation(b, a)
            .map_or(GroupCombatReaction::Neutral, |relation| relation.reaction);
        forward.most_hostile(backward)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> FactionRelationTable {
        let mut table = FactionRelationTable::new();
        table.insert(PreparedFaction {
            form_id: 0x10,
            editor_id: Some("RaidersFaction".into()),
            name: Some("Raiders".into()),
            relations: vec![
                FactionRelation {
                    faction_form_id: 0x20,
                    modifier: -60,
                    reaction: GroupCombatReaction::Enemy,
                },
                FactionRelation {
                    faction_form_id: 0x30,
                    modifier: 40,
                    reaction: GroupCombatReaction::Ally,
                },
            ],
        });
        table.insert(PreparedFaction {
            form_id: 0x20,
            editor_id: Some("TownsfolkFaction".into()),
            name: Some("Townsfolk".into()),
            relations: Vec::new(),
        });
        // Reciprocal ally so the combined reaction is Ally from both sides.
        table.insert(PreparedFaction {
            form_id: 0x30,
            editor_id: Some("AlliesFaction".into()),
            name: Some("Allies".into()),
            relations: vec![FactionRelation {
                faction_form_id: 0x10,
                modifier: 40,
                reaction: GroupCombatReaction::Ally,
            }],
        });
        table
    }

    #[test]
    fn from_raw_preserves_unknown_reaction_value() {
        assert_eq!(GroupCombatReaction::from_raw(1), GroupCombatReaction::Enemy);
        assert_eq!(
            GroupCombatReaction::from_raw(7),
            GroupCombatReaction::Unknown(7)
        );
    }

    #[test]
    fn relation_modifier_and_reaction_are_directional_lookups() {
        let table = table();
        assert_eq!(table.modifier(0x10, 0x20), -60);
        assert_eq!(table.modifier(0x20, 0x10), 0);
        assert_eq!(table.modifier(0x10, 0x999), 0);
    }

    #[test]
    fn combat_reaction_takes_most_hostile_direction() {
        let table = table();
        // Only 0x10 -> 0x20 declares Enemy; the reverse is silent Neutral.
        assert_eq!(
            table.combat_reaction(0x20, 0x10),
            GroupCombatReaction::Enemy
        );
        // Reciprocal ally on both sides combines to Ally.
        assert_eq!(table.combat_reaction(0x10, 0x30), GroupCombatReaction::Ally);
        // A one-sided ally with a silent (neutral) reverse combines to Neutral;
        // only Enemy is contagious across a single direction.
        assert_eq!(
            table.combat_reaction(0x20, 0x999),
            GroupCombatReaction::Neutral
        );
    }

    #[test]
    fn most_hostile_is_order_independent() {
        assert_eq!(
            GroupCombatReaction::Enemy.most_hostile(GroupCombatReaction::Friend),
            GroupCombatReaction::Enemy
        );
        assert_eq!(
            GroupCombatReaction::Friend.most_hostile(GroupCombatReaction::Enemy),
            GroupCombatReaction::Enemy
        );
        assert_eq!(
            GroupCombatReaction::Ally.most_hostile(GroupCombatReaction::Neutral),
            GroupCombatReaction::Neutral
        );
    }
}
