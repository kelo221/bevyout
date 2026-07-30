use std::fmt;

use serde::{Deserialize, Serialize};

const OBJECT_INDEX_MASK: u32 = 0x00ff_ffff;

/// A load-order-wide, master-remapped 32-bit record identifier.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct FormId(pub u32);

impl FormId {
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl From<u32> for FormId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<FormId> for u32 {
    fn from(value: FormId) -> Self {
        value.0
    }
}

impl fmt::Display for FormId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:08x}", self.0)
    }
}

/// Resolves a plugin-local FormID through its declared master table.
///
/// Local file slots index `master_indices`; the current plugin occupies the
/// first slot after its masters. Values beyond the declared table retain the
/// historical parser behavior and resolve to the current plugin, allowing the
/// record parser to diagnose malformed content at its own boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormIdResolver {
    current_index: u8,
    master_indices: Vec<u8>,
}

impl FormIdResolver {
    pub fn new(current_index: u8, master_indices: Vec<u8>) -> Self {
        Self {
            current_index,
            master_indices,
        }
    }

    pub fn adjust(&self, raw: u32) -> u32 {
        self.resolve(FormId(raw)).value()
    }

    pub fn resolve(&self, raw: FormId) -> FormId {
        let local_file_index = (raw.0 >> 24) as usize;
        let object_index = raw.0 & OBJECT_INDEX_MASK;
        let global_file_index = self
            .master_indices
            .get(local_file_index)
            .copied()
            .unwrap_or(self.current_index);
        FormId((u32::from(global_file_index) << 24) | object_index)
    }
}

#[cfg(test)]
#[path = "tests/form_id.rs"]
mod tests;
