//! Load-order winner and provenance policy shared by record collectors.
//!
//! This module is deliberately std-only so preparation collectors and the
//! executable feature suite exercise the same override/deletion behavior.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WinningRecord<T> {
    pub(crate) value: T,
    pub(crate) provenance: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct WinningRecords<T> {
    records: HashMap<u32, WinningRecord<T>>,
}

impl<T> Default for WinningRecords<T> {
    fn default() -> Self {
        Self {
            records: HashMap::new(),
        }
    }
}

impl<T> WinningRecords<T> {
    pub(crate) fn upsert(&mut self, form_id: u32, source: String, value: T) {
        match self.records.entry(form_id) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                let winner = entry.get_mut();
                winner.value = value;
                winner.provenance.push(source);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(WinningRecord {
                    value,
                    provenance: vec![source],
                });
            }
        }
    }

    pub(crate) fn delete(&mut self, form_id: u32) {
        self.records.remove(&form_id);
    }

    // Used by the std-only Cucumber inclusion and by later Wave 1 collectors;
    // the current ContentIndex collector consumes the completed map directly.
    #[allow(dead_code)]
    pub(crate) fn get(&self, form_id: u32) -> Option<&WinningRecord<T>> {
        self.records.get(&form_id)
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = (u32, WinningRecord<T>)> {
        self.records.into_iter()
    }
}
