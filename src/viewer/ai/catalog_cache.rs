//! Prepared package-runtime catalog cache.
//!
//! Package startup is shared by the autonomous runtime and the console
//! adapter. The cache belongs to the AI capability, not the console surface:
//! it prevents an actor batch from repeatedly reading and deserializing the
//! same prepared catalogs while allowing the per-cell actor half to invalidate
//! independently from the content-wide package half.

use std::sync::Arc;

use bevy::prelude::Resource;

use crate::vsa::{PreparedActorCatalog, PreparedPackageCatalog};

#[derive(Resource, Default)]
pub(crate) struct PackageCatalogCache {
    pub(crate) actor: Option<(String, Arc<PreparedActorCatalog>)>,
    pub(crate) packages: Option<(String, Arc<PreparedPackageCatalog>)>,
    pub(crate) actor_disk_loads: usize,
    pub(crate) package_disk_loads: usize,
}

#[cfg(test)]
impl PackageCatalogCache {
    pub(crate) fn disk_loads(&self) -> (usize, usize) {
        (self.actor_disk_loads, self.package_disk_loads)
    }
}
