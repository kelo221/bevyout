//! Integer game-time identifiers used by RPG policies.
//!
//! Wave 4 medical restoration and later elapsed-world-time mechanics take an
//! explicit [`GameTime`] rather than a Bevy frame clock. Wave 9 owns the
//! scheduler that advances this value.

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
