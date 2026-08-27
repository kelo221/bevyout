//! Unit tests for the pure chem/addiction kernel (M9 wave 3 #317): the
//! splitmix64 stream is pinned, and the addiction machine walks
//! Clean -> Addicted -> Withdrawing -> cured.

use super::*;

use crate::chems::{
    AddictionPhase, Addictions, BPS_MAX, RPG_RNG_DEFAULT_SEED, RpgRngState, roll_addiction,
};

/// Reference splitmix64 outputs for seed 0, computed independently of the
/// implementation under test (the standard splitmix64 constants).
#[test]
fn rng_first_five_u64_draws_are_pinned_for_seed_zero() {
    let mut rng = RpgRngState::new(0);
    let expected = [
        0xE220_A839_7B1D_CDAF_u64,
        0x6E78_9E6A_A1B9_65F4_u64,
        0x06C4_5D18_8009_454F_u64,
        0xF88B_B8A8_724C_81EC_u64,
        0x1B39_896A_51A8_749B_u64,
    ];
    for pinned in expected {
        assert_eq!(rng.next_u64(), pinned);
    }
    // next_u64 does not consume draw draws.
    assert_eq!(rng.draw_index, 0);
}

#[test]
fn draw_bps_stays_in_range_and_counts_draws() {
    let mut rng = RpgRngState::new(RPG_RNG_DEFAULT_SEED);
    for _ in 0..5 {
        let value = rng.draw_bps(BPS_MAX);
        assert!(value < BPS_MAX);
    }
    assert_eq!(rng.draw_index, 5);
    // Degenerate limits clamp to a one-bucket scale and always draw 0.
    assert_eq!(rng.draw_bps(0), 0);
    assert_eq!(rng.draw_bps(1), 0);
    assert_eq!(rng.draw_index, 7);
    // The stream is serializable and resumes deterministically.
    let snapshot = ron::to_string(&rng).expect("serialize rng");
    let mut restored: RpgRngState = ron::from_str(&snapshot).expect("deserialize rng");
    let mut fresh = rng;
    assert_eq!(restored.next_u64(), fresh.next_u64());
}

#[test]
fn roll_addiction_uses_effective_chance_and_resist() {
    // Zero chance never rolls (engine requirement) and burns no draw.
    let mut rng = RpgRngState::new(1);
    assert!(!roll_addiction(0, 0, &mut rng));
    assert_eq!(rng.draw_index, 0);
    // 100% chance with no resist always addicts.
    let mut rng = RpgRngState::new(2);
    for _ in 0..10 {
        assert!(roll_addiction(BPS_MAX, 0, &mut rng));
    }
    // Full chem resist neutralizes any chance.
    let mut rng = RpgRngState::new(3);
    for _ in 0..10 {
        assert!(!roll_addiction(BPS_MAX, BPS_MAX, &mut rng));
    }
    // Reproducibility: the same seed draws the same outcome sequence.
    let a: Vec<bool> = {
        let mut rng = RpgRngState::new(42);
        (0..8).map(|_| roll_addiction(2_000, 0, &mut rng)).collect()
    };
    let b: Vec<bool> = {
        let mut rng = RpgRngState::new(42);
        (0..8).map(|_| roll_addiction(2_000, 0, &mut rng)).collect()
    };
    assert_eq!(a, b);
    // Pinned stream positions: the default startup seed 0 fails Jet's 20%
    // roll on its first four draws (draws 7535, 5700, 5679, 2444 bps),
    // while seed 6's first draw is 592 bps and addicts. These pins are the
    // reproducibility evidence the console surface (#318) cites via
    // rng_draw_index.
    let mut rng = RpgRngState::new(0);
    let outcomes: Vec<bool> = (0..4).map(|_| roll_addiction(2_000, 0, &mut rng)).collect();
    assert_eq!(outcomes, vec![false, false, false, false]);
    assert_eq!(rng.draw_index, 4);
    let mut rng = RpgRngState::new(6);
    assert!(roll_addiction(2_000, 0, &mut rng));
    assert_eq!(rng.draw_index, 1);
}

#[test]
fn addiction_machine_walks_clean_addicted_withdrawing_cured() {
    let mut addictions = Addictions::default();
    assert!(addictions.is_empty());
    assert!(!addictions.is_addicted(0x0003_3067));
    addictions.addict(0x0003_3067);
    assert!(addictions.is_addicted(0x0003_3067));
    assert_eq!(addictions.0[&0x0003_3067], AddictionPhase::Addicted);
    // Withdrawal only starts while Addicted, not while Clean.
    assert!(addictions.begin_withdrawal(0x0003_3067));
    assert!(!addictions.begin_withdrawal(0x0003_3067));
    assert_eq!(addictions.0[&0x0003_3067], AddictionPhase::Withdrawing);
    assert!(addictions.is_addicted(0x0003_3067));
    // Re-dosing an unrelated chem does not touch this one; curing removes
    // it. Zero FormIDs are ignored (a defensive no-op).
    addictions.addict(0);
    assert_eq!(addictions.0.len(), 1);
    assert!(addictions.cure(0x0003_3067));
    assert!(!addictions.cure(0x0003_3067));
    assert!(!addictions.is_addicted(0x0003_3067));
}

#[test]
fn cure_all_reports_the_count() {
    let mut addictions = Addictions::default();
    addictions.addict(0x0003_3067);
    addictions.addict(0x0003_3064);
    addictions.begin_withdrawal(0x0003_3064);
    assert_eq!(addictions.cure_all(), 2);
    assert!(addictions.is_empty());
}
