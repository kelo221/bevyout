//! Player active-effects, radiation, and addiction runtime (M9 wave 3,
//! #318).
//!
//! Thin Bevy adapter over the pure `bevyout_core::effects`/`radiation`/
//! `chems` kernels (#317), exactly like `stats.rs` is for the wave-1
//! sheet: `PlayerRadiation`, `ActiveEffectsList`, `Addictions`, and
//! `PlayerVitals` attach to the FPS player through `Added<FpsPlayer>`,
//! restored from `PlayerProgression` so camera-mode despawn cannot drop
//! them. `EffectCatalog` and `RngResource` (the seeded PRNG) are app
//! resources loaded from the prepared effect catalog. The stored
//! `PlayerProgression.stats` sheet stays authoritative — effective SPECIAL
//! is a read through `projected_special` (#317), never written back.
//!
//! Timescale note (see also `bevyout_core::effects`): EFIT durations are
//! authored in game seconds; this wave ticks real frame milliseconds, so
//! application converts once (`duration_ms()`). Wave 9's scaled game clock
//! replaces that conversion.
//!
//! Chem-dose tracking: the core ledger merges by `(source, actor_value)`
//! and its expiry events cannot name which chem wore off, so this module
//! keeps a parallel `chem_doses_ms` map (withdrawal FormID -> remaining
//! ms) on the same component. It is the only place that decides when an
//! addicted chem's buff ends; the ledger stays the only authority on
//! magnitudes.

use std::collections::BTreeMap;
use std::path::Path;

use bevy::prelude::*;

use bevyout_core::actor_state::{ActorValue, SpecialAttribute};
use bevyout_core::chems::{RPG_RNG_DEFAULT_SEED, RpgRngState};
use bevyout_core::effects::{
    ActiveEffect, ActiveEffectsLedger, EffectSource, IngestibleConditionOutcome,
    IngestibleDefinition, active_rad_resistance_bps, evaluate_ingestible_condition,
    projected_derived, projected_special_with_limbs,
};
use bevyout_core::perks::PerkProgression;
use bevyout_core::radiation::{self, RadiationPool};

use super::player::FpsPlayer;
use super::plugins::ViewerSet;
use super::stats::{ActorStats, DerivedAttributes, PlayerProgression};
use crate::vsa::{EFFECT_CATALOG_REVISION, PreparedEffectCatalog, PreparedSceneManifest};

/// Prepared ingestible definitions keyed by FormID, loaded from
/// `catalogs/<source_fingerprint>/effects.ron` at startup. An absent or
/// stale catalog degrades to "no ingestibles known": item use behaves as
/// before and `addchem` reports `unknown_ingestible` rather than failing
/// startup.
#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub(crate) struct EffectCatalog {
    pub(crate) ingestibles: BTreeMap<u32, IngestibleDefinition>,
}

impl EffectCatalog {
    pub(crate) fn get(&self, form_id: u32) -> Option<&IngestibleDefinition> {
        self.ingestibles.get(&form_id)
    }
}

/// Core-owned deterministic PRNG state (addiction rolls), seeded with the
/// documented `RPG_RNG_DEFAULT_SEED` at startup so acceptance runs are
/// reproducible from launch. Every consumed draw index is inspectable
/// through `console`'s `effects` output.
#[derive(Resource, Debug, Clone, Copy, Deref, DerefMut, PartialEq)]
pub(crate) struct RngResource(pub(crate) RpgRngState);

impl Default for RngResource {
    fn default() -> Self {
        Self(RpgRngState::new(RPG_RNG_DEFAULT_SEED))
    }
}

/// The player's accumulated radiation dose (#318). The pure pool is the
/// authority; console commands and chem effects mutate through it.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct PlayerRadiation(pub(crate) RadiationPool);

/// Ordered ledger of the player's timed active effects plus the per-chem
/// dose timers (see the module note). `apply_chem_dose` keeps the two views
/// consistent; systems never mutate one without the other.
#[derive(Component, Debug, Clone, Default, PartialEq)]
pub(crate) struct ActiveEffectsList {
    pub(crate) ledger: ActiveEffectsLedger,
    /// Withdrawal FormID -> remaining buff milliseconds for chem doses.
    pub(crate) chem_doses_ms: BTreeMap<u32, u32>,
}

/// Player addiction state keyed by withdrawal FormID (#318).
#[derive(Component, Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Addictions(pub(crate) bevyout_core::chems::Addictions);

/// Current-vs-maximum health. Wave 1's `DerivedAttributes.max_health` is
/// the maximum; nothing before this wave tracked current health because no
/// gameplay dealt damage to or healed the player. Stimpak-style restore
/// effects clamp against it, so wave 3 introduces it at full health.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(crate) struct PlayerVitals {
    pub(crate) current_health: f32,
}

/// Recomputed effective-SPECIAL projection (sheet + effects + rads),
/// refreshed whenever any input changes so consumers read synchronously.
#[derive(Component, Debug, Clone, Default, PartialEq)]
pub(crate) struct ProjectedSpecial(pub(crate) BTreeMap<SpecialAttribute, u8>);

/// One expired chem dose's identity; written by the tick system and
/// consumed by the withdrawal system in the same frame.
#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub(crate) struct ExpiredChemDose {
    pub(crate) withdrawal_form_id: u32,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EffectsSet {
    Attach,
    Mutate,
    Project,
    Clamp,
}

pub(crate) struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EffectCatalog>()
            .init_resource::<RngResource>()
            .add_message::<ExpiredChemDose>()
            .configure_sets(
                Update,
                (
                    EffectsSet::Attach,
                    EffectsSet::Mutate,
                    EffectsSet::Project,
                    EffectsSet::Clamp,
                )
                    .chain()
                    .in_set(ViewerSet::WorldSync),
            )
            .add_systems(
                Update,
                (attach_effects_to_player, seed_player_vitals)
                    .chain()
                    .in_set(EffectsSet::Attach),
            )
            .add_systems(
                Update,
                (
                    tick_active_effects_and_doses,
                    start_withdrawal_on_chem_expiry,
                )
                    .chain()
                    .in_set(EffectsSet::Mutate),
            )
            .add_systems(
                Update,
                recalculate_projected_special.in_set(EffectsSet::Project),
            )
            .add_systems(Update, clamp_current_health.in_set(EffectsSet::Clamp));
    }
}

fn attach_effects_to_player(
    players: Query<Entity, Added<FpsPlayer>>,
    progression: Res<PlayerProgression>,
    mut commands: Commands,
) {
    for entity in &players {
        commands.entity(entity).insert((
            PlayerRadiation(progression.radiation),
            ActiveEffectsList {
                ledger: progression.effects.clone(),
                chem_doses_ms: progression.chem_doses_ms.clone(),
            },
            Addictions(progression.addictions.clone()),
            ProjectedSpecial::default(),
        ));
    }
}

/// Seeds vitals at derived max on spawn. Current health only ever moves
/// down (damage) or back up (restore effects clamp at max), so no per-frame
/// clamp system is needed yet.
fn seed_player_vitals(
    mut commands: Commands,
    settings: Option<Res<super::stats::StatsSettings>>,
    progression: Res<PlayerProgression>,
    players: Query<(Entity, &ActorStats), Added<ActorStats>>,
) {
    let Some(settings) = settings else {
        return;
    };
    for (entity, stats) in &players {
        let max = stats.0.derived(&settings.0).max_health;
        commands.entity(entity).insert(PlayerVitals {
            current_health: progression.current_health.unwrap_or(max).min(max).max(0.0),
        });
    }
}

/// The projection system's query shape (shared components, change-filtered).
type ProjectionQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static ActorStats,
        &'static ActiveEffectsList,
        &'static PlayerRadiation,
        &'static mut ProjectedSpecial,
    ),
    Or<(
        Changed<ActorStats>,
        Changed<ActiveEffectsList>,
        Changed<PlayerRadiation>,
    )>,
>;

/// Refreshes the effective-SPECIAL projection from the authoritative sheet
/// plus active modifiers and radiation penalties.
fn recalculate_projected_special(
    progression: Res<PlayerProgression>,
    mut players: ProjectionQuery,
) {
    for (stats, effects, radiation, mut projection) in &mut players {
        projection.0 = projected_special_with_limbs(
            &stats.0,
            &effects.ledger,
            radiation.0.rads,
            Some(&progression.limbs),
        );
    }
}

fn clamp_current_health(
    mut players: Query<(&DerivedAttributes, &mut PlayerVitals), Changed<DerivedAttributes>>,
) {
    for (derived, mut vitals) in &mut players {
        vitals.current_health = vitals.current_health.min(derived.0.max_health).max(0.0);
    }
}

/// Applies an environmental or ingestible radiation dose through active
/// RadResist modifiers.
pub(crate) fn apply_player_radiation(
    radiation: &mut PlayerRadiation,
    effects: &ActiveEffectsList,
    dose: u16,
) -> radiation::RadiationDoseOutcome {
    radiation::apply_radiation(
        &mut radiation.0,
        dose,
        active_rad_resistance_bps(&effects.ledger),
    )
}

/// Advances every timed effect and chem-dose timer by the frame's whole
/// milliseconds. Timed effects expire out of the ledger; an expiring chem
/// dose publishes `ExpiredChemDose` for the withdrawal transition below.
fn tick_active_effects_and_doses(
    time: Res<Time>,
    runtime: Option<Res<super::game_time::GameTimeRuntime>>,
    mut players: Query<&mut ActiveEffectsList>,
    mut expired: MessageWriter<ExpiredChemDose>,
) {
    let delta_ms = if runtime.is_some() {
        // Integer advances already ticked the ledger inside LifecycleWorld.
        // Isolated tests without GameTimeRuntime keep the frame fallback.
        return;
    } else {
        u32::try_from((time.delta_secs() * 1000.0).round() as u64).unwrap_or(u32::MAX)
    };
    if delta_ms == 0 {
        return;
    }
    for mut list in &mut players {
        list.ledger.tick(delta_ms);
        let mut finished = Vec::new();
        for (&withdrawal_form_id, remaining) in list.chem_doses_ms.iter_mut() {
            *remaining = remaining.saturating_sub(delta_ms);
            if *remaining == 0 {
                finished.push(withdrawal_form_id);
            }
        }
        for withdrawal_form_id in finished {
            list.chem_doses_ms.remove(&withdrawal_form_id);
            expired.write(ExpiredChemDose { withdrawal_form_id });
        }
    }
}

/// When an addicted chem's buff expires, move that addiction into
/// withdrawal. Withdrawal penalties themselves are authored as SPELs
/// (stored on the catalog entry but not run this wave); the phase
/// transition is what the addiction machine and `cureaddiction` key on.
fn start_withdrawal_on_chem_expiry(
    mut expired: MessageReader<ExpiredChemDose>,
    mut players: Query<&mut Addictions>,
) {
    for event in expired.read() {
        for mut addictions in &mut players {
            addictions.0.begin_withdrawal(event.withdrawal_form_id);
        }
    }
}

/// Applies one cataloged ingestible to the player's components: instant
/// effects mutate vitals/radiation directly, timed value modifiers merge
/// into the ledger, and an addictive chem rolls against the seeded PRNG.
///
/// Returns everything the caller (console/Pip-Boy) reports. The roll uses
/// exactly one PRNG draw when the chem is addictive, zero otherwise.
pub(crate) struct AppliedIngestible {
    pub(crate) editor_id: String,
    pub(crate) healed_to: Option<f32>,
    pub(crate) rads_removed: u16,
    pub(crate) rads_added: i32,
    pub(crate) applied_modifiers: usize,
    pub(crate) condition_false: usize,
    pub(crate) condition_unsupported: usize,
    pub(crate) addiction_roll: Option<bool>,
    /// The PRNG draw index after this ingestible's roll (unchanged for a
    /// non-addictive item); acceptance evidence cites it.
    pub(crate) rng_draw_index: u32,
}

/// The player's wave-3 components as one mutable bundle, so ingestion
/// touches them atomically (one query shape for every caller).
pub(crate) struct PlayerEffectComponents<'a> {
    pub(crate) vitals: &'a mut PlayerVitals,
    pub(crate) radiation: &'a mut PlayerRadiation,
    pub(crate) effects: &'a mut ActiveEffectsList,
    pub(crate) addictions: &'a mut Addictions,
}

/// Mutates the player components for one consumed ingestible.
pub(crate) fn apply_ingestible(
    definition: &IngestibleDefinition,
    stats: &ActorStats,
    perks: &PerkProgression,
    settings: &super::stats::StatsSettings,
    player: PlayerEffectComponents,
    rng: &mut RpgRngState,
) -> AppliedIngestible {
    let max_health = projected_derived(
        &stats.0,
        &player.effects.ledger,
        player.radiation.0.rads,
        &settings.0,
    )
    .max_health;
    let PlayerEffectComponents {
        vitals,
        radiation,
        effects,
        addictions,
    } = player;
    let mut outcome = AppliedIngestible {
        editor_id: definition.editor_id.clone(),
        healed_to: None,
        rads_removed: 0,
        rads_added: 0,
        applied_modifiers: 0,
        condition_false: 0,
        condition_unsupported: 0,
        addiction_roll: None,
        rng_draw_index: rng.draw_index,
    };
    for effect in &definition.effects {
        if let Some(condition) = &effect.condition {
            match evaluate_ingestible_condition(condition, perks) {
                IngestibleConditionOutcome::True => {}
                IngestibleConditionOutcome::False => {
                    outcome.condition_false += 1;
                    continue;
                }
                IngestibleConditionOutcome::Unsupported => {
                    outcome.condition_unsupported += 1;
                    continue;
                }
            }
        }
        let Some(actor_value) = effect.actor_value else {
            continue;
        };
        match actor_value {
            ActorValue::Health => {
                if effect.magnitude >= 0.0 && effect.duration_s == 0 {
                    vitals.current_health =
                        (vitals.current_health + effect.magnitude).min(max_health);
                    outcome.healed_to = Some(vitals.current_health);
                } else {
                    apply_timed(
                        effects,
                        EffectSource::Item,
                        actor_value,
                        effect.magnitude,
                        effect.duration_ms(),
                    );
                    outcome.applied_modifiers += 1;
                }
            }
            ActorValue::Rads => {
                // Real-data polarity (verified on the GOTY ESM): RadAway's
                // `RestoreRadiationLevel` carries a *positive* magnitude
                // (+50) with no Recover flag — "restore" for the Rads
                // value means lowering it. Positive instant magnitudes
                // therefore remove rads; negative ones irradiate.
                if effect.duration_s == 0 {
                    if effect.magnitude >= 0.0 {
                        outcome.rads_removed =
                            radiation::remove_rads(&mut radiation.0, effect.magnitude as u16);
                    } else {
                        let dose = (-effect.magnitude) as u16;
                        let absorbed =
                            apply_player_radiation(radiation, effects, dose).absorbed_rads;
                        outcome.rads_added = i32::from(absorbed);
                    }
                } else {
                    apply_timed(
                        effects,
                        EffectSource::Item,
                        actor_value,
                        effect.magnitude,
                        effect.duration_ms(),
                    );
                    outcome.applied_modifiers += 1;
                }
            }
            other => {
                apply_timed(
                    effects,
                    EffectSource::Chem,
                    other,
                    effect.magnitude,
                    effect.duration_ms(),
                );
                outcome.applied_modifiers += 1;
            }
        }
    }
    if definition.addictive() {
        let rolled = bevyout_core::chems::roll_addiction(definition.addiction_chance_bps(), 0, rng);
        if rolled {
            addictions.0.addict(definition.withdrawal_form_id);
        }
        outcome.addiction_roll = Some(rolled);
        outcome.rng_draw_index = rng.draw_index;
        // The dose timer drives the Addicted -> Withdrawing transition when
        // the buff runs out: the longest timed effect's duration (a chem
        // with only instant effects still wears off at its shortest
        // authored duration; zero-duration addictive chems would withdraw
        // immediately, which none do on real data).
        let buff_ms = definition
            .effects
            .iter()
            .filter(|effect| {
                effect.condition.as_ref().is_none_or(|condition| {
                    evaluate_ingestible_condition(condition, perks)
                        == IngestibleConditionOutcome::True
                })
            })
            .map(|effect| effect.duration_ms())
            .max()
            .unwrap_or(0);
        if buff_ms > 0 {
            effects
                .chem_doses_ms
                .insert(definition.withdrawal_form_id, buff_ms);
        }
    }
    outcome
}

fn apply_timed(
    effects: &mut ActiveEffectsList,
    source: EffectSource,
    actor_value: ActorValue,
    magnitude: f32,
    remaining_ms: u32,
) {
    effects.ledger.apply(ActiveEffect {
        source,
        actor_value,
        magnitude,
        remaining_ms,
    });
}

/// Loads the effect catalog for a manifest's content set from the
/// deterministic `catalogs/<source_fingerprint>/effects.ron` path (M9 wave
/// 3, #316/#318). Like the perk catalog — and unlike the item catalog — a
/// missing, stale, or unreadable catalog degrades to empty with a warning:
/// item use behaves as before instead of failing viewer startup.
pub(crate) fn load_effect_catalog_for_manifest(
    manifest: &PreparedSceneManifest,
    asset_root: &Path,
) -> EffectCatalog {
    let path = asset_root
        .join("catalogs")
        .join(&manifest.source_fingerprint)
        .join("effects.ron");
    let Some(text) = std::fs::read_to_string(&path).ok() else {
        warn!("effects: no effect catalog for this content set, ingestible commands disabled");
        return EffectCatalog::default();
    };
    let catalog: PreparedEffectCatalog = match ron::from_str(&text) {
        Ok(catalog) => catalog,
        Err(error) => {
            warn!("effects: effect catalog unreadable ({error}), ingestible commands disabled");
            return EffectCatalog::default();
        }
    };
    if catalog.revision != EFFECT_CATALOG_REVISION {
        warn!(
            "effects: effect catalog revision {} is stale, expected {EFFECT_CATALOG_REVISION}; run `prepare` again (ingestible commands disabled)",
            catalog.revision
        );
        return EffectCatalog::default();
    }
    if catalog.source_fingerprint != manifest.source_fingerprint {
        warn!("effects: effect catalog fingerprint mismatch, ingestible commands disabled");
        return EffectCatalog::default();
    }
    info!(
        "effects: loaded {} ingestibles ({} addictive), {} effect definitions from effect catalog",
        catalog.counters.ingestibles, catalog.counters.addictive, catalog.counters.effects
    );
    EffectCatalog {
        ingestibles: catalog
            .ingestibles
            .into_iter()
            .map(|ingestible| (ingestible.form_id, ingestible))
            .collect(),
    }
}
