//! Integer game-time authority used by RPG policies.
//!
//! [`GameTime`] is the millisecond identifier restock and medical already
//! take. [`GameClockState`] is the advancing clock: absolute milliseconds,
//! a fractional realtime remainder, and an integer timescale. Lighting
//! consumes [`GameClockState::hour_as_f32`]; that projection must never be
//! written back into the save authority.

use serde::{Deserialize, Serialize};

/// Whole milliseconds of in-game elapsed time.
#[derive(
    Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct GameTime(pub u64);

impl GameTime {
    #[must_use]
    pub const fn from_ms(ms: u64) -> Self {
        Self(ms)
    }

    #[must_use]
    pub const fn as_ms(self) -> u64 {
        self.0
    }
}

/// Game milliseconds in one second.
pub const MS_PER_SECOND: u64 = 1_000;
/// Game milliseconds in one minute.
pub const MS_PER_MINUTE: u64 = 60_000;
/// Game milliseconds in one hour.
pub const MS_PER_HOUR: u64 = 3_600_000;
/// Game milliseconds in one day.
pub const MS_PER_DAY: u64 = 86_400_000;
/// Versioned calendar: thirty days per month.
pub const DAYS_PER_MONTH: u64 = 30;
/// Versioned calendar: twelve months per year.
pub const MONTHS_PER_YEAR: u64 = 12;
/// Fallout 3 campaign epoch year.
pub const EPOCH_YEAR: u32 = 2277;
/// Fallout 3 campaign epoch month (October).
pub const EPOCH_MONTH: u8 = 10;
/// Fallout 3 campaign epoch day.
pub const EPOCH_DAY: u8 = 23;
/// Default vanilla timescale: thirty game seconds per real second.
pub const DEFAULT_TIMESCALE: u32 = 30;
/// Realtime remainder divisor: `real_us * timescale / 1000` is game ms.
pub const TIMESCALE_REMAINDER_DIVISOR: u64 = 1_000;
/// Calendar policy revision stored with snapshots.
pub const GAME_CALENDAR_REVISION: &str = "fo3-calendar-v1";

/// Why the clock advanced. Viewer systems must not invent a second reason.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimeAdvanceReason {
    Realtime,
    Wait,
    Sleep,
    FastTravel,
    Console,
}

/// One successful clock interval. Subsystems consume this instead of
/// independently rounding scaled frame time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameTimeAdvanced {
    pub from_game_ms: u64,
    pub to_game_ms: u64,
    pub reason: TimeAdvanceReason,
}

impl GameTimeAdvanced {
    #[must_use]
    pub fn delta_ms(self) -> u64 {
        self.to_game_ms.saturating_sub(self.from_game_ms)
    }
}

/// Derived civil calendar under [`GAME_CALENDAR_REVISION`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarDate {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

/// Authoritative integer clock. Timescale is game seconds per real second.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameClockState {
    pub absolute_game_ms: u64,
    pub fractional_timescale_remainder: u32,
    pub timescale: u32,
}

impl Default for GameClockState {
    fn default() -> Self {
        Self {
            absolute_game_ms: 0,
            fractional_timescale_remainder: 0,
            timescale: DEFAULT_TIMESCALE,
        }
    }
}

/// Clock mutation that would overflow `u64` milliseconds or the remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeError {
    Overflow,
}

impl GameClockState {
    #[must_use]
    pub fn now(self) -> GameTime {
        GameTime::from_ms(self.absolute_game_ms)
    }

    /// Lighting-only hour in `0.0..24.0`. Do not persist this float.
    #[must_use]
    pub fn hour_as_f32(self) -> f32 {
        (self.absolute_game_ms % MS_PER_DAY) as f32 / MS_PER_HOUR as f32
    }

    #[must_use]
    pub fn calendar(self) -> CalendarDate {
        calendar_from_ms(self.absolute_game_ms)
    }

    /// Advances by an exact game-millisecond delta. Remainder is unchanged.
    pub fn advance_game_ms(
        &mut self,
        delta_ms: u64,
        reason: TimeAdvanceReason,
    ) -> Result<GameTimeAdvanced, TimeError> {
        let from = self.absolute_game_ms;
        let to = from.checked_add(delta_ms).ok_or(TimeError::Overflow)?;
        self.absolute_game_ms = to;
        Ok(GameTimeAdvanced {
            from_game_ms: from,
            to_game_ms: to,
            reason,
        })
    }

    /// Converts a real-time interval through integer timescale + remainder.
    pub fn advance_realtime(&mut self, real_delta_us: u64) -> Result<GameTimeAdvanced, TimeError> {
        let from = self.absolute_game_ms;
        if self.timescale == 0 || real_delta_us == 0 {
            return Ok(GameTimeAdvanced {
                from_game_ms: from,
                to_game_ms: from,
                reason: TimeAdvanceReason::Realtime,
            });
        }
        let product = real_delta_us
            .checked_mul(u64::from(self.timescale))
            .and_then(|value| value.checked_add(u64::from(self.fractional_timescale_remainder)))
            .ok_or(TimeError::Overflow)?;
        let game_ms = product / TIMESCALE_REMAINDER_DIVISOR;
        let remainder = (product % TIMESCALE_REMAINDER_DIVISOR) as u32;
        let advanced = self.advance_game_ms(game_ms, TimeAdvanceReason::Realtime)?;
        self.fractional_timescale_remainder = remainder;
        Ok(advanced)
    }
}

#[must_use]
pub fn calendar_from_ms(ms: u64) -> CalendarDate {
    let millisecond = (ms % MS_PER_SECOND) as u16;
    let total_seconds = ms / MS_PER_SECOND;
    let second = (total_seconds % 60) as u8;
    let total_minutes = total_seconds / 60;
    let minute = (total_minutes % 60) as u8;
    let total_hours = total_minutes / 60;
    let hour = (total_hours % 24) as u8;
    let mut day_index = total_hours / 24;
    let mut year = EPOCH_YEAR;
    let mut month = EPOCH_MONTH;
    let mut day = EPOCH_DAY;
    while day_index > 0 {
        let remaining_in_month = DAYS_PER_MONTH - u64::from(day) + 1;
        if day_index < remaining_in_month {
            day += day_index as u8;
            break;
        }
        day_index -= remaining_in_month;
        day = 1;
        if month == MONTHS_PER_YEAR as u8 {
            month = 1;
            year = year.saturating_add(1);
        } else {
            month += 1;
        }
    }
    CalendarDate {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millisecond,
    }
}

#[cfg(test)]
#[path = "tests/time.rs"]
mod tests;
