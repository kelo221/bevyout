use super::*;

#[test]
fn calendar_epoch_is_october_23_2277() {
    assert_eq!(
        calendar_from_ms(0),
        CalendarDate {
            year: 2277,
            month: 10,
            day: 23,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }
    );
    assert_eq!(
        calendar_from_ms(MS_PER_HOUR * 12),
        CalendarDate {
            year: 2277,
            month: 10,
            day: 23,
            hour: 12,
            minute: 0,
            second: 0,
            millisecond: 0,
        }
    );
}

#[test]
fn lighting_hour_is_a_projection() {
    let mut clock = GameClockState::default();
    clock.absolute_game_ms = MS_PER_HOUR * 12;
    assert!((clock.hour_as_f32() - 12.0).abs() < f32::EPSILON);
    let _ = clock.hour_as_f32();
    assert_eq!(clock.absolute_game_ms, MS_PER_HOUR * 12);
}

#[test]
fn realtime_remainder_accumulates() {
    let mut clock = GameClockState::default();
    let first = clock.advance_realtime(500).unwrap();
    assert_eq!(first.to_game_ms, 15);
    assert_eq!(clock.fractional_timescale_remainder, 0);
    let second = clock.advance_realtime(1).unwrap();
    assert_eq!(second.to_game_ms, 15);
    assert_eq!(clock.fractional_timescale_remainder, 30);
    let third = clock.advance_realtime(33).unwrap();
    assert_eq!(third.to_game_ms, 16);
}

#[test]
fn overflow_leaves_clock_unchanged() {
    let mut clock = GameClockState {
        absolute_game_ms: u64::MAX,
        ..GameClockState::default()
    };
    assert_eq!(
        clock.advance_game_ms(1, TimeAdvanceReason::Console),
        Err(TimeError::Overflow)
    );
    assert_eq!(clock.absolute_game_ms, u64::MAX);
}

#[test]
fn calendar_rolls_month_and_year() {
    let after_october = calendar_from_ms(MS_PER_DAY * 8);
    assert_eq!(after_october.year, 2277);
    assert_eq!(after_october.month, 11);
    assert_eq!(after_october.day, 1);
    let next_year = calendar_from_ms(MS_PER_DAY * ((12 - 10) * 30 + (30 - 23) + 1));
    assert_eq!(next_year.year, 2278);
    assert_eq!(next_year.month, 1);
    assert_eq!(next_year.day, 1);
}
