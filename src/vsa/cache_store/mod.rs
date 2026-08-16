//! Global prepared-object storage.
//!
//! Recipes identify preparation inputs and policy. Objects identify the final
//! immutable bytes. Keeping those identities separate lets unrelated recipes
//! share one durable payload without changing artifact formats.

mod fs_store;
mod model;
pub(crate) mod policy;

pub(crate) use fs_store::FsPreparedObjectStore;
pub(crate) use model::{
    CandidateObject, PreparedObjectKind, PreparedObjectRef, PreparedObjectStore,
    PreparedRecipeInputs, PreparedRecipeRecord,
};
pub(crate) use policy::normalize_source_path;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
